use std::collections::{HashMap, HashSet};

use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Card {
    pub id: usize,
    winners: HashSet<usize>,
    got: Vec<usize>,
}

impl Card {
    pub fn winners(&self) -> Vec<&usize> {
        self.got
            .iter()
            .filter(|number| self.winners.contains(&number))
            .collect()
    }
    pub fn winner_count(&self) -> usize {
        self.winners().iter().count()
    }

    pub fn get_score(&self) -> u32 {
        let count = self.winner_count();
        if count == 0 {
            0
        } else {
            2_u32.pow(count as u32 - 1)
        }
    }

    pub fn prices(&self) -> Vec<usize> {
        let mut prices: Vec<usize> = Vec::new();
        for price_card in self.id + 1..self.id + self.winner_count() + 1 as usize {
            prices.push(price_card);
        }
        prices
    }
}

fn get_id(line: &str) -> Option<usize> {
    let re = Regex::new(r"Card\s+(?P<id>\d+)").unwrap();
    let Some(caps) = re.captures(line) else { return None };
    Some(caps["id"].parse::<usize>().unwrap().to_owned())
}

fn get_numbers(line: &str) -> Vec<usize> {
    let re = Regex::new(r"(\d+)").unwrap();
    let numbers = re
        .captures_iter(line)
        .map(|caps| caps.get(0).unwrap().as_str().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    numbers
}

pub fn parse_card(line: &str) -> Result<Card, &str> {
    let mut parts = line.split(": ");
    let id = get_id(parts.next().unwrap()).ok_or("No id")?;
    let mut parts = parts.next().unwrap().split("|");
    let winners = get_numbers(parts.next().unwrap());
    let winners = winners.into_iter().collect();
    let got = get_numbers(parts.next().unwrap());
    Ok(Card { id, winners, got })
}

pub fn parse_cards(lines: &str) -> HashMap<usize, Card> {
    let mut cards = HashMap::new();
    for line in lines.lines() {
        let card = parse_card(line).unwrap();
        cards.insert(card.id, card);
    }
    cards
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_card() {
        let line = "Card 1: 1 | 1";
        let card = parse_card(line).unwrap();
        assert_eq!(card.id, 1);
        assert_eq!(
            card.winners,
            vec![1].into_iter().collect::<HashSet<usize>>()
        );
        assert_eq!(card.got, vec![1]);
        assert_eq!(card.winner_count(), 1);
        assert_eq!(card.get_score(), 1);

        let line = "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19";
        let card = parse_card(line).unwrap();
        println!("{:?}", card);
        assert_eq!(card.id, 2);
        assert_eq!(
            card.winners,
            vec![13, 32, 20, 16, 61]
                .into_iter()
                .collect::<HashSet<usize>>()
        );
        assert_eq!(card.got, vec![61, 30, 68, 82, 17, 32, 24, 19]);
        assert_eq!(card.winner_count(), 2);
        assert_eq!(card.get_score(), 2);

        let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card = parse_card(line).unwrap();
        assert_eq!(card.get_score(), 8);

        let line = "Card   1: 99 46 62 92 60 37 52 56 41 31 | 83 40 31 33 46  3 10 39 82  8 64 35  5 63 60 72 48 87 11 81 95 34 97 37 99";
        let winners = parse_card(line).unwrap().winner_count();
        assert_eq!(winners, 5);
    }

    #[test]
    fn test_parse_cards() {
        let input = include_str!("../input/test-1.txt");
        let cards = parse_cards(input);
        assert_eq!(cards.len(), 6);
    }
}
