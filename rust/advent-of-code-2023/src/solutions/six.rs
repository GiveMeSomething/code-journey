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

pub fn count_win_ways(races: &Vec<Race>) -> usize {
    let mut result: Vec<usize> = vec![];
    for race in races {
        let mut count = 0;
        for i in 0..=race.time {
            if (race.time - i) * i > race.distance {
                count += 1;
            }
        }

        result.push(count);
    }

    let mut product = 1;
    for winning_ways in result {
        product *= winning_ways;
    }
    return product;
}

pub fn count_win_ways_single(races: &Vec<Race>) -> usize {
    let mut time_string = String::new();
    let mut distance_string = String::new();

    for race in races {
        time_string.push_str(race.time.to_string().as_str());
        distance_string.push_str(race.distance.to_string().as_str());
    }

    let time: usize = time_string.parse().unwrap_or_else(|err| {
        panic!(
            "Cannot parse {} to integer with error {:?}",
            time_string, err
        )
    });
    let distance = distance_string.parse().unwrap_or_else(|err| {
        panic!(
            "Cannot parse {} to integer with error {:?}",
            distance_string, err
        )
    });

    let mut counter = 0;
    for i in 0..time {
        if (time - i) * i > distance {
            counter += 1;
        }
    }

    return counter;
}
