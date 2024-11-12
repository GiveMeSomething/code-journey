use std::collections::{BinaryHeap, HashMap};

pub struct NumCount {
    value: i32,
    count: i32,
}

impl PartialEq for NumCount {
    fn eq(&self, other: &Self) -> bool {
        self.count.eq(&other.count)
    }
}

impl PartialOrd for NumCount {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.count.partial_cmp(&other.count)
    }
}

impl Eq for NumCount {}

impl Ord for NumCount {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.count.cmp(&other.count)
    }
}

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut count_map: HashMap<i32, i32> = HashMap::new();
    for num in nums {
        let current_count = match count_map.get(&num) {
            Some(value) => *value,
            None => 0,
        };

        count_map.insert(num, current_count + 1);
    }

    let mut max_heap: BinaryHeap<NumCount> = BinaryHeap::new();
    for (value, count) in count_map {
        max_heap.push(NumCount { value, count });
    }

    let mut result: Vec<i32> = vec![];
    for _ in 0..k {
        let num_count = max_heap.pop().unwrap();
        result.push(num_count.value);
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
            (vec![1, 2], 2, vec![1, 2]),
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
