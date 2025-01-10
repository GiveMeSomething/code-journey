pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    if nums.len() == 0 {
        return vec![];
    }

    if nums.len() == 1 {
        return vec![format!("{}", nums[0])];
    }

    let mut result: Vec<String> = vec![];
    let mut current_start = 0;

    let mut i = 0;
    while i + 1 < nums.len() {
        i += 1;
        let diff = nums[i] - nums[i - 1];
        if diff == 1 {
            continue;
        }

        if current_start == i - 1 {
            result.push(format!("{}", nums[i - 1]));
        } else {
            result.push(format!("{}->{}", nums[current_start], nums[i - 1]));
        }

        current_start = i;
    }

    if current_start == i {
        result.push(format!("{}", nums[i]));
    } else {
        result.push(format!("{}->{}", nums[current_start], nums[i]));
    }

    result
}

#[cfg(test)]
mod test {
    use super::summary_ranges;

    #[test]
    fn test_summary_ranges() {
        let test_cases: Vec<(Vec<i32>, Vec<&str>)> = vec![
            (vec![0, 1, 2, 4, 5, 7], vec!["0->2", "4->5", "7"]),
            (vec![0, 2, 3, 4, 6, 8, 9], vec!["0", "2->4", "6", "8->9"]),
        ];

        for (nums, expected) in test_cases {
            println!("Test case {:?}", nums);
            let result = summary_ranges(nums);

            println!("Result {:?}", result);

            assert_eq!(result.len(), expected.len());

            for i in 0..result.len() {
                assert_eq!(result[i], expected[i]);
            }
        }
    }
}
