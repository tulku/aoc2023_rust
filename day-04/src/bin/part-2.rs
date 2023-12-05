use std::collections::HashMap;

use day_04::{parse_cards, Card};

fn duplicates<'a>(card: &'a Card, cards: &'a HashMap<usize, Card>) -> Vec<&'a Card> {
    let mut duplicated: Vec<&Card> = Vec::new();
    let prices = card.prices();
    for price in prices {
        let card = match cards.get(&price) {
            Some(card) => card,
            None => continue,
        };
        duplicated.push(card);
    }
    duplicated
}

fn part_2(input: &str) -> usize {
    let cards = parse_cards(input);

    for (_, card) in &cards {
        let dups = duplicates(card, &cards);
        for dup in dups.iter() {
            println!("Duplicated {:?}", dup);
        }
    }

    20
}

fn main() {
    let input = include_str!("../../input/input-1.txt");
    let result = part_2(input);
    println!("Total points: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cards() {
        let input = include_str!("../../input/test-1.txt");
        let result = part_2(input);
        assert_eq!(30, result);
    }
}
