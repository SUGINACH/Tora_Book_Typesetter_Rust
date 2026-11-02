# Asset dependency triage — manifest run 41376496462

Date: 2026-11-02  
Target branch: `main`  
Code commit: `526d6a04508be4145b09efaf25424ca59fb7ef51`  
Manifest workflow: [41376496462](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41376496462)  
Artifact: [legacy-compatibility-manifest, 13111866073](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41376496462/artifacts/13111866073)  
Baseline artifact: [13105540442](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41360202733)

## 1. Exact comparison against baseline

| Metric | Baseline | Current | Delta |
|---|---:|---:|---:|
| Records | 18,854 | 18,854 | 0 |
| `compatId` set | 18,854 | 18,854 | identical |
| `sourceSha256` changes | — | 0 | 0 |
| Asset dependency edges | 41,192 | 40,940 | -252 |
| Resolved edges | 35,356 | 35,387 | +31 |
| Unresolved edges | 413 | 161 | -252 |

Edge-set diff:
- 278 old asset edges removed and 26 new asset edges added.
- 0 previously resolved edges were lost; 31 resolved edges were added.
- 263 previously unresolved edges disappeared and 11 new unresolved edges appeared.
- The 11 new unresolved edges are not treated as false positives: they are static fallback references to files that do not exist at the pinned repository paths (details below).

All record identities and source hashes remain byte-for-byte stable. This is an extractor-output change only, not a source-tree change.

## 2. Forensic classification of the 161 unresolved edges

### IMUD V20x — 147 edges, 133 unique paths

All 133 unique local paths are absent from the pinned IMUD tree at commit `6dd79d0ba224c169b0d19778ba181fc90edc8319`. The tree does contain partial archive folders, so this finding does **not** mean the folders themselves are absent; it means the specific referenced assets are not present in the pinned snapshot.

| Archive subtree | Unique missing paths | Edges |
|---|---:|---:|
| `archive/v19-baseline-6c5a7e1` | 39 | 40 |
| `archive/v20.1-tools` | 7 | 9 |
| `archive/v20.2-tip` | 10 | 10 |
| `archive/v20.3-tip` | 19 | 29 |
| `archive/v20.4-tip` | 9 | 9 |
| `archive/v20.5-tip` | 11 | 11 |
| `archive/v21-glm-11b2b55` | 38 | 39 |
| **Total** | **133** | **147** |

These are genuine unresolved historical dependencies relative to the pinned commit. They require an explicit provenance decision: restore the exact missing historical assets into the source snapshot, or document that the corresponding archived snapshots are intentionally partial/non-runnable. Do not silently suppress them or mark them resolved.

### Gold V21 — 13 edges, 4 unique paths

Pinned Gold commit: `a444d34ccad30fd5ed0d6fd3873adbe9c23cb7e0`.

- `engine-diagnostics.js`, `engine-inputs.js`, and `engine-trace.js`: each referenced from `renderer.html`, each absent from the pinned tree.
- `very_long_test_text.txt`: 10 edges from scripts under `scripts/`. The pinned tree contains `text_files_for_typesetter_tests/very_long_test_text.txt`, but not the root-level path the scripts use as their default fallback. For scripts that permit `TEST_TEXT_PATH`, an environment override may make a run succeed, but it does not make the committed fallback path exist.

These are source-level missing/default-path references, not extractor noise.

### Orange V23 — 1 edge, 1 unique path

Pinned Orange commit: `0b0ee279464a3db14672cd3f4051275aba5985da`.

- `parallel-step-by-step-audit-21.js` defaults to `path.join(__dirname, '..', 'very_long_test_text.txt')`. Since the script is at the repository root, this points outside the pinned repository. The target is not part of the pinned tree. An environment override may be intended, but the default fallback remains unresolved.

## 3. Extractor acceptance status

The current scan contains no unresolved edge that has been demonstrated to be a lexer false positive. The earlier npm package-path false positives and extension-only/template fragments are absent from the current unresolved set. The 31 newly resolved edges include static path references in browser tests and local test fixtures.

This is a materially improved scan, but **manifest/provenance acceptance remains open** until owners decide how to handle the partial historical archives and the missing Gold/Orange defaults. Do not change the baseline, suppressions, or tolerance to force closure.

## 4. Validation

- [Rust CI 41376496424](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41376496424) passed on `526d6a0`.
- [Manifest generation 41376496462](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41376496462) passed and produced the artifact cited above.
- [Legacy full-corpus run 41361263075](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41361263075) remains in progress at the full-book corpus comparison step. Smoke/renderer/DOM geometry/ledger steps completed successfully; the full corpus report and artifacts have not yet been produced. Do not infer a corpus result or cancel this run.

## 5. Gate decision

- Manifest scan generation: operationally passed.
- Identity/hash stability: passed.
- Extractor false-positive review: no current unresolved false positives found.
- Historical asset completeness and owner disposition: **open**.
- Full-book compatibility comparison: **open**.
- Stage A: remains open; only tasks 1–2 are closed, tasks 3–10 remain open.
- Stage B: not started.
