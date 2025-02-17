// function minDeletions(s: string): number {
//     const charMap = new Map<string, number>();

//     for (let i = 0; i < s.length; i++) {
//         charMap.set(s[i], (charMap.get(s[i]) ?? 0) + 1);
//     }

//     const frequency: boolean[] = [];
//     let deletion = 0;

//     for (let occurance of charMap.values()) {
//         if (frequency[occurance]) {
//             for (let i = occurance - 1; i >= 0; i--) {
//                 if (frequency[i]) {
//                     continue;
//                 }
//                 deletion += occurance - i;

//                 // 0 occurances is always available for all characters
//                 if (i > 0) {
//                     frequency[i] = true;
//                 }
//                 break;
//             }
//             continue;
//         }

//         frequency[occurance] = true;
//     }

//     return deletion;
// }

function minDeletions(s: string): number {
    const charMap = Array(26);
    const aCharCode = "a".charCodeAt(0);

    for (let i = 0; i < s.length; i++) {
        charMap[s.charCodeAt(i) - aCharCode] =
            (charMap[s.charCodeAt(i) - aCharCode] ?? 0) + 1;
    }

    charMap.sort((a, b) => a - b);

    let deletion = 0;
    for (let i = charMap.length - 2; i >= 0; i--) {
        if (charMap[i] == null || charMap[i + 1] == null) {
            continue;
        }

        // We have a duplication
        if (charMap[i] > 0 && charMap[i] >= charMap[i + 1]) {
            if (charMap[i + 1] === 0) {
                deletion += charMap[i];
                charMap[i] = 0;
            } else {
                deletion += charMap[i] - charMap[i + 1] + 1;
                charMap[i] = charMap[i + 1] - 1;
            }
        }
    }

    return deletion;
}

export const main_1647 = () => {
    // const s = "aab";
    // const s = "aaabbbcc";
    // const s = "bbcebabqwrty";
    const s = "abcabc";

    const result = minDeletions(s);
    console.log(result);
};
