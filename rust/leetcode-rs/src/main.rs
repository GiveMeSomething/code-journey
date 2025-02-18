use leetcode::random::palindromic_permutation::palindromic_permutation;

fn main() {
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
