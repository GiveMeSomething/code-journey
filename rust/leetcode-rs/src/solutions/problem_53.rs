use std::cmp::max;

pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut max_sum = nums[0];
    let mut current_sum = 0;

    for num in nums {
        current_sum += num;
        if num > current_sum {
            current_sum = num;
        }
        max_sum = max(max_sum, current_sum);
    }

    max_sum
}

#[cfg(test)]
mod test {
    use super::max_sub_array;

    #[test]
    fn test_max_sub_array() {
        let test_cases: Vec<(Vec<i32>, i32)> = vec![
            (vec![-2, 1, -3, 4, -1, 2, 1, -5, 4], 6),
            (vec![1], 1),
            (vec![5, 4, -1, 7, 8], 23),
        ];

        for (nums, expected) in test_cases {
            let result = max_sub_array(nums);
            assert_eq!(result, expected);
        }
    }
}
