use std::i32;

// This problem should be easy when using HashMap to count and return according to the requirement
// Otherwise, if follow-up with a question to solve this with O(n) time complexity + O(1) space complexity
// Use Boyee-Moore Voting Count algorithms. Don't even think about coming up with solution by your own
// https://www.geeksforgeeks.org/n3-repeated-number-array-o1-space/
pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
    let target = nums.len() / 3;
    let mut first_major = i32::MAX;
    let mut second_major = i32::MAX;

    let mut first_count = 0;
    let mut second_count = 0;

    for num in &nums {
        if *num == first_major {
            first_count += 1;
            continue;
        }
        if *num == second_major {
            second_count += 1;
            continue;
        }

        if first_count == 0 {
            first_major = *num;
            first_count = 1;
            continue;
        }
        if second_count == 0 {
            second_major = *num;
            second_count = 1;
            continue;
        }

        first_count -= 1;
        second_count -= 1;
    }

    let mut result: Vec<i32> = vec![];
    first_count = 0;
    second_count = 0;
    for num in &nums {
        if *num == first_major {
            first_count += 1;
        }
        if *num == second_major {
            second_count += 1;
        }
    }

    if first_count > target {
        result.push(first_major);
    }
    if second_count > target {
        result.push(second_major);
    }

    result
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use super::majority_element;

    #[test]
    fn test_majority_element() {
        let test_cases: Vec<(Vec<i32>, Vec<i32>)> = vec![
            (vec![3, 2, 3], vec![3]),
            (vec![1], vec![1]),
            (vec![1, 2], vec![1, 2]),
        ];

        for (nums, expected) in test_cases {
            let result = majority_element(nums);

            // Check for len first to avoid out-of-bound index
            assert_eq!(result.len(), expected.len());

            let expected_set: HashSet<i32> = HashSet::from_iter(expected);

            for i in 0..result.len() {
                assert!(expected_set.contains(&result[i]));
            }
        }
    }
}
