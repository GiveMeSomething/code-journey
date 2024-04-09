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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_horizontal_medium() {
        let input: Vec<isize> = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        let expected = 2;
        let result = min_horizontal_medium(&input);
        assert_eq!(result, expected);
    }
}
