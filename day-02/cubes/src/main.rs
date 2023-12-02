use std::collections::HashMap;
use std::cmp;
use std::path;
use std::fs;

use regex::Regex;
use clap::Parser;

/// Verify valid cube games.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CLI {
    /// Path to bag descriptor.
    #[arg(short, long)]
    bag_path: Option<path::PathBuf>,

    /// Path to gaming log file.
    log_path: path::PathBuf,
}

fn main() {
    let args = CLI::parse();

    let bag: HashMap<String, u32> = match args.bag_path {
        Some(path) => {
            let content = fs::read_to_string(path)
                .expect("Failed to read bag descriptor file!");
            content.lines().enumerate().map(|(index, line)| {
                let (color, number) = line.split_once(' ').unwrap_or_else(|| {
                    panic!("Bad bag descriptor format on line {}", index + 1)
                });
                (color.to_string(), number.parse::<u32>().unwrap_or_else(|err| {
                    panic!("Bad bag descriptor format on line {}: {}", index + 1, err)
                }))
            }).collect()
        },
        None => HashMap::from([
            ("red".to_string(), 12),
            ("green".to_string(), 13),
            ("blue".to_string(), 14),
        ])
    };

    let content = fs::read_to_string(args.log_path)
        .expect("Failed to read gaming log file!");

    let pattern = Regex::new(r"^Game (\d+): ((?:\d+ \w+(?:, |; )?)+)$").unwrap();

    let possible_games_sum = content.lines().enumerate().map(|(index, line)| {
        let caps = pattern.captures(line).unwrap_or_else(|| {
            panic!("Bad gaming log format on line {}", index + 1)
        });
        let game_id = caps.get(1).unwrap().as_str();
        let game_set = caps.get(2).unwrap().as_str();
        for game in game_set.split("; ") {
            for draw in game.split(", ") {
                let (number, color) = draw.split_once(' ').unwrap();
                let total = bag.get(color).unwrap_or_else(|| {
                    panic!("Unknown color: {}", color)
                });
                if number.parse::<u32>().unwrap() > *total {
                    println!("Game {} is impossible: {} > {}", game_id, number, total);
                    return 0;
                }
            }
        }
        // println!("Game {} is OK: {}", game_id, game_set);
        game_id.parse::<u32>().unwrap()
    }).sum::<u32>();

    let game_power_sum = content.lines().enumerate().map(|(index, line)| {
        let caps = pattern.captures(line).unwrap_or_else(|| {
            panic!("Bad gaming log format on line {}", index + 1)
        });
        let mut minimum_set: HashMap<&str, u32> = HashMap::new();
        let game_set = caps.get(2).unwrap().as_str();
        for game in game_set.split("; ") {
            for draw in game.split(", ") {
                let (number, color) = draw.split_once(' ').unwrap();
                minimum_set.insert(color, cmp::max(
                    minimum_set.get(color).copied().unwrap_or(0),
                    number.parse::<u32>().unwrap()
                ));
            }
        }
        minimum_set.values().product::<u32>()
    }).sum::<u32>();

    println!("Possible games: {}", possible_games_sum);
    println!("Power of games: {}", game_power_sum);
}
