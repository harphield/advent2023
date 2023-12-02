use std::fs::File;
use std::io;
use std::io::BufRead;
use regex::Regex;

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let re_game = Regex::new("Game ([0-9]+): (.+)").unwrap();
    let re_reveal = Regex::new("([0-9]+) ([^,;]+)").unwrap();

    let mut sum = 0;
    let mut power_sum = 0;

    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                let captures = re_game.captures(&line).unwrap();
                let game_id = captures.get(1).unwrap().as_str().to_string().parse::<u32>().unwrap();
                let game_data = captures.get(2).unwrap().as_str();

                let split = game_data.split(";");

                let mut ok = true;

                // R, G, B
                let mut maxes: [u32; 3] = [0; 3];

                for s in split {
                    for cap in re_reveal.captures_iter(s) {
                        let color = cap.get(2).unwrap().as_str();
                        let value = cap.get(1).unwrap().as_str().to_string().parse::<u32>().unwrap();

                        if color.eq("red") && value > 12 ||
                            color.eq("green") && value > 13 ||
                            color.eq("blue") && value > 14 {
                            ok = false;
                        }

                        if color.eq("red") && value >= maxes[0] {
                            maxes[0] = value;
                        } else if color.eq("green") && value >= maxes[1] {
                            maxes[1] = value;
                        } else if color.eq("blue") && value >= maxes[2] {
                            maxes[2] = value;
                        }

                    }
                }

                if ok {
                    sum += game_id;
                }

                power_sum += maxes[0] * maxes[1] * maxes[2];
            }
            Err(_) => break
        }
    }

    println!("Part 1 result: {}", sum);
    println!("Part 2 result: {}", power_sum);

    Ok(())
}