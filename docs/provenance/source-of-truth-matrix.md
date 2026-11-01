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
A byte-level recheck against the five files mounted with this conversation was performed on 2026-11-01. All five byte lengths and SHA-256 values matched the `unattributedUserAttachments` entries in `legacy-source-pins.json` exactly. This confirms that the provenance record describes the actual supplied files, not similarly named repository files. It does not identify their originating repository or branch.


## 8. Scope boundary

This is the initial conflict matrix for the six repositories and the core executable set. It is not yet the complete inventory of every font, SVG, ornament, settings preset, test fixture or historical branch. Those inventories are owned by tasks 7–9 and must be generated from the pinned trees, not estimated from filenames in a partial checkout.
