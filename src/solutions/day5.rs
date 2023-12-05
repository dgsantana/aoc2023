use std::{fmt::Display, ops::Range, str::Lines};

use super::Solution;

/// --- Day 5: If You Give A Seed A Fertilizer ---
/// You take the boat and find the gardener right where you were told he would
/// be: managing a giant "garden" that looks more to you like a farm.
///
/// "A water source? Island Island is the water source!" You point out that
/// Snow Island isn't receiving any water.
///
/// "Oh, we had to stop the water because we ran out of sand to filter it with!
/// Can't make snow with dirty water. Don't worry, I'm sure we'll get more sand
/// soon; we only turned off the water a few days... weeks... oh no." His face
/// sinks into a look of horrified realization.
///
/// "I've been so busy making sure everyone here has food that I completely
/// forgot to check why we stopped getting more sand! There's a ferry leaving
/// soon that is headed over in that direction - it's much faster than your
/// boat. Could you please go check it out?"
///
/// You barely have time to agree to this request when he brings up another.
/// "While you wait for the ferry, maybe you can help us with our food
/// production problem. The latest Island Island Almanac just arrived and
/// we're having trouble making sense of it."
///
/// The almanac (your puzzle input) lists all of the seeds that need to be
/// planted. It also lists what type of soil to use with each kind of seed,
/// what type of fertilizer to use with each kind of soil, what type of water
/// to use with each kind of fertilizer, and so on. Every type of seed, soil,
/// fertilizer and so on is identified with a number, but numbers are reused
/// by each category - that is, soil 123 and fertilizer 123 aren't necessarily
/// related to each other.
///
/// For example:
///
/// seeds: 79 14 55 13
///
/// seed-to-soil map:
/// 50 98 2
/// 52 50 48
///
/// soil-to-fertilizer map:
/// 0 15 37
/// 37 52 2
/// 39 0 15
///
/// fertilizer-to-water map:
/// 49 53 8
/// 0 11 42
/// 42 0 7
/// 57 7 4
///
/// water-to-light map:
/// 88 18 7
/// 18 25 70
///
/// light-to-temperature map:
/// 45 77 23
/// 81 45 19
/// 68 64 13
///
/// temperature-to-humidity map:
/// 0 69 1
/// 1 0 69
///
/// humidity-to-location map:
/// 60 56 37
/// 56 93 4
/// The almanac starts by listing which seeds need to be planted: seeds 79, 14, 55, and 13.
///
/// The rest of the almanac contains a list of maps which describe how to
/// convert numbers from a source category into numbers in a destination
/// category. That is, the section that starts with seed-to-soil map: describes
/// how to convert a seed number (the source) to a soil number (the destination).
/// This lets the gardener and his team know which soil to use with which seeds,
/// which water to use with which fertilizer, and so on.
///
/// Rather than list every source number and its corresponding destination number
/// one by one, the maps describe entire ranges of numbers that can be converted.
/// Each line within a map contains three numbers: the destination range start,
/// the source range start, and the range length.
///
/// Consider again the example seed-to-soil map:
///
/// 50 98 2
/// 52 50 48
/// The first line has a destination range start of 50, a source range start of
/// 98, and a range length of 2. This line means that the source range starts at
/// 98 and contains two values: 98 and 99. The destination range is the same length,
/// but it starts at 50, so its two values are 50 and 51. With this information, you
/// know that seed number 98 corresponds to soil number 50 and that seed number 99
/// corresponds to soil number 51.
///
/// The second line means that the source range starts at 50 and contains 48 values:
/// 50, 51, ..., 96, 97. This corresponds to a destination range starting at 52 and
/// also containing 48 values: 52, 53, ..., 98, 99. So, seed number 53 corresponds
/// to soil number 55.
///
/// Any source numbers that aren't mapped correspond to the same destination number.
/// So, seed number 10 corresponds to soil number 10.
///
/// So, the entire list of seed numbers and their corresponding soil numbers looks
/// like this:
///
/// seed  soil
/// 0     0
/// 1     1
/// ...   ...
/// 48    48
/// 49    49
/// 50    52
/// 51    53
/// ...   ...
/// 96    98
/// 97    99
/// 98    50
/// 99    51
/// With this map, you can look up the soil number required for each initial seed number:
///
/// Seed number 79 corresponds to soil number 81.
/// Seed number 14 corresponds to soil number 14.
/// Seed number 55 corresponds to soil number 57.
/// Seed number 13 corresponds to soil number 13.
/// The gardener and his team want to get started as soon as possible, so they'd like
/// to know the closest location that needs a seed. Using these maps, find the lowest
/// location number that corresponds to any of the initial seeds. To do this, you'll
/// need to convert each seed number through other categories until you can find its
/// corresponding location number. In this example, the corresponding types are:
///
/// Seed 79, soil 81, fertilizer 81, water 81, light 74, temperature 78, humidity 78, location 82.
/// Seed 14, soil 14, fertilizer 53, water 49, light 42, temperature 42, humidity 43, location 43.
/// Seed 55, soil 57, fertilizer 57, water 53, light 46, temperature 82, humidity 82, location 86.
/// Seed 13, soil 13, fertilizer 52, water 41, light 34, temperature 34, humidity 35, location 35.
/// So, the lowest location number in this example is 35.
///
/// What is the lowest location number that corresponds to any of the initial seed numbers?
fn part1(input: &str) -> u64 {
    let mut lines = input.lines();
    let mut seeds = Vec::new();
    let mut seed2soil = Vec::new();
    let mut soil2fertilizer = Vec::new();
    let mut fertilizer2water = Vec::new();
    let mut water2light = Vec::new();
    let mut light2temperature = Vec::new();
    let mut temperature2humidity = Vec::new();
    let mut humidity2locations = Vec::new();
    while let Some(line) = lines.next() {
        if line.starts_with("seeds:") {
            seeds = parse_seeds(line);
        }
        if line.starts_with("seed-to-soil map:") {
            seed2soil = parse_map(&mut lines);
        }
        if line.starts_with("soil-to-fertilizer map:") {
            soil2fertilizer = parse_map(&mut lines);
        }
        if line.starts_with("fertilizer-to-water map:") {
            fertilizer2water = parse_map(&mut lines);
        }
        if line.starts_with("water-to-light map:") {
            water2light = parse_map(&mut lines);
        }
        if line.starts_with("light-to-temperature map:") {
            light2temperature = parse_map(&mut lines);
        }
        if line.starts_with("temperature-to-humidity map:") {
            temperature2humidity = parse_map(&mut lines);
        }
        if line.starts_with("humidity-to-location map:") {
            humidity2locations = parse_map(&mut lines);
        }
    }

    for seed in seeds.iter_mut() {
        if let Some(soil) = seed2soil.iter().find(|s| s.contains(seed.seed)) {
            seed.soil = soil.map(seed.soil);
        }
        seed.fertilizer = soil2fertilizer
            .iter()
            .find(|f| f.contains(seed.soil))
            .map(|v| v.map(seed.soil))
            .unwrap_or(seed.soil);
        seed.water = fertilizer2water
            .iter()
            .find(|w| w.contains(seed.fertilizer))
            .map(|v| v.map(seed.fertilizer))
            .unwrap_or(seed.fertilizer);
        seed.light = water2light
            .iter()
            .find(|l| l.contains(seed.water))
            .map(|v| v.map(seed.water))
            .unwrap_or(seed.water);
        seed.temperature = light2temperature
            .iter()
            .find(|t| t.contains(seed.light))
            .map(|v| v.map(seed.light))
            .unwrap_or(seed.light);
        seed.humidity = temperature2humidity
            .iter()
            .find(|h| h.contains(seed.temperature))
            .map(|v| v.map(seed.temperature))
            .unwrap_or(seed.temperature);
        seed.location = humidity2locations
            .iter()
            .find(|l| l.contains(seed.humidity))
            .map(|v| v.map(seed.humidity))
            .unwrap_or(seed.humidity);
    }
    if cfg!(feature = "visualize") {
        println!("Seed to soil map:");
        for seed in seed2soil.iter() {
            println!("{}", seed);
        }
        println!();
        println!("Soil to fertilizer map:");
        for soil in soil2fertilizer.iter() {
            println!("{}", soil);
        }
        println!();
        println!("Fertilizer to water map:");
        for fertilizer in fertilizer2water.iter() {
            println!("{}", fertilizer);
        }
        println!();
        println!("Water to light map:");
        for water in water2light.iter() {
            println!("{}", water);
        }
        println!();
        println!("Light to temperature map:");
        for light in light2temperature.iter() {
            println!("{}", light);
        }
        println!();
        println!("Temperature to humidity map:");
        for temperature in temperature2humidity.iter() {
            println!("{}", temperature);
        }
        println!();
        println!("Humidity to location map:");
        for humidity in humidity2locations.iter() {
            println!("{}", humidity);
        }
        println!();
        println!("Seeds:");
        for seed in seeds.iter() {
            println!("{}", seed);
        }
    }

    seeds.iter().map(|s| s.location).min().unwrap()
}

/// --- Part Two ---
/// Everyone will starve if you only plant such a small number of
/// seeds. Re-reading the almanac, it looks like the seeds: line
/// actually describes ranges of seed numbers.
///
/// The values on the initial seeds: line come in pairs. Within
/// each pair, the first value is the start of the range and the
/// second value is the length of the range. So, in the first
/// line of the example above:
///
/// seeds: 79 14 55 13
/// This line describes two ranges of seed numbers to be planted
/// in the garden. The first range starts with seed number 79
/// and contains 14 values: 79, 80, ..., 91, 92. The second range
/// starts with seed number 55 and contains 13 values: 55, 56,
/// ..., 66, 67.
///
/// Now, rather than considering four seed numbers, you need to
/// consider a total of 27 seed numbers.
///
/// In the above example, the lowest location number can be
/// obtained from seed number 82, which corresponds to soil
/// 84, fertilizer 84, water 84, light 77, temperature 45,
/// humidity 46, and location 46. So, the lowest location
/// number is 46.
///
/// Consider all of the initial seed numbers listed in the
/// ranges on the first line of the almanac. What is the
/// lowest location number that corresponds to any of the
/// initial seed numbers?
fn part2(input: &str) -> u64 {
    let mut lines = input.lines();
    let mut seeds = Vec::new();
    let mut seed2soil = Vec::new();
    let mut soil2fertilizer = Vec::new();
    let mut fertilizer2water = Vec::new();
    let mut water2light = Vec::new();
    let mut light2temperature = Vec::new();
    let mut temperature2humidity = Vec::new();
    let mut humidity2locations = Vec::new();
    while let Some(line) = lines.next() {
        if line.starts_with("seeds:") {
            seeds = line
                .split_ascii_whitespace()
                .filter_map(|s| s.parse::<u64>().ok())
                .collect();
        }
        if line.starts_with("seed-to-soil map:") {
            seed2soil = parse_map(&mut lines);
        }
        if line.starts_with("soil-to-fertilizer map:") {
            soil2fertilizer = parse_map(&mut lines);
        }
        if line.starts_with("fertilizer-to-water map:") {
            fertilizer2water = parse_map(&mut lines);
        }
        if line.starts_with("water-to-light map:") {
            water2light = parse_map(&mut lines);
        }
        if line.starts_with("light-to-temperature map:") {
            light2temperature = parse_map(&mut lines);
        }
        if line.starts_with("temperature-to-humidity map:") {
            temperature2humidity = parse_map(&mut lines);
        }
        if line.starts_with("humidity-to-location map:") {
            humidity2locations = parse_map(&mut lines);
        }
    }

    let seeds_to_locations = |seed_range: &Range<u64>| {
        let seed_soils = source_to_target_ranges(seed_range.clone(), &seed2soil);
        let soil_fertilizers = seed_soils
            .iter()
            .flat_map(|s| source_to_target_ranges(s.start..s.end, &soil2fertilizer))
            .collect::<Vec<_>>();
        let fertilizer_waters = soil_fertilizers
            .iter()
            .flat_map(|s| source_to_target_ranges(s.start..s.end, &fertilizer2water))
            .collect::<Vec<_>>();
        let water_lights = fertilizer_waters
            .iter()
            .flat_map(|s| source_to_target_ranges(s.start..s.end, &water2light))
            .collect::<Vec<_>>();
        let light_temperatures = water_lights
            .iter()
            .flat_map(|s| source_to_target_ranges(s.start..s.end, &light2temperature))
            .collect::<Vec<_>>();
        let temperature_humidities = light_temperatures
            .iter()
            .flat_map(|s| source_to_target_ranges(s.start..s.end, &temperature2humidity))
            .collect::<Vec<_>>();
        let humidity_locations = temperature_humidities
            .iter()
            .flat_map(|s| source_to_target_ranges(s.start..s.end, &humidity2locations))
            .collect::<Vec<_>>();
        humidity_locations
    };

    let seed_ranges = seeds
        .chunks_exact(2)
        .map(|c| c[0]..(c[0] + c[1]))
        .collect::<Vec<_>>();

    let mut min_location = u64::MAX;
    for seed_range in seed_ranges.into_iter() {
        let seeds_to_locations = seeds_to_locations(&seed_range);
        let current_min = seeds_to_locations.into_iter().map(|r| r.start).min();
        if let Some(current_min) = current_min {
            if current_min < min_location {
                min_location = current_min;
            }
        }
    }

    min_location
}

#[derive(Copy, Clone, Debug)]
struct Seed {
    seed: u64,
    soil: u64,
    fertilizer: u64,
    water: u64,
    light: u64,
    temperature: u64,
    humidity: u64,
    location: u64,
}

#[derive(Copy, Clone, Debug)]
struct MapRange {
    destination: u64,
    source: u64,
    length: u64,
}

impl Seed {
    fn new(seed: u64) -> Self {
        Self {
            seed,
            soil: seed,
            fertilizer: seed,
            water: seed,
            light: seed,
            temperature: seed,
            humidity: seed,
            location: seed,
        }
    }
}

impl Display for Seed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Seed: {:010}, Soil: {:010}, Fertilizer: {:010}, Water: {:010}, Light: {:010}, Temperature: {:010}, Humidity: {:010}, Location: {:010}",
            self.seed, self.soil, self.fertilizer, self.water, self.light, self.temperature, self.humidity, self.location)
    }
}

impl MapRange {
    fn new(destination: u64, source: u64, length: u64) -> Self {
        Self {
            destination,
            source,
            length,
        }
    }

    fn contains(&self, value: u64) -> bool {
        value >= self.source && value < self.source + self.length
    }

    fn map(&self, value: u64) -> u64 {
        if self.contains(value) {
            self.destination + (value - self.source)
        } else {
            value
        }
    }

    fn source_range(&self) -> Range<u64> {
        self.source..self.source + self.length
    }

    fn destination_range(&self) -> Range<u64> {
        self.destination..self.destination + self.length
    }
}

impl Display for MapRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{:010}..{:010}] -> [{:010}..{:010}]",
            self.source,
            self.source + self.length,
            self.destination,
            self.destination + self.length
        )
    }
}

fn parse_seeds(input: &str) -> Vec<Seed> {
    let (_, seeds) = input.split_once("seeds:").unwrap();
    seeds
        .split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .map(Seed::new)
        .collect()
}

fn parse_map(lines: &mut Lines<'_>) -> Vec<MapRange> {
    let mut ranges = Vec::new();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let numbers = line
            .split_whitespace()
            .filter_map(|s| s.parse::<u64>().ok())
            .collect::<Vec<_>>();
        if numbers.len() != 3 {
            eprintln!("Invalid map line: {}", line);
            break;
        }
        ranges.push(MapRange::new(numbers[0], numbers[1], numbers[2]));
    }
    ranges.sort_by_key(|r| r.source);
    ranges
}

fn source_to_target_ranges(range: Range<u64>, source2targets: &[MapRange]) -> Vec<Range<u64>> {
    let mut ranges = Vec::new();
    for map in source2targets.iter() {
        let source_range = map.source_range();
        let intersect_start = source_range.start.max(range.start);
        let intersect_end = source_range.end.min(range.end);
        if intersect_start >= intersect_end {
            continue;
        }
        let target_range = map.destination_range();
        let target_start = target_range.start + (intersect_start - source_range.start);
        let target_end = target_start + (intersect_end - intersect_start);
        ranges.push(target_start..target_end);
        if range.start < intersect_start {
            let sub_range = range.start..intersect_start;
            ranges.extend(source_to_target_ranges(sub_range, source2targets));
        }
        if range.end > intersect_end {
            let sub_range: Range<u64> = intersect_end..range.end;
            ranges.extend(source_to_target_ranges(sub_range, source2targets));
        }
    }
    if ranges.is_empty() {
        ranges.push(range);
    } else {
        ranges = join_ranges(ranges);
    }
    ranges
}

fn join_ranges(mut ranges: Vec<Range<u64>>) -> Vec<Range<u64>> {
    ranges.sort_by_key(|r| r.start);
    let mut result = Vec::new();
    let mut current_range = ranges[0].clone();
    for range in ranges.into_iter().skip(1) {
        if range.start <= current_range.end {
            current_range.end = current_range.end.max(range.end);
        } else {
            result.push(current_range);
            current_range = range;
        }
    }
    result.push(current_range);
    result
}

pub struct Day5;

impl Solution for Day5 {
    fn solve_part1(input: &str) -> anyhow::Result<String> {
        Ok(part1(input).to_string())
    }

    fn solve_part2(input: &str) -> anyhow::Result<String> {
        Ok(part2(input).to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::read_sample_input;

    use super::*;

    #[test]
    fn test_part1() {
        let input = read_sample_input(5, 1);
        assert_eq!(part1(&input), 35);
    }

    #[test]
    fn test_part2() {
        let input = read_sample_input(5, 2);
        assert_eq!(part2(&input), 46);
    }
}
