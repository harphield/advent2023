use std::fs::File;
use std::io;
use std::io::BufRead;
use std::usize::MAX;
use regex::Regex;

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let mut values = vec![];
    let mut part2_sum = 0;

    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                // part 1
                values.push((find_digit(&line, 0) * 10) + find_digit(&line, 1));

                // part 2
                part2_sum += find_calibration_value(&line);
            }
            Err(_) => break
        }
    }

    println!("Part 1 result: {}", values.iter().sum::<u32>());

    println!("Part 2 result: {}", part2_sum);

    Ok(())
}

fn find_digit(s: &str, direction: u8) -> u32 {
    let reversed = s.chars().rev().collect::<String>();
    let iterator = match direction {
        0 => s.chars(),
        1 => reversed.chars(),
        _ => panic!("wat")
    };

    for c in iterator {
        if c.is_digit(10) {
            return c.to_digit(10).unwrap();
        }
    }

    panic!("no digits");
}

fn find_calibration_value(s: &str) -> u32 {
    const PATTERNS: [&str; 10] = [
        r"\d",
        r"one",
        r"two",
        r"three",
        r"four",
        r"five",
        r"six",
        r"seven",
        r"eight",
        r"nine",
    ];

    let mut first_index: usize = MAX;
    let mut first ;
    let mut first_value= 0;
    let mut last_index = 0;
    let mut last ;
    let mut last_value = 0;

    for p in PATTERNS {
        let re = Regex::new(p).unwrap();
        let mut matches = re.find_iter(s);

        let first_match = match matches.next() {
            None => continue,
            Some(v) => v
        };

        let last_match = match matches.last() {
            None => first_match,
            Some(v) => v
        };

        if first_match.start() < first_index {
            first_index = first_match.start();
            first = first_match.as_str();
            first_value = found_to_number(first);
        }

        if last_match.start() >= last_index {
            last_index = last_match.start();
            last = last_match.as_str();
            last_value = found_to_number(last);
        }
    }

    let result = (first_value * 10) + last_value;

    result
}

fn found_to_number(n: &str) -> u32 {
    if n.len() == 1 {
        // digit
        n.chars().next().unwrap().to_digit(10).unwrap()
    } else {
        // text representation
        match n {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => panic!("bad number")
        }
    }
}
