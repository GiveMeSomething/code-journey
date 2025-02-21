pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut result: Vec<String> = vec![];

    recur_generate(&mut result, "".to_string(), 0, 0, n);

    result
}

fn recur_generate(result: &mut Vec<String>, current: String, open: i32, close: i32, n: i32) {
    if open == close && open == n {
        result.push(current);
        return;
    }

    if current == "" {
        recur_generate(result, format!("{}{}", current, "("), open + 1, close, n);
        return;
    }

    if open < n {
        recur_generate(result, format!("{}{}", current, "("), open + 1, close, n);
    }
    if close < n && close < open {
        recur_generate(result, format!("{}{}", current, ")"), open, close + 1, n);
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use super::generate_parenthesis;

    #[test]
    fn test_generate_parenthesis() {
        let test_cases: Vec<(i32, Vec<&str>)> = vec![
            (3, vec!["((()))", "(()())", "(())()", "()(())", "()()()"]),
            (1, vec!["()"]),
        ];

        for (n, expected) in test_cases {
            println!("TEST CASE n = {}", n);
            let result = generate_parenthesis(n);

            println!("{:?}", result);

            let result_set: HashSet<String> = HashSet::from_iter(result);
            let expected_set: HashSet<&str> = HashSet::from_iter(expected);

            assert_eq!(result_set.len(), expected_set.len());
            for s in result_set {
                assert!(expected_set.contains(s.as_str()));
            }
        }
    }
}
