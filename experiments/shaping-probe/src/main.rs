use std::env;
use std::fs;
use std::process::ExitCode;
use unicode_bidi::BidiInfo;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("shaping-probe: {error}");
            ExitCode::from(2)
        }
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        return Err("usage: shaping-probe FONT.ttf CORPUS.txt OUTPUT.json".to_string());
    }
    let font_path = &args[1];
    let corpus_path = &args[2];
    let output_path = &args[3];
    if !std::path::Path::new(font_path).is_file() {
        return Err(format!("font file does not exist: {font_path}"));
    }

    let corpus = fs::read_to_string(corpus_path)
        .map_err(|error| format!("cannot read corpus {corpus_path}: {error}"))?;
    let mut output = String::from("{\n  \"schemaVersion\": 1,\n  \"shaper\": \"HarfRust hr-shape 0.15.0\",\n  \"bidi\": \"unicode-bidi 0.3.18\",\n  \"samples\": [\n");
    let mut sample_count = 0usize;

    for (index, sample) in corpus.lines().enumerate() {
        if sample.trim().is_empty() {
            continue;
        }
        if sample_count > 0 {
            output.push_str(",\n");
        }
        let bidi = BidiInfo::new(sample, None);
        let paragraph = bidi
            .paragraphs
            .first()
            .ok_or_else(|| format!("sample {} has no bidi paragraph", index + 1))?;
        let visual = bidi.reorder_line(paragraph, paragraph.range.clone());
        let mut levels = String::from("[");
        for (level_index, (byte_index, character)) in sample.char_indices().enumerate() {
            if level_index > 0 {
                levels.push_str(", ");
            }
            let level = bidi
                .levels
                .get(byte_index)
                .ok_or_else(|| format!("missing bidi level at byte {byte_index}"))?;
            levels.push_str(&format!(
                "{{\"byte\":{byte_index},\"codepoint\":\"U+{:04X}\",\"level\":{}}}",
                character as u32,
                level.number()
            ));
        }
        levels.push(']');

        let shaping = if sample
            .chars()
            .any(|character| character.is_ascii_alphabetic())
        {
            None
        } else {
            let options = "--direction rtl --script Hebr --language he --verify --show-text --show-unicode --show-extents --show-flags --no-glyph-names";
            Some(
                hr_shape::shape(font_path, sample, options).map_err(|error| {
                    format!("HarfRust failed for sample {}: {error}", index + 1)
                })?,
            )
        };

        output.push_str("    {\"index\":");
        output.push_str(&(index + 1).to_string());
        output.push_str(",\"text\":");
        output.push_str(&json_string(sample));
        output.push_str(",\"paragraphLevel\":");
        output.push_str(&paragraph.level.number().to_string());
        output.push_str(",\"bidiLevels\":");
        output.push_str(&levels);
        output.push_str(",\"visualText\":");
        output.push_str(&json_string(&visual));
        output.push_str(",\"shaping\":");
        match shaping {
            Some(result) => output.push_str(&json_string(&result)),
            None => output.push_str("null"),
        }
        output.push('}');
        sample_count += 1;
    }

    if sample_count != 8 {
        return Err(format!("expected 8 corpus samples, got {sample_count}"));
    }
    output.push_str("\n  ]\n}\n");
    fs::write(output_path, output)
        .map_err(|error| format!("cannot write {output_path}: {error}"))?;
    println!("Shaping/Bidi probe complete: {sample_count} samples -> {output_path}");
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
