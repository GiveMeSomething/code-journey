import { containsNearbyAlmostDuplicate } from "../solutions/220";

test("test containsNearbyAlmostDuplicate", () => {
  const testCases: Array<[number[], number, number, boolean]> = [
    [[1, 2, 3, 1], 3, 0, true],
    [[1, 5, 9, 1, 5, 9], 2, 3, false],
    [[1, 2, 5, 6, 7, 2, 4], 4, 0, true],
  ];

  for (const [nums, indexDiff, valueDiff, expected] of testCases) {
    let result = containsNearbyAlmostDuplicate(nums, indexDiff, valueDiff);
    expect(result).toEqual(expected);
  }
});
