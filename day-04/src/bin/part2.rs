use regex::Regex;
use std::collections::HashSet;

fn main() {
    let input = include_str!("./input2.txt");
    let output = part1(input);

    dbg!(output);
}

fn part1(input: &str) -> Option<String> {
    let cards_re = Regex::new(r"Card +\d+: (?<winning_numbers>[\d ]+) \| (?<my_awesome_numbers>[\d ]+)").unwrap();
    let numbers_re = Regex::new(r"\d+").unwrap();
    let mut copies: Vec<i32> = vec![1; input.lines().count()];

    for (i, line) in input.lines().enumerate() {
        let caps = cards_re.captures(line).unwrap();
        let winning_numbers = caps.name("winning_numbers")?.as_str();
        let winning_numbers: HashSet<i32> = numbers_re.find_iter(winning_numbers).map(|m| m.as_str().parse::<i32>().unwrap()).collect();
        let my_awesome_numbers = caps.name("my_awesome_numbers")?.as_str();
        let my_awesome_numbers: Vec<i32> = numbers_re.find_iter(my_awesome_numbers).map(|m| m.as_str().parse::<i32>().unwrap()).collect();
        let mut n = my_awesome_numbers.iter().filter(|n| winning_numbers.contains(n)).collect::<Vec<_>>().len() as usize;
        while n > 0 {
            *copies.get_mut(i + n)? += *copies.get(i)?;
            n -= 1;
        }
    }
    let total: i32 = copies.iter().sum();

    Some(format!("{}", total).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_input = include_str!("./test2.txt");
        let result = part1(test_input)?;
        assert_eq!(result, "30".to_string());
    }
}