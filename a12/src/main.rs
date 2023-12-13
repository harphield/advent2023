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

                sum +=
                    count_matching_combinations(&bad_counts, split[0].len(), &split[0].to_string());

                // Part 2: unfolding
                let mut unfolded = split[0].to_string().add("?");
                let mut unfolded_counts = bad_counts.clone();
                for i in 0..4 {
                    unfolded = unfolded.add(split[0]);
                    if i != 3 {
                        unfolded = unfolded.add("?");
                    }

                    let mut bcc = bad_counts.clone();
                    unfolded_counts.append(&mut bcc);
                }

                // println!("{:?}", unfolded);
                // println!("{:?}", unfolded_counts);
                // println!();
                // let result =
                //     decode_to_string(&unfolded_counts, unfolded.len(), Some(unfolded.to_string()));
                // sum2 += result.len();
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
    fill_and_count(missing, holes_count, &mut current, blocks, &re_p)
}

fn fill_and_count(
    missing: usize,
    hole_count: usize,
    current: &mut Vec<usize>,
    blocks: &[u32],
    pattern: &Regex,
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

        if pattern.is_match(&s)
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
        current.push(amount);
        result += fill_and_count(missing - amount, hole_count - 1, current, blocks, pattern);
        current.pop();
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::{count_matching_combinations, encode_to_broken_counts};
    use regex::Regex;

    #[test]
    fn test_encode() {
        let result = encode_to_broken_counts(".#.###.#.######").unwrap();

        assert!(result.eq(&vec![1, 3, 1, 6]));
    }

    #[test]
    fn test_counting_results() {
        let result = count_matching_combinations(
            &vec![3, 2, 1],
            "?###????????".len(),
            &"?###????????".to_string(),
        );

        assert_eq!(result, 10);
    }

    #[test]
    fn test_counting_results_larger() {
        let result = count_matching_combinations(
            &vec![4, 1, 1, 4, 1, 1, 4, 1, 1, 4, 1, 1, 4, 1, 1],
            "????.#...#...?????.#...#...?????.#...#...?????.#...#...?????.#...#...".len(),
            &"????.#...#...?????.#...#...?????.#...#...?????.#...#...?????.#...#...".to_string(),
        );

        assert_eq!(result, 10);
    }
}
