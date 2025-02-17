function calPoints(operations: string[]): number {
    const records: number[] = [];

    for (let operation of operations) {
        if (operation === "+") {
            records.push(
                records[records.length - 1] + records[records.length - 2],
            );
            continue;
        }

        if (operation === "D") {
            records.push(records[records.length - 1] * 2);
            continue;
        }

        if (operation === "C") {
            records.pop();
            continue;
        }

        records.push(Number(operation));
    }

    if (records.length === 0) {
        return 0;
    }

    return records.reduce((sum, curr) => sum + curr, 0);
}

export const main_682 = () => {
    const operations = ["5", "-2", "4", "C", "D", "9", "+", "+"];

    const result = calPoints(operations);

    console.log(result);
};
