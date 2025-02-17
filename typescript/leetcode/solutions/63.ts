// function uniquePathsWithObstacles(obstacleGrid: number[][]): number {
//     if (obstacleGrid[0][0] === 1) {
//         return 0;
//     }

//     const result: number[][] = [];
//     const m = obstacleGrid.length;
//     const n = obstacleGrid[0].length;

//     // Calculate first row
//     result.push([]);
//     for (let i = 0; i < n; i++) {
//         if (obstacleGrid[0][i] === 1) {
//             result[0][i] = 0;
//             continue;
//         }

//         if (i === 0) {
//             result[0][i] = 1;
//             continue;
//         }

//         result[0][i] = result[0][i - 1];
//     }

//     // Calculate first column
//     for (let j = 1; j < m; j++) {
//         result.push([]);
//         if (obstacleGrid[j][0] === 1) {
//             result[j][0] = 0;
//             continue;
//         }

//         result[j][0] = result[j - 1][0];
//     }

//     for (let i = 1; i < m; i++) {
//         for (let j = 1; j < n; j++) {
//             if (obstacleGrid[i][j] === 1) {
//                 result[i][j] = 0;
//                 continue;
//             }

//             result[i][j] = result[i][j - 1] + result[i - 1][j];
//         }
//     }

//     return result[m - 1][n - 1];
// }

// Shortened
function uniquePathsWithObstacles(obstacleGrid: number[][]): number {
    if (obstacleGrid[0][0] === 1) {
        return 0;
    }

    const m = obstacleGrid.length;
    const n = obstacleGrid[0].length;

    for (let i = 0; i < m; i++) {
        for (let j = 0; j < n; j++) {
            if (obstacleGrid[i][j] === 1) {
                obstacleGrid[i][j] = 0;
                continue;
            }

            if (i === 0 && j === 0) {
                obstacleGrid[0][0] = 1;
                continue;
            }

            if (i === 0) {
                obstacleGrid[0][j] = obstacleGrid[0][j - 1];
                continue;
            }

            if (j === 0) {
                obstacleGrid[i][0] = obstacleGrid[i - 1][0];
                continue;
            }

            obstacleGrid[i][j] =
                obstacleGrid[i][j - 1] + obstacleGrid[i - 1][j];
        }
    }

    return obstacleGrid[m - 1][n - 1];
}

export const main_63 = () => {
    const obstacleGrid = [
        [0, 0, 0],
        [0, 1, 0],
        [0, 0, 0],
    ];

    // const obstacleGrid = [
    //     [0, 0],
    //     [1, 1],
    //     [0, 0],
    // ];

    const result = uniquePathsWithObstacles(obstacleGrid);
    console.log(result);
};
