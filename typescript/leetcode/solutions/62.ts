function uniquePaths(m: number, n: number): number {
    const result: number[][] = [];

    for (let i = 0; i < m; i++) {
        result.push([]);
        for (let j = 0; j < n; j++) {
            if (i === 0 || j === 0) {
                result[i][j] = 1;
                continue;
            }
            result[i][j] = result[i - 1][j] + result[i][j - 1];
        }
    }

    return result[m - 1][n - 1];
}

export const main_62 = () => {
    // const m = 3;
    // const n = 2;

    const m = 3;
    const n = 7;

    const result = uniquePaths(m, n);
    console.log(result);
};
