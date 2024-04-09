use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_position_from_file() -> Vec<isize> {
    let file = File::open("src/inputs/seven.txt").expect("Cannot find/open input file for day 7");
    let mut reader = BufReader::new(file);
    let mut line = String::from("");
    let _ = reader.read_line(&mut line).unwrap();

    return line
        .split(",")
        .filter_map(|part| part.parse().ok())
        .collect();
}

pub fn min_horizontal_medium(positions: &Vec<isize>) -> isize {
    let mut min_sum = isize::MAX;
    for i in positions {
        let mut sum = 0;
        for j in positions {
            sum += (i - j).abs();
        }
        min_sum = min_sum.min(sum);
    }

    return min_sum;
}

pub fn min_horizontal_medium_increment(positions: &Vec<isize>) -> isize {
    let min_position = min_vec(positions);
    let max_position = max_vec(positions);
    let mut min_sum = isize::MAX;
    for i in min_position..=max_position {
        let mut sum = 0;
        for j in positions {
            sum += incremental_sum((i - j).abs());
        }
        min_sum = min_sum.min(sum);
    }
    return min_sum;
}

fn min_vec(input: &Vec<isize>) -> isize {
    let mut min = input[0];
    for value in input {
        min = min.min(*value);
    }
    min
}

fn max_vec(input: &Vec<isize>) -> isize {
    let mut max = input[0];
    for value in input {
        max = max.max(*value);
    }
    max
}

fn incremental_sum(input: isize) -> isize {
    input * (input + 1) / 2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_horizontal_medium() {
        let input: Vec<isize> = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        let expected = 37;
        let result = min_horizontal_medium(&input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_min_horizontal_medium_increment() {
        let input: Vec<isize> = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        let expected = 168;
        let result = min_horizontal_medium_increment(&input);
        assert_eq!(result, expected);
    }
}
