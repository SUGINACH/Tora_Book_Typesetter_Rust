# Hebrew shaping and bidi prototype

This isolated prototype tests a maintained HarfBuzz-compatible Rust shaper without adding it to the production workspace.

## Exact source pins

- HarfRust repository: `harfbuzz/harfrust`, tag commit `71d965130ea93905f81b8f03a03d7ecc794fbc47`, version 0.15.0.
- Bidi algorithm: `unicode-bidi = 0.3.18`.
- Font: Google Fonts `NotoSerifHebrew[wdth,wght].ttf` at commit `6c1dbe27dcf7d817d4b62cec033ba77fa8e2a5ec`; OFL license file is checked out beside it.
- The probe is a separate Cargo workspace and does not add HarfRust or unicode-bidi to the production Cargo workspace.

## Corpus

Eight lines cover Hebrew without niqqud, niqqud, cantillation, maqaf, mixed Hebrew/Latin/digits/parentheses/brackets, and Latin ligature opportunities.

## What is measured

- HarfRust `hr-shape` output with glyph indices, positions, advances, clusters, extents and flags; `--verify` is enabled.
- Unicode bidi paragraph levels, per-character levels and the visual string returned by `BidiInfo::reorder_line`.
- Exact font, license, corpus, Git commit, Rust toolchain and platform fingerprints.

## Deliberate limits

This is a component probe, not a complete bidi/shaping integration. Mixed-direction strings are analyzed by unicode-bidi but not yet split into script/direction runs before shaping. No final library selection is made until that integration, native HarfBuzz comparison, Hebrew mark positioning, missing glyphs, variable-font axes and cross-platform repeat tests are complete.
