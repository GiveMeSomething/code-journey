use std::collections::HashSet;

use leetcode::random::prioritize_permutation::prioritize_permutation;

fn main() {
    let test_cases: Vec<(Vec<Vec<i32>>, Vec<i32>)> = vec![
        (vec![vec![1, 2], vec![2, 3]], vec![1, 2, 12, 13, 23]),
        (
            vec![vec![1, 2], vec![2, 3], vec![4]],
            vec![1, 2, 12, 13, 23, 14, 24, 124, 134, 234],
        ),
    ];

    for (sets, expected) in test_cases {
        let result = prioritize_permutation(sets);

        let num_set: HashSet<i32> = HashSet::from_iter(expected);
        assert_eq!(num_set.len(), result.len());
        for num in result {
            assert!(num_set.contains(&num));
        }
    }
}
