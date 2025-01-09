use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[allow(dead_code)]
pub fn part_1() {
    let (list_1, list_2) = read_id_list_from_file();

    let result_part_1 = sum_list_distance(&list_1, &list_2);
    println!("Day 1 Part 1: {}", result_part_1);
}

fn sum_list_distance(list_1: &Vec<i32>, list_2: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in 0..list_1.len() {
        sum += (list_1[i] - list_2[i]).abs();
    }
    return sum;
}

fn read_id_list_from_file() -> (Vec<i32>, Vec<i32>) {
    let input_path = "src/inputs/one.txt";
    let input_file = File::open(input_path).expect("Failed to open input file");

    let mut line_1: Vec<i32> = vec![];
    let mut line_2: Vec<i32> = vec![];

    let reader = BufReader::new(input_file);
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let parts: Vec<&str> = line.split("   ").collect();

        // Validate if we can extract exact 2 parts from a line
        if parts.len() != 2 {
            panic!("Invalid input");
        }

        let first_id = str::parse::<i32>(parts[0]).expect("Input must be a number");
        let second_id = str::parse::<i32>(parts[1]).expect("Input must be a number");

        line_1.push(first_id);
        line_2.push(second_id);
    }

    line_1.sort();
    line_2.sort();

    (line_1, line_2)
}

#[cfg(test)]
mod test {
    use super::{read_id_list_from_file, sum_list_distance};

    #[test]
    fn test_read_day_one_input_from_file() {
        let (id_list_1, id_list_2) = read_id_list_from_file();

        assert!(id_list_1.len() > 0);
        assert!(id_list_2.len() > 0);
    }

    #[test]
    fn test_sum_list_distance() {
        let list_1 = vec![3, 4, 2, 1, 3, 3];
        let list_2 = vec![4, 3, 5, 3, 9, 3];

        let expected = 13;
        let result = sum_list_distance(&list_1, &list_2);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_day_one_part_one() {
        let mut list_1 = vec![3, 4, 2, 1, 3, 3];
        let mut list_2 = vec![4, 3, 5, 3, 9, 3];

        list_1.sort();
        list_2.sort();

        let expected = 11;
        let result = sum_list_distance(&list_1, &list_2);
        assert_eq!(result, expected);
    }
}
