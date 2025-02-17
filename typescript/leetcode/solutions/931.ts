function minFallingPathSum(matrix: number[][]): number {
    // n*n matrix
    const n = matrix.length;

    if (n === 1) {
        return matrix[0][0];
    }

    for (let i = 1; i < n; i++) {
        for (let j = 0; j < n; j++) {
            if (j === 0) {
                matrix[i][j] += Math.min(
                    matrix[i - 1][j],
                    matrix[i - 1][j + 1],
                );
                continue;
            }

            if (j === n - 1) {
                matrix[i][j] += Math.min(
                    matrix[i - 1][j],
                    matrix[i - 1][j - 1],
                );
                continue;
            }

            matrix[i][j] += Math.min(
                matrix[i - 1][j - 1],
                matrix[i - 1][j],
                matrix[i - 1][j + 1],
            );
        }
    }

    return Math.min(...matrix[n - 1]);
}

export const main_931 = () => {
    const matrix = [
        [2, 1, 3],
        [6, 5, 4],
        [7, 8, 9],
    ];

    const result = minFallingPathSum(matrix);
    console.log(result);
};
