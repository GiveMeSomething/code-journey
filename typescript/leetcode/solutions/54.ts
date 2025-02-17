function spiralOrder(matrix: number[][]): number[] {
    const direction = [
        [0, 1],
        [1, 0],
        [0, -1],
        [-1, 0],
    ];
    const result = [];

    const row = matrix.length;
    const col = matrix[0].length;
    const length = row * col;

    let current = [0, 0];
    let currentDirection = 0;

    for (let i = 0; i < length; i++) {
        // Add current value to list
        result.push(matrix[current[0]][current[1]]);
        matrix[current[0]][current[1]] = 101;

        // Ensure that next step is available
        const next = [
            current[0] + direction[currentDirection][0],
            current[1] + direction[currentDirection][1],
        ];

        if (
            next[0] >= row ||
            next[0] < 0 ||
            next[1] >= col ||
            next[1] < 0 ||
            matrix[next[0]][next[1]] === 101
        ) {
            // Change direction
            currentDirection = (currentDirection + 1) % 4;
            current = [
                current[0] + direction[currentDirection][0],
                current[1] + direction[currentDirection][1],
            ];
        } else {
            current = next;
        }
    }

    return result;
}

export const main_54 = () => {
    const matrix = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
    ];

    const result = spiralOrder(matrix);
    console.log(result);
};
