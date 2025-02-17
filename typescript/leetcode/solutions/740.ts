// Way 1: Sort the number array first
// From here we can calculate current max from the last result
// 1. If there are no previous value (previous = 0) => update current max = Math.max(max, current)
// 2. There are previous value => update current max = Math.max(max, previous + current)
//
// Notes: For adjacent duplicate number, we need to include them into previous value
// function deleteAndEarn(nums: number[]): number {
//     nums.sort((a, b) => a - b);

//     let max = 0;
//     const maxPoint = Array(nums.length);
//     maxPoint[0] = nums[0];

//     for (let i = 1; i < nums.length; i++) {
//         let previous = 0;
//         for (let j = i - 1; j >= 0; j--) {
//             if (nums[j] === nums[i] - 1 || nums[j] === nums[i] + 1) {
//                 continue;
//             }

//             if (nums[j] === nums[i]) {
//                 previous += nums[i];
//                 continue;
//             }

//             previous += maxPoint[j];
//             break;
//         }

//         if (previous > 0) {
//             max = Math.max(max, previous + nums[i]);
//         } else {
//             max = Math.max(max, nums[i]);
//         }

//         maxPoint[i] = max;
//         console.log(maxPoint);
//     }

//     return maxPoint[nums.length - 1];
// }

// Way 2: Turn the original problem into robber problem
// Turn original number array to a new array (new)
// new's index is the nums[i] and new[index] = sum(nums[i]) (sum of all occurance of nums[i])
function deleteAndEarn(nums: number[]): number {
    const n = Math.max(...nums) + 1;
    const buckets: number[] = Array(n).fill(0);

    for (let val of nums) {
        buckets[val] += val;
    }

    for (let i = 2; i < n; i++) {
        buckets[i] = Math.max(buckets[i - 1], buckets[i - 2] + buckets[i]);
    }

    return Math.max(buckets[n - 1], buckets[n - 2]);
}

export const main_740 = () => {
    // const nums = [3, 4, 2];
    // const nums = [2, 2, 3, 3, 3, 4];
    const nums = [1, 1, 1, 2, 4, 5, 5, 5, 6];
    // const nums = [8, 10, 4, 9, 1, 3, 5, 9, 4, 10];
    // const nums = [8, 3, 4, 7, 6, 6, 9, 2, 5, 8, 2, 4, 9, 5, 9, 1, 5, 7, 1, 4];

    const result = deleteAndEarn(nums);

    console.log(result);
};
