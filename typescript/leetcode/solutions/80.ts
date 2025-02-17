function removeDuplicates(nums: number[]): number {
    let numCount = 1;
    let dupCount = 0;

    for (let i = 1; i < nums.length; i++) {
        if (nums[i - 1] !== nums[i]) {
            nums[numCount++] = nums[i];
            dupCount = 0;
            continue;
        }

        if (dupCount === 0) {
            nums[numCount++] = nums[i];
        }
        dupCount++;
    }

    return numCount;
}

export const main_80 = () => {
    const nums = [1, 1, 1, 2, 2, 3];
    const result = removeDuplicates(nums);

    console.log(nums);
    console.log(result);
};
