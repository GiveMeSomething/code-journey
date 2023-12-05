use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

fn read_pairs_from_file() -> Lines<BufReader<File>> {
    let file = match File::open("src/inputs/four.txt") {
        Ok(file) => file,
        Err(error) => panic!("Cannot open day 4 input file, error: {:?}", error),
    };

    let reader = BufReader::new(file);
    return reader.lines();
}

pub fn count_overlap_pair() -> i32 {
    let pairs = read_pairs_from_file();
    let mut counter = 0;

    for pair in pairs {
        match pair {
            Ok(pair) => {
                let extreme_points: Vec<i32> = extract_extreme_points(pair);
                if is_overlap_ranges(
                    *extreme_points.get(0).unwrap(),
                    *extreme_points.get(1).unwrap(),
                    *extreme_points.get(2).unwrap(),
                    *extreme_points.get(3).unwrap(),
                ) {
                    counter += 1;
                }
            }
            Err(error) => panic!("Cannot read with error {:?}", error),
        };
    }

    return counter;
}

pub fn count_partial_overlap_pair() -> i32 {
    let pairs = read_pairs_from_file();
    let mut counter = 0;

    for pair in pairs {
        match pair {
            Ok(pair) => {
                let extreme_points: Vec<i32> = extract_extreme_points(pair);
                if is_partial_overlap_ranges(
                    *extreme_points.get(0).unwrap(),
                    *extreme_points.get(1).unwrap(),
                    *extreme_points.get(2).unwrap(),
                    *extreme_points.get(3).unwrap(),
                ) {
                    counter += 1;
                }
            }
            Err(error) => panic!("Cannot read with error {:?}", error),
        };
    }

    return counter;
}

fn extract_extreme_points(input: String) -> Vec<i32> {
    let mut extreme_points: Vec<i32> = vec![];
    let ranges: Vec<&str> = input.split(",").collect();
    for range in ranges {
        let points: Vec<&str> = range.split("-").collect();
        for point in points {
            let value: i32 = point.parse().unwrap();
            extreme_points.push(value);
        }
    }
    return extreme_points;
}

fn is_overlap_ranges(start1: i32, end1: i32, start2: i32, end2: i32) -> bool {
    return (start1 <= start2 && end1 >= end2) || (start1 >= start2 && end1 <= end2);
}

fn is_partial_overlap_ranges(start1: i32, end1: i32, start2: i32, end2: i32) -> bool {
    return (start1 <= end2 && end1 >= end2)
        || (start2 <= end1 && end2 >= end1)
        || (start1 <= start2 && end1 >= start2)
        || (start2 <= start1 && end2 >= start1);
}
