function lengthOfLastWord(s: string): number {
    let count = 0;
    for (let i = s.length - 1; i >= 0; i--) {
        if (s[i] !== " ") {
            count++;
        } else if (count !== 0) {
            return count;
        }
    }

    return count;
}

export const main_58 = () => {
    const s = "   fly me   to   the moon  ";
    const result = lengthOfLastWord(s);
    console.log(result);
};
