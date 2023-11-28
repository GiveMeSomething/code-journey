use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_forest_from_file() -> Vec<Vec<usize>> {
    let file = File::open("src/inputs/eight.txt")
        .unwrap_or_else(|err| panic!("Cannot open file with error {:?}", err));
    let reader = BufReader::new(file);

    let mut forest: Vec<Vec<usize>> = vec![];

    for line in reader.lines().map(|line| {
        line.unwrap_or_else(|err| panic!("Cannot read current line with error {:?}", err))
    }) {
        let row_trees: Vec<usize> = line
            .chars()
            .map(|char| {
                let value: usize = String::from(char).parse().unwrap();
                value
            })
            .collect();
        forest.push(row_trees);
    }

    return forest;
}
