use regex::Regex;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);

    dbg!(output);
}

// I'm getting cells all the time, might as well write it here
fn cell(m: &Vec<&str>, row: isize, col: isize) -> Option<char> {
    let rows = m.len() as isize;
    let cols = m.get(0)?.len() as isize;

    if row < 0 || row >= rows {
        return None
    }
    if col < 0 || col >= cols {
        return None
    }

    m.get(row as usize)?.chars().nth(col as usize)
}

// I don't want to write this many ifs
fn push_value(v: &mut Vec<char>, value: Option<char>) {
    if let Some(c) = value {
        v.push(c);
    }
}

fn get_neighbors(m: Vec<&str>, row: isize, col: isize, length: isize) -> Vec<char> {
    let mut v = Vec::new();
    // left neighbor
    push_value(&mut v, cell(&m, row, col-1));
    // right neighbor
    push_value(&mut v, cell(&m, row, col+length));
    // top and bottom neighbors
    for j in col-1..col+length+1 {
        push_value(&mut v, cell(&m, row-1, j));
        push_value(&mut v, cell(&m, row+1, j));
    }
    v
}

fn part1(input: &str) -> Option<String> {
    let numbers_re = Regex::new(r"\d+").unwrap();
    let matrix: Vec<&str> = input.lines().collect();
    let mut total = 0;
    for (row, line) in matrix.iter().enumerate() {
        let matches: Vec<_> = numbers_re.find_iter(line).collect();
        for number in matches.iter() {
            let col = number.start();
            let num = number.as_str();
            let neighbors = get_neighbors(matrix.clone(), row as isize, col as isize, num.len() as isize);
            // Check if there's a symbol besides the dot
            if neighbors.iter().any(|&x| x != '.') {
                total += num.parse::<u32>().unwrap();
            }
        }
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
        assert_eq!(result, "4361".to_string());
    }
}