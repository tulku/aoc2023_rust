use std::fs;
use std::path;

use lazy_static::lazy_static;

use clap::Parser;

/// Compute calibration value.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CLI {
    /// Path to obfuscated calibration file.
    path: path::PathBuf,
}

lazy_static! {
    static ref DECIMAL_DIGIT_NAMES: Vec<&'static str> = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"
    ];
}

fn find_decimal_digit(string: &str) -> Option<u32> {
    let mut digit_candidates: Vec<_> = (0..10).map(|digit| {
        match string.find(DECIMAL_DIGIT_NAMES[digit]) {
            Some(index) => Some((index, digit as u32)),
            _ => None
        }
    }).collect();
    digit_candidates.push(
        match string.find(char::is_numeric) {
            Some(index) => {
                let character = string.chars().nth(index).unwrap();
                Some((index, character.to_digit(10).unwrap()))
            },
            _ => None
        }
    );
    digit_candidates.iter().flatten().min().map(|(_, digit)| digit).copied()
}

fn rfind_decimal_digit(string: &str) -> Option<u32> {
    let mut digit_candidates: Vec<_> = (0..10).map(|digit| {
        match string.rfind(DECIMAL_DIGIT_NAMES[digit]) {
            Some(index) => Some((index, digit as u32)),
            _ => None
        }
    }).collect();
    digit_candidates.push(
        match string.rfind(char::is_numeric) {
            Some(index) => {
                let character = string.chars().nth(index).unwrap();
                Some((index, character.to_digit(10).unwrap()))
            },
            _ => None
        }
    );
    digit_candidates.iter().flatten().max().map(|(_, digit)| digit).copied()
}

fn parse_calibration_number(line: &str) -> Result<u32, &str> {
    match find_decimal_digit(line) {
        Some(leading_digit) => {
            let trailing_digit = rfind_decimal_digit(line).unwrap();
            Ok(leading_digit * 10 + trailing_digit)
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calibration_number_parsing() {
        assert_eq!(29, parse_calibration_number("two1nine").unwrap());
        assert_eq!(83, parse_calibration_number("eightwothree").unwrap());
        assert_eq!(13, parse_calibration_number("abcone2threexyz").unwrap());
        assert_eq!(24, parse_calibration_number("xtwone3four").unwrap());
        assert_eq!(42, parse_calibration_number("4nineeightseven2").unwrap());
        assert_eq!(14, parse_calibration_number("zoneight234").unwrap());
        assert_eq!(76, parse_calibration_number("7pqrstsixteen").unwrap());
        assert_eq!(22, parse_calibration_number("2q").unwrap());
        assert_eq!(11, parse_calibration_number("one").unwrap());
        assert_eq!(18, parse_calibration_number("1threeight").unwrap());
    }
}
