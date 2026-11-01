# ADR-0003: Hebrew shaping, bidi and font parsing stack

- Status: proposed — corpus experiment required
- Date: 2026-11-01
- Related tasks: 10, 23–30
- Related profiles: all; font aliases and fallback remain profile-scoped

## Context

Correct Hebrew layout requires bidi itemization, script/style runs, shaping, mark positioning, ligatures, cluster boundaries, glyph advances and source mapping. A glyph count is not a character count. Missing fonts and glyphs must be diagnosed, never silently replaced by generic serif.

## Alternatives to test

| Candidate stack | Components to evaluate | Primary risk |
|---|---|---|
| A | rustybuzz + unicode-bidi + ttf-parser | Integration of bidi levels, font tables, variation/features and cluster/source mapping |
| B | HarfBuzz through a native binding + ICU bidi | Native packaging, platform consistency and build reproducibility |
| C | swash + unicode-bidi + font parser | Coverage of required OpenType features and compatibility with exact font inventory |

These are candidates only; this ADR does not select a crate or version.

## Required Hebrew corpus

- Hebrew with and without niqqud, dagesh, shin/sin dots and cantillation marks.
- Mixed Hebrew/Latin/numbers, parentheses and nested square/round scopes.
- Ligatures, combining marks, PUA glyphs and missing glyphs.
- At least the fonts/aliases pinned for Orange, Green, Gold, Yellow and IMUD.
- SourceSpan/cluster mapping through shaping and back to logical copy/search order.

## Required measurements

For each stack record glyph IDs, advances, offsets, cluster map, bidi runs, line-break opportunities, missing-glyph behavior, font table support, output hash, CPU time and platform fingerprint. Include a PDF text extraction check for logical reading order.

## Acceptance

The selected stack must preserve Hebrew marks and clusters, provide stable source mapping, report font/fallback identity, run on supported Windows/Linux/macOS targets and reproduce canonical shaped-run hashes under the pinned environment.

## Research snapshot — 2026-11-01

- `harfrust 0.15.0` is an actively released pure-Rust port matching HarfBuzz v15.0.0 (released 2026-10-21) with MSRV 1.85. It uses `read-fonts 0.46.0`, exposes glyph buffers/positions and shaping traces, and publishes the `hr-shape` utility for inspecting glyph IDs, advances, offsets, clusters and flags. It is the leading prototype candidate, not an approved dependency.
- HarfRust's own documentation lists important limits: no FreeType/CoreText/Uniscribe/DirectWrite font-loading integration, no ICU/GLib integration, and no Graphite support. Those constraints are acceptable only if the engine owns a pinned font registry and our required fonts/features pass the Hebrew corpus.
- `unicode-bidi 0.3.18` implements UAX #9 in safe Rust. Its documented model resolves bidi levels and reorders line ranges after line wrapping; the engine must shape logical runs and reorder visual fragments at the correct stage.
- `rustybuzz 0.20.1` is a pure-Rust HarfBuzz port but is currently flagged unmaintained (RUSTSEC-2026-0206). Do not select it unless the advisory is resolved or a maintained fork is audited.
- `tectonic-rustybuzz 0.20.2` remains a comparison/fallback candidate only; its maintenance status, source divergence, security posture and parity with upstream must be checked.

Sources:
- https://docs.rs/harfrust/0.15.0/harfrust/
- https://github.com/harfbuzz/harfrust/blob/main/CHANGELOG.md
- https://docs.rs/hr-shape/0.15.0/hr_shape/
- https://docs.rs/unicode-bidi/latest/unicode_bidi/
- https://docs.rs/rustybuzz/latest/rustybuzz/
- https://rustsec.org/advisories/RUSTSEC-2026-0206.html
- https://docs.rs/crate/tectonic-rustybuzz/latest
- https://github.com/taiki-e/rustsec-advisory-db (advisory database; inspect the exact RUSTSEC-2026-0206 record before any dependency selection)

**Current result:** no crate/version selected. The next experiment must run Hebrew+niqqud and mixed-direction fixtures through at least two viable shaping stacks and inspect cluster/source mapping, not just visual screenshots.
