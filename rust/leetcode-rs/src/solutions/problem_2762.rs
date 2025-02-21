use std::{collections::VecDeque, i32};

pub fn continuous_subarrays(nums: Vec<i32>) -> i64 {
    let mut result = 0;
    let mut start = 0;
    let mut end = 0;

    let mut min_queue: VecDeque<i32> = VecDeque::new();
    let mut max_queue: VecDeque<i32> = VecDeque::new();

    while end < nums.len() {
        while !max_queue.is_empty() && nums[end] > *max_queue.back().unwrap() {
            max_queue.pop_back();
        }
        max_queue.push_back(nums[end]);

        while !min_queue.is_empty() && nums[end] < *min_queue.back().unwrap() {
            min_queue.pop_back();
        }
        min_queue.push_back(nums[end]);

        while *max_queue.front().unwrap() - *min_queue.front().unwrap() > 2 {
            // Move start until it's valid again
            if !min_queue.is_empty() && *min_queue.front().unwrap() == nums[start] {
                min_queue.pop_front();
            }
            if !max_queue.is_empty() && *max_queue.front().unwrap() == nums[start] {
                max_queue.pop_front();
            }

            start += 1;
        }

        println!("start {} end {}", start, end);
        result += end - start + 1;
        end += 1;
    }

    result as i64
}

#[cfg(test)]
mod test {
    use super::continuous_subarrays;

    #[test]
    fn test_continuous_subarrays() {
        let test_cases: Vec<(Vec<i32>, i64)> = vec![
            // (vec![5, 4, 2, 4], 8),
            // (vec![1, 2, 3], 6),
            (vec![35, 35, 36, 37, 36, 37, 38, 37, 38], 39),
        ];
        for (nums, expected) in test_cases {
            let result = continuous_subarrays(nums);
            assert_eq!(result, expected);
        }
    }
}
