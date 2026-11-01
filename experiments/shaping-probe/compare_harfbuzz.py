#!/usr/bin/env python3
"""Compare HarfRust hr-shape output with upstream HarfBuzz hb-shape."""

import argparse
import json
import subprocess
from pathlib import Path


def run(command: list[str]) -> str:
    result = subprocess.run(command, check=True, capture_output=True, text=True)
    return result.stdout


def glyph_line(output: str) -> str:
    for line in output.splitlines():
        if line.strip().startswith("[") and line.strip().endswith("]"):
            return line.strip()
    raise SystemExit("no serialized glyph line found in shaper output: " + repr(output[:1000]))


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--hb-shape", required=True, type=Path)
    parser.add_argument("--font", required=True, type=Path)
    parser.add_argument("--raw", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()

    raw = json.loads(args.raw.read_text(encoding="utf-8"))
    version = run([str(args.hb_shape), "--version"]).strip()
    comparisons = []
    for sample in raw["samples"]:
        if not sample["shaping"]:
            continue
        command = [
            str(args.hb_shape),
            "--font-size", "1000",
            "--direction", "rtl",
            "--script", "Hebr",
            "--language", "he",
            "--no-glyph-names",
            "--show-text",
            "--show-unicode",
            "--show-extents",
            "--show-flags",
            str(args.font),
            sample["text"],
        ]
        upstream_output = run(command)
        upstream_glyphs = glyph_line(upstream_output)
        harfrust_glyphs = glyph_line(sample["shaping"])
        comparisons.append({
            "sampleIndex": sample["index"],
            "text": sample["text"],
            "harfrustGlyphLine": harfrust_glyphs,
            "harfbuzzGlyphLine": upstream_glyphs,
            "exactGlyphLineMatch": harfrust_glyphs == upstream_glyphs,
        })

    report = {
        "schemaVersion": 1,
        "reference": "HarfBuzz 15.0.0 hb-shape",
        "referenceVersion": version,
        "fontSha256": None,
        "comparedSampleCount": len(comparisons),
        "exactMatchCount": sum(item["exactGlyphLineMatch"] for item in comparisons),
        "allExactMatches": bool(comparisons) and all(
            item["exactGlyphLineMatch"] for item in comparisons
        ),
        "comparisons": comparisons,
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(
        f"HarfRust/HarfBuzz glyph-line parity: "
        f"{report['exactMatchCount']}/{report['comparedSampleCount']}"
    )
    if not report["allExactMatches"]:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
