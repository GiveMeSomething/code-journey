/**
 * @param {number[]} nums
 * @return {number[]}
 */
var frequencySort = function (nums) {
  let result = [...nums];
  let countMap = new Map();
  for (const num of nums) {
    let count = countMap.get(num) ?? 0;
    countMap.set(num, count + 1);
  }

  result.sort((a, b) => {
    let aCount = countMap.get(a);
    let bCount = countMap.get(b);
    if (aCount === bCount) {
      return b - a;
    }
    return aCount - bCount;
  });

  return result;
};

export const test1636 = () => {
  let testCases = [
    [
      [1, 1, 2, 2, 2, 3],
      [3, 1, 1, 2, 2, 2],
    ],
    [
      [2, 3, 1, 3, 2],
      [1, 3, 3, 2, 2],
    ],
    [
      [-1, 1, -6, 4, 5, -6, 1, 4, 1],
      [5, -1, 4, 4, -6, -6, 1, 1, 1],
    ],
  ];

  for (const [nums, expected] of testCases) {
    let result = frequencySort(nums);

    if (result.length != expected.length) {
      throw new Error(
        `Expected length ${expected.length}, received length ${result.len}`,
      );
    }

    for (let i = 0; i < result.length; i++) {
      if (result[i] !== expected[i]) {
        throw new Error(`Expected ${expected}, received ${result}`);
      }
    }
  }

  console.log("OK");
};
