// function tictactoe(moves: number[][]): string {
//     const board = [
//         [0, 0, 0],
//         [0, 0, 0],
//         [0, 0, 0],
//     ];
//     let isA = true;

//     for (let move of moves) {
//         // Save current player's move
//         const player = isA ? 1 : -1;
//         const x = move[0];
//         const y = move[1];
//         board[x][y] = player;

//         // Check row
//         if (
//             board[x][0] === player &&
//             board[x][1] === player &&
//             board[x][2] === player
//         ) {
//             return isA ? "A" : "B";
//         }

//         // Check column
//         if (
//             board[0][y] === player &&
//             board[1][y] === player &&
//             board[2][y] === player
//         ) {
//             return isA ? "A" : "B";
//         }

//         // Check diagonal
//         if (x === y) {
//             if (
//                 board[0][0] === player &&
//                 board[1][1] === player &&
//                 board[2][2] === player
//             ) {
//                 return isA ? "A" : "B";
//             }
//         }

//         // Check anti-diagonal
//         if (x === 2 - y) {
//             if (
//                 board[0][2] === player &&
//                 board[1][1] === player &&
//                 board[2][0] === player
//             ) {
//                 return isA ? "A" : "B";
//             }
//         }

//         // Change turn
//         isA = !isA;
//     }

//     return moves.length >= 9 ? "Draw" : "Pending";
// }

function tictactoe(moves: number[][]): string {
    const rowCount = [
        [0, 0, 0],
        [0, 0, 0],
    ];
    const colCount = [
        [0, 0, 0],
        [0, 0, 0],
    ];
    const diagonalCount = [0, 0];
    const aDiagonalCount = [0, 0];

    let isA = true;
    for (let move of moves) {
        const [x, y] = move;
        const player = isA ? 0 : 1;

        if (
            ++rowCount[player][x] === 3 ||
            ++colCount[player][y] === 3 ||
            (x === y && ++diagonalCount[player] === 3) ||
            (x === 2 - y && ++aDiagonalCount[player] === 3)
        ) {
            return isA ? "A" : "B";
        }

        isA = !isA;
    }

    return moves.length >= 9 ? "Draw" : "Pending";
}

export const main_1275 = () => {
    const moves = [
        [0, 0],
        [2, 0],
        [1, 1],
        [2, 1],
        [2, 2],
    ];

    const result = tictactoe(moves);
    console.log(result);
};
