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

        let cn = nodes.get(&current_node).unwrap();
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
    let mut current_nodes = find_ending_with_a(&nodes);

    println!("{:?}", current_nodes);

    dir_index = 0;
    steps = 0;

    while !do_all_end_in_z(&current_nodes) {
        if dir_index >= directions_length {
            dir_index = 0;
        }

        let mut new_current_nodes = current_nodes.clone();

        for (i, current_node) in current_nodes.iter().enumerate() {
            let cn = nodes.get(current_node).unwrap();
            new_current_nodes[i] = match directions[dir_index] {
                'L' => cn.0.to_string(),
                'R' => cn.1.to_string(),
                _ => panic!("wrong dir"),
            };
        }

        current_nodes = new_current_nodes;

        // println!("{:?}", current_nodes);

        steps += 1;
        dir_index += 1;
    }

    println!("Part 2 result: {}", steps);

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

fn do_all_end_in_z(nodes: &[String]) -> bool {
    nodes.iter().all(|v| {
        v.ends_with('Z')
    })
}