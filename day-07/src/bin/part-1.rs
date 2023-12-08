use std::collections::HashSet;

use regex::Regex;
#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    bid: usize,
}

impl Hand {
    fn from_input(input: &str) -> Option<Self> {
        let re = Regex::new(
            r"([2-9|AKQJT])([2-9|AKQJT])([2-9|AKQJT])([2-9|AKQJT])([2-9|AKQJT])\s(\d+)\s?",
        )
        .unwrap();
        let caps = re.captures(input)?;

        let card_groups = 1..=5;
        let cards: Vec<char> = card_groups
            .map(|index| caps.get(index).map(|m| m.as_str().chars().next().unwrap()))
            .flatten()
            .collect();
        let bid = caps.get(6).unwrap().as_str().parse::<usize>().unwrap();
        Some(Hand { cards, bid })
    }

    fn find_repeated(&self) {
        let set: HashSet<char> = self.cards.iter().cloned().collect();
        println!("set: {:?}, repeated: {}", set, self.cards.len() - set.len());
    }
}

fn parse_input(input: &str) -> Vec<Hand> {
    input
        .split("\n")
        .map(|line| Hand::from_input(line))
        .flatten()
        .collect()
}

fn find_hand_bid(hands: &[Hand]) -> usize {
    return 0;
}

fn part_1(input: &str) -> usize {
    let hands = parse_input(input);
    for hand in hands.iter() {
        println!("hand: {:?}", hand);
        hand.find_repeated();
    }
    find_hand_bid(&hands)
}

fn main() {
    let input = include_str!("../../input/test-1.txt");
    let score = part_1(input);
    println!("Score: {}", score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn test_input() {
        let input = include_str!("../../input/test-1.txt");
        let score = part_1(input);
        assert_eq!(score, 288);
    }
}
