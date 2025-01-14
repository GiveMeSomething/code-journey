pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut prefix = String::from("");

    let min_len = strs.iter().map(|v| v.len()).min().unwrap();
    for i in 0..min_len {
        let base = String::from(&strs[0][i..i + 1]);
        for str in &strs {
            if &str[i..i + 1] != base {
                return prefix;
            }
        }

        prefix = format!("{}{}", prefix, base);
    }
    prefix
}

#[cfg(test)]
mod test {
    use super::longest_common_prefix;

    #[test]
    fn test_longest_common_prefix() {
        let test_cases: Vec<(Vec<&str>, &str)> = vec![
            (vec!["flower", "flow", "flight"], "fl"),
            (vec!["dog", "racecar", "car"], ""),
        ];

        for (strs, expected) in test_cases {
            let result = longest_common_prefix(strs.iter().map(|v| String::from(*v)).collect());
            assert!(result == String::from(expected));
        }
    }
}
