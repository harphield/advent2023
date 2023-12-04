use regex::{Captures, Match, Regex};
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let re_row = Regex::new("Card\\s+[0-9]+:\\s+([0-9\\s]+)|([0-9\\s]+)").unwrap();
    let re_numbers = Regex::new("([0-9]+)").unwrap();

    let mut sum = 0;

    let mut index = 1;

    let mut copies: HashMap<usize, u32> = HashMap::new();
    copies.insert(1, 1); // first card

    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                let groups: Vec<Captures> = re_row.captures_iter(&line).collect();
                let winning = groups.get(0).unwrap().get(1).unwrap().as_str();

                let winning_numbers = re_numbers
                    .find_iter(&winning)
                    .collect::<Vec<Match>>()
                    .iter()
                    .map(|m| {
                        return m.as_str().parse::<u32>().unwrap();
                    })
                    .collect::<Vec<u32>>();

                let have = groups.get(1).unwrap().get(0).unwrap().as_str();

                let have_numbers = re_numbers
                    .find_iter(&have)
                    .collect::<Vec<Match>>()
                    .iter()
                    .map(|m| {
                        return m.as_str().parse::<u32>().unwrap();
                    })
                    .collect::<Vec<u32>>();

                let mut points = 0;
                let mut add_copies = 0;
                for n in have_numbers.iter() {
                    if winning_numbers.contains(n) {
                        if points == 0 {
                            points += 1;
                        } else {
                            points *= 2;
                        }

                        add_copies += 1;
                    }
                }

                let copies_of_this_row = *copies.entry(index).or_insert(1);

                if add_copies > 0 {
                    for a in 1..=add_copies {
                        *copies.entry(index + a).or_insert(1) += copies_of_this_row;
                    }
                }

                sum += points;

                index += 1;
            }
            Err(_) => break,
        }
    }

    println!("Result part 1: {}", sum);

    println!(
        "Result part 2: {}",
        copies.iter().map(|v| { *v.1 }).sum::<u32>()
    );

    Ok(())
}
