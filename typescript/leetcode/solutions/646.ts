// Normal DP
// function findLongestChain(pairs: number[][]): number {
//     const memo: number[] = Array(pairs.length).fill(0);

//     let max = 0;
//     for (let i = 0; i < memo.length; i++) {
//         max = Math.max(longestChainRecur(pairs, i, memo), max);
//     }
//     return max;
// }

// function longestChainRecur(
//     pairs: number[][],
//     start: number,
//     memo: number[],
// ): number {
//     if (memo[start]) {
//         return memo[start];
//     }

//     let max = 0;
//     for (let i = 0; i < pairs.length; i++) {
//         if (i === start) {
//             continue;
//         }

//         if (pairs[i][0] > pairs[start][1]) {
//             max = Math.max(longestChainRecur(pairs, i, memo), max);
//         }
//     }

//     memo[start] = max + 1;
//     return memo[start];
// }

function findLongestChain(pairs: number[][]): number {
    pairs.sort((a, b) => a[1] - b[1]);

    let current = pairs[0];
    let count = 1;
    for (let i = 1; i < pairs.length; i++) {
        if (pairs[i][0] > current[1]) {
            current = pairs[i];
            count++;
        }
    }

    return count;
}

export const main_646 = () => {
    // const pairs = [
    //     [1, 2],
    //     [2, 3],
    //     [3, 4],
    // ];

    const pairs = [
        [1, 2],
        [7, 8],
        [4, 5],
    ];

    // const pairs = [
    //     [3, 4],
    //     [2, 3],
    //     [1, 2],
    // ];

    const result = findLongestChain(pairs);
    console.log(result);
};
