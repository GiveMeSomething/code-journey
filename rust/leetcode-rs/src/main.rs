use solutions::problem_35::search_insert;

mod solutions;

fn main() {
    let test_cases: Vec<(Vec<i32>, i32, i32)> = vec![
        // (vec![1, 3, 5, 6], 5, 2),
        (vec![1, 3, 5, 6], 2, 1),
        // (vec![1, 3, 5, 6], 7, 4),
    ];

    for test_case in test_cases {
        let nums = test_case.0;
        let target = test_case.1;

        let expected = test_case.2;
        let output = search_insert(nums, target);

        println!("output = {}, expected {}", output, expected);
    }
}
