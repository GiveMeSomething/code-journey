use std::collections::HashSet;

pub fn palindromic_permutation(inputs: Vec<String>) -> usize {
    let mut result: HashSet<String> = HashSet::new();
    recur_palindromize_permutation(&inputs, &mut result, String::from(""), 0);

    let mut count = 0;
    for s in result {
        if is_palindrome(&s) {
            count += 1;
        }
    }

    count
}

// Generate all possible permutation
fn recur_palindromize_permutation(
    inputs: &Vec<String>,
    result: &mut HashSet<String>,
    current: String,
    index: usize,
) {
    if index >= inputs.len() {
        result.insert(current);
        return;
    }

    let substrings = substring(&inputs[index]);
    for substring in substrings {
        recur_palindromize_permutation(
            inputs,
            result,
            format!("{}{}", current, substring),
            index + 1,
        );
    }
}

fn substring(input: &str) -> Vec<String> {
    let mut result: HashSet<String> = HashSet::new();
    for i in 0..input.len() {
        for j in i..input.len() {
            result.insert(String::from(&input[i..=j]));
        }
    }

    result.into_iter().collect()
}

fn is_palindrome(input: &str) -> bool {
    if input.len() == 1 {
        return true;
    }

    let chars: Vec<char> = input.chars().collect();
    let mut start = 0;
    let mut end = input.len() - 1;

    while start < end {
        if chars[start] != chars[end] {
            return false;
        }

        start += 1;
        end -= 1;
    }

    return true;
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use super::{is_palindrome, palindromic_permutation, substring};

    #[test]
    fn test_palindromic_permutation() {
        let test_cases: Vec<(Vec<&str>, usize)> = vec![
            (vec!["cab", "c", "ba"], 3),
            (vec!["aa", "b", "aa"], 2),
            (vec!["ab", "c"], 0),
        ];

        for (inputs, expected) in test_cases {
            let result = palindromic_permutation(inputs.iter().map(|v| String::from(*v)).collect());
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_is_palindrome() {
        let test_cases: Vec<(&str, bool)> = vec![("aba", true), ("abba", true), ("abc", false)];

        for (input, expected) in test_cases {
            let result = is_palindrome(input);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_substring() {
        let test_cases: Vec<(&str, Vec<&str>)> = vec![
            ("abc", vec!["a", "b", "c", "ab", "abc", "bc"]),
            ("aaa", vec!["a", "aa", "aaa"]),
        ];

        for (input, expected) in test_cases {
            let result = substring(input);
            let expected_set: HashSet<String> =
                HashSet::from_iter(expected.iter().map(|v| String::from(*v)));

            assert_eq!(result.len(), expected_set.len());
            for substring in result {
                assert!(expected_set.contains(&substring));
            }
        }
    }
}
