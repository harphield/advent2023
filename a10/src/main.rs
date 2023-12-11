use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let mut grid = vec![];
    let mut width = 0usize;
    let mut start = 0;

    for (line_nr, line_r) in io::BufReader::new(file).lines().enumerate() {
        match line_r {
            Ok(line) => {
                if width == 0 {
                    width = line.len();
                }

                let mut c = line.chars().collect::<Vec<char>>();
                match c.iter().enumerate().find(|(_i, v)| *v == &'S') {
                    None => {}
                    Some((i, _v)) => {
                        start = i + width * line_nr;
                    }
                }
                grid.append(&mut c);
            }
            Err(_) => break,
        }
    }

    let pipeloop = find_loop(&grid, &width, &start);

    draw_grid(&grid, &width, Some(&pipeloop));

    println!("Part 1 result: {}", (pipeloop.len() as f32 / 2f32).ceil());

    // PART 2
    // https://alienryderflex.com/polygon/
    let mut inside = 0;
    for (i, _c) in grid.iter().enumerate() {
        if !pipeloop.contains(&i) {
            // test against the loop
            let x = i % width;
            let y = i / width;

            let mut before = 0;
            let mut after = 0;
            pipeloop.iter().for_each(|l_i| {
                if *l_i / width == y && !['L', 'J', '-'].contains(&grid[*l_i]) {
                    if *l_i % width < x {
                        before += 1;
                    } else {
                        after += 1;
                    }
                }
            });

            if before > 0 && after > 0 && before % 2 != 0 && after % 2 != 0 {
                inside += 1;
            }
        }
    }

    println!("Part 2 result: {}", inside);

    Ok(())
}

fn find_loop(grid: &[char], width: &usize, start: &usize) -> Vec<usize> {
    // look around the start and find all pipes leading into it
    // try all pairs and find which one loops

    // left
    if start % width != 0 && ['-', 'F', 'L'].contains(&grid[start - 1]) {
        let mut pipeloop = vec![start - 1];
        match next_step_in_loop(grid, width, start - 1, *start, &mut pipeloop) {
            None => {}
            Some(ppl) => {
                return ppl;
            }
        }
    }

    // right
    if start % (width - 1) != 0 && ['-', '7', 'J'].contains(&grid[start + 1]) {
        let mut pipeloop = vec![start + 1];
        match next_step_in_loop(grid, width, start + 1, *start, &mut pipeloop) {
            None => {}
            Some(ppl) => {
                return ppl;
            }
        }
    }

    // up
    if start > width && ['|', 'F', '7'].contains(&grid[start - width]) {
        let mut pipeloop = vec![start - width];
        match next_step_in_loop(grid, width, start - width, *start, &mut pipeloop) {
            None => {}
            Some(ppl) => {
                return ppl;
            }
        }
    }

    // down
    if *start < grid.len() - width && ['|', 'L', 'J'].contains(&grid[start + width]) {
        let mut pipeloop = vec![start + width];
        match next_step_in_loop(grid, width, start + width, *start, &mut pipeloop) {
            None => {}
            Some(ppl) => {
                return ppl;
            }
        }
    }

    vec![]
}

fn next_step_in_loop(
    grid: &[char],
    width: &usize,
    index: usize,
    previous: usize,
    pipeloop: &mut Vec<usize>,
) -> Option<Vec<usize>> {
    let current_char = grid[index];
    let next = match current_char {
        '|' => {
            if previous < index {
                // coming from above, going down
                index + width
            } else {
                // coming from below, going up
                index - width
            }
        }
        '-' => {
            if previous < index {
                // coming from the left, going right
                index + 1
            } else {
                // coming from the right, going left
                index - 1
            }
        }
        'L' => {
            if previous < index {
                // coming from above, going right
                index + 1
            } else {
                // coming from the right, going up
                index - width
            }
        }
        'J' => {
            if previous == index - width {
                // coming from above, going left
                index - 1
            } else {
                // coming from the left, going up
                index - width
            }
        }
        '7' => {
            if previous == index + width {
                // coming from below, going left
                index - 1
            } else {
                // coming from the left, going down
                index + width
            }
        }
        'F' => {
            if previous == index + width {
                // coming from below, going right
                index + 1
            } else {
                // coming from the right, going down
                index + width
            }
        }
        _ => panic!("wat"),
    };

    match grid.get(next) {
        None => None,
        Some(c) => {
            let accept = match c {
                '.' => {
                    vec![]
                }
                'S' => {
                    pipeloop.push(next);
                    return Some(pipeloop.clone());
                }
                '|' => {
                    if index < next {
                        // coming from above, going down
                        vec!['|', '7', 'F']
                    } else {
                        // coming from below, going up
                        vec!['|', 'L', 'J']
                    }
                }
                '-' => {
                    if index < next {
                        // coming from the left, going right
                        vec!['-', 'F', 'L']
                    } else {
                        // coming from the right, going left
                        vec!['-', '7', 'J']
                    }
                }
                'L' => {
                    if index < next {
                        // coming from above, going right
                        vec!['|', '7', 'F']
                    } else {
                        // coming from the right, going up
                        vec!['-', '7', 'J']
                    }
                }
                'J' => {
                    if index == next - width {
                        // coming from above, going left
                        vec!['|', '7', 'F']
                    } else {
                        // coming from the left, going up
                        vec!['-', 'F', 'L']
                    }
                }
                '7' => {
                    if index == next + width {
                        // coming from below, going left
                        vec!['|', 'L', 'J']
                    } else {
                        // coming from the left, going down
                        vec!['-', 'F', 'L']
                    }
                }
                'F' => {
                    if index == next + width {
                        // coming from below, going right
                        vec!['|', 'L', 'J']
                    } else {
                        // coming from the right, going down
                        vec!['-', '7', 'J']
                    }
                }
                _ => panic!("wat"),
            };

            if !accept.contains(&current_char) {
                // not a loop
                return None;
            }

            if !pipeloop.contains(&next) {
                pipeloop.push(next);
                return next_step_in_loop(grid, width, next, index, pipeloop);
            }

            Some(pipeloop.clone())
        }
    }
}

fn draw_grid(grid: &[char], width: &usize, show_these: Option<&Vec<usize>>) {
    for (i, c) in grid.iter().enumerate() {
        if i != 0 && i % width == 0 {
            println!();
        }

        match show_these {
            None => {
                print!("{}", draw_symbol(c));
            }
            Some(st) => {
                if st.contains(&i) {
                    print!("{}", draw_symbol(c));
                } else {
                    print!("{}", draw_symbol(&'.'));
                }
            }
        }
    }

    println!();
}

fn draw_symbol(symbol: &char) -> char {
    match symbol {
        '|' => '║',
        '-' => '═',
        'L' => '╚',
        'J' => '╝',
        '7' => '╗',
        'F' => '╔',
        '.' => ' ',
        &c => c,
    }
}
