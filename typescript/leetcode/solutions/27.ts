function removeElement(nums: number[], val: number): number {
    let i = 0;
    let end = nums.length - 1;

    while (i <= end) {
        if (nums[end] === val) {
            end--;
            continue;
        }

        if (nums[i] === val) {
            nums[i] = nums[end];
            end--;
        }

        i++;
    }

    return i;
}

function removeElementv2(nums: number[], val: number): number {
    // Idea
    // The solution just require us to put all non-val value at the front
    let numCount = 0;
    for (let i = 0; i < nums.length; i++) {
        if (nums[i] !== val) {
            nums[numCount++] = nums[i];
        }
    }

    return numCount;
}

export const main_27 = () => {
    const nums: number[] = [4, 5];
    const val = 5;

    const numLen = removeElementv2(nums, val);

    console.log(nums);
    console.log(numLen);
};
