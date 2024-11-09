use std::{
    collections::{BinaryHeap, HashMap},
    vec,
};

use crate::dsa::btree::BTree;

pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
    // Build heap from input array
    let mut input: Vec<(i32, usize)> = vec![];
    for i in 0..score.len() {
        input.push((score[i], i));
    }

    heap_sort(&mut input);

    let mut result: Vec<String> = vec![String::from(""); score.len()];
    for i in 0..score.len() {
        let (_, index) = input[i];
        match i {
            0 => result[index] = "Gold Medal".to_string(),
            1 => result[index] = "Silver Medal".to_string(),
            2 => result[index] = "Bronze Medal".to_string(),
            value => result[index] = format!("{}", value + 1),
        };
    }
    result
}

pub fn find_relative_ranks_stdlib(score: Vec<i32>) -> Vec<String> {
    let mut input: Vec<(i32, usize)> = vec![];
    for i in 0..score.len() {
        input.push((score[i], i));
    }

    // Descending order
    input.sort_by(|&a, &b| b.0.cmp(&a.0));

    let mut result: Vec<String> = vec![String::from(""); score.len()];
    for i in 0..score.len() {
        let (_, index) = input[i];
        match i {
            0 => result[index] = "Gold Medal".to_string(),
            1 => result[index] = "Silver Medal".to_string(),
            2 => result[index] = "Bronze Medal".to_string(),
            value => result[index] = format!("{}", value + 1),
        };
    }
    result
}

pub fn find_relative_ranks_collection(scores: Vec<i32>) -> Vec<String> {
    let mut index_mapper: HashMap<i32, usize> = HashMap::new();
    for i in 0..scores.len() {
        index_mapper.insert(scores[i], i);
    }

    let mut heap: BinaryHeap<i32> = BinaryHeap::new();
    for i in 0..scores.len() {
        heap.push(scores[i]);
    }

    let mut result: Vec<String> = vec!["".to_string(); scores.len()];
    for i in 0..scores.len() {
        let max_score = heap.pop().expect("get heap root failed");
        let index = *index_mapper
            .get(&max_score)
            .expect("get index from mapper failed");

        match i {
            0 => result[index] = "Gold Medal".to_string(),
            1 => result[index] = "Silver Medal".to_string(),
            2 => result[index] = "Bronze Medal".to_string(),
            value => result[index] = format!("{}", value + 1),
        };
    }
    result
}

pub fn heapify(input: &mut Vec<(i32, usize)>, root: usize, limit: usize) {
    let left = BTree::left(root);
    let right = BTree::right(root);

    let mut max = root;
    if left < limit && input[left].0 < input[max].0 {
        max = left;
    }
    if right < limit && input[right].0 < input[max].0 {
        max = right;
    }

    if max != root {
        (input[max], input[root]) = (input[root], input[max]);
        heapify(input, max, limit);
    }
}

pub fn heap_sort(input: &mut Vec<(i32, usize)>) {
    let limit = input.len();
    for i in 0..limit {
        heapify(input, limit - i - 1, limit);
    }

    for i in 0..limit {
        (input[0], input[limit - i - 1]) = (input[limit - i - 1], input[0]);
        heapify(input, 0, limit - i - 1);
    }
}

#[cfg(test)]
mod test {
    use super::{find_relative_ranks, find_relative_ranks_collection, find_relative_ranks_stdlib};

    #[test]
    fn test_find_relative_ranks() {
        let test_cases: Vec<(Vec<i32>, Vec<&str>)> = vec![
            (
                vec![5, 4, 3, 2, 1],
                vec!["Gold Medal", "Silver Medal", "Bronze Medal", "4", "5"],
            ),
            (
                vec![10, 3, 8, 9, 4],
                vec!["Gold Medal", "5", "Bronze Medal", "Silver Medal", "4"],
            ),
        ];

        for (input, expected) in test_cases {
            let actual = find_relative_ranks(input);
            for i in 0..actual.len() {
                assert_eq!(actual[i], String::from(expected[i]));
            }
        }
    }

    #[test]
    fn test_find_relative_ranks_stdlib() {
        let test_cases: Vec<(Vec<i32>, Vec<&str>)> = vec![
            (
                vec![5, 4, 3, 2, 1],
                vec!["Gold Medal", "Silver Medal", "Bronze Medal", "4", "5"],
            ),
            (
                vec![10, 3, 8, 9, 4],
                vec!["Gold Medal", "5", "Bronze Medal", "Silver Medal", "4"],
            ),
        ];

        for (input, expected) in test_cases {
            let actual = find_relative_ranks_stdlib(input);
            for i in 0..actual.len() {
                assert_eq!(actual[i], String::from(expected[i]));
            }
        }
    }

    #[test]
    fn test_find_relative_ranks_collection() {
        let test_cases: Vec<(Vec<i32>, Vec<&str>)> = vec![
            (
                vec![5, 4, 3, 2, 1],
                vec!["Gold Medal", "Silver Medal", "Bronze Medal", "4", "5"],
            ),
            (
                vec![10, 3, 8, 9, 4],
                vec!["Gold Medal", "5", "Bronze Medal", "Silver Medal", "4"],
            ),
        ];

        for (input, expected) in test_cases {
            let actual = find_relative_ranks_collection(input);
            for i in 0..actual.len() {
                assert_eq!(actual[i], String::from(expected[i]));
            }
        }
    }
}
