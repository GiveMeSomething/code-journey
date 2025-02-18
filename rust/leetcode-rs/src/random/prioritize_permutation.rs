use std::collections::HashSet;

/**
 * Generate a set of permutations from a primary, secondary, and tertiary set of items
 * 1. Items in the primary set can be joined with the secondary set to form permutation
 * 2. Items in the primary set must come first in any permutation. Contain AT MOST 1 item from another permutation
 * 3. Permutation cannot be form with the same item
 *
 * For ease of implementation, every item in set is a digit 0 - 9
 */

pub fn prioritize_permutation(sets: Vec<Vec<i32>>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];

    recur_prioritize_perm(&sets, &mut result, 0, 0);

    result
}

pub fn recur_prioritize_perm(
    sets: &Vec<Vec<i32>>,
    result: &mut Vec<i32>,
    index: usize,
    current: i32,
) {
    println!(
        "Recur {:?} {:?} index {} current {}",
        sets, result, index, current
    );

    if sets.len() < index + 1 {
        return;
    }

    // For every set after this set

    for i in 0..sets[index].len() {
        let current_perm = current * 10 + sets[index][i];
        if !is_valid_perm(current_perm) {
            continue;
        }

        result.push(current_perm);

        for j in index + 1..sets.len() {
            recur_prioritize_perm(sets, result, j, current_perm);
        }
    }
}

pub fn is_valid_perm(mut num: i32) -> bool {
    if num == 0 {
        return true;
    }

    let mut num_set: HashSet<i32> = HashSet::new();
    while num > 0 {
        let ok = num_set.insert(num % 10);
        if !ok {
            return false;
        }
        num /= 10;
    }

    return true;
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use super::prioritize_permutation;

    #[test]
    fn test_prioritize_permutation() {
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
}
