use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let mut width = 0;
    let mut space = vec![];

    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                if width == 0 {
                    width = line.len();
                }

                let mut row = line.chars().collect::<Vec<char>>();
                space.append(&mut row);
            }
            Err(_) => break,
        }
    }

    // keep this for part 2
    let original_space = space.clone();
    let original_width = width;

    // add rows
    let mut new_space = space.clone();

    let mut expanded_rows = vec![];

    for i in (0..(new_space.len() / width)).rev() {
        if new_space
            .iter()
            .enumerate()
            .filter(|(n, _v)| n / width == i)
            .all(|(_n, v)| *v == '.')
        {
            expanded_rows.push(i);
            for _x in 0..width {
                new_space.insert(i * width, '.');
            }
        }
    }

    space = new_space;

    // add columns
    let mut new_space = space.clone();

    let mut expanded_cols = vec![];

    let mut new_width = width;
    for i in (0..new_width).rev() {
        if new_space
            .iter()
            .enumerate()
            .filter(|(n, _v)| n % new_width == i)
            .all(|(_n, v)| *v == '.')
        {
            expanded_cols.push(i);
            for row in (0..new_space.len() / new_width).rev() {
                new_space.insert(i + (row * new_width), '.');
            }

            new_width += 1;
        }
    }

    width = new_width;
    space = new_space;

    draw_space(&space, &width);

    // find galaxies
    let galaxies = space
        .iter()
        .enumerate()
        .filter(|(_n, v)| *v == &'#')
        .map(|(n, _v)| n)
        .collect::<Vec<usize>>();

    // Part 1: manattan distance
    let mut sum = 0;
    for (n, g) in galaxies.iter().enumerate() {
        for g2 in galaxies.iter().skip(n + 1) {
            let x1 = g % width;
            let y1 = g / width;
            let x2 = g2 % width;
            let y2 = g2 / width;
            sum += shortest_distance((x1, y1), (x2, y2));
        }
    }

    println!("Part 1 result: {}", sum);

    // Part 2: MILLIONS
    let galaxies = original_space
        .iter()
        .enumerate()
        .filter(|(_n, v)| *v == &'#')
        .map(|(n, _v)| n)
        .collect::<Vec<usize>>();

    let mut sum = 0;
    for (n, g) in galaxies.iter().enumerate() {
        for g2 in galaxies.iter().skip(n + 1) {
            let x1 = g % original_width;
            let y1 = g / original_width;
            let x2 = g2 % original_width;
            let y2 = g2 / original_width;
            sum += shortest_distance_with_expansion(
                (x1, y1),
                (x2, y2),
                &expanded_rows,
                &expanded_cols,
            );
        }
    }

    println!("Part 2 result: {}", sum);

    Ok(())
}

fn shortest_distance(from: (usize, usize), to: (usize, usize)) -> u64 {
    ((from.0 as i64 - to.0 as i64).abs() + (from.1 as i64 - to.1 as i64).abs()) as u64
}

fn shortest_distance_with_expansion(
    from: (usize, usize),
    to: (usize, usize),
    expanded_rows: &[usize],
    expanded_cols: &[usize],
) -> u64 {
    // add columns to x2
    let mut x1 = from.0;
    let mut x2 = to.0;
    expanded_cols
        .iter()
        .filter(|c| {
            if from.0 < to.0 {
                // going right
                *c > &from.0 && *c < &to.0
            } else {
                // going left
                *c < &from.0 && *c > &to.0
            }
        })
        .for_each(|_c| {
            if from.0 < to.0 {
                // going right
                x2 += 999_999;
            } else {
                // going left
                x1 += 999_999;
            }
        });

    // add rows to y2
    let mut y1 = from.1;
    let mut y2 = to.1;
    expanded_rows
        .iter()
        .filter(|c| {
            if from.1 < to.1 {
                // going down
                *c > &from.1 && *c < &to.1
            } else {
                // going up
                *c < &from.1 && *c > &to.1
            }
        })
        .for_each(|_c| {
            if from.1 < to.1 {
                // going down
                y2 += 999_999;
            } else {
                // going up
                y1 += 999_999;
            }
        });

    shortest_distance((x1, y1), (x2, y2))
}

fn draw_space(grid: &[char], width: &usize) {
    let mut row = 0;
    let length = grid.len();
    loop {
        println!("{}", grid[row..(row + width)].iter().collect::<String>());

        row += width;

        if row >= length {
            break;
        }
    }
}
