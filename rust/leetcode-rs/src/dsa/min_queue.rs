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
fn create_min_queue(arr: Vec<i32>) -> Vec<i32> {
    let mut min_queue: VecDeque<i32> = VecDeque::new();
    for num in arr {
        while !min_queue.is_empty() && num < *min_queue.back().unwrap() {
            min_queue.pop_back();
        }
        min_queue.push_back(num);
    }

    min_queue.into_iter().collect()
}

#[cfg(test)]
mod test {
    use crate::dsa::min_queue::create_min_queue;

    #[test]
    fn test_create_min_queue() {
        let test_cases: Vec<Vec<i32>> = vec![vec![5, 1, 4, 8, 2]];

        for nums in test_cases {
            println!("result {:?}", create_min_queue(nums));
        }
    }
}
