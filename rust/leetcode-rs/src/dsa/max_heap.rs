use std::fmt::Debug;

use super::btree::BTree;

struct MaxHeap {
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
        while i > 0 && self.array[i] < self.array[BTree::parent(i)] {
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
}
