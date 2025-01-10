use std::collections::HashMap;

pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
    let mut task_map: HashMap<char, i32> = HashMap::new();
    for task in tasks {
        let current_count = match task_map.get(&task) {
            Some(value) => *value,
            None => 0,
        };

        task_map.insert(task, current_count + 1);
    }

    let mut task_count = 0;
    let mut max_count = 0;
    let mut maxes: i32 = 0;
    for (_, value) in task_map {
        task_count += value;
        if value == max_count {
            maxes += 1;
        }
        if max_count < value {
            maxes = 1;
            max_count = value;
        }
    }

    let empty_slots = (n - maxes + 1) * (max_count - 1);
    let available_tasks = task_count - max_count * maxes;
    let idles = i32::max(0, empty_slots - available_tasks);
    task_count + idles
}

#[cfg(test)]
mod test {
    use super::least_interval;

    #[test]
    fn test_least_interval() {
        let test_cases: Vec<(Vec<char>, i32, i32)> = vec![
            (vec!['A', 'A', 'A', 'B', 'B', 'B'], 2, 8),
            (vec!['A', 'C', 'A', 'B', 'D', 'B'], 1, 6),
            (vec!['A', 'A', 'A', 'B', 'B', 'B'], 100, 204),
            (vec!['A', 'C', 'A', 'B', 'D', 'B'], 74, 77),
            (
                vec!['A', 'C', 'A', 'B', 'D', 'B', 'A', 'C', 'A', 'B', 'D', 'B'],
                34,
                107,
            ),
        ];

        for (tasks, n, expected) in test_cases {
            let actual = least_interval(tasks, n);
            assert_eq!(actual, expected);
        }
    }
}
