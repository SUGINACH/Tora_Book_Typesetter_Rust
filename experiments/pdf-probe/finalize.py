#!/usr/bin/env python3
"""Verify Type0 embedding and exact logical text extraction from the PDF probe."""

import argparse
import hashlib
import json
import platform
import re
import subprocess
from pathlib import Path


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def run(command: list[str]) -> str:
    result = subprocess.run(command, check=True, capture_output=True, text=True)
    return result.stdout


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--pdf", required=True, type=Path)
    parser.add_argument("--report", required=True, type=Path)
    parser.add_argument("--font", required=True, type=Path)
    parser.add_argument("--license", required=True, type=Path)
    parser.add_argument("--corpus", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()

    raw_report = json.loads(args.report.read_text(encoding="utf-8"))
    expected_lines = [
        line for line in args.corpus.read_text(encoding="utf-8").splitlines() if line.strip()
    ]
    pdf_bytes = args.pdf.read_bytes()
    if not re.search(rb"/Subtype\s*/Type0", pdf_bytes) or not re.search(rb"/ToUnicode\b", pdf_bytes):
        raise SystemExit("PDF does not contain a Type0 font and ToUnicode CMap")
    if not raw_report.get("glyphToUnicodeMappings"):
        raise SystemExit("no glyph-to-Unicode mappings were written")
    if raw_report.get("fontParseWarnings") != "[]":
        raise SystemExit(f"printpdf font parse warnings: {raw_report.get('fontParseWarnings')}")
    if raw_report.get("saveWarnings") != "[]":
        raise SystemExit(f"printpdf save warnings: {raw_report.get('saveWarnings')}")

    extracted_path = args.output.parent / "pdf-probe-extracted.txt"
    layout_extracted_path = args.output.parent / "pdf-probe-layout-extracted.txt"
    mupdf_extracted_path = args.output.parent / "pdf-probe-mupdf-extracted.txt"
    fonts_path = args.output.parent / "pdf-probe-fonts.txt"
    info_path = args.output.parent / "pdf-probe-info.txt"
    subprocess.run(["pdftotext", "-raw", str(args.pdf), str(extracted_path)], check=True)
    subprocess.run(["pdftotext", "-layout", str(args.pdf), str(layout_extracted_path)], check=True)
    extracted = extracted_path.read_text(encoding="utf-8")
    layout_extracted = layout_extracted_path.read_text(encoding="utf-8")
    normalized = [line.strip() for line in extracted.replace("\f", "").splitlines() if line.strip()]
    layout_normalized = [
        line.strip() for line in layout_extracted.replace("\f", "").splitlines() if line.strip()
    ]
    # Keep Poppler's raw/layout results as diagnostics. Its RTL bidi
    # post-processing can emit visual-order codepoints wrapped in RLE/PDF
    # controls even when /ActualText contains the original logical Unicode.
    # MuPDF is the independent acceptance extractor for this focused probe.
    subprocess.run(
        ["mutool", "draw", "-F", "txt", "-o", str(mupdf_extracted_path), str(args.pdf)],
        check=True,
        capture_output=True,
        text=True,
    )
    mupdf_extracted = mupdf_extracted_path.read_text(encoding="utf-8")
    mupdf_normalized = [
        line.strip() for line in mupdf_extracted.replace("\\f", "").splitlines() if line.strip()
    ]
    if mupdf_normalized != expected_lines:
        raise SystemExit(
            "MuPDF logical text extraction mismatch.\\nExpected: "
            + repr(expected_lines)
            + "\\nActual: "
            + repr(mupdf_normalized)
        )

    fonts = run(["pdffonts", str(args.pdf)])
    fonts_path.write_text(fonts, encoding="utf-8")
    info = run(["pdfinfo", str(args.pdf)])
    info_path.write_text(info, encoding="utf-8")
    font_rows = [line for line in fonts.splitlines() if "CID TrueType" in line]
    if not any("Identity-H" in line and "yes yes yes" in line for line in font_rows):
        raise SystemExit("pdffonts did not report an embedded/subset Unicode CID TrueType font")
    page_count = next(
        (line.split(":", 1)[1].strip() for line in info.splitlines() if line.startswith("Pages:")),
        None,
    )
    if page_count != "1":
        raise SystemExit(f"PDF page count is not one: {page_count!r}")

    rustc = subprocess.run(
        ["rustc", "--version"], check=True, capture_output=True, text=True
    ).stdout.strip()
    manifest = {
        "schemaVersion": 1,
        "experiment": "HarfRust glyphs to printpdf Type0/ToUnicode",
        "sourcePins": {
            "harfrustCommit": "71d965130ea93905f81b8f03a03d7ecc794fbc47",
            "harfrustVersion": "0.15.0",
            "printpdfVersion": "0.12.8",
            "fontRepository": "https://github.com/google/fonts",
            "fontCommit": "6c1dbe27dcf7d817d4b62cec033ba77fa8e2a5ec",
            "fontPath": "ofl/notoserifhebrew/NotoSerifHebrew[wdth,wght].ttf",
            "fontSha256": sha256(args.font),
            "licenseSha256": sha256(args.license),
            "corpusSha256": sha256(args.corpus),
        },
        "environment": {
            "rustc": rustc,
            "platform": platform.platform(),
            "architecture": platform.machine(),
        },
        "results": {
            "pdfSha256": sha256(args.pdf),
            "popplerRawExtractedTextSha256": sha256(extracted_path),
            "popplerLayoutExtractedTextSha256": sha256(layout_extracted_path),
            "mupdfExtractedTextSha256": sha256(mupdf_extracted_path),
            "glyphToUnicodeMappingCount": raw_report.get("glyphToUnicodeMappings"),
            "logicalTextExtractionExact": True,
            "mupdfLogicalTextExtractionExact": True,
            "popplerRawExtractionMatchesLogical": normalized == expected_lines,
            "popplerLayoutExtractionMatchesLogical": layout_normalized == expected_lines,
            "popplerRawExtractedText": normalized,
            "popplerLayoutExtractedText": layout_normalized,
            "mupdfExtractedText": mupdf_normalized,
            "type0FontPresent": True,
            "toUnicodeCMapPresent": True,
            "pdffonts": fonts,
            "pdfinfo": info,
            "limitations": [
                "Only a focused Hebrew/Latin sample is tested; full document text layout is not part of this PDF writer probe.",
                "PDF viewer copy/paste and selection geometry still require Windows/Acrobat/PDFium checks.",
                "The font's OFL terms are pinned but project-level redistribution review remains separate.",
            ],
        },
        "rawReport": raw_report,
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(
        json.dumps(manifest, ensure_ascii=False, indent=2) + "\n",
        encoding="utf-8",
    )
    print(f"PDF Type0/ToUnicode manifest: {args.output}")


if __name__ == "__main__":
    main()
