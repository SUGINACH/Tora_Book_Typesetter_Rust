# PDF Type0 / ToUnicode prototype

This isolated probe passes externally shaped HarfRust glyph IDs and positions to printpdf. It does not add either dependency to the production workspace.

## Pins

- HarfRust 0.15.0: commit `71d965130ea93905f81b8f03a03d7ecc794fbc47`.
- printpdf 0.12.8, with `text_layout` enabled.
- Noto Serif Hebrew variable TTF and its OFL file from Google Fonts commit `6c1dbe27dcf7d817d4b62cec033ba77fa8e2a5ec`.

## Probe contract

- Shape Hebrew with niqqud and Latin text with HarfRust.
- Transfer glyph IDs, clusters, advances and offsets into `TextItem::GlyphIds`.
- Emit glyphs in logical cluster order while retaining HarfRust's absolute visual positions.
- Populate `Codepoint.cid` with the corresponding source Unicode string, including multi-scalar strings for ligatures.
- Subset and embed the font.
- Require a Type0 font, ToUnicode CMap, `pdffonts` embedding evidence and exact independent `pdftotext -raw` extraction.

## Limitations

This is a focused writer contract test, not a full PDF backend selection. It does not test every PDF viewer, full-page text extraction, PDF/A, color management, bookmarks, accessibility structure or large-document performance. A failed ToUnicode mapping conflict is recorded as a backend limitation rather than hidden.
