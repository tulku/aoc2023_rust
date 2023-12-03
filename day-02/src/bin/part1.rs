use regex::Regex;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);

    dbg!(output);
}


fn part1(input: &str) -> String {
    let game_re = Regex::new(r"^Game (?<game_id>\d+): (?<game_data>.*)$").unwrap();
    let color_re = Regex::new(r"(?<cubes>\d+) (?<color>(red|green|blue))").unwrap();
    let mut total: u32 = 0;
    for line in input.lines() {
        let Some(caps) = game_re.captures(line) else { panic!("RUN FOR YOUR LIVES!") };
        let game_id = &caps["game_id"].parse::<u32>().unwrap();
        let game_data = &caps["game_data"];
        
        let mut game_possible: bool = true;
        for set in game_data.split(";") {
            let mut red: u32 = 0;
            let mut green: u32 = 0;
            let mut blue: u32 = 0;

            for caps in color_re.captures_iter(set) {
                let cubes = &caps["cubes"].parse::<u32>().unwrap();
                match &caps["color"] {
                    "red" => {red += cubes;},
                    "green" => {green += cubes;},
                    "blue" => {blue += cubes;},
                    _ => {panic!("THE COLOR IS A LIE");}
                }
                if red > 12 || green > 13 || blue > 14 {
                    game_possible = false;
                }
            };
        }
        if game_possible {
            total += game_id;
        }
    }

    format!("{}", total).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_input = include_str!("./test1.txt");
        let result = part1(test_input);
        assert_eq!(result, "8".to_string());
    }
}