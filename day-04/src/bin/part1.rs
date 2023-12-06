use regex::Regex;
use std::collections::HashSet;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);

    dbg!(output);
}

fn part1(input: &str) -> Option<String> {
    let cards_re = Regex::new(r"Card +\d+: (?<winning_numbers>[\d ]+) \| (?<my_awesome_numbers>[\d ]+)").unwrap();
    let numbers_re = Regex::new(r"\d+").unwrap();
    let mut total = 0;

    for line in input.lines() {
        let caps = cards_re.captures(line).unwrap();
        let winning_numbers = caps.name("winning_numbers")?.as_str();
        let winning_numbers: HashSet<i32> = numbers_re.find_iter(winning_numbers).map(|m| m.as_str().parse::<i32>().unwrap()).collect();
        let my_awesome_numbers = caps.name("my_awesome_numbers")?.as_str();
        let my_awesome_numbers: Vec<i32> = numbers_re.find_iter(my_awesome_numbers).map(|m| m.as_str().parse::<i32>().unwrap()).collect();
        let n = my_awesome_numbers.iter().filter(|n| winning_numbers.contains(n)).collect::<Vec<_>>().len() as u32;
        if n > 0 {
            total += (2 as i32).pow(n-1);
        }
    }

    Some(format!("{}", total).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_input = include_str!("./test1.txt");
        let result = part1(test_input)?;
        assert_eq!(result, "13".to_string());
    }
}