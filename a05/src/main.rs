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

    // sort the maps
    let mut new_maps = CatMap::new();
    maps.iter().for_each(|(key, value)| {
        let mut value = value.clone();
        value.sort_by(|a, b| {
            a.1.cmp(&b.1)
        });

        new_maps.insert(*key, value);
    });

    maps = new_maps;

    println!("{:?}", seeds);
    println!("{:?}", maps);

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

    // split the range into smaller ranges that are either
    // - mapped in one of the mappings = use the target value
    // - not mapped = use the same value

    // range: 10-50 (10;40)
    // map: 15-20 (15;5), 30-40 (30;10)
    // splits: 10-14 (not mapped), 15-20 (mapped), 21-29 (not mapped), 30-40 (mapped), 41-50 (not mapped)

    let mut index = source_range.0;
    let mut remaining_length = source_range.1;
    let mut lowest = u64::MAX;
    maps.get(&source_cat).unwrap().iter().for_each(|(mapping_dest_range_start, mapping_source_range_start, mapping_range_length)| {

        if &index < mapping_source_range_start {
            // what if my range starts before the mapped range starts?

            let length = min(mapping_source_range_start - index, source_range.1);
            // not mapped subrange
            let subrange = (index, length);
            lowest = min(lowest, find_lowest_by_ranges(source_cat + 1, &subrange, &maps));

            index += length;
            remaining_length -= length;

            // no need to do more
            if remaining_length == 0 {
                return;
            }
        }

        // what if my range starts after the mapped range?
        let subrange = (index, remaining_length);
        lowest = min(lowest, find_lowest_by_ranges(source_cat + 1, &subrange, &maps));
    });

    lowest
}

#[cfg(test)]
mod tests {
    use crate::{cat_to_cat, cat_to_next, CatMap};

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
    fn test_find_lowest_with_ranges() {
        let mut map = CatMap::new();
        map.insert(1, vec![
            (10, 1, 10)
        ]);

        map.insert(2, vec![
            (20, 10, 5),
        ]);
    }
}