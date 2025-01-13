/**
 * @param {number[]} nums
 * @return {number}
 */
// Suboptimal way to get majority element
// var majorityElement = function (nums) {
//   let countMap = new Map();

//   for (const value of nums) {
//     let count = countMap.get(value) ?? 0;
//     countMap.set(value, count + 1);
//   }

//   for (const [value, count] of countMap.entries()) {
//     if (count > nums.length / 2) {
//       return value;
//     }
//   }
//   return 0;
// };

/**
 * @param {number[]} nums
 * @return {number}
 */
// Optimal way, using Boyee-Moore Voting Count algorithm
var majorityElement = function (nums) {
  let major = nums[0];
  let point = 0;

  for (const num of nums) {
    if (num === major) {
      point++;
      continue;
    }

    point--;
    if (point < 0) {
      major = num;
      point = 0;
    }
  }

  return major;
};

export const test169 = () => {
  let testCases = [
    [[3, 2, 3], 3],
    [[2, 2, 1, 1, 1, 2, 2], 2],
  ];

  for (let [nums, expected] of testCases) {
    let result = majorityElement(nums);
    if (result !== expected) {
      throw new Error(`Expected ${expected}, received ${result}`);
    }
  }

  console.log("OK");
};
