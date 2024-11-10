use std::collections::BinaryHeap;

pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let mut max_heap = BinaryHeap::from(nums);
    let mut result = 0;
    for _ in 0..k {
        result = max_heap.pop().unwrap();
    }
    result
}

#[cfg(test)]
mod test {
    use super::find_kth_largest;

    #[test]
    fn test_find_kth_largest() {
        let test_cases: Vec<(Vec<i32>, i32, i32)> = vec![
            (vec![3, 2, 1, 5, 6, 4], 2, 5),
            (vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4, 4),
        ];

        for (input, k, expected) in test_cases {
            let actual = find_kth_largest(input, k);
            assert_eq!(actual, expected);
        }
    }
}
