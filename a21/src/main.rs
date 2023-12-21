use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let mut width = 0;
    let mut grid = vec![];

    let mut start = 0;
    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                if width == 0 {
                    width = line.len();
                }

                for c in line.chars() {
                    grid.push(c);

                    if c == 'S' {
                        start = grid.len() - 1;
                    }
                }
            }
            Err(_) => break,
        }
    }

    // println!("{:?}", grid);

    let mut found = vec![];
    let found_count = search(&grid, width, start, 6, &mut found);

    println!("{:?}", found_count);

    Ok(())
}

fn get_neighbors(grid: &Vec<char>, width: usize, index: usize) -> Vec<usize> {
    let x = index % width;
    let y = index / width;

    let rows = grid.len() / width;

    let mut try_indexes = vec![];
    if x < width - 1 {
        try_indexes.push(index + 1);
    }

    if x > 0 {
        try_indexes.push(index - 1);
    }

    if y < rows - 1 {
        try_indexes.push(index + width);
    }

    if y > 0 {
        try_indexes.push(index - width);
    }

    try_indexes
        .iter()
        .filter(|i| ['.', 'S'].contains(&grid[**i]))
        .map(|i| *i)
        .collect()
}

fn search(grid: &Vec<char>, width: usize, start: usize, depth: usize, found: &mut Vec<usize>) -> usize {
    if depth == 0 {
        return if !found.contains(&start) {
            found.push(start);
            1
        } else {
            0
        }
    }

    let mut result = 0;

    let neighbors = get_neighbors(&grid, width, start);
    for n in neighbors.iter() {
        result += search(&grid, width, *n, depth - 1, found);
    }

    result
}
