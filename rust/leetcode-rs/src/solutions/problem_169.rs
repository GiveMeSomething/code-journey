pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut current_majority = nums[0];
    let mut majority_score = 0;

    for num in nums {
        if num == current_majority {
            majority_score += 1;
        } else {
            majority_score -= 1;
        }

        if majority_score < 0 {
            majority_score = 0;
            current_majority = num;
        }
    }

    current_majority
}

#[cfg(test)]
mod test {
    use super::majority_element;

    #[test]
    fn test_majority_element() {
        let test_cases: Vec<(Vec<i32>, i32)> =
            vec![(vec![3, 2, 3], 3), (vec![2, 2, 1, 1, 1, 2, 2], 2)];

        for (nums, expected) in test_cases {
            let result = majority_element(nums);
            assert_eq!(result, expected);
        }
    }
}
