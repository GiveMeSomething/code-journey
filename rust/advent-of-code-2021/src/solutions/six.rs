use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_interval_from_file() -> Vec<isize> {
    let file = File::open("src/inputs/six.txt").expect("Cannot find/open input file for day 6");
    let reader = BufReader::new(file);

    let mut intervals: Vec<isize> = vec![0; 0];
    for line in reader.lines() {
        let current_line = line.expect("Cannot read next line");
        intervals = current_line
            .split(",")
            .map(|part| part.parse().expect("Interval should be a number"))
            .collect();
    }
    return intervals;
}

pub fn count_lanternfish(intervals: &Vec<isize>, remain: isize) -> isize {
    let mut sum = 0;

    let mut fish_queue: VecDeque<(isize, isize)> = VecDeque::new();
    for interval in intervals {
        fish_queue.push_back((*interval, remain));
    }

    while let Some((init, remaining_day)) = fish_queue.pop_front() {
        let mut child_num = 0;
        if remaining_day >= init + 1 {
            child_num += 1;
        }
        child_num += (remaining_day - init - 1) / 7;

        for i in 0..child_num {
            fish_queue.push_back((8, remaining_day - init - 1 - i * 7));
        }

        sum += 1;
    }

    return sum;
}

pub fn count_lanternfish_optimize(intervals: &Vec<isize>, remain: isize) -> isize {
    let mut sum = 0;

    for interval in intervals {
        sum += recur_count(*interval, remain);
    }

    return sum;
}

pub fn recur_count(interval: isize, remain: isize) -> isize {
    let mut sum = 1;

    if remain < interval + 1 {
        return 1;
    }

    let n = (remain - interval - 1) / 7 + 1;

    for i in 0..n {
        sum += recur_count(8, remain - interval - 1 - i * 7);
    }

    return sum;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_lanternfish() {
        let test_intervals = vec![3, 4, 3, 1, 2];
        let test_days = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 18, 80];
        let test_days_result = [5, 6, 7, 9, 10, 10, 10, 10, 11, 12, 26, 5934];

        for i in 0..test_days.len() {
            println!("Test case: {} days", test_days[i]);
            let result = count_lanternfish(&test_intervals, test_days[i]);
            assert_eq!(result, test_days_result[i]);
        }
    }

    #[test]
    fn test_count_lanternfish_optimize() {
        let test_intervals = vec![3, 4, 3, 1, 2];
        let test_days = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 18, 80, 256];
        let test_days_result = [5, 6, 7, 9, 10, 10, 10, 10, 11, 12, 26, 5934, 26984457539];

        for i in 0..test_days.len() {
            println!("Test case: {} days", test_days[i]);
            let result = count_lanternfish_optimize(&test_intervals, test_days[i]);
            assert_eq!(result, test_days_result[i]);
        }
    }
}
