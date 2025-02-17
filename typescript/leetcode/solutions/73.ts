function setZeroes(matrix: number[][]): void {
    const zeroCol = Array(matrix.length).fill(false);

    for (let i = 0; i < matrix.length; i++) {
        let isZeroRow = false;
        for (let j = 0; j < matrix[i].length; j++) {
            if (matrix[i][j] === 0) {
                isZeroRow = true;
                zeroCol[j] = true;
            }
        }

        if (isZeroRow) {
            for (let j = 0; j < matrix[i].length; j++) {
                matrix[i][j] = 0;
            }
        }
    }

    zeroCol.forEach((isZeroCol, index) => {
        if (!isZeroCol) {
            return;
        }
        for (let i = 0; i < matrix.length; i++) {
            matrix[i][index] = 0;
        }
    });
}

export const main_73 = () => {
    const matrix = [
        [0, 1, 2, 0],
        [3, 4, 5, 2],
        [1, 3, 1, 5],
    ];

    setZeroes(matrix);
    console.log(matrix);
};
