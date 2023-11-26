use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

struct Instruction {
    move_number: usize,
    from_stack: usize,
    to_stack: usize,
}

fn read_cargo_from_file() -> (Vec<Vec<String>>, Vec<Instruction>) {
    let file = match File::open("src/inputs/five.txt") {
        Ok(file) => file,
        Err(error) => panic!("Cannot open day 5 inputs with error {:?}", error),
    };

    let reader = BufReader::new(file);

    let mut cargos: Vec<Vec<String>> = vec![];
    let mut instructions: Vec<Instruction> = vec![];

    let mut init = false;

    // Step = 1 => Read cargos inputs
    // Step = 2 => Read instructions
    let mut current_step = 1;

    for line in reader.lines() {
        match line {
            Ok(line) => {
                // Switch to next step when encounter an empty line
                if line.trim().eq("") {
                    current_step = 2;
                    continue;
                }

                // Init all empty stack
                if !init {
                    let stack_number = line.len() / 4;
                    for _ in 0..=stack_number {
                        cargos.push(vec![]);
                    }
                    init = true;
                }

                // Read cargos
                if current_step == 1 {
                    let line_cargos = extract_cargo(&line);
                    for (position, value) in line_cargos {
                        let stack = cargos
                            .get_mut(position)
                            .unwrap_or_else(|| panic!("Cannot read index {}", position));
                        stack.insert(0, value);
                    }
                }

                // Read instructions
                if current_step == 2 {}
            }
            Err(error) => panic!("Cannot read line with error {:?}", error),
        };
    }

    return (cargos, instructions);
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
                    for (position, value) in extract_cargo(&input) {
                        let stack = cargos.get_mut(position).unwrap();
                        stack.insert(0, value);
                    }
                }

                // Read instructions and execute from file
                if current_step == 1 {
                    let (value, from, to) = extract_instruction(&input);
                    for _ in 0..value {
                        let cargo: String = { cargos.get_mut(from).unwrap().pop().unwrap() };
                        let target_stack = cargos.get_mut(to).unwrap();
                        target_stack.push(cargo);
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

pub fn peek_top_crates_9001() -> String {
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
                    for (position, value) in extract_cargo(&input) {
                        let stack = cargos.get_mut(position).unwrap();
                        stack.insert(0, value);
                    }
                }

                // Read instructions and execute from file
                if current_step == 1 {
                    let (value, from, to) = extract_instruction(&input);
                    let insert_position = { cargos.get(to).unwrap().len() };
                    for _ in 0..value {
                        let cargo: String = { cargos.get_mut(from).unwrap().pop().unwrap() };
                        let target_stack = cargos.get_mut(to).unwrap();
                        target_stack.insert(insert_position, cargo);
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

fn extract_cargo(input: &String) -> Vec<(usize, String)> {
    let mut result: Vec<(usize, String)> = vec![];
    for (i, char) in input.char_indices() {
        // Ignore whitespace and char wrapper
        if char == ' ' || char == '[' || char == ']' || char.is_numeric() {
            continue;
        }
        let position = (i - 1) / 4;
        result.push((position, String::from(char)));
    }

    return result;
}

fn extract_instruction(input: &String) -> Instruction {
    let instruction_regex: Regex =
        match Regex::new("move (?<value>.+) from (?<from>.+) to (?<to>.+)") {
            Ok(regex) => regex,
            Err(error) => panic!("Failed to create regex with error {:?}", error),
        };

    let Some(groups) = instruction_regex.captures(&input) else {
        return Instruction {
            move_number: 0,
            from_stack: 0,
            to_stack: 0,
        };
    };

    let value: usize = parse_int(&groups["value"]);
    let from: usize = parse_int(&groups["from"]);
    let to: usize = parse_int(&groups["to"]);

    return Instruction {
        move_number: value,
        from_stack: from,
        to_stack: to,
    };
}

fn parse_int(input: &str) -> usize {
    let value: usize = input
        .parse()
        .unwrap_or_else(|| panic!("Cannot parse {} to integer", input));
    return value;
}
