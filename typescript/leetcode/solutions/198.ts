// When at a house, we can choose to keep or steal
// We will keep if steal value is < keep value
// And loop that until the end of the list
// Max between the last two value will be the result
function rob(nums: number[]): number {
    if (nums.length === 1) {
        return nums[0];
    }

    nums[1] = Math.max(nums[0], nums[1]);

    for (let i = 2; i < nums.length; i++) {
        nums[i] = Math.max(nums[i - 1], nums[i - 2] + nums[i]);
    }

    return Math.max(nums[nums.length - 1], nums[nums.length - 2]);
}

export const main_198 = () => {};
