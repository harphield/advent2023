use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let mut pattern = vec![];
    let mut sum = 0;
    let mut n = 1;
    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                if line.is_empty() {
                    // analyze pattern
                    println!("pattern {}", n);
                    sum += analyze_pattern(&pattern);

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
    sum += analyze_pattern(&pattern);

    println!("Part 1 result: {}", sum);

    Ok(())
}

fn analyze_pattern(pattern: &Vec<Vec<char>>) -> u32 {
    let mut result = 0;

    let width = pattern[0].len();
    let height = pattern.len();

    // vertical reflections
    let mut vertical = vec![];
    let mut x = 1;
    let mut mirror_width = 1;
    let mut found;
    loop {
        found = true;
        for c in pattern.iter().take(height) {
            if mirror_width > x
                || x + mirror_width > width
                || c[x - mirror_width] != c[x + mirror_width - 1]
            {
                found = false;
                break;
            }
        }

        if found {
            // expand
            mirror_width += 1;
        } else if mirror_width > x || x + mirror_width > width {
            // only "perfect reflections" count, so I need to hit the edge
            // after which x is the reflection line and how wide is it
            vertical.push((x, mirror_width - 1));

            result += x as u32;

            if x + 1 < width - 1 {
                x += 1;
                mirror_width = 1;
            } else {
                break;
            }
        } else if x + 1 < width {
            x += 1;
            mirror_width = 1;
        } else {
            break;
        }
    }

    println!("vertical: {:?}", vertical);

    // horizontal reflections
    let mut horizontal = vec![];
    let mut y = 1;
    mirror_width = 1;

    loop {
        found = true;
        for x in 0..width {
            if mirror_width > y
                || y + mirror_width > height
                || pattern[y - mirror_width][x] != pattern[y + mirror_width - 1][x]
            {
                found = false;
                break;
            }
        }

        if found {
            // expand
            mirror_width += 1;
        } else if mirror_width > y || y + mirror_width > height {
            result += 100 * y as u32;
            horizontal.push((y, mirror_width - 1));
            if y + 1 < height - 1 {
                y += 1;
                mirror_width = 1;
            } else {
                break;
            }
        } else if y + 1 < height {
            y += 1;
            mirror_width = 1;
        } else {
            break;
        }
    }

    println!("horizontal: {:?}", horizontal);

    result
}
