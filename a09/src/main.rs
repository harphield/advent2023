use regex::Regex;
use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let re_numbers = Regex::new("([0-9\\-]+)").unwrap();

    let mut sum = 0;
    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                let sequence = re_numbers
                    .find_iter(&line)
                    .map(|m| m.as_str().parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();

                let prediction = predict_next_number(&sequence);
                sum += sequence.iter().last().unwrap() + prediction;
            }
            Err(_) => break,
        }
    }

    println!("Part 1 result: {}", sum);

    Ok(())
}

fn predict_next_number(sequence: &[i32]) -> i32 {
    let len = sequence.len();

    let mut new_sequence = vec![];
    let mut next = false;
    for i in 0..len - 1 {
        if sequence[i + 1] - sequence[i] != 0 {
            next = true;
        }
        new_sequence.push(sequence[i + 1] - sequence[i]);
    }

    return if !next {
        0
    } else {
        let add = predict_next_number(&new_sequence);
        new_sequence.iter().last().unwrap() + add
    };
}
