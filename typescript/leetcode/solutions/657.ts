function judgeCircle(moves: string): boolean {
    if (moves.length % 2 !== 0) {
        return false;
    }

    let x = 0;
    let y = 0;
    for (let move of moves) {
        switch (move) {
            case "R":
                x++;
                break;
            case "L":
                x--;
                break;
            case "U":
                y++;
                break;
            case "D":
                y--;
                break;
        }
    }
    return x === 0 && y === 0;
}

export const main_657 = () => {
    const moves = "RRLLUUDD";
    const result = judgeCircle(moves);

    console.log(result);
};
