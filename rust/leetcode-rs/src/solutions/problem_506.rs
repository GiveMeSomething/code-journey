pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
    vec![]
}

#[cfg(test)]
mod test {
    use super::find_relative_ranks;

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
}
