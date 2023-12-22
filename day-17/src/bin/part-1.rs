// Title: Day 17 - Part 1
// Description: https://adventofcode.com/2023/day/17
use itertools::Itertools;
use pathfinding::prelude::astar;
use std::hash::{Hash, Hasher};
#[derive(Debug, PartialEq, Copy, Clone, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Any,
}
#[derive(Debug, Copy, Clone)]
struct State {
    pos: (usize, usize),
    from: Direction,
    straight: u32,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}
impl Eq for State {}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
    }
}

impl State {
    fn distance(&self, other: &State) -> u32 {
        (self.pos.0.abs_diff(other.pos.0) + self.pos.1.abs_diff(other.pos.1)) as u32
    }

    fn successors(&self, map: &Vec<Vec<u32>>) -> Vec<(State, u32)> {
        let (x, y) = self.pos;

        let options = match self.from {
            Direction::Any => vec![
                (
                    Direction::Down,
                    if x > 0 { Some(x - 1) } else { None },
                    Some(y),
                ),
                (Direction::Up, Some(x + 1), Some(y)),
                (Direction::Left, Some(x), Some(y + 1)),
                (
                    Direction::Right,
                    Some(x),
                    if y > 0 { Some(y - 1) } else { None },
                ),
            ],
            Direction::Up => vec![
                (Direction::Up, Some(x + 1), Some(y)),
                (Direction::Left, Some(x), Some(y + 1)),
                (
                    Direction::Right,
                    Some(x),
                    if y > 0 { Some(y - 1) } else { None },
                ),
            ],
            Direction::Down => vec![
                (
                    Direction::Down,
                    if x > 0 { Some(x - 1) } else { None },
                    Some(y),
                ),
                (Direction::Left, Some(x), Some(y + 1)),
                (
                    Direction::Right,
                    Some(x),
                    if y > 0 { Some(y - 1) } else { None },
                ),
            ],
            Direction::Left => vec![
                (Direction::Left, Some(x), Some(y + 1)),
                (
                    Direction::Down,
                    if x > 0 { Some(x - 1) } else { None },
                    Some(y),
                ),
                (Direction::Up, Some(x + 1), Some(y)),
            ],
            Direction::Right => vec![
                (
                    Direction::Right,
                    Some(x),
                    if y > 0 { Some(y - 1) } else { None },
                ),
                (
                    Direction::Down,
                    if x > 0 { Some(x - 1) } else { None },
                    Some(y),
                ),
                (Direction::Up, Some(x + 1), Some(y)),
            ],
        };
        let mut iterator = options.iter();
        if self.straight >= 2 {
            iterator.next();
        }
        let max_pose = (map.len() - 1, map[0].len() - 1);
        let possible_states = iterator
            .filter(|(_, x, y)| {
                if x.is_none() || y.is_none() {
                    return false;
                }
                true
            })
            .map(|(direction, x, y)| (*direction, x.unwrap(), y.unwrap()))
            .filter(|(_, x, y)| {
                if *x > max_pose.0 || *y > max_pose.1 {
                    return false;
                }
                true
            })
            .map(|(direction, x, y)| {
                (
                    State {
                        pos: (x, y),
                        from: direction,
                        straight: if self.from == direction {
                            self.straight + 1
                        } else {
                            0
                        },
                    },
                    map[x][y],
                )
            })
            .unique()
            .collect::<Vec<(State, u32)>>();

        // let states = possible_states
        //     .iter()
        //     .map(|(state, cost)| (state.pos, state.from, *cost))
        //     .collect::<Vec<((usize, usize), Direction, u32)>>();

        // println!("From {:?} -> next: {:?}", self, states);
        possible_states
    }
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    let arr: Vec<Vec<u32>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|number| number.to_digit(10).unwrap())
                .collect()
        })
        .collect();
    arr
}

fn part_1(input: &str) -> usize {
    let map = parse_input(input);
    let start = State {
        pos: (0, 0),
        from: Direction::Any,
        straight: 0,
    };
    let goal = State {
        pos: (map.len() - 1, map[0].len() - 1),
        from: Direction::Any,
        straight: 0,
    };
    let result = astar(
        &start,
        |p| p.successors(&map),
        |p| p.distance(&goal),
        |p| *p == goal,
    );

    // println!("Result: {:?}", result);
    plot_path(&map, &result.as_ref().unwrap().0);
    result.unwrap().1 as usize - map[goal.pos.0][goal.pos.1] as usize
}

fn plot_path(map: &[Vec<u32>], path: &[State]) {
    let mut map: Vec<Vec<&str>> = map
        .iter()
        .map(|row| row.iter().map(|_| ".").collect())
        .collect();
    for state in path {
        let dir = match state.from {
            Direction::Up => "v",
            Direction::Down => "^",
            Direction::Left => ">",
            Direction::Right => "<",
            Direction::Any => "X",
        };
        map[state.pos.0][state.pos.1] = dir;
    }
    for row in map {
        println!("{}", row.as_slice().join(" "));
    }
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
        assert_eq!(score, 102);
    }
}
