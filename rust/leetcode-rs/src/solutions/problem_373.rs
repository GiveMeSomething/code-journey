use std::{cmp::Reverse, collections::BinaryHeap};

pub struct NumPair {
    pub v1: i32,
    pub v2: i32,
    pub i1: usize,
    pub i2: usize,
}

impl NumPair {
    pub fn sum(&self) -> i32 {
        self.v1 + self.v2
    }
}

impl PartialEq for NumPair {
    fn eq(&self, other: &Self) -> bool {
        self.sum().eq(&other.sum())
    }
}

impl Eq for NumPair {}

impl PartialOrd for NumPair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.sum().partial_cmp(&other.sum())
    }
}

impl Ord for NumPair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.sum().cmp(&other.sum())
    }
}

pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];
    let min_heap: BinaryHeap<Reverse<NumPair>> = BinaryHeap::new();

    // Init min_heap with the smallest pair
    min_heap.push(Reverse(NumPair {
        v1: nums1[0],
        v2: nums2[0],
        i1: 0,
        i2: 0,
    }));

    let limit = usize::try_from(k).expect("k is not usize");
    while result.len() < k {
        let current_min = min_heap.pop().unwrap().0;
        let (i1, i2) = (current_min.i1, current_min.i2);

        min_heap.push(Reverse(NumPair {
            v1: nums1[i1 + 1],
            v2: nums2[i2],
            i1: i1 + 1,
            i2,
        }));
        min_heap.push(Reverse(NumPair {
            v1: nums1[i1],
            v2: nums2[i2 + 1],
            i1,
            i2: i2 + 1,
        }));

        result.push(vec![current_min.v1, current_min.v2]);

        println!("{:?}", result);
    }

    result
}
