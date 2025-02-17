function groupThePeople(groupSizes: number[]): number[][] {
    const groupMap = new Map<number, number[][]>();

    for (let i = 0; i < groupSizes.length; i++) {
        const currentGroup = groupMap.get(groupSizes[i]);
        if (currentGroup == null) {
            groupMap.set(groupSizes[i], [[i]]);
            continue;
        }

        if (currentGroup[currentGroup.length - 1].length >= groupSizes[i]) {
            currentGroup.push([]);
        }
        currentGroup[currentGroup.length - 1].push(i);
    }

    const result: number[][] = [];
    for (let groups of groupMap.values()) {
        result.push(...groups);
    }

    return result;
}

export const main_1282 = () => {
    const groupSizes = [3, 3, 3, 3, 3, 1, 3];

    const result = groupThePeople(groupSizes);
    console.log(result);
};
