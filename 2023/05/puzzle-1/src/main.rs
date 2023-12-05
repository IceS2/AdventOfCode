use std::fs;
use rangemap::RangeMap;

fn main() {
    let contents: String = fs::read_to_string("input.txt").unwrap();
    let almanac: Almanac = contents.into();

    println!("{:?}", almanac.seeds_to_location().iter().min().unwrap());
}

fn input_string_to_almanac_map(s: &str) -> RangeMap<i64, i64> {
    let mut map: RangeMap<i64, i64> = RangeMap::new();

    let contents: Vec<String> = s.split('\n').map(|c| c.to_string()).collect();

    for item in contents[1..].iter() {
        if item.is_empty() {
            continue;
        }
        let split: Vec<String> = item.split(' ').map(|c| c.to_string()).collect();
        let destination_range_start: i64 = split[0].parse().unwrap();
        let source_range_start: i64 = split[1].parse().unwrap();
        let range: i64 = split[2].parse().unwrap();
        map.insert(source_range_start..source_range_start+range, destination_range_start);
    }
    map
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<i64>,
    seed_to_soil: RangeMap<i64, i64>,
    soil_to_fertilizer: RangeMap<i64, i64>,
    fertilizer_to_water: RangeMap<i64, i64>,
    water_to_light: RangeMap<i64, i64>,
    light_to_temperature: RangeMap<i64, i64>,
    temperature_to_humidity: RangeMap<i64, i64>,
    humidity_to_location: RangeMap<i64, i64>
}

impl From<String> for Almanac {
    fn from(s: String) -> Self {
        let mut contents: Vec<String> = s.split("\n\n").map(|c| c.to_string()).collect();
        contents[0].replace_range(0..7, "");

        // Seeds
        let seeds: Vec<i64> = contents[0].split(' ').map(|n| n.parse().unwrap()).collect();

        // Maps
        let seed_to_soil = input_string_to_almanac_map(&contents[1]);
        let soil_to_fertilizer = input_string_to_almanac_map(&contents[2]);
        let fertilizer_to_water = input_string_to_almanac_map(&contents[3]);
        let water_to_light = input_string_to_almanac_map(&contents[4]);
        let light_to_temperature = input_string_to_almanac_map(&contents[5]);
        let temperature_to_humidity = input_string_to_almanac_map(&contents[6]);
        let humidity_to_location = input_string_to_almanac_map(&contents[7]);

        Self {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location
        }
    }
}

impl Almanac {
    fn resolve_with_map(entry: &i64, map: &RangeMap<i64, i64>) -> i64 {
        match map.get_key_value(entry) {
            None => *entry,
            Some((range, value)) => value + (entry - range.start)
        }
    }

    fn seeds_to_location(&self) -> Vec<i64> {
        self.seeds.iter()
            .map(|seed| Almanac::resolve_with_map(seed, &self.seed_to_soil))
            .map(|soil| Almanac::resolve_with_map(&soil, &self.soil_to_fertilizer))
            .map(|fertilizer| Almanac::resolve_with_map(&fertilizer, &self.fertilizer_to_water))
            .map(|water| Almanac::resolve_with_map(&water, &self.water_to_light))
            .map(|light| Almanac::resolve_with_map(&light, &self.light_to_temperature))
            .map(|temperature| Almanac::resolve_with_map(&temperature, &self.temperature_to_humidity))
            .map(|humidity| Almanac::resolve_with_map(&humidity, &self.humidity_to_location))
            .collect()
    }
}
