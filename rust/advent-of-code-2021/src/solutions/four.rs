use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
pub struct Bingo {
    bingo: [[isize; 5]; 5],
    checker: [[bool; 5]; 5],
}

impl Bingo {
    pub fn new(bingo: [[isize; 5]; 5]) -> Bingo {
        Bingo {
            bingo,
            checker: [[false; 5]; 5],
        }
    }

    // (step, point)
    pub fn simulate_bingo(&mut self, numbers: &Vec<isize>) -> (usize, isize) {
        for a in 0..numbers.len() {
            let target = numbers[a];
            for i in 0..5 {
                for j in 0..5 {
                    if self.bingo[i][j] == target {
                        self.checker[i][j] = true;
                        if self.check_row(i) || self.check_col(j) {
                            return (a, self.sum_unchecked() * target);
                        }
                    }
                }
            }
        }
        return (0, 0);
    }

    fn check_row(&self, row: usize) -> bool {
        for i in 0..5 {
            if !self.checker[row][i] {
                return false;
            }
        }
        true
    }

    fn check_col(&self, col: usize) -> bool {
        for i in 0..5 {
            if !self.checker[i][col] {
                return false;
            }
        }
        true
    }

    fn sum_unchecked(&self) -> isize {
        let mut sum = 0;
        for i in 0..5 {
            for j in 0..5 {
                if self.checker[i][j] {
                    continue;
                }
                sum += self.bingo[i][j];
            }
        }
        return sum.into();
    }
}

pub fn read_bingo_from_file() -> (Vec<isize>, Vec<Bingo>) {
    let file = File::open("src/inputs/four.txt").expect("Cannot find/open input file for day 4");
    let reader = BufReader::new(file);

    let mut is_first_line = true;
    let mut bingo_numbers: Vec<isize> = vec![];
    let mut bingos: Vec<Bingo> = vec![];

    let mut bingo_buffer: [[isize; 5]; 5] = [[0; 5]; 5];
    let mut counter = 0;

    for line in reader.lines() {
        let current_line = match line {
            Ok(value) => value,
            Err(_) => {
                break;
            }
        };

        // Read all bingo numbers
        if is_first_line {
            let numbers = extract_numbers(&current_line, ",");
            bingo_numbers = numbers;
            is_first_line = false;
            continue;
        }

        if current_line == "" {
            // Add current bingo buffer
            if counter == 5 {
                bingos.push(Bingo::new(bingo_buffer));
                bingo_buffer = [[0; 5]; 5];
                counter = 0;
            }
            continue;
        }

        // Read bingos
        let numbers = extract_numbers(&current_line, " ");
        let length = numbers.len();
        bingo_buffer[counter] = numbers
            .try_into()
            .unwrap_or_else(|_| panic!("Expected a Vec of length {} but it was {}", 5, length));
        counter += 1;
    }

    // Push the last bingo into the list as the line reader ended before this can be added
    bingos.push(Bingo::new(bingo_buffer));

    return (bingo_numbers, bingos);
}

pub fn calculate_fastest_win(numbers: &Vec<isize>, bingos: &mut Vec<Bingo>) -> (usize, isize) {
    let mut fastest_step = usize::MAX;
    let mut fastest_point = 0;
    for bingo in bingos {
        let (step, point) = bingo.simulate_bingo(numbers);
        if step < fastest_step {
            fastest_step = step;
            fastest_point = point;
        }
    }
    return (fastest_step, fastest_point);
}

fn extract_numbers(s: &str, sep: &str) -> Vec<isize> {
    s.split(sep)
        .filter_map(|part| part.parse::<isize>().ok())
        .collect()
}
