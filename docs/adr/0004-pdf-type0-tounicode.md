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
