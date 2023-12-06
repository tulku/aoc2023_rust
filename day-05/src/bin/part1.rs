use regex::Regex;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);

    dbg!(output);
}

enum State {
    WaitingForMapping,
    FindingInterval,
}

fn part1(input: &str) -> Option<String> {
    let seeds_re = Regex::new(r"seeds: (?<seeds>[\d ]+)").unwrap();
    let numbers_re = Regex::new(r"\d+").unwrap();
    let weird_mapping_re = Regex::new(r"(?<dest>\d+) (?<source>\d+) (?<length>\d+)").unwrap();
    let mapping_is_comming = Regex::new(r"[a-z]+\-to\-[a-z]+ map:").unwrap();
    let mut total = i64::MAX;

    let mut state = State::WaitingForMapping;

    let seeds: &str = input.lines().take(1).next().unwrap();
    let seeds = seeds_re.captures(seeds).unwrap().name("seeds")?.as_str();
    let seeds: Vec<i64> = numbers_re.find_iter(seeds).map(|m| m.as_str().parse::<i64>().unwrap()).collect();
    for seed in seeds.iter() {
        println!("{}", seed);
        let mut location = *seed;
        for line in input.lines().skip(2) {
            match state {
                State::WaitingForMapping => {
                    if mapping_is_comming.is_match(line) {
                        state = State::FindingInterval;
                    } 
                },
                State::FindingInterval => {
                    if line.is_empty() {
                        state = State::WaitingForMapping;
                    } else {
                        let caps = weird_mapping_re.captures(line)?;
                        let dest = caps.name("dest")?.as_str().parse::<i64>().unwrap();
                        let source = caps.name("source")?.as_str().parse::<i64>().unwrap();
                        let length = caps.name("length")?.as_str().parse::<i64>().unwrap();
                        if source <= location && location - source < length {
                            location = dest + (location - source);
                            state = State::WaitingForMapping;
                        }
                    }
                }
            };
        }
        total = total.min(location);
    }


    Some(format!("{}", total).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_input = include_str!("./test1.txt");
        let result = part1(test_input)?;
        assert_eq!(result, "35".to_string());
    }
}