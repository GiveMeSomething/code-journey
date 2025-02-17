// LOL
// function myPow(x: number, n: number): number {
//     if (x === 0) {
//         return 0;
//     }

//     if (n === 0) {
//         return 1;
//     }

//     return Math.pow(x, n)
// }

function myPow(x: number, n: number): number {
    if (x === 0) {
        return 0;
    }

    if (n === 0) {
        return 1;
    }

    const isPowPositive = n > 0;

    n = Math.abs(n);
    let result = 1;
    while (n != 0) {
        if (n % 2 !== 0) {
            result *= x;
        }

        x *= x;
        n >>>= 1;
    }

    return isPowPositive ? result : 1 / result;
}

export const main_50 = () => {
    // const x = 7;
    // const n = 11;

    const x = 2;
    const n = -2;

    const result = myPow(x, n);
    console.log(result);
};
