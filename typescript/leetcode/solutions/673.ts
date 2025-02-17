function findNumberOfLIS(nums: number[]): number {
    const result: number[] = [];
    const resultCount: number[] = [];
    let max = 0;

    for (let i = 0; i < nums.length; i++) {
        result.push(1);
        resultCount.push(1);
        for (let j = 0; j < i; j++) {
            if (nums[i] > nums[j]) {
                if (result[j] + 1 > result[i]) {
                    result[i] = result[j] + 1;
                    resultCount[i] = resultCount[j];
                } else if (result[j] + 1 === result[i]) {
                    resultCount[i] += resultCount[j];
                }
            }
        }
        max = Math.max(max, result[i]);
    }

    console.log(result);
    console.log(max);
    console.log(resultCount);

    let maxCount = 0;
    for (let i = 0; i < result.length; i++) {
        if (result[i] === max) {
            maxCount += resultCount[i];
        }
    }

    return maxCount;
}

export const main_673 = () => {
    const nums = [1, 2, 4, 3, 5, 4, 7, 2];
    // const nums = [2, 2, 2, 2, 2];

    const result = findNumberOfLIS(nums);
    console.log(result);
};
