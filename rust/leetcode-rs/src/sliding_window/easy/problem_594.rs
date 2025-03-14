use std::collections::HashMap;

pub fn find_lhs(nums: Vec<i32>) -> i32 {
    let mut result = 0;
    let mut num_map: HashMap<i32, i32> = HashMap::new();
    for i in 0..nums.len() {
        let entry = num_map.entry(nums[i]).or_insert(0);
        *entry += 1;
    }
    for i in 0..nums.len() {
        let entry = *num_map.entry(nums[i]).or_insert(0);
        if let Some(max_count) = num_map.get(&(nums[i] + 1)) {
            result = result.max(*max_count + entry);
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::find_lhs;

    #[test]
    fn test_find_lhs() {
        let test_cases: Vec<(Vec<i32>, i32)> = vec![
            (vec![1, 3, 2, 2, 5, 2, 3, 7], 5),
            // (vec![1, 2, 3, 4], 2),
            // (vec![1, 1, 1, 1], 0),
        ];

        for (nums, expected) in test_cases {
            let result = find_lhs(nums);
            assert_eq!(result, expected);
        }
    }
}
