use std::{cmp::min, i32};

pub fn find_min_difference(time_points: Vec<String>) -> i32 {
    let mut time_minutes: Vec<i32> = vec![];
    for point in time_points {
        let (hour, minute) = extract_time(point.to_string());
        time_minutes.push((hour + 24) * 60 + minute);
        time_minutes.push(hour * 60 + minute);
    }

    time_minutes.sort();

    let mut min_diff = i32::MAX;
    for i in 0..time_minutes.len() - 1 {
        min_diff = min(time_minutes[i + 1] - time_minutes[i], min_diff)
    }
    min_diff
}

pub fn extract_time(time: String) -> (i32, i32) {
    let parts: Vec<&str> = time.split(":").collect();

    let hour = str::parse::<i32>(parts[0]).expect("Hour must be a number");
    let min = str::parse::<i32>(parts[1]).expect("Minute must be a number");

    (hour, min)
}

#[cfg(test)]
mod test {
    use super::{extract_time, find_min_difference};

    #[test]
    fn test_extract_time() {
        let test_cases: Vec<(&str, (i32, i32))> = vec![("23:59", (23, 59)), ("10:10", (10, 10))];

        for (input, expected) in test_cases {
            let result = extract_time(String::from(input));
            assert_eq!(result.0, expected.0);
            assert_eq!(result.1, expected.1);
        }
    }

    #[test]
    fn test_problem_539() {
        let test_cases: Vec<(Vec<&str>, i32)> = vec![
            (vec!["23:59", "00:00"], 1),
            (vec!["00:00", "23:59", "00:00"], 0),
            (vec!["12:12", "00:13"], 719),
            (vec!["02:39", "10:26", "21:43"], 296),
        ];

        for (input, expected) in test_cases {
            let input = input.iter().map(|v| String::from(*v)).collect();
            let result = find_min_difference(input);
            assert_eq!(result, expected);
        }
    }
}
