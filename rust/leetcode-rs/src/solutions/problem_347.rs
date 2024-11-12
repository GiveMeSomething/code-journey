pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    vec![]
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
