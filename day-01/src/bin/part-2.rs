// Your calculation isn't quite right. It looks like some of the digits are
// actually spelled out with letters:
// one, two, three, four, five, six, seven, eight, and nine
// also count as valid "digits".

// Equipped with this new information, you now need to find the real first and last digit on each line.

// For example:

// two1nine
// eightwothree
// abcone2threexyz
// xtwone3four
// 4nineeightseven2
// zoneight234
// 7pqrstsixteen

// In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76.
// Adding these together produces 281.

use regex::Regex;

#[derive(Debug, PartialEq)]
struct CalibrationNumbers {
    first: Option<u32>,
    last: Option<u32>,
}

impl CalibrationNumbers {
    fn num(&self) -> u32 {
        self.first.unwrap_or(0) * 10 + self.last.unwrap_or(0)
    }
}

fn find_digit(line: &str) -> Option<String> {
    let re = Regex::new(r"one|two|three|four|five|six|seven|eight|nine|[0-9]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();
    let matches = re.captures(line)?;
    matches.get(0).map(|matched| matched.as_str().to_string())
}

fn to_number(string_digit: Option<String>) -> Option<u32> {
    let digit = string_digit.as_ref()?;
    match digit.as_str() {
        "one" | "eno" => Some(1),
        "two" | "owt" => Some(2),
        "three" | "eerht" => Some(3),
        "four" | "ruof" => Some(4),
        "five" | "evif" => Some(5),
        "six" | "xis" => Some(6),
        "seven" | "neves" => Some(7),
        "eight" | "thgie" => Some(8),
        "nine" | "enin" => Some(9),
        _ => digit.parse::<u32>().ok(),
    }
}

fn find_calibration(line: &str) -> CalibrationNumbers {
    let first = to_number(find_digit(line));
    let rev_line = line.chars().rev().collect::<String>();
    let last = to_number(find_digit(&rev_line));
    CalibrationNumbers { first, last }
}

fn part_2(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        sum += find_calibration(line).num();
    }
    sum
}

fn main() {
    let input = include_str!("../../input/input-1.txt");
    let result = part_2(input);
    println!("Total calibration: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_digit() {
        let line = "1abc2";
        assert_eq!(find_digit(line), Some("1".to_string()));
    }

    #[test]
    fn test_find_string_digit() {
        let line = "two1nine";
        assert_eq!(find_digit(line), Some("two".to_string()));
    }

    #[test]
    fn test_find_no_string_digit() {
        let line = "abcd";
        assert_eq!(find_digit(line), None);
    }

    #[test]
    fn test_to_number() {
        let string_digit = Some("1".to_string());
        assert_eq!(to_number(string_digit), Some(1));
        let string_digit = Some("one".to_string());
        assert_eq!(to_number(string_digit), Some(1));
        let string_digit = Some("nine".to_string());
        assert_eq!(to_number(string_digit), Some(9));
        let string_digit = Some("hello".to_string());
        assert_eq!(to_number(string_digit), None);
    }

    #[test]
    fn test_find_calibration_to_num() {
        let line = "eightwothree";
        let calibration = find_calibration(line).num();
        assert_eq!(calibration, 83);
    }

    #[test]
    fn test_calibration_number_to_num() {
        let calibration = CalibrationNumbers {
            first: Some(1),
            last: Some(2),
        };
        assert_eq!(calibration.num(), 12);
        let calibration = CalibrationNumbers {
            first: Some(1),
            last: None,
        };
        assert_eq!(calibration.num(), 10);
        let calibration = CalibrationNumbers {
            first: None,
            last: Some(2),
        };
        assert_eq!(calibration.num(), 2);
        let calibration = CalibrationNumbers {
            first: None,
            last: None,
        };
        assert_eq!(calibration.num(), 0);
    }

    #[test]
    fn test_find_calibration() {
        let line = "two1nine";
        let calibration = find_calibration(line);
        assert_eq!(
            calibration,
            CalibrationNumbers {
                first: Some(2),
                last: Some(9)
            }
        );

        let line = "eightwothree";
        let calibration = find_calibration(line);
        assert_eq!(
            calibration,
            CalibrationNumbers {
                first: Some(8),
                last: Some(3)
            }
        );

        let line = "abcone2threexyz";
        let calibration = find_calibration(line);
        assert_eq!(
            calibration,
            CalibrationNumbers {
                first: Some(1),
                last: Some(3)
            }
        );

        let line = "xtwone3four";
        let calibration = find_calibration(line);
        assert_eq!(
            calibration,
            CalibrationNumbers {
                first: Some(2),
                last: Some(4)
            }
        );

        let line = "4nineeightseven2";
        let calibration = find_calibration(line);
        assert_eq!(
            calibration,
            CalibrationNumbers {
                first: Some(4),
                last: Some(2)
            }
        );

        let line = "zoneight234";
        let calibration = find_calibration(line);
        assert_eq!(
            calibration,
            CalibrationNumbers {
                first: Some(1),
                last: Some(4)
            }
        );

        let line = "7pqrstsixteen";
        let calibration = find_calibration(line);
        assert_eq!(
            calibration,
            CalibrationNumbers {
                first: Some(7),
                last: Some(6)
            }
        );

        let line = "abcd";
        let calibration = find_calibration(line);
        assert_eq!(
            calibration,
            CalibrationNumbers {
                first: None,
                last: None
            }
        );

        let line = "oneandonly";
        let calibration = find_calibration(line);
        assert_eq!(
            calibration,
            CalibrationNumbers {
                first: Some(1),
                last: Some(1)
            }
        );

        let line = "onlyatwo";
        let calibration = find_calibration(line);
        assert_eq!(
            calibration,
            CalibrationNumbers {
                first: Some(2),
                last: Some(2)
            }
        );
    }
}
