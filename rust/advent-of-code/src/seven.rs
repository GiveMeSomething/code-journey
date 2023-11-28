use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn generate_directory_map() -> HashMap<String, usize> {
    let file = File::open("src/inputs/seven.txt")
        .unwrap_or_else(|err| panic!("Cannot read file with error {:?}", err));
    let reader = BufReader::new(file);

    let mut dir_size_map: HashMap<String, usize> = HashMap::new();
    let mut current_directory: Vec<String> = vec![];

    for line in reader.lines() {
        // Split command to get tokens
        let tokens: Vec<String> = line
            .unwrap()
            .split(" ")
            .map(|token| String::from(token))
            .collect();

        let identifier = tokens.get(0).unwrap();

        if identifier.eq("$") {
            let command = tokens.get(1).unwrap();

            // Execute command
            if command.eq("cd") {
                let args = tokens.get(2).unwrap();
                match args.as_str() {
                    "/" => {
                        current_directory.clear();
                        current_directory.push(String::from("/"));
                    }
                    ".." => {
                        current_directory.pop();
                    }
                    other => {
                        current_directory.push(String::from(other));
                    }
                };
            } else {
                // Ignore other commands (should only be ls)
                continue;
            }
        } else {
            // Read command output

            // Skip current line if output if a directory
            if identifier.eq("dir") {
                continue;
            }

            let file_size: usize = tokens[0].parse().unwrap();

            for i in 0..current_directory.len() {
                let dir_path = current_directory[0..i + 1].join("/");
                let previous_dir_size = match dir_size_map.get(&dir_path) {
                    Some(dir_size) => *dir_size,
                    None => 0,
                };
                dir_size_map.insert(String::from(dir_path), file_size + previous_dir_size);
            }
        }
    }

    return dir_size_map;
}

pub fn sum_small_files(directory_map: &HashMap<String, usize>) -> usize {
    let mut size_sum = 0;
    for value in directory_map.values() {
        if *value < 100_000 {
            size_sum += *value;
        }
    }
    return size_sum;
}

pub fn deletable_size(directory_map: &HashMap<String, usize>) -> usize {
    let mut valid_file_size: Vec<usize> = vec![];
    let occupied_space = directory_map.get(&String::from("/")).unwrap();
    let required_at_least = occupied_space - 40_000_000;

    for value in directory_map.values() {
        if *value >= required_at_least {
            valid_file_size.push(*value);
        }
    }
    valid_file_size.sort();
    return valid_file_size[0];
}
