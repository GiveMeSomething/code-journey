#[allow(dead_code)]
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut shift: usize = 0;

    for i in 0..nums.len() {
        if nums[i] == val {
            shift += 1;
            continue;
        }

        // Shift non-target value if needed
        nums[i - shift] = nums[i];
    }

    return i32::try_from(nums.len() - shift).unwrap();
}

#[cfg(test)]
mod tests {
    use super::remove_element;

    #[test]
    fn test_remove_element() {
        let mut test_cases: Vec<(Vec<i32>, i32, i32, Vec<i32>)> = vec![
            (vec![3, 2, 2, 3], 3, 2, vec![2, 2]),
            (vec![0, 1, 2, 2, 3, 0, 4, 2], 2, 5, vec![0, 1, 4, 0, 3]),
            (vec![], 0, 0, vec![]),
        ];

        for i in 0..test_cases.len() {
            let (input, target, expected, expected_nums) = test_cases.get_mut(i).unwrap();
            let actual = remove_element(input, *target);

            assert_eq!(actual, *expected);

            for j in 0..expected_nums.len() {
                assert_eq!(expected_nums.contains(&input[j]), true);
            }
        }
    }
}
