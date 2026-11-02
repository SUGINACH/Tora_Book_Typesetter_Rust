# Compatibility and parity matrix

Status: acceptance contract defined; Rust parity not yet tested, 2026-11-02  
Source of truth: the pinned six-repository inventory and the compatibility plan. This matrix is a policy; it does not claim that Rust has implemented any row yet.

## 1. Independent profile matrix

| Profile | Source pin | Input/config baseline | Current evidence | Rust acceptance target |
|---|---|---|---|---|
| `legacy-orange-v23` | Orange `0b0ee27...` | Long corpus + default settings + font CSS; historical baseline remeasured under pinned Node/Playwright/Chromium/viewport | Full-corpus replay: 639 pages, 39 findings (0/11/28), 857/857 ledger, 3,105 paragraphs, 529 headings, 2,042 side notes, 485/485 fonts, deterministic repeat | Reproduce the pinned Orange semantic and decision contracts; differences require an explicit ADR |
| `legacy-green-v23` | Green `e5cde5f...` | Long corpus and default settings pinned; no complete measured baseline fingerprint yet | Engineering docs/worklog and source-level evidence | Separate profile; never inherit Orange's metrics or settings silently |
| `legacy-gold-v21` | Gold `a444d34...` | Long corpus/default settings pinned; historical metrics lack a complete environment identity | V21 historical evidence | Separate V21 algorithm and fixtures; no V21-to-V23 merge |
| `legacy-yellow-v10` | Yellow `878535e...` (branch label V22) | Long corpus/default settings pinned; no complete measured baseline | Historical font and compatibility source | V10 compatibility only; missing keys resolve using Yellow defaults |
| `legacy-imud-v20x` | IMUD `6dd79d0...` | Long corpus/default settings and V20.5 preset pinned; existing scenario baseline is not a complete parity baseline | Historical V20.x scenarios and presets | Explicit V20.x profiles; no global default substitution |
| `requirements-docs` | Typesetter_doc `534b8f4...` | Specification/research documents pinned | Documentation evidence only | Requirements become features/ADRs; not executable parity fixtures |

The abbreviated SHAs above are display-only. The full 40-character commits and file hashes are in `docs/provenance/legacy-source-pins.json` and `legacy/corpus-manifest.json`.

## 2. Three independent parity dimensions

### A. Semantic parity — hard gate

- Source text is conserved exactly according to the profile's normalization contract.
- Paragraph, heading, note, footnote and marker counts/order match the fixture's expected source model.
- No source span is dropped, duplicated or reordered; unknown settings and metadata are preserved or explicitly diagnosed.
- Every output fragment maps back to a stable source token/span.
- Required heading levels, note classes, TOC entries, headers, ornaments and separators are present or produce an explicit unsupported/conflict record.
- Acceptance: zero unexplained missing/mismatched tokens, zero unexplained source/output count differences, and zero silent fallback.

### B. Typographic parity — profile-scoped gate

- Compare only within the same pinned profile, settings, font files, aliases, shaping version and feature flags.
- Compare resolved font identity, glyph/cluster mapping, bidi order, line-break opportunities, line text spans, line counts and note marker attachment.
- Exact line-break equality is required when the same font/shaper environment is used and the fixture declares strict legacy parity.
- If Rust intentionally adopts a different shaping or line-breaking algorithm, the difference is not waived by a tolerance: it must be recorded as an approved profile migration with before/after fixtures and an ADR.
- Never compare Green/Orange/Gold/Yellow/IMUD as if they shared one typographic baseline.

### C. Geometric/visual parity — measured gate

- Compare page count and transitions, column assignment, block bounds, line bounds, footnote reservation, heading keep behavior, ornament/header/divider bounds and clipping.
- Rust's own repeated builds must be exactly deterministic at the canonical fixed-point representation; no epsilon is allowed in its decision comparator.
- For comparisons against legacy browser geometry, retain the source audit's 0.5 CSS-pixel measurement epsilon as a *measurement classification* only. It is not a permission to change page/column/line structure.
- Any cross-renderer tolerance must be named, unit-bearing, tied to a specific metric and recorded in the report. Unknown or absent geometry is a failed measurement, not a pass.
- The legacy audit's note-marker containment-rounding exception (0–3 px) remains a source-specific audit classification; it must not be copied into Rust's core geometry rules.

## 3. Matrix dimensions required for each parity run

| Dimension | Required fields |
|---|---|
| Source | repository, full commit, branch/profile ID, source file hashes |
| Input | path, byte length, SHA-256, encoding/newline/BOM facts |
| Settings | profile ID, raw settings SHA-256, preset ID/hash, unknown keys |
| Fonts | font file hashes, CSS/manifest hashes, aliases, loaded/failed set, license state |
| Engine | engine commit/version, parser/shaper versions, feature flags, algorithm profile |
| Platform | OS/architecture, toolchain, browser version when legacy, PDF renderer version when applicable |
| Output | semantic conservation report, line/cluster mapping, page/column/line geometry, decision ledger |
| Determinism | repeated output hashes, stable ordering, no timestamp/path contamination |
| Verdict | semantic, typographic and geometric outcomes separately; deviations linked to ADR |

## 4. Stable test identifiers and measurable gates

| Test ID | Applies to | Measurable acceptance criterion |
|---|---|---|
| `SRC-PIN-HASH` | Every source record | Full commit and SHA-256 match the manifest exactly; zero unpinned inputs |
| `CORPUS-BYTE-IDENTITY` | Corpus/fixture records | Input byte length and SHA-256 match; no newline/BOM normalization unless the profile explicitly declares it |
| `PROFILE-REPLAY` | Executable profile records | Run with the record's own input, settings, font and environment pins; never borrow another profile's baseline |
| `SEMANTIC-CONSERVATION` | Engine and corpus records | Zero unexplained missing, duplicated, reordered or mismatched source spans; heading/note counts and order exact |
| `DECISION-LEDGER-PARITY` | Layout-engine records | One valid ledger record per required DOM/layout block; zero missing, excess or malformed records |
| `DETERMINISM-REPEAT` | Every executable profile | Two runs with identical immutable inputs produce identical canonical page/decision hashes |
| `FONT-PREFLIGHT` | Font/CSS/settings records | Every required font face loads; zero failed required faces; alias resolution and glyph coverage reported; license separately cleared |
| `SETTINGS-ROUNDTRIP` | Settings/preset records | Unknown keys preserved losslessly; serialized normalized settings stable; no cross-profile fallback |
| `ASSET-REFERENCE-CLOSURE` | SVG/CSS/HTML/JSON asset records | Every local dependency resolves to a pinned asset; zero unapproved external requests, path traversal or missing references |
| `DATA-ROUNDTRIP` | Profile-owned JSON data records | JSON parses under the pinned schema policy and parse→serialize→parse preserves the complete value tree and unknown keys; source bytes remain SHA-256 pinned |
| `BASELINE-IDENTITY` | Historical/profile baseline records | Baseline bytes/schema/hash are immutable and profile-scoped; updates require an explicit reviewed baseline migration |
| `LEGACY-DOM-GEOMETRY` | DOM/CSS/decoration records | Measured page/column/block bounds available; no clipping/overflow; cross-renderer epsilon explicit and unit-bearing |
| `PDF-UNICODE-EXTRACTION` | PDF export records | Independent extractor returns exact logical Unicode text, Hebrew RTL order, niqqud and punctuation from embedded Type0/ToUnicode fonts |
| `SPEC-TO-ADR` | Spec-only documentation records | Every accepted requirement links to a feature/test or ADR; documentation alone never marks executable parity passed |

The IDs are stable test contracts. A manifest record may list multiple IDs. Until the implementation exists, the record's verification level must remain below `parity-tested`; listing a test ID is not evidence that it passed.

## 5. Acceptance rules

1. Semantic parity is mandatory for every required compatibility record.
2. Typographic and geometric parity are evaluated only against the record's owning profile and pinned environment.
3. A profile without a measured baseline may be scanned and inventoried, but it cannot receive a passing parity verdict.
4. A fixture is not a baseline unless input, settings, fonts/assets, source commit and execution environment are all pinned.
5. An unexplained fallback, missing font, dropped setting, nondeterministic tie-break or missing ledger entry fails the relevant gate.
6. A known conflict is not silently converted into a pass. It receives status `conflict`, an ADR, a named migration rule and a fixture that demonstrates the chosen behavior.
7. Visual similarity cannot compensate for semantic loss; semantic equality cannot compensate for a documented clipping/overflow defect.
8. Every `verified-data` or `verified-executable` manifest record must have at least one exact profile-scoped fixture reference. `blocked-license`, `historical-only` and `spec-only` records may omit fixtures only while retaining that explicit disposition.

## 6. First measured run — Orange smoke

Evidence: [legacy renderer workflow run](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41344525335).

- Pinned Orange commit: `0b0ee279464a3db14672cd3f4051275aba5985da`.
- Playwright: `1.55.0`; Chromium: `140.0.7339.16`; viewport: `1440×1000`.
- The geometry report records `fontPreflight.faceCount=485`, `loaded=485`, `failed=0`, and an empty failures list. The runner manifest from this earlier run did not duplicate that nested object; the report itself is the evidence.
- Self-hosted font CSS fingerprint matched.
- 6 pages; 3 audit findings (0 high, 2 medium, 1 low).
- 2 decision-ledger records matched 2 DOM blocks; no malformed or missing ledger record.
- Text conservation: 7 source tokens, 4 source paragraphs; no missing/mismatched paragraphs, headings or notes.
- No external requests; repeat-run page signatures were identical; no browser page/console errors.

This smoke is a verified runner contract, not a full-book parity baseline. The long corpus baseline is recorded separately with its own input/config/environment fingerprint. The first run's report and summary are preserved as a GitHub Actions artifact.

## 7. Open coverage gaps

- Full long-corpus replay under the current pinned Orange environment.
- A reproducible measured full-document baseline for Green, Gold, Yellow and IMUD.
- Font binary hashes, license review, glyph coverage and alias resolution for every profile.
- A fixture-to-compatibility-record mapping for every mandatory feature.
- Separate strict regression fixtures for V21, V23, V10 and V20.x behavior.


## 8. Full-corpus measured replay — Orange V23 (2026-11-02)

Evidence: [isolated legacy runner 41395543056](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41395543056), artifact 13119497022; runner code commit [45bbc56](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/commit/45bbc56d278fdc95311b59a918c899409844a517). The job completed successfully, including the full-book comparison step. This supersedes the earlier cancelled run 41361263075; that cancelled run produced no full-corpus report.

Pinned execution identity:
- Legacy source: Orange V23 0b0ee279464a3db14672cd3f4051275aba5985da (legacy-orange-v23).
- Input: very_long_test_text.txt, 3,424,454 bytes, SHA-256 5e73b913f9843fce695d0e4073c176961b8dbc2fa87e8e4c3d99051a826ce0e4.
- Settings: default_settings.json, SHA-256 5c78a49ca2b205f0a846e6f87a1058b4a61595967b472bda65ac7412f13f5b3f.
- Font CSS: selfhost-fonts/google-fonts-selfhost.css, SHA-256 eec01e2b2a65ab67ad959625dd7b3640dceeb09a5cee8b7677787a1e89acd229.
- Runtime: Node v22.23.4, Playwright 1.55.0, Chromium 140.0.7339.16, offline-enforced, viewport 1440×1000, deviceScaleFactor 1.

| Measure | Result |
|---|---:|
| Pages | 639 |
| Audit findings | 39 (0 high, 11 medium, 28 low) |
| Valid decision-ledger records / DOM two-column blocks | 857 / 857 |
| Source paragraphs | 3,105; missing 0; text mismatches 0; extra rendered IDs 0 |
| Source headings | 529; missing 0; mismatches 0 |
| Side notes | 2,042; metadata/text missing or mismatched 0 |
| Font faces | 485 loaded / 485; failed 0 |
| External requests | 0 |
| Repeat-render determinism | PASS |
| Browser page/console errors | 0 / 0 |
| Render duration | 323,813 ms |

Issue classes: 12 container-gap, 8 imbalance, 7 containment-rounding, 6 structural-fallback, 3 sparse-page, and 3 square-bracket-scope. The 39 findings remain in the audit inventory; workflow success does not mean that they disappeared. The active geometry gate was **disabled** in this run; its reported PASS means zero findings blocked that disabled gate, not that the document has zero geometry findings. Seven note-marker rounding findings were non-blocking under the legacy audit's source-specific 0–3 px classification.

Parenthesis diagnostics covered all 857 decision records: 2,483 paired scopes, 81 cross-column pairs, 48 cross-page pairs, zero unmatched opens at document end, and 3 unmatched closing parentheses. The unmatched closers remain source diagnostics and must not be silently normalized.

The run's measured metrics exactly matched the pinned historical expected_baseline.json: 639 pages, 39 findings (0/11/28 severity split), and 857 ledger records. This is a reproducible **Orange V23 legacy baseline replay under the current pinned environment**; it is not a Rust-vs-legacy parity result and does not establish baselines for Green, Gold, Yellow or IMUD. Report SHA-256: 7fa09479bde1050ca51ee20ae5a2bf59f958f99dbdf69731dff75e77d5dce95a; summary SHA-256: 71ec2c6030cff7161fec045e78c191abd9b3ca341a6a27d44bfb14ebd471cc55.

## 9. Profile-scoped tolerance register

| Profile | Semantic/text tolerance | Legacy geometry measurement tolerance | Status |
|---|---|---|---|
| legacy-orange-v23 | Exact: zero unexplained token/paragraph/heading/note loss, duplication, reorder or mismatch | 0.5 CSS px classification epsilon for measured DOM bounds only; it cannot alter page/column/line structure. Note-marker protrusion 0–3 CSS px is a legacy audit-only classification, not an engine allowance. | Measured full corpus; 39 findings remain recorded |
| legacy-green-v23 | Exact | Not assigned; do not inherit Orange's tolerance or metrics | Blocked pending its own pinned measured baseline |
| legacy-gold-v21 | Exact | Not assigned; keep V21-specific | Blocked pending its own pinned measured baseline |
| legacy-yellow-v10 | Exact | Not assigned; keep V10-specific | Blocked pending its own pinned measured baseline |
| legacy-imud-v20x | Exact | Not assigned; keep V20.x-specific | Blocked pending its own pinned measured baseline |
| requirements-docs | Not applicable to executable parity | Not applicable | spec-only; requirements must map to tests/ADRs |

For Rust-native comparisons, canonical fixed-point values and repeated outputs must match exactly; epsilon is always zero in the decision comparator. No cross-renderer tolerance is authorized until it is named, unit-bearing, tied to a metric and supported by a profile-specific fixture. Missing measurements fail the relevant gate; they are never interpreted as zero error.

## 10. Mask and exclusion policy

- No masks, wildcard ignores, page-range exclusions, text substitutions or geometry clipping may be used to turn a failed parity comparison into a pass.
- Semantic data (source spans, glyph/cluster mappings, line breaks, headings, notes, markers and reading order) is never maskable.
- Page count, page transitions, column assignment, line/block bounds, overflow, clipping, ornaments, headers, separators and asset references are never maskable.
- A known source-specific exception must be represented as a named audit classification with exact metric, units, profile, source evidence and a test ID. It remains visible in the report and cannot be generalized to another profile or copied into Rust's core rules.
- Volatile report metadata may be excluded only by an explicit field-path allowlist in the canonical serializer; the allowlist must identify each field and reason. Wildcard path masks and content masks are prohibited.

## 11. Manifest-to-test mapping audit — 2026-11-02

The successful canonical scan [41395791545](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41395791545), artifact 13119601763, contains 18,854 records from six pinned sources. Independent inspection found: all 18,854 records have non-empty rustOwner, migrationRule, testIds and acceptanceCriteria; all 11 test IDs actually used by records are declared in §4; no unknown test IDs were found. PDF-UNICODE-EXTRACTION is defined as a stable contract but is not yet attached to a generated legacy source record.

The scan contains 140 records with at least one parityFixtures entry and 18,714 without one. The latter are not silently treated as parity-tested: many are fonts, historical documentation or non-fixture assets, but the mandatory feature-to-fixture ownership pass is still incomplete. Verification levels are 18,843 scanned-hash-only and 11 offline-smoke-tested; the canonical manifest has not yet ingested the separate full-corpus runner evidence as offline-full-corpus-tested. Dependency inventory: 18,489 total edges, 18,199 resolved local edges, 163 unresolved local edges across 115 unique paths, and 127 external references.

**Task 9 status:** the parity contract, profile isolation, tolerance register, mask prohibition and measurable test-ID mapping are now documented and audited. Formal closure remains blocked by task 6 (only Orange has a complete current-environment full-corpus replay) and task 8 (fixture/provenance ownership and unresolved dependencies remain open). No parity pass is claimed for the Rust engine, which is still a scaffold without production parser/shaper/pagination/renderer.
