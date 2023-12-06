// --- Day 3: Gear Ratios ---
// You and the Elf eventually reach a gondola lift station; he says the gondola lift will take you up to the water source, but this is as far as he can bring you. You go inside.

// It doesn't take long to find the gondolas, but there seems to be a problem: they're not moving.

// "Aaah!"

// You turn around to see a slightly-greasy Elf with a wrench and a look of surprise. "Sorry, I wasn't expecting anyone! The gondola lift isn't working right now; it'll still be a while before I can fix it." You offer to help.

// The engineer explains that an engine part seems to be missing from the engine, but nobody can figure out which one. If you can add up all the part numbers in the engine schematic, it should be easy to work out which part is missing.

// The engine schematic (your puzzle input) consists of a visual representation of the engine. There are lots of numbers and symbols you don't really understand, but apparently any number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum. (Periods (.) do not count as a symbol.)

// Here is an example engine schematic:

// 467..114..
// ...*......
// ..35..633.
// ......#...
// 617*......
// .....+.58.
// ..592.....
// ......755.
// ...$.*....
// .664.598..
// In this schematic, two numbers are not part numbers because they are not adjacent to a symbol: 114 (top right) and 58 (middle right). Every other number is adjacent to a symbol and so is a part number; their sum is 4361.

// Of course, the actual engine schematic is much larger. What is the sum of all of the part numbers in the engine schematic?

use std::cmp;

fn to_u32(number: &str) -> (u32, u32) {
    let mut sum: u32 = 0;
    let mut n_digits: u32 = 0;
    for character in number.chars() {
        let new_digit = character.to_digit(10);
        match new_digit {
            None => return (sum, n_digits),
            Some(digit) => {
                n_digits += 1;
                sum = 10 * sum + digit;
            }
        }
    }
    (sum, n_digits)
}

fn get_line_value(line: &str, neighbor_to_symbol: &[bool]) -> Option<u32> {
    let mut sum: u32 = 0;
    let characters = line.chars();
    let mut i: usize = 0;
    while i < line.len() {
        if characters.clone().nth(i).unwrap().is_digit(10) {
            let (value, length) = to_u32(&line[i..]);
            for j in i..i + usize::try_from(length).unwrap() {
                if neighbor_to_symbol[j] {
                    sum += value;
                    i += usize::try_from(length).unwrap();
                    break;
                }
            }
        }
        i += 1;
    }
    return Some(sum);
}

fn part_1(input: &str) -> u32 {
    let mut sum: u32 = 0;
    let lines: Vec<&str> = input.lines().collect();
    let n_rows: usize = lines.len();
    let n_cols = lines[0].len();

    let mut neighbor_to_symbol: Vec<bool> = vec![false; n_rows * n_cols];

    for row in 0..n_rows {
        for col in 0..n_cols {
            let line = lines[row];
            let character = line.chars().nth(col).unwrap();
            if !character.is_digit(10) && character != '.' {
                for i in cmp::max(row - 1, 0)..=cmp::min(row + 1, n_rows) {
                    for j in cmp::max(col - 1, 0)..=cmp::min(col + 1, n_cols) {
                        neighbor_to_symbol[i * n_cols + j] = true;
                    }
                }
            }
        }
    }

    // Print the matrix of bool values
    // for i in 0..n_rows {
    //     println!("{:?}", &neighbor_to_symbol[i * n_cols..(i + 1) * n_cols]);
    // }

    for i in 0..n_rows {
        sum += get_line_value(&lines[i], &neighbor_to_symbol[i * n_cols..(i + 1) * n_cols])
            .unwrap_or(0);
    }
    sum
}

fn main() {
    let input = include_str!("../../input/input-1.txt");
    let result = part_1(input);
    println!("Total sum: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_accumulation() {
        let file = include_str!("../../input/test.txt");
        assert_eq!(part_1(file), 4361);
    }
}
