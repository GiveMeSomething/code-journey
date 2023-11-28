use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub struct Instruction {
    move_number: usize,
    from_stack: usize,
    to_stack: usize,
}

pub fn read_cargo_from_file() -> (Vec<Vec<String>>, Vec<Instruction>) {
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
                if current_step == 2 {
                    instructions.push(extract_instruction(&line));
                }
            }
            Err(error) => panic!("Cannot read line with error {:?}", error),
        };
    }

    return (cargos, instructions);
}

pub fn peek_top_crates(cargos: &mut Vec<Vec<String>>, instructions: &Vec<Instruction>) -> String {
    for instruction in instructions {
        for _ in 0..instruction.move_number {
            let move_value: String = {
                let from = cargos.get_mut(instruction.from_stack).unwrap_or_else(|| {
                    panic!("Cannot get from stack at index {}", instruction.from_stack)
                });
                from.pop().unwrap_or_else(|| {
                    panic!("Encounter empty stack at {}", instruction.from_stack)
                })
            };

            let to = cargos
                .get_mut(instruction.to_stack)
                .unwrap_or_else(|| panic!("Cannot get to stack at index {}", instruction.to_stack));
            to.push(move_value);
        }
    }

    let mut result: String = String::from("");
    // Extract final message
    for stack in cargos {
        match stack.pop() {
            Some(value) => result.push_str(value.as_str()),
            None => continue,
        };
    }

    return result;
}

pub fn peek_top_crates_9001(
    cargos: &mut Vec<Vec<String>>,
    instructions: &Vec<Instruction>,
) -> String {
    for instruction in instructions {
        let mut move_values: Vec<String> = {
            let from = cargos.get_mut(instruction.from_stack).unwrap_or_else(|| {
                panic!("Cannot get from stack at index {}", instruction.from_stack)
            });

            let result = from[from.len() - instruction.move_number..from.len()].to_vec();

            for _ in 0..instruction.move_number {
                from.pop().unwrap_or_else(|| {
                    panic!("Encounter empty stack at {}", instruction.from_stack)
                });
            }

            result
        };

        let to = cargos
            .get_mut(instruction.to_stack)
            .unwrap_or_else(|| panic!("Cannot get to stack at index {}", instruction.to_stack));
        to.append(&mut move_values);
    }

    let mut result: String = String::from("");
    // Extract final message
    for stack in cargos {
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
        from_stack: from - 1,
        to_stack: to - 1,
    };
}

pub fn clone_vec(input: &Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut result: Vec<Vec<String>> = vec![];
    for i in 0..input.len() {
        result.push(vec![]);
        for value in input.get(i).unwrap() {
            result[i].push(value.clone());
        }
    }
    return result;
}

fn parse_int(input: &str) -> usize {
    let value: usize = input.parse().unwrap_or_else(|error| {
        panic!("Cannot parse {} to integer with error {:?}", input, error);
    });
    return value;
}
