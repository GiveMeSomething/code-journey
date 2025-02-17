function arraySign(nums: number[]): number {
    let sign = true;
    for (let i = 0; i < nums.length; i++) {
        if (nums[i] === 0) {
            return 0;
        }

        if (nums[i] < 0) {
            sign = !sign;
        }
    }

    return sign ? 1 : -1;
}

export const main_1822 = () => {
    const nums = [-1, -2, -3, -4, 3, 2, 1];
    const result = arraySign(nums);

    console.log(result);
};
