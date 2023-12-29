use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
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

pub fn read_maps_from_file() {
    let file = File::open("src/inputs/five.txt")
        .unwrap_or_else(|err| panic!("Cannot read input file with error{:?}", err));
    let reader = BufReader::new(file);

    // Step 0: Read the seeds
    // Step 1: Read seed-to-soil map
    // Step 2: Read soil-to-fertilizer map
    // Step 3: Read fertilizer-to-water map
    // Step 4: Read water-to-light map
    // Step 5: Read light-to-temparate map
    // Step 6: Read temparature-to-humidity map
    // Step 7: Read humidity-to-location map
    let mut step = 0;
    for line in reader.lines() {
        let current_line =
            line.unwrap_or_else(|err| panic!("Cannot read line with error {:?}", err));

        println!("{:?}", extract_seeds(current_line.as_str()));
        break;
    }
}

fn extract_seeds(input: &str) -> Vec<usize> {
    let regex = Regex::new(r"\d+").unwrap_or_else(|err| panic!("Invalid regex {:?}", err));

    return regex
        .find_iter(input)
        .filter_map(|matched| matched.as_str().parse().ok())
        .collect();
}

fn extract_input_range(input: &str, limit: usize) -> InputRange {
    let regex = Regex::new(r"\d+").unwrap_or_else(|err| panic!("Invalid regex {:?}", err));

    let matched: Vec<usize> = regex
        .find_iter(input)
        .filter_map(|matched| {
            println!("{}", matched.as_str());
            matched.as_str().parse().ok()
        })
        .collect();

    if matched.len() != limit {
        panic!(
            "There should not {} values from an input line. Receive {}",
            limit,
            matched.len()
        );
    }

    InputRange {
        src_start: matched[0],
        dest_start: matched[1],
        range: matched[2],
    }
}
