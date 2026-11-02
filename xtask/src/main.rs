use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

#[derive(Debug, Clone)]
struct SourceSpec {
    repository: String,
    commit: String,
    profile: String,
    root: PathBuf,
}

#[derive(Debug, Clone)]
struct FileRecord {
    compat_id: String,
    kind: String,
    repository: String,
    commit: String,
    source_path: String,
    source_sha256: String,
    external_identifier: String,
    legacy_name: String,
    owning_profile: String,
    status: String,
    parser_or_consumer: String,
    asset_dependencies: Vec<String>,
    resolved_asset_dependencies: Vec<String>,
    unresolved_asset_dependencies: Vec<String>,
    asset_dependency_status: String,
    parity_fixtures: Vec<String>,
    rust_owner: String,
    migration_rule: String,
    test_ids: Vec<String>,
    acceptance_criteria: String,
    verification_level: String,
    verification_evidence: Vec<String>,
    notes: String,
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("xtask compat-scan: {error}");
            ExitCode::from(2)
        }
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);
    let Some(command) = args.next() else {
        print_help();
        return Ok(());
    };

    if command == "--help" || command == "-h" {
        print_help();
        return Ok(());
    }
    if command != "compat-scan" {
        return Err(format!("unknown command: {command}"));
    }

    let mut sources = Vec::new();
    let mut output = None;
    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--source" => {
                let value = args.next().ok_or_else(|| {
                    "--source requires REPOSITORY@COMMIT@PROFILE=PATH".to_string()
                })?;
                sources.push(parse_source(&value)?);
            }
            "--out" => {
                output = Some(PathBuf::from(
                    args.next()
                        .ok_or_else(|| "--out requires a file path".to_string())?,
                ));
            }
            "--help" | "-h" => {
                print_help();
                return Ok(());
            }
            _ => return Err(format!("unknown argument: {argument}")),
        }
    }

    if sources.is_empty() {
        return Err("at least one --source is required".to_string());
    }
    let output = output.ok_or_else(|| "--out is required".to_string())?;

    let mut records = Vec::new();
    for source in &sources {
        if !source.root.is_dir() {
            return Err(format!(
                "source root is not a directory for {}: {}",
                source.repository,
                source.root.display()
            ));
        }
        scan_source(source, &mut records).map_err(|error| {
            format!(
                "failed scanning {} at {}: {error}",
                source.repository,
                source.root.display()
            )
        })?;
    }

    validate_asset_dependencies(&mut records);
    apply_runner_evidence(&mut records)?;
    records.sort_by(|left, right| {
        (
            left.repository.as_str(),
            left.owning_profile.as_str(),
            left.source_path.as_str(),
        )
            .cmp(&(
                right.repository.as_str(),
                right.owning_profile.as_str(),
                right.source_path.as_str(),
            ))
    });
    let json = render_manifest(&sources, &records);
    if let Some(parent) = output.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)
                .map_err(|error| format!("cannot create output directory: {error}"))?;
        }
    }
    fs::write(&output, json.as_bytes())
        .map_err(|error| format!("cannot write {}: {error}", output.display()))?;

    let mut counts = BTreeMap::<String, usize>::new();
    for record in &records {
        *counts.entry(record.kind.clone()).or_default() += 1;
    }
    println!(
        "Compatibility scan complete: {} sources, {} records -> {}",
        sources.len(),
        records.len(),
        output.display()
    );
    for (kind, count) in counts {
        println!("  {kind}: {count}");
    }
    Ok(())
}

fn print_help() {
    println!("xtask — tora-typesetter development tasks");
    println!("Usage:");
    println!("  cargo run -p xtask -- compat-scan --source REPOSITORY@COMMIT@PROFILE=PATH [--source ...] --out FILE");
    println!();
    println!("The scanner reads pinned local checkouts without modifying them.");
    println!("It hashes relevant source/data/fixture/font files and writes stable canonical JSON.");
    println!("Example:");
    println!("  cargo run -p xtask -- compat-scan --source SUGINACH/Tora_Book_Typesetter_Orange@<40-hex-commit>@legacy-orange-v23=legacy/orange --out artifacts/compat.json");
}

fn parse_source(value: &str) -> Result<SourceSpec, String> {
    let (identity, root) = value.split_once('=').ok_or_else(|| {
        format!("invalid --source {value:?}; expected REPOSITORY@COMMIT@PROFILE=PATH")
    })?;
    let mut parts = identity.split('@');
    let repository = parts.next().unwrap_or_default().trim();
    let commit = parts.next().unwrap_or_default().trim();
    let profile = parts.next().unwrap_or_default().trim();
    if parts.next().is_some()
        || repository.is_empty()
        || !repository.contains('/')
        || commit.len() != 40
        || !commit
            .chars()
            .all(|character| character.is_ascii_hexdigit())
        || profile.is_empty()
        || root.trim().is_empty()
    {
        return Err(format!("invalid --source identity or path: {value:?}"));
    }
    Ok(SourceSpec {
        repository: repository.to_string(),
        commit: commit.to_ascii_lowercase(),
        profile: profile.to_string(),
        root: PathBuf::from(root),
    })
}

fn scan_source(source: &SourceSpec, records: &mut Vec<FileRecord>) -> io::Result<()> {
    let mut files = Vec::new();
    collect_files(&source.root, &source.root, &mut files)?;
    files.sort();
    for path in files {
        let bytes = fs::read(&path)?;
        let source_path = path
            .strip_prefix(&source.root)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "path escaped source root"))?
            .to_string_lossy()
            .replace('\\', "/");
        let kind =
            if source.profile == "requirements-docs" && source_path.starts_with("source_code/") {
                "source-snapshot".to_string()
            } else {
                classify(&source_path)
            };
        let hash = hex(&sha256(&bytes));
        let legacy_name = path
            .file_name()
            .and_then(OsStr::to_str)
            .unwrap_or("unknown")
            .to_string();
        let status = status_for(&source.profile, &kind);
        let (parser_or_consumer, rust_owner, migration_rule, parity_fixtures) =
            ownership_for(source, &source_path, &kind);
        let (test_ids, acceptance_criteria) = tests_for(source, &kind, &rust_owner);
        let asset_dependencies = extract_asset_dependencies(&source_path, &kind, &bytes);
        let notes = match kind.as_str() {
            "font" => "Byte identity scanned; glyph coverage, font tables and redistribution license are not yet verified. Ownership is provisional.",
            "documentation" => "Documentation source; not proof of executable behavior. Converted to requirements/ADRs only after review.",
            "source-snapshot" => "Historical source snapshot stored as documentation text; not compiled or executed as source code.",
            "fixture" | "corpus" => "Pinned profile input; retain independently and never substitute another profile's fixture.",
            _ => "Scanner-generated record with a conservative owner/migration classification; semantic dependency edges require review.",
        };
        records.push(FileRecord {
            compat_id: format!("{}:{}:{}", source.profile, kind, source_path),
            kind,
            repository: source.repository.clone(),
            commit: source.commit.clone(),
            source_path: source_path.clone(),
            source_sha256: hash,
            external_identifier: source_path,
            legacy_name,
            owning_profile: source.profile.clone(),
            status: status.to_string(),
            parser_or_consumer,
            asset_dependencies,
            resolved_asset_dependencies: Vec::new(),
            unresolved_asset_dependencies: Vec::new(),
            asset_dependency_status: "pending-validation".to_string(),
            parity_fixtures,
            rust_owner,
            migration_rule,
            test_ids,
            acceptance_criteria,
            verification_level: "scanned-hash-only".to_string(),
            verification_evidence: Vec::new(),
            notes: notes.to_string(),
        });
    }
    Ok(())
}

fn ownership_for(
    source: &SourceSpec,
    source_path: &str,
    kind: &str,
) -> (String, String, String, Vec<String>) {
    let lower = source_path.to_ascii_lowercase();
    let fixture_path = match source.profile.as_str() {
        "legacy-orange-v23" => "tests/fixtures/geometry-audit-smoke.txt",
        "legacy-green-v23" | "legacy-gold-v21" | "legacy-imud-v20x" => {
            "text_files_for_typesetter_tests/very_long_test_text.txt"
        }
        "legacy-yellow-v10" => "very_long_test_text.txt",
        _ => "source_code/V_current_snapshot.txt",
    };
    let profile_fixture = format!("{}@{}:{}", source.repository, source.commit, fixture_path);
    let (parser, owner, migration) = if source.profile == "requirements-docs" {
        (
            "requirements-and-research".to_string(),
            "docs/requirements".to_string(),
            "spec-only; convert accepted requirement to feature or ADR, never infer executable behavior".to_string(),
        )
    } else if lower.ends_with("/typesetter.js") || lower == "typesetter.js" {
        (
            "legacy-layout-engine".to_string(),
            "engine-core".to_string(),
            "port behavior under its owning profile with a named fixture; never merge algorithms across profiles".to_string(),
        )
    } else if lower.ends_with("/decorations.js") || lower == "decorations.js" {
        (
            "legacy-decoration-and-geometry-runtime".to_string(),
            "engine-core::decorations".to_string(),
            "model decoration flow/visual/anchor geometry as explicit engine operations; preserve profile behavior".to_string(),
        )
    } else if lower.ends_with("/geometry-audit.js") {
        (
            "legacy-independent-geometry-audit".to_string(),
            "audit".to_string(),
            "reimplement as an independent audit; do not use audit measurements as layout inputs"
                .to_string(),
        )
    } else if lower.ends_with("geometry-audit-cli.js") {
        (
            "legacy-offline-renderer-runner".to_string(),
            "xtask".to_string(),
            "retain as read-only pinned oracle; do not link legacy JavaScript into Rust crates"
                .to_string(),
        )
    } else if lower == "app.js" || lower.ends_with("/app.js") {
        (
            "legacy-application-orchestration".to_string(),
            "app-core".to_string(),
            "replace with immutable typed jobs and explicit application state; preserve profile settings".to_string(),
        )
    } else if lower == "index.html"
        || lower.ends_with("/index.html")
        || lower == "renderer.html"
        || lower.ends_with("/renderer.html")
        || lower.ends_with("/renderer-bridge.js")
        || kind == "style"
        || kind == "dom-contract"
    {
        (
            "legacy-dom-and-css-renderer".to_string(),
            "legacy-renderer-adapter".to_string(),
            "retain only as a compatibility/export adapter; never use browser reflow to decide Rust pagination".to_string(),
        )
    } else if lower.contains("setting")
        || lower.contains("preset")
        || lower.contains("default_settings")
        || kind == "settings"
    {
        (
            "legacy-profile-settings-loader".to_string(),
            "settings".to_string(),
            "preserve losslessly and resolve defaults only within the selected legacy profile"
                .to_string(),
        )
    } else if kind == "font" {
        (
            "font-environment-inventory".to_string(),
            "font-environment".to_string(),
            "verify font tables, glyph coverage, aliases and license before packaging or selecting fallback".to_string(),
        )
    } else if kind == "asset" {
        (
            "legacy-asset-loader".to_string(),
            "asset-registry".to_string(),
            "validate and hash assets; retain offline and profile-specific dependencies"
                .to_string(),
        )
    } else if kind == "fixture" || kind == "corpus" {
        (
            "legacy-fixture-runner".to_string(),
            "compat-test-harness".to_string(),
            "retain exact bytes and profile identity; use as a parity fixture, never mutate baseline input".to_string(),
        )
    } else if kind == "documentation" || kind == "source-snapshot" {
        (
            "documentation-and-history".to_string(),
            "docs/history".to_string(),
            "historical-only; cite as context and require executable evidence for behavior claims"
                .to_string(),
        )
    } else if kind == "test-infrastructure" {
        (
            "legacy-test-infrastructure".to_string(),
            "compat-test-harness".to_string(),
            "port test intent into deterministic Rust tests; keep legacy runner pinned".to_string(),
        )
    } else if kind == "build-config" {
        (
            "legacy-build-tooling".to_string(),
            "build-system".to_string(),
            "retain only the configuration required by the isolated legacy runner".to_string(),
        )
    } else {
        (
            "legacy-source-inventory".to_string(),
            "legacy-source-reference".to_string(),
            "historical-only until a concrete feature owner and parity fixture are approved"
                .to_string(),
        )
    };
    let fixtures = if matches!(kind, "fixture" | "corpus")
        || owner == "engine-core"
        || owner == "engine-core::decorations"
    {
        vec![profile_fixture]
    } else if owner == "xtask" || owner == "audit" {
        vec!["tools/legacy-runner/README.md".to_string()]
    } else {
        Vec::new()
    };
    (parser, owner, migration, fixtures)
}

fn tests_for(source: &SourceSpec, kind: &str, owner: &str) -> (Vec<String>, String) {
    if source.profile == "requirements-docs" {
        return (
            vec!["SPEC-TO-ADR".to_string()],
            "Every accepted requirement links to a feature/test or ADR; prose alone never proves executable behavior.".to_string(),
        );
    }
    if matches!(kind, "fixture" | "corpus") {
        return (
            vec![
                "SRC-PIN-HASH".to_string(),
                "CORPUS-BYTE-IDENTITY".to_string(),
                "PROFILE-REPLAY".to_string(),
                "DETERMINISM-REPEAT".to_string(),
            ],
            "Pinned input byte length and SHA-256 must match; replay only with this profile's own settings/fonts; repeated canonical output hashes must be identical.".to_string(),
        );
    }
    match owner {
        "engine-core" | "engine-core::decorations" => (
            vec![
                "SRC-PIN-HASH".to_string(),
                "PROFILE-REPLAY".to_string(),
                "SEMANTIC-CONSERVATION".to_string(),
                "DECISION-LEDGER-PARITY".to_string(),
                "DETERMINISM-REPEAT".to_string(),
            ],
            "Zero unexplained missing/duplicated/reordered source spans; one valid decision record per required layout block; identical repeated output.".to_string(),
        ),
        "settings" => (
            vec![
                "SRC-PIN-HASH".to_string(),
                "SETTINGS-ROUNDTRIP".to_string(),
            ],
            "Unknown settings keys survive lossless round-trip; defaults are resolved only inside the owning profile; no silent cross-profile fallback.".to_string(),
        ),
        "font-environment" => (
            vec!["SRC-PIN-HASH".to_string(), "FONT-PREFLIGHT".to_string()],
            "Every required face loads and required glyph coverage is reported; failed faces are zero; redistribution license is reviewed separately.".to_string(),
        ),
        "asset-registry" => (
            vec![
                "SRC-PIN-HASH".to_string(),
                "ASSET-REFERENCE-CLOSURE".to_string(),
            ],
            "Every local reference resolves to a pinned asset; no missing dependency, path traversal or unapproved external URL.".to_string(),
        ),
        "legacy-renderer-adapter" => (
            vec![
                "SRC-PIN-HASH".to_string(),
                "LEGACY-DOM-GEOMETRY".to_string(),
                "DETERMINISM-REPEAT".to_string(),
            ],
            "Page/column/block bounds are measured with explicit units and tolerances; zero clipping/overflow; browser output never feeds Rust layout decisions.".to_string(),
        ),
        "xtask" | "audit" => (
            vec!["SRC-PIN-HASH".to_string(), "LEGACY-DOM-GEOMETRY".to_string()],
            "Pinned runner/audit source hash matches; offline run records zero external requests and emits a complete report.".to_string(),
        ),
        "app-core" => (
            vec![
                "SRC-PIN-HASH".to_string(),
                "SETTINGS-ROUNDTRIP".to_string(),
                "PROFILE-REPLAY".to_string(),
            ],
            "Application jobs carry immutable profile/config/input identities; unknown settings are not dropped and cancellation cannot publish stale output.".to_string(),
        ),
        "compat-test-harness" => (
            vec![
                "SRC-PIN-HASH".to_string(),
                "PROFILE-REPLAY".to_string(),
                "DETERMINISM-REPEAT".to_string(),
            ],
            "The fixture executes under its exact source profile and environment; repeat output signatures match.".to_string(),
        ),
        "build-system" => (
            vec!["SRC-PIN-HASH".to_string()],
            "Only the pinned legacy runner dependencies are installed; lockfile and toolchain identities are checked.".to_string(),
        ),
        _ => (
            vec!["SRC-PIN-HASH".to_string()],
            "Source identity is pinned; semantic ownership and migration remain historical-only until reviewed.".to_string(),
        ),
    }
}

fn parse_runner_evidence_line(line: &str) -> Option<[&str; 8]> {
    let mut fields = line.split('\t');
    let parsed = [
        fields.next()?,
        fields.next()?,
        fields.next()?,
        fields.next()?,
        fields.next()?,
        fields.next()?,
        fields.next()?,
        fields.next()?,
    ];
    if fields.next().is_some() {
        return None;
    }
    Some(parsed)
}

fn apply_runner_evidence(records: &mut [FileRecord]) -> Result<(), String> {
    const EVIDENCE: &str = include_str!("../../legacy/runner-evidence.tsv");
    apply_runner_evidence_rows(records, EVIDENCE)
}

fn apply_runner_evidence_rows(records: &mut [FileRecord], evidence: &str) -> Result<(), String> {
    let mut applied_rows = 0usize;
    for line in evidence.lines() {
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        let fields = parse_runner_evidence_line(line)
            .ok_or_else(|| format!("malformed runner evidence row: {line:?}"))?;
        let [profile, source_commit, runner_commit, runner_url, level, path, sha256, role] = fields;
        let mut matched = false;
        for record in records.iter_mut() {
            if record.owning_profile == profile
                && record.commit == source_commit
                && record.source_path == path
                && record.source_sha256 == sha256
            {
                record.verification_level = level.to_string();
                record.verification_evidence.push(format!(
                    "{runner_url} (runner commit {runner_commit}; role {role})"
                ));
                if record.status != "blocked-license" {
                    record.status = match record.kind.as_str() {
                        "javascript" | "dom-contract" => "verified-executable".to_string(),
                        "style" | "fixture" | "data" | "asset" => "verified-data".to_string(),
                        _ => record.status.clone(),
                    };
                }
                matched = true;
            }
        }
        if !matched {
            return Err(format!(
                "runner evidence did not match an exact profile/commit/path/SHA-256 record: {line}"
            ));
        }
        applied_rows += 1;
    }
    if applied_rows == 0 {
        return Err("runner evidence table contains no evidence rows".to_string());
    }
    Ok(())
}

fn extract_asset_dependencies(source_path: &str, kind: &str, bytes: &[u8]) -> Vec<String> {
    let lower_path = source_path.to_ascii_lowercase();
    if lower_path.starts_with("reports/")
        || lower_path.starts_with("debug/")
        || lower_path.contains("rendered-document.html")
    {
        return Vec::new();
    }
    let is_svg_asset = kind == "asset"
        && Path::new(source_path)
            .extension()
            .and_then(OsStr::to_str)
            .is_some_and(|extension| extension.eq_ignore_ascii_case("svg"));
    if !matches!(
        kind,
        "javascript" | "dom-contract" | "style" | "settings" | "data"
    ) && !is_svg_asset
    {
        return Vec::new();
    }
    let text = String::from_utf8_lossy(bytes);
    let mut dependencies = BTreeSet::new();

    if kind == "javascript" {
        extract_javascript_dependencies(&text, &mut dependencies, source_path);
    } else {
        extract_url_functions(&text, &mut dependencies, source_path);
        extract_attributes(
            &text,
            &[
                "src=",
                "href=",
                "poster=",
                "data-src=",
                "xlink:href=",
                "srcset=",
            ],
            &mut dependencies,
            source_path,
        );
        // Settings and data files are structured catalogs. Retain their
        // explicit file-name values, but reject extension-only and directory
        // fragments in normalize_asset_reference.
        if lower_path.ends_with("package-lock.json") {
            // npm lockfiles contain package-internal bin paths (for example
            // "cli.js") that do not exist in the repository. Keep only the
            // explicit package resolution URLs as external provenance edges.
            for value in extract_json_string_property(&text, "resolved") {
                if let Some(normalized) = normalize_asset_reference(source_path, &value) {
                    dependencies.insert(normalized);
                }
            }
        } else if matches!(kind, "settings" | "data")
            && !matches!(
                lower_path.as_str(),
                "fonts-extended.json" | "fonts-priority.json" | "fonts/fonts/fonts.json"
            )
        {
            // These font catalogs are not all asset manifests:
            // - fonts/fonts.json is the active catalog whose file entries are loaded.
            // - fonts-extended.json is used only to populate picker labels.
            // - fonts-priority.json is not loaded by the pinned app.
            // - fonts/fonts/fonts.json is an unreferenced duplicate catalog.
            // Treating every file value in the latter three as a path creates
            // thousands of false dependencies to fonts that are not fetched there.
            for value in extract_quoted_strings(&text) {
                if is_path_candidate(&value) {
                    if let Some(normalized) = normalize_asset_reference(source_path, &value) {
                        dependencies.insert(normalized);
                    }
                }
            }
        }
    }
    dependencies.into_iter().collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum JsToken {
    Identifier(String),
    String(String),
    Template(String),
    Punct(char),
}

fn is_regex_literal_start(bytes: &[u8], slash_index: usize) -> bool {
    let mut previous = slash_index;
    while previous > 0 && bytes[previous - 1].is_ascii_whitespace() {
        previous -= 1;
    }
    if previous == 0 {
        return true;
    }

    match bytes[previous - 1] {
        b'(' | b'[' | b'{' | b',' | b':' | b'=' | b'!' | b'?' | b';' | b'&' | b'|' | b'>' => true,
        _ => {
            let mut start = previous;
            while start > 0
                && (bytes[start - 1].is_ascii_alphanumeric()
                    || bytes[start - 1] == b'_'
                    || bytes[start - 1] == b'$')
            {
                start -= 1;
            }
            let keyword = &bytes[start..previous];
            keyword == b"return"
                || keyword == b"throw"
                || keyword == b"case"
                || keyword == b"delete"
                || keyword == b"void"
                || keyword == b"typeof"
                || keyword == b"instanceof"
                || keyword == b"in"
                || keyword == b"of"
                || keyword == b"yield"
                || keyword == b"await"
                || keyword == b"else"
        }
    }
}

fn skip_regex_literal(bytes: &[u8], start: usize) -> usize {
    let mut index = start + 1;
    let mut escaped = false;
    let mut in_character_class = false;

    while index < bytes.len() {
        let current = bytes[index];
        if escaped {
            escaped = false;
        } else if current == b'\\' {
            escaped = true;
        } else if current == b'[' {
            in_character_class = true;
        } else if current == b']' {
            in_character_class = false;
        } else if current == b'/' && !in_character_class {
            index += 1;
            while index < bytes.len() && bytes[index].is_ascii_alphabetic() {
                index += 1;
            }
            return index;
        } else if current == b'\n' || current == b'\r' {
            return start + 1;
        }
        index += 1;
    }

    start + 1
}

fn skip_template_literal(bytes: &[u8], start: usize) -> usize {
    let mut index = start + 1;
    let mut expression_depth = 0usize;

    while index < bytes.len() {
        let current = bytes[index];
        if expression_depth == 0 {
            if current == b'\\' {
                index = (index + 2).min(bytes.len());
            } else if current == b'$' && bytes.get(index + 1) == Some(&b'{') {
                expression_depth = 1;
                index += 2;
            } else if current == 0x60 {
                return index + 1;
            } else {
                index += 1;
            }
            continue;
        }

        if current == b'\'' || current == b'"' {
            let quote = current;
            index += 1;
            let mut escaped = false;
            while index < bytes.len() {
                let byte = bytes[index];
                if escaped {
                    escaped = false;
                } else if byte == b'\\' {
                    escaped = true;
                } else if byte == quote {
                    index += 1;
                    break;
                }
                index += 1;
            }
        } else if current == b'/' && bytes.get(index + 1) == Some(&b'/') {
            index += 2;
            while index < bytes.len() && bytes[index] != b'\n' {
                index += 1;
            }
        } else if current == b'/' && bytes.get(index + 1) == Some(&b'*') {
            index += 2;
            while index + 1 < bytes.len() && !(bytes[index] == b'*' && bytes[index + 1] == b'/') {
                index += 1;
            }
            index = (index + 2).min(bytes.len());
        } else if current == b'/' && is_regex_literal_start(bytes, index) {
            index = skip_regex_literal(bytes, index);
        } else if current == 0x60 {
            index = skip_template_literal(bytes, index);
        } else if current == b'{' {
            expression_depth += 1;
            index += 1;
        } else if current == b'}' {
            expression_depth -= 1;
            index += 1;
        } else {
            index += 1;
        }
    }

    bytes.len()
}

fn tokenize_javascript(text: &str) -> Vec<JsToken> {
    let bytes = text.as_bytes();
    let mut tokens = Vec::new();
    let mut index = 0usize;

    while index < bytes.len() {
        let current = bytes[index];
        if current.is_ascii_whitespace() {
            index += 1;
            continue;
        }

        if current == b'/' && bytes.get(index + 1) == Some(&b'/') {
            index += 2;
            while index < bytes.len() && bytes[index] != b'\n' {
                index += 1;
            }
            continue;
        }
        if current == b'/' && bytes.get(index + 1) == Some(&b'*') {
            index += 2;
            while index + 1 < bytes.len() && !(bytes[index] == b'*' && bytes[index + 1] == b'/') {
                index += 1;
            }
            index = (index + 2).min(bytes.len());
            continue;
        }

        if current == b'/' && is_regex_literal_start(bytes, index) {
            index = skip_regex_literal(bytes, index);
            continue;
        }

        if current == b'\'' || current == b'"' {
            let quote = current;
            index += 1;
            let start = index;
            let mut escaped = false;
            while index < bytes.len() {
                let byte = bytes[index];
                if escaped {
                    escaped = false;
                } else if byte == b'\\' {
                    escaped = true;
                } else if byte == quote {
                    break;
                }
                index += 1;
            }
            let value = text[start..index].replace("\\/", "/").replace("\\\\", "\\");
            tokens.push(JsToken::String(value));
            if index < bytes.len() {
                index += 1;
            }
            continue;
        }

        // Template literals are retained as opaque text so explicit HTML/CSS
        // URL references inside them can be inspected, but their interpolated
        // expressions are never mistaken for static file-name strings.
        if current == 0x60 {
            let template_end = skip_template_literal(bytes, index);
            let content_end = template_end.saturating_sub(1).max(index + 1);
            tokens.push(JsToken::Template(text[index + 1..content_end].to_string()));
            index = template_end;
            continue;
        }

        if current.is_ascii_alphabetic() || current == b'_' || current == b'$' {
            let start = index;
            index += 1;
            while index < bytes.len()
                && (bytes[index].is_ascii_alphanumeric()
                    || bytes[index] == b'_'
                    || bytes[index] == b'$')
            {
                index += 1;
            }
            tokens.push(JsToken::Identifier(text[start..index].to_string()));
            continue;
        }

        tokens.push(JsToken::Punct(current as char));
        index += 1;
    }
    tokens
}

fn has_function_parameter(tokens: &[JsToken], name: &str) -> bool {
    for index in 0..tokens.len() {
        if tokens.get(index) == Some(&JsToken::Identifier("function".to_string())) {
            let mut open = index + 1;
            while open < tokens.len() && tokens.get(open) != Some(&JsToken::Punct('(')) {
                open += 1;
            }
            if open == tokens.len() {
                continue;
            }
            let mut cursor = open + 1;
            let mut depth = 1usize;
            while cursor < tokens.len() && depth > 0 {
                match tokens.get(cursor) {
                    Some(JsToken::Punct('(')) => depth += 1,
                    Some(JsToken::Punct(')')) => depth -= 1,
                    Some(JsToken::Identifier(parameter)) if depth == 1 && parameter == name => {
                        return true;
                    }
                    _ => {}
                }
                cursor += 1;
            }
        }

        if tokens.get(index) == Some(&JsToken::Punct('(')) {
            let mut cursor = index + 1;
            let mut depth = 1usize;
            let mut contains_name = false;
            while cursor < tokens.len() && depth > 0 {
                match tokens.get(cursor) {
                    Some(JsToken::Punct('(')) => depth += 1,
                    Some(JsToken::Punct(')')) => depth -= 1,
                    Some(JsToken::Identifier(parameter)) if depth == 1 && parameter == name => {
                        contains_name = true;
                    }
                    _ => {}
                }
                cursor += 1;
            }
            if contains_name
                && tokens.get(cursor) == Some(&JsToken::Punct('='))
                && tokens.get(cursor + 1) == Some(&JsToken::Punct('>'))
            {
                return true;
            }
        }

        if matches!(tokens.get(index), Some(JsToken::Identifier(parameter)) if parameter == name)
            && tokens.get(index + 1) == Some(&JsToken::Punct('='))
            && tokens.get(index + 2) == Some(&JsToken::Punct('>'))
        {
            return true;
        }
    }

    false
}

fn is_write_target_path_join(tokens: &[JsToken], join_index: usize) -> bool {
    if join_index < 4 {
        return false;
    }
    matches!(
        tokens.get(join_index - 4),
        Some(JsToken::Identifier(name))
            if matches!(
                name.as_str(),
                "writeFileSync" | "writeFile" | "appendFileSync" | "appendFile" | "createWriteStream"
            )
    ) && tokens.get(join_index - 3) == Some(&JsToken::Punct('('))
        && tokens.get(join_index - 2) == Some(&JsToken::Identifier("path".to_string()))
        && tokens.get(join_index - 1) == Some(&JsToken::Punct('.'))
}

fn static_fetch_prefix_for_array(tokens: &[JsToken], array_name: &str) -> Option<String> {
    for index in 0..tokens.len() {
        if tokens.get(index) != Some(&JsToken::Identifier("for".to_string()))
            || tokens.get(index + 1) != Some(&JsToken::Punct('('))
        {
            continue;
        }

        let mut cursor = index + 2;
        if matches!(
            tokens.get(cursor),
            Some(JsToken::Identifier(kind)) if kind == "const" || kind == "let" || kind == "var"
        ) {
            cursor += 1;
        }
        let Some(JsToken::Identifier(iterator)) = tokens.get(cursor) else {
            continue;
        };
        let iterator = iterator.clone();
        if tokens.get(cursor + 1) != Some(&JsToken::Identifier("of".to_string()))
            || tokens.get(cursor + 2) != Some(&JsToken::Identifier(array_name.to_string()))
        {
            continue;
        }

        let mut close = cursor + 3;
        while close < tokens.len() && tokens.get(close) != Some(&JsToken::Punct(')')) {
            close += 1;
        }
        if close == tokens.len() || tokens.get(close + 1) != Some(&JsToken::Punct('{')) {
            continue;
        }

        let mut brace_depth = 1usize;
        let mut body_end = close + 2;
        while body_end < tokens.len() && brace_depth > 0 {
            match tokens.get(body_end) {
                Some(JsToken::Punct('{')) => brace_depth += 1,
                Some(JsToken::Punct('}')) => brace_depth -= 1,
                _ => {}
            }
            body_end += 1;
        }

        for call in close + 2..body_end.saturating_sub(1) {
            if tokens.get(call) != Some(&JsToken::Identifier("fetch".to_string()))
                || tokens.get(call + 1) != Some(&JsToken::Punct('('))
                || tokens.get(call + 3) != Some(&JsToken::Punct('+'))
                || tokens.get(call + 4) != Some(&JsToken::Identifier(iterator.clone()))
                || tokens.get(call + 5) != Some(&JsToken::Punct(')'))
            {
                continue;
            }
            if let Some(JsToken::String(prefix)) = tokens.get(call + 2) {
                if !prefix.is_empty() {
                    return Some(prefix.clone());
                }
            }
        }
    }
    None
}

fn has_static_repository_root_binding(tokens: &[JsToken], name: &str, before: usize) -> bool {
    // A function parameter named "root" is runtime input, not proof of a
    // repository-root alias. Be conservative rather than resolving arbitrary
    // caller-supplied directories as repository assets.
    if has_function_parameter(tokens, name) {
        return false;
    }

    for index in 0..before.min(tokens.len()) {
        if !matches!(
            tokens.get(index),
            Some(JsToken::Identifier(kind)) if kind == "const" || kind == "let" || kind == "var"
        ) {
            continue;
        }

        match tokens.get(index + 1) {
            Some(JsToken::Identifier(binding))
                if binding == name && tokens.get(index + 2) == Some(&JsToken::Punct('=')) =>
            {
                let start = (index + 3).min(tokens.len());
                let end = (index + 20).min(tokens.len());
                let initializer = &tokens[start..end];
                let path_resolve = initializer.windows(3).any(|window| {
                    matches!(window[0], JsToken::Identifier(ref value) if value == "path")
                        && window[1] == JsToken::Punct('.')
                        && matches!(window[2], JsToken::Identifier(ref value) if value == "resolve")
                });
                let path_dirname = initializer.windows(3).any(|window| {
                    matches!(window[0], JsToken::Identifier(ref value) if value == "path")
                        && window[1] == JsToken::Punct('.')
                        && matches!(window[2], JsToken::Identifier(ref value) if value == "dirname")
                });
                let known_anchor = initializer.iter().any(|token| {
                    matches!(
                        token,
                        JsToken::Identifier(value)
                            if value == "__dirname" || value == "__filename" || value == "fileURLToPath"
                    ) || matches!(token, JsToken::String(value) if value == ".")
                });
                let process_cwd = initializer.windows(3).any(|window| {
                    matches!(window[0], JsToken::Identifier(ref value) if value == "process")
                        && window[1] == JsToken::Punct('.')
                        && matches!(window[2], JsToken::Identifier(ref value) if value == "cwd")
                });
                if ((path_resolve || path_dirname) && known_anchor) || process_cwd {
                    return true;
                }
            }
            Some(JsToken::Punct('{')) => {
                let mut cursor = index + 2;
                let mut has_root_property = false;
                while cursor < tokens.len() && tokens.get(cursor) != Some(&JsToken::Punct('}')) {
                    if matches!(tokens.get(cursor), Some(JsToken::Identifier(property)) if property == name)
                    {
                        has_root_property = true;
                    }
                    cursor += 1;
                }
                if has_root_property
                    && tokens.get(cursor + 1) == Some(&JsToken::Punct('='))
                    && tokens.get(cursor + 2) == Some(&JsToken::Identifier("require".to_string()))
                    && tokens.get(cursor + 3) == Some(&JsToken::Punct('('))
                {
                    if let Some(JsToken::String(module)) = tokens.get(cursor + 4) {
                        if module.contains("harness") {
                            return true;
                        }
                    }
                }
            }
            _ => {}
        }
    }

    false
}

fn static_string_expression(tokens: &[JsToken], start: usize) -> Option<(String, usize)> {
    let mut cursor = start;
    let mut value = String::new();

    loop {
        match tokens.get(cursor)? {
            JsToken::String(part) => {
                value.push_str(part);
                cursor += 1;
            }
            JsToken::Template(part) if !part.contains("${") => {
                value.push_str(part);
                cursor += 1;
            }
            JsToken::Identifier(name)
                if name == "encodeURIComponent"
                    && tokens.get(cursor + 1) == Some(&JsToken::Punct('(')) =>
            {
                let encoded_part = match tokens.get(cursor + 2)? {
                    JsToken::String(part) => part,
                    JsToken::Template(part) if !part.contains("${") => part,
                    _ => return None,
                };
                if tokens.get(cursor + 3) != Some(&JsToken::Punct(')')) {
                    return None;
                }
                // URL percent-encoding does not change the repository filename;
                // retain the decoded Unicode path for filesystem lookup.
                value.push_str(encoded_part);
                cursor += 4;
            }
            _ => return None,
        }

        if tokens.get(cursor) == Some(&JsToken::Punct('+')) {
            cursor += 1;
            continue;
        }

        return Some((value, cursor));
    }
}

fn extract_javascript_dependencies(text: &str, output: &mut BTreeSet<String>, source_path: &str) {
    let tokens = tokenize_javascript(text);
    let mut index = 0usize;

    while index < tokens.len() {
        match &tokens[index] {
            JsToken::Identifier(identifier) => {
                let next_is_open_call = tokens.get(index + 1) == Some(&JsToken::Punct('('));
                if identifier == "import" {
                    if let Some(JsToken::String(value)) = tokens.get(index + 1) {
                        if let Some(normalized) = normalize_module_reference(source_path, value) {
                            output.insert(normalized);
                        }
                    }
                }
                if matches!(identifier.as_str(), "fetch" | "require" | "import")
                    && next_is_open_call
                {
                    if identifier == "fetch" {
                        if let Some((value, end)) = static_string_expression(&tokens, index + 2) {
                            if matches!(
                                tokens.get(end),
                                Some(JsToken::Punct(',') | JsToken::Punct(')'))
                            ) {
                                if let Some(normalized) =
                                    normalize_fetch_reference(source_path, &value)
                                {
                                    output.insert(normalized);
                                }
                            }
                        }
                    } else if let Some(JsToken::String(value)) = tokens.get(index + 2) {
                        if let Some(normalized) = normalize_module_reference(source_path, value) {
                            output.insert(normalized);
                        }
                    }
                }

                if identifier == "new"
                    && tokens.get(index + 1) == Some(&JsToken::Identifier("URL".to_string()))
                    && tokens.get(index + 2) == Some(&JsToken::Punct('('))
                {
                    if let Some(JsToken::String(value)) = tokens.get(index + 3) {
                        if let Some(normalized) = normalize_module_reference(source_path, value) {
                            output.insert(normalized);
                        }
                    }
                }

                // Static module specifiers: import ... from "x" / export ... from "x".
                if identifier == "from" {
                    if let Some(JsToken::String(value)) = tokens.get(index + 1) {
                        if let Some(normalized) = normalize_module_reference(source_path, value) {
                            output.insert(normalized);
                        }
                    }
                }

                // Explicit asset properties in object literals and manifests.
                if matches!(
                    identifier.as_str(),
                    "file" | "src" | "href" | "poster" | "url" | "path" | "asset" | "fontFile"
                ) && tokens.get(index + 1) == Some(&JsToken::Punct(':'))
                {
                    if let Some(JsToken::String(value)) = tokens.get(index + 2) {
                        if let Some(normalized) = normalize_asset_reference(source_path, value) {
                            output.insert(normalized);
                        }
                    }
                }

                // Direct DOM asset URL assignments (link.href = "...").
                if matches!(identifier.as_str(), "href" | "src" | "poster")
                    && tokens.get(index.wrapping_sub(1)) == Some(&JsToken::Punct('.'))
                    && tokens.get(index + 1) == Some(&JsToken::Punct('='))
                {
                    if let Some(JsToken::String(value)) = tokens.get(index + 2) {
                        if let Some(normalized) = normalize_asset_reference(source_path, value) {
                            output.insert(normalized);
                        }
                    }
                }

                // Static path.join(__dirname|root|SCRIPTS_DIR, ...) calls
                // represent concrete repository file paths when every segment
                // is literal. This covers Node test fixtures and script entrypoints.
                if identifier == "join"
                    && tokens.get(index.wrapping_sub(1)) == Some(&JsToken::Punct('.'))
                    && tokens.get(index + 1) == Some(&JsToken::Punct('('))
                    && !is_write_target_path_join(&tokens, index)
                {
                    let base = match tokens.get(index + 2) {
                        Some(JsToken::Identifier(name)) if name == "__dirname" => Some(source_path),
                        Some(JsToken::Identifier(name))
                            if name == "SCRIPTS_DIR"
                                && has_static_repository_root_binding(
                                    &tokens,
                                    "SCRIPTS_DIR",
                                    index,
                                ) =>
                        {
                            Some(source_path)
                        }
                        Some(JsToken::Identifier(name))
                            if (name == "root" || name == "ROOT")
                                && has_static_repository_root_binding(&tokens, name, index) =>
                        {
                            Some("")
                        }
                        _ => None,
                    };
                    if let Some(base) = base {
                        let mut declaration = index;
                        let mut output_destination = false;
                        while declaration > 0 {
                            if matches!(tokens.get(declaration - 1), Some(JsToken::Punct(';'))) {
                                break;
                            }
                            if matches!(
                                tokens.get(declaration - 1),
                                Some(JsToken::Identifier(kind)) if kind == "const" || kind == "let" || kind == "var"
                            ) {
                                if let Some(JsToken::Identifier(name)) = tokens.get(declaration) {
                                    let upper = name.to_ascii_uppercase();
                                    output_destination = [
                                        "OUT", "OUTPUT", "REPORT", "SUMMARY", "RESULT", "LATEST",
                                        "ARTIFACT",
                                    ]
                                    .iter()
                                    .any(|marker| upper.contains(marker));
                                }
                                break;
                            }
                            declaration -= 1;
                        }
                        let mut cursor = index + 3;
                        let mut depth = 1usize;
                        let mut segments = Vec::new();
                        let mut valid = true;
                        while cursor < tokens.len() && depth > 0 {
                            match &tokens[cursor] {
                                JsToken::Punct('(') => depth += 1,
                                JsToken::Punct(')') => depth -= 1,
                                JsToken::String(value) if depth == 1 => {
                                    segments.push(value.clone())
                                }
                                JsToken::Template(_) | JsToken::Identifier(_) if depth == 1 => {
                                    valid = false;
                                    break;
                                }
                                _ => {}
                            }
                            cursor += 1;
                        }
                        if valid && !output_destination && !segments.is_empty() {
                            let joined = segments.join("/");
                            if has_known_asset_extension(&joined) {
                                if let Some(normalized) = normalize_asset_reference(base, &joined) {
                                    output.insert(normalized);
                                }
                            }
                        }
                    }
                }

                // Static Playwright page.goto(baseUrl + "/renderer.html") and
                // equivalent static page paths resolve from the served document root.
                if identifier == "goto"
                    && tokens.get(index.wrapping_sub(1)) == Some(&JsToken::Punct('.'))
                    && tokens.get(index + 1) == Some(&JsToken::Punct('('))
                {
                    let mut cursor = index + 2;
                    let mut depth = 1usize;
                    let mut static_path = None;
                    while cursor < tokens.len() && depth > 0 {
                        match &tokens[cursor] {
                            JsToken::Punct('(') => depth += 1,
                            JsToken::Punct(')') => depth -= 1,
                            JsToken::Punct(',') if depth == 1 => break,
                            JsToken::String(value) if depth == 1 => {
                                if static_path.is_some() {
                                    static_path = None;
                                    break;
                                }
                                static_path = Some(value.as_str());
                            }
                            _ => {}
                        }
                        cursor += 1;
                    }
                    if let Some(value) = static_path {
                        if let Some(normalized) = normalize_asset_reference("", value) {
                            output.insert(normalized);
                        }
                    }
                }

                // fs.readFileSync(process.argv[n] || "fallback.ext") has a
                // static fallback file even though its preferred path is dynamic.
                if identifier == "readFileSync"
                    && tokens.get(index + 1) == Some(&JsToken::Punct('('))
                {
                    let mut cursor = index + 2;
                    let mut has_fallback = false;
                    let mut fallback = None;
                    while cursor < tokens.len() {
                        match &tokens[cursor] {
                            JsToken::Punct(',') | JsToken::Punct(')') => break,
                            JsToken::Punct('|')
                                if tokens.get(cursor + 1) == Some(&JsToken::Punct('|')) =>
                            {
                                has_fallback = true;
                                cursor += 1;
                            }
                            JsToken::String(value) => fallback = Some(value.as_str()),
                            _ => {}
                        }
                        cursor += 1;
                    }
                    if has_fallback {
                        if let Some(value) = fallback {
                            if let Some(normalized) = normalize_asset_reference(source_path, value)
                            {
                                output.insert(normalized);
                            }
                        }
                    }
                }

                // Static arrays with names that explicitly describe asset
                // inventories (for example PRIORITY_FONT_FILES). These preserve
                // real font basenames used to construct runtime URLs.
                if matches!(identifier.as_str(), "const" | "let" | "var") {
                    if let (
                        Some(JsToken::Identifier(name)),
                        Some(JsToken::Punct('=')),
                        Some(JsToken::Punct('[')),
                    ) = (
                        tokens.get(index + 1),
                        tokens.get(index + 2),
                        tokens.get(index + 3),
                    ) {
                        let upper = name.to_ascii_uppercase();
                        if [
                            "FONT",
                            "FILE",
                            "ASSET",
                            "IMAGE",
                            "ORNAMENT",
                            "DECORATION",
                            "PRESET",
                        ]
                        .iter()
                        .any(|marker| upper.contains(marker))
                        {
                            let font_file_inventory = upper.contains("FONT_FILES");
                            let fetch_prefix = if font_file_inventory {
                                static_fetch_prefix_for_array(&tokens, name)
                            } else {
                                None
                            };
                            let mut cursor = index + 4;
                            let mut depth = 1usize;
                            while cursor < tokens.len() && depth > 0 {
                                match &tokens[cursor] {
                                    JsToken::Punct('[') => depth += 1,
                                    JsToken::Punct(']') => depth -= 1,
                                    JsToken::String(value) if depth == 1 => {
                                        let normalized = if let Some(prefix) = &fetch_prefix {
                                            normalize_fetch_reference(
                                                source_path,
                                                &format!("{prefix}{value}"),
                                            )
                                        } else if font_file_inventory {
                                            // A font basename is not a path by itself. If no
                                            // static fetch prefix is proven, do not guess its
                                            // repository location.
                                            None
                                        } else {
                                            normalize_asset_reference(source_path, value)
                                        };
                                        if let Some(normalized) = normalized {
                                            output.insert(normalized);
                                        }
                                    }
                                    _ => {}
                                }
                                cursor += 1;
                            }
                        }
                    }
                }
            }
            JsToken::String(value) | JsToken::Template(value) => {
                // HTML attributes and CSS url() inside literal/template content
                // are explicit runtime asset references. Arbitrary prose
                // strings are not treated as paths.
                extract_url_functions(value, output, source_path);
                extract_attributes(
                    value,
                    &[
                        "src=",
                        "href=",
                        "poster=",
                        "data-src=",
                        "xlink:href=",
                        "srcset=",
                    ],
                    output,
                    source_path,
                );
            }
            JsToken::Punct(_) => {}
        }
        index += 1;
    }
}

fn normalize_fetch_reference(source_path: &str, value: &str) -> Option<String> {
    // Browser fetch() resolves against document.baseURI, not the JS file path.
    // Test runners load renderer.html from the repository root.
    if source_path.starts_with("tests/") && !value.starts_with("./") && !value.starts_with("../") {
        return normalize_asset_reference("", value);
    }
    normalize_asset_reference(source_path, value)
}

fn has_known_asset_extension(value: &str) -> bool {
    let filename = value
        .rsplit('/')
        .next()
        .unwrap_or(value)
        .to_ascii_lowercase();
    [
        ".css", ".js", ".mjs", ".cjs", ".json", ".svg", ".png", ".jpg", ".jpeg", ".webp", ".ico",
        ".woff", ".woff2", ".ttf", ".otf", ".gif", ".avif", ".txt", ".html", ".md", ".pdf",
    ]
    .iter()
    .any(|extension| filename.ends_with(extension) && filename.len() > extension.len())
}

fn normalize_module_reference(source_path: &str, value: &str) -> Option<String> {
    // Bare specifiers (for example "playwright/package.json") are resolved by
    // Node's package resolver, not relative to the source file. Only explicit
    // relative or root-relative module paths are local repository assets.
    if !(value.starts_with("./") || value.starts_with("../") || value.starts_with('/')) {
        return None;
    }
    normalize_asset_reference(source_path, value)
}

fn extract_url_functions(text: &str, output: &mut BTreeSet<String>, source_path: &str) {
    let lower = text.to_ascii_lowercase();
    let mut offset = 0usize;
    while let Some(relative) = lower[offset..].find("url(") {
        let start = offset + relative + 4;
        let Some(end_relative) = text[start..].find(')') else {
            break;
        };
        let end = start + end_relative;
        let value = text[start..end]
            .trim()
            .trim_matches(|character| character == '"' || character == '\'');
        if let Some(normalized) = normalize_css_url_reference(source_path, value) {
            output.insert(normalized);
        }
        offset = end + 1;
    }
}

fn extract_attributes(
    text: &str,
    patterns: &[&str],
    output: &mut BTreeSet<String>,
    source_path: &str,
) {
    let lower = text.to_ascii_lowercase();
    for pattern in patterns {
        let mut offset = 0usize;
        while let Some(relative) = lower[offset..].find(pattern) {
            let value_start = offset + relative + pattern.len();
            let mut value_end = value_start;
            let bytes = text.as_bytes();
            while value_end < bytes.len() && bytes[value_end].is_ascii_whitespace() {
                value_end += 1;
            }
            if value_end >= bytes.len() {
                break;
            }
            let quote = bytes[value_end];
            let value = if quote == b'"' || quote == b'\'' {
                value_end += 1;
                let start = value_end;
                while value_end < bytes.len() && bytes[value_end] != quote {
                    value_end += 1;
                }
                let extracted = &text[start..value_end];
                if value_end < bytes.len() {
                    value_end += 1;
                }
                extracted
            } else {
                let start = value_end;
                while value_end < bytes.len()
                    && !bytes[value_end].is_ascii_whitespace()
                    && bytes[value_end] != b'>'
                {
                    value_end += 1;
                }
                &text[start..value_end]
            };

            if *pattern == "srcset=" {
                for candidate in value.split(',') {
                    let url = candidate.split_whitespace().next().unwrap_or("");
                    if let Some(normalized) = normalize_asset_reference(source_path, url) {
                        output.insert(normalized);
                    }
                }
            } else if let Some(normalized) = normalize_asset_reference(source_path, value) {
                output.insert(normalized);
            }
            offset = value_end.max(offset + relative + pattern.len());
        }
    }
}

fn extract_json_string_property(text: &str, property: &str) -> Vec<String> {
    let tokens = tokenize_javascript(text);
    let mut values = Vec::new();
    for window in tokens.windows(3) {
        if let [JsToken::String(key), JsToken::Punct(':'), JsToken::String(value)] = window {
            if key == property {
                values.push(value.clone());
            }
        }
    }
    values
}

fn extract_quoted_strings(text: &str) -> Vec<String> {
    let bytes = text.as_bytes();
    let mut values = Vec::new();
    let mut index = 0usize;
    while index < bytes.len() {
        let quote = bytes[index];
        if quote != b'"' && quote != b'\'' {
            index += 1;
            continue;
        }
        index += 1;
        let start = index;
        let mut escaped = false;
        while index < bytes.len() {
            let current = bytes[index];
            if escaped {
                escaped = false;
            } else if current == b'\\' {
                escaped = true;
            } else if current == quote {
                let value = text[start..index].replace("\\/", "/");
                values.push(value);
                index += 1;
                break;
            }
            index += 1;
        }
    }
    values
}

fn is_path_candidate(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    if lower.trim() != lower || lower.ends_with('/') || lower.is_empty() {
        return false;
    }
    let known_extension = has_known_asset_extension(value);
    lower.starts_with("./")
        || lower.starts_with("../")
        || lower.starts_with('/')
        || lower.starts_with("//")
        || lower.contains("://")
        || [
            "assets/",
            "fonts/",
            "images/",
            "decorations/",
            "presets/",
            "backgrounds/",
            "ornaments/",
        ]
        .iter()
        .any(|prefix| lower.contains(prefix))
        || known_extension
}

fn normalize_asset_reference(source_path: &str, value: &str) -> Option<String> {
    normalize_asset_reference_impl(source_path, value, false)
}

fn normalize_css_url_reference(source_path: &str, value: &str) -> Option<String> {
    // CSS url("font name.woff2") may legally reference a local file whose
    // basename contains spaces. Keep this exception scoped to an explicit
    // url() token; generic JavaScript strings still reject whitespace.
    normalize_asset_reference_impl(source_path, value, true)
}

fn normalize_asset_reference_impl(
    source_path: &str,
    value: &str,
    allow_whitespace: bool,
) -> Option<String> {
    let value = value
        .trim()
        .trim_matches(|character| character == '"' || character == '\'');
    if value.is_empty()
        || value.starts_with('#')
        || value.starts_with("data:")
        || value.starts_with("blob:")
        || value.starts_with("javascript:")
        || value.starts_with("mailto:")
    {
        return None;
    }
    if value.starts_with("http://")
        || value.starts_with("https://")
        || value.starts_with("//")
        || value.contains("://")
    {
        return Some(format!("external:{value}"));
    }
    if (!allow_whitespace && value.chars().any(char::is_whitespace))
        || value.chars().any(char::is_control)
        || (value.contains('$') && value.contains('{'))
        || value.contains('}')
        || value.contains(" + ")
        || !is_path_candidate(value)
    {
        return None;
    }

    let path_only = value
        .split(['?', '#'])
        .next()
        .unwrap_or("")
        .replace('\\', "/");
    if path_only.is_empty() || path_only.ends_with('/') {
        return None;
    }

    let mut components = Vec::<String>::new();
    if !path_only.starts_with('/') {
        if let Some(parent) = Path::new(source_path).parent() {
            for component in parent.to_string_lossy().split('/') {
                if !component.is_empty() && component != "." {
                    components.push(component.to_string());
                }
            }
        }
    }
    for component in path_only.trim_start_matches('/').split('/') {
        match component {
            "" | "." => {}
            ".." => {
                if components.last().is_some_and(|last| last != "..") {
                    components.pop();
                } else {
                    components.push("..".to_string());
                }
            }
            value => components.push(value.to_string()),
        }
    }
    if components.is_empty() {
        None
    } else {
        Some(format!("local:{}", components.join("/")))
    }
}

fn resolve_local_asset_path(path: &str, known_paths: &BTreeSet<String>) -> Option<String> {
    if known_paths.contains(path) {
        return Some(path.to_string());
    }
    // Node-style static imports commonly omit the .js/.mjs/.cjs suffix.
    // Resolve only a unique same-profile match; never guess between variants.
    if Path::new(path).extension().is_none() {
        let matches: Vec<String> = ["js", "mjs", "cjs"]
            .iter()
            .map(|extension| format!("{path}.{extension}"))
            .filter(|candidate| known_paths.contains(candidate))
            .collect();
        if matches.len() == 1 {
            return matches.into_iter().next();
        }
    }
    // Font catalogs store basenames while the application loads them from
    // fonts/<file>. Resolve only unique matches within the same profile.
    if !path.contains('/') {
        let directories = [
            "fonts",
            "assets",
            "images",
            "decorations",
            "presets",
            "backgrounds",
            "ornaments",
        ];
        let matches: Vec<String> = directories
            .iter()
            .map(|directory| format!("{directory}/{path}"))
            .filter(|candidate| known_paths.contains(candidate))
            .collect();
        if matches.len() == 1 {
            return matches.into_iter().next();
        }
    }
    None
}

fn validate_asset_dependencies(records: &mut [FileRecord]) {
    let mut paths_by_profile = BTreeMap::<String, BTreeSet<String>>::new();
    for record in records.iter() {
        paths_by_profile
            .entry(record.owning_profile.clone())
            .or_default()
            .insert(record.source_path.clone());
    }
    for record in records.iter_mut() {
        let Some(known_paths) = paths_by_profile.get(&record.owning_profile) else {
            record.asset_dependency_status = "profile-path-index-missing".to_string();
            continue;
        };
        let mut resolved = BTreeSet::new();
        let mut unresolved = BTreeSet::new();
        for dependency in &record.asset_dependencies {
            if let Some(path) = dependency.strip_prefix("local:") {
                if let Some(resolved_path) = resolve_local_asset_path(path, known_paths) {
                    resolved.insert(format!("local:{resolved_path}"));
                } else {
                    unresolved.insert(dependency.clone());
                }
            }
        }
        record.resolved_asset_dependencies = resolved.into_iter().collect();
        record.unresolved_asset_dependencies = unresolved.into_iter().collect();
        let has_external = record
            .asset_dependencies
            .iter()
            .any(|dependency| dependency.starts_with("external:"));
        record.asset_dependency_status = if has_external {
            "contains-external".to_string()
        } else if !record.unresolved_asset_dependencies.is_empty() {
            "unresolved-local".to_string()
        } else if record.asset_dependencies.is_empty() {
            "no-references-detected".to_string()
        } else {
            "all-local-resolved".to_string()
        };
    }
}

fn collect_files(root: &Path, directory: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
    let mut entries = fs::read_dir(directory)?.collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(|entry| entry.file_name());
    for entry in entries {
        let path = entry.path();
        let file_type = entry.file_type()?;
        if file_type.is_symlink() {
            continue;
        }
        if file_type.is_dir() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if matches!(
                name.as_ref(),
                ".git"
                    | "node_modules"
                    | "target"
                    | ".cache"
                    | "coverage"
                    | "playwright-report"
                    | "test-results"
            ) {
                continue;
            }
            collect_files(root, &path, files)?;
        } else if file_type.is_file() {
            let relative = path
                .strip_prefix(root)
                .map_err(|_| {
                    io::Error::new(io::ErrorKind::InvalidData, "path escaped source root")
                })?
                .to_string_lossy()
                .replace('\\', "/");
            if is_relevant(&relative) {
                files.push(path);
            }
        }
    }
    Ok(())
}

fn is_relevant(path: &str) -> bool {
    let extension = Path::new(path)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap_or("")
        .to_ascii_lowercase();
    matches!(
        extension.as_str(),
        "js" | "mjs"
            | "cjs"
            | "html"
            | "htm"
            | "css"
            | "json"
            | "svg"
            | "png"
            | "jpg"
            | "jpeg"
            | "webp"
            | "gif"
            | "avif"
            | "bmp"
            | "ico"
            | "xml"
            | "webmanifest"
            | "jsonc"
            | "eot"
            | "woff"
            | "woff2"
            | "ttf"
            | "otf"
            | "txt"
            | "md"
            | "pdf"
            | "yml"
            | "yaml"
            | "toml"
    )
}

fn status_for(profile: &str, kind: &str) -> &'static str {
    if profile == "requirements-docs" && kind == "documentation" {
        "spec-only"
    } else if kind == "font" {
        "blocked-license"
    } else if matches!(kind, "settings" | "data" | "asset" | "fixture" | "corpus") {
        "verified-data"
    } else {
        "historical-only"
    }
}

fn classify(path: &str) -> String {
    let extension = Path::new(path)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap_or("")
        .to_ascii_lowercase();
    let lower = path.to_ascii_lowercase();
    if matches!(extension.as_str(), "woff" | "woff2" | "ttf" | "otf" | "eot") {
        "font".to_string()
    } else if matches!(
        extension.as_str(),
        "svg" | "png" | "jpg" | "jpeg" | "webp" | "gif" | "avif" | "bmp" | "ico"
    ) {
        "asset".to_string()
    } else if matches!(extension.as_str(), "js" | "mjs" | "cjs") {
        "javascript".to_string()
    } else if matches!(extension.as_str(), "html" | "htm") {
        "dom-contract".to_string()
    } else if extension == "css" {
        "style".to_string()
    } else if matches!(extension.as_str(), "json" | "jsonc" | "webmanifest") {
        if lower.contains("setting") || lower.contains("preset") || lower.contains("config") {
            "settings".to_string()
        } else {
            "data".to_string()
        }
    } else if extension == "xml" {
        "data".to_string()
    } else if extension == "txt" {
        if lower.contains("fixture") || lower.contains("/tests/") || lower.starts_with("tests/") {
            "fixture".to_string()
        } else {
            "corpus".to_string()
        }
    } else if matches!(extension.as_str(), "md" | "pdf") {
        "documentation".to_string()
    } else if matches!(extension.as_str(), "yml" | "yaml") {
        "test-infrastructure".to_string()
    } else if extension == "toml" {
        "build-config".to_string()
    } else {
        "other".to_string()
    }
}

fn render_manifest(sources: &[SourceSpec], records: &[FileRecord]) -> String {
    let mut output = String::new();
    output.push_str(
        "{\n  \"schemaVersion\": 1,\n  \"generatedBy\": \"xtask compat-scan\",\n  \"sources\": [\n",
    );
    for (index, source) in sources.iter().enumerate() {
        if index > 0 {
            output.push_str(",\n");
        }
        output.push_str("    {\"repository\": ");
        output.push_str(&json_string(&source.repository));
        output.push_str(", \"commit\": ");
        output.push_str(&json_string(&source.commit));
        output.push_str(", \"owningProfile\": ");
        output.push_str(&json_string(&source.profile));
        output.push('}');
    }
    output.push_str("\n  ],\n  \"records\": [\n");
    for (index, record) in records.iter().enumerate() {
        if index > 0 {
            output.push_str(",\n");
        }
        output.push_str("    {\n");
        push_field(&mut output, "compatId", &record.compat_id, true);
        push_field(&mut output, "kind", &record.kind, true);
        push_field(&mut output, "repository", &record.repository, true);
        push_field(&mut output, "commit", &record.commit, true);
        push_field(&mut output, "sourcePath", &record.source_path, true);
        push_field(&mut output, "sourceSha256", &record.source_sha256, true);
        push_field(
            &mut output,
            "externalIdentifier",
            &record.external_identifier,
            true,
        );
        push_field(&mut output, "legacyName", &record.legacy_name, true);
        push_field(&mut output, "owningProfile", &record.owning_profile, true);
        push_field(&mut output, "status", &record.status, true);
        push_field(
            &mut output,
            "parserOrConsumer",
            &record.parser_or_consumer,
            true,
        );
        output.push_str("      \"assetDependencies\": [");
        for (dependency_index, dependency) in record.asset_dependencies.iter().enumerate() {
            if dependency_index > 0 {
                output.push_str(", ");
            }
            output.push_str(&json_string(dependency));
        }
        output.push_str("],\n");
        output.push_str("      \"resolvedAssetDependencies\": [");
        for (dependency_index, dependency) in record.resolved_asset_dependencies.iter().enumerate()
        {
            if dependency_index > 0 {
                output.push_str(", ");
            }
            output.push_str(&json_string(dependency));
        }
        output.push_str("],\n");
        output.push_str("      \"unresolvedAssetDependencies\": [");
        for (dependency_index, dependency) in
            record.unresolved_asset_dependencies.iter().enumerate()
        {
            if dependency_index > 0 {
                output.push_str(", ");
            }
            output.push_str(&json_string(dependency));
        }
        output.push_str("],\n");
        push_field(
            &mut output,
            "assetDependencyStatus",
            &record.asset_dependency_status,
            true,
        );
        output.push_str("      \"parityFixtures\": [");
        for (fixture_index, fixture) in record.parity_fixtures.iter().enumerate() {
            if fixture_index > 0 {
                output.push_str(", ");
            }
            output.push_str(&json_string(fixture));
        }
        output.push_str("],\n");
        push_field(&mut output, "rustOwner", &record.rust_owner, true);
        push_field(&mut output, "migrationRule", &record.migration_rule, true);
        output.push_str("      \"testIds\": [");
        for (test_index, test_id) in record.test_ids.iter().enumerate() {
            if test_index > 0 {
                output.push_str(", ");
            }
            output.push_str(&json_string(test_id));
        }
        output.push_str("],\n");
        push_field(
            &mut output,
            "acceptanceCriteria",
            &record.acceptance_criteria,
            true,
        );
        push_field(
            &mut output,
            "verificationLevel",
            &record.verification_level,
            true,
        );
        output.push_str("      \"verificationEvidence\": [");
        for (evidence_index, evidence) in record.verification_evidence.iter().enumerate() {
            if evidence_index > 0 {
                output.push_str(", ");
            }
            output.push_str(&json_string(evidence));
        }
        output.push_str("],\n");
        push_field(&mut output, "notes", &record.notes, false);
        output.push_str("    }");
    }
    output.push_str("\n  ]\n}\n");
    output
}

fn push_field(output: &mut String, name: &str, value: &str, comma: bool) {
    output.push_str("      ");
    output.push_str(&json_string(name));
    output.push_str(": ");
    output.push_str(&json_string(value));
    if comma {
        output.push(',');
    }
    output.push('\n');
}

fn json_string(value: &str) -> String {
    let mut output = String::from("\"");
    for character in value.chars() {
        match character {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => {
                output.push('\\');
                output.push('t');
            }
            character if character.is_control() => {
                output.push_str(&format!("\\u{:04x}", character as u32));
            }
            character => output.push(character),
        }
    }
    output.push('"');
    output
}

fn hex(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push_str(&format!("{byte:02x}"));
    }
    output
}

fn sha256(input: &[u8]) -> [u8; 32] {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];
    let mut state: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];
    let bit_length = (input.len() as u64).wrapping_mul(8);
    let mut padded = input.to_vec();
    padded.push(0x80);
    while padded.len() % 64 != 56 {
        padded.push(0);
    }
    padded.extend_from_slice(&bit_length.to_be_bytes());

    for chunk in padded.chunks_exact(64) {
        let mut words = [0u32; 64];
        for (index, word) in words.iter_mut().take(16).enumerate() {
            let offset = index * 4;
            *word = u32::from_be_bytes([
                chunk[offset],
                chunk[offset + 1],
                chunk[offset + 2],
                chunk[offset + 3],
            ]);
        }
        for index in 16..64 {
            let x = words[index - 15];
            let y = words[index - 2];
            let sigma0 = x.rotate_right(7) ^ x.rotate_right(18) ^ (x >> 3);
            let sigma1 = y.rotate_right(17) ^ y.rotate_right(19) ^ (y >> 10);
            words[index] = words[index - 16]
                .wrapping_add(sigma0)
                .wrapping_add(words[index - 7])
                .wrapping_add(sigma1);
        }

        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = state;
        for index in 0..64 {
            let sum1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let choice = (e & f) ^ (!e & g);
            let temp1 = h
                .wrapping_add(sum1)
                .wrapping_add(choice)
                .wrapping_add(K[index])
                .wrapping_add(words[index]);
            let sum0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let majority = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = sum0.wrapping_add(majority);
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }
        state[0] = state[0].wrapping_add(a);
        state[1] = state[1].wrapping_add(b);
        state[2] = state[2].wrapping_add(c);
        state[3] = state[3].wrapping_add(d);
        state[4] = state[4].wrapping_add(e);
        state[5] = state[5].wrapping_add(f);
        state[6] = state[6].wrapping_add(g);
        state[7] = state[7].wrapping_add(h);
    }

    let mut digest = [0u8; 32];
    for (index, word) in state.iter().enumerate() {
        digest[index * 4..index * 4 + 4].copy_from_slice(&word.to_be_bytes());
    }
    digest
}

#[cfg(test)]
mod tests {
    use super::{
        apply_runner_evidence_rows, classify, extract_asset_dependencies,
        has_static_repository_root_binding, hex, json_string, normalize_asset_reference,
        ownership_for, parse_runner_evidence_line, resolve_local_asset_path, sha256, status_for,
        tokenize_javascript, FileRecord, SourceSpec,
    };
    use std::collections::BTreeSet;
    use std::path::PathBuf;

    #[test]
    fn sha256_matches_standard_empty_and_abc_vectors() {
        assert_eq!(
            hex(&sha256(b"")),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(
            hex(&sha256(b"abc")),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn classification_keeps_fonts_and_fixtures_distinct() {
        assert_eq!(classify("fonts/example.woff2"), "font");
        assert_eq!(classify("tests/fixtures/layout.txt"), "fixture");
        assert_eq!(classify("setting_sets/תלמוד.json"), "settings");
        assert_eq!(classify("decorations.js"), "javascript");
    }

    #[test]
    fn decorations_are_owned_by_the_engine_core() {
        let source = SourceSpec {
            repository: "SUGINACH/Tora_Book_Typesetter_Orange".to_string(),
            commit: "0b0ee279464a3db14672cd3f4051275aba5985da".to_string(),
            profile: "legacy-orange-v23".to_string(),
            root: PathBuf::from("legacy/orange"),
        };
        let (_, owner, migration, fixtures) =
            ownership_for(&source, "decorations.js", "javascript");
        assert_eq!(owner, "engine-core::decorations");
        assert!(migration.contains("geometry"));
        assert_eq!(fixtures.len(), 1);
    }

    #[test]
    fn v21_and_v23_profiles_keep_distinct_fixture_identities() {
        let gold = SourceSpec {
            repository: "SUGINACH/Tora_Book_Typesetter_Gold".to_string(),
            commit: "a444d34ccad30fd5ed0d6fd3873adbe9c23cb7e0".to_string(),
            profile: "legacy-gold-v21".to_string(),
            root: PathBuf::from("legacy/gold"),
        };
        let orange = SourceSpec {
            repository: "SUGINACH/Tora_Book_Typesetter_Orange".to_string(),
            commit: "0b0ee279464a3db14672cd3f4051275aba5985da".to_string(),
            profile: "legacy-orange-v23".to_string(),
            root: PathBuf::from("legacy/orange"),
        };
        let (_, _, _, gold_fixtures) = ownership_for(&gold, "typesetter.js", "javascript");
        let (_, _, _, orange_fixtures) = ownership_for(&orange, "typesetter.js", "javascript");
        assert_ne!(gold.profile, orange.profile);
        assert_ne!(gold.commit, orange.commit);
        assert_ne!(gold_fixtures, orange_fixtures);
        assert!(gold_fixtures[0].contains("Tora_Book_Typesetter_Gold"));
        assert!(orange_fixtures[0].contains("Tora_Book_Typesetter_Orange"));
    }

    #[test]
    fn requirements_documents_are_not_classified_as_executable_code() {
        let source = SourceSpec {
            repository: "SUGINACH/Typesetter_doc".to_string(),
            commit: "534b8f4e0ba9f5cb8abe9e3ad888904a217910cd".to_string(),
            profile: "requirements-docs".to_string(),
            root: PathBuf::from("legacy/docs"),
        };
        let (_, owner, migration, _) = ownership_for(
            &source,
            "shared/specs/Software_specification.md",
            "documentation",
        );
        assert_eq!(owner, "docs/requirements");
        assert!(migration.contains("spec-only"));
    }

    #[test]
    fn runner_evidence_rows_require_exactly_eight_tab_separated_fields() {
        let valid =
            "legacy-orange-v23\tcommit\trunner\turl\toffline-smoke-tested\tfile.js\tsha\tloaded";
        let fields = parse_runner_evidence_line(valid);
        assert!(fields.is_some(), "valid runner evidence row should parse");
        let fields = fields.unwrap_or([""; 8]);
        assert_eq!(fields[0], "legacy-orange-v23");
        assert_eq!(fields[7], "loaded");
        assert!(parse_runner_evidence_line("only\ttwo").is_none());
    }

    fn runner_record() -> FileRecord {
        FileRecord {
            compat_id: "legacy-orange-v23:javascript:typesetter.js".to_string(),
            kind: "javascript".to_string(),
            repository: "SUGINACH/Tora_Book_Typesetter_Orange".to_string(),
            commit: "0b0ee279464a3db14672cd3f4051275aba5985da".to_string(),
            source_path: "typesetter.js".to_string(),
            source_sha256: "a".repeat(64),
            external_identifier: "typesetter.js".to_string(),
            legacy_name: "typesetter.js".to_string(),
            owning_profile: "legacy-orange-v23".to_string(),
            status: "historical-only".to_string(),
            parser_or_consumer: "legacy-layout-engine".to_string(),
            asset_dependencies: Vec::new(),
            resolved_asset_dependencies: Vec::new(),
            unresolved_asset_dependencies: Vec::new(),
            asset_dependency_status: "no-references-detected".to_string(),
            parity_fixtures: Vec::new(),
            rust_owner: "engine-core".to_string(),
            migration_rule: "profile-scoped".to_string(),
            test_ids: Vec::new(),
            acceptance_criteria: "exact".to_string(),
            verification_level: "scanned-hash-only".to_string(),
            verification_evidence: Vec::new(),
            notes: String::new(),
        }
    }

    #[test]
    fn runner_evidence_requires_exact_profile_commit_path_and_hash() {
        let mut records = vec![runner_record()];
        let row = format!(
            "legacy-orange-v23\t0b0ee279464a3db14672cd3f4051275aba5985da\trunner-sha\thttps://example.test/run\toffline-smoke-tested\ttypesetter.js\t{}\tloaded",
            "a".repeat(64)
        );
        assert!(apply_runner_evidence_rows(&mut records, &row).is_ok());
        assert_eq!(records[0].status, "verified-executable");
        assert_eq!(records[0].verification_level, "offline-smoke-tested");
        assert_eq!(records[0].verification_evidence.len(), 1);

        let mut mismatched = vec![runner_record()];
        let bad_row = row.replace(&"a".repeat(64), &"b".repeat(64));
        assert!(apply_runner_evidence_rows(&mut mismatched, &bad_row).is_err());
        assert_eq!(mismatched[0].status, "historical-only");
    }

    #[test]
    fn font_catalog_basenames_resolve_to_unique_profile_font_paths() {
        let paths = BTreeSet::from(["fonts/Abir-Black.woff2".to_string()]);
        assert_eq!(
            resolve_local_asset_path("Abir-Black.woff2", &paths).as_deref(),
            Some("fonts/Abir-Black.woff2")
        );
    }

    #[test]
    fn extensionless_static_imports_resolve_only_unique_javascript_modules() {
        let unique = BTreeSet::from(["tests/lib/harness.js".to_string()]);
        assert_eq!(
            resolve_local_asset_path("tests/lib/harness", &unique).as_deref(),
            Some("tests/lib/harness.js")
        );
        let ambiguous = BTreeSet::from([
            "tests/lib/harness.js".to_string(),
            "tests/lib/harness.mjs".to_string(),
        ]);
        assert_eq!(
            resolve_local_asset_path("tests/lib/harness", &ambiguous),
            None
        );
    }

    #[test]
    fn ambiguous_bare_asset_names_are_not_guessed() {
        let paths = BTreeSet::from(["fonts/logo.svg".to_string(), "assets/logo.svg".to_string()]);
        assert_eq!(resolve_local_asset_path("logo.svg", &paths), None);
    }

    #[test]
    fn dynamic_and_non_path_strings_are_not_static_asset_edges() {
        assert_eq!(
            normalize_asset_reference("app.js", "fonts/${encodeURIComponent(font.file)}"),
            None
        );
        assert_eq!(normalize_asset_reference("app.js", "a.href"), None);
        assert_eq!(
            normalize_asset_reference("app.js", "מתוך ornaments_left_right.json"),
            None
        );
        assert_eq!(
            normalize_asset_reference("app.js", "Abir-Black.woff2").as_deref(),
            Some("local:Abir-Black.woff2")
        );
    }

    #[test]
    fn javascript_asset_extraction_uses_static_syntax_and_asset_inventories() {
        let javascript = br#"
            // "comment-only.svg"
            const prose = "this sentence mentions sample.json";
            const markup = `<svg><image href="icons/real.svg"></image></svg>`;
            const PRIORITY_FONT_FILES = ["Alpha-Black.woff2", "Book-Regular.woff2"];
            const assetsToLoad = [{ file: "ornaments.json", type: "ornaments" }];
            for (const fontFile of PRIORITY_FONT_FILES) {
                await fetch("fonts/" + fontFile);
            }
            fetch("fonts/fonts.json");
            import "./side-effect.css";
            import module from "./module.js";
            const helper = require("./helper.cjs");
            const worker = new URL("./worker.mjs", import.meta.url);
        "#;
        let dependencies = extract_asset_dependencies("app.js", "javascript", javascript);
        for expected in [
            "local:icons/real.svg",
            "local:fonts/Alpha-Black.woff2",
            "local:fonts/Book-Regular.woff2",
            "local:ornaments.json",
            "local:fonts/fonts.json",
            "local:side-effect.css",
            "local:module.js",
            "local:helper.cjs",
            "local:worker.mjs",
        ] {
            assert!(
                dependencies.iter().any(|dependency| dependency == expected),
                "missing expected dependency {expected:?} in {dependencies:?}"
            );
        }
        for rejected in [
            "local:comment-only.svg",
            "local:sample.json",
            "local:fonts",
            "local:Alpha-Black.woff2",
            "local:Book-Regular.woff2",
        ] {
            assert!(
                !dependencies.iter().any(|dependency| dependency == rejected),
                "unexpected dependency {rejected:?} in {dependencies:?}"
            );
        }
    }

    #[test]
    fn path_join_and_playwright_navigation_preserve_static_files() {
        let javascript = br#"
            const root = path.resolve(__dirname, '..');
            const checker = fs.readFileSync(path.join(__dirname, '..', 'layout', 'check_layout.js'), 'utf8');
            const defaults = fs.readFileSync(path.join(root, 'default_settings.json'), 'utf8');
            await page.goto(baseUrl + '/renderer.html', { waitUntil: 'load' });
            const out = val('--out') || path.join(__dirname, 'results', 'latest.json');
            const outputDir = path.join(__dirname, '..', 'issue-reports');
        "#;
        let dependencies =
            extract_asset_dependencies("tests/layout/run-suite.js", "javascript", javascript);
        assert!(dependencies.contains(&"local:tests/layout/check_layout.js".to_string()));
        assert!(dependencies.contains(&"local:default_settings.json".to_string()));
        assert!(dependencies.contains(&"local:renderer.html".to_string()));
        assert!(!dependencies.contains(&"local:tests/layout/results/latest.json".to_string()));
        assert!(!dependencies.contains(&"local:issue-reports".to_string()));
    }

    #[test]
    fn path_join_ignores_runtime_root_parameters() {
        let javascript = br#"
            async function runTree(browser, root, plan) {
                const settings = fs.readFileSync(path.join(root, 'default_settings.json'), 'utf8');
            }
            function hasFonts(root) {
                return fs.existsSync(path.join(root, 'fonts', 'fonts.json'));
            }
        "#;
        let dependencies =
            extract_asset_dependencies("scripts/layout-ab.mjs", "javascript", javascript);
        assert!(dependencies.is_empty());
    }

    #[test]
    fn path_join_does_not_reuse_outer_root_for_shadowing_function_parameter() {
        let javascript = br#"
            const root = process.cwd();
            function loadExternalTree(root) {
                return fs.readFileSync(path.join(root, 'default_settings.json'), 'utf8');
            }
        "#;
        let dependencies = extract_asset_dependencies("tools/runner.js", "javascript", javascript);
        assert!(dependencies.is_empty());
    }

    #[test]
    fn path_join_resolves_process_working_directory_root() {
        let javascript = br#"
            const root = process.cwd();
            fs.readFileSync(path.join(root, 'selfhost-fonts', 'google-fonts-selfhost.css'), 'utf8');
        "#;
        let dependencies =
            extract_asset_dependencies("tools/geometry-audit-cli.js", "javascript", javascript);
        assert!(
            dependencies.contains(&"local:selfhost-fonts/google-fonts-selfhost.css".to_string())
        );
    }

    #[test]
    fn path_join_resolves_directory_of_current_script() {
        let javascript = br#"
            const SCRIPTS_DIR = path.dirname(__filename);
            const ENHANCED_SCRIPT = path.join(SCRIPTS_DIR, 'test-chapters-enhanced.js');
        "#;
        let dependencies =
            extract_asset_dependencies("scripts/regression-test.js", "javascript", javascript);
        assert!(
            dependencies.contains(&"local:scripts/test-chapters-enhanced.js".to_string()),
            "unexpected extracted dependencies: {dependencies:?}"
        );
    }

    #[test]
    fn path_join_resolves_declared_repository_root_and_imported_harness_root() {
        let javascript = br#"
            const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
            fs.readFileSync(path.join(ROOT, 'default_settings.json'), 'utf8');
        "#;
        let dependencies =
            extract_asset_dependencies("scripts/runner.mjs", "javascript", javascript);
        assert!(dependencies.contains(&"local:default_settings.json".to_string()));

        let harness = br#"
            const { root, startServer } = require('./lib/harness');
            fs.readFileSync(path.join(root, 'renderer.html'), 'utf8');
        "#;
        let harness_text = match std::str::from_utf8(harness) {
            Ok(value) => value,
            Err(error) => panic!("test fixture must be UTF-8: {error}"),
        };
        let tokens = tokenize_javascript(harness_text);
        let join_index = tokens
            .iter()
            .position(|token| token == &super::JsToken::Identifier("join".to_string()))
            .unwrap_or(tokens.len());
        assert!(
            join_index < tokens.len(),
            "path.join token not found: {tokens:?}"
        );
        assert!(
            has_static_repository_root_binding(&tokens, "root", join_index),
            "the imported harness root binding was not recognized: {tokens:?}"
        );
        assert_eq!(
            normalize_asset_reference("", "renderer.html"),
            Some("local:renderer.html".to_string())
        );
        let harness_dependencies =
            extract_asset_dependencies("scripts/runner.mjs", "javascript", harness);
        assert!(
            harness_dependencies.contains(&"local:renderer.html".to_string()),
            "unexpected extracted dependencies: {harness_dependencies:?}"
        );
    }

    #[test]
    fn font_picker_metadata_and_duplicate_catalog_do_not_create_file_dependencies() {
        let json = br#"[{"family":"Example Hebrew","file":"Example-Regular.woff2","format":"woff2"}]"#;

        for path in ["fonts-extended.json", "fonts-priority.json", "fonts/fonts/fonts.json"] {
            let dependencies = extract_asset_dependencies(path, "data", json);
            assert!(
                dependencies.is_empty(),
                "{path} is metadata or an unreferenced duplicate, not a file-loading manifest: {dependencies:?}"
            );
        }

        let active_catalog =
            extract_asset_dependencies("fonts/fonts.json", "data", json);
        assert!(
            active_catalog.contains(&"local:fonts/Example-Regular.woff2".to_string()),
            "the active font loader's catalog must retain its real font dependency: {active_catalog:?}"
        );
    }

    #[test]
    fn write_file_sync_path_join_is_an_output_not_a_dependency() {
        let javascript = br#"
            fs.writeFileSync(path.join(__dirname, 'fuzz_last.json'), JSON.stringify(summary));
        "#;
        let dependencies =
            extract_asset_dependencies("archive/v20.1-tools/fuzz.js", "javascript", javascript);
        assert!(
            !dependencies.contains(&"local:archive/v20.1-tools/fuzz_last.json".to_string()),
            "generated output was misclassified as an input dependency: {dependencies:?}"
        );
    }

    #[test]
    fn read_file_sync_static_fallback_is_a_dependency() {
        let javascript = br#"
            const source = fs.readFileSync(process.argv[2] || 'typesetter.js', 'utf8');
        "#;
        let dependencies =
            extract_asset_dependencies("archive/v20.2-tip/verify_cut.js", "javascript", javascript);
        assert!(dependencies.contains(&"local:archive/v20.2-tip/typesetter.js".to_string()));
    }

    #[test]
    fn top_level_regex_literals_with_quotes_do_not_hide_later_fetches() {
        let javascript = br#"
            const svgText = '<svg></svg>';
            const safe = svgText.replace(/preserveAspectRatio="[^"]*"/, 'preserveAspectRatio="none"');
            fetch('styles.css');
        "#;
        let dependencies =
            extract_asset_dependencies("archive/v20.2-tip/script.js", "javascript", javascript);
        assert!(dependencies.contains(&"local:archive/v20.2-tip/styles.css".to_string()));
    }

    #[test]
    fn nested_template_literals_do_not_hide_later_static_fetches() {
        let javascript = br#"
            const anchorRow = (kind !== 'textStyle')
              ? `<div>${EXT_ANCHOR_OPTIONS.map(([v,l])=>`<option value="${v}">${l}</option>`).join('')}</div>`
              : '';
            panel.innerHTML = `
              <span title="${String(name).replace(/"/g, '&quot;')}">${icon} ${name}</span>
              ${anchorRow}
            `;
            fetch('styles.css');
        "#;
        let dependencies =
            extract_asset_dependencies("archive/v20.2-tip/script.js", "javascript", javascript);
        assert!(dependencies.contains(&"local:archive/v20.2-tip/styles.css".to_string()));
    }

    #[test]
    fn browser_test_fetch_paths_resolve_from_document_root() {
        let javascript = br#"
            await page.evaluate(async () => {
                const defaults = await (await fetch('default_settings.json')).json();
            });
        "#;
        let dependencies = extract_asset_dependencies(
            "tests/features/20-settings-io.test.js",
            "javascript",
            javascript,
        );
        assert!(dependencies.contains(&"local:default_settings.json".to_string()));
        assert!(!dependencies.contains(&"local:tests/features/default_settings.json".to_string()));
    }

    #[test]
    fn browser_fetch_concatenation_with_encoded_static_filename_is_an_asset() {
        let javascript = r#"
            const set = await (await fetch(
                'setting_sets/' + encodeURIComponent('קונטרס.json')
            )).json();
        "#;
        let dependencies = extract_asset_dependencies(
            "tests/features/20-settings-io.test.js",
            "javascript",
            javascript.as_bytes(),
        );
        assert!(dependencies.contains(&"local:setting_sets/קונטרס.json".to_string()));
        assert!(
            !dependencies.contains(&"local:tests/features/setting_sets/קונטרס.json".to_string())
        );
    }

    #[test]
    fn browser_fetch_static_template_is_an_asset_but_dynamic_template_is_not() {
        let javascript = r#"
            fetch(`setting_sets/קונטרס.json`);
            fetch(`setting_sets/${selectedPreset}`);
        "#;
        let dependencies = extract_asset_dependencies(
            "tests/features/20-settings-io.test.js",
            "javascript",
            javascript.as_bytes(),
        );
        assert!(dependencies.contains(&"local:setting_sets/קונטרס.json".to_string()));
        assert_eq!(dependencies.len(), 1);
    }

    #[test]
    fn browser_fetch_dynamic_concatenation_is_not_guessed() {
        let javascript = r#"
            fetch('setting_sets/' + encodeURIComponent(selectedPreset));
        "#;
        let dependencies = extract_asset_dependencies(
            "tests/features/20-settings-io.test.js",
            "javascript",
            javascript.as_bytes(),
        );
        assert!(dependencies.is_empty());
    }

    #[test]
    fn direct_dom_href_assignment_is_an_asset_reference() {
        let javascript = br#"
            const link = document.createElement('link');
            link.href = 'fonts-extended.css';
        "#;
        let dependencies = extract_asset_dependencies("app.js", "javascript", javascript);
        assert!(dependencies.contains(&"local:fonts-extended.css".to_string()));
    }

    #[test]
    fn explicit_html_text_markdown_and_pdf_paths_are_candidates() {
        for path in [
            "renderer.html",
            "very_long_test_text.txt",
            "notes.md",
            "input.pdf",
        ] {
            assert!(
                normalize_asset_reference("", path).is_some(),
                "explicit file path was rejected: {path}"
            );
        }
    }

    #[test]
    fn extension_only_and_directory_fragments_are_not_asset_paths() {
        for value in [".json", ".svg", "scripts/.css", "fonts/", "assets/"] {
            assert_eq!(
                normalize_asset_reference("app.js", value),
                None,
                "must reject non-file path fragment {value:?}"
            );
        }
        assert_eq!(
            normalize_asset_reference("app.js", "fonts/Book-Regular.woff2").as_deref(),
            Some("local:fonts/Book-Regular.woff2")
        );
    }

    #[test]
    fn generated_report_outputs_do_not_become_runtime_asset_edges() {
        let dependencies = extract_asset_dependencies(
            "reports/run-1/layout-issue-screenshot-manifest.json",
            "data",
            br#"{"reportPdf":"artifacts/report.pdf"}"#,
        );
        assert!(dependencies.is_empty());
    }

    #[test]
    fn fonts_remain_blocked_until_license_review() {
        assert_eq!(status_for("legacy-orange-v23", "font"), "blocked-license");
        assert_eq!(status_for("legacy-orange-v23", "asset"), "verified-data");
        assert_eq!(
            status_for("requirements-docs", "documentation"),
            "spec-only"
        );
    }

    #[test]
    fn bare_javascript_module_specifiers_are_not_local_assets() {
        let javascript = br#"
            const playwright = require("playwright/package.json");
            import packagePart from "some-package/subpath";
            const local = require("./local.js");
            fetch("./assets/catalog.json");
        "#;
        let dependencies = extract_asset_dependencies("tools/runner.js", "javascript", javascript);
        assert!(!dependencies
            .iter()
            .any(|value| value.contains("playwright")));
        assert!(!dependencies
            .iter()
            .any(|value| value.contains("some-package")));
        assert!(dependencies.contains(&"local:tools/local.js".to_string()));
        assert!(dependencies.contains(&"local:tools/assets/catalog.json".to_string()));
    }

    #[test]
    fn package_lock_bin_paths_are_not_repository_asset_dependencies() {
        let lockfile = br#"{
            "packages": {
                "node_modules/playwright": {
                    "resolved": "https://registry.npmjs.org/playwright/-/playwright.tgz",
                    "bin": { "playwright": "cli.js" }
                }
            }
        }"#;
        let dependencies = extract_asset_dependencies("package-lock.json", "data", lockfile);
        assert!(!dependencies.iter().any(|value| value == "local:cli.js"));
        assert!(dependencies
            .iter()
            .any(|value| value.starts_with("external:")));
    }

    #[test]
    fn css_asset_references_are_normalized_and_external_urls_are_retained() {
        let css = br#"
            @font-face { src: url("../fonts/Book.woff2"); }
            @font-face { font-family: "Arial Italic"; src: url("../fonts/Arial Italic.woff2"); }
            .ornament { background-image: url("https://cdn.example.test/ornament.svg"); }
            .inline { background: url(data:image/png;base64,AAAA); }
        "#;
        let dependencies = extract_asset_dependencies("styles/book.css", "style", css);
        assert_eq!(
            dependencies,
            vec![
                "external:https://cdn.example.test/ornament.svg".to_string(),
                "local:fonts/Arial Italic.woff2".to_string(),
                "local:fonts/Book.woff2".to_string(),
            ]
        );
    }

    #[test]
    fn html_asset_references_skip_data_urls_and_resolve_relative_paths() {
        let html = br#"
            <img src="../images/ornament.svg">
            <script src="https://cdn.example.test/app.js"></script>
            <img src="data:image/png;base64,AAAA">
        "#;
        let dependencies = extract_asset_dependencies("pages/index.html", "dom-contract", html);
        assert_eq!(
            dependencies,
            vec![
                "external:https://cdn.example.test/app.js".to_string(),
                "local:images/ornament.svg".to_string(),
            ]
        );
    }

    #[test]
    fn asset_reference_normalization_removes_query_and_fragment() {
        assert_eq!(
            normalize_asset_reference("css/site.css", "../fonts/book.woff2?v=2#face"),
            Some("local:fonts/book.woff2".to_string())
        );
        assert_eq!(normalize_asset_reference("css/site.css", "#gradient"), None);
    }

    #[test]
    fn json_escaping_is_valid_for_control_characters() {
        assert_eq!(json_string("a\n\"b"), "\"a\\n\\\"b\"");
        assert_eq!(json_string("a\tb"), "\"a\\tb\"");
    }
}
