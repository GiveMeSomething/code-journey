use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    usize,
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
            // If encounter "." or special characters
            if *character == '.' || is_special_char(character) {
                if is_part {
                    sum += current_part_value;
                }
                current_part_value = 0;
                is_part = false;
                continue;
            }

            // Is a digit
            if character.is_ascii_digit() {
                let value: usize = character.to_string().parse().expect("Should be an integer");
                current_part_value = current_part_value * 10 + value;
            }

            if j == line.len() - 1 && (is_part || is_char_around(parts, i, j, is_special_char).0) {
                sum += current_part_value;
                continue;
            }

            // Already a part, no need to check
            if is_part {
                continue;
            }

            is_part = is_char_around(parts, i, j, is_special_char).0;
        }
    }

    return sum;
}

pub fn cal_gear_ratio(parts: &Vec<Vec<char>>) -> usize {
    let mut gear_map: HashMap<String, Vec<usize>> = HashMap::new();

    for (i, line) in parts.iter().enumerate() {
        let mut is_gear_value = false;
        let mut gear_position = String::from("");
        let mut current_part_value = 0;

        for (j, character) in line.iter().enumerate() {
            if *character == '.' || is_special_char(character) {
                if is_gear_value && current_part_value > 0 {
                    match gear_map.get_mut(&gear_position) {
                        Some(position_list) => position_list.push(current_part_value),
                        None => {
                            gear_map.insert(gear_position.to_string(), vec![current_part_value]);
                        }
                    };
                }
                current_part_value = 0;
                is_gear_value = false;
                continue;
            }

            // Is a number
            if character.is_ascii_digit() {
                let value: usize = character.to_string().parse().expect("Should be an integer");
                current_part_value = current_part_value * 10 + value;
            }

            if j == line.len() - 1 {
                if is_gear_value {
                    match gear_map.get_mut(&gear_position) {
                        Some(position_list) => position_list.push(current_part_value),
                        None => {
                            gear_map.insert(gear_position.to_string(), vec![current_part_value]);
                        }
                    };
                    continue;
                }

                let (is_gear_around, position) = is_char_around(parts, i, j, is_gear);
                if is_gear_around {
                    match gear_map.get_mut(&position) {
                        Some(position_list) => position_list.push(current_part_value),
                        None => {
                            gear_map.insert(position, vec![current_part_value]);
                        }
                    };
                }
            }

            // Already a part, no need to check
            if is_gear_value {
                continue;
            }

            (is_gear_value, gear_position) = is_char_around(parts, i, j, is_gear);
        }
    }

    let mut sum = 0;
    println!("{:?}", gear_map);
    for (key, list) in gear_map {
        if key.eq("") {
            continue;
        }
        if list.len() == 2 {
            println!("{}: {} {}", key, list[0], list[1]);
            sum += list[0] * list[1];
        }
    }

    return sum;
}

fn is_char_around(
    parts: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    matcher: fn(&char) -> bool,
) -> (bool, String) {
    let max_i = parts.len();
    let max_j = parts[0].len();

    let low_i = if i == 0 { 0 } else { i - 1 };
    let high_i = if i == max_i - 1 { max_i - 1 } else { i + 1 };

    let low_j = if j == 0 { 0 } else { j - 1 };
    let high_j = if j == max_j - 1 { max_j - 1 } else { j + 1 };

    for (a, line) in parts[low_i..=high_i].iter().enumerate() {
        for (b, character) in line[low_j..=high_j].iter().enumerate() {
            if matcher(character) {
                return (true, format!("{}-{}", low_i + a, low_j + b));
            }
        }
    }

    return (false, String::from(""));
}

fn is_special_char(character: &char) -> bool {
    !(*character == '.') && !character.is_alphanumeric()
}

fn is_gear(character: &char) -> bool {
    *character == '*'
}
