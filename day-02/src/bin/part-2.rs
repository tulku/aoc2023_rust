// --- Part Two ---
// The Elf says they've stopped producing snow because they aren't getting any water! He isn't sure why the water stopped; however, he can show you how to get to the water source to check it out for yourself. It's just up ahead!

// As you continue your walk, the Elf poses a second question: in each game you played, what is the fewest number of cubes of each color that could have been in the bag to make the game possible?

// Again consider the example games from earlier:

// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
// In game 1, the game could have been played with as few as 4 red, 2 green, and 6 blue cubes. If any color had even one fewer cube, the game would have been impossible.
// Game 2 could have been played with a minimum of 1 red, 3 green, and 4 blue cubes.
// Game 3 must have been played with at least 20 red, 13 green, and 6 blue cubes.
// Game 4 required at least 14 red, 3 green, and 15 blue cubes.
// Game 5 needed no fewer than 6 red, 3 green, and 2 blue cubes in the bag.
// The power of a set of cubes is equal to the numbers of red, green, and blue cubes multiplied together. The power of the minimum set of cubes in game 1 is 48. In games 2-5 it was 12, 1560, 630, and 36, respectively. Adding up these five powers produces the sum 2286.

// For each game, find the minimum set of cubes that must have been present. What is the sum of the power of these sets?

fn to_u32(number: &str) -> u32 {
    let mut sum: u32 = 0;
    for character in number.chars() {
        sum = 10 * sum + character.to_digit(10).unwrap();
    }
    sum
}

fn find_attempt_power(attempt: &str) -> Option<Vec<u32>> {
    let colors = vec!["red".to_string(), "green".to_string(), "blue".to_string()];
    let mut min_amount = vec![0, 0, 0];
    let balls_group: Vec<&str> = attempt.split(',').collect();
    for ball in balls_group {
        let collection_pair: Vec<&str> = ball.split(' ').collect();
        for index in 0..colors.len() {
            if collection_pair[2].starts_with(&colors[index]) {
                let amount = collection_pair[1];
                let amount_u32 = to_u32(amount);
                min_amount[index] = amount_u32;
            }
        }
    }
    Some(min_amount)
}

fn get_game_power(line: &str) -> Option<u32> {
    let attempts_no_name: Vec<&str> = line.split(":").collect();
    let attempts: Vec<&str> = attempts_no_name[1].split(';').collect();

    let mut min_amount = vec![0, 0, 0];
    for attempt in attempts {
        let new_powers = find_attempt_power(attempt);
        match new_powers {
            None => return None,
            Some(vector) => {
                for index in 0..vector.len() {
                    if vector[index] > min_amount[index] {
                        min_amount[index] = vector[index];
                    }
                }
            }
        }
    }
    return Some(min_amount[0] * min_amount[1] * min_amount[2]);
}

fn part_2(input: &str) -> u32 {
    let mut sum: u32 = 0;
    for game in input.lines() {
        sum += get_game_power(game).unwrap_or(0);
    }
    sum
}

fn main() {
    let input = include_str!("../../input/input-1.txt");
    let result = part_2(input);
    println!("Total sum: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_power() {
        let game = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        assert_eq!(get_game_power(game), Some(48));
        let game = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        assert_eq!(get_game_power(game), Some(12));
        let game = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        assert_eq!(get_game_power(game), Some(1560));
        let game = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        assert_eq!(get_game_power(game), Some(630));
        let game = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(get_game_power(game), Some(36));
    }

    #[test]
    fn test_power_accumulation() {
        let game = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part_2(game), 2286);
    }
}
