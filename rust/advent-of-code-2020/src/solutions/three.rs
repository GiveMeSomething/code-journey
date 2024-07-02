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

pub fn count_slope_tree(trees: &Vec<Vec<char>>, right: usize, down: usize) -> usize {
    let row = trees.len();
    let col = trees[0].len();

    let mut current_row = down;
    let mut current_col = right;
    let mut tree_count = 0;

    while current_row < row {
        if trees[current_row][current_col % col] == '#' {
            tree_count += 1;
        }

        current_row += down;
        current_col += right;
    }

    tree_count
}
