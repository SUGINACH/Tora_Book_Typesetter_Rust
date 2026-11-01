# Isolated legacy renderer runner

This runner executes the pinned Orange V23 browser implementation as a read-only compatibility oracle. It is deliberately outside the Rust workspace and does not import legacy JavaScript into any Rust crate.

## Pins

- Repository: `SUGINACH/Tora_Book_Typesetter_Orange`
- Commit: `0b0ee279464a3db14672cd3f4051275aba5985da`
- Profile: `legacy-orange-v23`
- Node: 22
- Playwright: 1.55.0 (locked by the pinned Orange package-lock)
- Browser: Chromium installed by the pinned Playwright package
- Input: `tests/fixtures/geometry-audit-smoke.txt`
- Offline behavior: the pinned CLI serves the checkout on loopback and aborts/records every browser request outside loopback/data/blob/about. The runner fails if any external request is recorded.
- Font behavior: the pinned CLI waits for `document.fonts.ready` before typesetting; the self-hosted font CSS is SHA-256 pinned and its fingerprint is checked in the report.

The source commit, CLI, fixture, font CSS, package lock and source file hashes are recorded in `docs/provenance/legacy-source-pins.json`.

## CI execution

The workflow `.github/workflows/legacy-runner.yml` checks out the exact Orange commit into `legacy/orange`, installs only that checkout's locked npm dependencies, installs its pinned Chromium, then invokes this wrapper.

The wrapper validates:
- exact Git commit and SHA-256 for CLI, fixture and self-hosted font CSS;
- Playwright and Chromium version metadata;
- zero external browser requests;
- repeat-run layout signature equality;
- nonempty rendered page set and decision ledger;
- decision-ledger/DOM count parity;
- availability of source-to-rendered text conservation and font diagnostics;
- zero browser page errors.

It writes a machine-readable manifest to `artifacts/legacy-runner-manifest.json`; the raw geometry report and Markdown summary remain in the isolated legacy checkout under `legacy/orange/artifacts/` and are uploaded by CI.

## Isolation guarantees and limits

- The Rust Cargo workspace does not depend on Node, npm, Playwright, Chromium, DOM APIs or legacy source files.
- The legacy checkout is a separate directory at a detached, exact commit. It is not copied into, merged with or modified by the Rust crates.
- Dependency installation and browser installation require CI network access. The browser execution itself is offline-enforced by request interception; this is not a claim that the CI machine has no network interface.
- This first runner is a smoke fixture and a reproducibility contract. It is not yet the complete corpus/baseline matrix; those are later tasks.
