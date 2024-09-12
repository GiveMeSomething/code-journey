use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    vec,
};

use regex::Regex;

use super::{input_range::InputRange, seed::SeedRange};

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

pub fn min_location_seeds_range(seeds: &Vec<usize>, seed_maps: &Vec<Vec<InputRange>>) -> usize {
    println!("{:?}", seeds);

    let mut min = usize::MAX;

    for i in 0..(seeds.len() / 2) {
        let start = seeds[i * 2];
        let range = seeds[i * 2 + 1];

        println!("index {} start {} range {}", i, start, range);

        let mut current_seeds = vec![];
        for j in start..start + range {
            current_seeds.push(j);
        }

        let current_min = min_location_seed(&current_seeds, seed_maps);

        println!("Current min {} -> Next min {} ", min, current_min);

        if current_min < min {
            min = current_min;
        }
    }

    return min;
}

pub fn min_location_seed(seeds: &Vec<usize>, seed_maps: &Vec<Vec<InputRange>>) -> usize {
    let mut current_values = seeds.to_vec();

    for map in seed_maps {
        let mut changed: HashMap<usize, bool> = HashMap::new();
        for range in map {
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
    }

    current_values.sort();
    return current_values[0];
}
