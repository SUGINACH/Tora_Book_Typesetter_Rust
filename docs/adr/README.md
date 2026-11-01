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
