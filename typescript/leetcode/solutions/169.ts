// https://en.wikipedia.org/wiki/Boyer%E2%80%93Moore_majority_vote_algorithm
// Condition: Majority element must appear more than n/2 times
// with n is the length of the input array

function majorityElement(nums: number[]): number {
    if (nums.length <= 2) {
        return nums[nums.length - 1];
    }

    let m = 0;
    let counter = 0;
    for (let i = 0; i < nums.length; i++) {
        if (counter === 0) {
            m = nums[i];
            counter = 1;
            continue;
        }
        counter += m === nums[i] ? 1 : -1;
    }
    return m;
}

export const main_169 = () => {
    const nums = [2, 2, 1, 1, 1, 3, 2, 5, 2, 9, 9];
    const result = majorityElement(nums);

    console.log(result);
};
