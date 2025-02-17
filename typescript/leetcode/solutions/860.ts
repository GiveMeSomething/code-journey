function lemonadeChange(bills: number[]): boolean {
    // No need to store $20 changes
    const changes = [0, 0];
    for (let bill of bills) {
        if (bill === 5) {
            changes[0]++;
            continue;
        }

        if (bill === 10) {
            if (changes[0] <= 0) {
                return false;
            }
            changes[0]--;
            changes[1]++;
            continue;
        }

        if (bill === 20) {
            if (changes[1] >= 1 && changes[0] >= 1) {
                changes[0]--;
                changes[1]--;
            } else if (changes[0] >= 3) {
                changes[0] -= 3;
            } else {
                return false;
            }
        }
    }

    return true;
}

export const main_860 = () => {
    const bills = [5, 5, 5, 10, 20];

    const result = lemonadeChange(bills);
    console.log(result);
};
