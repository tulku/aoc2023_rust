use regex::Regex;

#[derive(Debug)]
struct MapRange {
    destination_start: usize,
    source_start: usize,
    length: usize,
}

impl MapRange {
    fn new(destination_start: usize, source_start: usize, length: usize) -> Self {
        MapRange {
            destination_start,
            source_start,
            length,
        }
    }

    fn map(&self, input: usize) -> Option<usize> {
        if input < self.source_start || input > self.source_start + self.length - 1 {
            return None;
        }
        let offset = input - self.source_start;
        Some(self.destination_start + offset)
    }
}

#[derive(Debug)]
struct RangedMap {
    ranges: Vec<MapRange>,
}

impl RangedMap {
    fn new() -> Self {
        RangedMap { ranges: Vec::new() }
    }

    fn from_ranges(numbers: Vec<usize>) -> Self {
        let mut ranges = Vec::new();
        for map_set in numbers.chunks(3) {
            let destination_start = map_set[0];
            let source_start = map_set[1];
            let length = map_set[2];
            ranges.push(MapRange::new(destination_start, source_start, length));
        }
        RangedMap { ranges }
    }

    fn map(&self, input: usize) -> usize {
        for range in &self.ranges {
            if let Some(output) = range.map(input) {
                return output;
            }
        }
        input
    }
}

#[derive(Debug)]
pub struct Almanac {
    seeds: Vec<usize>,
    seeds_to_soil: RangedMap,
    soil_to_fertilizer: RangedMap,
    fertilizer_to_water: RangedMap,
    water_to_light: RangedMap,
    light_to_temperature: RangedMap,
    temperature_to_humidity: RangedMap,
    humidity_to_location: RangedMap,
}

impl Almanac {
    fn new() -> Self {
        Almanac {
            seeds: Vec::new(),
            seeds_to_soil: RangedMap::new(),
            soil_to_fertilizer: RangedMap::new(),
            fertilizer_to_water: RangedMap::new(),
            water_to_light: RangedMap::new(),
            light_to_temperature: RangedMap::new(),
            temperature_to_humidity: RangedMap::new(),
            humidity_to_location: RangedMap::new(),
        }
    }

    pub fn seed_to_soil(&self, seed: usize) -> usize {
        self.seeds_to_soil.map(seed)
    }

    pub fn soil_to_fertilizer(&self, soil: usize) -> usize {
        self.soil_to_fertilizer.map(soil)
    }

    pub fn fertilizer_to_water(&self, input: usize) -> usize {
        self.fertilizer_to_water.map(input)
    }

    pub fn water_to_light(&self, input: usize) -> usize {
        self.water_to_light.map(input)
    }

    pub fn light_to_temperature(&self, input: usize) -> usize {
        self.light_to_temperature.map(input)
    }

    pub fn temperature_to_humidity(&self, input: usize) -> usize {
        self.temperature_to_humidity.map(input)
    }

    pub fn humidity_to_location(&self, input: usize) -> usize {
        self.humidity_to_location.map(input)
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

    pub fn get_all_locations(&self) -> Vec<usize> {
        let mut locations = Vec::new();
        for seed in &self.seeds {
            locations.push(self.seed_to_location(*seed));
        }
        locations
    }

    pub fn get_min_location_ranged_seeds(&self) -> usize {
        let mut min_location = usize::MAX;
        for seed_range in self.seeds.chunks(2) {
            let start = seed_range[0];
            let length = seed_range[1];
            for seed in start..=start + length - 1 {
                let location = self.seed_to_location(seed);
                if location < min_location {
                    min_location = location;
                }
            }
        }
        min_location
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

fn build_map(numbers: Vec<usize>) -> RangedMap {
    RangedMap::from_ranges(numbers)
}

pub fn parse_almanac(almanac: &str) -> Almanac {
    let re = Regex::new(r"(?P<label>[\w|-]+(\smap)?)?:\s(?P<numbers>(\d+\s?)*)").unwrap();
    let mut parsed_almanac = Almanac::new();
    for captures in re.captures_iter(almanac) {
        let label = captures.name("label").unwrap().as_str();
        let numbers = str::replace(captures.name("numbers").unwrap().as_str(), "\n", " ");
        let numbers = get_numbers(numbers.as_str());
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
        assert_eq!(almanac.seed_to_location(14), 43);
        assert_eq!(almanac.seed_to_location(13), 35);

        let locations = almanac.get_all_locations();
        assert_eq!(locations.iter().min(), Some(&35));

        let location = almanac.get_min_location_ranged_seeds();
        assert_eq!(location, 46);
    }

    #[test]

    fn test_parse_input_almanac() {
        let input = include_str!("../input/input-1.txt");
        let almanac = parse_almanac(input);
        assert!(almanac.seeds.len() > 0);
    }
}
