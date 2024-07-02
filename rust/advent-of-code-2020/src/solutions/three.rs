use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_tree_from_file() -> Vec<Vec<char>> {
    let file = File::open("src/inputs/three.txt").expect("Cannot read/open input file for day 3");
    let reader = BufReader::new(file);

    let mut result: Vec<Vec<char>> = vec![];
    for line in reader.lines() {
        let line = line.expect("Cannot read line: Check input file's content");
        result.push(line.chars().collect());
    }
    result
}
