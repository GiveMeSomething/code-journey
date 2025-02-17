function minPathSum(grid: number[][]): number {
    const col = grid.length;
    const row = grid[0].length;

    // Calculate first row
    for (let i = 1; i < row; i++) {
        grid[0][i] += grid[0][i - 1];
    }

    // Calculate first column
    for (let i = 1; i < col; i++) {
        grid[i][0] += grid[i - 1][0];
    }

    for (let i = 1; i < col; i++) {
        for (let j = 1; j < row; j++) {
            grid[i][j] += Math.min(grid[i][j - 1], grid[i - 1][j]);
        }
    }

    return grid[col - 1][row - 1];
}

export const main_64 = () => {
    const grid = [
        [1, 3, 1],
        [1, 5, 1],
        [4, 2, 1],
    ];

    const result = minPathSum(grid);
    console.log(result);
};
