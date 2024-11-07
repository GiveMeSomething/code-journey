#[allow(dead_code)]
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut current_index: usize = 0;
    let mut count = 1;
    for i in 0..nums.len() {
        if nums[i] == nums[current_index] {
            continue;
        }

        nums[current_index + 1] = nums[i];
        current_index += 1;
        count += 1;
    }

    return count;
}

#[cfg(test)]
mod tests {
    use super::remove_duplicates;

    #[test]
    fn test_remove_duplicates() {
        let mut test_cases: Vec<(Vec<i32>, i32, Vec<i32>)> = vec![
            (vec![1, 1, 2], 2, vec![1, 2]),
            (vec![1], 1, vec![1]),
            (vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4], 5, vec![0, 1, 2, 3, 4]),
        ];

        for i in 0..test_cases.len() {
            let (input, expected, expected_nums) = test_cases.get_mut(i).unwrap();
            let count = remove_duplicates(input.as_mut());

            assert_eq!(count, *expected);

            for j in 0..expected_nums.len() {
                assert_eq!(input[j], expected_nums[j]);
            }
        }
    }
}
