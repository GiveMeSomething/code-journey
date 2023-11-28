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

pub fn count_visible_trees(forest: &Vec<Vec<usize>>) -> usize {
    let mut counter = 0;

    let rows = forest.len();
    let cols = forest[0].len();

    for i in 0..rows {
        for j in 0..cols {
            if i == 0 || j == 0 || i == rows || j == cols {
                counter += 1;
                continue;
            }

            let current_tree = forest[i][j];

            // Check if current tree is invisible

            // Check top
            let top_visible: bool = forest[0..i]
                .iter()
                .filter(|current_row| current_tree <= current_row[j])
                .count()
                == 0;
            let bot_visible: bool = forest[i + 1..rows]
                .iter()
                .filter(|current_row| current_tree <= current_row[j])
                .count()
                == 0;
            let left_visible: bool = forest[i][0..j]
                .iter()
                .filter(|tree| current_tree <= **tree)
                .count()
                == 0;
            let right_visible: bool = forest[i][j + 1..cols]
                .iter()
                .filter(|tree| current_tree <= **tree)
                .count()
                == 0;

            // Increase the counter if the current tree is still visible
            if top_visible || bot_visible || left_visible || right_visible {
                counter += 1;
            }
        }
    }

    return counter;
}
