use core::panic;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Lines},
};

fn read_rucksack_from_file() -> Lines<BufReader<File>> {
    let file = match File::open("src/inputs/three.txt") {
        Ok(file) => file,
        Err(error) => panic!("Cannot open day 3 input with error {:?}", error),
    };

    let reader = BufReader::new(file);
    return reader.lines();
}

pub fn calculate_rucksack_priority() -> i32 {
    let mut sum = 0;
    let inputs = read_rucksack_from_file();
    for input in inputs {
        match input {
            Ok(input) => {
                sum += calculate_single_priority(input);
            }
            Err(error) => panic!("Cannot read input, error: {:?}", error),
        }
    }

    return sum;
}

pub fn calculate_rucksack_group_priority() -> i32 {
    let mut sum = 0;
    let inputs = read_rucksack_from_file();

    let mut group: [String; 3] = Default::default();
    let mut count = 0;
    for input in inputs {
        match input {
            Ok(input) => {
                group[count] = input;
                count += 1;
                if count == 3 {
                    sum += calculate_group_priority(
                        group[0].to_string(),
                        group[1].to_string(),
                        group[2].to_string(),
                    );
                    count = 0;
                }
            }
            Err(error) => panic!("Cannot read input, error: {:?}", error),
        }
    }

    return sum;
}

fn calculate_single_priority(input: String) -> i32 {
    let pivot = input.len() / 2;
    let first = &input[..pivot];
    let second = &input[pivot..];

    let mut char_map: HashMap<char, bool> = HashMap::new();
    for char in first.chars() {
        char_map.insert(char, true);
    }

    for char in second.chars() {
        match char_map.get(&char) {
            Some(existed) => {
                if *existed {
                    return get_char_value(char);
                }
            }
            None => continue,
        }
    }

    return 0;
}

fn calculate_group_priority(first: String, second: String, third: String) -> i32 {
    let mut char_map: HashMap<char, u8> = HashMap::new();

    for char in first.chars() {
        char_map.insert(char, 1);
    }

    for char in second.chars() {
        match char_map.get(&char) {
            Some(_) => char_map.insert(char, 2),
            None => continue,
        };
    }

    for char in third.chars() {
        match char_map.get(&char) {
            Some(appear_time) => {
                if *appear_time == 2 {
                    return get_char_value(char);
                }
            }
            None => continue,
        }
    }

    return 0;
}

fn get_char_value(input: char) -> i32 {
    let ascii_value = input as i32;
    if ascii_value >= 65 && ascii_value <= 90 {
        return ascii_value - 38;
    } else {
        return ascii_value - 96;
    }
}
