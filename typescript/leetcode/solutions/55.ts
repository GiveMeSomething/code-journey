// Attemp 1: Brute force with queue ?
// function canJump(nums: number[]): boolean {
//     if (nums.length === 1) {
//         return true;
//     }

//     let indexQueue: number[] = [0];
//     while (indexQueue.length > 0) {
//         const currentIndex = indexQueue.pop();
//         if (currentIndex == null) {
//             break;
//         }

//         const currentValue = nums[currentIndex];
//         if (currentValue === -1) {
//             continue;
//         }

//         // Return condition
//         if (currentIndex + currentValue >= nums.length - 1) {
//             return true;
//         }

//         for (let i = 1; i <= nums[currentIndex]; i++) {
//             indexQueue.push(currentIndex + i);
//         }

//         // Mark as checked
//         nums[currentIndex] = -1;
//     }

//     return false;
// }

// Attemp 2: Self-made greedy algo
function canJump(nums: number[]): boolean {
    if (nums.length === 1) {
        return true;
    }

    let extraStep = 0;
    for (let i = 0; i < nums.length; i++) {
        // Return true if can jump from current index to end
        if (nums[i] + i >= nums.length - 1) {
            return true;
        }

        // Recalculate how far can we jump forward
        extraStep = Math.max(extraStep - 1, nums[i]);

        // Everything lead to a deadend
        if (extraStep === 0) {
            break;
        }
    }

    return false;
}

export const main_55 = () => {
    const nums = [2, 3, 1, 1, 4];
    const result = canJump(nums);

    console.log(result);
};
