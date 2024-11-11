use std::cmp::min;

use crate::dsa::btree::BTree;

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

pub fn insert(nums: &mut Vec<i32>, value: i32) {
    nums.push(value);

    let mut i = nums.len() - 1;
    while i > 0 && nums[i] > nums[BTree::parent(i)] {
        (nums[i], nums[BTree::parent(i)]) = (nums[BTree::parent(i)], nums[i]);
        i = BTree::parent(i);
    }
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::nth_ugly_number;

    #[test]
    fn test_nth_ugly_number() {
        let test_cases: Vec<(i32, i32)> = vec![(10, 12), (1, 1), (1690, 2123366400)];

        for (n, expected) in test_cases {
            let actual = nth_ugly_number(n);
            assert_eq!(actual, expected);
        }
    }
}
