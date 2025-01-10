pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
    let mut start = 0;
    let mut slice_count = 0;

    while start + 2 < nums.len() {
        let mut end = start + 2;
        while end < nums.len() {
            if is_arithmetic_slice(&nums, start, end) {
                slice_count += 1;
            } else {
                break;
            }
            end += 1;
        }
        start += 1;
    }

    slice_count
}

pub fn is_arithmetic_slice(nums: &Vec<i32>, start: usize, end: usize) -> bool {
    let diff = nums[start + 1] - nums[start];
    for i in start..end {
        let current_diff = nums[i + 1] - nums[i];
        if current_diff != diff {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::number_of_arithmetic_slices;

    #[test]
    fn test_number_of_arithmetic_slices() {
        let test_cases: Vec<(Vec<i32>, i32)> = vec![(vec![1, 2, 3, 4], 3), (vec![1], 0)];

        for (nums, expected) in test_cases {
            println!("test case {:?}", &nums);
            let result = number_of_arithmetic_slices(nums);
            assert_eq!(result, expected);
        }
    }
}
