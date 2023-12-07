// This document describes three races:

//     The first race lasts 7 milliseconds. The record distance in this race is 9 millimeters.
//     The second race lasts 15 milliseconds. The record distance in this race is 40 millimeters.
//     The third race lasts 30 milliseconds. The record distance in this race is 200 millimeters.

use regex::Regex;
#[derive(Debug)]
struct Race {
    time: usize,
    max_distance: usize,
}

fn get_distance(accel_time: usize, max_time: usize) -> usize {
    let remaining_time = max_time - accel_time;
    accel_time * remaining_time
}

impl Race {
    fn find_winning_accel_times(&self) -> Vec<usize> {
        let mut times = Vec::new();
        for i in 1..self.time {
            let distance = get_distance(i, self.time);
            if distance > self.max_distance {
                times.push(i);
            }
        }
        times
    }
}

fn get_numbers(line: &str) -> Vec<usize> {
    let line: String = line.split_whitespace().collect();
    let re = Regex::new(r"(\d+)").unwrap();
    let numbers = re
        .captures_iter(line.as_str())
        .map(|caps| caps.get(0).unwrap().as_str().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    numbers
}

fn get_races(times: &[usize], distances: &[usize]) -> Vec<Race> {
    let mut races = Vec::new();
    for it in times.iter().zip(distances.iter()) {
        races.push(Race {
            time: *it.0,
            max_distance: *it.1,
        });
    }
    races
}

fn parse_input(input: &str) -> Vec<Race> {
    let re = Regex::new(r"Time:\s+(?P<times>((\d+)\s*)+)|Distance:\s+(?P<distances>((\d+)\s*)+)")
        .unwrap();

    let mut times: Option<Vec<usize>> = None;
    let mut distances: Option<Vec<usize>> = None;

    for captures in re.captures_iter(input) {
        times = captures
            .name("times")
            .map_or(times, |time_str| Some(get_numbers(time_str.as_str())));
        distances = captures
            .name("distances")
            .map_or(distances, |distance_str| {
                Some(get_numbers(distance_str.as_str()))
            });
    }

    if times.as_ref().and(distances.as_ref()).is_some() {
        get_races(&times.unwrap(), &distances.unwrap())
    } else {
        Vec::<Race>::new()
    }
}

fn find_winning_score(races: &[Race]) -> usize {
    let mut total_score: usize = 1;
    for race in races {
        let winning_count = race.find_winning_accel_times().len();
        total_score *= winning_count;
        println!("For race {:?}, Winning count: {}", race, winning_count);
    }
    total_score
}

fn part_1(input: &str) -> usize {
    let races = parse_input(input);
    find_winning_score(&races)
}

fn main() {
    let input = include_str!("../../input/input-1.txt");
    let score = part_1(input);
    println!("Score: {}", score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn test_input() {
        let input = include_str!("../../input/test-1.txt");
        let score = part_1(input);
        assert_eq!(score, 71503);
    }
}
