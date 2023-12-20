use std::fs::File;
use std::io;
use std::io::BufRead;

enum Pulse {
    Low,
    High,
}

trait Module<T> {
    fn create(destinations: Vec<String>) -> T;
    fn accept_pulse(&mut self, pulse: Pulse) -> Option<(Pulse, Vec<String>)>;
}

struct FlipFlop {
    state: bool,
    destinations: Vec<String>,
}

impl Module<FlipFlop> for FlipFlop {
    fn create(destinations: Vec<String>) -> FlipFlop {
        FlipFlop {
            state: false,
            destinations,
        }
    }

    fn accept_pulse(&mut self, pulse: Pulse) -> Option<(Pulse, Vec<String>)> {
        match pulse {
            Pulse::Low => {
                self.state = !self.state;

                Some((
                    if self.state { Pulse::High } else { Pulse::Low },
                    self.destinations.clone(),
                ))
            }
            Pulse::High => None
        }
    }
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
