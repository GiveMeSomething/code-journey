pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];
    for i in 0..matrix[0].len() {
        result.push(vec![0; matrix.len()]);
        for j in 0..matrix.len() {
            result[i][j] = matrix[j][i];
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::transpose;

    #[test]
    fn test_transpose() {
        let test_cases: Vec<(Vec<Vec<i32>>, Vec<Vec<i32>>)> = vec![
            (
                vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
                vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]],
            ),
            (
                vec![vec![1, 2, 3], vec![4, 5, 6]],
                vec![vec![1, 4], vec![2, 5], vec![3, 6]],
            ),
        ];

        for (matrix, expected) in test_cases {
            let result = transpose(matrix);

            assert_eq!(result.len(), expected.len());
            assert_eq!(result[0].len(), expected[0].len());

            for i in 0..expected.len() {
                for j in 0..expected[0].len() {
                    assert_eq!(result[i][j], expected[i][j]);
                }
            }
        }
    }
}
