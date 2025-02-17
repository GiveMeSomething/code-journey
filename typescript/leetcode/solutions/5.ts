// function longestPalindrome(s: string): string {
//     let max = s[0];

//     for (let i = 0; i < s.length; i++) {
//         let low = i - 1;
//         let high = i + 1;
//         while (low >= 0 && s[low] === s[i]) {
//             low--;
//         }

//         while (high < s.length && s[high] === s[i]) {
//             high++;
//         }

//         while (low >= 0 && high < s.length && s[low] === s[high]) {
//             low--;
//             high++;
//         }

//         const result = s.slice(low + 1, high);
//         if (result.length > max.length) {
//             max = result;
//         }
//     }

//     return max;
// }

// function longestPalindrome(s: string): string {
//     let max = "";

//     for (let i = 0; i < s.length; i++) {
//         let low = i - 1;
//         let high = i + 1;
//         while (low >= 0 && s[low] === s[i]) {
//             low--;
//         }
//         while (high < s.length && s[high] === s[i]) {
//             high++;
//         }
//         while (low >= 0 && high < s.length && s[low] === s[high]) {
//             low--;
//             high++;
//         }

//         if (high - low - 1 > max.length) {
//             max = s.slice(low + 1, high);
//         }
//     }

//     return max;
// }

function longestPalindrome(s: string): string {
    let max: number[] = [0, 0];
    let upperBound = s.length - 1;
    for (let i = 0; i < s.length; i++) {
        let low = i - 1;
        let high = i + 1;

        while (low >= 0 && s[low] === s[i]) {
            low--;
        }

        while (high <= upperBound && s[high] === s[i]) {
            high++;
        }

        while (low >= 0 && high <= upperBound && s[low] === s[high]) {
            low--;
            high++;
        }

        if (high - low > max[1] - max[0]) {
            max = [low, high];
        }
    }

    // low always end at [trueLow - 1], high always end at at [trueHigh + 1]
    // (because it need to test s[low - 1] !== s[high + 1])
    // String.slice() exclusive the end so it doesn't need max[1] - 1
    return s.slice(max[0] + 1, max[1]);
}

export const main_5 = () => {
    // const s = "babad";
    // const s = "cbbd";
    // const s = "bbbb";
    // const s = "ac";
    // const s = "bb";

    const tests = ["babad", "cbbd", "bbbb", "ac", "bb", "b"];
    for (let s of tests) {
        const result = longestPalindrome(s);
        console.log(result);
    }
};
