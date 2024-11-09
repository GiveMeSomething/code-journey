// Implementation of min-heap

use std::fmt::Debug;

use super::btree::BTree;

struct MinHeap {
    pub array: Vec<isize>,
}

impl MinHeap {
    pub fn new(input: &Vec<isize>) -> MinHeap {
        let heap_array = vec![];
        let mut min_heap = MinHeap { array: heap_array };

        for value in input {
            min_heap.insert(*value);
        }
        return min_heap;
    }

    pub fn insert(&mut self, value: isize) {
        let mut i = self.array.len() - 1;
        while i > 0 && self.array[BTree::parent(i)] > self.array[i] {
            println!(
                "Parent {}, Current {}",
                self.array[BTree::parent(i)],
                self.array[i]
            );
            (self.array[BTree::parent(i)], self.array[i]) =
                (self.array[i], self.array[BTree::parent(i)]);
            i = BTree::parent(i);
        }
    }

    #[allow(dead_code)]
    pub fn heapify(&mut self, root: usize) {
        let left = BTree::left(root);
        let right = BTree::right(root);
        let limit = self.array.len();

        // Skip running out-of-bound indes
        if root >= limit {
            return;
        }

        let mut min_index = root;
        if left < limit && self.array[root] > self.array[min_index] {
            min_index = left;
        }
        if right < limit && self.array[root] > self.array[min_index] {
            min_index = right;
        }

        // Swap root with min_index
        if min_index != root {
            (self.array[root], self.array[min_index]) = (self.array[min_index], self.array[root]);
            self.heapify(min_index);
        }
    }
}

impl Debug for MinHeap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.array)
    }
}

#[cfg(test)]
mod test {
    use crate::dsa::btree::BTree;

    use super::MinHeap;

    #[test]
    fn test_min_heap_property() {
        let test_cases: Vec<Vec<isize>> = vec![
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            vec![10, 2, 3, 4, 5, 6, 7, 8, 9, 1],
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 10],
            vec![1, 2, 3, 9, 5, 6, 7, 8, 10, 4],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 10, 9],
        ];

        for test_case in test_cases {
            let min_heap = MinHeap::new(&test_case);

            // Every node must be greater than their parent's
            for i in 0..min_heap.array.len() {
                if i == 0 {
                    continue;
                }

                assert!(min_heap.array[i] >= min_heap.array[BTree::parent(i)]);
            }
        }
    }
}
