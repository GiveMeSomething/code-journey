function longestCommonSubsequence(text1: string, text2: string): number {
    const memo: number[][] = [];

    for (let i = 0; i <= text1.length; i++) {
        memo.push([]);
        for (let j = 0; j <= text2.length; j++) {
            if (i === 0 || j === 0) {
                memo[i][j] = 0;
                continue;
            }

            if (text1[i - 1] === text2[j - 1]) {
                memo[i][j] = memo[i - 1][j - 1] + 1;
            } else {
                memo[i][j] = Math.max(memo[i - 1][j], memo[i][j - 1]);
            }
        }
    }

    return memo[text1.length][text2.length];
}

export const main_1143 = () => {
    // const text1 = "abcde";
    // const text2 = "ace";

    const text1 = "abc";
    const text2 = "def";

    const result = longestCommonSubsequence(text1, text2);
    console.log(result);
};
