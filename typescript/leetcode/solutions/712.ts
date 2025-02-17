function minimumDeleteSum(s1: string, s2: string): number {
    const resultMap: number[][] = [];
    for (let i = 0; i <= s1.length; i++) {
        resultMap.push([]);
        if (i === 0) {
            resultMap[i][0] = 0;
            continue;
        }

        resultMap[i][0] = resultMap[i - 1][0] + s1.charCodeAt(i - 1);
    }

    for (let i = 0; i <= s2.length; i++) {
        if (i === 0) {
            resultMap[0][i] = 0;
            continue;
        }
        resultMap[0][i] = resultMap[0][i - 1] + s2.charCodeAt(i - 1);
    }

    for (let i = 1; i <= s1.length; i++) {
        for (let j = 1; j <= s2.length; j++) {
            if (s1[i - 1] === s2[j - 1]) {
                resultMap[i][j] = resultMap[i - 1][j - 1];
            } else {
                resultMap[i][j] = Math.min(
                    resultMap[i][j - 1] + s2.charCodeAt(j - 1),
                    resultMap[i - 1][j] + s1.charCodeAt(i - 1),
                );
            }
        }
    }

    return resultMap[s1.length][s2.length];
}

export const main_712 = () => {
    // const s1 = "ccaccjp";
    // const s2 = "fwosarcwge";

    const s1 = "djoqzmzxe";
    const s2 = "onydroiizrlggfh";

    const result = minimumDeleteSum(s1, s2);
    console.log(result);
};
