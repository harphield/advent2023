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

    let rows_count = grid.len() / width;

    for i in 0..grid.len() {
        let c = grid[i];

        let x = i % width;
        let y = i / width;

        if c == 'O' {
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

    print_grid(&grid, width);

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

    println!("Part 1 result: {}", sum);

    Ok(())
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
