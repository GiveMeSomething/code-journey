const romanMap: Record<string, number> = {
    I: 1,
    V: 5,
    X: 10,
    L: 50,
    C: 100,
    D: 500,
    M: 1000,
};

// Way 1: Just normal checking and accumulating result
// function romanToInt(s: string): number {
//     let result = 0;
//     for (let i = 0; i < s.length; i++) {
//         if (s[i] === "I") {
//             if (s[i + 1] === "V") {
//                 result += 4;
//                 i++;
//                 continue;
//             }

//             if (s[i + 1] === "X") {
//                 result += 9;
//                 i++;
//                 continue;
//             }
//         }

//         if (s[i] === "X") {
//             if (s[i + 1] === "L") {
//                 result += 40;
//                 i++;
//                 continue;
//             }

//             if (s[i + 1] === "C") {
//                 result += 90;
//                 i++;
//                 continue;
//             }
//         }

//         if (s[i] === "C") {
//             if (s[i + 1] === "D") {
//                 result += 400;
//                 i++;
//                 continue;
//             }

//             if (s[i + 1] === "M") {
//                 result += 900;
//                 i++;
//                 continue;
//             }
//         }

//         result += romanMap[s[i]];
//     }

//     return result;
// }

// Way 2: Still O(n) but better checking
// If current number is smaller than next number in the string, then we need to substract it
// e.g. IV => I, V => 1 < 5 => -1 + 5 = 4
function romanToInt(s: string): number {
    return s.split("").reduce<number>((sum, curr, i) => {
        let currentValue = romanMap[curr];
        let nextValue = romanMap[s[i + 1]];
        sum += currentValue < nextValue ? -romanMap[curr] : romanMap[curr];
        return sum;
    }, 0);
}

export const main_13 = () => {
    // const s = "III"
    // const s = "XLVIII";
    // const s = "MCMXCIV";
    const s = "MMMCMXCIX";
    const result = romanToInt(s);

    console.log(result);
};
