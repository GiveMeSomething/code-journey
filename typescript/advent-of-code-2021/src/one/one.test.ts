import { countIncreasingDepth, countIncreasingDepthWindow } from ".";

describe("part 1 tests", () => {
  it("should return 0 for empty input array", () => {
    const input: number[] = [];
    const expected = 0;
    const result = countIncreasingDepth(input);

    expect(result).toStrictEqual(expected);
  });

  it("should return 0 for one-value input array", () => {
    const input = [Number.MAX_VALUE];
    const expected = 0;
    const result = countIncreasingDepth(input);

    expect(result).toStrictEqual(expected);
  });

  it("should pass example", () => {
    const input = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    const expected = 7;
    const result = countIncreasingDepth(input);

    expect(result).toStrictEqual(expected);
  });
});

describe("part 2 tests", () => {
  it("should return 0 for empty array", () => {
    const input: number[] = [];
    const expected = 0;
    const result = countIncreasingDepthWindow(input);

    expect(result).toStrictEqual(expected);
  });

  it("should return 0 if array len <= 3", () => {
    const inputs = [[1], [1, 2], [1, 2, 3]];
    const expected = 0;
    for (const input of inputs) {
      const result = countIncreasingDepthWindow(input);
      expect(result).toStrictEqual(expected);
    }
  });

  it("should pass example", () => {
    const input = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    const expected = 5;
    const result = countIncreasingDepthWindow(input);

    expect(result).toStrictEqual(expected);
  });
});
