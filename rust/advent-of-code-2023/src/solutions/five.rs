use std::{
    collections::HashMap,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    vec,
};

use regex::Regex;

pub struct InputRange {
    src_start: usize,
    dest_start: usize,
    range: usize,
}

impl Display for InputRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Source start {}, Destination start {}, with range {}",
            self.src_start, self.dest_start, self.range
        )
    }
}

impl Clone for InputRange {
    fn clone(&self) -> Self {
        InputRange {
            src_start: self.src_start,
            dest_start: self.dest_start,
            range: self.range,
        }
    }
}

pub fn read_maps_from_file() -> (Vec<usize>, Vec<Vec<InputRange>>) {
    let file = File::open("src/inputs/five.txt")
        .unwrap_or_else(|err| panic!("Cannot read input file with error{:?}", err));
    let reader = BufReader::new(file);

    let mut seeds: Vec<usize> = vec![];
    let mut maps: Vec<Vec<InputRange>> = vec![];
    let mut map_buffer: Vec<InputRange> = vec![];

    let mut step = 0;
    for line in reader.lines() {
        let current_line =
            line.unwrap_or_else(|err| panic!("Cannot read line with error {:?}", err));

        if current_line == "" {
            maps.push(map_buffer.to_vec());
            map_buffer.clear();
            continue;
        }

        if step == 0 {
            seeds = extract_seeds(&current_line);
            step += 1;
        } else {
            match extract_input_range(&current_line, 3) {
                Some(value) => {
                    map_buffer.push(value);
                }
                None => continue,
            };
        }
    }

    maps.push(map_buffer);
    return (seeds, maps);
}

fn extract_seeds(input: &str) -> Vec<usize> {
    let regex = Regex::new(r"\d+").unwrap_or_else(|err| panic!("Invalid regex {:?}", err));

    return regex
        .find_iter(input)
        .filter_map(|matched| matched.as_str().parse().ok())
        .collect();
}

fn extract_input_range(input: &str, limit: usize) -> Option<InputRange> {
    let regex = Regex::new(r"\d+").unwrap_or_else(|err| panic!("Invalid regex {:?}", err));

    let matched: Vec<usize> = regex
        .find_iter(input)
        .filter_map(|matched| matched.as_str().parse().ok())
        .collect();

    if matched.len() != limit {
        if matched.len() == 0 {
            return None;
        }

        panic!(
            "There should not {} values from an input line. Receive {}",
            limit,
            matched.len()
        );
    }

    Some(InputRange {
        src_start: matched[1],
        dest_start: matched[0],
        range: matched[2],
    })
}

pub fn min_location_number(seeds: &Vec<usize>, seed_maps: &Vec<Vec<InputRange>>) -> usize {
    let mut current_values = seeds.to_vec();

    for map in seed_maps {
        let mut changed: HashMap<usize, bool> = HashMap::new();
        for range in map {
            println!("{:?}", current_values);
            current_values = current_values
                .iter()
                .map(|value| {
                    if changed.contains_key(value) {
                        return *value;
                    }

                    if *value >= range.src_start && *value <= range.src_start + range.range - 1 {
                        let dest_value = range.dest_start + value - range.src_start;
                        changed.insert(dest_value, true);
                        return dest_value;
                    }

                    *value
                })
                .collect();
        }
        println!();
    }

    println!("{:?}", current_values);

    current_values.sort();
    return current_values[0];
}
