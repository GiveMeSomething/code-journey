function isRobotBounded(instructions: string): boolean {
    let x = 0;
    let y = 0;
    let facing = 0;
    let move = [
        [0, 1],
        [1, 0],
        [0, -1],
        [-1, 0],
    ];

    for (let i = 0; i < instructions.length; i++) {
        const instruction = instructions[i];
        switch (instruction) {
            case "G":
                x += move[facing][0];
                y += move[facing][1];
                break;

            case "L":
                facing = (facing + 3) % 4;
                break;

            case "R":
                facing = (facing + 1) % 4;
                break;
        }
    }

    return (x === 0 && y === 0) || facing > 0;
}

export const main_1041 = () => {
    const instructions = "GGLLGG";
    const result = isRobotBounded(instructions);

    console.log(result);
};
