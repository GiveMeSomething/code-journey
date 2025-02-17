function minDistance(word1: string, word2: string): number {
    const wordMatrix: number[][] = [];
    for (let i = 0; i <= word1.length; i++) {
        wordMatrix.push([]);
    }

    for (let i = 0; i <= word1.length; i++) {
        wordMatrix[i][0] = i;
    }

    for (let j = 0; j <= word2.length; j++) {
        wordMatrix[0][j] = j;
    }

    for (let i = 1; i <= word1.length; i++) {
        for (let j = 1; j <= word2.length; j++) {
            if (word1[i - 1] === word2[j - 1]) {
                wordMatrix[i][j] = wordMatrix[i - 1][j - 1];
                continue;
            }

            wordMatrix[i][j] =
                Math.min(
                    wordMatrix[i - 1][j - 1],
                    wordMatrix[i - 1][j],
                    wordMatrix[i][j - 1],
                ) + 1;
        }
    }

    return wordMatrix[word1.length][word2.length];
}

export const main_72 = () => {
    const word1 = "horse";
    const word2 = "ros";

    const result = minDistance(word1, word2);
    console.log(result);
};
