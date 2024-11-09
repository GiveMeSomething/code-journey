use super::{btree::BTree, max_heap::MaxHeap};

// This will sort ascendingly
pub fn heap_sort(input: &Vec<isize>) -> Vec<isize> {
    let mut max_heap = MaxHeap::new(input);

    let limit = max_heap.array.len() - 1;
    for i in 0..=limit {
        // Swap the root with the last node
        (max_heap.array[0], max_heap.array[limit - i]) =
            (max_heap.array[limit - i], max_heap.array[0]);

        max_heap.limit_heapify(0, limit - i);
    }

    return max_heap.array;
}

pub fn heapify(input: &mut Vec<isize>, root: usize, limit: usize) {
    let left = BTree::left(root);
    let right = BTree::right(root);

    let mut max = root;
    if left < limit && input[left] > input[max] {
        max = left;
    }
    if right < limit && input[right] > input[max] {
        max = right;
    }

    if max != root {
        (input[root], input[max]) = (input[max], input[root]);
        heapify(input, max, limit);
    }
}

pub fn inplace_heap_sort(input: &mut Vec<isize>) {
    println!("Input {:?}", input);

    // Build max_heap: Take O(n)
    let limit = input.len() - 1;
    for i in 0..=limit {
        heapify(input, limit - i, limit + 1);
    }

    println!("Max heap {:?}", input);

    // Normal heap_sort
    // Traverse through all elements: O(n)
    for i in 0..=limit {
        (input[0], input[limit - i]) = (input[limit - i], input[0]);

        println!("{:?}", input);

        // This take O(logn)
        heapify(input, 0, limit - i);
        println!("{:?}", input);
        println!();
    }
}

#[cfg(test)]
mod test {
    use super::{heap_sort, inplace_heap_sort};

    #[test]
    fn test_heap_sort() {
        let mut test_cases: Vec<Vec<isize>> = vec![
            vec![9, 3, 7, 5, 1],
            vec![25, 12, 22, 11, 9],
            vec![100, 50, 75, 25, 60],
            vec![4, 2, 8, 6, 10],
            vec![15, 30, 45, 10, 20],
        ];

        for i in 0..test_cases.len() {
            let test_case = test_cases.get_mut(i).expect("invalid index");
            let result = heap_sort(test_case);

            // Check
            for j in 1..result.len() {
                assert!(result[j] > result[j - 1]);
            }
        }
    }

    #[test]
    fn test_inplace_heap_sort() {
        let mut test_cases: Vec<Vec<isize>> = vec![
            vec![9, 3, 7, 5, 1],
            vec![25, 12, 22, 11, 9],
            vec![100, 50, 75, 25, 60],
            vec![4, 2, 8, 6, 10],
            vec![15, 30, 45, 10, 20],
        ];

        for i in 0..test_cases.len() {
            let test_case = test_cases.get_mut(i).expect("invalid index");
            inplace_heap_sort(test_case);

            // Check
            for j in 1..test_case.len() {
                assert!(test_case[j] > test_case[j - 1]);
            }
        }
    }
}
