use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;

enum Pulse {
    Low(String),
    High(String),
}

trait Module<T> {
    fn create(name: &String, destinations: Vec<String>) -> T;
    fn accept_pulse(&mut self, pulse: Pulse) -> Option<(Pulse, Vec<String>)>;
}

struct FlipFlop {
    name: String,
    state: bool,
    destinations: Vec<String>,
}

impl Module<FlipFlop> for FlipFlop {
    fn create(name: &String, destinations: Vec<String>) -> FlipFlop {
        FlipFlop {
            name: name.clone(),
            state: false,
            destinations,
        }
    }

    fn accept_pulse(&mut self, pulse: Pulse) -> Option<(Pulse, Vec<String>)> {
        match pulse {
            Pulse::Low(_) => {
                self.state = !self.state;

                Some((
                    if self.state { Pulse::High(self.name.clone()) } else { Pulse::Low },
                    self.destinations.clone(),
                ))
            }
            Pulse::High(_) => None
        }
    }
}

struct Conjunction {
    inputs: HashMap<String, Pulse>
}

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {}
            Err(_) => break,
        }
    }

    Ok(())
}
