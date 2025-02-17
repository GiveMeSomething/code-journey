function findTheDifference(s: string, t: string): string {
    let sum = 0;
    for (let i = 0; i < t.length; i++) {
        sum += t.charCodeAt(i);
    }

    for (let i = 0; i < s.length; i++) {
        sum -= s.charCodeAt(i);
    }

    return String.fromCharCode(sum);
}

export const main_389 = () => {
    const s = "abcd";
    const t = "abcde";

    const difference = findTheDifference(s, t);

    console.log(difference);
};
