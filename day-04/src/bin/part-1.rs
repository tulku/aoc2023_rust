use day_04::parse_card;

fn get_card_points(line: &str) -> Option<u32> {
    let card = match parse_card(line) {
        Ok(card) => card,
        Err(_) => return None,
    };
    Some(card.get_score())
}

fn part_1(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        sum += get_card_points(line).unwrap_or(0);
    }
    sum
}

fn main() {
    let input = include_str!("../../input/input-1.txt");
    let result = part_1(input);
    println!("Total points: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cards() {
        let input = include_str!("../../input/test-1.txt");
        let result = part_1(input);
        assert_eq!(13, result);
    }
}
