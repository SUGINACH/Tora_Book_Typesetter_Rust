# Compatibility and parity matrix

Status: acceptance contract draft, 2026-11-01  
Source of truth: the pinned six-repository inventory and the compatibility plan. This matrix is a policy; it does not claim that Rust has implemented any row yet.

## 1. Independent profile matrix

| Profile | Source pin | Input/config baseline | Current evidence | Rust acceptance target |
|---|---|---|---|---|
| `legacy-orange-v23` | Orange `0b0ee27...` | Long corpus + default settings + font CSS; historical full baseline plus a newly executed smoke fixture | Offline Chromium smoke: 6 pages, 3 findings, 2/2 ledger, 485/485 fonts, deterministic repeat | Reproduce the pinned Orange semantic and decision contracts; differences require an explicit ADR |
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

## 6. First measured run — Orange smoke

Evidence: [legacy renderer workflow run](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41344525335).

- Pinned Orange commit: `0b0ee279464a3db14672cd3f4051275aba5985da`.
- Playwright: `1.55.0`; Chromium: `140.0.7339.16`; viewport: `1440×1000`.
- Self-hosted font CSS fingerprint matched. This artifact does not contain an independently verifiable 485/485 font-load result; font preflight must remain unverified until a runner report records each face and its load outcome.
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
