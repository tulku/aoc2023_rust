// --- Part Two ---
// Your calculation isn't quite right. It looks like some of the digits are actually spelled out with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".

// Equipped with this new information, you now need to find the real first and last digit on each line. For example:

// two1nine
// eightwothree
// abcone2threexyz
// xtwone3four
// 4nineeightseven2
// zoneight234
// 7pqrstsixteen
// In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together produces 281.

// What is the sum of all of the calibration values?

fn find_calibration(line: &str) -> Option<u32> {
    let mut first_value: Option<u32> = None;
    let mut second_value: Option<u32> = None;

    let number_strings = vec![
        "zero".to_string(),
        "one".to_string(),
        "two".to_string(),
        "three".to_string(),
        "four".to_string(),
        "five".to_string(),
        "six".to_string(),
        "seven".to_string(),
        "eight".to_string(),
        "nine".to_string(),
    ];

    for (i, character) in line.chars().enumerate() {
        if first_value.is_none() {
            match character {
                '0'..='9' => first_value = character.to_digit(10),
                _ => first_value = None,
            }
            let string_from_i = &line[i..];
            for index in 0..number_strings.len() {
                if string_from_i.starts_with(&number_strings[index]) {
                    first_value = Some(index.try_into().unwrap());
                    continue;
                }
            }
        } else {
            break;
        }
    }
    first_value?;
    for (i, character) in line.chars().rev().enumerate() {
        if second_value.is_none() {
            match character {
                '0'..='9' => second_value = character.to_digit(10),
                _ => second_value = None,
            }
            let string_from_i = &line[(line.len() - i)..];
            for index in 0..number_strings.len() {
                if string_from_i.starts_with(&number_strings[index]) {
                    second_value = Some(index.try_into().unwrap());
                    continue;
                }
            }
        } else {
            break;
        }
    }

    Some(first_value? * 10 + second_value?)
}

fn part_2(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        sum += find_calibration(line).unwrap_or(0);
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
    fn test_find_calibration() {
        let line = "two1nine";
        assert_eq!(find_calibration(line), Some(29));
        let line = "eightwothree";
        assert_eq!(find_calibration(line), Some(83));
        let line = "abcone2threexyz";
        assert_eq!(find_calibration(line), Some(13));
        let line = "xtwone3four";
        assert_eq!(find_calibration(line), Some(24));
        let line = "4nineeightseven2";
        assert_eq!(find_calibration(line), Some(42));
        let line = "zoneight234";
        assert_eq!(find_calibration(line), Some(14));
        let line = "7pqrstsixteen";
        assert_eq!(find_calibration(line), Some(76));
    }
}
