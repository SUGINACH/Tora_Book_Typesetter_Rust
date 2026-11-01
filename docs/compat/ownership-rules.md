# Compatibility ownership rules

The scanner assigns a conservative owner and migration rule to every scanned record. These are *initial ownership assignments*, not proof that the corresponding Rust implementation already exists.

| Source pattern / record kind | Rust owner | Consumer / parser | Migration rule |
|---|---|---|---|
| `typesetter.js` | `engine-core` | `legacy-layout-engine` | Port under the source profile with explicit fixtures; never merge profile algorithms |
| `decorations.js` | `engine-core::decorations` | `legacy-decoration-and-geometry-runtime` | Treat as core executable geometry behavior, not decorative data |
| `geometry-audit.js` | `audit` | `legacy-independent-geometry-audit` | Reimplement independently; audit output is not a layout input |
| `tools/geometry-audit-cli.js` | `xtask` | `legacy-offline-renderer-runner` | Read-only pinned oracle; do not link its JS into Rust crates |
| `app.js` | `app-core` | `legacy-application-orchestration` | Replace with immutable typed jobs and explicit state |
| `index.html`, `renderer.html`, bridge, CSS | `legacy-renderer-adapter` | `legacy-dom-and-css-renderer` | Keep only for compatibility/export; no browser reflow in core |
| settings/presets/default settings | `settings` | `legacy-profile-settings-loader` | Lossless storage; defaults resolved only inside owning profile |
| font binaries and font CSS | `font-environment` | `font-environment-inventory` | Verify tables, glyph coverage, aliases and license before packaging |
| SVG/images | `asset-registry` | `legacy-asset-loader` | Validate, hash and register offline dependencies |
| text corpus and fixtures | `compat-test-harness` | `legacy-fixture-runner` | Preserve exact bytes and profile identity |
| Typesetter_doc markdown/PDF | `docs/requirements` | `requirements-and-research` | Spec-only; promote reviewed requirements to features/ADRs |
| source snapshots/transcripts | `docs/history` | `documentation-and-history` | Historical-only; not executable evidence |
| build/test infrastructure | `build-system` / `compat-test-harness` | corresponding legacy tooling | Port only the required test intent or pinned-runner setup |

## Important interpretation

- `rustOwner` means "the Rust subsystem responsible for resolving or migrating this source record"; it does not mean that subsystem is already implemented.
- `parserOrConsumer` records the legacy consumer or intended reader boundary.
- `parityFixtures` contains exact repository/commit/path references for profile inputs and smoke fixtures where available.
- `assetDependencies` remains empty until CSS, SVG and JSON reference edges are extracted and validated. Empty is not evidence of no dependencies.
- `verificationLevel=scanned-hash-only` means only that exact file bytes were inventoried.
- Fonts are not cleared for redistribution by a hash scan. Font licensing and glyph coverage remain a later mandatory audit.
- Any heuristic owner assignment can be overridden only by a reviewed compatibility record and a corresponding ADR/fixture update.
