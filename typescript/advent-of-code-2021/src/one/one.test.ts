import { countIncreasingDepth } from ".";

describe("part 1 tests", () => {
  it("should return 0 for empty input array", () => {
    const input = [];
    const expected = 0;
    const result = countIncreasingDepth([]);

    expect(result).toStrictEqual(expected);
  });

  it("should return 0 for one-value input array", () => {
    const input = [Number.MAX_VALUE];
    const expected = 0;
    const result = countIncreasingDepth(input);

    expect(result).toStrictEqual(expected);
  });
});

describe("part 2 tests", () => {});
