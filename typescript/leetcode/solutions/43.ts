function multiply(num1: string, num2: string): string {
    if (num1 === "0" || num2 === "0") {
        return "0";
    }

    let result: number[] = [];

    for (let i = num1.length - 1, outerShift = 0; i >= 0; i--, outerShift++) {
        let extra = 0;
        for (
            let j = num2.length - 1, innerShift = 0;
            j >= 0;
            j--, innerShift++
        ) {
            const product = Number(num1[i]) * Number(num2[j]) + extra;
            extra = Math.floor(product / 10);
            const digit = product % 10;

            console.log(
                "product",
                product,
                "innerShift",
                innerShift,
                "outerShift",
                outerShift,
            );

            const digitSum = (result[outerShift + innerShift] ?? 0) + digit;
            if (digitSum >= 10) {
                result[outerShift + innerShift] = digitSum % 10;
                extra++;
            } else {
                result[outerShift + innerShift] = digitSum;
            }

            let extraShift = 1;
            while (extra > 0) {
                const sum =
                    (result[outerShift + innerShift + extraShift] ?? 0) + extra;
                if (sum >= 10) {
                    extra = 1;
                    result[outerShift + innerShift + extraShift] = sum % 10;
                } else {
                    extra = 0;
                    result[outerShift + innerShift + extraShift] = sum;
                }
                extraShift++;
            }

            console.log(result);
        }
    }

    let resultStr = "";
    for (let i = result.length - 1; i >= 0; i--) {
        resultStr += String(result[i]);
    }

    return resultStr;
}

export const main_43 = () => {
    // const num1 = "123";
    // const num2 = "456";

    const num1 = "99";
    const num2 = "9";

    const result = multiply(num1, num2);
    console.log(result);
};
