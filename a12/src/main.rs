use regex::Regex;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::ops::Add;

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let mut sum = 0;
    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                let split: Vec<_> = line.split(' ').collect();

                let bad_counts: Vec<_> = split[1].split(',').collect();
                let bad_counts = bad_counts
                    .iter()
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();

                let result =
                    decode_to_string(&bad_counts, split[0].len(), Some(split[0].to_string()));
                sum += result.len();
            }
            Err(_) => break,
        }
    }

    println!("Part 1 result: {}", sum);

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

fn decode_to_string(blocks: &[u32], length: usize, pattern: Option<String>) -> Vec<String> {
    let regex_pattern = match pattern {
        None => "".to_string(),
        Some(ref p) => {
            let re_q = Regex::new("[?]+").unwrap();
            let mut questionmarks: Vec<String> =
                re_q.find_iter(p).map(|m| m.as_str().to_string()).collect();
            questionmarks.sort_by_key(|a| a.len());
            questionmarks.reverse();

            let mut regex_pattern = p.clone().replace('.', "\\.");
            for q in questionmarks {
                let replace = format!("([#\\.]{{{}}})", q.len());
                regex_pattern = regex_pattern.replace(q.as_str(), replace.as_str());
            }

            regex_pattern
        }
    };

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
        return vec![shortest];
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

    let combinations = fill_holes(missing, holes_count);
    combinations
        .iter()
        .map(|c| {
            let s: Vec<String> = c
                .iter()
                .enumerate()
                .map(|(hole_index, fill_size)| {
                    if hole_index == 0 {
                        vec!["."; *fill_size]
                            .join("")
                            .add(vec!["#"; blocks[hole_index] as usize].join("").as_str())
                    } else if hole_index == holes_count - 1 {
                        vec!["."; *fill_size].join("")
                    } else {
                        vec!["."; *fill_size]
                            .join("")
                            .add(".")
                            .add(vec!["#"; blocks[hole_index] as usize].join("").as_str())
                    }
                })
                .collect();

            s.join("")
        })
        .filter(|s| match &pattern {
            None => true,
            Some(_p) => {
                if re_p.is_match(s) {
                    match encode_to_broken_counts(s) {
                        None => false,
                        Some(bc) => bc.eq(blocks),
                    }
                } else {
                    false
                }
            }
        })
        .collect()
}

fn fill_holes(missing: usize, hole_count: usize) -> Vec<Vec<usize>> {
    if hole_count == 1 {
        return vec![vec![missing]];
    }

    let mut result = vec![];

    for amount in 0..=missing {
        let mut next = fill_holes(missing - amount, hole_count - 1);
        next.iter_mut().for_each(|n| {
            let mut add = vec![amount];
            add.append(n);
            result.push(add);
        });
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::{decode_to_string, encode_to_broken_counts, fill_holes};

    #[test]
    fn test_encode() {
        let result = encode_to_broken_counts(".#.###.#.######").unwrap();

        assert!(result.eq(&vec![1, 3, 1, 6]));
    }

    #[test]
    fn test_decode() {
        let result = decode_to_string(&vec![1, 3, 1, 6], 15, None);

        println!("{:#?}", result);

        assert!(result.len() > 0);
    }

    #[test]
    fn test_decode_with_pattern() {
        let result = decode_to_string(
            &vec![1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3],
            39,
            Some("???.###????.###????.###????.###????.###".to_string()),
        );

        println!("{:#?}", result);

        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_filling() {
        let combinations = fill_holes(4, 4);

        assert!(combinations.len() > 0);
    }
}
