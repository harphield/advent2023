use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;

#[derive(Clone, Debug)]
enum Pulse {
    Low(String),
    High(String),
}

trait Module {
    fn accept_pulse(&mut self, pulse: &Pulse) -> Option<(Pulse, Vec<String>)>;
}

struct FlipFlop {
    name: String,
    state: bool,
    destinations: Vec<String>,
}

impl FlipFlop {
    fn create(name: &str, destinations: Vec<String>) -> FlipFlop {
        FlipFlop {
            name: name.to_owned(),
            state: false,
            destinations,
        }
    }
}
impl Module for FlipFlop {
    fn accept_pulse(&mut self, pulse: &Pulse) -> Option<(Pulse, Vec<String>)> {
        match pulse {
            Pulse::Low(_) => {
                self.state = !self.state;

                Some((
                    if self.state {
                        Pulse::High(self.name.clone())
                    } else {
                        Pulse::Low(self.name.clone())
                    },
                    self.destinations.clone(),
                ))
            }
            Pulse::High(_) => None,
        }
    }
}

struct Conjunction {
    name: String,
    inputs: HashMap<String, Pulse>,
    destinations: Vec<String>,
}

impl Conjunction {
    fn create(name: &str, destinations: Vec<String>) -> Conjunction {
        Conjunction {
            name: name.to_owned(),
            inputs: HashMap::new(),
            destinations,
        }
    }

    fn set_inputs(&mut self, inputs: Vec<String>) {
        for i in inputs.iter() {
            self.inputs.insert(i.clone(), Pulse::Low(i.clone()));
        }
    }
}

impl Module for Conjunction {
    fn accept_pulse(&mut self, pulse: &Pulse) -> Option<(Pulse, Vec<String>)> {
        let p = pulse.clone();
        match pulse {
            Pulse::Low(from) | Pulse::High(from) => {
                let f = from.clone();
                *self
                    .inputs
                    .entry(f.clone())
                    .or_insert(Pulse::Low(f.clone())) = p;
            }
        }

        if self.inputs.iter().all(|(_k, v)| match v {
            Pulse::Low(_) => false,
            Pulse::High(_) => true,
        }) {
            Some((Pulse::Low(self.name.clone()), self.destinations.clone()))
        } else {
            Some((Pulse::High(self.name.clone()), self.destinations.clone()))
        }
    }
}

struct Broadcast {
    name: String,
    destinations: Vec<String>,
}

impl Broadcast {
    fn create(name: &str, destinations: Vec<String>) -> Broadcast {
        Broadcast {
            name: name.to_owned(),
            destinations,
        }
    }
}

impl Module for Broadcast {
    fn accept_pulse(&mut self, pulse: &Pulse) -> Option<(Pulse, Vec<String>)> {
        let p = match pulse {
            Pulse::Low(_) => Pulse::Low(self.name.clone()),
            Pulse::High(_) => Pulse::High(self.name.clone()),
        };

        Some((p.clone(), self.destinations.clone()))
    }
}

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let mut modules_except_cons: HashMap<String, Box<dyn Module>> = HashMap::new();
    let mut conjunctions: HashMap<String, Conjunction> = HashMap::new();
    let mut out_to_in = HashMap::new();

    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                let s = line.split(" -> ").collect::<Vec<&str>>();
                let destinations: Vec<String> = s[1].split(", ").map(|v| v.to_string()).collect();
                let dest_to_save = destinations.clone();

                let name = if s[0] == "broadcaster" {
                    modules_except_cons.insert(
                        s[0].to_string(),
                        Box::new(Broadcast::create(s[0], destinations)),
                    );
                    "broadcaster"
                } else if s[0].starts_with('%') {
                    // flipflop
                    let name = &s[0][1..];
                    modules_except_cons.insert(
                        name.to_string(),
                        Box::new(FlipFlop::create(name, destinations)),
                    );
                    name
                } else if s[0].starts_with('&') {
                    // conjunction
                    let name = &s[0][1..];
                    conjunctions.insert(
                        name.to_string(),
                        Conjunction::create(name, destinations),
                    );
                    name
                } else {
                    panic!("oh no");
                };

                out_to_in.insert(name.to_string(), dest_to_save);
            }
            Err(_) => break,
        }
    }

    for (c_name, c) in conjunctions.iter_mut() {
        c.set_inputs(
            out_to_in
                .iter()
                .filter(|(_k, v)| v.contains(c_name))
                .map(|(k, _v)| k.clone())
                .collect(),
        );
    }

    let mut low_pulses = 0;
    let mut high_pulses = 0;

    let mut rx_hit_presses = 0;

    for i in 0..1000 {
        let mut queue = vec![(Pulse::Low("".to_string()), "broadcaster".to_string())];
        low_pulses += 1;

        let mut rx_high_hits = 0;
        let mut rx_low_hits = 0;

        loop {
            // println!("{:?}", queue);
            if queue.is_empty() {
                break;
            }

            let (pulse, destination) = queue.pop().unwrap();
            match modules_except_cons.get_mut(&destination) {
                None => match conjunctions.get_mut(&destination) {
                    None => {
                        if destination == "rx" {
                            match pulse {
                                Pulse::Low(_) => {
                                    rx_low_hits += 1;
                                }
                                Pulse::High(_) => {
                                    rx_high_hits += 1;
                                }
                            }
                        }
                    }
                    Some(c) => match c.accept_pulse(&pulse) {
                        None => {}
                        Some((r_pulse, r_destinations)) => {
                            for d in r_destinations.iter() {
                                queue.insert(0, (r_pulse.clone(), d.clone()));

                                match r_pulse {
                                    Pulse::Low(_) => {
                                        low_pulses += 1;
                                    }
                                    Pulse::High(_) => {
                                        high_pulses += 1;
                                    }
                                }
                            }
                        }
                    },
                },
                Some(module) => match module.accept_pulse(&pulse) {
                    None => {}
                    Some((r_pulse, r_destinations)) => {
                        for d in r_destinations.iter() {
                            queue.insert(0, (r_pulse.clone(), d.clone()));

                            match r_pulse {
                                Pulse::Low(_) => {
                                    low_pulses += 1;
                                }
                                Pulse::High(_) => {
                                    high_pulses += 1;
                                }
                            }
                        }
                    }
                },
            }
        }

        if rx_low_hits == 1 && rx_hit_presses == 0 {
            rx_hit_presses = i;
        } else {
            // println!("rx low {} high {}", rx_low_hits, rx_high_hits);
        }
    }

    // println!("{} high {} low", high_pulses, low_pulses);
    println!("Part 1 result: {}", high_pulses * low_pulses);
    println!("Part 2 result: {}", rx_hit_presses);

    Ok(())
}
