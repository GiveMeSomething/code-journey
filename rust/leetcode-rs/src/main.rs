use crate::solutions::problem_69::my_sqrt;

mod solutions;

fn main() {
    let test_cases: Vec<(i32, i32)> =
        vec![(4, 2), (8, 2), (2147395600, 46340), (1, 1), (2, 1), (6, 2)];

    for test_case in test_cases {
        let x = test_case.0;
        let expected = test_case.1;
        let output = my_sqrt(x);
        println!("output {}", output);
        assert_eq!(output, expected);
    }
}
