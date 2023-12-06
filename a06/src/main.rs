use regex::Regex;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::ops::Add;

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let re = Regex::new("([0-9]+)").unwrap();

    let mut line_nr = 0;

    let mut times = vec![];
    let mut distances = vec![];

    let mut part2time = 0;
    let mut part2distance = 0;

    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                let numbers = re
                    .find_iter(&line)
                    .map(|m| m.as_str().parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();

                if line_nr == 0 {
                    // times
                    times = numbers;

                    part2time = re
                        .find_iter(&line)
                        .map(|m| m.as_str().to_string())
                        .reduce(|acc, e| acc.add(e.as_str()))
                        .unwrap()
                        .parse::<u64>()
                        .unwrap();
                } else {
                    // distances
                    distances = numbers;

                    part2distance = re
                        .find_iter(&line)
                        .map(|m| m.as_str().to_string())
                        .reduce(|acc, e| acc.add(e.as_str()))
                        .unwrap()
                        .parse::<u64>()
                        .unwrap();
                }
            }
            Err(_) => break,
        }

        line_nr += 1;
    }

    let mut mul = 1;

    times.iter().enumerate().for_each(|(i, time)| {
        let distance = distances.get(i).unwrap();

        mul *= find_record_times(time, distance);
    });

    println!("Part 1 result: {}", mul);

    let found = find_record_times(&part2time, &part2distance);

    println!("Part 2 result: {}", found);

    Ok(())
}

fn find_record_times(race_time: &u64, min_distance: &u64) -> u64 {
    let mut found = 0;

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

    let d = (*race_time as f64).powf(2f64) as f64 - 4f64 * *min_distance as f64;
    let at_most = (*race_time as f64 + d.sqrt()) / 2f64;
    let at_least = (*race_time as f64 - d.sqrt()) / 2f64;

    for hold in at_least.ceil() as u64..=at_most.floor() as u64 {
        if hold * (race_time - hold) > *min_distance {
            found += 1;
        }
    }

    found
}
