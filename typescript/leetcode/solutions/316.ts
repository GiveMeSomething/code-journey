function removeDuplicateLetters(s: string): string {
    const charMap: number[] = Array(26);
    for (let i = 0; i < s.length; i++) {
        const charCode = s.charCodeAt(i) - 97;
        charMap[charCode] = i;
    }

    const result: number[] = [];
    const seen: number[] = Array(26).fill(0);

    for (let i = 0; i < s.length; i++) {
        const currentCharCode = s.charCodeAt(i) - 97;

        // If the current character have appear
        if (seen[currentCharCode]++ > 0) {
            continue;
        }

        while (
            result.length > 0 &&
            result[result.length - 1] > currentCharCode &&
            i < charMap[result[result.length - 1]]
        ) {
            const lastCharCode = result.pop();
            if (lastCharCode != null) {
                seen[lastCharCode] = 0;
            }
        }

        result.push(currentCharCode);
    }

    return result
        .map((charCode) => String.fromCharCode(charCode + 97))
        .join("");
}
