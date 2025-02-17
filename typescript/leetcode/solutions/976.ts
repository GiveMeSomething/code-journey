function largestPerimeter(nums: number[]): number {
    nums.sort((a, b) => a - b);

    for (let i = nums.length - 1; i >= 2; i--) {
        if (nums[i] < nums[i - 1] + nums[i - 2]) {
            return nums[i] + nums[i - 1] + nums[i - 2];
        }
    }

    return 0;
}
