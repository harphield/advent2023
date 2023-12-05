use std::cmp::min;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;

type CatMap = HashMap<u64, Vec<(u64, u64, u64)>>;

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let mut seeds: Vec<u64> = vec![];
    let mut maps = CatMap::new();

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

    let mut new_maps = CatMap::new();
    maps.iter().for_each(|(key, value)| {
        let mut map = value.clone();
        // sort the maps
        map.sort_by(|a, b| {
            a.1.cmp(&b.1)
        });

        let mut new_map = vec![];

        // filling gaps
        for (i, rule) in map.iter().enumerate() {
            new_map.push(*rule);
            match map.get(i + 1) {
                None => {}
                Some(next_rule) => {
                    if rule.1 + rule.2 != next_rule.1 {
                        // got a gap
                        // fill gap
                        new_map.push((rule.1 + rule.2, rule.1 + rule.2, next_rule.1 - (rule.1 + rule.2)));
                    }
                }
            }
        }

        new_maps.insert(*key, new_map);
    });

    maps = new_maps;

    // println!("{:?}", seeds);
    // println!("{:?}", maps);

    let mut lowest: u64 = u64::MAX;

    for seed in seeds.iter() {
        lowest = min(lowest, cat_to_cat(1, 8, *seed, &maps));
    }

    println!("Result part 1: {}", lowest);

    let mut lowest: u64 = u64::MAX;
    // part 2 are ranges!

    let mut ranges = vec![];

    for range in seeds.chunks(2) {
        ranges.push((range[0], range[1]));
    }

    ranges.sort_by(|a, b| {
        a.0.cmp(&b.0)
    });

    println!("{:?}", ranges);

    for range in ranges.iter() {
        lowest = min(lowest, find_lowest_by_ranges(1, range, &maps));
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

fn find_lowest_by_ranges(source_cat: u64, source_range: &(u64, u64), maps: &CatMap) -> u64 {
    if source_cat == 8 {
        return source_range.0;
    }

    let map = match maps.get(&source_cat) {
        None => {
            return source_range.0;
        }
        Some(m) => m
    };

    let mut lowest = u64::MAX;

    let mut start = source_range.0;
    let mut remaining = source_range.1;

    if start < map[0].1 {
        // new range before mappings
        lowest = min(lowest, find_lowest_by_ranges(source_cat + 1, &(start, map[0].1 - start), &maps));

        if remaining < map[0].1 - start {
            return lowest;
        }

        remaining -= map[0].1 - start;
        start = map[0].1;
    }

    map.iter().for_each(|rule| {
        if remaining == 0 || (start > rule.1 && start >= rule.1 + rule.2) {
            return;
        }

        if start < rule.1 {
            panic!("start < rule.1 {} {}", start, rule.1);
        }

        if remaining > rule.2 {
            // we need to continue after this
            lowest = min(lowest, find_lowest_by_ranges(source_cat + 1, &(rule.0 + (start - rule.1), rule.2 - (start - rule.1)), &maps));

            start += rule.2 - (start - rule.1);
            remaining -= rule.2 - (start - rule.1);
        } else {
            // we end here
            lowest = min(lowest, find_lowest_by_ranges(source_cat + 1, &(rule.0 + (start - rule.1), min(remaining, rule.2 - (start - rule.1))), &maps));
            remaining = 0;
        }
    });

    if remaining > 0 {
        lowest = min(lowest, find_lowest_by_ranges(source_cat + 1, &(start, remaining), &maps));
    }

    lowest
}

#[cfg(test)]
mod tests {
    use crate::{cat_to_cat, cat_to_next, CatMap, find_lowest_by_ranges};

    #[test]
    fn test_cat_1_to_next() {
        let mut map = CatMap::new();
        map.insert(1, vec![
            (20, 1, 5)
        ]);

        let result = cat_to_next(1, 2, &map);
        assert_eq!(result, 21);
    }

    #[test]
    fn test_cat_1_to_3() {
        let mut map = CatMap::new();
        map.insert(1, vec![
            (20, 1, 5)
        ]);

        map.insert(2, vec![
            (60, 10, 20),
        ]);

        let result = cat_to_cat(1, 2, 2, &map);
        assert_eq!(result, 71);
    }

    #[test]
    fn test_find_lowest_with_ranges_01() {
        let mut map = CatMap::new();
        map.insert(1, vec![
            (10, 1, 10),
            (50, 11, 10),
        ]);

        assert_eq!(find_lowest_by_ranges(1, &(1, 10), &map), 10);
    }

    #[test]
    fn test_find_lowest_with_ranges_02() {
        let mut map = CatMap::new();
        map.insert(1, vec![
            (10, 5, 10),
            (50, 15, 10),
        ]);
        map.insert(2, vec![
            (5, 1, 5)
        ]);

        assert_eq!(find_lowest_by_ranges(1, &(1, 10), &map), 5);
    }

    #[test]
    fn test_find_lowest_with_ranges_02() {
        let mut map = CatMap::new();
        map.insert(1, vec![
            (10, 5, 10),
            (50, 15, 10),
        ]);
        map.insert(2, vec![
            (5, 1, 5)
        ]);

        assert_eq!(find_lowest_by_ranges(1, &(1, 10), &map), 5);
    }
}