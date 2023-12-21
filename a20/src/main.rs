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
                    conjunctions.insert(name.to_string(), Conjunction::create(name, destinations));
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

    let mut tx_high = 0;
    let mut dd_high = 0;
    let mut nz_high = 0;
    let mut ph_high = 0;

    let mut i = 0;
    let mut stop = false;
    loop {
        let mut queue = vec![(Pulse::Low("".to_string()), "broadcaster".to_string())];
        if i < 1000 {
            low_pulses += 1;
        }

        loop {
            // println!("{:?}", queue);
            if queue.is_empty() {
                break;
            }

            let (pulse, destination) = queue.pop().unwrap();
            match modules_except_cons.get_mut(&destination) {
                None => match conjunctions.get_mut(&destination) {
                    None => {}
                    Some(c) => match c.accept_pulse(&pulse) {
                        None => {}
                        Some((r_pulse, r_destinations)) => {
                            for d in r_destinations.iter() {
                                queue.insert(0, (r_pulse.clone(), d.clone()));

                                match &r_pulse {
                                    Pulse::Low(_) => {
                                        if i < 1000 {
                                            low_pulses += 1;
                                        }
                                    }
                                    Pulse::High(from) => {
                                        if d == "ls" {
                                            // all of these need to send HIGH at the same time...
                                            if from == "tx" && tx_high == 0 {
                                                tx_high = i + 1;
                                            } else if from == "dd" && dd_high == 0 {
                                                dd_high = i + 1;
                                            } else if from == "nz" && nz_high == 0 {
                                                nz_high = i + 1;
                                            } else if from == "ph" && ph_high == 0 {
                                                ph_high = i + 1;
                                            } else if i > 1000 {
                                                stop = true;
                                                break;
                                            }
                                        }

                                        if i < 1000 {
                                            high_pulses += 1;
                                        }
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
                                    if i < 1000 {
                                        low_pulses += 1;
                                    }
                                }
                                Pulse::High(_) => {
                                    if i < 1000 {
                                        high_pulses += 1;
                                    }
                                }
                            }
                        }
                    }
                },
            }
        }

        i += 1;

        if stop {
            break;
        }
    }

    println!("Part 1 result: {}", high_pulses * low_pulses);
    println!(
        "Part 2 result: {} {} {} {} - now find the least common multiple",
        tx_high, ph_high, dd_high, nz_high
    );

    Ok(())
}
