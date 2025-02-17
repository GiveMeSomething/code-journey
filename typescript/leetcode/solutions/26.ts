function removeDuplicates(nums: number[]): number {
    let numCount = 0;
    for (let i = 1; i < nums.length; i++) {
        if (nums[i - 1] !== nums[i]) {
            nums[numCount++] = nums[i - 1];
        }
    }
    nums[numCount++] = nums[nums.length - 1];
    return numCount;
}

export const main_26 = () => {
    const nums = [0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    const result = removeDuplicates(nums);

    console.log(nums);
    console.log(result);
};
