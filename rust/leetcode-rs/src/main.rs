use crate::solutions::problem_153::find_min;

mod solutions;

fn main() {
    let test_cases: Vec<(Vec<i32>, i32)> = vec![
        (vec![3, 4, 5, 1, 2], 1),
        (vec![4, 5, 6, 7, 0, 1, 2], 0),
        (vec![11, 13, 15, 17], 11),
    ];

    for test_case in test_cases {
        let nums = test_case.0;
        let expected = test_case.1;

        let output = find_min(nums);
        assert_eq!(output, expected);
    }
}
