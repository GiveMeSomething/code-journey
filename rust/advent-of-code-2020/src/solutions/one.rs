use std::{
    collections::HashMap,
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

pub fn find_entries_by_sum(expenses: &Vec<usize>, sum: usize) -> usize {
    let mut sum_map: HashMap<usize, bool> = HashMap::new();

    for expense in expenses {
        // Return point if sum add up to target
        if sum_map.contains_key(expense) {
            return expense * (sum - expense);
        }

        // Add current key
        if sum < *expense {
            continue;
        }

        let key = sum - expense;
        sum_map.insert(key, true);
    }

    0
}

pub fn find_triplet_by_sum(expenses: &Vec<usize>, sum: usize) -> usize {
    for expense in expenses {
        let target = sum - expense;

        let point = find_entries_by_sum(expenses, target);
        if point == 0 {
            continue;
        }

        return point * expense;
    }

    0
}
