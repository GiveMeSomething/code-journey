function mergeAlternately(word1: string, word2: string): string {
    let result = "";
    let i1 = 0;
    let i2 = 0;

    while (true) {
        if (i1 >= word1.length || i2 >= word2.length) {
            break;
        }

        if (i1 === i2) {
            result += word1[i1];
            i1++;
        } else {
            result += word2[i2];
            i2++;
        }
    }

    while (i1 < word1.length) {
        result += word1[i1];
        i1++;
    }

    while (i2 < word2.length) {
        result += word2[i2];
        i2++;
    }

    return result;
}

export const main_1768 = () => {
    const word1 = "abc";
    const word2 = "pqr";

    const merged = mergeAlternately(word1, word2);

    console.log(merged);
};
