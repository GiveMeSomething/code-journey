use std::collections::VecDeque;

/**
 * Min/Max queues are used in problem where we need to know the
 * next smallest/largest number in some iterated sequence in O(1) time
 *
 * The smallest/largerst number will always be in the front, following with the next smallest/largest, and so on
 *
 * It will totally depends on the problems, but it's a way to track min/max in some sequence
 *
 * for example: [5,1,4,8,2]
 * the largest is 8 -> next largest is 5
 */
#[allow(dead_code)]
fn create_max_queue(nums: Vec<i32>) -> Vec<i32> {
    let mut max_queue: VecDeque<i32> = VecDeque::new();
    for num in nums {
        while !max_queue.is_empty() && num > *max_queue.back().unwrap() {
            max_queue.pop_back();
        }
        max_queue.push_back(num);
    }

    max_queue.into_iter().collect()
}
