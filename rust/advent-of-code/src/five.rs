use std::{
    fs::{self, File},
    intrinsics::mir::Len,
    io::{BufRead, BufReader, Lines},
    string,
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
                    for i in (0..input.len() / 3) {
                        cargos.push(vec![]);
                    }
                    init = true;
                }
                // Move to next step when encounter a empty line
                if current_step == 0 && input.trim().eq("") {
                    current_step = 1;
                    continue;
                }

                // Read cargo inputs
                for (i, char) in input.char_indices() {
                    // Ignore whitespace and char wrapper
                    if char == ' ' || char == '[' || char == ']' {
                        continue;
                    }
                }
            }
            Err(_) => break,
        }
    }

    return String::from("Hello World");
}
