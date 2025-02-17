// Way 1: Using Map to store char as key
// function isAnagram(s: string, t: string): boolean {
//     let charMap = new Map<string, number>();
//     for (let i = 0; i < s.length; i++) {
//         const currentChar = charMap.get(s[i]);
//         if (currentChar == null) {
//             charMap.set(s[i], 1);
//         } else {
//             charMap.set(s[i], currentChar + 1);
//         }
//     }

//     for (let i = 0; i < t.length; i++) {
//         const currentChar = charMap.get(t[i]);
//         if (currentChar == null) {
//             return false;
//         }

//         charMap.set(t[i], currentChar - 1);
//     }

//     for (let count of charMap.values()) {
//         if (count !== 0) {
//             return false;
//         }
//     }

//     return true;
// }

// Way 2: Using an array of length 26 (26 characters in the English alphabet)
// Then update the array according to current char
// Note: JS return undefined for out-of-bound index
function isAnagram(s: string, t: string): boolean {
    const charMap = Array(26).fill(0);
    const len = Math.max(s.length, t.length);

    for (let i = 0; i < len; i++) {
        charMap[s.charCodeAt(i) - 97]++;
        charMap[t.charCodeAt(i) - 97]--;
    }

    for (let counter of charMap) {
        if (counter !== 0) {
            return false;
        }
    }

    return true;
}

export const main_242 = () => {
    const s = "a";
    const t = "ab";

    const result = isAnagram(s, t);
    console.log(result);
};
