use regex::Regex;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);

    dbg!(output);
}


fn part1(input: &str) -> String {
    let re = Regex::new(r"(?<d>\d)").unwrap();
    let mut total: u32 = 0;
    for line in input.lines() {
        let digits: Vec<&str> = re.captures_iter(line)
            .flat_map(|caps| caps.get(1)
            .map(|n| n.as_str()))
            .collect();
        let num = 
            format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
            .parse::<u32>().unwrap();
        println!("line: {} {}", line, num);
        total += num;
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
        assert_eq!(result, "142".to_string());
    }
}