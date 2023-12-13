use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let mut pattern = vec![];
    let mut sum = 0;
    let mut sum_with_flipped = 0;
    let mut n = 1;
    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                if line.is_empty() {
                    // analyze pattern
                    println!("pattern {}", n);
                    let (r, rf) = analyze_pattern(&pattern);
                    sum += r;
                    sum_with_flipped += rf;

                    pattern = vec![];
                    n += 1;
                } else {
                    let v: Vec<char> = line.chars().collect();
                    pattern.push(v);
                }
            }
            Err(_) => break,
        }
    }

    // last pattern here
    println!("pattern last");
    let (r, rf) = analyze_pattern(&pattern);
    sum += r;
    sum_with_flipped += rf;

    println!("Part 1 result: {}", sum);
    println!("Part 2 result: {}", sum_with_flipped);

    Ok(())
}

fn analyze_pattern(pattern: &Vec<Vec<char>>) -> (u32, u32) {
    let mut result = 0;
    let mut result_with_flipped = 0;

    let width = pattern[0].len();
    let height = pattern.len();

    // vertical reflections
    let mut vertical = vec![];
    let mut x = 1;
    let mut mirror_width = 1;
    let mut found;
    let mut tried_flip = false;

    loop {
        found = true;
        for c in pattern.iter().take(height) {
            if mirror_width > x || x + mirror_width > width {
                found = false;
                break;
            }

            if c[x - mirror_width] != c[x + mirror_width - 1] {
                if !tried_flip {
                    tried_flip = true;
                } else {
                    found = false;
                    break;
                }
            }
        }

        if found {
            // expand
            mirror_width += 1;
        } else if mirror_width > x || x + mirror_width > width {
            // only "perfect reflections" count, so I need to hit the edge
            // after which x is the reflection line and how wide is it
            vertical.push((x, mirror_width - 1));

            if tried_flip {
                result_with_flipped += x as u32;
            } else {
                result += x as u32;
            }

            if x + 1 < width {
                x += 1;
                mirror_width = 1;
                tried_flip = false;
            } else {
                break;
            }
        } else if x + 1 < width {
            x += 1;
            mirror_width = 1;
            tried_flip = false;
        } else {
            break;
        }
    }

    println!("vertical: {:?}", vertical);

    // horizontal reflections
    let mut horizontal = vec![];
    let mut y = 1;
    mirror_width = 1;
    tried_flip = false;

    loop {
        found = true;
        for x in 0..width {
            if mirror_width > y || y + mirror_width > height {
                found = false;
                break;
            }

            if pattern[y - mirror_width][x] != pattern[y + mirror_width - 1][x] {
                if !tried_flip {
                    tried_flip = true;
                } else {
                    found = false;
                    break;
                }
            }
        }

        if found {
            // expand
            mirror_width += 1;
        } else if mirror_width > y || y + mirror_width > height {
            if tried_flip {
                result_with_flipped += 100 * y as u32;
            } else {
                result += 100 * y as u32;
            }

            horizontal.push((y, mirror_width - 1));
            if y + 1 < height {
                y += 1;
                mirror_width = 1;
                tried_flip = false;
            } else {
                break;
            }
        } else if y + 1 < height {
            y += 1;
            mirror_width = 1;
            tried_flip = false;
        } else {
            break;
        }
    }

    println!("horizontal: {:?}", horizontal);

    (result, result_with_flipped)
}
