use std::collections::BinaryHeap;

pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let mut max_heap = BinaryHeap::from(nums);
    let mut result = 0;
    for _ in 0..k {
        result = max_heap.pop().unwrap();
    }
    result
}
