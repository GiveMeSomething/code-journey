function isValidNeedle(
    haystack: string,
    needle: string,
    start: number,
): boolean {
    for (let j = 1; j < needle.length; j++) {
        if (haystack[start + j] !== needle[j]) {
            return false;
        }
    }

    return true;
}

function strStr(haystack: string, needle: string): number {
    const firstChar = needle[0];

    for (let i = 0; i < haystack.length; i++) {
        if (i + needle.length > haystack.length) {
            return -1;
        }

        if (haystack[i] === firstChar) {
            if (isValidNeedle(haystack, needle, i)) {
                return i;
            }
        }
    }

    return -1;
}

export const main_28 = () => {
    const haystack = "leetcode";
    const needle = "leeto";

    const needleIndex = strStr(haystack, needle);
    console.log(needleIndex);
};
