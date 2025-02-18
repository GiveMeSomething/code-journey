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
pub fn recur_palindromize_permutation(
    inputs: &Vec<String>,
    result: &mut HashSet<String>,
    current: String,
    index: usize,
) {
    if index >= inputs.len() {
        result.insert(current);
        return;
    }

    let substrings = substring(String::from(&inputs[index]));
    for substring in substrings {
        recur_palindromize_permutation(
            inputs,
            result,
            format!("{}{}", current, substring),
            index + 1,
        );
    }
}

pub fn substring(input: String) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    for i in 0..input.len() {
        for j in i..input.len() {
            result.push(String::from(&input[i..=j]));
        }
    }
    result
}

pub fn is_palindrome(input: &str) -> bool {
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
    use super::palindromic_permutation;

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
}
