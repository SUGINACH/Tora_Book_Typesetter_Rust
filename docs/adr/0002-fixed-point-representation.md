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

## Experiment 1 — exact rational scale comparison

A standard-library-only reproducible prototype is in `experiments/fixed-point-scale.py`; its output is committed as `experiments/fixed-point-scale-results.json`.

Inputs: A4 `595.276 × 841.89 pt`, line height `13.2 pt`, 1,000 repeated lines, and `4/3 CSS px per point`.

| Candidate | Quantum | Independent rounding drift after 1,000 lines | A4 width ticks | Positive i64 range |
|---|---|---:|---:|---:|
| 1/64 CSS px | 1/64 px | -6.25 CSS px | 50,797 | ~1.08×10^17 pt |
| 1/1024 pt | 1/1024 pt | +0.2604 CSS px | 609,563 | ~9.01×10^15 pt |
| 1/65536 pt | 1/65536 pt | -0.00407 CSS px | 39,012,008 | ~1.41×10^14 pt |

All three have zero final drift in this particular 1,000-line case when the cumulative exact position is rounded with residual carry. This demonstrates that a fine quantum alone does not prevent drift: the accumulation/rounding policy is equally important. `1/65536 pt` exactly represents a legacy `1/64 CSS px` quantum (768 ticks), but this is not yet a production selection.

## Reproducibility evidence — 2026-11-01

The exact-rational scale calculation was rerun in GitHub Actions as part of [Rust CI run 41347908551](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41347908551) and passed. The committed JSON result is `experiments/fixed-point-scale-results.json`.

This confirms reproducibility of the comparison only. It does **not** complete the required property tests for signed conversions, percentage/font-unit conversions, tie cases, checked overflow or real font/page geometry. No production scale is selected by this result.

## Required follow-up experiment

Build property tests and arithmetic prototypes for:
- px↔pt, percentages, font units and page-template conversion;
- negative coordinates, zero, ties, half-way rounding and repeated accumulation;
- checked add/subtract/multiply/divide and overflow boundaries;
- invariance under operation order where the mathematical operation is commutative;
- maximum supported page dimensions and 1,000-page accumulation;
- conversion of the pinned Orange smoke geometry and a Hebrew/niqqud font fixture.

Record max absolute/relative error, overflow behavior, canonical serialized values and CPU cost. No epsilon may enter the Rust decision comparator.

## Acceptance

Choose one representation only after the remaining alternatives have a recorded result, a reproducible fixture/hash, checked-overflow/property tests and a documented conversion/rounding rule. The current scale study is insufficient to accept an implementation.
