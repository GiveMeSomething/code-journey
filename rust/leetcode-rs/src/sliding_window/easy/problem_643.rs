pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let mut sum = 0;
    let mut result: f64 = f64::MIN;

    let k = k as usize;
    for i in 0..nums.len() {
        sum += nums[i];
        if i >= k {
            sum -= nums[i - k];
            result = result.max((sum as f64) / (k as f64));
        }

        if i == k - 1 {
            result = result.max((sum as f64) / (k as f64));
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::find_max_average;

    #[test]
    fn test_find_max_average() {
        let test_cases: Vec<(Vec<i32>, i32, f64)> = vec![
            (vec![1, 12, -5, -6, 50, 3], 4, 12.7500),
            (vec![5], 1, 5.00000),
            (vec![-1], 1, -1.00000),
            (vec![4, 0, 4, 3, 3], 5, 2.80000),
            (vec![9, 7, 3, 5, 6, 2, 0, 8, 1, 9], 6, 5.33333),
        ];

        for (nums, k, expected) in test_cases {
            let result = find_max_average(nums, k);

            let precision = f64::powi(10.0, 5);
            let precise_result = (result * precision).round() / precision;
            let precise_expected = (expected * precision).round() / precision;
            assert_eq!(precise_result, precise_expected);
        }
    }
}
