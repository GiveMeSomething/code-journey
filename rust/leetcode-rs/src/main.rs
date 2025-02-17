use leetcode::solutions::problem_1438::{longest_subarray, longest_subarray_v3};

fn main() {
    let test_cases: Vec<(Vec<i32>, i32, i32)> = vec![
        // (vec![8, 2, 4, 7], 4, 2),
        // (vec![10, 1, 2, 4, 7, 2], 5, 4),
        // (vec![4, 2, 2, 2, 4, 4, 2, 2], 0, 3),
        (vec![2, 2, 2, 4, 4, 2, 5, 5, 5, 5, 5, 2], 2, 6),
    ];

    for (nums, limit, expected) in test_cases {
        println!("TEST CASE {:?}", nums);
        let result = longest_subarray_v3(nums, limit);

        println!("Result {}", result);
        assert_eq!(result, expected);
    }
}
