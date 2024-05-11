import {
  calculateAutocompletePoint,
  calculateLineAutocompletePoint,
  calculateLineIllegalPoint,
} from ".";

describe("part 1 tests", () => {
  it("should return the same as example", () => {
    const inputs = [
      "[[<[([]))<([[{}[[()]]]",
      "[<(<(<(<{}))><([]([]()",
      "[{[{({}]{}}([{[{{{}}([]",
      "{([(<{}[<>[]}>{[]{[(<()>",
      "<{([([[(<>()){}]>(<<{{",
    ];
    const expected = [3, 3, 57, 1197, 25137];

    for (let i = 0; i < inputs.length; i++) {
      const result = calculateLineIllegalPoint(inputs[i]);
      expect(result).toStrictEqual(expected[i]);
    }
  });

  it("should detect single illegal character", () => {
    // Test edge case where there are only one character and it's the illegal one
    const inputs = [")", "]", "}", ">"];
    const expected = [3, 57, 1197, 25137];

    for (let i = 0; i < inputs.length; i++) {
      const result = calculateLineIllegalPoint(inputs[i]);
      expect(result).toStrictEqual(expected[i]);
    }
  });
});

describe("part 2 its", () => {
  it("should return the same as example", () => {
    const inputs = [
      "[({(<(())[]>[[{[]{<()<>>",
      "[(()[<>])]({[<{<<[]>>(",
      "(((({<>}<{<{<>}{[]{[]{}",
      "{<[[]]>}<{[{[{[]{()[[[]",
      "<{([{{}}[<[[[<>{}]]]>[]]",
    ];
    const expected = [288957, 5566, 1480781, 995444, 294];
    for (let i = 0; i < inputs.length; i++) {
      const result = calculateLineAutocompletePoint(inputs[i]);
      expect(result).toStrictEqual(expected[i]);
    }
  });

  it("should return the same as example", () => {
    const input = [
      "[({(<(())[]>[[{[]{<()<>>",
      "[(()[<>])]({[<{<<[]>>(",
      "(((({<>}<{<{<>}{[]{[]{}",
      "{<[[]]>}<{[{[{[]{()[[[]",
      "<{([{{}}[<[[[<>{}]]]>[]]",
    ];
    const expected = 288957;

    const result = calculateAutocompletePoint(input);
    expect(result).toStrictEqual(expected);
  });
});
