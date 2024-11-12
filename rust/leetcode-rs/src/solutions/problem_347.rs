use std::collections::{BinaryHeap, HashMap};

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut count_map: HashMap<i32, i32> = HashMap::new();
    for num in nums {
        let current_count = match count_map.get(&num) {
            Some(value) => *value,
            None => 0,
        };

        count_map.insert(num, current_count + 1);
    }

    let mut number_map: HashMap<i32, i32> = HashMap::new();
    let mut max_heap: BinaryHeap<i32> = BinaryHeap::new();
    for (key, value) in count_map {
        number_map.insert(value, key);
        max_heap.push(value);
    }

    let mut result: Vec<i32> = vec![];
    for _ in 0..k {
        result.push(*number_map.get(&max_heap.pop().unwrap()).unwrap());
    }
    result
}

#[cfg(test)]
mod test {
    use super::top_k_frequent;

    #[test]
    fn test_top_k_frequent() {
        let test_cases: Vec<(Vec<i32>, i32, Vec<i32>)> = vec![
            (vec![1, 1, 1, 2, 2, 3], 2, vec![1, 2]),
            (vec![1], 1, vec![1]),
        ];

        for (nums, k, expected) in test_cases {
            let actual = top_k_frequent(nums, k);

            assert_eq!(expected.len(), actual.len());

            for value in actual {
                assert_eq!(expected.contains(&value), true);
            }
        }
    }
}
