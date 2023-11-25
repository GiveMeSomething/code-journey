use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

fn read_cargo_from_file() -> Lines<BufReader<File>> {
    let file = match File::open("src/inputs/five.txt") {
        Ok(file) => file,
        Err(error) => panic!("Cannot open day 5 inputs with error {:?}", error),
    };

    let reader = BufReader::new(file);
    return reader.lines();
}

pub fn peek_top_crates() -> String {
    let inputs = read_cargo_from_file();

    let mut cargos: Vec<Vec<String>> = vec![];
    let mut init: bool = false;

    // Step = 0 => Read cargo inputs
    // Step = 1 => Read cargo moving instructions
    let mut current_step = 0;

    for input in inputs {
        match input {
            Ok(input) => {
                // Init cargo stacks
                if !init {
                    for _ in 0..input.len() / 3 {
                        cargos.push(vec![]);
                    }
                    init = true;
                }
                // Move to next step when encounter a empty line
                if current_step == 0 && input.trim().eq("") {
                    current_step = 1;
                    continue;
                }

                // Read cargo from file
                if current_step == 0 {
                    // Read cargo inputs
                    for (position, value) in extract_cargo(input) {
                        let stack = cargos.get_mut(position).unwrap();
                        stack.push(value);
                    }
                }

                // Read instructions and execute from file
                if current_step == 1 {
                    let (value, from, to) = extract_instruction(input);
                    let from = cargos.get_mut(from).unwrap();
                    let to = cargos.get_mut(to).unwrap();
                    for _ in 0..value {
                        let cargo = from.pop().unwrap();
                        to.push(cargo);
                    }
                }
            }
            Err(_) => break,
        }
    }

    let mut result: String = String::from("");
    // Extract final message
    for mut stack in cargos {
        match stack.pop() {
            Some(value) => result.push_str(value.as_str()),
            None => continue,
        };
    }

    return result;
}

fn extract_cargo(input: String) -> Vec<(usize, String)> {
    let mut result: Vec<(usize, String)> = vec![];
    for (i, char) in input.char_indices() {
        // Ignore whitespace and char wrapper
        if char == ' ' || char == '[' || char == ']' {
            continue;
        }
        let position = (i - 1) / 4;
        result.push((position, String::from(char)));
    }

    return result;
}

fn extract_instruction(input: String) -> (usize, usize, usize) {
    let instruction_regex: Regex =
        match Regex::new("move (?<value>.+) from (?<from>.+) to (?<to>.+)") {
            Ok(regex) => regex,
            Err(error) => panic!("Failed to create regex with error {:?}", error),
        };

    let Some(groups) = instruction_regex.captures(&input) else {
        return (0, 0, 0);
    };

    let value: usize = groups["value"].parse().unwrap();
    let from: usize = groups["from"].parse().unwrap();
    let to: usize = groups["to"].parse().unwrap();

    return (value, from, to);
}
