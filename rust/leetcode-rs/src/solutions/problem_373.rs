use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

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
    let mut min_heap: BinaryHeap<Reverse<NumPair>> = BinaryHeap::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    // Init min_heap with the smallest pair
    min_heap.push(Reverse(NumPair {
        v1: nums1[0],
        v2: nums2[0],
        i1: 0,
        i2: 0,
    }));

    let limit = usize::try_from(k).expect("k is not usize");
    while result.len() < limit {
        let current_min = min_heap.pop().unwrap().0;
        let (i1, i2) = (current_min.i1, current_min.i2);

        if i1 + 1 < nums1.len() && !visited.contains(&(i1 + 1, i2)) {
            min_heap.push(Reverse(NumPair {
                v1: nums1[i1 + 1],
                v2: nums2[i2],
                i1: i1 + 1,
                i2,
            }));
            visited.insert((i1 + 1, i2));
        }

        if i2 + 1 < nums2.len() && !visited.contains(&(i1, i2 + 1)) {
            min_heap.push(Reverse(NumPair {
                v1: nums1[i1],
                v2: nums2[i2 + 1],
                i1,
                i2: i2 + 1,
            }));
            visited.insert((i1, i2 + 1));
        }

        result.push(vec![current_min.v1, current_min.v2]);
    }

    result
}

#[cfg(test)]
mod test {
    use super::k_smallest_pairs;

    #[test]
    fn test_k_smallest_pairs() {
        let test_cases: Vec<(Vec<i32>, Vec<i32>, i32, Vec<Vec<i32>>)> = vec![
            (
                vec![1, 7, 11],
                vec![2, 4, 6],
                3,
                vec![vec![1, 2], vec![1, 4], vec![1, 6]],
            ),
            (
                vec![1, 1, 2],
                vec![1, 2, 3],
                2,
                vec![vec![1, 1], vec![1, 1]],
            ),
        ];

        for (nums1, nums2, k, expected) in test_cases {
            let actual = k_smallest_pairs(nums1, nums2, k);

            assert_eq!(actual.len(), expected.len());
            for i in 0..expected.len() {
                assert_eq!(actual[i][0] + actual[i][1], expected[i][0] + expected[i][1]);
            }
        }
    }
}
