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
