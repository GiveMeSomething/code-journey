// Way 1: Bad performance
// function maximumWealth(accounts: number[][]): number {
//     let max = 0;
//     accounts.forEach((account) => {
//         max = Math.max(
//             max,
//             account.reduce((sum, bank) => sum + bank),
//         );
//     });

//     return max;
// }

// Way 2: Traditional for loop
// function maximumWealth(accounts: number[][]): number {
//     let max = 0;
//     for (let i = 0; i < accounts.length; i++) {
//         let currentSum = 0;
//         for (let j = 0; j < accounts[i].length; j++) {
//             currentSum += accounts[i][j];
//         }

//         max = Math.max(max, currentSum);
//     }

//     return max;
// }

// Way 3: One-liner
function maximumWealth(accounts: number[][]): number {
    return Math.max(
        ...accounts.map((account) => account.reduce((sum, bank) => sum + bank)),
    );
}

export const main_1672 = () => {
    const accounts = [
        [1, 2, 3],
        [3, 2, 1],
    ];

    const result = maximumWealth(accounts);
    console.log(result);
};
