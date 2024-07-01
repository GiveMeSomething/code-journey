pub mod policy;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use policy::Policy;

pub fn read_password_from_file() -> Vec<Policy> {
    let file = File::open("src/inputs/two.txt").expect("Cannot open/find input file for day 2");
    let reader = BufReader::new(file);

    let mut result: Vec<Policy> = vec![];

    for line in reader.lines() {
        let line = line.expect("Cannot read line from input");
        result.push(Policy::try_from(line).expect("Invalid policy from input"));
    }

    return result;
}

pub fn count_valid_password(inputs: &Vec<Policy>) -> usize {
    let mut count = 0;
    for input in inputs {
        if input.is_valid() {
            count += 1;
        }
    }
    count
}
