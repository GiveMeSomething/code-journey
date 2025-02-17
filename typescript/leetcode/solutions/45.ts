// Attempt 1: Brute force with queue
// function jump(nums: number[]): number {
//     if (nums.length === 1) {
//         return 0;
//     }

//     if (nums[0] >= nums.length - 1) {
//         return 1;
//     }

//     const jumpCount: number[] = Array(nums.length);
//     jumpCount[0] = 0;

//     for (let i = 0; i < nums.length; i++) {
//         if (nums[i] + i >= nums.length - 1) {
//             return jumpCount[i] + 1;
//         }

//         for (let j = 1; j <= nums[i]; j++) {
//             jumpCount[i + j] = Math.min(
//                 jumpCount[i] + 1,
//                 jumpCount.at(i + j) ?? Number.MAX_VALUE,
//             );
//         }
//     }

//     return jumpCount[nums.length - 1];
// }

// Attemp 2: ???
function jump(nums: number[]): number {
    if (nums.length === 1) {
        return 0;
    }

    if (nums[0] >= nums.length - 1) {
        return 1;
    }

    let jumpCount = 1;
    let max = 0;
    let step = nums[0];

    for (let i = 1; i < nums.length; i++) {
        if (nums[i] + i >= nums.length - 1) {
            return jumpCount + 1;
        }

        // Calculate next jump max value
        max = Math.max(nums[i] + i, max);

        // Increase jump count when the next max is determined
        if (--step === 0) {
            jumpCount++;
            step = max - i;
        }
    }

    return jumpCount;
}

export const main_45 = () => {
    const nums = [1, 1, 1, 1, 1];
    const jumpNum = jump(nums);

    console.log(jumpNum);
};
