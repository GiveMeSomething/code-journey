use super::max_heap::MaxHeap;

// This will sort ascendingly
pub fn heap_sort(input: &mut Vec<isize>) {
    let mut max_heap = MaxHeap::new(input);
    println!("Max heap: {:?}", max_heap);

    let limit = max_heap.array.len() - 1;
    for i in 0..=limit {
        // Swap the root with the last node
        (max_heap.array[0], max_heap.array[limit - i]) =
            (max_heap.array[limit - i], max_heap.array[0]);

        max_heap.limit_heapify(0, limit - i);
    }
}

#[cfg(test)]
mod test {
    use super::heap_sort;

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
            heap_sort(test_case);

            // Check
            for j in 1..test_case.len() {
                assert!(test_case[j] >= test_case[j - 1]);
            }
        }
    }
}
