use std::{cmp::min, i32};

pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let mut start = 0;
    let mut end = 0;

    let mut current_sum = 0;
    let mut min_len = i32::MAX;

    if nums.len() == 1 {
        return if nums[0] >= target { 1 } else { 0 };
    }

    while end < nums.len() {
        current_sum += nums[end];
        while current_sum >= target {
            min_len = min(
                min_len,
                i32::try_from(end - start + 1).expect("Out of bound for i32"),
            );

            // Move start + update current_sum
            current_sum -= nums[start];
            start += 1;
        }
        end += 1;
    }

    if min_len == i32::MAX {
        0
    } else {
        min_len
    }
}

#[cfg(test)]
mod test {
    use super::min_sub_array_len;

    #[test]
    fn test_min_sub_array_len() {
        let test_cases: Vec<(i32, Vec<i32>, i32)> = vec![
            (4, vec![4], 1),
            (4, vec![3], 0),
            (7, vec![2, 3, 1, 2, 4, 3], 2),
            (4, vec![1, 4, 4], 1),
            (11, vec![1, 1, 1, 1, 1, 1, 1, 1], 0),
        ];

        for (target, nums, expected) in test_cases {
            println!("Test case: {:?} target = {}", nums, target);
            let result = min_sub_array_len(target, nums);
            assert_eq!(result, expected);
        }
    }
}
