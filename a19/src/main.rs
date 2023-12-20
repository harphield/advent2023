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

    println!("{:?}", parts);

    let mut sum = 0;
    for p in parts.iter() {
        sum += apply_workflows(&workflows, p, &re_rule);
    }

    println!("Part 1 result: {}", sum);

    Ok(())
}

fn apply_workflows(workflows: &HashMap<String, Vec<String>>, part: &Part, re_rule: &Regex) -> u32 {
    println!("part {}", part.x);

    let mut out = 0;

    let mut wf_name = "in".to_string();
    let mut stop = false;
    loop {
        println!("wf: {}", wf_name);
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
                println!("{:?}", c);
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

fn try_workflows(workflows: &HashMap<String, Vec<String>>, re_rule: &Regex, wf_name: &String) -> u64 {
    let rules = workflows.get(wf_name).unwrap();

    let out = 0;

    for rule in rules.iter() {
        if rule.contains(':') {
            // <> rule
            let c = re_rule
                .captures(rule)
                .unwrap()
                .iter()
                .map(|c| c.unwrap().as_str())
                .collect::<Vec<&str>>();

            let comparator = c[2];
            let value = c[3].parse::<u32>().unwrap();
            let result = c[4].to_string();

            // TODO comparator stuff
            // TODO 1 to value and value to 4000

            if result == "A" {
                break;
            } else if result == "R" {
                return 0;
            } else {
                try_workflows(&workflows, &re_rule, &result);
            }
        } else if rule == "A" {
            break;
        } else if rule == "R" {
            break;
        } else {
            try_workflows(&workflows, &re_rule, &rule);
            break;
        }
    }

    out
}
