use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;

type Nodes = HashMap<String, (String, String)>;

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let mut directions = vec![];
    let mut nodes = Nodes::new();
    let re = Regex::new("([0-9A-Z]+)").unwrap();

    for (n, line_r) in io::BufReader::new(file).lines().enumerate() {
        match line_r {
            Ok(line) => {
                if n == 0 {
                    // directions
                    directions = line.chars().collect::<Vec<char>>();
                } else if !line.is_empty() {
                    let row = re
                        .find_iter(&line)
                        .map(|m| m.as_str().to_string())
                        .collect::<Vec<String>>();

                    nodes.insert(row[0].clone(), (row[1].clone(), row[2].clone()));
                }
            }
            Err(_) => break,
        }
    }

    // Part 1
    let mut current_node = "AAA".to_string();
    let mut dir_index = 0usize;
    let directions_length = directions.len();
    let mut steps = 0;
    while !current_node.eq("ZZZ") {
        if dir_index >= directions_length {
            dir_index = 0;
        }

        let cn = match nodes.get(&current_node) {
            None => panic!("why? {}", current_node),
            Some(v) => v,
        };
        current_node = match directions[dir_index] {
            'L' => cn.0.to_string(),
            'R' => cn.1.to_string(),
            _ => panic!("wrong dir"),
        };

        steps += 1;
        dir_index += 1;
    }

    println!("Part 1 result: {}", steps);

    // Part 2
    let current_nodes = find_ending_with_a(&nodes);

    println!("{:?}", current_nodes);

    let mut z_steps = current_nodes.iter().map(|_v| 0).collect::<Vec<u64>>();

    for i in 0..current_nodes.len() {
        z_steps[i] += steps_to_closest_z_from_node(&nodes, &directions, &current_nodes[i], 0);
    }

    println!(
        "Part 2 result: {:?} and do Least Common Multiple of these, lol",
        z_steps
    );

    Ok(())
}

fn find_ending_with_a(nodes: &Nodes) -> Vec<String> {
    let mut result = vec![];

    for (k, _v) in nodes.iter() {
        if k.ends_with('A') {
            result.push(k.clone());
        }
    }

    result
}

fn steps_to_closest_z_from_node(
    nodes: &Nodes,
    directions: &[char],
    node: &str,
    starting_direction: usize,
) -> u64 {
    let directions_length = directions.len();
    let mut cn = node.to_string();
    let mut dir_index = starting_direction;
    let mut steps = 0;

    // first step must always go
    while steps == 0 || !cn.ends_with('Z') {
        if dir_index >= directions_length {
            dir_index = 0;
        }

        let cnd = nodes.get(&cn).unwrap();
        cn = match directions[dir_index] {
            'L' => cnd.0.to_string(),
            'R' => cnd.1.to_string(),
            _ => panic!("wrong dir"),
        };

        dir_index += 1;
        steps += 1;
    }

    steps
}
