//! SVG conversion tool

use palette::oklch::Oklch;
use palette::rgb::Srgb;
use palette::FromColor;
use regex::Regex;
use std::{env, fs, process};

fn convert_oklch_to_hex(svg_content: String) -> String {
    let oklch_regex = Regex::new(r"oklch\(([^)]+)\)").unwrap();
    oklch_regex
        .replace_all(&svg_content, |caps: &regex::Captures| {
            let oklch_str = caps.get(1).unwrap().as_str();
            let parts: Vec<&str> = oklch_str.split_whitespace().collect();
            if parts.len() == 3 {
                let l: f32 = if let Some(val) = parts[0].strip_suffix("%") {
                    val.parse().unwrap_or(0.0) / 100.0
                } else {
                    parts[0].parse().unwrap_or(0.0)
                };
                let c: f32 = parts[1].parse().unwrap_or(0.0);
                let h: f32 = parts[2].parse().unwrap_or(0.0);
                let oklch = Oklch::new(l, c, h);
                let rgb: Srgb = Srgb::from_color(oklch);
                format!(
                    "#{:02x}{:02x}{:02x}",
                    (rgb.red * 255.0) as u8,
                    (rgb.green * 255.0) as u8,
                    (rgb.blue * 255.0) as u8
                )
            } else {
                caps.get(0).unwrap().as_str().to_string()
            }
        })
        .to_string()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input.svg> <output.svg>", args[0]);
        process::exit(1);
    }
    let svg_content = fs::read_to_string(&args[1]).expect("Failed to read input file");
    let converted_svg = convert_oklch_to_hex(svg_content);
    fs::write(&args[2], converted_svg).expect("Failed to write output file");
    println!("SVG colors converted successfully!");
}
