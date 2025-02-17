function maxProfit(prices: number[]): number {
    if (prices.length === 0) {
        return 0;
    }

    let max = 0;
    let lastMax = 0;
    let currentMax = 0;
    for (let i = 1; i < prices.length; i++) {
        currentMax = lastMax + (prices[i] - prices[i - 1]);
        if (currentMax < lastMax) {
            max += lastMax;
            lastMax = 0;
        } else {
            lastMax = currentMax;
        }
    }

    return max + lastMax;
}

export const main_122 = () => {
    const prices = [6, 1, 3, 2, 4, 7];
    const profit = maxProfit(prices);

    console.log(profit);
};
