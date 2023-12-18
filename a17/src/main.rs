use std::cmp::min;
use std::collections::HashMap;
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

    // println!("{:?}", previous);

    let mut ps = vec![];

    for (node, prev_node) in previous.iter() {
        if node.0 == grid.len() - 1 {
            ps.push(prev_node);
        }
    }

    let mut sum = u32::MAX;
    for p_i in ps.iter() {
        let mut path = vec![grid.len() - 1];
        let mut p = p_i.clone();
        loop {
            path.push(p.0);
            p = match previous.get(p) {
                None => {
                    break;
                }
                Some(pr) => pr,
            };
        }

        path.reverse();
        println!("{:?}", path);

        sum = min(sum, path.iter().map(|i| grid[*i]).sum());
    }

    println!("Part 1 result: {}", sum - grid[0]);

    // let result = a_star(&grid, width, 0, grid.len() - 1);

    // println!("{:?}", n);

    // for n in result {
    //     let mut sum = 0;
    //     let mut p = Some(Rc::new(n));
    //     loop {
    //         match &p {
    //             None => {
    //                 break;
    //             }
    //             Some(pv) => {
    //                 print!("{}, ", pv.position);
    //                 sum += grid[pv.position];
    //                 p = pv.parent.clone();
    //             }
    //         }
    //     }
    //
    //     println!("Part 1 result: {}", sum - grid[0]);
    // }

    // println!("Part 1 result: {}", result);

    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

fn min_distance(distances: &Distances, spt_set: &Closed) -> ((usize, u8, Direction), u32) {
    let min = distances
        .iter()
        .filter(|(k, _v)| !spt_set.contains_key(*k))
        .min_by_key(|(_k, v)| **v)
        .unwrap();

    (min.0.clone(), *min.1)
}

fn get_neighbors(
    index: &(usize, u8, Direction),
    graph: &[u32],
    width: usize,
    spt_set: &Closed,
    previous: &Previous,
) -> Vec<(usize, u8, Direction)> {
    let mut result = vec![];

    let x = index.0 % width;
    let y = index.0 / width;
    let rows = graph.len() / width;

    let prev = previous.get(&index);

    if x == 0 {
        // right
        result.push((
            index.0 + 1,
            match index.2 {
                Direction::Right => index.1 + 1,
                _ => 0,
            },
            Direction::Right,
        ));
    } else if x == width - 1 {
        // left
        result.push((
            index.0 - 1,
            match index.2 {
                Direction::Left => index.1 + 1,
                _ => 0,
            },
            Direction::Left,
        ));
    } else {
        // both
        result.push((
            index.0 + 1,
            match index.2 {
                Direction::Right => index.1 + 1,
                _ => 0,
            },
            Direction::Right,
        ));
        result.push((
            index.0 - 1,
            match index.2 {
                Direction::Left => index.1 + 1,
                _ => 0,
            },
            Direction::Left,
        ));
    }

    if y == 0 {
        // down
        result.push((
            index.0 + width,
            match index.2 {
                Direction::Down => index.1 + 1,
                _ => 0,
            },
            Direction::Down,
        ));
    } else if y == rows - 1 {
        // up
        result.push((
            index.0 - width,
            match index.2 {
                Direction::Up => index.1 + 1,
                _ => 0,
            },
            Direction::Up,
        ));
    } else {
        // both
        result.push((
            index.0 + width,
            match index.2 {
                Direction::Down => index.1 + 1,
                _ => 0,
            },
            Direction::Down,
        ));
        result.push((
            index.0 - width,
            match index.2 {
                Direction::Up => index.1 + 1,
                _ => 0,
            },
            Direction::Up,
        ));
    }

    result = result
        .iter()
        .filter(|v| {
            if v.1 >= 3 {
                return false;
            }

            match spt_set.get(*v) {
                None => {}
                Some(_) => {
                    return false;
                }
            }

            match prev {
                None => {
                    return true;
                }
                Some(p1) => {
                    // don't go back
                    if p1.0 == v.0 {
                        return false;
                    }
                }
            }

            true
        })
        .map(|v| v.clone())
        .collect::<Vec<(usize, u8, Direction)>>();

    result
}

type Distances = HashMap<(usize, u8, Direction), u32>;
type Closed = HashMap<(usize, u8, Direction), bool>;
type Previous = HashMap<(usize, u8, Direction), (usize, u8, Direction)>;

fn dijkstra(graph: &[u32], width: usize, start: usize, end: usize) -> Previous {
    // position, step count, direction (0 = north, 1 = west, 2 = south, 3 = east)
    let mut distances: Distances = Distances::new();
    let mut spt_set: Closed = Closed::new();
    let mut previous: Previous = Previous::new();

    distances.insert((0, 0, Direction::Right), 0);
    distances.insert((0, 0, Direction::Down), 0);

    loop {
        let md = min_distance(&distances, &spt_set);

        if md.0 .0 == end {
            break;
        }

        spt_set.insert(md.0.clone(), true);

        let neighbors = get_neighbors(&md.0, &graph, width, &spt_set, &previous);
        for n in neighbors.iter() {
            let alt = md.1 + graph[n.0];

            let mut n_entry = distances.entry(n.clone()).or_insert(u32::MAX);

            if &alt < n_entry {
                *n_entry = alt;
                previous.insert(n.clone(), md.0.clone());
            }
        }

        // if spt_set.iter().all(|(_k, v)| v == true) {
        //     break;
        // }
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
                None => {}
                Some(p) => {
                    if p.position == **v {
                        return false;
                    }
                }
            }

            if (node.straight_count.0 == 3 && (node.position % width == **v % width))
                || (node.straight_count.1 == 3 && (node.position / width == **v / width))
            {
                return false;
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
    straight_count: (u8, u8),
    f: u32,
    g: u32,
    h: u32,
    parent: Option<Rc<Node>>,
}

fn a_star(graph: &[u32], width: usize, start: usize, end: usize) -> u32 {
    let start_node = Node {
        position: start,
        straight_count: (1, 1),
        f: 0,
        g: 0,
        h: 0,
        parent: None,
    };

    // let mut results = vec![];

    let mut open = vec![start_node];
    let mut closed: Vec<Node> = vec![];

    let mut stop = false;

    let mut min_length = u32::MAX;

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
                straight_count: {
                    match &q.parent {
                        None => {
                            if q.position == start {
                                (1, 1)
                            } else {
                                (0, 0)
                            }
                        }
                        Some(p) => {
                            if (p.position % width == q.position % width)
                                && (q.position % width == *n % width)
                            {
                                (q.straight_count.0 + 1, 0)
                            } else if (p.position / width == q.position / width)
                                && (q.position / width == *n / width)
                            {
                                (0, q.straight_count.1 + 1)
                            } else {
                                (1, 1)
                            }
                        }
                    }
                },
                f: 0,
                g: 0,
                h: 0,
                parent: Some(Rc::new(q.clone())),
            };

            if *n == end {
                // results.push(neighbor.clone());
                min_length = min(min_length, q.g + graph[neighbor.position]);

                // continue;
                stop = true;
                break;
            }

            neighbor.g = q.g + graph[*n];
            neighbor.h = get_distance(&graph, width, *n, end);
            neighbor.f = neighbor.g + neighbor.h;

            match open.iter().find(|o| {
                o.position == neighbor.position
                    && o.straight_count == neighbor.straight_count
                    && o.f < neighbor.f
            }) {
                None => {}
                Some(_) => {
                    continue;
                }
            }

            match closed.iter().find(|o| {
                o.position == neighbor.position
                    && o.straight_count == neighbor.straight_count
                    && o.f < neighbor.f
            }) {
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

    min_length
}
