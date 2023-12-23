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

    // let mut found = vec![];
    // let found_count = search(&grid, width, start, 6, &mut found);
    //
    // println!("{:?}", found_count);

    let mut edge_depth = 0;
    let mut cnt = 0;
    let mut at_least_one = false;
    let mut prev_edges = vec![start];
    loop {
        println!("depth {}", edge_depth);

        let current_edges = get_edges(&grid, width, start, edge_depth);

        for e in current_edges.iter() {
            for pe in prev_edges.iter() {
                if iddfs(&grid, width, *e, *pe, 6 - edge_depth) {
                    cnt += 1;
                    at_least_one = true;
                    break;
                }
            }
        }

        if !at_least_one && edge_depth > 4 {
            break;
        }

        at_least_one = false;
        edge_depth += 1;
    }

    println!("{}", cnt);

    Ok(())
}

fn get_edges(grid: &Vec<char>, width: usize, center: usize, depth: usize) -> Vec<usize> {
    if depth == 0 {
        get_neighbors(&grid, width, center, None)
    } else {
        let mut up = get_neighbors(&grid, width, center - depth * width, Some(center - depth * width + width));
        let mut down = get_neighbors(&grid, width, center + depth * width, Some(center + depth * width - width));
        let mut left = get_neighbors(&grid, width, center - depth, Some(center - depth + 1));
        let mut right = get_neighbors(&grid, width, center + depth, Some(center + depth - 1));

        up.append(&mut down);
        up.append(&mut left);
        up.append(&mut right);

        up
    }
}

fn get_neighbors(grid: &Vec<char>, width: usize, index: usize, skip: Option<usize>) -> Vec<usize> {
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
        .filter(|i| {
            let skipped = match skip {
                None => false,
                Some(s) => { **i == s }
            };

            ['.', 'S'].contains(&grid[**i]) && !skipped
        })
        .map(|i| *i)
        .collect()
}

fn iddfs(grid: &Vec<char>,
         width: usize,
         start: usize,
        goal: usize,
        max_depth: usize) -> bool {
    for d in 1..max_depth {
        if dls(&grid, width, start, goal, d) {
            return true;
        }
    }

    false
}

fn dls(grid: &Vec<char>,
       width: usize,
       start: usize,
       goal: usize,
       limit: usize) -> bool {
    if start == goal {
        return true;
    }

    if limit <= 0 {
        return false;
    }

    for n in get_neighbors(&grid, width, start, None).iter() {
        if dls(&grid, width, *n, goal, limit - 1) {
            return true;
        }
    }

    false
}

fn search(
    grid: &Vec<char>,
    width: usize,
    start: usize,
    depth: usize,
    found: &mut Vec<usize>,
) -> usize {
    if depth == 0 {
        return if !found.contains(&start) {
            found.push(start);
            1
        } else {
            0
        };
    }

    let mut result = 0;

    let neighbors = get_neighbors(&grid, width, start, None);
    for n in neighbors.iter() {
        result += search(&grid, width, *n, depth - 1, found);
    }

    result
}
