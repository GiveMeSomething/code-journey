// To rotate a matrix 90deg
// 1. Transpose
// 2. Flip row

pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    // Tranpose n x n matrix
    for i in 0..matrix.len() {
        for j in 0..matrix.len() {
            if j < i {
                // Skip duplicate change
                continue;
            }
            let temp = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = temp;
        }
    }

    for i in 0..matrix.len() {
        let mut start = 0;
        let mut end = matrix.len() - 1;

        while start < end {
            (matrix[i][start], matrix[i][end]) = (matrix[i][end], matrix[i][start]);
            start += 1;
            end -= 1;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::solutions::problem_48::rotate;

    #[test]
    fn test_rotate() {
        let test_cases: Vec<(Vec<Vec<i32>>, Vec<Vec<i32>>)> = vec![
            (
                vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
                vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]],
            ),
            (
                vec![
                    vec![5, 1, 9, 11],
                    vec![2, 4, 8, 10],
                    vec![13, 3, 6, 7],
                    vec![15, 14, 12, 16],
                ],
                vec![
                    vec![15, 13, 2, 5],
                    vec![14, 3, 4, 1],
                    vec![12, 6, 8, 9],
                    vec![16, 7, 10, 11],
                ],
            ),
        ];

        for (mut matrix, expected) in test_cases {
            rotate(&mut matrix);

            assert_eq!(matrix.len(), expected.len());

            for i in 0..expected.len() {
                for j in 0..expected[0].len() {
                    assert_eq!(matrix[i][j], expected[i][j]);
                }
            }
        }
    }
}
