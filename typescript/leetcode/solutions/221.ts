function maximalSquare(matrix: string[][]): number {
    const m = matrix.length;
    const n = matrix[0].length;

    let max = 0;
    for (let i = 0; i < m; i++) {
        for (let j = 0; j < n; j++) {
            if (matrix[i][j] === "0") {
                continue;
            }

            if (i === 0 || j === 0) {
                max = Math.max(max, Number(matrix[i][j]));
                continue;
            }

            if (
                Number(matrix[i][j - 1]) > 0 &&
                Number(matrix[i - 1][j]) > 0 &&
                Number(matrix[i - 1][j - 1]) > 0
            ) {
                const value = Math.pow(
                    Math.sqrt(
                        Math.min(
                            Number(matrix[i][j - 1]),
                            Number(matrix[i - 1][j]),
                            Number(matrix[i - 1][j - 1]),
                        ),
                    ) + 1,
                    2,
                );

                max = Math.max(max, value);

                matrix[i][j] = value.toString();
            }
        }
    }

    console.log(matrix);

    return max;
}

export const main_221 = () => {
    // const matrix = [
    //     ["1", "0", "1", "0", "0"],
    //     ["1", "0", "1", "1", "1"],
    //     ["1", "1", "1", "1", "1"],
    //     ["1", "0", "0", "1", "0"],
    // ];

    const matrix = [
        ["1", "1", "1", "0", "0"],
        ["1", "1", "1", "1", "1"],
        ["1", "1", "1", "1", "1"],
        ["1", "0", "0", "1", "0"],
    ];

    const result = maximalSquare(matrix);
    console.log(result);
};
