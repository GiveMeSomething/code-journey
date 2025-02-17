function tribonacci(n: number): number {
    if (n === 0) {
        return 0;
    }

    if (n === 1 || n === 2) {
        return 1;
    }

    const result = [0, 1, 1];
    for (let i = 3; i <= n; i++) {
        result[i % 3] += result[(i - 1) % 3] + result[(i - 2) % 3];
    }

    return Math.max(result[0], result[1], result[2]);
}

export const main_1137 = () => {
    const n = [0, 1, 2, 3, 15, 37];
    for (let value of n) {
        console.log(tribonacci(value));
    }
};
