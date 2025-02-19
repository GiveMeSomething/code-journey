pub fn increasing_triplet(nums: Vec<i32>) -> bool {
    let mut min_1 = i32::MAX;
    let mut min_2 = i32::MAX;

    for num in nums {
        if num <= min_1 {
            min_1 = num;
        } else if num <= min_2 {
            min_2 = num;
        } else {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::increasing_triplet;

    #[test]
    fn test_increasing_triplet() {
        let test_cases: Vec<(Vec<i32>, bool)> = vec![
            (vec![1, 2, 3, 4, 5], true),
            (vec![5, 4, 3, 2, 1], true),
            (vec![2, 1, 5, 0, 4, 6], true),
        ];

        for (nums, expected) in test_cases {
            let result = increasing_triplet(nums);
            assert_eq!(result, expected);
        }
    }
}
