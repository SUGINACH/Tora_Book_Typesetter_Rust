# ADR-0002: Fixed-point representation and unit system

- Status: proposed — experiment required
- Date: 2026-11-01
- Related tasks: 10, 21, 22, 34
- Related profiles: all legacy profiles; numeric conversion policies remain profile-scoped

## Context

The Rust engine must make layout decisions without floating-point nondeterminism. The comprehensive plan requires an `i64`-based fixed representation, checked arithmetic, explicit conversions and defined rounding/anchoring. The exact quantum and canonical unit have not yet been proven against real Hebrew/font geometry and large books.

## Alternatives to test

| Candidate | Proposed representation | Questions |
|---|---|---|
| A | 1/64 CSS px in signed i64 | Compatibility with legacy DOM measurements; conversion to PDF points; accumulated rounding |
| B | 1/1024 pt in signed i64 | Print-native units; browser px conversion; subpixel precision |
| C | Rational scale with explicit numerator/denominator | Exact conversions; cost and serialization complexity |
| D | Font-unit coordinates inside shaping, fixed physical units after shaping | Separation of font metrics from page geometry; conversion stability |

No candidate is selected by this ADR.

## Required experiment

Build a small standalone arithmetic prototype and property tests for:
- px↔pt, percentages, font units and page-template conversion;
- negative coordinates, zero, ties, half-way rounding and repeated accumulation;
- checked add/subtract/multiply/divide and overflow boundaries;
- invariance under operation order where the mathematical operation is commutative;
- maximum supported page dimensions and 1,000-page accumulation;
- conversion of the pinned Orange smoke geometry and a Hebrew/niqqud font fixture.

Record max absolute/relative error, overflow behavior, canonical serialized values and CPU cost. No epsilon may enter the Rust decision comparator.

## Acceptance

Choose one representation only after all alternatives have a recorded result, a reproducible fixture/hash, checked-overflow tests and a documented conversion/rounding rule.
