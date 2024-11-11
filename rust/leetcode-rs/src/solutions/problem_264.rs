use std::{
    cmp::{min, Reverse},
    collections::{BinaryHeap, HashSet},
};

// DP
pub fn nth_ugly_number(n: i32) -> i32 {
    // Init simple heap with enough value to avoid index-out-of-bound
    let mut result: Vec<i32> = vec![1];

    let mut i1 = 0;
    let mut i2 = 0;
    let mut i3 = 0;
    while result.len() < usize::try_from(n).unwrap() {
        let v1 = result[i1] * 2;
        let v2 = result[i2] * 3;
        let v3 = result[i3] * 5;

        let next = min(min(v1, v2), v3);
        result.push(next);
        if next == v1 {
            i1 += 1;
        }
        if next == v2 {
            i2 += 1;
        }
        if next == v3 {
            i3 += 1;
        }
    }

    return *result.last().unwrap();
}

// Using collections, this solution run with huge overhead of creating a min_heap + a hash set
pub fn nth_ugly_number_heap(n: i32) -> i32 {
    let primes: Vec<i32> = vec![2, 3, 5];

    // Simple min_heap
    let mut min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    let mut visited: HashSet<i32> = HashSet::new();
    min_heap.push(Reverse(1));

    let mut current = 0;
    for _ in 0..n {
        current = min_heap.pop().unwrap().0;
        for j in 0..primes.len() {
            let value = match current.checked_mul(primes[j]) {
                Some(value) => value,
                None => continue,
            };
            if !visited.contains(&value) {
                min_heap.push(Reverse(value));
            }

            visited.insert(value);
        }
    }

    return current;
}

#[cfg(test)]
mod test {
    use std::vec;

    use crate::solutions::problem_264::nth_ugly_number_heap;

    use super::nth_ugly_number;

    #[test]
    fn test_nth_ugly_number() {
        let test_cases: Vec<(i32, i32)> = vec![(10, 12), (1, 1), (1690, 2123366400)];

        for (n, expected) in test_cases {
            let actual = nth_ugly_number(n);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_nth_ugly_number_heap() {
        let test_cases: Vec<(i32, i32)> = vec![(10, 12), (1, 1), (1690, 2123366400)];

        for (n, expected) in test_cases {
            let actual = nth_ugly_number_heap(n);
            assert_eq!(actual, expected);
        }
    }
}
