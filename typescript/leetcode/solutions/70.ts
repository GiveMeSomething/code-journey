function climbStairs(n: number): number {
    // There is 1 way to go to step 1 and 2 ways to go to step 2
    const routeCount = [1, 2];
    for (let i = 2; i < n; i++) {
        routeCount[i] = routeCount[i - 1] + routeCount[i - 2];
    }

    return routeCount[n - 1];
}

export const main_70 = () => {
    const n = 3;
    const result = climbStairs(n);

    console.log(result);
};
