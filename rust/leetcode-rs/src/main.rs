use crate::solutions::problem_33::search;

mod solutions;

fn main() {
    let test_cases: Vec<(Vec<i32>, i32, i32)> = vec![(vec![3, 4, 5, 6, 7, 8, 1, 2], 2, 7)];

    for test_case in test_cases {
        let nums = test_case.0;
        let target = test_case.1;

        let expected = test_case.2;
        let output = search(nums, target);

        println!("{}", output);

        assert_eq!(output, expected);
    }
}
