use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fs::File;
use std::hash::{Hash, Hasher};
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

    let original_grid = grid.clone();

    let rows_count = grid.len() / width;
    tilt_w_orientation(&mut grid, width, rows_count, Orientation::North);

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
    grid = original_grid;
    let mut map: HashMap<u64, (u64, u32)> = HashMap::new();

    let mut sum = 0;
    let mut steps = vec![];
    for n in 0..1_000_000_000 {
        let mut hasher = DefaultHasher::new();
        grid.hash(&mut hasher);
        let grid_hash = hasher.finish();

        match map.get(&grid_hash) {
            None => {
                for i in 0..4 {
                    tilt_w_orientation(
                        &mut grid,
                        width,
                        rows_count,
                        match i {
                            0 => Orientation::North,
                            1 => Orientation::West,
                            2 => Orientation::South,
                            3 => Orientation::East,
                            _ => panic!("aaaa"),
                        },
                    );

                    // print_grid(&grid, width);
                    // println!();
                }

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

                let mut hasher = DefaultHasher::new();
                grid.hash(&mut hasher);
                let grid_hash_after = hasher.finish();

                map.insert(grid_hash, (grid_hash_after, sum));

                println!("Saving at {} : {} {} {}", n, grid_hash, grid_hash_after, sum);
                steps.push((grid_hash, grid_hash_after, sum));
            }
            Some((gh, s)) => {
                // found a loop
                println!("Loop at {} : {} {} {}", n, grid_hash, gh, sum);
                println!("{:?}", steps);

                let mut before_loop_start = 0;
                let mut loop_length = 0;
                let mut loop_started = false;
                for i in 0..steps.len() {
                    if loop_started && steps[i].0 != grid_hash {
                        loop_length += 1;
                    } else if steps[i].0 != grid_hash {
                        before_loop_start += 1;
                    } else {
                        loop_started = true;
                        loop_length += 1;
                    }
                }

                println!("before loop: {}, loop length: {}", before_loop_start, loop_length);

                let loop_step_at_1b = before_loop_start + ((1_000_000_000f32 - before_loop_start as f32) % loop_length as f32) as usize;

                println!("{:?}", steps[loop_step_at_1b]);

                sum = *s;
                break;
            }
        }

        // println!("I'm at {} : {}", n, sum);
        // if n % 100_000 == 0 {
        //     println!("I'm at {} : {}", n, sum);
        // }
    }

    println!("Part 2 result: {}", sum);

    Ok(())
}

fn tilt_w_orientation(grid: &mut [char], width: usize, row_count: usize, orientation: Orientation) {
    let mut rock_positions: Vec<usize> = grid
        .iter()
        .enumerate()
        .filter(|(_n, c)| *c == &'O')
        .map(|(n, _c)| n)
        .collect();

    rock_positions.sort_by_cached_key(|n| {
        let coords = get_coords_from_index_and_orientation(*n, width, row_count, &orientation);
        (coords.1, coords.0)
    });

    for rp in rock_positions.iter() {
        // let x = rp % width;
        // let y = rp / width;
        //
        let new_coords = get_coords_from_index_and_orientation(*rp, width, row_count, &orientation);
        let x = new_coords.0;
        let y = new_coords.1;

        if y == 0 {
        } else {
            // going up
            let mut fall_to_y = y - 1;
            loop {
                if grid[get_index_by_coords_and_orientation(
                    x,
                    fall_to_y,
                    width,
                    row_count,
                    &orientation,
                )] == '.'
                {
                    if fall_to_y > 0 {
                        fall_to_y -= 1;
                    } else {
                        // stop falling
                        if grid[get_index_by_coords_and_orientation(
                            x,
                            fall_to_y,
                            width,
                            row_count,
                            &orientation,
                        )] != grid[get_index_by_coords_and_orientation(
                            x,
                            y,
                            width,
                            row_count,
                            &orientation,
                        )] {
                            grid[get_index_by_coords_and_orientation(
                                x,
                                fall_to_y,
                                width,
                                row_count,
                                &orientation,
                            )] = 'O';
                            grid[get_index_by_coords_and_orientation(
                                x,
                                y,
                                width,
                                row_count,
                                &orientation,
                            )] = '.';
                        }
                        break;
                    }
                } else {
                    // stop falling
                    if grid[get_index_by_coords_and_orientation(
                        x,
                        fall_to_y + 1,
                        width,
                        row_count,
                        &orientation,
                    )] != grid
                        [get_index_by_coords_and_orientation(x, y, width, row_count, &orientation)]
                    {
                        grid[get_index_by_coords_and_orientation(
                            x,
                            fall_to_y + 1,
                            width,
                            row_count,
                            &orientation,
                        )] = 'O';
                        grid[get_index_by_coords_and_orientation(
                            x,
                            y,
                            width,
                            row_count,
                            &orientation,
                        )] = '.';
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

fn get_index_by_coords_and_orientation(
    x: usize,
    y: usize,
    width: usize,
    row_count: usize,
    orientation: &Orientation,
) -> usize {
    match orientation {
        Orientation::North => y * width + x,
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

fn get_coords_from_index_and_orientation(
    index: usize,
    width: usize,
    row_count: usize,
    orientation: &Orientation,
) -> (usize, usize) {
    match orientation {
        Orientation::North => (index % width, index / width),
        Orientation::West => {
            // 0 = [9,0]
            // 10 = [8,0]
            // 11 = [8,1]
            // 99 = [0,9]
            let x = row_count - (index / row_count) - 1;
            let y = index % width;
            (x, y)
        }
        Orientation::South => {
            // 10x10 grid
            // 0 = [9,9]
            // 99 = [0,0]
            let x = width - (index % width) - 1;
            let y = row_count - (index / row_count) - 1;

            (x, y)
        }
        Orientation::East => {
            // 0 = [0,9]
            // 99 = [9,0]
            // 9 = [0,0]
            let x = index / row_count;
            let y = width - (index % width) - 1;

            (x, y)
        }
    }
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
    use crate::{
        get_coords_from_index_and_orientation, get_index_by_coords_and_orientation, Orientation,
    };

    #[test]
    fn test_get_coords() {
        assert_eq!(
            get_index_by_coords_and_orientation(0, 0, 10, 10, &Orientation::North),
            0
        );
        assert_eq!(
            get_index_by_coords_and_orientation(0, 0, 10, 10, &Orientation::West),
            90
        );
        assert_eq!(
            get_index_by_coords_and_orientation(0, 0, 10, 10, &Orientation::South),
            99
        );
        assert_eq!(
            get_index_by_coords_and_orientation(0, 0, 10, 10, &Orientation::East),
            9
        );
    }

    #[test]
    fn test_get_index() {
        // assert_eq!(get_index_by_coords_and_orientation(0, 0, 10, 10, &Orientation::North), 0);
        assert_eq!(
            get_coords_from_index_and_orientation(0, 10, 10, &Orientation::West),
            (9, 0)
        );
        assert_eq!(
            get_coords_from_index_and_orientation(99, 10, 10, &Orientation::West),
            (0, 9)
        );
        assert_eq!(
            get_coords_from_index_and_orientation(0, 10, 10, &Orientation::South),
            (9, 9)
        );
        assert_eq!(
            get_coords_from_index_and_orientation(0, 10, 10, &Orientation::East),
            (0, 9)
        );
        assert_eq!(
            get_coords_from_index_and_orientation(99, 10, 10, &Orientation::East),
            (9, 0)
        );
    }
}
