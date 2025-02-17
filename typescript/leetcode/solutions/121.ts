function maxProfit(prices: number[]): number {
    let max = 0;
    let lastMax = 0;

    for (let i = 1; i < prices.length; i++) {
        let currentMax = lastMax + (prices[i] - prices[i - 1]);
        if (currentMax > max) {
            max = currentMax;
        }

        lastMax = currentMax < 0 ? 0 : currentMax;
    }

    return max;
}

export const main_121 = () => {
    const prices = [7, 1, 5, 3, 6, 4];

    const profit = maxProfit(prices);
    console.log(profit);
};
