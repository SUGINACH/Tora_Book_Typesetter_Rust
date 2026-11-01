use printpdf::text::Codepoint;
use printpdf::{
    Mm, Op, ParsedFont, PdfDocument, PdfFontHandle, PdfPage, PdfSaveOptions, Pt, TextItem,
    TextMatrix,
};
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::process::ExitCode;

#[derive(Debug, Clone)]
struct Glyph {
    original_index: usize,
    gid: u16,
    cluster: usize,
    x_advance: i32,
    y_advance: i32,
    x_offset: i32,
    y_offset: i32,
    x_pt: f32,
    y_pt: f32,
    cid: String,
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("pdf-probe: {error}");
            ExitCode::from(2)
        }
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        return Err("usage: pdf-probe FONT.ttf CORPUS.txt OUTPUT.pdf REPORT.json".to_string());
    }
    let font_path = &args[1];
    let corpus_path = &args[2];
    let pdf_path = &args[3];
    let report_path = &args[4];
    let font_bytes =
        fs::read(font_path).map_err(|error| format!("cannot read font {font_path}: {error}"))?;
    let corpus = fs::read_to_string(corpus_path)
        .map_err(|error| format!("cannot read corpus {corpus_path}: {error}"))?;
    let lines: Vec<&str> = corpus
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();
    if lines.len() != 3 {
        return Err(format!("expected 3 corpus lines, got {}", lines.len()));
    }

    let mut font_parse_warnings = Vec::new();
    let parsed_font = ParsedFont::from_bytes(&font_bytes, 0, &mut font_parse_warnings)
        .ok_or_else(|| format!("printpdf could not parse the font: {font_parse_warnings:?}"))?;
    let mut document = PdfDocument::new("Hebrew Type0 ToUnicode probe");
    let font_id = document.add_font(&parsed_font);
    let mut ops = vec![
        Op::StartTextSection,
        Op::SetFont {
            font: PdfFontHandle::External(font_id),
            size: Pt(18.0),
        },
    ];
    let mut report = String::from(
        "{\n  \"schemaVersion\": 1,\n  \"shaper\": \"HarfRust hr-shape 0.15.0\",\n  \"pdfWriter\": \"printpdf 0.12.8\",\n  \"fontSizePt\": 18,\n  \"lines\": [\n",
    );
    let mut mapping = BTreeSet::<(u16, String)>::new();

    for (line_index, text) in lines.iter().enumerate() {
        if line_index > 0 {
            report.push_str(",\n");
        }
        let (direction, script, language, baseline) =
            if text.chars().any(|c| c.is_ascii_alphabetic()) {
                ("ltr", "Latn", "en", 105.0_f32)
            } else {
                ("rtl", "Hebr", "he", 205.0_f32 - (line_index as f32 * 45.0))
            };
        let mut glyphs = shape_line(font_path, text, direction, script, language, baseline)?;
        assign_cluster_text(text, &mut glyphs)?;
        for glyph in &glyphs {
            mapping.insert((glyph.gid, glyph.cid.clone()));
        }

        // Glyphs are positioned in HarfRust visual order, but emitted to the PDF
        // content stream in logical cluster order for text extraction.
        glyphs.sort_by_key(|glyph| (glyph.cluster, glyph.original_index));
        for glyph in &glyphs {
            ops.push(Op::SetTextMatrix {
                matrix: TextMatrix::Raw([1.0, 0.0, 0.0, 1.0, glyph.x_pt, glyph.y_pt]),
            });
            ops.push(Op::ShowText {
                items: vec![TextItem::GlyphIds(vec![Codepoint {
                    gid: glyph.gid,
                    offset: 0.0,
                    cid: Some(glyph.cid.clone()),
                }])],
            });
        }

        report.push_str(&format!(
            "    {{\"text\":{},\"direction\":{},\"script\":{},\"glyphs\":[",
            json_string(text),
            json_string(direction),
            json_string(script)
        ));
        for (glyph_index, glyph) in glyphs.iter().enumerate() {
            if glyph_index > 0 {
                report.push(',');
            }
            report.push_str(&format!(
                "{{\"gid\":{},\"cluster\":{},\"xAdvance\":{},\"yAdvance\":{},\"xOffset\":{},\"yOffset\":{},\"xPt\":{:.4},\"yPt\":{:.4},\"cid\":{}}}",
                glyph.gid,
                glyph.cluster,
                glyph.x_advance,
                glyph.y_advance,
                glyph.x_offset,
                glyph.y_offset,
                glyph.x_pt,
                glyph.y_pt,
                json_string(&glyph.cid)
            ));
        }
        report.push_str("]}");
    }

    ops.push(Op::EndTextSection);
    let page = PdfPage::new(Mm(210.0), Mm(297.0), ops);
    let options = PdfSaveOptions {
        subset_fonts: true,
        optimize: false,
        ..Default::default()
    };
    let mut save_warnings = Vec::new();
    let pdf = document
        .with_pages(vec![page])
        .save(&options, &mut save_warnings);
    fs::write(pdf_path, &pdf).map_err(|error| format!("cannot write PDF {pdf_path}: {error}"))?;
    report.push_str("\n  ],\n");
    report.push_str(&format!(
        "  \"glyphToUnicodeMappings\": {},\n",
        mapping.len()
    ));
    report.push_str(&format!(
        "  \"fontParseWarnings\": {},\n",
        json_string(&format!("{font_parse_warnings:?}"))
    ));
    report.push_str(&format!(
        "  \"saveWarnings\": {}\n}}\n",
        json_string(&format!("{save_warnings:?}"))
    ));
    fs::write(report_path, report)
        .map_err(|error| format!("cannot write report {report_path}: {error}"))?;
    println!(
        "PDF probe generated: {} glyph mappings, {} bytes -> {}",
        mapping.len(),
        pdf.len(),
        pdf_path
    );
    Ok(())
}

fn shape_line(
    font_path: &str,
    text: &str,
    direction: &str,
    script: &str,
    language: &str,
    baseline: f32,
) -> Result<Vec<Glyph>, String> {
    let options = format!(
        "--direction {direction} --script {script} --language {language} --font-size 1000 --verify --show-text --show-unicode --show-extents --show-flags --no-glyph-names"
    );
    let output = hr_shape::shape(font_path, text, &options)
        .map_err(|error| format!("HarfRust failed to shape {text:?}: {error}"))?;
    let glyph_line = output
        .lines()
        .find(|line| line.starts_with('['))
        .ok_or_else(|| format!("HarfRust output has no glyph line for {text:?}: {output}"))?;
    let mut glyphs = Vec::new();
    let body = glyph_line
        .strip_prefix('[')
        .and_then(|line| line.strip_suffix(']'))
        .ok_or_else(|| format!("malformed HarfRust glyph line: {glyph_line}"))?;
    let mut pen_x = 0_i64;
    let mut pen_y = 0_i64;
    let mut raw_positions = Vec::new();

    for (original_index, token) in body.split('|').enumerate() {
        let (gid, cluster, x_offset, y_offset, x_advance, y_advance) = parse_glyph_token(token)?;
        let x_units = pen_x + i64::from(x_offset);
        let y_units = pen_y + i64::from(y_offset);
        raw_positions.push((x_units, y_units));
        pen_x += i64::from(x_advance);
        pen_y += i64::from(y_advance);
        glyphs.push(Glyph {
            original_index,
            gid,
            cluster,
            x_advance,
            y_advance,
            x_offset,
            y_offset,
            x_pt: 0.0,
            y_pt: 0.0,
            cid: String::new(),
        });
    }

    let min_x = raw_positions.iter().map(|(x, _)| *x).min().unwrap_or(0);
    let max_x = raw_positions.iter().map(|(x, _)| *x).max().unwrap_or(0);
    let center_translation = 105.0_f32 - ((min_x + max_x) as f32 * 18.0 / 2000.0);
    for (glyph, (x_units, y_units)) in glyphs.iter_mut().zip(raw_positions) {
        glyph.x_pt = center_translation + x_units as f32 * 18.0 / 1000.0;
        glyph.y_pt = baseline + y_units as f32 * 18.0 / 1000.0;
    }
    Ok(glyphs)
}

fn parse_glyph_token(token: &str) -> Result<(u16, usize, i32, i32, i32, i32), String> {
    let token = token.split('<').next().unwrap_or(token);
    let token = token.split('#').next().unwrap_or(token);
    let (head, advance) = token
        .split_once('+')
        .ok_or_else(|| format!("missing advance in HarfRust token {token:?}"))?;
    let (identity, offset) = head.split_once('@').unwrap_or((head, "0,0"));
    let (gid, cluster) = identity
        .split_once('=')
        .ok_or_else(|| format!("missing cluster in HarfRust token {token:?}"))?;
    let gid = gid
        .parse::<u16>()
        .map_err(|error| format!("invalid glyph id {gid:?}: {error}"))?;
    let cluster = cluster
        .parse::<usize>()
        .map_err(|error| format!("invalid cluster {cluster:?}: {error}"))?;
    let (x_offset, y_offset) = parse_pair(offset)?;
    let (x_advance, y_advance) = parse_pair(advance)?;
    Ok((gid, cluster, x_offset, y_offset, x_advance, y_advance))
}

fn parse_pair(value: &str) -> Result<(i32, i32), String> {
    let mut parts = value.split(',');
    let first = parts
        .next()
        .unwrap_or("0")
        .parse::<i32>()
        .map_err(|error| format!("invalid numeric component {value:?}: {error}"))?;
    let second = parts
        .next()
        .unwrap_or("0")
        .parse::<i32>()
        .map_err(|error| format!("invalid numeric component {value:?}: {error}"))?;
    if parts.next().is_some() {
        return Err(format!("too many numeric components in {value:?}"));
    }
    Ok((first, second))
}

fn assign_cluster_text(text: &str, glyphs: &mut [Glyph]) -> Result<(), String> {
    let characters: Vec<char> = text.chars().collect();
    let mut clusters = BTreeSet::new();
    for glyph in glyphs.iter() {
        clusters.insert(glyph.cluster);
    }
    let ordered: Vec<usize> = clusters.into_iter().collect();
    for (index, cluster) in ordered.iter().enumerate() {
        if *cluster >= characters.len() {
            return Err(format!(
                "cluster {cluster} exceeds source character count {}",
                characters.len()
            ));
        }
        let end = ordered.get(index + 1).copied().unwrap_or(characters.len());
        let source: String = characters[*cluster..end].iter().collect();
        let indices: Vec<usize> = glyphs
            .iter()
            .enumerate()
            .filter_map(|(glyph_index, glyph)| (glyph.cluster == *cluster).then_some(glyph_index))
            .collect();
        if indices.len() == source.chars().count() {
            for (glyph_index, character) in indices.iter().zip(source.chars()) {
                glyphs[*glyph_index].cid = character.to_string();
            }
        } else if let Some(first) = indices.first() {
            glyphs[*first].cid = source;
            for glyph_index in indices.iter().skip(1) {
                glyphs[*glyph_index].cid.clear();
            }
        }
    }
    Ok(())
}

fn json_string(value: &str) -> String {
    let mut output = String::from("\"");
    for character in value.chars() {
        match character {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            character if character.is_control() => {
                output.push_str(&format!("\\u{:04x}", character as u32));
            }
            character => output.push(character),
        }
    }
    output.push('"');
    output
}
