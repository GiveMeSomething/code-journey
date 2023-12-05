use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_value_from_files() -> Vec<String> {
    let file = File::open("src/inputs/one.txt").expect("Cannot read day 1 input file");
    let reader = BufReader::new(file);

    let mut values = vec![];
    for line in reader.lines() {
        values.push(line.unwrap());
    }
    return values;
}

pub fn calculate_calibration_sum(values: &Vec<String>) -> usize {
    let mut sum = 0;
    for value in values {
        sum += extract_calibration_value(value);
    }
    return sum;
}

fn extract_calibration_value(input: &String) -> usize {
    let digit_regex = match Regex::new(r"/\d/g") {
        Ok(regex) => regex,
        Err(_) => panic!("Invalid regular expression"),
    };

    let digits: Vec<usize> = digit_regex
        .find_iter(input.as_str())
        .filter_map(|digits| digits.as_str().parse().ok())
        .collect();

    return digits[0] + digits[digits.len() - 1];
}
