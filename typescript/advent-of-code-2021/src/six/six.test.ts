import { calculateLineIllegalPoint } from ".";

describe("part 1 tests", () => {
  test("should detect single illegal character", () => {
    // Test edge case where there are only one character and it's the illegal one
    const inputs = [")", "]", "}", ">"];
    const expected = [3, 57, 1197, 25137];

    for (let i = 0; i < inputs.length; i++) {
      const result = calculateLineIllegalPoint(inputs[i]);
      expect(result).toStrictEqual(expected[i]);
    }
  });
});
