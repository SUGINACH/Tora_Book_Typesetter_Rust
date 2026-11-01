# ADR-0001: Isolate legacy profiles and keep the Rust layout core renderer-independent

- Status: accepted
- Date: 2026-11-01
- Decision owners: typesetter architecture
- Related tasks: 3, 4, 5, 8, 9
- Related profiles: `legacy-orange-v23`, `legacy-green-v23`, `legacy-gold-v21`, `legacy-yellow-v10`, `legacy-imud-v20x`

## Context

The six pinned source repositories contain overlapping filenames and multiple generations of behavior. The core files `typesetter.js`, `app.js`, `decorations.js`, `style.css` and `index.html` differ across the five executable repositories. Orange documents a clean-history import from Green V23 and separate Gold V21 capability ports; that ancestry does not make the current branches equivalent.

The target requirement is a deterministic, transparent engine whose pagination decisions do not depend on browser DOM measurements. The legacy browser is useful as a reproducible compatibility oracle, but its measured geometry is not an authority for Rust's internal arithmetic.

Evidence:
- `docs/provenance/legacy-source-pins.json`
- `docs/provenance/source-of-truth-matrix.md`
- `legacy/corpus-manifest.json`
- `tools/legacy-runner/README.md`
- `docs/compat/parity-matrix.md`

## Decision

1. Every legacy generation is represented by a named immutable profile with a full source commit, input/config/font/asset identities and its own fixtures/baselines.
2. Green V23, Orange V23 and Gold V21 remain separate profiles. No branch merge, algorithm substitution or default-setting inheritance is allowed between them without a new ADR and an explicit migration contract.
3. Yellow V10 and IMUD V20.x retain their own historical defaults, aliases, assets and compatibility rules.
4. The Rust layout core accepts immutable, explicit inputs and returns deterministic output/decision records. It does not query a DOM, CSS layout engine, browser canvas or WebView for measurements or pagination decisions.
5. The pinned legacy Chromium/Playwright runner is an external read-only oracle. It may generate DOM measurements and legacy decision-ledger evidence, but its JavaScript is not linked into Rust crates and its output is never silently treated as Rust geometry.
6. Conflicts between executable code, data, tests and documentation are recorded with both pinned sources. Documentation does not override executable evidence; a target requirement that intentionally differs is handled by a separate migration/feature decision.

## Alternatives considered

| Alternative | Decision | Reason |
|---|---|---|
| Merge Gold V21 balancing code into Green/Orange V23 | Rejected | The plan explicitly requires separate profile behavior; merging destroys the ability to identify regressions and reproduce branch history |
| Treat Orange as the universal latest source | Rejected | Orange is a distribution-oriented V23 line; Green contains distinct engineering/audit history and Gold remains a V21 profile |
| Use the browser renderer as Rust's measurement oracle at runtime | Rejected | It would make layout dependent on renderer/font load timing and violate the deterministic-core requirement |
| Copy all legacy files into the Rust crates | Rejected | Legacy source is pinned and executed separately; imports would create accidental coupling and unclear ownership |
| Keep the six repos unpinned and use their current HEADs | Rejected | "Latest" is not reproducible and branch heads can move independently |

## Experiment and reproducibility

The decision is architectural and does not require a layout benchmark to establish profile isolation. The first isolated renderer smoke run is supporting evidence:
- Orange commit: `0b0ee279464a3db14672cd3f4051275aba5985da`
- Playwright: `1.55.0`
- Chromium: `140.0.7339.16`
- Viewport: `1440×1000`
- 485/485 font faces loaded; 0 external requests.
- 6 pages; 2/2 decision-ledger records matched DOM blocks.
- Repeat-run page signatures were identical.
- [Workflow artifact run](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41344525335)

This is smoke evidence, not proof of full parity.

## Consequences

- Every setting, fixture, font and baseline must carry a profile identity.
- Shared file hashes may be deduplicated as immutable data, but behavior is not shared merely because filenames or some JSON blobs match.
- The compatibility manifest must distinguish executable source, data, historical-only material and spec-only material.
- New Rust behavior must have an explicit owner, profile policy and parity/migration tests.
- The renderer-independent boundary is testable: core crates must not depend on browser/DOM APIs or read layout measurements from an external renderer.

## Acceptance tests

- A profile-loading test must reject unknown profile IDs rather than silently falling back to another profile.
- A settings test must prove missing keys resolve using the selected profile's defaults.
- A determinism test must show identical canonical layout and ledger hashes for identical immutable inputs.
- A dependency/architecture check must ensure the engine core has no browser/DOM/WebView dependency.
- A parity test must never compare or reuse a V21 baseline as a V23 baseline.
