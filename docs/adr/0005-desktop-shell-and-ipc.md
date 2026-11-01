# ADR-0005: Desktop shell and minimal IPC boundary

- Status: proposed — feasibility experiment required
- Date: 2026-11-01
- Related tasks: 10, 71–79
- Related profiles: application shell; engine core remains independent

## Context

The product is expected to run as a Windows desktop application, but the engine must remain independently testable and must not depend on WebView layout. The UI must send immutable snapshots to the Rust application core and receive progress, cancellation and rendered artifacts.

## Alternatives to test

| Candidate | Experiment | Acceptance concern |
|---|---|---|
| Tauri 2 | Minimal window, typed commands, file dialogs, event progress, cancellation and offline assets | IPC validation, filesystem allowlist, WebView update behavior, installer size and Windows signing |
| Native Rust shell / egui-style UI | Minimal editor/preview host consuming the same app-core API | RTL editing maturity, accessibility, PDF/SVG preview and long-document responsiveness |
| CLI-first plus later shell | Prove app-core and export pipeline without a GUI | Keeps engine independent but delays editor/preview UX |

No shell is selected.

## Required experiment

Build the same small immutable `LayoutJob` through each feasible shell/host. The UI may submit content/configuration and display progress/artifacts; it may not supply measured text widths, page breaks or DOM geometry. Verify cancellation and stale-generation rejection.

## Acceptance

Choose only after a working Windows prototype demonstrates safe typed IPC, offline operation, project save/load, cancellation and no renderer-dependent layout decisions. Record exact toolchain, shell and dependency versions.
