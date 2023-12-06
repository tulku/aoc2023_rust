use day_05::parse_almanac;

fn part_2(input: &str) -> usize {
    let almanac = parse_almanac(input);
    almanac.get_min_location_ranged_seeds()
}

fn main() {
    let input = include_str!("../../input/input-1.txt");
    let min_location = part_2(input);
    println!("Min location: {}", min_location);
}
