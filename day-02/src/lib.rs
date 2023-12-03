// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green

use regex::Regex;

pub fn parse_game(line: &str) -> Result<Game, &str> {
    let mut parts = line.split(": ");
    let id = get_id(parts.next().unwrap()).ok_or("No id")?;
    let draws = get_cubes(parts.next().unwrap());

    Ok(Game { id, draws })
}

#[derive(Debug, PartialEq)]
pub struct Draw {
    pub blue: u32,
    pub red: u32,
    pub green: u32,
}

#[derive(Debug, PartialEq)]
pub struct Game {
    pub id: u32,
    pub draws: Vec<Draw>,
}

fn get_id(line: &str) -> Option<u32> {
    let re = Regex::new(r"Game\s(?P<id>\d+)").unwrap();
    let Some(caps) = re.captures(line) else { return None };
    Some(caps["id"].parse::<u32>().unwrap().to_owned())
}

fn get_draw(line: &str) -> Draw {
    let re = Regex::new(r"((?P<count>\d+)\s(?P<color>blue|red|green)),?\s?").unwrap();
    let mut draw = Draw {
        blue: 0,
        red: 0,
        green: 0,
    };
    for captures in re.captures_iter(line) {
        let count = captures["count"].parse::<u32>().unwrap();
        let color = &captures["color"];
        match color {
            "blue" => draw.blue = count,
            "red" => draw.red = count,
            "green" => draw.green = count,
            _ => (),
        }
    }
    draw
}

fn get_cubes(line: &str) -> Vec<Draw> {
    let games = line.split(";");
    games
        .map(|game| {
            let draws = get_draw(game);
            draws
        })
        .collect::<Vec<Draw>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_id() {
        let line = "Game: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let id = get_id(line);
        assert_eq!(id, None);

        let line = "Game 14: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let id = get_id(line);
        assert_eq!(id, Some(14));

        let line = "Game 0: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let id = get_id(line);
        assert_eq!(id, Some(0));
    }

    #[test]
    fn test_parse_game() {
        let line = "Game: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = parse_game(line);
        assert_eq!(game, Err("No id"));

        let line = "Game 14: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = parse_game(line);
        let gt = Game {
            id: 14,
            draws: vec![
                Draw {
                    blue: 3,
                    red: 4,
                    green: 0,
                },
                Draw {
                    blue: 6,
                    red: 1,
                    green: 2,
                },
                Draw {
                    blue: 0,
                    red: 0,
                    green: 2,
                },
            ],
        };
        assert_eq!(game, Ok(gt));
    }
}
