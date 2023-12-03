use day_02::{parse_game, Draw, Game};

type MaxDraw = Draw;

fn is_game_impossible(game: &Game, max_draw: &MaxDraw) -> bool {
    for draw in game.draws.iter() {
        if draw.blue > max_draw.blue || draw.red > max_draw.red || draw.green > max_draw.green {
            return true;
        }
    }
    false
}

fn possible_game_id(game: &str, max_draw: &MaxDraw) -> Option<u32> {
    let game = parse_game(game);
    let game = match game {
        Ok(game) => game,
        Err(_) => return None,
    };
    if is_game_impossible(&game, max_draw) {
        None
    } else {
        Some(game.id)
    }
}

fn part_1(input: &str) -> u32 {
    let max_draw = MaxDraw {
        blue: 14,
        red: 12,
        green: 13,
    };

    let mut sum = 0;
    for line in input.lines() {
        sum += possible_game_id(line, &max_draw).unwrap_or(0);
    }
    sum
}

fn main() {
    let input = include_str!("../../input/input-1.txt");
    let result = part_1(input);
    println!("Total impossible games: {}", result);
}
