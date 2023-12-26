use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_part_from_file() -> Vec<Vec<char>> {
    let file = File::open("src/inputs/three.txt").expect("Should have day 3 input file");
    let reader = BufReader::new(file);

    let mut result: Vec<Vec<char>> = vec![];
    for line in reader.lines() {
        match line {
            Ok(line) => {
                result.push(line.chars().collect());
            }
            Err(_) => panic!("Cannot read line"),
        };
    }

    return result;
}

pub fn cal_engine_parts_sum(parts: &Vec<Vec<char>>) -> usize {
    let mut sum = 0;
    for (i, line) in parts.iter().enumerate() {
        let mut is_part = false;
        let mut current_part_value = 0;
        for (j, character) in line.iter().enumerate() {
            // If encounter ".", or special characters, or at the of current line
            // Calculate if current value is a part then reset variables
            if *character == '.' || is_special_char(Some(character)) {
                if is_part {
                    sum += current_part_value;
                    if current_part_value > 0 {
                        print!("{} ", current_part_value);
                    }
                }
                current_part_value = 0;
                is_part = false;
                continue;
            }

            // Is a number
            if character.is_ascii_digit() {
                let value: usize = character.to_string().parse().expect("Should be an integer");
                current_part_value = current_part_value * 10 + value;
            }

            if j == line.len() - 1 {
                if is_part {
                    sum += current_part_value;
                    continue;
                }

                if is_special_around(parts, i, j) {
                    sum += current_part_value;
                    continue;
                }
            }

            // Already a part, no need to check
            if is_part {
                continue;
            }

            is_part = is_special_around(parts, i, j);
        }
    }

    return sum;
}

// Check if there speacial character around
fn is_special_around(parts: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let max_row_len = parts[0].len() - 1;
    let max_row = parts.len() - 1;

    if i >= 1 {
        match parts.get(i - 1) {
            Some(upper_line) => {
                if j >= 1 && is_special_char(upper_line.get(j - 1)) {
                    return true;
                }

                if is_special_char(upper_line.get(j)) {
                    return true;
                }

                if j + 1 <= max_row_len && is_special_char(upper_line.get(j + 1)) {
                    return true;
                }
            }
            None => {}
        };
    }

    match parts.get(i) {
        Some(upper_line) => {
            if j >= 1 && is_special_char(upper_line.get(j - 1)) {
                return true;
            }

            if is_special_char(upper_line.get(j)) {
                return true;
            }

            if j + 1 <= max_row_len && is_special_char(upper_line.get(j + 1)) {
                return true;
            }
        }
        None => {}
    };

    if i <= max_row - 1 {
        match parts.get(i + 1) {
            Some(upper_line) => {
                if j >= 1 && is_special_char(upper_line.get(j - 1)) {
                    return true;
                }

                if is_special_char(upper_line.get(j)) {
                    return true;
                }

                if j + 1 <= max_row_len && is_special_char(upper_line.get(j + 1)) {
                    return true;
                }
            }
            None => {}
        };
    }

    return false;
}

fn is_special_char(character: Option<&char>) -> bool {
    match character {
        Some(character) => return !(*character == '.') && !character.is_alphanumeric(),
        None => return false,
    }
}
