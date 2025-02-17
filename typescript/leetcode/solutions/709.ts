// Way 1: Using JS built-in function
// function toLowerCase(s: string): string {
//   return s.toLowerCase();
// }

// Way 2: ASCII tricks
function toLowerCase(s: string): string {
    let result = "";
    for (let i = 0; i < s.length; i++) {
        const currentChar = s.charCodeAt(i);
        if (currentChar >= 65 && currentChar <= 90) {
            result += String.fromCharCode(currentChar + 32);
        } else {
            result += s[i];
        }
    }

    return result;
}

export const main_709 = () => {
    const s = "LOVELY";
    const result = toLowerCase(s);

    console.log(result);
};
