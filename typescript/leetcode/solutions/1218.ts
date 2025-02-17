// Dynamic programming
// function longestSubsequence(arr: number[], difference: number): number {
//     const memo: number[] = Array(arr.length);
//     for (let i = 0; i < arr.length; i++) {
//         memo[i] = 1;
//         for (let j = i - 1; j >= 0; j--) {
//             if (arr[i] - arr[j] === difference) {
//                 memo[i] = memo[j] + 1;
//                 break;
//             }
//         }
//     }

//     console.log(memo);

//     return Math.max(...memo);
// }

function longestSubsequence(arr: number[], difference: number): number {
    const valueMap = new Map<number, number>();

    for (let i = 0; i < arr.length; i++) {
        const prevValue = arr[i] - difference;
        const prevCount = valueMap.get(prevValue);
        if (prevCount == null) {
            valueMap.set(arr[i], 1);
            continue;
        }

        valueMap.set(arr[i], prevCount + 1);
    }

    let max = 0;
    for (let count of valueMap.values()) {
        max = Math.max(max, count);
    }

    return max;
}

export const main_1218 = () => {
    // const arr = [1, 2, 3, 4];
    // const arr = [1, 3, 5, 7];
    // const arr = [1, 5, 7, 8, 5, 3, 4, 2, 1];
    const arr = [4, 12, 10, 0, -2, 7, -8, 9, -9, -12, -12, 8, 8];
    const difference = 0;

    const result = longestSubsequence(arr, difference);
    console.log(result);
};
