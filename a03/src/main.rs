use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;

#[derive(Debug)]
struct Number {
    value: u32,
    gears: Vec<usize>
}

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let mut length = 0;
    let mut plan: Vec<char> = vec![];

    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                if length == 0 {
                    length = line.len();
                }

                let mut c: Vec<char> = line.chars().collect();
                plan.append(&mut c);
            }
            Err(_) => break
        }
    }

    let mut sum = 0;
    let mut digits = vec![];
    let mut number_has_symbol_neighbor = false;
    let mut gear_neighbors = vec![];
    let mut numbers_with_gears = vec![];

    for (i, n) in plan.iter().enumerate() {
        if !n.is_digit(10) {
            if digits.len() > 0 && number_has_symbol_neighbor {
                let mut order = 1;
                let mut number = 0;
                for d in digits.iter().rev() {
                    let digit: char = plan[*d];
                    number += digit.to_digit(10).unwrap() * order;
                    order *= 10;
                }

                sum += number;

                gear_neighbors.sort();
                gear_neighbors.dedup();

                if gear_neighbors.len() > 0 {
                    numbers_with_gears.push(Number {
                        value: number,
                        gears: gear_neighbors,
                    })
                }
            }

            digits = vec![];
            number_has_symbol_neighbor = false;

            gear_neighbors = vec![];

            continue;
        }

        digits.push(i);
        if !number_has_symbol_neighbor {
            number_has_symbol_neighbor = has_symbol_neighbor(i, &plan, length);
        }

        let mut gn = get_gear_neighbors(i, &plan, length);
        gear_neighbors.append(&mut gn);
    }

    println!("Part 1 result: {}", sum);

    let mut gears: HashMap<usize, Vec<u32>> = HashMap::new();
    for number in numbers_with_gears.iter() {
        for g in number.gears.iter() {
            let e = gears.entry(*g).or_insert(vec![]);
            e.push(number.value);
        }
    }

    let mut sum = 0;
    for (_i, gear_neighbors) in gears.iter() {
        if gear_neighbors.len() == 2 {
            sum += gear_neighbors[0] * gear_neighbors[1];
        }
    }

    println!("Part 2 result: {}", sum);

    Ok(())
}

fn has_symbol_neighbor(i: usize, plan: &Vec<char>, length: usize) -> bool {
    // check left
    if i > 0 && i % length > 0 && !plan[i - 1].is_digit(10) && plan[i - 1] != '.' {
        return true;
    }

    // check right
    if i < plan.len() - 1 && (i + length) % length > 0 && !plan[i + 1].is_digit(10) && plan[i + 1] != '.' {
        return true;
    }

    // check down
    if i < plan.len() - length && !plan[i + length].is_digit(10) && plan[i + length] != '.' {
        return true;
    }

    // check up
    if i > length && !plan[i - length].is_digit(10) && plan[i - length] != '.' {
        return true;
    }

    // check up left
    if i > length && i % length > 0 && !plan[i - length - 1].is_digit(10) && plan[i - length - 1] != '.' {
        return true;
    }

    // check up right
    if i > length && (i + length) % length > 0 && !plan[i - length + 1].is_digit(10) && plan[i - length + 1] != '.' {
        return true;
    }

    // check down left
    if i < plan.len() - length && i % length > 0 && !plan[i + length - 1].is_digit(10) && plan[i + length - 1] != '.' {
        return true;
    }

    // check down right
    if i < plan.len() - length && (i + length) % length > 0 && !plan[i + length + 1].is_digit(10) && plan[i + length + 1] != '.' {
        return true;
    }

    false
}

fn get_gear_neighbors(i: usize, plan: &Vec<char>, length: usize) -> Vec<usize> {
    let mut result = vec![];

    // check left
    if i > 0 && i % length > 0 && plan[i - 1] == '*' {
        result.push(i - 1);
    }

    // check right
    if i < plan.len() - 1 && (i + length) % length > 0 && plan[i + 1] == '*' {
        result.push(i + 1);
    }

    // check down
    if i < plan.len() - length && plan[i + length] == '*' {
        result.push(i + length);
    }

    // check up
    if i > length && plan[i - length] == '*' {
        result.push(i - length);
    }

    // check up left
    if i > length && i % length > 0 && plan[i - length - 1] == '*' {
        result.push(i - length - 1);
    }

    // check up right
    if i > length && (i + length) % length > 0 && plan[i - length + 1] == '*' {
        result.push(i - length + 1);
    }

    // check down left
    if i < plan.len() - length && i % length > 0 && plan[i + length - 1] == '*' {
        result.push(i + length - 1);
    }

    // check down right
    if i < plan.len() - length && (i + length) % length > 0 && plan[i + length + 1] == '*' {
        result.push(i + length + 1);
    }

    result
}