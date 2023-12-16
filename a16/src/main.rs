use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::BufRead;
use std::{fmt, io};

enum Type {
    Empty,
    UpwardMirror,
    DownwardMirror,
    HorizontalSplitter,
    VerticalSplitter,
}

impl Type {
    fn identify(repr: &char) -> Type {
        match repr {
            '.' => Type::Empty,
            '/' => Type::UpwardMirror,
            '\\' => Type::DownwardMirror,
            '-' => Type::HorizontalSplitter,
            '|' => Type::VerticalSplitter,
            &_ => panic!("nonexistant type"),
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Type::Empty => '.',
                Type::UpwardMirror => '/',
                Type::DownwardMirror => '\\',
                Type::HorizontalSplitter => '-',
                Type::VerticalSplitter => '|',
            }
        )
    }
}

#[derive(Clone, PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
struct Beam {
    position: usize,
    direction: Direction,
}

impl PartialEq for Beam {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.direction.eq(&other.direction)
    }
}

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
                    grid.push(Type::identify(&c));
                }
            }
            Err(_) => break,
        }
    }

    // print_grid(&grid, width);

    let result = find_energized_count(0, Direction::Right, &grid, width);
    println!("Part 1 result: {}", result);

    // Part 2
    let rows = grid.len() / width;
    let mut results = vec![];
    for y in 0..rows {
        for x in 0..width {
            if x == 0 && y == 0 {
                // right + down
                results.push(result); // already done this before
                results.push(find_energized_count(0, Direction::Down, &grid, width));
            } else if x == width - 1 && y == 0 {
                // left + down
                results.push(find_energized_count(
                    y * rows + x,
                    Direction::Left,
                    &grid,
                    width,
                ));
                results.push(find_energized_count(
                    y * rows + x,
                    Direction::Down,
                    &grid,
                    width,
                ));
            } else if x == 0 && y == rows - 1 {
                // right + up
                results.push(find_energized_count(
                    y * rows + x,
                    Direction::Right,
                    &grid,
                    width,
                ));
                results.push(find_energized_count(
                    y * rows + x,
                    Direction::Up,
                    &grid,
                    width,
                ));
            } else if x == width - 1 && y == rows - 1 {
                // left + up
                results.push(find_energized_count(
                    y * rows + x,
                    Direction::Left,
                    &grid,
                    width,
                ));
                results.push(find_energized_count(
                    y * rows + x,
                    Direction::Up,
                    &grid,
                    width,
                ));
            } else if x == 0 {
                // right
                results.push(find_energized_count(
                    y * rows + x,
                    Direction::Right,
                    &grid,
                    width,
                ));
            } else if x == width - 1 {
                // left
                results.push(find_energized_count(
                    y * rows + x,
                    Direction::Left,
                    &grid,
                    width,
                ));
            } else if y == 0 {
                // down
                results.push(find_energized_count(
                    y * rows + x,
                    Direction::Down,
                    &grid,
                    width,
                ));
            } else if y == rows - 1 {
                // up
                results.push(find_energized_count(
                    y * rows + x,
                    Direction::Up,
                    &grid,
                    width,
                ));
            }
        }
    }

    results.sort();

    println!("Part 2 result: {}", results.last().unwrap());

    Ok(())
}

fn find_energized_count(
    position: usize,
    direction: Direction,
    grid: &Vec<Type>,
    width: usize,
) -> usize {
    let mut path = vec![];
    let start = Beam {
        position,
        direction,
    };

    light_travel(start, grid, width, &mut path);

    let mut positions: Vec<usize> = path.iter().map(|b| b.position).collect();
    positions.sort();
    positions.dedup();

    positions.len()
}

fn light_travel(beam: Beam, grid: &Vec<Type>, width: usize, path: &mut Vec<Beam>) {
    // we stop traveling when:
    // - we go off the edge
    // - we get to the same position and direction we were at before

    let x = beam.position % width;
    let y = beam.position / width;

    if path.contains(&beam) {
        // println!("contains {} {} {:?}", x, y, beam.direction);
        return;
    }

    path.push(beam.clone());

    let rows = grid.len() / width;

    // println!("{},{} : {} {:?}", x, y, grid[beam.position], beam.direction);

    // check where we are now
    match grid[beam.position] {
        Type::Empty => {
            // continue the same direction
            match beam.direction {
                Direction::Up => {
                    if y > 0 {
                        light_travel(
                            Beam {
                                position: beam.position - width,
                                direction: beam.direction,
                            },
                            grid,
                            width,
                            path,
                        );
                    }
                }
                Direction::Down => {
                    if y < rows - 1 {
                        light_travel(
                            Beam {
                                position: beam.position + width,
                                direction: beam.direction,
                            },
                            grid,
                            width,
                            path,
                        );
                    }
                }
                Direction::Left => {
                    if x > 0 {
                        light_travel(
                            Beam {
                                position: beam.position - 1,
                                direction: beam.direction,
                            },
                            grid,
                            width,
                            path,
                        );
                    }
                }
                Direction::Right => {
                    if x < width - 1 {
                        light_travel(
                            Beam {
                                position: beam.position + 1,
                                direction: beam.direction,
                            },
                            grid,
                            width,
                            path,
                        );
                    }
                }
            }
        }
        Type::UpwardMirror => {
            // change direction
            match beam.direction {
                Direction::Up => {
                    // to the right
                    if x < width - 1 {
                        light_travel(
                            Beam {
                                position: beam.position + 1,
                                direction: Direction::Right,
                            },
                            grid,
                            width,
                            path,
                        );
                    }
                }
                Direction::Down => {
                    // to the left
                    if x > 0 {
                        light_travel(
                            Beam {
                                position: beam.position - 1,
                                direction: Direction::Left,
                            },
                            grid,
                            width,
                            path,
                        );
                    }
                }
                Direction::Left => {
                    // going down
                    if y < rows - 1 {
                        light_travel(
                            Beam {
                                position: beam.position + width,
                                direction: Direction::Down,
                            },
                            grid,
                            width,
                            path,
                        );
                    }
                }
                Direction::Right => {
                    // going up
                    if y > 0 {
                        light_travel(
                            Beam {
                                position: beam.position - width,
                                direction: Direction::Up,
                            },
                            grid,
                            width,
                            path,
                        );
                    }
                }
            }
        }
        Type::DownwardMirror => {
            match beam.direction {
                Direction::Up => {
                    // to the left
                    if x > 0 {
                        light_travel(
                            Beam {
                                position: beam.position - 1,
                                direction: Direction::Left,
                            },
                            grid,
                            width,
                            path,
                        );
                    }
                }
                Direction::Down => {
                    // to the right
                    if x < width - 1 {
                        light_travel(
                            Beam {
                                position: beam.position + 1,
                                direction: Direction::Right,
                            },
                            grid,
                            width,
                            path,
                        );
                    }
                }
                Direction::Left => {
                    // going up
                    if y > 0 {
                        light_travel(
                            Beam {
                                position: beam.position - width,
                                direction: Direction::Up,
                            },
                            grid,
                            width,
                            path,
                        );
                    }
                }
                Direction::Right => {
                    // going down
                    if y < rows - 1 {
                        light_travel(
                            Beam {
                                position: beam.position + width,
                                direction: Direction::Down,
                            },
                            grid,
                            width,
                            path,
                        );
                    }
                }
            }
        }
        Type::HorizontalSplitter => {
            match beam.direction {
                Direction::Up | Direction::Down => {
                    // split left and right
                    if x > 0 {
                        light_travel(
                            Beam {
                                position: beam.position - 1,
                                direction: Direction::Left,
                            },
                            grid,
                            width,
                            path,
                        );
                    }
                    if x < width - 1 {
                        light_travel(
                            Beam {
                                position: beam.position + 1,
                                direction: Direction::Right,
                            },
                            grid,
                            width,
                            path,
                        );
                    }
                }
                Direction::Left => {
                    // to the left
                    if x > 0 {
                        light_travel(
                            Beam {
                                position: beam.position - 1,
                                direction: Direction::Left,
                            },
                            grid,
                            width,
                            path,
                        );
                    }
                }
                Direction::Right => {
                    // to the right
                    if x < width - 1 {
                        light_travel(
                            Beam {
                                position: beam.position + 1,
                                direction: Direction::Right,
                            },
                            grid,
                            width,
                            path,
                        );
                    }
                }
            }
        }
        Type::VerticalSplitter => {
            match beam.direction {
                Direction::Up => {
                    // going up
                    if y > 0 {
                        light_travel(
                            Beam {
                                position: beam.position - width,
                                direction: Direction::Up,
                            },
                            grid,
                            width,
                            path,
                        );
                    }
                }
                Direction::Down => {
                    // going down
                    if y < rows - 1 {
                        light_travel(
                            Beam {
                                position: beam.position + width,
                                direction: Direction::Down,
                            },
                            grid,
                            width,
                            path,
                        );
                    }
                }
                Direction::Left | Direction::Right => {
                    // going up
                    if y > 0 {
                        light_travel(
                            Beam {
                                position: beam.position - width,
                                direction: Direction::Up,
                            },
                            grid,
                            width,
                            path,
                        );
                    }
                    // going down
                    if y < rows - 1 {
                        light_travel(
                            Beam {
                                position: beam.position + width,
                                direction: Direction::Down,
                            },
                            grid,
                            width,
                            path,
                        );
                    }
                }
            }
        }
    }
}

fn print_grid(grid: &[Type], width: usize) {
    for c in grid.chunks(width) {
        for t in c {
            print!("{}", t);
        }

        println!();
    }
}

fn print_energized(positions: &[usize], grid: &[Type], width: usize) {
    for (i, _t) in grid.iter().enumerate() {
        if i % width == 0 {
            println!();
        }

        if positions.contains(&i) {
            print!("#");
        } else {
            print!(".");
        }
    }

    println!();
}
