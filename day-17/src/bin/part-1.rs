// Title: Day 17 - Part 1
// Description: https://adventofcode.com/2023/day/17
use pathfinding::prelude::astar;
use std::hash::{Hash, Hasher};
#[derive(Debug, PartialEq, Copy, Clone)]
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
                (Direction::Down, if x > 0 { x - 1 } else { x }, y),
                (Direction::Up, x + 1, y),
                (Direction::Left, x, y + 1),
                (Direction::Right, x, if y > 0 { y - 1 } else { y }),
            ],

            Direction::Up => vec![
                (Direction::Up, x + 1, y),
                (Direction::Left, x, y + 1),
                (Direction::Right, x, if y > 0 { y - 1 } else { y }),
            ],
            Direction::Down => vec![
                (Direction::Down, if x > 0 { x - 1 } else { x }, y),
                (Direction::Left, x, y + 1),
                (Direction::Right, x, if y > 0 { y - 1 } else { y }),
            ],
            Direction::Left => vec![
                (Direction::Left, x, y + 1),
                (Direction::Down, if x > 0 { x - 1 } else { x }, y),
                (Direction::Up, x + 1, y),
            ],
            Direction::Right => vec![
                (Direction::Right, x, if y > 0 { y - 1 } else { y }),
                (Direction::Down, if x > 0 { x - 1 } else { x }, y),
                (Direction::Up, x + 1, y),
            ],
        };
        let mut iterator = options.iter();
        if self.straight >= 3 {
            iterator.next();
        }
        let max_pose = (map.len() - 1, map[0].len() - 1);
        let possible_states = iterator
            .filter(|(_, x, y)| {
                if *x > max_pose.0 || *y > max_pose.1 {
                    return false;
                }
                true
            })
            .map(|(direction, x, y)| {
                (
                    State {
                        pos: (*x, *y),
                        from: *direction,
                        straight: if self.from == *direction {
                            self.straight + 1
                        } else {
                            0
                        },
                    },
                    map[*x][*y],
                )
            })
            .collect();
        println!("Possible states: {:?}", possible_states);
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

    println!("{:?}", arr);
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
    println!("Pos 1, 2: {:?}", map[0]);
    let result = astar(
        &start,
        |p| p.successors(&map),
        |p| p.distance(&goal),
        |p| *p == goal,
    );

    println!("Result: {:?}", result);
    return 0;
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
        assert_eq!(score, 288);
    }
}
