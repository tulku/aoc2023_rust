use std::collections::HashSet;

use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Card {
    pub id: u32,
    winners: HashSet<u32>,
    got: Vec<u32>,
}

impl Card {
    pub fn winner_count(&self) -> usize {
        self.got
            .iter()
            .filter(|number| self.winners.contains(&number))
            .count()
    }

    pub fn get_score(&self) -> u32 {
        let count = self.winner_count();
        if count == 0 {
            0
        } else {
            2_u32.pow(count as u32 - 1)
        }
    }
}

fn get_id(line: &str) -> Option<u32> {
    let re = Regex::new(r"Card\s+(?P<id>\d+)").unwrap();
    let Some(caps) = re.captures(line) else { return None };
    Some(caps["id"].parse::<u32>().unwrap().to_owned())
}

fn get_numbers(line: &str) -> Vec<u32> {
    let re = Regex::new(r"(\d+)").unwrap();
    let numbers = re
        .captures_iter(line)
        .map(|caps| caps.get(0).unwrap().as_str().parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    numbers
}

pub fn parse_card(line: &str) -> Result<Card, &str> {
    let mut parts = line.split(": ");
    let id = get_id(parts.next().unwrap()).ok_or("No id")?;
    let mut parts = parts.next().unwrap().split("|");
    let winners = get_numbers(parts.next().unwrap());
    let winners: HashSet<u32> = winners.into_iter().collect();
    let got = get_numbers(parts.next().unwrap());
    Ok(Card { id, winners, got })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_card() {
        let line = "Card 1: 1 | 1";
        let card = parse_card(line).unwrap();
        assert_eq!(card.id, 1);
        assert_eq!(card.winners, vec![1].into_iter().collect::<HashSet<u32>>());
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
                .collect::<HashSet<u32>>()
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

        // Card   2: 98 96 50 60  7 40 83 93 55 26 | 45 38 47 98 32 50 55 35 93 11 97 53 74 83 99 60 73 56 40 58 96 66 90 26  7
        // Card   3: 82  8 12 15 53 23 29 61  5 21 | 21 73  5 65 44 29 61 97 15  4 90 76 53 91 13 27  9 11  2 75 22 92 95 82 86
        // Card   4: 68 22 77 52 23 60 57 31 74 38 | 22 38 68 79 52 23 40 57 10 74 31 83 24 60 95 17 78 89 39 37 87 26 77 63 54
    }
}
