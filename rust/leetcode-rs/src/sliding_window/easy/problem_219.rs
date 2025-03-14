use std::collections::HashSet;

pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let mut start = 0;
    let mut end = 0;

    let mut num_set: HashSet<i32> = HashSet::new();

    while end < nums.len() {
        if (end - start) as i32 > k {
            num_set.remove(&nums[start]);
            start += 1;
        }

        let is_new = num_set.insert(nums[end]);
        if !is_new {
            return true;
        }

        end += 1;
    }

    false
}

#[cfg(test)]
mod test {
    use super::contains_nearby_duplicate;

    #[test]
    fn test_contains_nearby_duplicate() {
        let test_cases: Vec<(Vec<i32>, i32, bool)> = vec![
            (vec![1, 2, 3, 1], 3, true),
            (vec![1, 0, 1, 1], 1, true),
            (vec![1, 2, 3, 1, 2, 3], 2, false),
        ];

        for (nums, k, expected) in test_cases {
            println!("nums = {:?}, k = {}", nums, k);
            let result = contains_nearby_duplicate(nums, k);
            assert_eq!(result, expected);
        }
    }
}
