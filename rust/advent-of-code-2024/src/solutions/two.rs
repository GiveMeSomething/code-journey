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
        if is_safe_report_asc(&report) || is_safe_report_desc(&report) {
            count += 1;
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

fn is_safe_report_desc(report: &Vec<i32>) -> bool {
    for i in 1..report.len() {
        if report[i] >= report[i - 1] {
            return false;
        }

        let diff = (report[i] - report[i - 1]).abs();
        if diff < 1 || diff > 3 {
            return false;
        }
    }
    return true;
}

fn is_safe_report_asc(report: &Vec<i32>) -> bool {
    for i in 1..report.len() {
        if report[i] <= report[i - 1] {
            return false;
        }

        let diff = (report[i] - report[i - 1]).abs();
        if diff < 1 || diff > 3 {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod test {
    use super::{is_safe_report_asc, is_safe_report_desc};

    #[test]
    fn test_is_safe_report_desc() {
        let test_cases: Vec<(Vec<i32>, bool)> = vec![
            (vec![7, 6, 4, 2, 1], true),
            (vec![1, 2, 7, 8, 9], false),
            (vec![1, 1, 1, 1, 1], false),
        ];

        for (input, expected) in test_cases {
            let result = is_safe_report_desc(&input);
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
            let result = is_safe_report_asc(&input);
            assert_eq!(result, expected);
        }
    }
}
