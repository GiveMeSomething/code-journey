use leetcode::solutions::problem_33::search;

#[test]
fn problem_33() {
    // nums, target, expected output
    let test_cases: Vec<(Vec<i32>, i32, i32)> = vec![
        (vec![4, 5, 6, 7, 0, 1, 2], 0, 4),
        (vec![4, 5, 6, 7, 0, 1, 2], 3, -1),
        (vec![1], 1, 0),
        (vec![1], 0, -1),
        (vec![5, 1, 2, 3, 4], 4, 4),
        (vec![3, 4, 5, 6, 7, 8, 1, 2], 2, 7),
    ];

    for test_case in test_cases {
        let nums = test_case.0;
        let target = test_case.1;

        let expected = test_case.2;
        let output = search(nums, target);
        assert_eq!(output, expected);
    }
}
