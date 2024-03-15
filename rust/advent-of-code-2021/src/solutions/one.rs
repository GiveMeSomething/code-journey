use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_sweep_from_file() -> Vec<isize> {
    let file = File::open("src/inputs/one.txt").expect("Cannot find/open input file for day 1");
    let reader = BufReader::new(file);

    let mut result: Vec<isize> = vec![];
    for line in reader.lines() {
        let current_line = line.expect("Cannot read current line. Aborting...");
        let value: isize = current_line
            .parse()
            .expect("Cannot parse current string as number");
        result.push(value);
    }

    return result;
}

pub fn count_increase_sweep(sweeps: &Vec<isize>) -> usize {
    let mut count: usize = 0;

    let mut previous: isize = sweeps[0];
    for i in 1..sweeps.len() {
        if sweeps[i] > previous {
            count += 1;
        }

        previous = sweeps[i];
    }

    return count;
}
