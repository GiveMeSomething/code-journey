use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_bits_from_file() -> Vec<String> {
    let file = File::open("src/inputs/three.txt").expect("Cannot find/open input file for day 3");
    let reader = BufReader::new(file);

    let mut result: Vec<String> = vec![];
    for line in reader.lines() {
        let current_line = line.expect("Cannot read line. Aborting...");
        result.push(current_line.trim().to_string());
    }
    result
}

pub fn process_bits(bits: &Vec<String>) -> isize {
    let mut bit_counter: Vec<isize> = vec![0; bits[0].len()];
    for bit_line in bits {
        let mut index = 0;
        for part in bit_line.split("") {
            if part == "" {
                continue;
            }

            bit_counter[index] += if part == "0" { -1 } else { 1 };
            index += 1;
        }
    }

    let mut gamma = String::from("");
    let mut epsilon = String::from("");
    for bit_count in bit_counter {
        if bit_count > 0 {
            gamma.push_str("1");
            epsilon.push_str("0");
        } else {
            gamma.push_str("0");
            epsilon.push_str("1");
        }
    }

    let gamma_value = isize::from_str_radix(gamma.as_str(), 2).unwrap();
    let epsilon_value = isize::from_str_radix(epsilon.as_str(), 2).unwrap();

    gamma_value * epsilon_value
}

pub fn find_oxygen_rating(bits: &Vec<String>) -> isize {
    let mut i = 0;

    let mut result = bits.clone();
    while result.len() > 1 {
        let mut counter = 0;

        for bit_line in &result {
            counter += if bit_line.chars().nth(i).unwrap() == '1' {
                1
            } else {
                -1
            };
        }

        let filter = if counter < 0 { '1' } else { '0' };

        result = result
            .iter()
            .filter(|bit| bit.chars().nth(i).unwrap().eq(&filter))
            .map(|bit| String::from(bit))
            .collect();
        i += 1;
    }

    isize::from_str_radix(result.get(0).unwrap(), 2).unwrap()
}

pub fn find_co2_rating(bits: &Vec<String>) -> isize {
    let mut i = 0;

    let mut result = bits.clone();
    while result.len() > 1 {
        let mut counter = 0;

        for bit_line in &result {
            counter += if bit_line.chars().nth(i).unwrap() == '1' {
                1
            } else {
                -1
            };
        }

        let filter = if counter < 0 { '0' } else { '1' };

        result = result
            .iter()
            .filter(|bit| bit.chars().nth(i).unwrap().eq(&filter))
            .map(|bit| String::from(bit))
            .collect();
        i += 1;
    }

    isize::from_str_radix(result.get(0).unwrap(), 2).unwrap()
}
