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
    let digit_regex = match Regex::new(r"\d") {
        Ok(regex) => regex,
        Err(_) => panic!("Invalid regular expression"),
    };

    let first_digit: isize = digit_regex
        .find(input.as_str())
        .unwrap()
        .as_str()
        .parse()
        .expect("Expect a digit");

    let first_digit: usize = digit_regex
        .find(input)
        .unwrap()
        .as_str()
        .parse()
        .expect("Expect a digit");

    let rev_input: String = input.chars().rev().collect();
    let second_digit: usize = digit_regex
        .find(&rev_input)
        .unwrap()
        .as_str()
        .parse()
        .expect("Expect a digit");

    // "1" + "2" = 12 = 1 * 10 + 2
    return first_digit * 10 + second_digit;
}

fn extract_calibration_value_text(digit_text_map: &HashMap<&str, usize>, input: &String) -> usize {
    let inner_regex: String = digit_text_map
        .keys()
        .fold(String::new(), |a, b| format!("{a}{b}|"));
    let regex = format!(r"{inner_regex}\d");

    // Regex to match digits and text-based digits
    let digit_regex = Regex::new(&regex).expect("Invalid regex");

    let matched_texts: Vec<&str> = digit_regex
        .iter(input.as_str())
        .map(|res| res.as_str())
        .collect();
    println!("{}", input);
    println!("{:?}", matched_texts);

    let digits: Vec<usize> = digit_regex
        .find_iter(input.as_str())
        .filter_map(|digit| match digit.as_str().parse().ok() {
            Some(value) => Some(value),
            None => Some(*digit_text_map.get(digit.as_str()).unwrap()),
        })
        .collect();

    return digits[0] * 10 + digits[digits.len() - 1];
}
