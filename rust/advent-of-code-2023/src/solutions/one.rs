use regex::Regex;
use std::{
    collections::HashMap,
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

pub fn calculate_calibration_sum_text(values: &Vec<String>) -> usize {
    let digit_text_map: HashMap<&str, usize> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut sum = 0;
    for value in values {
        sum += extract_calibration_value_text(&digit_text_map, value);
    }
    return sum;
}

fn extract_calibration_value(input: &String) -> usize {
    // Regex to match digits
    let digit_regex = Regex::new(r"\d").expect("Expect a valid regex");

    let digits: Vec<usize> = digit_regex
        .find_iter(input.as_str())
        .filter_map(|matched_value| matched_value.as_str().parse().ok())
        .collect();

    // "1" + "2" = 12 = 1 * 10 + 2
    return digits[0] * 10 + digits[digits.len() - 1];
}

fn extract_calibration_value_text(digit_text_map: &HashMap<&str, usize>, input: &String) -> usize {
    let forward_regex = Regex::new(r"one|two|three|four|five|six|seven|eight|nine|\d")
        .expect("Should be a valid regular expression");

    let backward_regex = Regex::new(r"eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|\d")
        .expect("Should be a valid regular expresison");

    let first_digit: usize = match forward_regex.find(&input) {
        Some(value) => match value.as_str().parse() {
            Ok(value) => value,
            Err(_) => *digit_text_map
                .get(value.as_str())
                .expect("Should be able to parse"),
        },
        None => 0,
    };

    let reverse_input: String = input.chars().rev().collect();
    let second_digit: usize = match backward_regex.find(&reverse_input) {
        Some(value) => match value.as_str().parse() {
            Ok(value) => value,
            Err(_) => {
                let digit_text: String = value.as_str().chars().rev().collect();
                *digit_text_map.get(digit_text.as_str()).unwrap()
            }
        },
        None => 0,
    };

    if second_digit == 0 {
        return first_digit * 11;
    } else {
        return first_digit * 10 + second_digit;
    }
}
