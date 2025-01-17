pub fn does_valid_array_exist(derived: Vec<i32>) -> bool {
    let mut result = 0;
    for num in derived {
        result ^= num;
    }
    result == 0
}

#[cfg(test)]
mod test {
    use super::does_valid_array_exist;

    #[test]
    fn test_does_valid_array_exist() {
        let test_cases: Vec<(Vec<i32>, bool)> = vec![
            (vec![1, 1, 0], true),
            (vec![1, 1], true),
            (vec![0, 1], false),
        ];

        for (nums, expected) in test_cases {
            let result = does_valid_array_exist(nums);
            assert_eq!(result, expected);
        }
    }
}
