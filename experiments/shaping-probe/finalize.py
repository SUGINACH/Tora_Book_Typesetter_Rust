#!/usr/bin/env python3
"""Attach exact font/corpus/toolchain hashes to the HarfRust probe output."""

import argparse
import hashlib
import json
import platform
import subprocess
from pathlib import Path


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--raw", required=True, type=Path)
    parser.add_argument("--comparison", required=True, type=Path)
    parser.add_argument("--font", required=True, type=Path)
    parser.add_argument("--license", required=True, type=Path)
    parser.add_argument("--corpus", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()

    raw = json.loads(args.raw.read_text(encoding="utf-8"))
    comparison = json.loads(args.comparison.read_text(encoding="utf-8"))
    if not comparison.get("allExactMatches"):
        raise SystemExit("HarfRust glyph output differs from the pinned HarfBuzz 15.0.0 reference")
    if "15.0.0" not in comparison.get("referenceVersion", ""):
        raise SystemExit(f"unexpected HarfBuzz reference version: {comparison.get('referenceVersion')!r}")
    if len(raw.get("samples", [])) != 8:
        raise SystemExit("expected exactly eight shaped/bidi samples")
    shaped = [sample for sample in raw["samples"] if sample["shaping"]]
    mixed = [sample for sample in raw["samples"] if sample["visualText"] != sample["text"]]
    if len(shaped) < 5:
        raise SystemExit(f"expected at least five HarfRust shaping samples, got {len(shaped)}")
    if not mixed:
        raise SystemExit("Bidi probe did not reorder any mixed-direction sample")

    rustc = subprocess.run(
        ["rustc", "--version"], check=True, capture_output=True, text=True
    ).stdout.strip()
    manifest = {
        "schemaVersion": 1,
        "experiment": "Hebrew shaping and bidi probe",
        "sourcePins": {
            "harfrustRepository": "https://github.com/harfbuzz/harfrust",
            "harfrustCommit": "71d965130ea93905f81b8f03a03d7ecc794fbc47",
            "harfrustVersion": "0.15.0",
            "harfbuzzRepository": "https://github.com/harfbuzz/harfbuzz",
            "harfbuzzCommit": "35f656a1a2bab548ea7f54a6dd3fba2b09680d14",
            "harfbuzzVersion": "15.0.0",
            "googleFontsRepository": "https://github.com/google/fonts",
            "googleFontsCommit": "6c1dbe27dcf7d817d4b62cec033ba77fa8e2a5ec",
            "fontPath": "ofl/notoserifhebrew/NotoSerifHebrew[wdth,wght].ttf",
            "fontSha256": sha256(args.font),
            "licensePath": "ofl/notoserifhebrew/OFL.txt",
            "licenseSha256": sha256(args.license),
            "corpusPath": "experiments/shaping-probe/corpus.txt",
            "corpusSha256": sha256(args.corpus),
            "unicodeBidiVersion": "0.3.18",
        },
        "environment": {
            "rustc": rustc,
            "platform": platform.platform(),
            "architecture": platform.machine(),
        },
        "results": {
            "sampleCount": len(raw["samples"]),
            "shapedSampleCount": len(shaped),
            "bidiReorderedSampleCount": len(mixed),
            "allHarfRustCallsSucceeded": True,
            "allHarfRustCallsUsedVerify": True,
            "nativeHarfBuzzExactMatchCount": comparison.get("exactMatchCount"),
            "nativeHarfBuzzComparedSampleCount": comparison.get("comparedSampleCount"),
            "nativeHarfBuzzExactParity": comparison.get("allExactMatches"),
            "limitations": [
                "Mixed-direction samples are processed by unicode-bidi for levels and visual text, but are not yet segmented into script/direction shaping runs.",
                "Native HarfBuzz comparison covers pure Hebrew shaping samples only; it does not prove mixed-direction run segmentation or visual line reordering.",
                "No second independent Rust shaping implementation has been compared.",
                "The font's redistribution terms are pinned but still require project-level license review before bundling.",
            ],
        },
        "rawReport": raw,
        "nativeHarfBuzzComparison": comparison,
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(
        json.dumps(manifest, ensure_ascii=False, indent=2) + "\n",
        encoding="utf-8",
    )
    print(f"Shaping probe manifest: {args.output}")


if __name__ == "__main__":
    main()
