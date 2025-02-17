use std::{
    cmp::{max, min},
    collections::VecDeque,
    i32,
};

// Time Limit Exceeded as we are iterating to find min/max everytime
// We need a better way to track min/max when the window is moving
pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
    let mut current_min = i32::MAX;
    let mut current_max = 0;
    let mut start = 0;
    let mut end = 0;
    let mut max_subarray_len = 0;

    while end < nums.len() {
        current_min = min(current_min, nums[end]);
        current_max = max(current_max, nums[end]);

        let mut diff = (current_max - current_min).abs();

        // Handle abnormal
        if diff > limit {
            // Try to increase start until the current subarray is valid again
            while diff > limit && start < end {
                if nums[start] == current_min {
                    current_min = i32::MAX;
                    for i in start + 1..=end {
                        current_min = min(current_min, nums[i]);
                    }
                }

                if nums[start] == current_max {
                    current_max = 0;
                    for i in start + 1..=end {
                        current_max = max(current_max, nums[i]);
                    }
                }

                diff = (current_max - current_min).abs();
                start += 1;
            }

            end += 1;
            continue;
        }

        // Hanlde normal case
        max_subarray_len = max(max_subarray_len, end - start + 1);
        end += 1;
    }

    max_subarray_len as i32
}

// As we need only one min/max, I always think about Heap
// BinaryHeap in Rust does not support search ops, so finding and deleting one have complexity of O(n) (no difference compared to above approach)
// pub fn longest_subarray_v2(nums: Vec<i32>, limit: i32) -> i32 {
//     let mut max_heap: BinaryHeap<i32> = BinaryHeap::new();
//     let mut min_heap: BinaryHeap<i32> = BinaryHeap::new();

//     let mut start = 0;
//     let mut end = 0;
//     let mut max_subarray_len = 0;

//     while end < nums.len() {
//         max_heap.push(nums[end]);
//         min_heap.push(Reverse(nums[end]));

//         let mut diff = (max_heap.peek() - min_heap.peek()).abs();

//         // Handle abnormal
//         if diff > limit {
//             // Try to increase start until the current subarray is valid again
//             while diff > limit && start < end {
//                 // Delete the element and heapify again
//             }

//             end += 1;
//             continue;
//         }

//         // Hanlde normal case
//         max_subarray_len = max(max_subarray_len, end - start + 1);
//         end += 1;
//     }

//     max_subarray_len as i32
// }

// Optimal way: VecDequeue
// VecDequeue way track the next min or the next max in the current subarray
pub fn longest_subarray_v3(nums: Vec<i32>, limit: i32) -> i32 {
    let mut max_queue: VecDeque<i32> = VecDeque::new();
    let mut min_queue: VecDeque<i32> = VecDeque::new();

    let mut start = 0;
    let mut end = 0;
    let mut max_subarray_len = 0;

    while end < nums.len() {
        // Keep max_queue in decreasing order
        // Keep min_queue in increasing order
        while !max_queue.is_empty() && *max_queue.back().unwrap() < nums[end] {
            max_queue.pop_back();
        }
        max_queue.push_back(nums[end]);

        while !min_queue.is_empty() && *min_queue.back().unwrap() > nums[end] {
            min_queue.pop_back();
        }
        min_queue.push_back(nums[end]);

        while {
            let current_max = match max_queue.front() {
                Some(value) => *value,
                None => 0,
            };
            let current_min = match min_queue.front() {
                Some(value) => *value,
                None => 0,
            };
            (current_max - current_min).abs() > limit
        } {
            if !max_queue.is_empty() && nums[start] == *max_queue.front().unwrap() {
                max_queue.pop_front();
            }
            if !min_queue.is_empty() && nums[start] == *min_queue.front().unwrap() {
                min_queue.pop_front();
            }
            start += 1;
        }

        max_subarray_len = max(max_subarray_len, end - start + 1);
        end += 1;
    }

    max_subarray_len as i32
}

#[cfg(test)]
mod test {
    use super::longest_subarray;

    #[test]
    fn test_longest_subarray() {
        let test_cases: Vec<(Vec<i32>, i32, i32)> = vec![
            (vec![8, 2, 4, 7], 4, 2),
            (vec![10, 1, 2, 4, 7, 2], 5, 4),
            (vec![4, 2, 2, 2, 4, 4, 2, 2], 0, 3),
            (vec![2, 2, 2, 4, 4, 2, 5, 5, 5, 5, 5, 2], 2, 6),
        ];

        for (nums, limit, expected) in test_cases {
            println!("TEST CASE {:?}", nums);
            let result = longest_subarray(nums, limit);

            println!("Result {}", result);
            assert_eq!(result, expected);
        }
    }
}
