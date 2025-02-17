function canMakeArithmeticProgression(arr: number[]): boolean {
    arr.sort((a, b) => a - b);
    const step = arr[1] - arr[0];
    for (let i = 1; i < arr.length; i++) {
        if (arr[i] - arr[i - 1] !== step) {
            return false;
        }
    }

    return true;
}

export const main_1502 = () => {
    const arr = [-68, -96, -12, -40, 16];
    const result = canMakeArithmeticProgression(arr);

    console.log(result);
};
