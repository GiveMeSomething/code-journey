use std::{collections::HashMap, vec};

#[allow(dead_code)]
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut sum_map: HashMap<i32, usize> = HashMap::new();
    for i in 0..nums.len() {
        let current_number = nums[i];
        let remain = target - current_number;
        if sum_map.contains_key(&remain) {
            return vec![
                i32::try_from(i).unwrap(),
                i32::try_from(sum_map.get(&remain).unwrap().to_owned()).unwrap(),
            ];
        }

        sum_map.insert(current_number, i);
    }

    return vec![0, 0];
}

#[cfg(test)]
mod tests {
    use super::two_sum;

    #[test]
    fn test_two_sum() {
        let test_cases: Vec<(Vec<i32>, i32, Vec<i32>)> = vec![
            (vec![2, 7, 11, 15], 9, vec![0, 1]),
            (vec![3, 2, 4], 6, vec![1, 2]),
            (vec![3, 3], 6, vec![0, 1]),
        ];

        for (input, target, expected) in test_cases {
            let actual = two_sum(input, target);

            assert_eq!(actual.len(), expected.len());
            for number in expected {
                assert_eq!(actual.contains(&number), true);
            }
        }
    }
}
