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

## Research snapshot — 2026-11-01

- The Tauri 2 line is actively released; the official ecosystem release page lists Tauri 2.12.3 dated 2026-10-10, while docs.rs currently exposes `tauri 2.12.5` (released 2026-10-21).
- Tauri 2.12 raised its MSRV to Rust 1.90. The target workspace pins Rust 1.99.1, so the declared MSRV is compatible, but Windows/WebView2 packaging and native build dependencies still require a real Windows CI prototype.
- Tauri 3.0.0-alpha.5 exists (2026-10-14); it is not a production candidate while alpha.
- A shell experiment must keep the engine core independent: Tauri may present SVG/PDF/HTML artifacts and submit typed immutable jobs, but must not measure text or decide page/column breaks.

Sources:
- https://tauri.app/release/
- https://docs.rs/tauri/latest/tauri/
- https://tauri.app/blog/tauri-2.12/

## Prototype run 1 — Windows build feasibility

Evidence: [Tauri workflow run 41347404787](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41347404787), source commit `795a82abcfddf0015efd3ecb7f4fa0a68486fd96`.

- Windows formatting, deterministic resource icon generation, debug build, unit tests, Clippy, release build and dependency graph capture all passed.
- The exact graph included `tauri 2.12.5` and `tauri-build 2.7.4`.
- The probe demonstrated a minimal offline local frontend, typed command signatures and a generation gate that rejects stale/cancelled jobs.
- Scope limitation: this run did **not** exercise an actual WebView `invoke` round-trip, file dialogs, project save/load, progress events or packaged installer. The bundle is intentionally disabled. It is build feasibility evidence, not acceptance of the desktop architecture.
- Follow-up [run 41348380250](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41348380250) passed after adding strict validation for profile IDs, job IDs, positive generations and 64-character hexadecimal SHA-256 fingerprints. Tests prove invalid input does not advance the generation gate; stale generations are rejected. This remains a Rust-side command-handler test, not an actual WebView invoke round-trip.

**Current result:** Tauri 2.12.5 is the first candidate to test, not an accepted selection. The remaining experiment must prove an actual IPC round-trip, allowlisted project I/O, progress/cancellation events, offline operation and stale-generation rejection before shell selection.
