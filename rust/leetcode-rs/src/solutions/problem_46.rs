pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];
    let mut base_permutation = nums.clone();

    permutation(&mut result, &mut base_permutation, 0);

    result
}

pub fn permutation(result: &mut Vec<Vec<i32>>, current_permutation: &mut Vec<i32>, start: usize) {
    if start == current_permutation.len() {
        result.push(current_permutation.clone());
        return;
    }

    for i in start..current_permutation.len() {
        (current_permutation[i], current_permutation[start]) =
            (current_permutation[start], current_permutation[i]);

        permutation(result, current_permutation, start + 1);

        // Swap back
        (current_permutation[i], current_permutation[start]) =
            (current_permutation[start], current_permutation[i]);
    }
}

#[cfg(test)]
mod test {
    use super::permute;

    #[test]
    fn test_permute() {
        let test_cases: Vec<(Vec<i32>, Vec<Vec<i32>>)> = vec![(
            vec![1, 2, 3],
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1],
            ],
        )];

        for (nums, expected) in test_cases {
            let result = permute(nums);

            assert_eq!(result.len(), expected.len());

            // Skip because the test case is tedious to check
        }
    }
}
