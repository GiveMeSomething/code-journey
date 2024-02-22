use leetcode::solutions::problem_69::my_sqrt;

#[test]
fn problem_69() {
    // x, expected output
    let test_cases: Vec<(i32, i32)> = vec![(4, 2), (8, 2)];

    for test_case in test_cases {
        let x = test_case.0;
        let expected = test_case.1;
        let output = my_sqrt(x);
        assert_eq!(output, expected);
    }
}
