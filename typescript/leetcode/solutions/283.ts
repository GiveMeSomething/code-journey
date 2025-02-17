function moveZeroes(nums: number[]): void {
    if (nums.length === 1) {
        return;
    }

    let zeroCount = 0;
    let currentIndex = 0;
    for (let i = 0; i < nums.length; i++) {
        if (nums[i] !== 0) {
            nums[currentIndex] = nums[i];
            currentIndex++;
            continue;
        }

        zeroCount++;
    }

    while (zeroCount > 0) {
        nums[currentIndex] = 0;
        currentIndex++;
        zeroCount--;
    }
}

export const main_283 = () => {
    const nums = [0, 1, 0, 3, 12];
    moveZeroes(nums);

    console.log(nums);
};
