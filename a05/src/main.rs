use std::cmp::min;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;

const SEED_TO_SOIL: u64 = 1;
const SOIL_TO_FERTILIZER: u64 = 2;
const FERTILIZER_TO_WATER: u64 = 3;
const WATER_TO_LIGHT: u64 = 4;
const LIGHT_TO_TEMPERATURE: u64 = 5;
const TEMPERATURE_TO_HUMIDITY: u64 = 6;
const HUMIDITY_TO_LOCATION: u64 = 7;

type CatMap = HashMap<u64, Vec<(u64, u64, u64)>>;

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let mut seeds: Vec<u64> = vec![];
    let mut maps: CatMap = HashMap::new();

    let re_numbers = Regex::new("([0-9]+)").unwrap();

    let mut current_reading = 0;
    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                if line.is_empty() {
                    current_reading += 1;
                    continue;
                }

                let mapping = re_numbers
                    .find_iter(&line)
                    .map(|m| m.as_str().parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();

                if current_reading == 0 {
                    // seeds are a vector
                    seeds = mapping;
                } else if !mapping.is_empty() {
                    maps.entry(current_reading)
                        .or_insert(vec![])
                        .push((mapping[0], mapping[1], mapping[2]));
                }
            }
            Err(_) => break,
        }
    }

    println!("{:?}", seeds);
    println!("{:?}", maps);

    let mut lowest: u64 = u64::MAX;

    for seed in seeds.iter() {
        lowest = min(lowest, cat_to_cat(SEED_TO_SOIL, 8, *seed, &maps));
    }

    println!("Result part 1: {}", lowest);

    let mut lowest: u64 = u64::MAX;
    // part 2 are ranges!
    for range in seeds.chunks(2) {
        for i in range[0]..range[0] + range[1] {
            lowest = min(lowest, cat_to_cat(SEED_TO_SOIL, 8, i, &maps));
        }
    }

    println!("Result part 2: {}", lowest);

    Ok(())
}

fn cat_to_cat(source_cat: u64, destination_cat: u64, source_value: u64, maps: &CatMap) -> u64 {
    let mut current_value = source_value;
    for current_cat in source_cat..destination_cat {
        current_value = cat_to_next(current_cat, current_value, &maps);
    }

    current_value
}

fn cat_to_next(source_cat: u64, source_value: u64, maps: &CatMap) -> u64 {
    let mappings = maps.get(&source_cat).unwrap();

    let explicit = mappings.iter().filter(|(_dest_range_start, source_range_start, range_length)| {
        source_range_start <= &source_value && source_range_start + range_length > source_value
    }).collect::<Vec<&(u64, u64, u64)>>();

    if explicit.is_empty() {
        return source_value;
    }

    let found = explicit[0];

    found.0 + source_value - found.1
}

#[cfg(test)]
mod tests {
    use crate::{cat_to_cat, cat_to_next, CatMap, FERTILIZER_TO_WATER, SEED_TO_SOIL, SOIL_TO_FERTILIZER};

    #[test]
    fn test_cat_1_to_next() {
        let mut map = CatMap::new();
        map.insert(SEED_TO_SOIL, vec![
            (20, 1, 5)
        ]);

        let result = cat_to_next(SEED_TO_SOIL, 2, &map);
        assert_eq!(result, 21);
    }

    #[test]
    fn test_cat_1_to_3() {
        let mut map = CatMap::new();
        map.insert(SEED_TO_SOIL, vec![
            (20, 1, 5)
        ]);

        map.insert(SOIL_TO_FERTILIZER, vec![
            (60, 10, 20),
        ]);

        let result = cat_to_cat(SEED_TO_SOIL, FERTILIZER_TO_WATER, 2, &map);
        assert_eq!(result, 71);
    }
}