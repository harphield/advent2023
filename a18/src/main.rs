use std::cmp::{max, min};
use std::fs::File;
use std::io;
use std::io::BufRead;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let mut plan = vec![];

    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                let split = line.split(' ');

                let mut direction = Direction::Down;
                let mut distance = 0;
                let mut color = "".to_string();
                for (i, s) in split.enumerate() {
                    match i {
                        0 => {
                            direction = match s.chars().collect::<Vec<char>>()[0] {
                                'R' => Direction::Right,
                                'L' => Direction::Left,
                                'U' => Direction::Up,
                                'D' => Direction::Down,
                                _ => panic!("wat"),
                            }
                        }
                        1 => distance = s.parse::<u32>().unwrap(),
                        2 => color = s.to_string(),
                        _ => panic!("err"),
                    }
                }

                plan.push((direction, distance, color));
            }
            Err(_) => break,
        }
    }

    let mut vertices: Vec<(i32, i32)> = vec![];
    let mut vertices_2: Vec<(i32, i32)> = vec![];
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;

    let mut min_x_2 = i32::MAX;
    let mut min_y_2 = i32::MAX;
    let mut max_x_2 = i32::MIN;
    let mut max_y_2 = i32::MIN;

    let mut add_to_area = 1u64; // why?
    let mut add_to_area_2 = 1u64; // why?

    for (direction, distance, color) in plan.iter() {
        let previous = match vertices.last() {
            None => (0i32, 0i32),
            Some(v) => *v,
        };

        let next = match direction {
            Direction::Up => (previous.0, previous.1 - *distance as i32),
            Direction::Down => {
                add_to_area += *distance as u64;
                (previous.0, previous.1 + *distance as i32)
            }
            Direction::Left => (previous.0 - *distance as i32, previous.1),
            Direction::Right => {
                add_to_area += *distance as u64;
                (previous.0 + *distance as i32, previous.1)
            }
        };

        min_x = min(min_x, next.0);
        min_y = min(min_y, next.1);
        max_x = max(max_x, next.0);
        max_y = max(max_y, next.1);

        vertices.push(next);

        // TODO part 2 will use color for numbers
        let hex = color.replace("(#", "").replace(')', "");
        let direction_2 = match hex.chars().last().unwrap() {
            '0' => Direction::Right,
            '1' => Direction::Down,
            '2' => Direction::Left,
            '3' => Direction::Up,
            _ => panic!("nein"),
        };

        let distance_2 = u64::from_str_radix(&hex[0..hex.len() - 1], 16).unwrap();

        let previous_2 = match vertices_2.last() {
            None => (0i32, 0i32),
            Some(v) => *v,
        };

        let next_2 = match direction_2 {
            Direction::Up => (previous_2.0, previous_2.1 - distance_2 as i32),
            Direction::Down => {
                add_to_area_2 += distance_2;
                (previous_2.0, previous_2.1 + distance_2 as i32)
            }
            Direction::Left => (previous_2.0 - distance_2 as i32, previous_2.1),
            Direction::Right => {
                add_to_area_2 += distance_2;
                (previous_2.0 + distance_2 as i32, previous_2.1)
            }
        };

        min_x_2 = min(min_x_2, next_2.0);
        min_y_2 = min(min_y_2, next_2.1);
        max_x_2 = max(max_x_2, next_2.0);
        max_y_2 = max(max_y_2, next_2.1);

        vertices_2.push(next_2);
    }

    let a = area(&vertices) + add_to_area;
    println!("Part 1 result: {}", a);

    let a = area(&vertices_2) + add_to_area_2;
    println!("Part 2 result: {}", a);

    Ok(())
}

/// https://en.wikipedia.org/wiki/Shoelace_formula
/// But I need to add all Downward and Right going edges, plus 1
fn area(vertices: &[(i32, i32)]) -> u64 {
    let mut result = 0i64;

    for (i, v) in vertices.iter().enumerate() {
        let pv = if i == 0 {
            vertices.last().unwrap()
        } else {
            vertices.get(i - 1).unwrap()
        };

        result += (pv.0 + v.0) as i64 * (pv.1 - v.1) as i64;
    }

    (result / 2).unsigned_abs()
}
