// We could use heap to pop max numbers, but that would be overkill
pub fn max_product(nums: Vec<i32>) -> i32 {
    let mut max_index = 0;
    let mut result = 1;

    let mut current_max = 0;
    for i in 0..nums.len() {
        if current_max < nums[i] {
            max_index = i;
            current_max = nums[i];
        }
    }

    result *= current_max - 1;
    current_max = 0;
    for i in 0..nums.len() {
        if current_max < nums[i] && i != max_index {
            current_max = nums[i];
        }
    }

    return result * (current_max - 1);
}

#[cfg(test)]
mod test {
    use super::max_product;

    #[test]
    fn test_max_product() {
        let test_cases: Vec<(Vec<i32>, i32)> = vec![
            (vec![3, 4, 5, 2], 12),
            (vec![1, 5, 4, 5], 16),
            (vec![3, 7], 12),
        ];

        for (input, expected) in test_cases {
            let actual = max_product(input);
            assert_eq!(actual, expected);
        }
    }
}
