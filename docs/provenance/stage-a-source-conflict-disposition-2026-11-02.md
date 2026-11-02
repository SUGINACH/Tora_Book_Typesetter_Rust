# Stage A — source conflict disposition (2026-11-02)

## Purpose and decision

This document closes the source-of-truth mapping question for Stage A task 4. It does **not** claim that missing historical files have been restored, that every profile is executable, or that parity is complete. It establishes, for each known conflict or missing reference, the owning profile, exact source pin, observed state, and handling rule.

**Decision:** the six primary sources remain separate, immutable profiles. Historical archive commits remain separate read-only provenance pins. A path found in another commit is not silently imported into the primary profile. A missing path remains a visible, owner-assigned conflict until its exact source is recovered or the owning profile is explicitly declared partial/non-runnable. No placeholders, wildcard suppressions, cross-profile substitution, or baseline edits are permitted.

## 1. Primary source ownership

| Profile | Owning repository | Exact source commit | Authority |
|---|---|---|---|
| `legacy-orange-v23` | `SUGINACH/Tora_Book_Typesetter_Orange` | `0b0ee279464a3db14672cd3f4051275aba5985da` | Distribution-oriented V23 executable behavior |
| `legacy-green-v23` | `SUGINACH/Tora_Book_Typesetter_Green` | `e5cde5f194d0d9e9b20fdaa2a742f9bf6e8aec53` | V23 engineering, diagnostics and audit behavior |
| `legacy-gold-v21` | `SUGINACH/Tora_Book_Typesetter_Gold` | `a444d34ccad30fd5ed0d6fd3873adbe9c23cb7e0` | Independent V21 algorithm and fixtures |
| `legacy-yellow-v10` | `SUGINACH/Tora_Book_Typesetter_Yellow` | `878535e12eb4aa7e3c4a72e0e79687082d8a6d3b` | V10 compatibility behavior; repository branch label V22 does not change profile identity |
| `legacy-imud-v20x` | `SUGINACH/IMUD_TORANI` | `6dd79d0ba224c169b0d19778ba181fc90edc8319` | V20.x root snapshot; nested/archive versions are separate historical sources |
| `requirements-docs` | `SUGINACH/Typesetter_doc` | `534b8f4e0ba9f5cb8abe9e3ad888904a217910cd` | Requirements/research only; never evidence of executable behavior |

All six commit objects and root tree IDs were revalidated. All 47 pinned core-file Git blob IDs match their pinned trees. The detailed authority, lineage, settings, preset and test-harness comparisons are in `source-of-truth-matrix.md`; exact refs are in `legacy-source-pins.json`.

## 2. Non-interchangeable executable ownership

The following five core files differ by Git blob across the five executable profiles. Filename equality is not implementation identity.

- `typesetter.js` — engine flow, pagination and balancing; owned by the corresponding profile's `engine-core`.
- `decorations.js` — **core executable geometry**, including decoration flow/visual/anchor behavior; owned by `engine-core::decorations`, not treated as decorative data.
- `app.js` — application orchestration and settings/asset wiring; owned by `app-core`.
- `style.css` — legacy renderer geometry/presentation; owned by the profile's renderer adapter, not a source of Rust layout decisions.
- `index.html` — DOM/iframe structure and script/style wiring; owned by the profile's renderer adapter.

Gold V21, Green V23 and Orange V23 remain independent. No V21 algorithm, fixture, baseline or settings value may be merged into V23. Orange's historical import ancestry does not authorize a current branch merge.

## 3. Historical archive references — exact disposition

The pinned IMUD main tree contains partial archive folders. Seven exact historical refs were checked and pinned in `historical-archive-source-pins.json`. Of 115 unique missing paths previously reported, 98 exist in the corresponding exact historical trees; those 98 are **candidate recovery sources**, not dependencies resolved in the primary IMUD main profile.

The remaining 17 targets are all classified below. Their owner and source pin are known; absence is explicit rather than silently suppressed.

| Owner profile | Exact source pin checked | Missing path(s) | Disposition |
|---|---|---|---|
| `legacy-imud-v20x` | IMUD V20.1 `0f6de1791fe38268fe0b6053a4e9795ece6036ae` | `archive/v20.1-tools/doc_full.txt`; `archive/v20.1-tools/doc_part1.txt` | Absent from the exact V20.1 tree; unresolved historical inputs |
| `legacy-imud-v20x` | IMUD V20.3 `2a8723b2155b974e7e304ca09fb44a49362f30cb` | `archive/v20.3-tip/work_v4/assets_manifest.json`; `archive/v20.3-tip/work_v4/fonts/fonts.json`; `archive/v20.3-tip/work_v4/header_ornaments.json`; `archive/v20.3-tip/work_v4/note_rules.json`; `archive/v20.3-tip/work_v4/ornaments_left_right.json`; `archive/v20.3-tip/work_v4/ornaments_up_down.json`; `archive/v20.3-tip/work_v4/section_dividers.json`; `archive/v20.3-tip/work_v4/shaar_back.json`; `archive/v20.3-tip/work_v4/shaar_main.json`; `archive/v20.3-tip/work_v4/shaar_sub.json` | Ten work_v4-relative assets absent from exact V20.3 tree; root copies are not substitutes because work_v4 has no base URL override |
| `legacy-gold-v21` | Gold current pin `a444d34ccad30fd5ed0d6fd3873adbe9c23cb7e0` and historical `11b2b55487815508e2934c4792ee652c16865565` | `engine-diagnostics.js`; `engine-inputs.js`; `engine-trace.js` | Referenced by renderer but absent from both checked Gold trees; do not fabricate |
| `legacy-gold-v21` | Gold current pin `a444d34ccad30fd5ed0d6fd3873adbe9c23cb7e0` | `very_long_test_text.txt` at repository root | Root fallback absent. The pinned nested corpus `text_files_for_typesetter_tests/very_long_test_text.txt` exists; older root-level file is a different blob and is not substituted |
| `legacy-orange-v23` | Orange pin `0b0ee279464a3db14672cd3f4051275aba5985da` | `../very_long_test_text.txt` | Script fallback escapes the repository root; no sibling source established |

The seven historical refs and their tree IDs are pinned separately. V20.2 has two same-path content differences; V20.3 has two core-file differences. Those differences remain explicit and are not overwritten by same-name files.

## 4. Settings, presets and test ownership

- Five-profile root `default_settings.json` comparison: 267-key union, 246 keys shared by all five, 21 presence-only differences, and zero value conflicts among shared keys. Missing is not false/zero/null and must not inherit another profile's value.
- Three root style sets are byte-identical across the five executable profiles, but each named style set has distinct values (16 values differ in each of `קונטרס` and `תלמוד` versus `פרשנות`).
- IMUD nested `presets/makom-bina-v20.5/` files and V19/V20.4 archived defaults are separate snapshots. The pinned active settings loader does not load the nested V20.5 folder.
- Test harness ownership is profile-specific: Orange focused npm suites; Green geometry-audit commands; Gold V21 chapter/baseline/determinism scripts; Yellow V10 tests and geometry audit; IMUD layout and feature suites. A same-named fixture is not shared unless its content hash matches.
- The five files supplied with the conversation (`typesetter.js`, `decorations.js`, `app.js`, `style.css`, `index.html`) match their pinned attachment hashes but have no exact blob match in the six pinned trees. Their owner is the explicit source set `user-attachment-unattributed`; repository/branch attribution remains unknown and is not guessed.

## 5. Manifest unresolved-edge ownership

The latest audited primary manifest reports 163 unresolved local dependency edges across 115 unique paths. These are not anonymous scanner failures: each edge belongs to a pinned profile and source record. Historical candidates are recorded in the archive recovery pins; the 17 absent targets above are explicit unresolved conflicts. Gold's missing engine scripts and test fallback, and Orange's parent-directory fallback, remain visible. No dependency is marked resolved merely because a similarly named file exists elsewhere.

## 6. Task 4 acceptance decision

Task 4's deliverable is a source-of-truth and conflict matrix, not restoration of every historical asset. Every known conflict now has:
1. a named owning profile/source set;
2. an exact primary or historical commit pin (or explicit unatttributed attachment hashes);
3. a path-level observed state and content-identity distinction;
4. an explicit handling rule that prevents silent substitution or cross-profile merging.

**Task 4 is accepted as complete.** The 17 missing targets remain open source defects/conflicts assigned to their owners; they are not concealed and do not block the source-mapping deliverable itself. This decision does not close tasks 6 or 8 and does not imply full compatibility or runnable historical archives.
