function maxUncrossedLines(nums1: number[], nums2: number[]): number {
    let memo: number[][] = [];

    for (let i = 0; i <= nums1.length; i++) {
        memo.push([]);
        for (let j = 0; j <= nums2.length; j++) {
            memo[i][j] = 0;
        }
    }

    let max = 0;
    for (let i = 1; i <= nums1.length; i++) {
        for (let j = 1; j <= nums2.length; j++) {
            if (nums1[i - 1] === nums2[j - 1]) {
                memo[i][j] = memo[i - 1][j - 1] + 1;
            } else {
                memo[i][j] = Math.max(memo[i - 1][j], memo[i][j - 1]);
            }

            max = Math.max(max, memo[i][j]);
        }
    }

    return max;
}

export const main_1035 = () => {
    const nums1 = [2, 5, 1, 2, 5];
    const nums2 = [10, 5, 2, 1, 5, 2];

    const result = maxUncrossedLines(nums1, nums2);
    console.log(result);
};
