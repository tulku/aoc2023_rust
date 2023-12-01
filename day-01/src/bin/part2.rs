fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);

    dbg!(output);
}


fn part2(input: &str) -> String {
    let mut total: u32 = 0;
    for line in input.lines() {
        let mut nums: Vec<u32> = Vec::new();
        for (i, _) in line.char_indices() {
            let tail_slice = &line[i..];
            if tail_slice.starts_with("1") || tail_slice.starts_with("one") {
                nums.push(1);
            } else if tail_slice.starts_with("2") || tail_slice.starts_with("two") {
                nums.push(2);
            } else if tail_slice.starts_with("3") || tail_slice.starts_with("three") {
                nums.push(3);
            } else if tail_slice.starts_with("4") || tail_slice.starts_with("four") {
                nums.push(4);
            } else if tail_slice.starts_with("5") || tail_slice.starts_with("five") {
                nums.push(5);
            } else if tail_slice.starts_with("6") || tail_slice.starts_with("six") {
                nums.push(6);
            } else if tail_slice.starts_with("7") || tail_slice.starts_with("seven") {
                nums.push(7);
            } else if tail_slice.starts_with("8") || tail_slice.starts_with("eight") {
                nums.push(8);
            } else if tail_slice.starts_with("9") || tail_slice.starts_with("nine") {
                nums.push(9);
            } else if tail_slice.starts_with("0") || tail_slice.starts_with("zero") {
                nums.push(0);
            }
        }
        total += format!("{}{}", nums.first().unwrap(), nums.last().unwrap())
            .parse::<u32>().unwrap();
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
        assert_eq!(result, "281".to_string());
    }
}