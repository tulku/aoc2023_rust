use day_05::parse_almanac;

fn part_1(input: &str) -> usize {
    let almanac = parse_almanac(input);
    let locations = almanac.get_all_locations();
    *locations.iter().min().unwrap()
}

fn main() {
    let input = include_str!("../../input/input-1.txt");
    let min_location = part_1(input);
    println!("Min location: {}", min_location);
}
