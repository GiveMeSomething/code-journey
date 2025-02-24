import { minWindow } from "../solutions/76";

test("test_min_window", () => {
  const testCases: Array<[string, string, string]> = [
    ["ADOBECODEBANC", "ABC", "BANC"],
    ["ADOBECODEBANC", "AABC", "ADOBECODEBA"],
    ["a", "a", "a"],
    ["a", "aa", ""],
    ["a", "b", ""],
  ];

  for (const [s, t, expected] of testCases) {
    const result = minWindow(s, t);
    expect(result).toEqual(expected);
  }
});
