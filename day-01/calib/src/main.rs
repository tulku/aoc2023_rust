use std::fs;
use std::char;
use std::path;

use clap::Parser;

/// Compute calibration value.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CLI {
    /// Path to obfuscated calibration file.
    path: path::PathBuf,
}

fn parse_calibration_number(line: &str) -> Result<u32, &str> {
    match line.find(char::is_numeric) {
        Some(leading_index) => {
            let trailing_index = line.rfind(char::is_numeric).unwrap();
            let leading_char = line.chars().nth(leading_index).unwrap();
            let trailing_char = line.chars().nth(trailing_index).unwrap();
            let leading_number = leading_char.to_digit(10).unwrap();
            let trailing_number = trailing_char.to_digit(10).unwrap();
            Ok(leading_number * 10 + trailing_number)
        },
        None => Err("missing number")
    }
}

fn main() {
    let args = CLI::parse();

    let content = fs::read_to_string(args.path)
        .expect("Failed to read calibration file!");

    let sum = content.lines().enumerate().map(
        |(index, line)| parse_calibration_number(line).unwrap_or_else(
            |err| panic!("Failed to parse line {}: {}", index + 1, err)
        )).sum::<u32>();

    println!("{}", sum);
}
