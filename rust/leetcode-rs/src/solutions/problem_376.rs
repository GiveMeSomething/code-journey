use std::cmp::max;

pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
    let mut asc = 1;
    let mut desc = 1;
    for i in 1..nums.len() {
        if nums[i] > nums[i - 1] {
            asc = desc + 1;
            continue;
        }

        if nums[i] < nums[i - 1] {
            desc = asc + 1;
            continue;
        }
    }

    max(asc, desc)
}

#[cfg(test)]
mod test {
    use super::wiggle_max_length;

    #[test]
    fn test_wiggle_max_length() {
        let test_cases: Vec<(Vec<i32>, i32)> = vec![
            (vec![1, 7, 4, 9, 2, 5], 6),
            (vec![1, 17, 5, 10, 13, 15, 10, 5, 16, 8], 7),
            (vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 2),
        ];

        for (nums, expected) in test_cases {
            let result = wiggle_max_length(nums);
            assert_eq!(result, expected);
        }
    }
}
