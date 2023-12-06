use itertools::Itertools;

#[derive(Debug)]
struct RangeMap {
    source_range_start: u64,
    dest_range_start: u64,
    range_length: u64,
}

impl RangeMap {
    fn map(&self, src_val: u64) -> Option<u64> {
        if (self.source_range_start..(self.source_range_start + self.range_length))
            .contains(&src_val)
        {
            let offset = src_val - self.source_range_start;
            Some(self.dest_range_start + offset)
        } else {
            None
        }
    }
    fn map_reverse(&self, dst_val: u64) -> Option<u64> {
        let dst_range = self.dest_range_start..(self.dest_range_start + self.range_length);
        if dst_range.contains(&dst_val) {
            let offset = dst_val - self.dest_range_start;
            Some(self.source_range_start + offset)
        } else {
            None
        }
    }
    fn from_str(line: &str) -> RangeMap {
        let mut iter = line
            .split_whitespace()
            .filter_map(|substring| substring.parse().ok());

        let dest_range_start: u64 = iter.next().unwrap();
        let source_range_start: u64 = iter.next().unwrap();
        let range_length: u64 = iter.next().unwrap();
        RangeMap {
            source_range_start,
            dest_range_start,
            range_length,
        }
    }
}

struct Map {
    maps: Vec<RangeMap>,
}

impl Map {
    fn new() -> Map {
        Map { maps: Vec::new() }
    }
    fn map(&self, src: u64) -> u64 {
        let mut dst = src;
        for map in self.maps.iter() {
            if let Some(val) = map.map(dst) {
                dst = val;
                break;
            }
        }
        dst
    }
    fn map_reverse(&self, dst: u64) -> u64 {
        let mut src = dst;
        for map in self.maps.iter() {
            if let Some(val) = map.map_reverse(src) {
                src = val;
                break;
            }
        }
        src
    }
}

fn part_a(input: &str) -> u64 {
    let input_split: Vec<&str> = input.split("\r\n\r\n").collect();
    let mut seeds: Vec<u64> = Vec::new();
    let mut seed_to_soil = Map::new();
    let mut soil_to_fertilizer = Map::new();
    let mut fertilizer_to_water = Map::new();
    let mut water_to_light = Map::new();
    let mut light_to_temperature = Map::new();
    let mut temperature_to_humidity = Map::new();
    let mut humidity_to_location = Map::new();
    for (index, split) in input_split.into_iter().enumerate() {
        match index {
            0 => {
                // Seeds list
                split
                    .split_once(":")
                    .unwrap()
                    .1
                    .split_whitespace()
                    .filter_map(|substring| substring.parse().ok())
                    .for_each(|x| seeds.push(x));
            }
            1 => {
                // seed-to-soil map
                seed_to_soil.maps = split
                    .strip_prefix("seed-to-soil map:\r\n")
                    .unwrap()
                    .split("\r\n")
                    .map(|line| RangeMap::from_str(line))
                    .collect::<Vec<RangeMap>>();
            }
            2 => {
                // soil-to-fertilizer map
                soil_to_fertilizer.maps = split
                    .strip_prefix("soil-to-fertilizer map:\r\n")
                    .unwrap()
                    .split("\r\n")
                    .map(|line| RangeMap::from_str(line))
                    .collect::<Vec<RangeMap>>();
            }
            3 => {
                // fertilizer-to-water map
                fertilizer_to_water.maps = split
                    .strip_prefix("fertilizer-to-water map:\r\n")
                    .unwrap()
                    .split("\r\n")
                    .map(|line| RangeMap::from_str(line))
                    .collect::<Vec<RangeMap>>();
            }
            4 => {
                // water-to-light map
                water_to_light.maps = split
                    .strip_prefix("water-to-light map:\r\n")
                    .unwrap()
                    .split("\r\n")
                    .map(|line| RangeMap::from_str(line))
                    .collect::<Vec<RangeMap>>();
            }
            5 => {
                // light-to-temperature map
                light_to_temperature.maps = split
                    .strip_prefix("light-to-temperature map:\r\n")
                    .unwrap()
                    .split("\r\n")
                    .map(|line| RangeMap::from_str(line))
                    .collect::<Vec<RangeMap>>();
            }
            6 => {
                // temperature-to-humidity map
                temperature_to_humidity.maps = split
                    .strip_prefix("temperature-to-humidity map:\r\n")
                    .unwrap()
                    .split("\r\n")
                    .map(|line| RangeMap::from_str(line))
                    .collect::<Vec<RangeMap>>();
            }
            7 => {
                // humidity-to-location map
                humidity_to_location.maps = split
                    .strip_prefix("humidity-to-location map:\r\n")
                    .unwrap()
                    .split("\r\n")
                    .map(|line| RangeMap::from_str(line))
                    .collect::<Vec<RangeMap>>();
            }
            _ => panic!("There should only be 8 sections"),
        }
    }
    seeds
        .iter()
        .map(|seed| seed_to_soil.map(*seed))
        .map(|seed| soil_to_fertilizer.map(seed))
        .map(|seed| fertilizer_to_water.map(seed))
        .map(|seed| water_to_light.map(seed))
        .map(|seed| light_to_temperature.map(seed))
        .map(|seed| temperature_to_humidity.map(seed))
        .map(|seed| humidity_to_location.map(seed))
        .min()
        .unwrap() as u64
}

fn part_b(input: &str) -> u64 {
    let input_split: Vec<&str> = input.split("\r\n\r\n").collect();
    let mut seeds: Vec<std::ops::Range<u64>> = Vec::new();
    let mut seed_to_soil = Map::new();
    let mut soil_to_fertilizer = Map::new();
    let mut fertilizer_to_water = Map::new();
    let mut water_to_light = Map::new();
    let mut light_to_temperature = Map::new();
    let mut temperature_to_humidity = Map::new();
    let mut humidity_to_location = Map::new();
    for (index, split) in input_split.into_iter().enumerate() {
        match index {
            0 => {
                // Seeds list
                split
                    .split_once(":")
                    .unwrap()
                    .1
                    .split_whitespace()
                    .filter_map(|substring| substring.parse().ok())
                    .collect::<Vec<u64>>()
                    .into_iter()
                    .chunks(2)
                    .into_iter()
                    .for_each(|mut chunk| {
                        let start = chunk.next().unwrap();
                        let len = chunk.next().unwrap();
                        seeds.push(start..(start + len));
                    });
            }
            1 => {
                // seed-to-soil map
                seed_to_soil.maps = split
                    .strip_prefix("seed-to-soil map:\r\n")
                    .unwrap()
                    .split("\r\n")
                    .map(|line| RangeMap::from_str(line))
                    .collect::<Vec<RangeMap>>();
            }
            2 => {
                // soil-to-fertilizer map
                soil_to_fertilizer.maps = split
                    .strip_prefix("soil-to-fertilizer map:\r\n")
                    .unwrap()
                    .split("\r\n")
                    .map(|line| RangeMap::from_str(line))
                    .collect::<Vec<RangeMap>>();
            }
            3 => {
                // fertilizer-to-water map
                fertilizer_to_water.maps = split
                    .strip_prefix("fertilizer-to-water map:\r\n")
                    .unwrap()
                    .split("\r\n")
                    .map(|line| RangeMap::from_str(line))
                    .collect::<Vec<RangeMap>>();
            }
            4 => {
                // water-to-light map
                water_to_light.maps = split
                    .strip_prefix("water-to-light map:\r\n")
                    .unwrap()
                    .split("\r\n")
                    .map(|line| RangeMap::from_str(line))
                    .collect::<Vec<RangeMap>>();
            }
            5 => {
                // light-to-temperature map
                light_to_temperature.maps = split
                    .strip_prefix("light-to-temperature map:\r\n")
                    .unwrap()
                    .split("\r\n")
                    .map(|line| RangeMap::from_str(line))
                    .collect::<Vec<RangeMap>>();
            }
            6 => {
                // temperature-to-humidity map
                temperature_to_humidity.maps = split
                    .strip_prefix("temperature-to-humidity map:\r\n")
                    .unwrap()
                    .split("\r\n")
                    .map(|line| RangeMap::from_str(line))
                    .collect::<Vec<RangeMap>>();
            }
            7 => {
                // humidity-to-location map
                humidity_to_location.maps = split
                    .strip_prefix("humidity-to-location map:\r\n")
                    .unwrap()
                    .split("\r\n")
                    .map(|line| RangeMap::from_str(line))
                    .collect::<Vec<RangeMap>>();
            }
            _ => panic!("There should only be 8 sections"),
        }
    }
    let mut location = 0;
    let mut humidity = humidity_to_location.map_reverse(location);
    let mut temperature = temperature_to_humidity.map_reverse(humidity);
    let mut light = light_to_temperature.map_reverse(temperature);
    let mut water = water_to_light.map_reverse(light);
    let mut fertilizer = fertilizer_to_water.map_reverse(water);
    let mut soil = soil_to_fertilizer.map_reverse(fertilizer);
    let mut seed = seed_to_soil.map_reverse(soil);
    while !seeds.iter().any(|r| r.contains(&seed)) {
        location += 1;
        humidity = humidity_to_location.map_reverse(location);
        temperature = temperature_to_humidity.map_reverse(humidity);
        light = light_to_temperature.map_reverse(temperature);
        water = water_to_light.map_reverse(light);
        fertilizer = fertilizer_to_water.map_reverse(water);
        soil = soil_to_fertilizer.map_reverse(fertilizer);
        seed = seed_to_soil.map_reverse(soil);
    }
    location
}

pub use crate::boilerplate;

boilerplate!(5, 35, 46, u64);
