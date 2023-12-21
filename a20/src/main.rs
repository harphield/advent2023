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
    fn create(name: &String, destinations: Vec<String>) -> FlipFlop {
        FlipFlop {
            name: name.clone(),
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
                    if self.state { Pulse::High(self.name.clone()) } else { Pulse::Low(self.name.clone()) },
                    self.destinations.clone(),
                ))
            }
            Pulse::High(_) => None
        }
    }
}

struct Conjunction {
    name: String,
    inputs: HashMap<String, Pulse>,
    destinations: Vec<String>,
}

impl Conjunction {
    fn create(name: &String, destinations: Vec<String>) -> Conjunction {
        Conjunction {
            name: name.clone(),
            inputs: HashMap::new(),
            destinations,
        }
    }
}

impl Module for Conjunction {
    fn accept_pulse(&mut self, pulse: &Pulse) -> Option<(Pulse, Vec<String>)> {
        let p = pulse.clone();
        match pulse {
            Pulse::Low(from) | Pulse::High(from) => {
                let f = from.clone();
                *self.inputs.entry(f.clone()).or_insert(Pulse::Low(f.clone())) = p;
            }
        }

        if self.inputs.iter().all(|(_k, v)| {
            match v {
                Pulse::Low(_) => false,
                Pulse::High(_) => true
            }
        }) {
            Some((Pulse::High(self.name.clone()), self.destinations.clone()))
        } else {
            Some((Pulse::Low(self.name.clone()), self.destinations.clone()))
        }
    }
}

struct Broadcast {
    name: String,
    destinations: Vec<String>,
}

impl Broadcast {
    fn create(name: &String, destinations: Vec<String>) -> Broadcast {
        Broadcast {
            name: name.clone(),
            destinations,
        }
    }
}

impl Module for Broadcast {
    fn accept_pulse(&mut self, pulse: &Pulse) -> Option<(Pulse, Vec<String>)> {
        Some((pulse.clone(), self.destinations.clone()))
    }
}

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let mut config: HashMap<String, Box<dyn Module>> = HashMap::new();

    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                let s = line.split(" -> ").collect::<Vec<&str>>();
                let destinations = s[1].split(", ").map(|v| v.to_string()).collect();

                if s[0] == "broadcaster" {
                    config.insert(s[0].to_string(), Box::new(Broadcast::create(&s[0].to_string(), destinations)));
                } else if s[0].starts_with('%') {
                    // flipflop
                    let name = &s[0][1..];
                    config.insert(name.to_string(), Box::new(FlipFlop::create(&name.to_string(), destinations)));
                } else if s[0].starts_with('&') {
                    let name = &s[0][1..];
                    config.insert(name.to_string(), Box::new(Conjunction::create(&name.to_string(), destinations)));
                }
            }
            Err(_) => break,
        }
    }

    config.iter().for_each(|(k, v)| {
        println!("{}", k);
    });

    let mut queue = vec![(Pulse::Low("".to_string()), "broadcaster".to_string())];

    let mut low_pulses = 1;  // the default push
    let mut high_pulses = 0;
    loop {
        if queue.is_empty() {
            break;
        }

        let (pulse, destination) = queue.pop().unwrap();
        match config.get_mut(&destination) {
            None => {}
            Some(module) => {
                match module.accept_pulse(&pulse) {
                    None => {}
                    Some((r_pulse, r_destinations)) => {
                        for d in r_destinations.iter() {
                            queue.insert(0, (r_pulse.clone(), d.clone()));

                            match r_pulse {
                                Pulse::Low(_) => { low_pulses += 1; }
                                Pulse::High(_) => { high_pulses += 1; }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{} high {} low", high_pulses, low_pulses);

    Ok(())
}
