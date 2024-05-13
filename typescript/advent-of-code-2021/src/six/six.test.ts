import { countFish } from ".";

describe("part 1 tests", () => {
  it("should return the same as example", () => {
    const input = [3, 4, 3, 1, 2];
    const days = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 18, 80, 256];
    const expected = [
      5, 6, 7, 9, 10, 10, 10, 10, 11, 12, 26, 5934, 26984457539,
    ];

    for (let i = 0; i < days.length; i++) {
      const result = countFish(input, days[i]);
      expect(result).toStrictEqual(expected[i]);
    }
  });
});
