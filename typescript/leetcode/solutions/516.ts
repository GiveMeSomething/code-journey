// function longestPalindromeSubseq(s: string): number {
//     const count: number[][] = [];

//     for (let i = 0; i < s.length; i++) {
//         count.push([]);
//         for (let j = 0; j < s.length; j++) {
//             if (i === j) {
//                 count[i][j] = 1;
//                 continue;
//             }

//             count[i][j] = 0;
//         }
//     }

//     count[0][0] = 0;

//     console.log(count);

//     let max = 0;

//     for (let i = s.length - 2; i >= 0; i--) {
//         for (let j = i + 1; j < s.length; j++) {
//             if (s[i] === s[j]) {
//                 count[i][j] = count[i + 1][j - 1] + 2;
//             } else {
//                 count[i][j] = Math.max(1, count[i + 1][j], count[i][j - 1]);
//             }

//             max = Math.max(max, count[i][j]);
//         }
//     }

//     console.log(count);

//     return max;
// }

function longestPalindromeSubseq(s: string): number {
    const count: number[][] = [];
    for (let i = 0; i < s.length; i++) {
        count.push([]);
    }

    return maxSubsequence(s, 0, s.length - 1, count);
}

function maxSubsequence(
    s: string,
    start: number,
    end: number,
    count: number[][],
): number {
    if (start === end) {
        return 1;
    }

    if (start > end) {
        return 0;
    }

    let max = 1;
    if (count[start][end] != null) {
        return count[start][end];
    }

    if (s[start] === s[end]) {
        max = maxSubsequence(s, start + 1, end - 1, count) + 2;
    } else {
        max = Math.max(
            maxSubsequence(s, start + 1, end, count),
            maxSubsequence(s, start, end - 1, count),
        );
    }

    count[start][end] = max;
    return max;
}

export const main_516 = () => {
    // const s = "bbbab";
    // const s = "bbbabbb";
    const s = "aa";

    const result = longestPalindromeSubseq(s);
    console.log(result);
};
