function plusOne(digits: number[]): number[] {
    let residual = 1;
    let i = digits.length - 1;
    while (residual > 0 && i >= 0) {
        const currentDigit = digits[i] + residual;
        if (currentDigit >= 10) {
            digits[i] = currentDigit % 10;
            residual = 1;
        } else {
            digits[i] = currentDigit;
            residual = 0;
        }
        i--;
    }

    if (residual > 0) {
        digits.unshift(residual);
    }

    return digits;
}

export const main_66 = () => {
    // const digits = [1, 2, 3];
    // const digits = [4,3,2,1]
    const digits = [9];

    const result = plusOne(digits);
    console.log(result);
};
