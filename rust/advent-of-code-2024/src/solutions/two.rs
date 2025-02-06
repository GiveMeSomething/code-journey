use std::{
    fs::File,
    io::{BufRead, BufReader},
    num::ParseIntError,
};

#[allow(dead_code)]
pub fn part_1() -> i32 {
    let reports = read_report_from_file();

    let mut count = 0;
    for report in reports {
        if is_safe_report(&report, true) || is_safe_report(&report, false) {
            count += 1;
        }
    }

    count
}

#[allow(dead_code)]
pub fn part_2() -> i32 {
    let reports = read_report_from_file();

    let mut count = 0;
    for report in reports {
        if is_safe_threshold_report(&report, true) || is_safe_threshold_report(&report, false) {
            count += 1;
        } else {
            println!("UNSAFE REPORT {:?}", report);
        }
    }

    count
}

fn read_report_from_file() -> Vec<Vec<i32>> {
    let input_path = "src/inputs/two.txt";
    let file = File::open(input_path).expect("Failed to open input file");
    let reader = BufReader::new(file);

    let mut reports: Vec<Vec<i32>> = vec![];
    for line in reader.lines() {
        let report = extract_report(line.expect("Failed to read report line"))
            .expect("Failed to extract report");
        reports.push(report);
    }

    reports
}

fn extract_report(line: String) -> Result<Vec<i32>, ParseIntError> {
    let parts: Vec<&str> = line.split(" ").collect();
    let mut levels = vec![];

    for part in parts {
        let level = part.parse::<i32>()?;
        levels.push(level);
    }

    Ok(levels)
}

fn is_safe_report(report: &Vec<i32>, desc: bool) -> bool {
    let multiplier = if desc { 1 } else { -1 };
    for i in 1..report.len() {
        if report[i] * multiplier >= report[i - 1] * multiplier {
            return false;
        }

        let diff = (report[i] - report[i - 1]).abs();
        if diff < 1 || diff > 3 {
            return false;
        }
    }
    return true;
}

// Allow "threshold" error when checking
fn is_safe_threshold_report(report: &Vec<i32>, desc: bool) -> bool {
    if is_safe_report(&report, desc) {
        return true;
    }

    let mut error_index = 0;

    let multiplier = if desc { 1 } else { -1 };
    for i in 1..report.len() {
        let diff = (report[i] - report[i - 1]).abs();
        if report[i] * multiplier >= report[i - 1] * multiplier || diff < 1 || diff > 3 {
            error_index = i;
        }
    }

    let mut temp_array_1 = report.clone();
    temp_array_1.remove(error_index);

    let mut temp_array_2 = report.clone();
    temp_array_2.remove(error_index - 1);

    return is_safe_report(&temp_array_1, desc) || is_safe_report(&temp_array_2, desc);
}

#[cfg(test)]
mod test {
    use crate::solutions::two::{is_safe_report, is_safe_threshold_report};

    #[test]
    fn test_is_safe_report_desc() {
        let test_cases: Vec<(Vec<i32>, bool)> = vec![
            (vec![7, 6, 4, 2, 1], true),
            (vec![1, 2, 7, 8, 9], false),
            (vec![1, 1, 1, 1, 1], false),
        ];

        for (input, expected) in test_cases {
            let result = is_safe_report(&input, true);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_is_safe_report_asc() {
        let test_cases: Vec<(Vec<i32>, bool)> = vec![
            (vec![1, 3, 6, 7, 9], true),
            (vec![1, 2, 7, 8, 9], false),
            (vec![1, 1, 1, 1, 1], false),
        ];

        for (input, expected) in test_cases {
            let result = is_safe_report(&input, false);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_is_safe_threshold_report_desc() {
        let test_cases: Vec<(Vec<i32>, bool)> = vec![
            (vec![7, 6, 4, 2, 1], true),
            (vec![8, 6, 4, 4, 1], true),
            (vec![1, 2, 7, 8, 9], false),
            (vec![1, 1, 1, 1, 1], false),
        ];

        for (input, expected) in test_cases {
            let result = is_safe_threshold_report(&input, true);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_is_safe_threshold_report_asc() {
        let test_cases: Vec<(Vec<i32>, bool)> = vec![
            (vec![1, 3, 6, 7, 9], true),
            (vec![1, 3, 2, 4, 5], true),
            (vec![1, 2, 7, 8, 9], false),
            (vec![1, 1, 1, 1, 1], false),
            (vec![27, 30, 28, 31, 32, 35, 38], true),
        ];

        for (input, expected) in test_cases {
            let result = is_safe_threshold_report(&input, false);
            assert_eq!(result, expected);
        }
    }
}
