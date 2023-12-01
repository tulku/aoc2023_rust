// As they're making the final adjustments, they discover that their calibration document (your puzzle input)
// has been amended by a very young Elf who was apparently just excited to show off her art skills.
// Consequently, the Elves are having trouble reading the values on the document.

// The newly-improved calibration document consists of lines of text; each line originally contained
// a specific calibration value that the Elves now need to recover. On each line, the calibration value
// can be found by combining the first digit and the last digit (in that order) to form a single
// two-digit number.

// For example:
// ```
// 1abc2
// pqr3stu8vwx
// a1b2c3d4e5f
// treb7uchet
// ```
// In this example, the calibration values of these four lines are:
// 12, 38, 15, and 77. Adding these together produces 142.

// Consider your entire calibration document. What is the sum of all of the calibration values?

fn find_digit(line_iter: impl Iterator<Item = char>) -> Option<u32> {
    for char in line_iter {
        match char.to_digit(10) {
            Some(number) => return Some(number),
            None => continue,
        };
    }
    None
}

fn find_calibration(line: &str) -> Option<u32> {
    let line_iter = line.chars();
    let first_digit = find_digit(line_iter.clone())?;
    let last_digit = find_digit(line_iter.rev())?;
    Some(first_digit * 10 + last_digit)
}

fn part_1(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        sum += find_calibration(line).unwrap_or(0);
    }
    sum
}

fn main() {
    let input = include_str!("../../input/input-1.txt");
    let result = part_1(input);
    println!("Total calibration: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_digit() {
        let line = "1abc2".chars();
        assert_eq!(find_digit(line), Some(1));
    }

    #[test]
    fn test_find_no_digit() {
        let line = "abc".chars();
        assert_eq!(find_digit(line), None);
    }

    #[test]
    fn test_find_last_digit() {
        let line = "a1b2c".chars().rev();
        assert_eq!(find_digit(line), Some(2));
    }

    #[test]
    fn test_find_same_first_and_last_digit() {
        let line = "a1b1c".chars();
        assert_eq!(find_digit(line), Some(1));
        let rev_line = "a1b1c".chars().rev();
        assert_eq!(find_digit(rev_line), Some(1));
    }

    #[test]
    fn test_find_calibration() {
        let line = "1abc2";
        assert_eq!(find_calibration(line), Some(12));
        let line = "pqr3stu8vwx";
        assert_eq!(find_calibration(line), Some(38));
        let line = "a1b2c3d4e5f";
        assert_eq!(find_calibration(line), Some(15));
        let line = "treb7uchet";
        assert_eq!(find_calibration(line), Some(77));
    }
}
