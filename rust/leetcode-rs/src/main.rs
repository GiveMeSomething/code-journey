use leetcode::solutions::problem_15::three_sum;

fn main() {
    let test_cases: Vec<(Vec<i32>, Vec<Vec<i32>>)> = vec![(
        vec![-1, 0, 1, 2, -1, -4],
        vec![vec![-1, -1, 2], vec![-1, 0, 1]],
    )];

    for (nums, expected) in test_cases {
        let result = three_sum(nums);

        assert_eq!(result.len(), expected.len());

        for i in 0..result.len() {
            for j in 0..result[i].len() {
                assert_eq!(result[i][j], expected[i][j]);
            }
        }
    }
}
