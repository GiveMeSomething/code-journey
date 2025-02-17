function minimumTotal(triangle: number[][]): number {
    for (let i = 1; i < triangle.length; i++) {
        for (let j = 0; j < i + 1; j++) {
            if (j === 0) {
                triangle[i][j] += triangle[i - 1][j];
                continue;
            }

            if (j === i) {
                triangle[i][j] += triangle[i - 1][j - 1];
                continue;
            }

            triangle[i][j] += Math.min(
                triangle[i - 1][j],
                triangle[i - 1][j - 1],
            );
        }
    }

    return Math.min(...triangle[triangle.length - 1]);
}

export const main_120 = () => {
    // const triangle = [[2], [3, 4], [6, 5, 7], [4, 1, 8, 3]];
    const triangle = [[-10]];

    const result = minimumTotal(triangle);
    console.log(result);
};
