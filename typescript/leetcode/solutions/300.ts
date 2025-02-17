// function lengthOfLIS(nums: number[]): number {
//     const countTable: number[] = [];
//     let max = 0;
//     for (let i = 0; i < nums.length; i++) {
//         countTable.push(1);
//         for (let j = i; j >= 0; j--) {
//             if (nums[i] > nums[j]) {
//                 countTable[i] = Math.max(countTable[j] + 1, countTable[i]);
//             }
//         }

//         max = Math.max(max, countTable[i]);
//     }

//     return max;
// }

function lengthOfLIS(nums: number[]): number {
    const sub: number[] = [nums[0]];

    for (let i = 1; i < nums.length; i++) {
        if (nums[i] > sub[sub.length - 1]) {
            sub.push(nums[i]);
        } else {
            sub[sub.findIndex((val) => val >= nums[i])] = nums[i];
        }
    }

    return sub.length;
}

export const main_300 = () => {
    // const nums = [10, 9, 2, 5, 3, 7, 101, 18];
    // const nums = [1, 3, 6, 7, 9, 4, 10, 5, 6];
    const nums = [4, 10, 4, 3, 8, 9];

    const result = lengthOfLIS(nums);
    console.log(result);
};
