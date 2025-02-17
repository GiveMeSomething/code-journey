// Way 1: Traditional for-loop
// function diagonalSum(mat: number[][]): number {
//     let sum = 0;
//     const length = mat.length - 1;
//     for (let i = 0; i <= length; i++) {
//         for (let j = 0; j <= length; j++) {
//             if (i === j || i + j === length) {
//                 sum += mat[i][j];
//             }
//         }
//     }

//     return sum;
// }

// Way 2: One-liner JS
function diagonalSum(mat: number[][]): number {
    return mat.reduce(
        (sum, row, i) =>
            sum +
            row.reduce(
                (rowSum, value, j) =>
                    i === j || i + j === mat.length - 1
                        ? rowSum + value
                        : rowSum,
                0,
            ),
        0,
    );
}

export const main_1572 = () => {
    const matrix = [
        [1, 1, 1, 1],
        [1, 1, 1, 1],
        [1, 1, 1, 1],
        [1, 1, 1, 1],
    ];

    const result = diagonalSum(matrix);
    console.log(result);
};
