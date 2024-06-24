use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_expense_from_file() -> Vec<usize> {
    let file = File::open("src/inputs/one.txt").expect("Cannot open/find input file for day one");

    let mut expenses: Vec<usize> = vec![];
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("Cannot read current line");
        expenses.push(line.parse().expect("Expense must be a positive number"));
    }

    expenses
}
