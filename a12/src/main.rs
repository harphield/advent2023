use std::cmp::max;
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::ops::Add;

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let mut sum = 0;
    let mut sum2 = 0;
    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                let split: Vec<_> = line.split(' ').collect();

                let bad_counts: Vec<_> = split[1].split(',').collect();
                let bad_counts = bad_counts
                    .iter()
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();

                let first = count_matching_combinations(&bad_counts, split[0].len(), &split[0].to_string());
                sum += first;

                // Part 2: unfolding
                // how to choose, if we add the "?" in front or in the back?

                let simplified = split[0].to_string();

                let mut unfolded_front = "?".to_string().add(&simplified);
                let mut unfolded_back = &simplified.add("?");

                let others_front =
                    count_matching_combinations(&bad_counts, unfolded_front.len(), &unfolded_front.to_string());
                let others_back =
                    count_matching_combinations(&bad_counts, unfolded_back.len(), &unfolded_back.to_string());

                let result = first * others_back.pow(4);
                println!("{} = {}", split[0], result);

                sum2 += result;
            }
            Err(_) => break,
        }
    }

    println!("Part 1 result: {}", sum);
    println!("Part 2 result: {}", sum2);

    Ok(())
}

fn encode_to_broken_counts(line: &str) -> Option<Vec<u32>> {
    let mut result = vec![];

    let mut group_is_ok = true;
    let mut count = 0;
    for (i, c) in line.chars().enumerate() {
        if c == '?' {
            return None;
        }

        if c == '.' {
            if i != 0 && !group_is_ok {
                // change
                result.push(count);
                count = 0;
            }

            group_is_ok = true;
        } else if c == '#' {
            if i != 0 && group_is_ok {
                // change
                count = 0;
            }

            group_is_ok = false;
        }

        count += 1;
    }

    if !group_is_ok {
        // final push
        result.push(count);
    }

    Some(result)
}

fn count_matching_combinations(blocks: &[u32], length: usize, pattern: &String) -> u32 {
    let re_q = Regex::new("[?]+").unwrap();
    let mut questionmarks: Vec<String> = re_q
        .find_iter(pattern)
        .map(|m| m.as_str().to_string())
        .collect();
    questionmarks.sort_by_key(|a| a.len());
    questionmarks.reverse();

    let mut regex_pattern = pattern.clone().replace('.', "\\.");
    for q in questionmarks {
        let replace = format!("([#\\.]{{{}}})", q.len());
        regex_pattern = regex_pattern.replace(q.as_str(), replace.as_str());
    }

    let re_p = Regex::new(&regex_pattern).unwrap();

    let min_operational = blocks.len() - 1; // at least 1 operational between broken ones

    if blocks.iter().sum::<u32>() > (length - min_operational) as u32 {
        panic!("can't fit");
    }

    let shortest = blocks
        .iter()
        .map(|c| vec!["#"; *c as usize].join(""))
        .collect::<Vec<String>>()
        .join(".");

    if shortest.len() == length {
        // this is the only possibility
        return 1;
    }

    // try to add dots to fill length
    let missing = length - shortest.len();

    // we can add dots at the beginning
    // we can add dots at the end
    // we can add dots to every "hole" in the shortest string
    let holes_count = blocks.len() + 1; // before + after + between

    // if we are missing 4 dots and 3 blocks, we can distribute them:
    // 4 before, 0 between block 1 and 2, 0 between blocks 2 and 3, 0 after
    // 3 before, 1 between block 1 and 2, 0 between blocks 2 and 3, 0 after
    // 3 before, 0 between block 1 and 2, 1 between blocks 2 and 3, 0 after
    // 3 before, 0 between block 1 and 2, 0 between blocks 2 and 3, 1 after
    // ...

    let mut current = vec![];
    fill_and_count(missing, holes_count, &mut current, blocks, &re_p, pattern)
}

fn fill_and_count(
    missing: usize,
    hole_count: usize,
    current: &mut Vec<usize>,
    blocks: &[u32],
    pattern_regex: &Regex,
    pattern: &String
) -> u32 {
    if hole_count == 1 {
        current.push(missing);

        // and check stuff
        let s: Vec<String> = current
            .iter()
            .enumerate()
            .map(|(hole_index, fill_size)| {
                if hole_index == 0 {
                    vec!["."; *fill_size]
                        .join("")
                        .add(vec!["#"; blocks[hole_index] as usize].join("").as_str())
                } else if hole_index == blocks.len() {
                    vec!["."; *fill_size].join("")
                } else {
                    vec!["."; *fill_size]
                        .join("")
                        .add(".")
                        .add(vec!["#"; blocks[hole_index] as usize].join("").as_str())
                }
            })
            .collect();
        let s = s.join("");

        current.pop();

        if pattern_regex.is_match(&s)
            && match encode_to_broken_counts(&s) {
                None => false,
                Some(bc) => bc.eq(blocks),
            }
        {
            return 1;
        }

        return 0;
    }

    let mut result = 0;

    for amount in 0..=missing {
        if !prune(pattern, blocks, hole_count, amount) {
            current.push(amount);
            result += fill_and_count(missing - amount, hole_count - 1, current, blocks, pattern_regex, pattern);
            current.pop();
        }
    }

    result
}

fn prune(pattern: &String, blocks: &[u32], hole_index: usize, amount: usize) -> bool {
    // if we would add {amount} dots in {hole_index}, would it have problems with the pattern?
    if hole_index == 0 {
        // in front of block 0
        let block_0_offset = 0;
        let block_0_length = blocks[0] as usize;
        for i in 0..=amount + block_0_length {
            // added {amount} dots before and at least 1 dot will be after
            if i < amount || i == amount + block_0_length {
                // looking for a .
                let c = &pattern[block_0_offset + i..block_0_offset + i + 1];
                if c != "." && c != "?" {
                    return true;
                }
            } else {
                // looking for a #
                let c = &pattern[block_0_offset + i..block_0_offset + i + 1];
                if c != "#" && c != "?" {
                    return true;
                }
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::{count_matching_combinations, encode_to_broken_counts, prune};

    #[test]
    fn test_encode() {
        let result = encode_to_broken_counts(".#.###.#.######").unwrap();

        assert!(result.eq(&vec![1, 3, 1, 6]));
    }

    #[test]
    fn test_counting_results() {
        let result = count_matching_combinations(
            &vec![4, 1, 1],
            "?????.#...#...".len(),
            &"?????.#...#...".to_string(),
        );

        assert_eq!(result, 2);
    }

    #[test]
    fn test_counting_results_2() {
        let result = count_matching_combinations(
            &vec![1, 1, 3],
            ".??..??...?##.".len(),
            &".??..??...?##.".to_string(),
        );

        assert_eq!(result, 4);

        let result = count_matching_combinations(
            &vec![1, 1, 3],
            "?.??..??...?##.".len(),
            &"?.??..??...?##.".to_string(),
        );

        assert_eq!(result, 8);
    }

    #[test]
    fn test_counting_results_larger() {
        let result = count_matching_combinations(
            &vec![4, 1, 1, 4, 1, 1, 4, 1, 1, 4, 1, 1, 4, 1, 1],
            "????.#...#...?????.#...#...?????.#...#...?????.#...#...?????.#...#...".len(),
            &"????.#...#...?????.#...#...?????.#...#...?????.#...#...?????.#...#...".to_string(),
        );

        // 1 * 2 * 2 * 2 * 2
        assert_eq!(result, 16);
    }

    #[test]
    fn test_prune() {
        assert!(prune(&"?###????????".to_string(), &vec![3, 2, 1], 0, 0));
        assert!(!prune(&"?###????????".to_string(), &vec![3, 2, 1], 0, 1));
        assert!(prune(&"?###????????".to_string(), &vec![3, 2, 1], 0, 2));
    }
}
