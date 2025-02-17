use std::{
    cmp::{max, min},
    i32,
};

// Time Limit Exceeded as we are iterating to find min/max everytime
// We need a better way to track min/max when the window is moving
pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
    let mut current_min = i32::MAX;
    let mut current_max = 0;
    let mut start = 0;
    let mut end = 0;
    let mut max_subarray_len = 0;

    while end < nums.len() {
        current_min = min(current_min, nums[end]);
        current_max = max(current_max, nums[end]);

        let mut diff = (current_max - current_min).abs();

        // Handle abnormal
        if diff > limit {
            // Try to increase start until the current subarray is valid again
            while diff > limit && start < end {
                if nums[start] == current_min {
                    current_min = i32::MAX;
                    for i in start + 1..=end {
                        current_min = min(current_min, nums[i]);
                    }
                }

                if nums[start] == current_max {
                    current_max = 0;
                    for i in start + 1..=end {
                        current_max = max(current_max, nums[i]);
                    }
                }

                diff = (current_max - current_min).abs();
                start += 1;
            }

            end += 1;
            continue;
        }

        // Hanlde normal case
        max_subarray_len = max(max_subarray_len, end - start + 1);
        end += 1;
    }

    max_subarray_len as i32
}

#[cfg(test)]
mod test {
    use super::longest_subarray;

    #[test]
    fn test_longest_subarray() {
        let test_cases: Vec<(Vec<i32>, i32, i32)> = vec![
            (vec![8, 2, 4, 7], 4, 2),
            (vec![10, 1, 2, 4, 7, 2], 5, 4),
            (vec![4, 2, 2, 2, 4, 4, 2, 2], 0, 3),
        ];

        for (nums, limit, expected) in test_cases {
            println!("TEST CASE {:?}", nums);
            let result = longest_subarray(nums, limit);
            assert_eq!(result, expected);
        }
    }
}
