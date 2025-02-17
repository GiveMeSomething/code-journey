// Way 1: Pretty slow...
// function isSubstring(s: string, substring: string, start: number): boolean {
//     for (let i = 0; i < substring.length; i++) {
//         if (substring[i] !== s[i + start]) {
//             return false;
//         }
//     }

//     return true;
// }

// function isRepeated(s: string, loop: number): boolean {
//     const step = Math.floor(s.length / loop);
//     const substring = s.slice(0, step);

//     for (let i = 1; i < loop; i++) {
//         if (!isSubstring(s, substring, i * step)) {
//             return false;
//         }
//     }

//     return true;
// }

// function repeatedSubstringPattern(s: string): boolean {
//     let loop = 2;
//     while (loop <= s.length) {
//         // For exmaple, you cannot repeat 3 times in a string of length 10
//         if (s.length % loop !== 0) {
//             loop++;
//             continue;
//         }

//         if (isRepeated(s, loop)) {
//             return true;
//         }

//         loop++;
//     }

//     return false;
// }

// Way 2: Compare string
// function repeatedSubstringPattern(s: string): boolean {
//     let loops = 2;
//     let tempStr = "";

//     while (loops <= s.length) {
//         if (s.length % loops !== 0) {
//             loops++;
//             continue;
//         }

//         const step = Math.floor(s.length / loops);
//         const substring = s.slice(0, step);

//         tempStr = "";
//         for (let i = 0; i < loops; i++) {
//             tempStr += substring;
//         }

//         if (s === tempStr) {
//             return true;
//         }
//         loops++;
//     }

//     return false;
// }

// Way 3: Using (weird) string theorem(s)
// A string consists of the same pattern repeated multiple times if and only if the string is a nontrivial rotation of itself.
// If x and y are strings of the same length, then x is a rotation of y if and only if x is a substring of yy.
function repeatedSubstringPattern(s: string): boolean {
    return (s + s).indexOf(s, 1) !== s.length;
}

export const main_459 = () => {
    const s = "abcabcabcabc";
    // const s = "aba";
    // const s = "bb";
    // const s = "ababab";

    console.log(s.repeat(2).slice(1, -1));

    const result = repeatedSubstringPattern(s);
    console.log(result);
};
