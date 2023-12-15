use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let mut sum = 0;
    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                for step in line.split(',') {
                    sum += hash(step);
                }
            }
            Err(_) => break,
        }
    }

    println!("Part 1 result: {}", sum);

    Ok(())
}

/// - Determine the ASCII code for the current character of the string.
/// - Increase the current value by the ASCII code you just determined.
/// - Set the current value to itself multiplied by 17.
/// - Set the current value to the remainder of dividing itself by 256.
fn hash(value: &str) -> u32 {
    let mut hash: u32 = 0;
    for c in value.chars() {
        let ascii = c as u8;
        hash += ascii as u32;
        hash *= 17;
        hash %= 256;
    }

    hash
}
