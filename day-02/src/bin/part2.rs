use regex::Regex;

fn main() {
    let input = include_str!("./input2.txt");
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
        
        let mut min_red: u32 = 0;
        let mut min_green: u32 = 0;
        let mut min_blue: u32 = 0;
        for set in game_data.split(";") {
            for caps in color_re.captures_iter(set) {
                let cubes: u32 = *(&caps["cubes"].parse::<u32>().unwrap());
                match &caps["color"] {
                    "red" => {min_red = min_red.max(cubes);},
                    "green" => {min_green = min_green.max(cubes);},
                    "blue" => {min_blue = min_blue.max(cubes);},
                    _ => {panic!("THE COLOR IS A LIE");}
                }
            };
        }
        let power_set = min_red * min_green * min_blue;
        total += power_set;
    }

    format!("{}", total).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_input = include_str!("./test2.txt");
        let result = part1(test_input);
        assert_eq!(result, "2286".to_string());
    }
}