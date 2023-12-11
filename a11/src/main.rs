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
            Err(_) => break
        }
    }

    // add rows
    let mut new_space = space.clone();

    for i in (0..(new_space.len() / width)).rev() {
        if new_space.iter().enumerate().filter(|(n, _v)| {
            n / width == i
        }).all(|(_n, v)| *v == '.') {
            for _x in 0..width {
                new_space.insert(i * width, '.');
            }
        }
    }

    space = new_space;

    // add columns
    let mut new_space = space.clone();

    let mut new_width = width;
    for i in (0..new_width).rev() {
        if new_space.iter().enumerate().filter(|(n, _v)| {
            n % new_width == i
        }).all(|(_n, v)| *v == '.') {
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
    let galaxies = space.iter().enumerate().filter(|(_n, v)| *v == &'#').map(|(n, _v)| n ).collect::<Vec<usize>>();

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

    Ok(())
}

fn shortest_distance(from: (usize, usize), to: (usize, usize)) -> u32 {
    ((from.0 as i32 - to.0 as i32).abs() + (from.1 as i32 - to.1 as i32).abs()) as u32
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