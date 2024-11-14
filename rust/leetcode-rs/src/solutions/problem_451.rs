pub fn frequency_sort(s: String) -> String {
    "".to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_frequency_sort() {
        let test_cases: Vec<(String, Vec<String>)> = vec![
            ("tree".to_string(), vec!["eert".to_string()]),
            (
                "cccaaa".to_string(),
                vec!["cccaaa".to_string(), "aaaccc".to_string()],
            ),
            ("Aabb".to_string(), vec!["bbAa".to_string()]),
        ];

        for (input, expected) in test_cases {
            let actual = frequency_sort(input);
            assert!(expected.contains(&actual));
        }
    }
}
