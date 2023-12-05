use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let input = include_str!("./input2.txt");
    let output = part1(input);

    dbg!(output);
}

type EngineNumber = (isize, isize, i32);
type Coord = (isize, isize);

// I'm getting cells all the time, might as well write it here
fn cell(m: &Vec<&str>, row: isize, col: isize) -> Option<(isize, isize, char)> {
    let rows = m.len() as isize;
    let cols = m.get(0)?.len() as isize;

    if row < 0 || row >= rows {
        return None
    }
    if col < 0 || col >= cols {
        return None
    }

    Some((row, col, m.get(row as usize)?.chars().nth(col as usize)?))
}

// I don't want to write this many ifs
fn push_potential_gear_location(v: &mut Vec<(isize, isize)>, value: Option<(isize, isize, char)>) {
    if let Some((row, col, c)) = value {
        if c == '*' {
            v.push((row, col));
        }
    }
}

fn get_potential_gears(m: Vec<&str>, row: isize, col: isize, length: isize) -> Vec<(isize, isize)> {
    let mut v = Vec::new();
    // left neighbor
    push_potential_gear_location(&mut v, cell(&m, row, col-1));
    // right neighbor
    push_potential_gear_location(&mut v, cell(&m, row, col+length));
    // top and bottom neighbors
    for j in col-1..col+length+1 {
        push_potential_gear_location(&mut v, cell(&m, row-1, j));
        push_potential_gear_location(&mut v, cell(&m, row+1, j));
    }
    v
}

fn has_gear(number: &EngineNumber, number_to_potential_gears: &HashMap<EngineNumber, Vec<Coord>>, gear_location: Coord) -> bool {
    number_to_potential_gears.get(number).unwrap().contains(&gear_location)
}

fn part1(input: &str) -> Option<String> {
    let numbers_re = Regex::new(r"\d+").unwrap();
    let matrix: Vec<&str> = input.lines().collect();
    let mut total = 0;
    let mut number_to_potential_gear: HashMap<EngineNumber, Vec<Coord>> = HashMap::new();
    let mut potential_gears_locations: HashSet<Coord> = HashSet::new();
    let mut numbers: Vec<EngineNumber> = vec![];


    for (row, line) in matrix.iter().enumerate() {
        let matches: Vec<_> = numbers_re.find_iter(line).collect();
        for number in matches.iter() {
            let col = number.start();
            let num = number.as_str();
            let number_potential_gears = get_potential_gears(matrix.clone(), row as isize, col as isize, num.len() as isize);
            
            let num = (row as isize, col as isize, num.parse::<i32>().unwrap());
            potential_gears_locations.extend(number_potential_gears.iter());
            number_to_potential_gear.insert(num, number_potential_gears);
            numbers.push(num);
        }
    }
    for potential_gear_pos in potential_gears_locations {   
        let filtered: Vec<&EngineNumber> = numbers.iter().filter(|n| has_gear(n, &number_to_potential_gear, potential_gear_pos)).collect();
        if filtered.len() == 2 {
            total += filtered.get(0).unwrap().2 * filtered.get(1).unwrap().2;
        }
    }

    Some(format!("{}", total).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_input = include_str!("./test2.txt");
        let result = part1(test_input)?;
        assert_eq!(result, "467835".to_string());
    }
}