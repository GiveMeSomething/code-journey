function minCostClimbingStairs(cost: number[]): number {
    for (let i = 2; i < cost.length; i++) {
        cost[i] += Math.min(cost[i - 1], cost[i - 2]);
    }

    return Math.min(cost[cost.length - 1], cost[cost.length - 2]);
}

export const main_746 = () => {
    const cost = [1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
    const result = minCostClimbingStairs(cost);

    console.log(result);
};
