/**
 Do not return anything, modify nums in-place instead.
 */
function reverse(nums: number[], start: number, end: number): void {
    while (start < end) {
        let temp = nums[start];
        nums[start] = nums[end];
        nums[end] = temp;

        start++;
        end--;
    }
}

// Way 2: O(n) time complexity with O(1) space complexity
function rotate(nums: number[], k: number): void {
    const reducedK = k % nums.length;
    if (reducedK === 0) {
        return;
    }

    reverse(nums, 0, nums.length - 1);
    reverse(nums, 0, reducedK - 1);
    reverse(nums, reducedK, nums.length - 1);
}

// Way 1: O(n) time complexity with O(k) space complexity
// function rotate(nums: number[], k: number): void {
//     const reducedK = nums.length - (k % nums.length);
//     if (reducedK === nums.length) {
//         return;
//     }

//     const tail = nums.slice(0, reducedK);
//     for (let i = 0; i < nums.length - reducedK; i++) {
//         nums[i] = nums[i + reducedK];
//     }

//     for (let i = nums.length - reducedK; i < nums.length; i++) {
//         nums[i] = tail[i - nums.length + reducedK];
//     }
// }

export const main_189 = () => {
    const nums = [1, 2, 3, 4, 5, 6, 7];
    const k = 3;

    const result = [5, 6, 7, 1, 2, 3, 4];

    rotate(nums, k);
    console.log(nums);
};
