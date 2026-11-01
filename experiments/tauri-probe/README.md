# Tauri 2 desktop-shell and IPC prototype

This isolated Windows prototype checks that a Tauri WebView can host a UI while typed, immutable jobs and generation/cancellation state remain in Rust.

## Pins

- `tauri = 2.12.5`
- `tauri-build = 2.7.4`
- `serde = 1.0.228`
- Rust toolchain: workspace-pinned 1.99.1

## Contract tested

- Typed `LayoutJob` input carries job ID, profile ID, generation and input/settings SHA-256 identities.
- A Rust `JobGate` rejects stale generations and invalidates a cancelled generation.
- Tauri commands expose submit/cancel operations; the WebView does not measure text or decide pagination.
- The prototype builds on Windows CI and unit-tests the generation gate.

## Deliberate limits

This does not yet test file-dialog allowlists, persistent project storage, installer/signing, runtime WebView2 availability, accessibility, or a full progress/event stream. It is a shell/IPC feasibility probe, not a production application decision.
