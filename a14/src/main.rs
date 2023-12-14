use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let mut width = 0;
    let mut grid = vec![];

    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                if width == 0 {
                    width = line.len();
                }

                for c in line.chars() {
                    grid.push(c);
                }
            }
            Err(_) => break,
        }
    }

    let mut rows_count = grid.len() / width;
    tilt(&mut grid, width);

    let sum: u32 = grid
        .iter()
        .enumerate()
        .map(|(n, c)| {
            if *c != 'O' {
                return 0;
            }

            (rows_count - (n / width)) as u32
        })
        .sum();

    print_grid(&grid, width);

    println!("Part 1 result: {}", sum);

    // Part 2
    let mut sum = 0;
    for n in 0..1_000_000_000 {
        // one spin
        for i in 0..4 {
            tilt(&mut grid, width);
            if i == 3 {
                sum = grid
                    .iter()
                    .enumerate()
                    .map(|(n, c)| {
                        if *c != 'O' {
                            return 0;
                        }

                        (rows_count - (n / width)) as u32
                    })
                    .sum();
            }

            grid = rotate_grid(&mut grid, width);
            width = rows_count;
            rows_count = grid.len() / width;
        }

        // print_grid(&grid, width);
        // println!();
        if n % 100_000 == 0 {
            println!("I'm at {} : {}", n, sum);
        }
    }

    println!("Part 2 result: {}", sum);

    Ok(())
}

fn tilt(
    grid: &mut [char],
    width: usize
) {
    let rock_positions: Vec<usize> = grid
        .iter()
        .enumerate()
        .filter(|(_n, c)| *c == &'O')
        .map(|(n, _c)| n)
        .collect();

    for rp in rock_positions.iter() {
        let x = rp % width;
        let y = rp / width;

        if y == 0 {
        } else {
            // going up
            let mut fall_to_y = y - 1;
            loop {
                if grid[fall_to_y * width + x] == '.' {
                    if fall_to_y > 0 {
                        fall_to_y -= 1;
                    } else {
                        // stop falling
                        if grid[fall_to_y * width + x] != grid[y * width + x] {
                            grid[fall_to_y * width + x] = 'O';
                            grid[y * width + x] = '.';
                        }
                        break;
                    }
                } else {
                    // stop falling
                    if grid[(fall_to_y + 1) * width + x] != grid[y * width + x] {
                        grid[(fall_to_y + 1) * width + x] = 'O';
                        grid[y * width + x] = '.';
                    }
                    break;
                }
            }
        }
    }
}

fn tilt_w_orientation(
    grid: &mut [char],
    width: usize,
    row_count: usize,
    orientation: Orientation
) {
    for i in 0..grid.len() {
        if grid[i] != 'O' {
            continue;
        }

        let x = i % width;
        let y = i / width;

        if y == 0 {
        } else {
            // going up
            let mut fall_to_y = y - 1;
            loop {
                if grid[get_index_by_coords_and_orientation(x, fall_to_y, width, row_count, &orientation)] == '.' {
                    if fall_to_y > 0 {
                        fall_to_y -= 1;
                    } else {
                        // stop falling
                        if grid[get_index_by_coords_and_orientation(x, fall_to_y, width, row_count, &orientation)] != grid[y * width + x] {
                            grid[get_index_by_coords_and_orientation(x, fall_to_y, width, row_count, &orientation)] = 'O';
                            grid[get_index_by_coords_and_orientation(x, y, width, row_count, &orientation)] = '.';
                        }
                        break;
                    }
                } else {
                    // stop falling
                    if grid[get_index_by_coords_and_orientation(x, fall_to_y + 1, width, row_count, &orientation)] != grid[y * width + x] {
                        grid[get_index_by_coords_and_orientation(x, fall_to_y + 1, width, row_count, &orientation)] = 'O';
                        grid[get_index_by_coords_and_orientation(x, y, width, row_count, &orientation)] = '.';
                    }
                    break;
                }
            }
        }
    }
}

enum Orientation {
    North,
    West,
    South,
    East,
}

fn get_index_by_coords_and_orientation(x: usize, y: usize, width: usize, row_count: usize, orientation: &Orientation) -> usize {
    match orientation {
        Orientation::North => {
            y * width + x
        }
        Orientation::West => {
            // [0,0] = [0,9] in a 10x10 grid
            // [1,0] = [0,8]
            // [1,1] = [1,8]

            let new_x = y;
            let new_y = row_count - 1 - x;

            new_y * width + new_x
        }
        Orientation::South => {
            // [0,0] = [9,9] in a 10x10 grid
            // [1,0] = [8,9]
            // [1,1] = [8,8]

            let new_x = width - 1 - x;
            let new_y = row_count - 1 - y;

            new_y * width + new_x
        }
        Orientation::East => {
            // [0,0] = [9,0] in a 10x10 grid
            // [1,0] = [9,1]
            // [1,1] = [8,1]

            let new_x = width - 1 - y;
            let new_y = x;

            new_y * width + new_x
        }

    }
}

/// Always rotate to the left (N -> W -> S -> E)
fn rotate_grid(grid: &mut Vec<char>, width: usize) -> Vec<char> {
    let mut new_grid = vec![];

    let rows_count = grid.len() / width;
    let mut y = rows_count - 1; // last row
    let mut x = 0; // first column

    loop {
        new_grid.push(grid[y * width + x]);

        if y == 0 {
            y = rows_count;
            x += 1;
            if x == width {
                break;
            }
        }

        y -= 1;
    }

    new_grid
}

fn print_grid(grid: &[char], width: usize) {
    let mut iterator = grid.chunks(width);
    loop {
        match iterator.next() {
            None => {
                break;
            }
            Some(c) => {
                println!(
                    "{}",
                    c.iter()
                        .map(|c| c.to_string())
                        .collect::<Vec<String>>()
                        .join("")
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{get_index_by_coords_and_orientation, Orientation};

    #[test]
    fn test_get_coords() {
        assert_eq!(get_index_by_coords_and_orientation(0, 0, 10, 10, &Orientation::North), 0);
        assert_eq!(get_index_by_coords_and_orientation(0, 0, 10, 10, &Orientation::West), 90);
        assert_eq!(get_index_by_coords_and_orientation(0, 0, 10, 10, &Orientation::South), 99);
        assert_eq!(get_index_by_coords_and_orientation(0, 0, 10, 10, &Orientation::East), 9);
    }
}