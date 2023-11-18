use core::panic;
use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

pub fn read_calorie_from_file() -> Lines<BufReader<File>> {
    let file = match File::open("src/inputs/one.txt") {
        Ok(file) => file,
        Err(error) => panic!("Cannot read one.txt {:?}", error),
    };

    let reader = BufReader::new(file);
    reader.lines()
}

pub fn max_calorie() -> i32 {
    let lines = read_calorie_from_file();

    let mut max = 0;
    let mut sum = 0;
    for line in lines {
        match line {
            Ok(line) => {
                if line.trim().eq("") {
                    if sum > max {
                        max = sum;
                    }
                    sum = 0;
                    continue;
                }

                let value: i32 = match line.parse() {
                    Ok(val) => val,
                    Err(error) => {
                        println!("Cannot parse string {:?} with error {:?}", line, error);
                        0
                    }
                };
                sum += value;
            }
            Err(error) => panic!("Day 1 Part 1 failed with error {:?}", error),
        }
    }

    return max;
}

pub fn top_three_calorie() -> (i32, i32, i32) {
    let lines = read_calorie_from_file();
    let mut calories_count: Vec<i32> = vec![];
    let mut sum = 0;

    for line in lines {
        match line {
            Ok(line) => {
                if line.trim().eq("") {
                    calories_count.push(sum);
                    sum = 0;
                    continue;
                }

                let value: i32 = match line.parse() {
                    Ok(value) => value,
                    Err(error) => {
                        println!("Failed to parse value {} with error {:?}", line, error);
                        0
                    }
                };
                sum += value;
            }
            Err(error) => panic!("Day 1 Part 2 failed with error {:?}", error),
        }
    }

    calories_count.sort();
    let result_len = calories_count.len();
    return (
        calories_count[result_len - 1],
        calories_count[result_len - 2],
        calories_count[result_len - 3],
    );
}
