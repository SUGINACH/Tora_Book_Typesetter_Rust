# ADR-0004: PDF backend, Type0 embedding and ToUnicode

- Status: proposed — output prototype required
- Date: 2026-11-01
- Related tasks: 10, 66, 67, 70
- Related profiles: output backend shared; source text and glyph mapping are profile-independent inputs

## Context

The PDF must contain real embedded/subset fonts and preserve searchable/copyable Hebrew text. Visual raster equality alone is insufficient: PDF text extraction must map glyph codes through ToUnicode to logical Unicode text and maintain RTL order, niqqud and ligatures.

## Alternatives to test

| Candidate | Approach | Primary risk |
|---|---|---|
| A | Low-level PDF object construction (e.g. lopdf) | Correct Type0/CIDFontType2, widths, CID mapping, ToUnicode CMap, subset naming and resource management |
| B | Higher-level PDF library (e.g. printpdf) with custom font/text layer | Whether the library exposes enough control over Type0, glyph IDs, subsetting and ToUnicode |
| C | Dedicated PDF writer with a shaping-aware text API | Dependency maturity, deterministic output and ability to preserve the engine's glyph/source map |

No library is selected.

## Required prototype

Generate a small multi-page Hebrew sample with niqqud, mixed Latin/numbers, parentheses, ligatures and a footnote. Embed a real font subset. Include ToUnicode CMap and a stable glyph-to-source map.

Validate:
- font embedding and subset tables;
- text extraction with an independent PDF extractor;
- logical Hebrew order, punctuation and combining marks after extraction;
- raster output and bounding boxes against the DisplayList;
- deterministic content streams after excluding permitted container metadata;
- hyperlinks/bookmarks and page anchors in a second pass.

## Acceptance

Reject any backend that requires converting all text to outlines or cannot prove logical Unicode extraction. Selection requires both visual and text-extraction evidence, pinned library version and cross-platform build tests.

## Research snapshot — 2026-11-01

- `printpdf 0.12.8` documents embedded fonts, Unicode text output, automatic font subsetting, and advanced text operations. Its own documentation labels text extraction as experimental. This is useful functionality but does not by itself prove correct Type0/CID mapping and ToUnicode behavior for Hebrew niqqud and shaped glyph clusters.
- `lopdf 0.46.0` is a lower-level PDF object manipulation library. It can support a custom writer, but a correct Type0 font dictionary, CIDFont, widths, CIDToGIDMap, ToUnicode CMap and subset remapping would remain our responsibility.
- A dedicated font/PDF crate exposes ToUnicode parsing and subsetting capabilities, but its suitability for PDF *writing* and the exact Type0 mapping still needs a direct prototype.

Sources:
- https://docs.rs/crate/printpdf/latest
- https://docs.rs/lopdf/latest/lopdf/
- https://docs.rs/prismpdf-fonts/latest/pdf_fonts/

- `lightweight-pdf 0.3.0` is a recent MIT-licensed Rust project whose writer emits `/Subtype /Type0`, a `/CIDFontType2` descendant, `Identity-H`, `CIDToGIDMap /Identity` and a `ToUnicode` CMap. Its tests invoke `pdftotext` and verify extraction for German special characters.
- Important limitation: the project explicitly says it is **not a typesetting system**. Its `CidFont.to_unicode` field is a list of `(CID, Unicode scalar)` pairs, which does not by itself prove correct mapping for ligatures or shaped clusters that correspond to multiple Unicode scalars. Its font subsystem uses `skrifa` and direct character-to-glyph lookup; that is not evidence of Hebrew GPOS mark positioning or bidi shaping.
- The library is therefore a valuable Type0/ToUnicode implementation reference and possible writer-only experiment, not a candidate for the layout engine. Reusing its writer crate would require a separate API test to feed externally shaped glyph IDs and multi-scalar cluster mappings.
- `printpdf 0.12.8` documents embedded fonts and subsetting but calls text extraction experimental; it remains unproven for the required Hebrew round-trip.
- `lopdf 0.46.0` offers low-level PDF objects but leaves the font/CMap correctness burden with this project.

Sources:
- https://github.com/casoon/lightweight-pdf
- https://docs.rs/lightweight-pdf/latest/lightweight_pdf/
- https://docs.rs/lopdf/latest/lopdf/
- https://docs.rs/crate/printpdf/latest

**Current result:** no backend selected. The acceptance test remains independent text extraction of Hebrew+niqqud from a real Type0 subset, followed by logical-order comparison and raster comparison against the DisplayList.
