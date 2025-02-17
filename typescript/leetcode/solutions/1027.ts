function longestArithSeqLength(nums: number[]): number {
    // index -> diff -> count
    const diffMap: Array<Map<number, number>> = Array(nums.length);
    let max = 1;

    for (let i = 0; i < nums.length; i++) {
        diffMap[i] = new Map<number, number>();
        for (let j = 0; j < i; j++) {
            const diff = nums[i] - nums[j];
            const diffCount = diffMap[j].get(diff);

            if (diffCount == null) {
                diffMap[i].set(diff, 2);
                max = Math.max(max, 2);
                continue;
            }

            diffMap[i].set(diff, diffCount + 1);
            max = Math.max(max, diffCount + 1);
        }
    }

    return max;
}

export const main_1027 = () => {
    // const nums = [3, 6, 9, 12];
    // const nums = [83, 20, 17, 43, 52, 78, 68, 45];
    // const nums = [
    //     12, 28, 13, 6, 34, 36, 53, 24, 29, 2, 23, 0, 22, 25, 53, 34, 23, 50, 35,
    //     43, 53, 11, 48, 56, 44, 53, 31, 6, 31, 57, 46, 6, 17, 42, 48, 28, 5, 24,
    //     0, 14, 43, 12, 21, 6, 30, 37, 16, 56, 19, 45, 51, 10, 22, 38, 39, 23, 8,
    //     29, 60, 18,
    // ];

    const nums = [
        44, 46, 22, 68, 45, 66, 43, 9, 37, 30, 50, 67, 32, 47, 44, 11, 15, 4,
        11, 6, 20, 64, 54, 54, 61, 63, 23, 43, 3, 12, 51, 61, 16, 57, 14, 12,
        55, 17, 18, 25, 19, 28, 45, 56, 29, 39, 52, 8, 1, 21, 17, 21, 23, 70,
        51, 61, 21, 52, 25, 28,
    ];

    const result = longestArithSeqLength(nums);
    console.log(result);
};
