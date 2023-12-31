use std::{
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

pub struct Race {
    time: usize,
    distance: usize,
}

impl Debug for Race {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Time: {}; Distance: {}", self.time, self.distance)
    }
}

pub fn read_races_from_file() -> Vec<Race> {
    let file = File::open("src/inputs/six.txt")
        .unwrap_or_else(|err| panic!("Cannot read day 6 input file with error {:?}", err));
    let reader = BufReader::new(file);

    // Read Time: line
    let mut counter = 0;
    let mut times: Vec<usize> = vec![];
    let mut distances: Vec<usize> = vec![];
    for line in reader.lines() {
        let current_line =
            line.unwrap_or_else(|err| panic!("Cannot read line {} with error {:?}", counter, err));
        if counter == 0 {
            times = extract_numbers(&current_line);
        }
        if counter == 1 {
            distances = extract_numbers(&current_line);
        }
        counter += 1;
    }

    if times.len() != distances.len() {
        panic!(
            "Time: {:?}\nDistance: {:?}\nLength mismatch, unable to continue",
            times, distances
        );
    }

    let mut result: Vec<Race> = vec![];
    for i in 0..times.len() {
        result.push(Race {
            time: times[i],
            distance: distances[i],
        });
    }
    return result;
}

fn extract_numbers(input: &str) -> Vec<usize> {
    let regex = Regex::new(r"\d+").expect("Expect a valid regex");
    regex
        .find_iter(input)
        .map(|matched| {
            matched.as_str().parse().unwrap_or_else(|err| {
                panic!(
                    "Cannot parse {} to an integer with error {:?}",
                    matched.as_str(),
                    err
                )
            })
        })
        .collect()
}
