use std::fmt::Debug;

use super::btree::BTree;

pub struct MaxHeap {
    pub array: Vec<isize>,
}

impl Debug for MaxHeap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.array)
    }
}

impl MaxHeap {
    pub fn new(input: &Vec<isize>) -> MaxHeap {
        let mut max_heap = MaxHeap { array: vec![] };

        for value in input {
            max_heap.insert(*value);
        }
        return max_heap;
    }

    pub fn insert(&mut self, value: isize) {
        self.array.push(value);

        let mut i = self.array.len() - 1;

        // Move the node up until parent no longer > value
        // This ensure that the root at any index always greater than their child
        while i > 0 && self.array[i] > self.array[BTree::parent(i)] {
            (self.array[i], self.array[BTree::parent(i)]) =
                (self.array[BTree::parent(i)], self.array[i]);
            i = BTree::parent(i);
        }
    }

    #[allow(dead_code)]
    pub fn heapify(&mut self, root: usize) {
        let left = BTree::left(root);
        let right = BTree::right(root);

        let mut max = root;
        let limit = self.array.len() - 1;
        if left < limit && self.array[left] > self.array[max] {
            max = left;
        }
        if right < limit && self.array[right] > self.array[max] {
            max = right;
        }

        if max != root {
            (self.array[max], self.array[root]) = (self.array[root], self.array[max]);
            self.heapify(max);
        }
    }

    #[allow(dead_code)]
    pub fn limit_heapify(&mut self, root: usize, limit: usize) {
        let left = BTree::left(root);
        let right = BTree::right(root);

        // Skip running out-of-bound indes
        if root >= limit {
            return;
        }

        let mut max = root;
        if left < limit && self.array[left] > self.array[max] {
            max = left;
        }
        if right < limit && self.array[right] > self.array[max] {
            max = right;
        }

        if max != root {
            (self.array[max], self.array[root]) = (self.array[root], self.array[max]);
            self.limit_heapify(max, limit);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::dsa::btree::BTree;

    use super::MaxHeap;

    #[test]
    fn test_max_heap_property() {
        let test_cases: Vec<Vec<isize>> = vec![
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            vec![10, 2, 3, 4, 5, 6, 7, 8, 9, 1],
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 10],
            vec![1, 2, 3, 9, 5, 6, 7, 8, 10, 4],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 10, 9],
        ];

        for test_case in test_cases {
            let max_heap = MaxHeap::new(&test_case);
            for i in 0..max_heap.array.len() {
                if i == 0 {
                    continue;
                }

                assert!(max_heap.array[i] < max_heap.array[BTree::parent(i)]);
            }
        }
    }
}
