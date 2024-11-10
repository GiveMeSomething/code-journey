use std::fmt::Debug;

use super::btree::BTree;

pub struct MaxHeap<T: PartialOrd + Clone> {
    pub array: Vec<T>,
}

impl<T> Debug for MaxHeap<T>
where
    T: PartialOrd + Debug + Copy,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.array)
    }
}

impl<T> From<Vec<T>> for MaxHeap<T>
where
    T: PartialOrd + Copy,
{
    fn from(values: Vec<T>) -> Self {
        let mut max_heap: MaxHeap<T> = MaxHeap::new();
        for value in values {
            max_heap.insert(value);
        }
        return max_heap;
    }
}

impl<T> MaxHeap<T>
where
    T: PartialOrd + Copy,
{
    pub fn new() -> MaxHeap<T> {
        MaxHeap { array: vec![] }
    }

    pub fn insert(&mut self, value: T) {
        self.array.push(value);

        let mut i = self.array.len() - 1;
        while i > 0 && self.array[BTree::parent(i)] < self.array[i] {
            let parent = BTree::parent(i);
            (self.array[parent], self.array[i]) = (self.array[i], self.array[parent]);
            i = parent;
        }
    }

    pub fn pop_max(&mut self) -> T {
        let last = self.array.len() - 1;
        (self.array[0], self.array[last]) = (self.array[last], self.array[0]);
        let max = self.array.pop().expect("array is empty");
        self.heapify(0, last + 1);
        return max;
    }

    pub fn heapify(&mut self, root: usize, limit: usize) {
        let left = BTree::left(root);
        let right = BTree::right(root);
        let mut max = root;
        if left < limit && self.array[max] < self.array[left] {
            max = left;
        }
        if right < limit && self.array[max] < self.array[right] {
            max = right;
        }
        if max != root {
            (self.array[root], self.array[max]) = (self.array[max], self.array[root]);
            self.heapify(max, limit);
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
            let max_heap = MaxHeap::from(test_case);
            for i in 0..max_heap.array.len() {
                if i == 0 {
                    continue;
                }

                assert!(max_heap.array[i] < max_heap.array[BTree::parent(i)]);
            }
        }
    }
}
