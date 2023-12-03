use std::collections::HashSet;

use regex::Regex;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Number {
    value: u32,
    line_index: usize,
    start_index: usize,
    end_index: usize,
}

#[derive(Debug)]
struct PossibleGear {
    number: Number,
    device_line_index: usize,
    device_index: usize,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Gear {
    numbers: Vec<Number>,
    device_line_index: usize,
    device_index: usize,
}

fn find_numbers(line: &str, line_index: usize) -> Vec<Number> {
    let mut numbers = Vec::new();
    let re = Regex::new(r"(\d+)").unwrap();
    for caps in re.captures_iter(line) {
        let captured = &caps.get(0).unwrap();
        numbers.push(Number {
            value: captured.as_str().parse::<u32>().unwrap(),
            line_index,
            start_index: captured.start(),
            end_index: captured.end(),
        });
    }
    numbers
}

fn load_schematic(input: &str) -> Vec<String> {
    let mut schematic = Vec::new();
    for line in input.lines() {
        schematic.push(line.to_string());
    }
    schematic
}

fn validate_number(number: &Number, schematic: &Vec<String>) -> (bool, Vec<PossibleGear>) {
    let possible_lines = if number.line_index > 0 {
        vec![
            number.line_index - 1,
            number.line_index,
            number.line_index + 1,
        ]
    } else {
        vec![number.line_index, number.line_index + 1]
    };

    let possible_cols = if number.start_index > 0 && number.end_index > 0 {
        vec![
            number.start_index - 1,
            number.start_index,
            number.start_index + 1,
            number.end_index - 1,
            number.end_index,
        ]
    } else {
        vec![number.start_index, number.start_index + 1, number.end_index]
    };
    let mut gears: Vec<PossibleGear> = Vec::new();
    let mut valid: bool = false;

    schematic
        .iter()
        .enumerate()
        .filter(|&(i, _)| possible_lines.contains(&i))
        .for_each(|(line_index, line)| {
            line.chars()
                .enumerate()
                .filter(|&(j, _)| possible_cols.contains(&j))
                .for_each(|(char_index, symbol)| {
                    if !symbol.is_digit(10) && symbol != '.' {
                        valid = true;
                    }
                    if symbol == '*' {
                        gears.push(PossibleGear {
                            number: number.to_owned(),
                            device_line_index: line_index,
                            device_index: char_index,
                        });
                    }
                })
        });

    (valid, gears)
}

fn process_gears(gears: &[PossibleGear]) -> u64 {
    let mut real_gears: HashSet<Gear> = HashSet::new();
    for gear in gears {
        for other_gear in gears {
            if gear.number == other_gear.number {
                continue;
            }
            if gear.device_index == other_gear.device_index
                && gear.device_line_index == other_gear.device_line_index
            {
                let numbers = vec![gear.number.clone(), other_gear.number.clone()];
                let rev_numbers = vec![other_gear.number.clone(), gear.number.clone()];
                if real_gears
                    .iter()
                    .any(|g| g.numbers == numbers || g.numbers == rev_numbers)
                {
                    continue;
                }
                real_gears.insert(Gear {
                    numbers: vec![gear.number.clone(), other_gear.number.clone()],
                    device_line_index: gear.device_line_index,
                    device_index: gear.device_index,
                });
            }
        }
    }
    let mut sum: u64 = 0;
    for gear in &real_gears {
        let ratio: u64 = gear.numbers[0].value as u64 * gear.numbers[1].value as u64;
        sum += ratio;
        println!("Gear Ratio: {:?}", ratio);
    }
    sum
}

fn part_1(input: &str) -> u64 {
    let schematic = load_schematic(input);
    let mut possible_gears: Vec<PossibleGear> = Vec::new();
    for (index, line) in schematic.iter().enumerate() {
        let numbers = find_numbers(line, index);
        for number in numbers {
            let (_, mut gears) = validate_number(&number, &schematic);
            possible_gears.append(&mut gears);
        }
    }
    process_gears(&possible_gears)
}

fn main() {
    let input = include_str!("../../input/input-1.txt");
    let result = part_1(input);
    println!("All ratios together: {}", result);
}
