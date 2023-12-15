use indexmap::IndexMap;
use std::fs::File;
use std::io;
use std::io::BufRead;

enum Operation {
    Remove(String),
    Add(String, u32),
}

type LensBox = IndexMap<String, u32>;

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let mut boxes: Vec<LensBox> = vec![LensBox::new(); 256];
    let mut sum = 0;
    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                for step in line.split(',') {
                    sum += hash(step);

                    let box_index;
                    let operation = if step.contains('-') {
                        let split: Vec<&str> = step.split('-').collect();
                        box_index = hash(split[0]);
                        Operation::Remove(split[0].to_string())
                    } else {
                        let split: Vec<&str> = step.split('=').collect();
                        box_index = hash(split[0]);
                        Operation::Add(split[0].to_string(), split[1].parse::<u32>().unwrap())
                    };

                    match operation {
                        Operation::Remove(s) => {
                            boxes[box_index].shift_remove(&s);
                        }
                        Operation::Add(s, v) => {
                            *boxes[box_index].entry(s).or_insert(0) = v;
                        }
                    }
                }
            }
            Err(_) => break,
        }
    }

    println!("Part 1 result: {}", sum);

    // Part 2

    let mut sum = 0;
    for (box_nr, lens_box) in boxes.iter().enumerate() {
        let box_power = 1 + box_nr as u32;

        for (n, (_label, focal_length)) in lens_box.iter().enumerate() {
            let focusing_power = box_power * (n as u32 + 1) * focal_length;
            sum += focusing_power;
        }
    }

    println!("Part 2 result: {}", sum);

    Ok(())
}

/// - Determine the ASCII code for the current character of the string.
/// - Increase the current value by the ASCII code you just determined.
/// - Set the current value to itself multiplied by 17.
/// - Set the current value to the remainder of dividing itself by 256.
fn hash(value: &str) -> usize {
    let mut hash: usize = 0;
    for c in value.chars() {
        let ascii = c as u8;
        hash += ascii as usize;
        hash *= 17;
        hash %= 256;
    }

    hash
}
