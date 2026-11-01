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


def main() -> None:
    candidates = [
        # 1/64 CSS px = 3/256 pt, hence 256/3 ticks per pt.
        describe("1/64 CSS px", Fraction(256, 3)),
        describe("1/1024 pt", Fraction(1024, 1)),
        # 1/65536 pt represents 1/64 CSS px exactly (768 ticks).
        describe("1/65536 pt", Fraction(65536, 1)),
    ]
    result = {
        "experiment": "fixed-point-quantum-and-rounding-drift",
        "inputs": {
            "a4WidthPt": str(A4_WIDTH_PT),
            "a4HeightPt": str(A4_HEIGHT_PT),
            "lineHeightPt": str(LINE_HEIGHT_PT),
            "repeatedLines": LINE_COUNT,
            "cssPixelsPerPoint": "4/3",
        },
        "candidates": candidates,
        "conclusion": (
            "Scale alone does not guarantee drift-free layout. Independent rounding "
            "of every line accumulates error; retain rational residual/carry or round "
            "only canonical cumulative positions. 1/65536 pt exactly represents "
            "1/64 CSS px and gives a much finer print-native quantum, but a production "
            "choice still requires overflow/property tests and real font metrics."
        ),
    }
    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
