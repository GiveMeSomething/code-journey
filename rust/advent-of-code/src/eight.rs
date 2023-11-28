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

pub fn max_scenic_point(forest: &Vec<Vec<usize>>) -> usize {
    let mut max_point = 0;

    let rows = forest.len();
    let cols = forest[0].len();

    for i in 0..rows {
        for j in 0..cols {
            if i == 0 || j == 0 || i == rows || j == cols {
                continue;
            }

            let current_tree = forest[i][j];
            let mut top_point = 0;
            let mut bot_point = 0;
            let mut left_point = 0;
            let mut right_point = 0;

            // Get top points
            for index in (0..i).rev() {
                let target_tree = forest[index][j];
                if current_tree > target_tree {
                    top_point += 1;
                } else if current_tree == target_tree {
                    top_point += 1;
                    break;
                } else {
                    break;
                }
            }

            // Get bottom points
            for index in i + 1..rows {
                let target_tree = forest[index][j];
                if current_tree > target_tree {
                    bot_point += 1;
                } else if current_tree == target_tree {
                    bot_point += 1;
                    break;
                } else {
                    break;
                }
            }

            // Get left points
            for index in (0..j).rev() {
                let target_tree = forest[i][index];
                if current_tree > target_tree {
                    left_point += 1;
                } else if current_tree == target_tree {
                    left_point += 1;
                    break;
                } else {
                    break;
                }
            }

            // Get right points
            for index in j + 1..cols {
                let target_tree = forest[i][index];
                if current_tree > target_tree {
                    right_point += 1;
                } else if current_tree == target_tree {
                    right_point += 1;
                    break;
                } else {
                    break;
                }
            }

            let current_tree_point = top_point * bot_point * right_point * left_point;
            if current_tree_point > max_point {
                max_point = current_tree_point;
            }
        }
    }

    return max_point;
}
