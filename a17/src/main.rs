use std::fs::File;
use std::io;
use std::io::BufRead;
use std::rc::Rc;

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
                    grid.push(c.to_string().parse::<u32>().unwrap());
                }
            }
            Err(_) => break,
        }
    }

    let previous = dijkstra(&grid, width, 0, grid.len() - 1);

    let mut p = previous[grid.len() - 1];
    let mut path = vec![grid.len() - 1];
    loop {
        match p {
            None => {
                break;
            }
            Some(n) => {
                path.push(n);
                p = previous[n];
            }
        }
    }

    path.reverse();
    println!("{:?}", path);

    let sum: u32 = path.iter().map(|i| grid[*i]).sum();

    println!("Part 1 result: {}", sum - grid[0]);

    let result = a_star(&grid, width, 0, grid.len() - 1);

    // println!("{:?}", n);

    for n in result {
        let mut sum = 0;
        let mut p = Some(Rc::new(n));
        loop {
            match &p {
                None => {
                    break;
                }
                Some(pv) => {
                    print!("{}, ", pv.position);
                    sum += grid[pv.position];
                    p = pv.parent.clone();
                }
            }
        }

        println!("Part 1 result: {}", sum - grid[0]);
    }

    Ok(())
}

fn min_distance(distances: &Vec<u32>, spt_set: &Vec<bool>) -> usize {
    let min = distances
        .iter()
        .enumerate()
        .filter(|(k, _v)| !spt_set[*k])
        .min_by_key(|(_k, v)| **v)
        .unwrap();

    min.0
}

fn get_neighbors(
    index: usize,
    graph: &[u32],
    width: usize,
    spt_set: &Vec<bool>,
    previous: &Vec<Option<usize>>,
) -> Vec<usize> {
    let mut result = vec![];

    let x = index % width;
    let y = index / width;
    let rows = graph.len() / width;

    if x == 0 {
        // right
        result.push(index + 1);
    } else if x == width - 1 {
        // left
        result.push(index - 1);
    } else {
        // both
        result.push(index + 1);
        result.push(index - 1);
    }

    if y == 0 {
        // down
        result.push(index + width);
    } else if y == rows - 1 {
        // up
        result.push(index - width);
    } else {
        // both
        result.push(index + width);
        result.push(index - width);
    }

    result = result
        .iter()
        .filter(|v| {
            if spt_set[**v] {
                return false;
            }

            match previous[index] {
                None => {
                    return true;
                }
                Some(p1) => {
                    // don't go back
                    if p1 == **v {
                        return false;
                    }

                    match previous[p1] {
                        None => {
                            return true;
                        }
                        Some(p2) => match previous[p2] {
                            None => {
                                return true;
                            }
                            Some(p3) => {
                                if (index % width == p1 % width
                                    && p1 % width == p2 % width
                                    && p2 % width == p3 % width
                                    && p3 % width == **v % width)
                                    || (index / width == p1 / width
                                        && p1 / width == p2 / width
                                        && p2 / width == p3 / width
                                        && p3 / width == **v / width)
                                {
                                    return false;
                                }
                            }
                        },
                    }
                }
            }

            true
        })
        .map(|v| *v)
        .collect::<Vec<usize>>();

    result
}

fn dijkstra(graph: &[u32], width: usize, start: usize, end: usize) -> Vec<Option<usize>> {
    let mut distances = vec![u32::MAX; graph.len()];
    let mut spt_set = vec![false; graph.len()];
    let mut previous: Vec<Option<usize>> = vec![None; graph.len()];

    distances[start] = 0;

    loop {
        let md = min_distance(&distances, &spt_set);

        // if md == end {
        //     break;
        // }

        spt_set[md] = true;

        let neighbors = get_neighbors(md, &graph, width, &spt_set, &previous);
        for n in neighbors.iter() {
            let alt = distances[md] + graph[*n];

            if alt < distances[*n] {
                distances[*n] = alt;
                previous[*n] = Some(md);
            }
        }

        if spt_set.iter().all(|v| *v == true) {
            break;
        }
    }

    previous
}

fn get_neighbors_a(node: &Node, graph: &[u32], width: usize) -> Vec<usize> {
    let mut result = vec![];

    let x = node.position % width;
    let y = node.position / width;
    let rows = graph.len() / width;

    if x == 0 {
        // right
        result.push(node.position + 1);
    } else if x == width - 1 {
        // left
        result.push(node.position - 1);
    } else {
        // both
        result.push(node.position + 1);
        result.push(node.position - 1);
    }

    if y == 0 {
        // down
        result.push(node.position + width);
    } else if y == rows - 1 {
        // up
        result.push(node.position - width);
    } else {
        // both
        result.push(node.position + width);
        result.push(node.position - width);
    }

    result = result
        .iter()
        .filter(|v| {
            match &node.parent {
                None => {
                    return true;
                }
                Some(p1) => {
                    // don't go back
                    if p1.position == **v {
                        return false;
                    }

                    match &p1.parent {
                        None => {
                            return true;
                        }
                        Some(p2) => match &p2.parent {
                            None => {
                                return true;
                            }
                            Some(p3) => {
                                if (node.position % width == p1.position % width
                                    && p1.position % width == p2.position % width
                                    && p2.position % width == p3.position % width
                                    && p3.position % width == **v % width)
                                    || (node.position / width == p1.position / width
                                        && p1.position / width == p2.position / width
                                        && p2.position / width == p3.position / width
                                        && p3.position / width == **v / width)
                                {
                                    return false;
                                }
                            }
                        },
                    }
                }
            }

            true
        })
        .map(|v| *v)
        .collect::<Vec<usize>>();

    result
}

fn get_distance(graph: &[u32], width: usize, source: usize, destination: usize) -> u32 {
    let x1 = source % width;
    let y1 = source / width;
    let x2 = destination % width;
    let y2 = destination / width;

    5 * ((x1 as i32 - x2 as i32).abs() + (y1 as i32 - y2 as i32).abs()) as u32
}

#[derive(Clone, Debug)]
struct Node {
    position: usize,
    f: u32,
    g: u32,
    h: u32,
    parent: Option<Rc<Node>>,
}

fn a_star(graph: &[u32], width: usize, start: usize, end: usize) -> Vec<Node> {
    let start_node = Node {
        position: start,
        f: 0,
        g: 0,
        h: 0,
        parent: None,
    };

    let mut results = vec![];

    let mut open = vec![start_node];
    let mut closed: Vec<Node> = vec![];

    let mut stop = false;
    loop {
        if open.is_empty() {
            break;
        }

        open.sort_by_key(|n| n.f);
        open.reverse();
        let q = open.pop().unwrap();

        let neighbors = get_neighbors_a(&q, &graph, width);
        for n in neighbors.iter() {
            let mut neighbor = Node {
                position: *n,
                f: 0,
                g: 0,
                h: 0,
                parent: Some(Rc::new(q.clone())),
            };

            if *n == end {
                results.push(neighbor.clone());
                stop = true;
                break;
            }

            neighbor.g = q.g + graph[*n];
            neighbor.h = get_distance(&graph, width, *n, end);
            neighbor.f = neighbor.g + neighbor.h;

            match open
                .iter()
                .find(|o| o.position == neighbor.position && o.f < neighbor.f)
            {
                None => {}
                Some(_) => {
                    continue;
                }
            }

            match closed
                .iter()
                .find(|o| o.position == neighbor.position && o.f < neighbor.f)
            {
                None => {}
                Some(_) => {
                    continue;
                }
            }

            open.push(neighbor);
        }

        closed.push(q);

        if stop {
            break;
        }
    }

    results
}
