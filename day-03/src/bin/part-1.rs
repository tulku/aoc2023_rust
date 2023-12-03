use regex::Regex;

#[derive(Debug)]
struct Number {
    value: u32,
    line_index: usize,
    start_index: usize,
    end_index: usize,
}

fn find_numbers(line: &str, line_index: usize) -> Vec<Number> {
    let mut numbers = Vec::new();
    let re = Regex::new(r"(\d+)").unwrap();
    for caps in re.captures_iter(line) {
        let captured = &caps.get(0).unwrap();
        numbers.push(Number {
            value: captured.as_str().parse::<u32>().unwrap(),
            line_index: line_index,
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

fn validate_number(number: &Number, schematic: &Vec<String>) -> bool {
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
    let mut symbols: Vec<char> = Vec::new();
    schematic
        .iter()
        .enumerate()
        .filter(|&(i, _)| possible_lines.contains(&i))
        .for_each(|(_, line)| {
            line.chars()
                .enumerate()
                .filter(|&(j, _)| possible_cols.contains(&j))
                .for_each(|(_, symbol)| {
                    if !symbol.is_digit(10) && symbol != '.' {
                        symbols.push(symbol);
                    }
                })
        });
    symbols.len() > 0
}

fn part_1(input: &str) -> u64 {
    let schematic = load_schematic(input);
    let mut sum: u64 = 0;
    for (index, line) in schematic.iter().enumerate() {
        let numbers = find_numbers(line, index);
        for number in numbers {
            if validate_number(&number, &schematic) {
                sum += number.value as u64;
            }
        }
    }
    sum
}

fn main() {
    let input = include_str!("../../input/input-1.txt");
    let result = part_1(input);
    println!("All parts together: {}", result);
}
