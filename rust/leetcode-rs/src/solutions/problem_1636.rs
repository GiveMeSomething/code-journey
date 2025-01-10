use std::collections::HashMap;

pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
    let mut frequency_map: HashMap<i32, i32> = HashMap::new();
    for num in &nums {
        let count = frequency_map.entry(*num).or_insert(0);
        *count += 1;
    }

    let mut result = nums.to_vec();
    result.sort_by(|a, b| {
        let frequency_a = *frequency_map.get(a).unwrap_or(&0);
        let frequency_b = *frequency_map.get(b).unwrap_or(&0);
        if frequency_a == frequency_b {
            return b.cmp(a);
        }
        return frequency_a.cmp(&frequency_b);
    });

    result
}

#[cfg(test)]
mod test {
    use super::frequency_sort;

    #[test]
    fn test_frequency_sort() {
        let test_cases: Vec<(Vec<i32>, Vec<i32>)> = vec![
            (vec![1, 1, 2, 2, 2, 3], vec![3, 1, 1, 2, 2, 2]),
            (vec![2, 3, 1, 3, 2], vec![1, 3, 3, 2, 2]),
            (
                vec![-1, 1, -6, 4, 5, -6, 1, 4, 1],
                vec![5, -1, 4, 4, -6, -6, 1, 1, 1],
            ),
        ];

        for (nums, expected) in test_cases {
            let result = frequency_sort(nums);

            assert_eq!(result.len(), expected.len());
            for i in 0..result.len() {
                assert_eq!(result[i], expected[i]);
            }
        }
    }
}
