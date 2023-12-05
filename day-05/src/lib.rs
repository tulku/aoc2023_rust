use std::collections::HashMap;

use regex::Regex;
#[derive(Debug)]
pub struct Almanac {
    seeds: Vec<usize>,
    seeds_to_soil: HashMap<usize, usize>,
    soil_to_fertilizer: HashMap<usize, usize>,
    fertilizer_to_water: HashMap<usize, usize>,
    water_to_light: HashMap<usize, usize>,
    light_to_temperature: HashMap<usize, usize>,
    temperature_to_humidity: HashMap<usize, usize>,
    humidity_to_location: HashMap<usize, usize>,
}

impl Almanac {
    fn new() -> Self {
        Almanac {
            seeds: Vec::new(),
            seeds_to_soil: HashMap::new(),
            soil_to_fertilizer: HashMap::new(),
            fertilizer_to_water: HashMap::new(),
            water_to_light: HashMap::new(),
            light_to_temperature: HashMap::new(),
            temperature_to_humidity: HashMap::new(),
            humidity_to_location: HashMap::new(),
        }
    }

    pub fn seed_to_soil(&self, seed: usize) -> usize {
        *self.seeds_to_soil.get(&seed).unwrap_or(&seed)
    }

    pub fn soil_to_fertilizer(&self, soil: usize) -> usize {
        *self.soil_to_fertilizer.get(&soil).unwrap_or(&soil)
    }

    pub fn fertilizer_to_water(&self, input: usize) -> usize {
        *self.fertilizer_to_water.get(&input).unwrap_or(&input)
    }

    pub fn water_to_light(&self, input: usize) -> usize {
        *self.water_to_light.get(&input).unwrap_or(&input)
    }

    pub fn light_to_temperature(&self, input: usize) -> usize {
        *self.light_to_temperature.get(&input).unwrap_or(&input)
    }

    pub fn temperature_to_humidity(&self, input: usize) -> usize {
        *self.temperature_to_humidity.get(&input).unwrap_or(&input)
    }

    pub fn humidity_to_location(&self, input: usize) -> usize {
        *self.humidity_to_location.get(&input).unwrap_or(&input)
    }

    pub fn seed_to_location(&self, seed: usize) -> usize {
        let soil = self.seed_to_soil(seed);
        let fertilizer = self.soil_to_fertilizer(soil);
        let water = self.fertilizer_to_water(fertilizer);
        let light = self.water_to_light(water);
        let temperature = self.light_to_temperature(light);
        let humidity = self.temperature_to_humidity(temperature);
        let location = self.humidity_to_location(humidity);
        location
    }
}

fn get_numbers(line: &str) -> Vec<usize> {
    let re = Regex::new(r"(\d+)").unwrap();
    let numbers = re
        .captures_iter(line)
        .map(|caps| caps.get(0).unwrap().as_str().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    numbers
}

fn build_map(numbers: Vec<usize>) -> HashMap<usize, usize> {
    let mut map = HashMap::new();
    for map_set in numbers.chunks(3) {
        let destination_start = map_set[0];
        let source_start = map_set[1];
        let length = map_set[2];
        for i in 0..length - 1 {
            map.insert(source_start + i, destination_start + i);
        }
    }
    map
}

pub fn parse_almanac(almanac: &str) -> Almanac {
    let re = Regex::new(r"(?P<label>[\w|-]+(\smap)?)?:\s(?P<numbers>(\d+\s?)*)").unwrap();
    let mut parsed_almanac = Almanac::new();
    for captures in re.captures_iter(almanac) {
        let label = captures.name("label").unwrap().as_str();
        let numbers = str::replace(captures.name("numbers").unwrap().as_str(), "\n", " ");
        let numbers = get_numbers(numbers.as_str());
        println!("Label {}", label);
        match label {
            "seeds" => parsed_almanac.seeds = numbers,
            "seed-to-soil map" => parsed_almanac.seeds_to_soil = build_map(numbers),
            "soil-to-fertilizer map" => parsed_almanac.soil_to_fertilizer = build_map(numbers),
            "fertilizer-to-water map" => parsed_almanac.fertilizer_to_water = build_map(numbers),
            "water-to-light map" => parsed_almanac.water_to_light = build_map(numbers),
            "light-to-temperature map" => parsed_almanac.light_to_temperature = build_map(numbers),
            "temperature-to-humidity map" => {
                parsed_almanac.temperature_to_humidity = build_map(numbers)
            }
            "humidity-to-location map" => parsed_almanac.humidity_to_location = build_map(numbers),
            _ => (),
        }
    }
    parsed_almanac
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn test_parse_test_almanac() {
        let input = include_str!("../input/test-1.txt");
        let almanac = parse_almanac(input);
        assert_eq!(almanac.seed_to_soil(79), 81);
        assert_eq!(almanac.seed_to_soil(14), 14);
        assert_eq!(almanac.seed_to_soil(55), 57);
        assert_eq!(almanac.seed_to_soil(13), 13);

        assert_eq!(almanac.seed_to_location(79), 82);
        assert_eq!(almanac.seed_to_location(55), 86);
        // assert_eq!(almanac.seed_to_location(14), 43);
        // assert_eq!(almanac.seed_to_location(13), 35);
    }

    #[test]

    fn test_parse_input_almanac() {
        let input = include_str!("../input/input-1.txt");
        let almanac = parse_almanac(input);
        assert!(almanac.seeds.len() > 0);
    }
}
