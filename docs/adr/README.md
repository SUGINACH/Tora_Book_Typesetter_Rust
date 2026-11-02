# Architecture Decision Records

ADRs are immutable decision records. Accepted decisions are binding; proposed records describe the evidence still required and do not authorize implementation choices.

| ADR | Subject | Status |
|---|---|---|
| [0001](0001-legacy-profile-isolation.md) | Legacy profile isolation and renderer-independent Rust core | Accepted |
| [0002](0002-fixed-point-representation.md) | Fixed-point representation and unit system | Proposed — numeric prototype required |
| [0003](0003-hebrew-shaping-and-bidi.md) | Hebrew shaping, bidi and font parsing stack | Proposed — Hebrew corpus experiment required |
| [0004](0004-pdf-type0-tounicode.md) | PDF Type0 embedding and ToUnicode | Proposed — output prototype required |
| [0005](0005-desktop-shell-and-ipc.md) | Desktop shell and minimal IPC boundary | Proposed — Windows feasibility experiment required |

No dependency version or technology is selected by a proposed ADR. A selection requires an attached reproducible fixture, command, exact environment, output artifact and measured result.


## Stage A evidence checkpoint — 2026-11-02

ADRs 0002–0005 remain **proposed**. The reproducible experiments are recorded in their individual files, but their open acceptance gaps do not justify production dependency selection: fixed-point lacks Rust-level checked arithmetic and real Hebrew geometry; shaping lacks mixed-direction run segmentation and complete source-span mapping; PDF is a one-page/three-line prototype; Tauri lacks an actual WebView IPC round-trip. ADR-0001 remains the only accepted decision. Do not add the candidate libraries to the production workspace on the basis of prototype evidence alone.
