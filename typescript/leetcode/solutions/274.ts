function hIndex(citations: number[]): number {
    return citations
        .sort((a, b) => b - a)
        .filter((citeCount, index) => citeCount > index).length;
}

export const main_274 = () => {
    const citations = [1, 1, 3, 6, 7, 10, 7, 1, 8, 5, 9, 1, 4, 4, 3];

    const h = hIndex(citations);

    console.log(h);
};
