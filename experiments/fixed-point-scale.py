#!/usr/bin/env python3
"""Reproducible fixed-point scale comparison; stdlib only.

This is an architecture experiment, not production layout code. It compares
quantization and repeated-rounding drift for common print/browser units.
"""

from fractions import Fraction
import json
import math


I64_MAX = 2**63 - 1
A4_WIDTH_PT = Fraction(595276, 1000)
A4_HEIGHT_PT = Fraction(841890, 1000)
LINE_HEIGHT_PT = Fraction(132, 10)
LINE_COUNT = 1000


def round_nearest(value: Fraction) -> int:
    """Round positive or negative rational values to nearest, ties away from zero."""
    if value < 0:
        return -round_nearest(-value)
    return (2 * value.numerator + value.denominator) // (2 * value.denominator)


def describe(name: str, ticks_per_pt: Fraction) -> dict:
    line_ticks = round_nearest(LINE_HEIGHT_PT * ticks_per_pt)
    independent_total_pt = Fraction(line_ticks * LINE_COUNT, 1) / ticks_per_pt
    exact_total_pt = LINE_HEIGHT_PT * LINE_COUNT

    # Error-carry strategy: round the cumulative exact position, not each
    # line-height independently. This keeps the final endpoint within one
    # half-quantum rather than multiplying the same rounding error by N.
    cumulative_ticks = round_nearest(exact_total_pt * ticks_per_pt)
    carried_total_pt = Fraction(cumulative_ticks, 1) / ticks_per_pt

    return {
        "name": name,
        "ticksPerPoint": str(ticks_per_pt),
        "quantumPt": str(1 / ticks_per_pt),
        "quantumCssPx": str(Fraction(4, 3) / ticks_per_pt),
        "a4WidthTicks": round_nearest(A4_WIDTH_PT * ticks_per_pt),
        "a4HeightTicks": round_nearest(A4_HEIGHT_PT * ticks_per_pt),
        "lineHeightPt": str(LINE_HEIGHT_PT),
        "lineHeightTicks": line_ticks,
        "independentRoundingDriftAfter1000LinesPt": str(independent_total_pt - exact_total_pt),
        "independentRoundingDriftAfter1000LinesCssPx": str((independent_total_pt - exact_total_pt) * Fraction(4, 3)),
        "cumulativeErrorCarryDriftAfter1000LinesPt": str(carried_total_pt - exact_total_pt),
        "maxPositivePhysicalRangePt": str(Fraction(I64_MAX, 1) / ticks_per_pt),
    }


def checked_i64(value: int) -> int:
    if value < -(2**63) or value > I64_MAX:
        raise OverflowError("signed i64 overflow")
    return value


def run_property_checks(candidates: list[dict]) -> dict:
    """Deterministic exhaustive conversion/rounding/overflow checks."""
    counts = {
        "cssPxToPtAndBack": 0,
        "percentageRounding": 0,
        "fontUnitConversion": 0,
        "commutativeIntegerAddition": 0,
        "overflowBoundary": 0,
    }
    max_round_trip_error_px = Fraction(0)

    # Exhaust all 1/64 CSS-pixel positions across a signed interval. All
    # three candidate scales must represent these exact positions.
    for candidate in candidates:
        scale = Fraction(candidate["ticksPerPoint"])
        for sixty_fourths in range(-4096, 4097):
            css_px = Fraction(sixty_fourths, 64)
            points = css_px * Fraction(3, 4)
            ticks = round_nearest(points * scale)
            reconstructed_px = Fraction(ticks, 1) / scale * Fraction(4, 3)
            error = abs(reconstructed_px - css_px)
            assert error <= Fraction(1, 2) / scale * Fraction(4, 3)
            max_round_trip_error_px = max(max_round_trip_error_px, error)
            counts["cssPxToPtAndBack"] += 1

    # Percentages use exact integer ratios and one explicit final rounding.
    for basis_ticks in (0, 1, 2, 7, 64, 1024, 65536, 1_000_000):
        for percent in range(-100, 201):
            exact = Fraction(basis_ticks * percent, 100)
            rounded = round_nearest(exact)
            assert abs(Fraction(rounded) - exact) <= Fraction(1, 2)
            counts["percentageRounding"] += 1

    # Signed font-unit conversions over several common UPEM values and sizes.
    for units_per_em in (1000, 1024, 2048, 4096):
        for font_size_pt in (6, 8, 10, 12, 18, 36, 72):
            for font_units in (-units_per_em, -units_per_em // 2, -1, 0, 1,
                               units_per_em // 2, units_per_em):
                for candidate in candidates:
                    scale = Fraction(candidate["ticksPerPoint"])
                    exact_ticks = (
                        Fraction(font_units, units_per_em)
                        * font_size_pt
                        * scale
                    )
                    rounded_ticks = round_nearest(exact_ticks)
                    assert abs(Fraction(rounded_ticks) - exact_ticks) <= Fraction(1, 2)
                    counts["fontUnitConversion"] += 1

    # Integer addition is exact and commutative until a checked boundary.
    values = (-I64_MAX, -1_000_000, -1, 0, 1, 1_000_000, I64_MAX)
    for left in values:
        for right in values:
            if -(2**63) <= left + right <= I64_MAX:
                assert checked_i64(left + right) == checked_i64(right + left)
                counts["commutativeIntegerAddition"] += 1

    for boundary in (-(2**63), -1, 0, 1, I64_MAX):
        assert checked_i64(boundary) == boundary
        counts["overflowBoundary"] += 1
    for overflow in (-(2**63) - 1, I64_MAX + 1):
        try:
            checked_i64(overflow)
        except OverflowError:
            counts["overflowBoundary"] += 1
        else:
            raise AssertionError(f"overflow was not rejected: {overflow}")

    # Explicit signed tie policy: nearest, ties away from zero.
    tie_cases = {
        Fraction(1, 2): 1,
        Fraction(3, 2): 2,
        Fraction(-1, 2): -1,
        Fraction(-3, 2): -2,
    }
    for value, expected in tie_cases.items():
        assert round_nearest(value) == expected

    return {
        "counts": counts,
        "totalChecks": sum(counts.values()) + len(tie_cases),
        "tiePolicy": "nearest; exact half-way cases away from zero",
        "maxCssPxRoundTripError": str(max_round_trip_error_px),
        "cssPixelQuantumRepresentableExactly": True,
        "fontUnitsUseExactRationalIntermediate": True,
        "overflowPolicy": "checked signed i64 boundary; no wrapping or saturation",
        "limitations": [
            "These are exact-arithmetic model properties, not compiled Rust Fixed implementation tests.",
            "No real font's full glyph metrics or page corpus is benchmarked here.",
            "CPU cost and i128 intermediate implementation still require Rust-level measurement.",
        ],
    }

def main() -> None:
    candidates = [
        # 1/64 CSS px = 3/256 pt, hence 256/3 ticks per pt.
        describe("1/64 CSS px", Fraction(256, 3)),
        describe("1/1024 pt", Fraction(1024, 1)),
        # 1/65536 pt represents 1/64 CSS px exactly (768 ticks).
        describe("1/65536 pt", Fraction(65536, 1)),
    ]
    result = {
        "experiment": "fixed-point-quantum-rounding-and-conversion-properties",
        "inputs": {
            "a4WidthPt": str(A4_WIDTH_PT),
            "a4HeightPt": str(A4_HEIGHT_PT),
            "lineHeightPt": str(LINE_HEIGHT_PT),
            "repeatedLines": LINE_COUNT,
            "cssPixelsPerPoint": "4/3",
        },
        "candidates": candidates,
        "propertyChecks": run_property_checks(candidates),
        "conclusion": (
            "Scale alone does not guarantee drift-free layout. Independent rounding "
            "of every line accumulates error; retain rational residual/carry or round "
            "only canonical cumulative positions. 1/65536 pt exactly represents "
            "1/64 CSS px and gives a much finer print-native quantum, but a production "
            "choice still requires Rust-level overflow/property tests and real font metrics."
        ),
    }
    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
