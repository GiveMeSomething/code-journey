function wordBreak(s: string, wordDict: string[]): boolean {
    const wordMap = new Map<string, boolean>();

    for (let i = 0; i < wordDict.length; i++) {
        wordMap.set(wordDict[i], true);
    }

    let wordEnd = [0];

    for (let i = 0; i < s.length; i++) {
        for (let j = wordEnd.length - 1; j >= 0; j--) {
            const currentWord = s.slice(wordEnd[j], i + 1);
            if (wordMap.has(currentWord)) {
                wordEnd.push(i + 1);
                break;
            }
        }
    }

    return wordEnd[wordEnd.length - 1] === s.length;
}

export const main_139 = () => {
    // const s = "leetcode";
    // const wordDict = ["leet", "code"];

    // const s = "penapplepen";
    // const wordDict = ["apple", "pen"];

    const s = "catsandog";
    const wordDict = ["cats", "dog", "sand", "and", "cat"];

    const result = wordBreak(s, wordDict);
    console.log(result);
};
