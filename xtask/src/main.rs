use std::collections::BTreeMap;
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
    verification_level: String,
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
                let value = args
                    .next()
                    .ok_or_else(|| "--source requires REPOSITORY@COMMIT@PROFILE=PATH".to_string())?;
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
                source.repository, source.root.display()
            )
        })?;
    }

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
    let (identity, root) = value
        .split_once('=')
        .ok_or_else(|| format!("invalid --source {value:?}; expected REPOSITORY@COMMIT@PROFILE=PATH"))?;
    let mut parts = identity.split('@');
    let repository = parts.next().unwrap_or_default().trim();
    let commit = parts.next().unwrap_or_default().trim();
    let profile = parts.next().unwrap_or_default().trim();
    if parts.next().is_some()
        || repository.is_empty()
        || !repository.contains('/')
        || commit.len() != 40
        || !commit.chars().all(|character| character.is_ascii_hexdigit())
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
        let kind = classify(&source_path);
        let hash = hex(&sha256(&bytes));
        let legacy_name = path
            .file_name()
            .and_then(OsStr::to_str)
            .unwrap_or("unknown")
            .to_string();
        let status = if source.profile == "requirements-docs" && kind == "documentation" {
            "spec-only"
        } else if matches!(
            kind.as_str(),
            "settings" | "data" | "asset" | "font" | "fixture" | "corpus"
        ) {
            "verified-data"
        } else {
            "historical-only"
        };
        let notes = match kind.as_str() {
            "font" => "Byte identity scanned; glyph coverage, font tables and redistribution license are not yet verified.",
            "verified-data" => "File existence and content hash verified; semantic schema and runtime consumption remain to be assigned.",
            "documentation" => "Documentation source; not proof of executable behavior.",
            _ => "Scanner-generated inventory record; semantic ownership and parity coverage require review.",
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
            verification_level: "scanned-hash-only".to_string(),
            notes: notes.to_string(),
        });
    }
    Ok(())
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
                ".git" | "node_modules" | "target" | ".cache" | "coverage" | "playwright-report" | "test-results"
            ) {
                continue;
            }
            collect_files(root, &path, files)?;
        } else if file_type.is_file() {
            let relative = path
                .strip_prefix(root)
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "path escaped source root"))?
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
        "js" | "mjs" | "cjs" | "html" | "htm" | "css" | "json" | "svg" | "png" | "jpg"
            | "jpeg" | "webp" | "ico" | "woff" | "woff2" | "ttf" | "otf" | "txt" | "md"
            | "pdf" | "yml" | "yaml" | "toml"
    )
}

fn classify(path: &str) -> String {
    let extension = Path::new(path)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap_or("")
        .to_ascii_lowercase();
    let lower = path.to_ascii_lowercase();
    if matches!(extension.as_str(), "woff" | "woff2" | "ttf" | "otf") {
        "font".to_string()
    } else if matches!(extension.as_str(), "svg" | "png" | "jpg" | "jpeg" | "webp" | "ico") {
        "asset".to_string()
    } else if matches!(extension.as_str(), "js" | "mjs" | "cjs") {
        "javascript".to_string()
    } else if matches!(extension.as_str(), "html" | "htm") {
        "dom-contract".to_string()
    } else if extension == "css" {
        "style".to_string()
    } else if extension == "json" {
        if lower.contains("setting") || lower.contains("preset") || lower.contains("config") {
            "settings".to_string()
        } else {
            "data".to_string()
        }
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
    output.push_str("{\n  \"schemaVersion\": 1,\n  \"generatedBy\": \"xtask compat-scan\",\n  \"sources\": [\n");
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
        push_field(&mut output, "externalIdentifier", &record.external_identifier, true);
        push_field(&mut output, "legacyName", &record.legacy_name, true);
        push_field(&mut output, "owningProfile", &record.owning_profile, true);
        push_field(&mut output, "status", &record.status, true);
        output.push_str("      \"parserOrConsumer\": null,\n");
        output.push_str("      \"assetDependencies\": [],\n");
        output.push_str("      \"parityFixtures\": [],\n");
        output.push_str("      \"rustOwner\": null,\n");
        push_field(&mut output, "migrationRule", "unassigned; task 8 review required", true);
        push_field(&mut output, "verificationLevel", &record.verification_level, true);
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
            '\t' => output.push_str("\\t"),
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
    let mut state = [
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
    use super::{classify, hex, json_string, sha256};

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
    fn json_escaping_is_valid_for_control_characters() {
        assert_eq!(json_string("a\\n\\\"b"), "\\"a\\\\n\\\\\\\"b\\"");
    }
}
