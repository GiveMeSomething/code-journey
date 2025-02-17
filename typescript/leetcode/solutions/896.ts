// Way 1:
function isMonotonic(nums: number[]): boolean {
    if (nums.length === 1 || nums.length === 2) {
        return true;
    }

    let increasing: boolean | undefined = undefined;
    for (let i = 1; i < nums.length; i++) {
        const sub = nums[i] - nums[i - 1];
        if (sub === 0) {
            continue;
        }
        const isIncreasing = sub > 0 ? true : false;
        if (increasing == null) {
            increasing = isIncreasing;
            continue;
        }

        if (isIncreasing !== increasing) {
            return false;
        }
    }

    return true;
}

// Way 2:
// function isMonotonic(nums: number[]): boolean {
//     let isIncrease = false;
//     let isDecrease = false;

//     for (let i = 1; i < nums.length; i++) {
//         const subtract = nums[i] - nums[i - 1];
//         if (subtract > 0) {
//             isIncrease = true;
//         } else if (subtract < 0) {
//             isDecrease = true;
//         }
//     }

//     return isIncrease && isDecrease ? false : true;
// }

export const main_896 = () => {
    const nums = [1, 2, 2, 1];
    const result = isMonotonic(nums);

    console.log(result);
};
