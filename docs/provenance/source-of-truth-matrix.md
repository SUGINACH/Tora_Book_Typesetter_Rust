# Source-of-truth and conflict matrix

Status: initial audited matrix, 2026-11-01  
Companion pin file: [legacy-source-pins.json](legacy-source-pins.json)

## 1. Authority by information class

| Information class | Primary evidence / owner | Secondary evidence | Explicit non-authority |
|---|---|---|---|
| Executable V23 distribution behavior | Orange pinned executable files at `0b0ee279464a3db14672cd3f4051275aba5985da` | Green V23 pinned code and engineering history | README claims or a newer branch name without a pinned commit |
| V23 engine architecture, diagnostics and decision-ledger concepts | Green V23 at `e5cde5f194d0d9e9b20fdaa2a742f9bf6e8aec53` | Orange imported history and its own audit/CLI | Gold V21 algorithm copied into V23 without a separately named contract |
| V21 behavior and alternatives | Gold V21 at `a444d34ccad30fd5ed0d6fd3873adbe9c23cb7e0` | Gold tests, fixtures and worklog at that same commit | Green/Orange results or baselines |
| Historical V10 compatibility | Yellow repository pinned at `878535e12eb4aa7e3c4a72e0e79687082d8a6d3b` (branch label V22) | Yellow settings, fonts, fixtures and tests | Orange defaults substituted for absent Yellow keys |
| V20.x historical behavior, assets and presets | IMUD_TORANI pinned at `6dd79d0ba224c169b0d19778ba181fc90edc8319` | Its archived snapshots and named subdirectories | Treating the repository's current root as a complete representation of every historical branch |
| Product requirements, research and transcripts | Typesetter_doc pinned at `534b8f4e0ba9f5cb8abe9e3ad888904a217910cd` | Source-repository technical docs tied to the matching code | A transcript or specification as proof that code currently implements the behavior |
| Rust target architecture and behavior | Rust repository's approved plans, ADRs and tested Rust implementation | Pinned legacy evidence used for compatibility decisions | Silent copying of legacy browser/DOM behavior into Rust |

## 2. Branch and profile isolation

| Profile ID | Repository / ref | Rule |
|---|---|---|
| `legacy-orange-v23` | Orange / `main` / pinned commit | Distribution-oriented V23 baseline |
| `legacy-green-v23` | Green / `V23` / pinned commit | Separate V23 engineering/diagnostics profile |
| `legacy-gold-v21` | Gold / `V21` / pinned commit | Separate V21 algorithm and baseline |
| `legacy-yellow-v10` | Yellow / `V22` branch / pinned commit | V10 compatibility profile; branch label does not redefine profile generation |
| `legacy-imud-v20x` | IMUD_TORANI / `main` / pinned commit | Historical V20.x root snapshot; archives/presets require their own references |
| `requirements-docs` | Typesetter_doc / `main` / pinned commit | Requirements and research only |

**Hard rule:** Green V23, Orange V23 and Gold V21 are not interchangeable. Orange's documented import ancestry is not permission to merge current branches. Gold's V21 contribution to Orange is described as separate port commits, not as permission to combine V21 and V23 algorithms. Each profile keeps its own settings, fixtures, expected results, font environment and compatibility identifiers.

## 3. Orange–Green–Gold lineage (evidence-based)

1. Orange's own `PROVENANCE.md` states that its clean history was imported from Green `V23@0ddaf89010e84e9e999c0e1faef2888994393915`.
2. The same provenance record identifies Gold `V21@31ed2ca830c1868cb2c9157d080f8922d112ff8e` as a reference point and records common ancestor `5c646c1fc2ea52233be76618463933dbe013cf2c`.
3. Orange's README says the Gold capabilities were ported as separate commits, with tests.
4. Current pinned heads are later than those ancestry references: Green `e5cde5f...`, Gold `a444d34...`, Orange `0b0ee27...`. Never substitute a current HEAD for the historical source commit.
5. Orange documents explicit distribution differences from Green at the imported point: self-hosted fonts, pinned Playwright/package lock, and offline/fingerprint protections. These differences must remain visible in comparisons.

## 4. Overlapping executable components — do not deduplicate by filename

The following filenames exist in the five executable repositories, but their Git blob IDs differ. A filename match is not an implementation identity.

| Component | Orange | Green | Gold | Yellow | IMUD |
|---|---|---|---|---|---|
| `typesetter.js` | `c940061d...` | `ed143b07...` | `8641b14a...` | `c77b2003...` | `7360b266...` |
| `app.js` | `cefade55...` | `a4d333a9...` | `2d0029e0...` | `1eea2971...` | `c5a24ac2...` |
| `decorations.js` | `0f5263b4...` | `a649ddb8...` | `2094b0c0...` | `63db474a...` | `4dcded5c...` |
| `style.css` | `791d2265...` | `b4136411...` | `d9eb7738...` | `d7afb1a4...` | `b613a946...` |
| `index.html` | `2bd20ba0...` | `a5c5c4e5...` | `5a013033...` | `f3eb221f...` | `0c32198a...` |

Full Git blob SHA-1 and SHA-256 values are in the companion JSON.

### Ownership classification

- `typesetter.js`: engine behavior, flow decisions, column balancing, pagination and layout corrections.
- `decorations.js`: **core executable component**, not merely decorative data. It owns decoration-related behavior and participates in layout/geometry.
- `app.js`: application orchestration, settings collection, asset loading and user interactions.
- `style.css`: legacy geometry and renderer-dependent presentation; relevant to compatibility measurements, but not an authority for Rust geometry.
- `index.html`: DOM structure, controls, script/style wiring and renderer embedding.
- `geometry-audit.js` and CLI tools: audit implementation and diagnostic evidence; they must not be treated as the layout engine itself.
- JSON settings, ornament collections, note rules, shaar assets and font manifests: data inputs with profile-specific defaults and provenance.

## 5. Known shared data versus conflicting implementations

- The pinned trees contain some byte-identical shared font manifests/CSS across several branches. A matching Git blob SHA means the file bytes match; it does not mean the entire font environment, license status, aliases or runtime resolution is identical.
- Gold and IMUD currently share the exact `default_settings.json` Git blob. This is a useful identity fact, not a rule that their other defaults or behavior should be unified.
- The core executable files listed above differ across all five repositories. No global "latest file wins" rule is valid.
- The IMUD root has no `geometry-audit.js`; historical audit code exists under archived paths. The Rust provenance manifest records the root-path absence rather than silently substituting an archived implementation.

## 6. Conflict-resolution protocol

When two sources disagree:

1. Pin both exact commits and paths; compare hashes and source diffs.
2. Classify the disagreement as executable behavior, data/default, test/baseline, documentation, or target requirement.
3. Assign it to the profile that owns the behavior. Do not average or merge algorithms.
4. If a Rust behavior intentionally differs, record an ADR/profile rule and add a test that distinguishes it from each legacy profile.
5. If evidence is insufficient, mark the item unresolved; do not infer behavior from a transcript, filename, version label or passing smoke test.
6. A migration between profiles must be explicit and testable; unknown/missing settings must not silently inherit another profile's defaults.

## 7. Unattributed user-supplied source files

Five files were supplied alongside the Rust project: `app.js`, `decorations.js`, `typesetter.js`, `style.css`, `index.html`. Their byte sizes and SHA-256 hashes are recorded in `legacy-source-pins.json`. None matches the corresponding root-level file in the five executable repositories at the pinned commits.

Until their origin is identified, they are an independent source set named `user-attachment-unattributed`. They must not overwrite a pinned repository snapshot or be assigned to Orange/Green/Gold/Yellow/IMUD by guesswork. Their content remains available for a later provenance diff.
A byte-level recheck against the five files mounted with the conversation was performed on 2026-11-01 and repeated against the five files uploaded in the 2026-11-02 continuation. All five byte lengths and SHA-256 values matched the `unattributedUserAttachments` entries in `legacy-source-pins.json` exactly on both checks. This confirms that the provenance record describes the actual supplied files, not similarly named repository files. It does not identify their originating repository or branch.


## 8. Scope boundary

This is the initial conflict matrix for the six repositories and the core executable set. It is not yet the complete inventory of every font, SVG, ornament, settings preset, test fixture or historical branch. Those inventories are owned by tasks 7–9 and must be generated from the pinned trees, not estimated from filenames in a partial checkout.


## 9. Pin revalidation — 2026-11-02

The six repository commits and tree IDs in `legacy-source-pins.json` were re-fetched from GitHub and checked against the pinned metadata. All six commit objects exist and all six root tree SHAs match exactly:

| Repository | Pinned commit | Tree SHA match |
|---|---|---|
| Orange | `0b0ee279464a3db14672cd3f4051275aba5985da` | yes |
| Green | `e5cde5f194d0d9e9b20fdaa2a742f9bf6e8aec53` | yes |
| Gold | `a444d34ccad30fd5ed0d6fd3873adbe9c23cb7e0` | yes |
| Yellow | `878535e12eb4aa7e3c4a72e0e79687082d8a6d3b` | yes |
| IMUD_TORANI | `6dd79d0ba224c169b0d19778ba181fc90edc8319` | yes |
| Typesetter_doc | `534b8f4e0ba9f5cb8abe9e3ad888904a217910cd` | yes |

The three historical lineage references in Orange's provenance record were also checked for existence: Green import point `0ddaf89010e84e9e999c0e1faef2888994393915`, Gold reference point `31ed2ca830c1868cb2c9157d080f8922d112ff8e`, and common-ancestor reference `5c646c1fc2ea52233be76618463933dbe013cf2c`. This verifies that the referenced commit objects are retrievable; it does not by itself prove all file-level port relationships. Those remain grounded in the pinned Orange provenance document and need file-level lineage evidence where a specific port is material.

The five user-uploaded core files were re-hashed again on 2026-11-02. All five byte lengths and SHA-256 values match the `unattributedUserAttachments` entries in `legacy-source-pins.json` exactly. Their source repository/branch remains unattributed.


## 10. Pinned default-settings key-set comparison — 2026-11-02

A structural comparison was run over the root `default_settings.json` in the five executable profiles (Orange, Green, Gold, Yellow, IMUD), each at the exact commit pinned above.

- Key counts: Orange 254; Green 248; Gold 264; Yellow 246; IMUD 264.
- Union: 267 keys; intersection present in all five profiles: 246 keys.
- **Zero value conflicts** were found among keys present in more than one profile: every shared key has the same JSON value and type wherever it exists.
- The remaining 21 differences are key-presence differences, not contradictory values:

| Presence profile(s) | Keys | Classification |
|---|---|---|
| Green only | `allow-terminal-residual-fallback=true`; `balance-chapter-end="balance"` | Green-specific engine controls; do not inject into other profiles by default |
| Orange only | `hdr-h1-title-mode="full"` | Orange-specific header setting |
| Gold + IMUD only | `aq-enabled`, `aq-line-height`, `aq-line-height-max`, `aq-para-spacing`, `aq-para-spacing-max`, `aq-shared-load`, `aq-word-spacing`, `aq-word-spacing-max`, `aq-word-spacing-max-diff`, `aq-word-spacing-min`, `aq-word-spacing-threshold` | AQ controls present in the byte-identical Gold/IMUD default file; absent from Orange/Green/Yellow |
| Orange + Gold + IMUD | `h2-orn-scope="both"`, `heading-divider-h3` through `heading-divider-h6="none"`, `shaar-main-divider="none"`, `shaar-sub-divider="none"` | Shared subset of decoration/divider defaults; absent from Green/Yellow |

**Interpretation rule:** a missing key is not equivalent to `false`, zero, empty string, or another profile's value. Preserve the distinction between present, explicit null/zero/false, and absent. The fact that Gold and IMUD share the same default-settings blob is not permission to merge their executable behavior.

This is a first settings-data conflict pass for task 4, not completion of the full source-of-truth matrix. JSON presets, font/ornament assets, tests, and other profile-specific data still require inventory and owner mapping.


## 11. Shared root data/assets — Git blob identity pass — 2026-11-02

The five pinned executable repository trees were compared by path and Git blob SHA-1. The following root-level data assets exist in all five profiles and have identical blob IDs in Orange, Green, Gold, Yellow and IMUD:

- `assets_manifest.json`
- `fonts/fonts.json` and `fonts/fonts/fonts.json`
- `header_ornaments.json`
- `note_rules.json`
- `ornaments_left_right.json`
- `ornaments_up_down.json`
- `section_dividers.json`
- `shaar_back.json`
- `shaar_main.json`
- `shaar_sub.json`

The following font inventory files are also byte-identical across all five pinned trees: `fonts-def.css`, `fonts-extended.css`, `fonts-extended.json`, and `fonts-priority.json`.

The root font file `Guttman-Rashi-Normal.woff2` is present with the same blob ID in Orange and Yellow; it is absent at the root in Green, Gold and IMUD. This is a path/presence observation only; other copies may exist under nested font directories and are part of the complete font inventory.

**Interpretation:** these exact shared blobs are reusable provenance facts, not evidence that the owning engines or their runtime font-loading behavior are interchangeable. The core executable files still have different blob IDs across the profiles. Do not deduplicate or remove profile-specific copies without a separate license, path-resolution and runtime-dependency review.

This section is a bounded root-data comparison, not a complete inventory of nested presets, archive assets, tests or all font binaries.


## 12. Test and runner ownership — pinned profile pass — 2026-11-02

The five pinned executable profiles do not share one interchangeable test harness:

| Profile | Pinned test/fixture layout | Root package test entry point | Ownership note |
|---|---|---|---|
| Orange V23 | 33 files under `tests/` or `fixtures/` | `npm test` runs seven focused Node suites: settings consistency, settings wiring, header settings, H2 ornament scope, heading divider, shaar divider and renderer assets | Distribution test suite; Playwright is a development dependency |
| Green V23 | 12 files under `tests/` or `fixtures/` | No `test` script; package scripts expose geometry audit commands | Engineering fixtures/audit evidence; do not infer that Orange's test suite is automatically Green's suite |
| Gold V21 | No `tests/` or `fixtures/` directory matched by the path scan; 44 paths contain test/debug/audit/regression/verify/check naming across scripts, workflows, test-triggers and work documentation | `test:baseline`, `test:quick`, and `determinism` run scripts under `scripts/` | V21-specific test/debug scripts and trigger corpus; keep independent of V23 |
| Yellow V10 profile (repository branch V22) | 13 files under `test/` or `tests/` | No root `package.json` at the pinned commit | Includes engine input/trace and geometry-audit tests plus historical PDF outputs |
| IMUD V20x | 18 files under `tests/` or `fixtures/` | `npm test` runs quick layout suite and feature suite | Historical feature/layout test harness; package name is `tora-book-typesetter-orange`, but the pinned source identity remains IMUD |

### Shared Orange–Green fixtures

Eight paths under `tests/fixtures/` exist in both Orange and Green:
- Seven have identical Git blob IDs: `geometry-audit-diagnostics-smoke.txt`, `geometry-audit-smoke.txt`, `no-legal-two-column-split.txt`, `ornament-geometry-parity-settings.json`, `ornament-geometry-parity-smoke.txt`, `ornament-geometry-smoke.txt`, and `ornament-geometry-title-tb-settings.json`.
- `ornament-geometry-settings.json` differs. Six keys differ: `hdr-verso-r-type`, `hdr-verso-c-type`, `hdr-verso-l-type`, `hdr-r-type`, `hdr-c-type`, and `hdr-l-type`. Orange sets all six to `ext________2_svg`; Green sets all six to `none`.

The differing fixture is an intentional profile-sensitive test input until proven otherwise; do not normalize it to one value or reuse one profile's expected output for the other. The seven byte-identical fixtures are shared test data, but do not imply identical engine behavior.

This is a structural ownership inventory, not a test-quality or coverage judgment. Test names/counts alone do not establish that every required behavior is covered; task 9 must map each required compatibility record to an executable fixture and measurable criterion.


### 12.1 Exact package/test commands and dependency pins

The package metadata at the pinned commits confirms the test ownership distinction:

| Profile | Package name | Exact test command(s) | Playwright dependency |
|---|---|---|---|
| Orange | `tora-book-typesetter-unified` | `npm test` = settings-consistency + settings-wiring + header-settings + h2-ornament-scope + heading-divider + shaar-divider + renderer-assets; also `npm run test:settings`, `test:header`, `test:consistency`, `test:h2scope`, `test:dividers`, `test:shaar`, `test:assets` | exact `1.55.0` |
| Green | `imud-torani` | No test script; `npm run audit:geometry` and `npm run audit:geometry:long` only | `^1.55.0` |
| Gold | `tora-book-typesetter-gold` | `npm run test:baseline` → `cd scripts && node test-chapters-enhanced.js 1 79 baseline`; `npm run test:quick` → `... 1 1 quick`; `npm run determinism` → `node determinism-test.js 9 3` | none declared |
| Yellow | no root `package.json` at pinned commit | no npm script contract recorded | not declared |
| IMUD | `tora-book-typesetter-orange` | `npm test` = `npm run test:layout:quick && npm run test:features`; layout suite `node tests/layout/run-suite.js`; feature suite `node tests/features/run.js`; baseline update `node tests/layout/run-suite.js --update-baseline` | `^1.50.0` |

The package names are metadata labels, not source identity: in particular, the IMUD V20x pin's package name contains “orange”, while the Orange V23 pin has the distinct package name `tora-book-typesetter-unified`. Repository identity remains the pinned owner/repository/commit in `legacy-source-pins.json`.


### 9.1 Exact-content search of uploaded files across all six pinned trees

The five uploaded files were converted to Git blob SHA-1 values using Git's `blob {length}\0{bytes}` framing, then compared against every blob in all six pinned repository trees. **No exact blob match was found** in any of the six pinned trees. This confirms only that the files are not byte-identical to a file in those six snapshots; it does not rule out a modified ancestor, another branch, or a deleted historical commit.

| Uploaded file | Bytes | SHA-256 | Git blob SHA-1 |
|---|---:|---|---|
| `style.css` | 51,621 | `1d69f505806bae7c6d114e9ca979f6bb54244ffbbb00ec634b5bb2043bf2047b` | `b2bf310856fa31b593b18b2bc9a41a02be400661` |
| `decorations.js` | 35,597 | `69696aa6dbc0bed4772ba9a67508f08460ef26468fba469a89d98d3708414d65` | `de72223ef66e38100f2d10ec86fc622820312ace` |
| `typesetter.js` | 158,627 | `7a02b7b770a861c96bcff919958d66d8c472f9e35d3783bd832519862a025d10` | `e06a9686345ca63aa892ac6387634b6541aad386` |
| `index.html` | 97,694 | `f595b654e78055509150cd652403b905152a3a9c10b7f6d656178591a935b728` | `d165233f11ad8ff315990c3f967babe5cd921728` |
| `app.js` | 78,737 | `744cfcdbf2b81e30987232ef17d8ee13ad85dcf9bcdbd1d96cccb207a5defe6b` | `9faba3b111b23451edeaef6625a0945b8f99141f` |

The five attachment byte lengths and SHA-256 values continue to match the five `unattributedUserAttachments` records in `legacy-source-pins.json`. Their source attribution remains open; no branch or repository has been guessed.


## 13. Preset and style-set provenance — 2026-11-02

### Root style sets shared by all five executable profiles

At the exact pinned commits, all five repositories contain the same three root paths with identical Git blob IDs:

| Path | Git blob SHA-1 (all five profiles) | JSON keys |
|---|---|---:|
| `setting_sets/פרשנות.json` | `2f92a7002f3a9ef64e7e6592fb848ddf077bbe83` | 249 |
| `setting_sets/קונטרס.json` | `c56c30f4f3d3c0f46411bba8760fa37a4ad5645a` | 249 |
| `setting_sets/תלמוד.json` | `bb99eb2cc8d59edce7a18c8f92a5597627217d93` | 249 |

These are exact byte identities, not merely equal parsed JSON. Comparing the three shared files: each has the same 249-key set; `קונטרס` differs from `פרשנות` in 16 values, and `תלמוד` differs from `פרשנות` in 16 values. Differences include body last-line alignment, footnote columns, body line-height, column proportions/gap, and heading sizes. The three styles must remain distinct inputs even though their schemas are identical.

### IMUD V20x nested makom-bina-v20.5 snapshot

The pinned IMUD tree also contains four nested JSON files under `presets/makom-bina-v20.5/`. Each has 249 keys, but these are not byte-identical to the current root `default_settings.json` (264 keys):

| File | Keys | Root-only keys | Preset-only keys | Shared keys with changed values vs root default |
|---|---:|---:|---:|---:|
| `default_settings.json` | 249 | 24 | 9 | 66 |
| `setting_sets/פרשנות.json` | 249 | 24 | 9 | 79 |
| `setting_sets/קונטרס.json` | 249 | 24 | 9 | 80 |
| `setting_sets/תלמוד.json` | 249 | 24 | 9 | 82 |

The 24 root-only keys include `style-set-select`, the parenthesis-format controls, footnote layout mode, runt-word avoidance, H2 scope/divider controls, and the AQ controls. The nine preset-only keys are asset-path settings such as `assets-folder-input` and the builder/header/section SVG selectors. Value differences include font-family names (Hebrew family labels versus the root defaults), header configuration, book metadata, and style metrics.

The pinned IMUD `settings-manager.js` explicitly loads root `default_settings.json` and then loads selected styles from `setting_sets/`; its manually enumerated style list is exactly `קונטרס.json`, `פרשנות.json`, `תלמוד.json`. It does not reference `presets/makom-bina-v20.5/`. Therefore the nested folder is recorded as a pinned preset snapshot, **not asserted to be part of the current root UI loading path**.

### Historical archived defaults

Two additional IMUD archived defaults were compared to the current root default:

| File | Keys | Keys only in archive | Keys only in root | Shared values that differ |
|---|---:|---:|---:|---:|
| `archive/v19-baseline-6c5a7e1/default_settings.json` | 250 | 1 (`avoid-runt-word`) | 0 | 20 |
| `archive/v20.4-tip/default_settings.json` | 242 | 2 | 9 | 38 |

The V19 file contains distinct book metadata and layout defaults. The V20.4 file also has a different theme/header configuration and omits several root asset selectors. These files are historical profile data, not candidates to replace the pinned root defaults.

### Acceptance consequence

- Shared root style-set blobs may be represented once in a content-addressed inventory while preserving five profile/path owners.
- The three style-set names remain semantically distinct because their values differ.
- Nested V20.5 and archived defaults must retain explicit snapshot identity and must not be merged into current root defaults.
- Task 4 remains open: this closes only a further preset/style-set comparison slice. It does not complete all profile-owned assets, tests, or source conflict ownership.


## 14. Exact historical source recovery for partial archive folders — 2026-11-02

The pinned IMUD main tree's archive folders are partial snapshots, not complete standalone trees. Exact historical refs have now been identified and checked:

| Archive folder | Exact source ref | Archive files byte-identical to source | Missing referenced paths present at that source | Not present |
|---|---|---:|---:|---:|
| `archive/v19-baseline-6c5a7e1/` | IMUD commit `6c5a7e1f19b4b40348d9d8d6ee00d9c9b8fd15db` | 6/6 | 27/27 | 0 |
| `archive/v20.1-tools/` | IMUD branch V20.1, `0f6de1791fe38268fe0b6053a4e9795ece6036ae` | 12 identical among 27 archive files | 6/8 | 2 |
| `archive/v20.2-tip/` | IMUD branch V20.2, `1b7900433bbed49c7e4336155d0d4d96764170c8` | 7/9 | 10/10 | 0 |
| `archive/v20.3-tip/` | IMUD branch V20.3, `2a8723b2155b974e7e304ca09fb44a49362f30cb` | 27/29 | 9/19 | 10 work_v4 paths |
| `archive/v20.4-tip/` | IMUD branch V20.4, `ff079951ce6f8a0c9243f97801e2fa1c27dc516a` | 6/6 | 9/9 | 0 |
| `archive/v20.5-tip/` | IMUD branch V20.5, `30bf8f36b4a1676214e6ea4f50b75e17baf37170` | 6/6 | 11/11 | 0 |
| `archive/v21-glm-11b2b55/` | Gold commit `11b2b55487815508e2934c4792ee652c16865565` | 13/13 | 26/26 | 0 |

The path-presence total is 98 candidate historical files. Seventeen unique targets were not found in the corresponding exact commits checked: V20.1's `doc_full.txt` and `doc_part1.txt`; ten V20.3 `work_v4/` data files; Gold's three engine scripts and current root test-text fallback; and Orange's parent-directory test-text fallback. These are not declared globally lost; they are absent from the exact source trees checked.

Important qualifications:
- V20.2 has two archive/branch content differences (`script.js`, `styles.css`).
- V20.3 has two archive/branch core-file differences (`app.js`, `decorations.js`); its work_v4 page has no base URL override, so its missing data paths cannot be satisfied by root-level copies.
- The older Gold root test fixture differs by blob from the current nested fixture; it is not a safe replacement.
- The exact refs are preserved in `docs/provenance/historical-archive-source-pins.json`; the full audit is in `docs/provenance/archive-source-recovery-2026-11-02.md`.

These historical commits remain separate, read-only provenance references. They have **not** been merged into the primary source profiles or used to mark primary-manifest dependencies resolved. Stage A task 4 remains open pending explicit profile integration/owner decisions.


## 15. Stage A task 4 — conflict ownership and disposition (2026-11-02)

The complete path-level owner/disposition decision is recorded in [stage-a-source-conflict-disposition-2026-11-02.md](stage-a-source-conflict-disposition-2026-11-02.md).

This closes the **mapping** deliverable for task 4: all known conflicts are assigned to a profile/source set and an exact primary or historical pin (or explicitly to the unattributed attachment set), with path-level evidence and a no-substitution rule. It does not restore the 17 absent historical targets, resolve the 163 primary-manifest dependency edges, or make partial archives runnable. Those remain visible owner-assigned conflicts for tasks 6 and 8.

The 98 files found at separate historical refs remain candidate recovery sources only. They are not added to the primary IMUD profile and do not change any V21/V23 separation rule. 
