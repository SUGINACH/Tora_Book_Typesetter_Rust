# Asset dependency triage — manifest run 41395166090

Date: 2026-11-02  
Target branch: `main`  
Extractor commit: [3a86af8](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/commit/3a86af841aa1321160eae821518b7a52a2fa5543)  
Manifest workflow: [41395166090](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41395166090)  
Artifact: [legacy-compatibility-manifest, 13118628003](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41395166090/artifacts/13118628003)  
Baseline artifact: [13105540442](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41360202733)

## 1. Canonical comparison against the pinned baseline

| Metric | Baseline | Current | Delta |
|---|---:|---:|---:|
| Records | 18,854 | 18,854 | 0 |
| `compatId` set | 18,854 | 18,854 | identical |
| `sourceSha256` changes | — | 0 | 0 |
| Asset dependency edges | 41,192 | 18,489 | -22,703 |
| Resolved local edges | 35,356 | 18,199 | -17,157 |
| Unresolved local edges | 413 | 163 | -250 |
| External URL edges | 5,423 | 127 | -5,296 |

Edge-set diff:
- 22,835 old dependency edges removed; 132 new edges added.
- Resolved: 17,202 removed, 45 added.
- Unresolved: 290 removed, 40 added.
- External: 5,297 removed, 1 added.
- No record identity or source-content hash changed.

The large reduction is an extractor-semantics correction, not source deletion:
1. Commit [b3032d1](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/commit/b3032d16d1226d489a88bebad4a8494f67292590) stopped treating three non-loader font catalogs as runtime file manifests: `fonts-extended.json` (picker metadata), `fonts-priority.json` (not loaded by the pinned app), and the unreferenced duplicate `fonts/fonts/fonts.json`. The active `fonts/fonts.json` remains scanned. The actual static `PRIORITY_FONT_FILES` → `fetch('fonts/' + fontFile)` flow is captured by [e2e2831](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/commit/e2e283151c9ae5bc25517a3fdfe6c867219f491e).
2. Inline SVG strings in JSON/JavaScript were being misread as local paths or external URLs (notably SVG namespace URLs). [e80dda4](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/commit/e80dda48a01c46278d25f8ae615f1ba78ee50aea) added the first markup guard; [3a86af8](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/commit/3a86af841aa1321160eae821518b7a52a2fa5543) rejects angle-bracket fragments anywhere in a candidate. A regression test exposed the remaining `local:></svg>` fragment; the guard now covers it.

The 17,200 resolved edges removed from the three font catalogs were metadata/duplicate-catalog entries, not actual loads. The 5,297 external edges removed were predominantly XML namespace URLs and fragments extracted from inline SVG catalog strings. The 40 newly surfaced unresolved edges remain represented in the current unresolved set; none was suppressed.

## 2. Forensic classification of the 163 unresolved edges

There are 115 unique unresolved paths across the three affected profiles:

| Profile | Unresolved edges | Unique paths | Finding |
|---|---:|---:|---|
| IMUD V20x | 149 | 110 | References to missing files in seven partial archive subtrees |
| Gold V21 | 13 | 4 | Three missing renderer scripts plus one root-level test-text fallback |
| Orange V23 | 1 | 1 | Default test-text path escapes repository root |
| **Total** | **163** | **115** | |

### IMUD V20x — 149 edges, 110 unique paths

All 110 targets are absent from the pinned IMUD main tree at commit `6dd79d0ba224c169b0d19778ba181fc90edc8319`. The seven archive directories exist as partial snapshots; the finding is about the exact referenced files, not absence of the directories.

| Archive subtree | Edges | Unique paths |
|---|---:|---:|
| `archive/v19-baseline-6c5a7e1` | 40 | 27 |
| `archive/v20.1-tools` | 11 | 8 |
| `archive/v20.2-tip` | 10 | 10 |
| `archive/v20.3-tip` | 29 | 19 |
| `archive/v20.4-tip` | 9 | 9 |
| `archive/v20.5-tip` | 11 | 11 |
| `archive/v21-glm-11b2b55` | 39 | 26 |
| **Total** | **149** | **110** |

The exact historical-tree audit in `archive-source-recovery-2026-11-02.md` found 98 of these 110 paths at their corresponding historical commits and 12 not present there. The 98 are candidate historical sources only; they are not marked resolved in the primary IMUD-main profile.

### Gold V21 — 13 edges, 4 unique paths

Pinned Gold commit: `a444d34ccad30fd5ed0d6fd3873adbe9c23cb7e0`.

- `engine-diagnostics.js`, `engine-inputs.js`, and `engine-trace.js`: each referenced by `renderer.html`, each absent from the pinned Gold tree and the checked V21 archive commit.
- `very_long_test_text.txt`: 10 script references expect a root-level fallback. The pinned tree contains `text_files_for_typesetter_tests/very_long_test_text.txt`, not the root-level path. The older Gold archive has a root-level file, but its blob differs from the current nested fixture and is not a safe substitution.

### Orange V23 — 1 edge, 1 unique path

Pinned Orange commit: `0b0ee279464a3db14672cd3f4051275aba5985da`.

- `parallel-step-by-step-audit-21.js` defaults to `path.join(__dirname, '..', 'very_long_test_text.txt')`. Since the script is at repository root, this points outside the pinned repository. No sibling source was established.

## 3. Historical-source disposition

The seven exact read-only historical refs are pinned in `historical-archive-source-pins.json`. Across the 115 unique unresolved paths:
- 98 exist at the corresponding exact historical commits and remain candidates for explicit historical profiles/content mappings.
- 17 are absent from the exact historical trees checked: V20.1's two text inputs; ten V20.3 `work_v4/` assets; Gold's three engine scripts and root-level test-text fallback; Orange's parent-directory fallback.

Do not merge historical trees into the six primary profiles, do not satisfy one profile's dependencies from another profile implicitly, and do not create placeholders. V21/Gold and V23/Orange/Green remain distinct.

## 4. Validation and gate decision

- Rust CI [41395166264](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41395166264) passed all five jobs on `3a86af8`, including Linux, Windows and macOS quality jobs, fixed-point reproducibility and the negative unwrap gate.
- Manifest [41395166090](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41395166090) passed and generated artifact 13118628003. A second deterministic scan on `3a86af8` produced exactly the same edge sets and counts as the prior `e80dda4` scan.
- The earlier full-corpus run [41361263075](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41361263075) was cancelled after six hours during the full-book comparison step. Its smoke stage passed; no full-corpus report exists.
- The revised runner [45bbc56](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/commit/45bbc56d278fdc95311b59a918c899409844a517) disables per-step event tracing only for the full-document run while retaining full model export, geometry audit, decision ledger, text-conservation checks and repeat-run determinism. New run [41395543056](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41395543056) is in progress; no result is claimed yet.

Manifest generation and identity/hash stability pass. Extractor false-positive correction is validated by CI and deterministic manifest output. Historical source completeness and owner disposition remain open; full-corpus compatibility remains open. Stage A remains open (tasks 1–3 closed, 4–10 open); Stage B has not started. No baseline/tolerance change and no V21/V23 merge.
