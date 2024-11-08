#[allow(dead_code)]
pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut start: usize = 0;
    let mut end: usize = nums.len() - 1;

    while start < end {
        let pivot = (start + end) / 2;
        if nums[pivot] == target {
            return pivot as i32;
        }
        if target > nums[pivot] {
            start = pivot + 1;
        } else {
            end = pivot;
        }
    }

    println!("{} {}", start, end);

    if target <= nums[end] {
        return end as i32;
    }

    return (end + 1) as i32;
}

#[cfg(test)]
mod tests {
    use super::search_insert;

    #[test]
    fn test_search_insert() {
        let test_cases: Vec<(Vec<i32>, i32, i32)> = vec![
            (vec![1, 3, 5, 6], 5, 2),
            (vec![1, 3, 5, 6], 2, 1),
            (vec![1, 3, 5, 6], 7, 4),
            (vec![1, 3, 5, 6, 7, 8, 9, 123, 465, 1235], 100, 7),
            (vec![1, 3, 5, 6, 7, 8, 9, 123, 465, 1235], 400, 8),
            (vec![1, 3, 5, 6, 7, 8, 9, 123, 465, 1235], 1000, 9),
            (vec![1, 3], 0, 0),
            (vec![1, 3, 5, 6], 0, 0),
            (vec![1], 1, 0),
        ];

        let mut i = 0;
        for (input, target, expected) in test_cases {
            println!("Test case {:?} {} {}", input, target, expected);
            i = i + 1;
            let actual = search_insert(input, target);
            assert_eq!(actual, expected);
        }
    }
}
