use day_02::parse_game;

fn find_game_power(game: &str) -> Option<u64> {
    let game = parse_game(game);
    let game = match game {
        Ok(game) => game,
        Err(_) => return None,
    };
    let mut max_blue = 0;
    let mut max_red = 0;
    let mut max_green = 0;
    for draw in game.draws.iter() {
        if draw.blue > max_blue {
            max_blue = draw.blue;
        }
        if draw.red > max_red {
            max_red = draw.red;
        }
        if draw.green > max_green {
            max_green = draw.green;
        }
    }
    Some(max_blue as u64 * max_red as u64 * max_green as u64)
}

fn part_2(input: &str) -> u64 {
    let mut sum = 0;
    for line in input.lines() {
        sum += find_game_power(line).unwrap_or(0);
    }
    sum
}

fn main() {
    let input = include_str!("../../input/input-1.txt");
    let result = part_2(input);
    println!("Total power: {}", result);
}
