function fib(n: number): number {
    if (n === 0) {
        return 0;
    }

    if (n === 1) {
        return 1;
    }

    let temp1 = 0;
    let temp2 = 1;

    for (let i = 2; i <= n; i++) {
        const temp = temp1 + temp2;
        temp1 = Math.max(temp1, temp2);
        temp2 = temp;
    }

    return Math.max(temp1, temp2);
}

export const main_509 = () => {
    const n = [0, 1, 2, 15, 30];
    for (let value of n) {
        console.log(fib(value));
    }
};
