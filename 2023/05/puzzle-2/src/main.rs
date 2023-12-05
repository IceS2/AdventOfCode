use std::{fs, ops::Range};
use rangemap::RangeMap;
use regex::Regex;

fn main() {
    let contents: String = fs::read_to_string("input.txt").unwrap();
    let almanac: Almanac = contents.into();

    almanac.find_nearest_location();
}

fn input_string_to_almanac_reversed_map(s: &str) -> RangeMap<i64, Range<i64>> {
    let mut map: RangeMap<i64, Range<i64>> = RangeMap::new();

    let contents: Vec<String> = s.split('\n').map(|c| c.to_string()).collect();

    for item in contents[1..].iter() {
        if item.is_empty() {
            continue;
        }
        let split: Vec<String> = item.split(' ').map(|c| c.to_string()).collect();
        let destination_range_start: i64 = split[0].parse().unwrap();
        let source_range_start: i64 = split[1].parse().unwrap();
        let range: i64 = split[2].parse().unwrap();
        map.insert(destination_range_start..destination_range_start+range, source_range_start..source_range_start+range);
    }
    map
}

#[derive(Debug)]
struct Almanac {
    seed_ranges: RangeMap<i64, bool>,

    soil_to_seed: RangeMap<i64, Range<i64>>,
    fertilizer_to_soil: RangeMap<i64, Range<i64>>,
    water_to_fertilizer: RangeMap<i64, Range<i64>>,
    light_to_water: RangeMap<i64, Range<i64>>,
    temperature_to_light: RangeMap<i64, Range<i64>>,
    humidity_to_temperature: RangeMap<i64, Range<i64>>,
    location_to_humidity: RangeMap<i64, Range<i64>>
}

impl From<String> for Almanac {
    fn from(s: String) -> Self {
        let contents: Vec<String> = s.split("\n\n").map(|c| c.to_string()).collect();

        // Seeds
        let mut seed_ranges: RangeMap<i64, bool> = RangeMap::new();
        let re = Regex::new(r"[0-9]+ [0-9]+").unwrap();
        let captures = re.captures_iter(&contents[0]);
        for capture in captures {
            let range: Vec<i64> = capture[0].split(' ').map(|n| n.parse().unwrap()).collect();
            seed_ranges.insert(range[0]..(range[0] + range[1]), true);
        }

        // Maps
        let soil_to_seed = input_string_to_almanac_reversed_map(&contents[1]);
        let fertilizer_to_soil = input_string_to_almanac_reversed_map(&contents[2]);
        let water_to_fertilizer = input_string_to_almanac_reversed_map(&contents[3]);
        let light_to_water = input_string_to_almanac_reversed_map(&contents[4]);
        let temperature_to_light = input_string_to_almanac_reversed_map(&contents[5]);
        let humidity_to_temperature = input_string_to_almanac_reversed_map(&contents[6]);
        let location_to_humidity = input_string_to_almanac_reversed_map(&contents[7]);

        Self {
            seed_ranges,

            soil_to_seed,
            fertilizer_to_soil,
            water_to_fertilizer,
            light_to_water,
            temperature_to_light,
            humidity_to_temperature,
            location_to_humidity
        }
    }
}

impl Almanac {
    fn resolve_with_map(entry: &i64, map: &RangeMap<i64, Range<i64>>) -> i64 {
        match map.get_key_value(entry) {
            None => *entry,
            Some((range, value)) => value.start + (entry - range.start)
        }
    }

    fn find_nearest_location(&self) {
        let mut final_location = 0;
        for (location_range, humidity_range) in self.location_to_humidity.iter() {
            if final_location > 0 {
                break;
            }
            for humidity in humidity_range.clone() {
                let temperature = Almanac::resolve_with_map(&humidity, &self.humidity_to_temperature);
                let light = Almanac::resolve_with_map(&temperature, &self.temperature_to_light);
                let water = Almanac::resolve_with_map(&light, &self.light_to_water);
                let fertilizer = Almanac::resolve_with_map(&water, &self.water_to_fertilizer);
                let soil = Almanac::resolve_with_map(&fertilizer, &self.fertilizer_to_soil);
                let seed = Almanac::resolve_with_map(&soil, &self.soil_to_seed);
                if self.seed_ranges.get_key_value(&seed).is_some() {
                    final_location = location_range.start + (humidity - humidity_range.start);
                    break;
                }
            }
        }
    }
}
