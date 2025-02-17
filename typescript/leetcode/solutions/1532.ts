function countOdds(low: number, high: number): number {
    const isEvenStart = low % 2 === 0;
    // Number of numbers in the interval
    const numCount = high - low + 1;
    // The number of odd numbers is approximately half the number count
    const oddCount = numCount / 2;
    return isEvenStart ? Math.floor(oddCount) : Math.ceil(oddCount);
}

// Way 2: Save some variables space
// function countOdds(low: number, high: number): number {
//     return low % 2 === 0
//         ? Math.floor((high - low + 1) / 2)
//         : Math.ceil((high - low + 1) / 2);
// }

export const main_1532 = () => {
    const low = 0;
    const high = 100;

    const result = countOdds(low, high);
    console.log(result);
};
