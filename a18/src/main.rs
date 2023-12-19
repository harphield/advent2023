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

    let mut x_offset = 0;
    let mut y_offset = 0;

    for (direction, distance, _color) in plan.iter() {
        let start = match vertices.last() {
            None => (0i32, 0i32),
            Some(v) => *v,
        };

        let next = match direction {
            Direction::Up => (start.0, start.1 - *distance as i32),
            Direction::Down => (start.0, start.1 + *distance as i32),
            Direction::Left => (start.0 - *distance as i32, start.1),
            Direction::Right => (start.0 + *distance as i32, start.1),
        };

        min_x = min(min_x, next.0);
        min_y = min(min_y, next.1);
        max_x = max(max_x, next.0);
        max_y = max(max_y, next.1);

        vertices.push(next);

        // part 2 will use color for numbers
        let start_2 = match vertices_2.last() {
            None => (0i32, 0i32),
            Some(v) => *v,
        };

        let next_2 = match direction {
            Direction::Up => (start_2.0, y_offset + start_2.1 - *distance as i32),
            Direction::Down => {
                y_offset += 1;
                (start_2.0, 1 + start_2.1 + *distance as i32)
            },
            Direction::Left => (x_offset + start_2.0 - *distance as i32, start_2.1),
            Direction::Right => {
                x_offset += 1;
                (1 + start_2.0 + *distance as i32, start_2.1)
            },
        };

        min_x_2 = min(min_x_2, next_2.0);
        min_y_2 = min(min_y_2, next_2.1);
        max_x_2 = max(max_x_2, next_2.0);
        max_y_2 = max(max_y_2, next_2.1);

        vertices_2.push(next_2);
    }

    println!("{:?}", vertices);

    // normalize vertices
    let mut normalized_vertices = vec![];
    for v in vertices.iter() {
        normalized_vertices.push((v.0 - min_x, v.1 - min_y));
    }

    vertices = normalized_vertices;

    max_x -= min_x;
    max_y -= min_y;

    let width = max_x as usize + 1;
    let height = max_y as usize + 1;

    let mut grid = vec!['.'; width * height];

    // normalize 2
    let mut normalized_vertices_2 = vec![];
    for v in vertices_2.iter() {
        normalized_vertices_2.push((v.0 - min_x, v.1 - min_y));
    }

    vertices_2 = normalized_vertices_2;

    max_x_2 -= min_x_2;
    max_y_2 -= min_y_2;

    let width_2 = max_x_2 as usize + 1;
    let height_2 = max_y_2 as usize + 1;

    let mut grid_2 = vec!['.'; width_2 * height_2];

    let mut border = vec![];
    let mut vertex_indexes = vec![];

    // fill in the border
    let mut prev_v: Option<&(i32, i32)> = None;
    for (vi, v) in vertices.iter().enumerate() {
        let vertex_index = (v.1 as usize * width) + v.0 as usize;
        vertex_indexes.push(vertex_index);
        border.push(vertex_index);

        let pv = prev_v.unwrap_or_else(|| vertices.last().unwrap());

        prev_v = Some(v);

        let nv = vertices
            .get(vi + 1)
            .unwrap_or_else(|| vertices.first().unwrap());

        if v.0 == pv.0 {
            if v.1 < pv.1 {
                for y in v.1 + 1..pv.1 {
                    grid[y as usize * width + v.0 as usize] = '|';
                    border.push(y as usize * width + v.0 as usize);
                }

                // vertex is either 7 or F
                if v.0 < nv.0 {
                    grid[vertex_index] = 'F';
                } else {
                    grid[vertex_index] = '7';
                }
            } else {
                for y in pv.1 + 1..v.1 {
                    grid[y as usize * width + v.0 as usize] = '|';
                    border.push(y as usize * width + v.0 as usize);
                }

                // vertex is either L or J
                if v.0 < nv.0 {
                    grid[vertex_index] = 'L';
                } else {
                    grid[vertex_index] = 'J';
                }
            }
        } else if v.1 == pv.1 {
            if v.0 < pv.0 {
                for x in v.0 + 1..pv.0 {
                    grid[v.1 as usize * width + x as usize] = '-';
                    border.push(v.1 as usize * width + x as usize);
                }

                // vertex is either F or L
                if v.1 < nv.1 {
                    grid[vertex_index] = 'F';
                } else {
                    grid[vertex_index] = 'L';
                }
            } else {
                for x in pv.0 + 1..v.0 {
                    grid[v.1 as usize * width + x as usize] = '-';
                    border.push(v.1 as usize * width + x as usize);
                }

                // vertex is either 7 or J
                if v.1 < nv.1 {
                    grid[vertex_index] = '7';
                } else {
                    grid[vertex_index] = 'J';
                }
            }
        }
    }

    border.sort();
    border.dedup();

    draw_grid(&grid, &width);

    let mut inside = 0;
    for (i, c) in grid.iter().enumerate() {
        if *c == '.' {
            // test against the loop
            let x = i % width;
            let y = i / width;

            let mut before = 0;
            let mut after = 0;
            border.iter().for_each(|l_i| {
                if *l_i / width == y && !['L', 'J', '-'].contains(&grid[*l_i]) {
                    let l_x = *l_i % width;

                    if l_x < x {
                        before += 1;
                    } else {
                        after += 1;
                    }
                }
            });

            if before > 0 && after > 0 && before % 2 != 0 && after % 2 != 0 {
                inside += 1;
            }
        } else {
            inside += 1;
        }
    }

    println!("{}", inside);

    let a = area(&vertices);

    println!("Area: {}", a);

    Ok(())
}

fn draw_grid(grid: &[char], width: &usize) {
    for (i, c) in grid.iter().enumerate() {
        if i != 0 && i % width == 0 {
            println!();
        }

        print!("{}", c);
    }

    println!();
}

fn area(vertices: &[(i32, i32)]) -> u64 {
    let mut result = 0i64;

    for (i, v) in vertices.iter().enumerate() {
        let pv = if i == 0 {
            vertices.get(vertices.len() - 1).unwrap()
        } else {
            vertices.get(i - 1).unwrap()
        };

        result += (pv.0 + v.0) as i64 * (pv.1 - v.1) as i64;
    }

    (result / 2).abs() as u64
}
