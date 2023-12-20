use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;

#[derive(Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let re_workflow = Regex::new("([a-z]+)\\{(.+)\\}").unwrap();
    let re_rule = Regex::new("([xmas])([<>])([0-9]+):(.+)").unwrap();

    let mut is_workflows = true;

    let mut workflows = HashMap::new();
    let mut parts: Vec<Part> = vec![];

    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                if line.is_empty() {
                    is_workflows = false;
                    continue;
                }

                if is_workflows {
                    // parsing workflows
                    let wf = re_workflow
                        .captures(&line)
                        .unwrap()
                        .iter()
                        .map(|m| m.unwrap().as_str())
                        .collect::<Vec<&str>>();
                    let wf_name = wf[1].to_string();
                    let wf_rules = wf[2];

                    let rules: Vec<String> = wf_rules.split(',').map(|s| s.to_string()).collect();

                    workflows.insert(wf_name, rules);
                } else {
                    // parsing parts
                    let p = line
                        .replace(['{', '}'], "")
                        .split(',')
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>();

                    parts.push(Part {
                        x: p[0][2..].parse::<u32>().unwrap(),
                        m: p[1][2..].parse::<u32>().unwrap(),
                        a: p[2][2..].parse::<u32>().unwrap(),
                        s: p[3][2..].parse::<u32>().unwrap(),
                    })
                }
            }
            Err(_) => break,
        }
    }

    let mut sum = 0;
    for p in parts.iter() {
        sum += apply_workflows(&workflows, p, &re_rule);
    }

    println!("Part 1 result: {}", sum);

    let p2 = try_workflows(
        &workflows,
        &re_rule,
        &"in".to_string(),
        &[(1u32, 4000u32); 4],
    );

    println!("Part 2 result: {}", p2);

    Ok(())
}

fn apply_workflows(workflows: &HashMap<String, Vec<String>>, part: &Part, re_rule: &Regex) -> u32 {
    let mut out = 0;

    let mut wf_name = "in".to_string();
    let mut stop = false;
    loop {
        let rules = workflows.get(&wf_name).unwrap();

        for rule in rules.iter() {
            if rule.contains(':') {
                // <> rule
                let c = re_rule
                    .captures(rule)
                    .unwrap()
                    .iter()
                    .map(|c| c.unwrap().as_str())
                    .collect::<Vec<&str>>();

                let letter = c[1];
                let comparator = c[2];
                let value = c[3].parse::<u32>().unwrap();
                let result = c[4].to_string();

                let part_value = match letter {
                    "x" => part.x,
                    "m" => part.m,
                    "a" => part.a,
                    "s" => part.s,
                    _ => panic!("oh no"),
                };

                let yes = if comparator == ">" {
                    part_value > value
                } else {
                    part_value < value
                };

                if yes {
                    if result == "A" {
                        out = part.x + part.m + part.s + part.a;
                        stop = true;
                        break;
                    } else if result == "R" {
                        stop = true;
                        break;
                    } else {
                        wf_name = result.clone();
                        break;
                    }
                }
            } else if rule == "A" {
                out = part.x + part.m + part.s + part.a;
                stop = true;
                break;
            } else if rule == "R" {
                stop = true;
                break;
            } else {
                wf_name = rule.clone();
                break;
            }
        }

        if stop {
            break;
        }
    }

    out
}

fn try_workflows(
    workflows: &HashMap<String, Vec<String>>,
    re_rule: &Regex,
    wf_name: &String,
    ranges: &[(u32, u32); 4],
) -> u128 {
    let rules = workflows.get(wf_name).unwrap();

    let mut out: u128 = 0;

    let mut new_ranges = *ranges;

    // println!(
    //     "{} {}",
    //     wf_name,
    //     new_ranges
    //         .iter()
    //         .map(|(s, e)| (e - s) as u128 + 1)
    //         .reduce(|acc, v| acc * v)
    //         .unwrap()
    // );

    for rule in rules.iter() {
        if rule.contains(':') {
            // <> rule
            let c = re_rule
                .captures(rule)
                .unwrap()
                .iter()
                .map(|c| c.unwrap().as_str())
                .collect::<Vec<&str>>();

            let letter_index = match c[1] {
                "x" => 0,
                "m" => 1,
                "a" => 2,
                "s" => 3,
                _ => panic!("aaaa"),
            };

            let comparator = c[2];
            let value = c[3].parse::<u32>().unwrap();
            let result = c[4].to_string();

            let mut yes = false;
            let mut reversed_range = new_ranges[letter_index];
            if comparator == "<" {
                // compare with ranges[letter_index]
                if new_ranges[letter_index].0 < value {
                    yes = true;
                    new_ranges[letter_index].1 = value - 1;
                    reversed_range.0 = value;
                }
            } else if comparator == ">" {
                // compare with ranges[letter_index]
                if new_ranges[letter_index].1 > value {
                    yes = true;
                    new_ranges[letter_index].0 = value + 1;
                    reversed_range.1 = value;
                }
            }

            if yes {
                if result == "A" {
                    out += new_ranges
                        .iter()
                        .map(|(s, e)| (e - s) as u128 + 1)
                        .reduce(|acc, v| acc * v)
                        .unwrap();
                } else if result != "R" {
                    out += try_workflows(workflows, re_rule, &result, &new_ranges);
                }

                new_ranges[letter_index] = reversed_range;
            }
        } else if rule == "A" {
            out += new_ranges
                .iter()
                .map(|(s, e)| (e - s) as u128 + 1)
                .reduce(|acc, v| acc * v)
                .unwrap();
            break;
        } else if rule == "R" {
            break;
        } else {
            out += try_workflows(workflows, re_rule, rule, &new_ranges);
            break;
        }
    }

    out
}
