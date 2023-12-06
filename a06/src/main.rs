use std::fs::File;
use std::io;
use std::io::BufRead;
use regex::Regex;

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let re = Regex::new("([0-9]+)").unwrap();

    let mut line_nr = 0;

    let mut times = vec![];
    let mut distances = vec![];

    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                let numbers = re.find_iter(&line).map(|m| {
                    m.as_str().parse::<u32>().unwrap()
                }).collect::<Vec<u32>>();

                if line_nr == 0 {
                    // times
                    times = numbers;
                } else {
                    // distances
                    distances = numbers;
                }
            }
            Err(_) => break
        }

        line_nr += 1;
    }

    let mut mul = 1;

    times.iter().enumerate().for_each(|(i, time)| {
        let distance = distances.get(i).unwrap();

        let found = find_record_times(time, distance);
        mul *= found.len();
    });

    println!("Part 1 result: {}", mul);

    Ok(())
}

fn find_record_times(race_time: &u32, min_distance: &u32) -> Vec<u32> {
    let mut found = vec![];

    // distance = time_held * time_traveled
    // race_time = time_held + time_traveled
    // time_traveled = race_time - time_held
    // distance = time_held * (race_time - time_held)
    // distance = -time_held^2 + race_time*time_held
    // time_held^2 - race_time*time_held + distance = 0

    // quadratic equation!
    // x^2 - race_time*x + distance = 0
    // D = (-race_time)^2 - 4*distance
    // x1 = (race_time + sqrt(D)) / 2
    // x2 = (race_time - sqrt(D)) / 2

    let d = race_time.pow(2) as f32 - 4f32 * *min_distance as f32;
    let at_most = (*race_time as f32 + d.sqrt()) / 2f32;
    let at_least = (*race_time as f32 - d.sqrt()) / 2f32;

    for hold in at_least.ceil() as u32..=at_most.floor() as u32 {
        if hold * (race_time - hold) > *min_distance {
            found.push(hold);
        }
    }

    found
}