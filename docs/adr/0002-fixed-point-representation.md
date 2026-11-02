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

## Experiment 2 — exact conversion/property model

The experiment now includes a deterministic exact-rational property suite, recorded in `experiments/fixed-point-scale-results.json`:

- 24,579 signed CSS-pixel ↔ point ↔ tick round-trip cases across the three candidate scales.
- 2,408 percentage-rounding cases.
- 588 font-unit conversions across UPEM values 1000, 1024, 2048 and 4096, and font sizes 6–72 pt.
- 41 commutative integer-addition cases within signed-i64 bounds.
- 7 signed-i64 boundary/overflow cases.
- Four explicit positive/negative half-way rounding cases.

Total: **27,627 deterministic checks**. The tested 1/64 CSS-pixel grid round-trips with zero error for all three candidate scales. The declared tie policy is nearest, exact ties away from zero.

These are Python `Fraction` model checks, not tests of a compiled Rust `Fixed` implementation. They do not yet prove checked i128 intermediates, CPU cost, real-font metrics, page geometry or 1,000-page corpus behavior. No scale is selected.

## Reproducibility evidence — 2026-11-01

The exact-rational scale calculation was rerun in GitHub Actions as part of [Rust CI run 41347908551](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41347908551) and passed. The committed JSON result is `experiments/fixed-point-scale-results.json`.

The scale comparison and exact-rational model checks are reproducible. Rust-level checked arithmetic, i128 intermediate behavior, real font/page geometry and CPU cost remain untested. No production scale is selected by these results.

## Required follow-up experiment

Build Rust-level property tests and arithmetic prototypes for:
- px↔pt, percentages, font units and page-template conversion;
- negative coordinates, zero, ties, half-way rounding and repeated accumulation;
- checked add/subtract/multiply/divide with i128 intermediates and overflow boundaries;
- invariance under operation order where the mathematical operation is commutative;
- maximum supported page dimensions and 1,000-page accumulation;
- conversion of the pinned Orange smoke geometry and a Hebrew/niqqud font fixture.

Record max absolute/relative error, overflow behavior, canonical serialized values and CPU cost. No epsilon may enter the Rust decision comparator.

## Acceptance

Choose one representation only after the remaining alternatives have a recorded result, a reproducible fixture/hash, checked-overflow/property tests and a documented conversion/rounding rule. The current scale study is insufficient to accept an implementation.


## Stage A technology-gate update — 2026-11-02

The fixed-point model was rerun by [Rust CI 41395791603](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41395791603); the reproducibility job passed. The committed exact-rational study contains 27,627 deterministic Fraction-model checks, including signed px/pt/tick conversions, percentages, font-unit conversions, i64 boundaries and tie cases. Residual-carry accumulation removes drift in the tested 1,000-line case.

This is not a Rust Fixed implementation test. It does not prove checked i128 intermediates, overflow behavior in production code, real Hebrew/niqqud font geometry, maximum supported page dimensions, 1,000-page accumulation or CPU cost. The candidates remain 1/64 CSS px, 1/1024 pt, 1/65536 pt, rational scaling and font-unit coordinates. The reason for not selecting one is evidentiary: the current experiment is an exact arithmetic model, not a compiled engine geometry path. No candidate is accepted and no production dependency or scale is locked by this ADR.

**Task 10 gate:** open. Required next evidence is a Rust-level checked-arithmetic prototype plus the pinned Hebrew/niqqud geometry fixture and measured large-page accumulation.
