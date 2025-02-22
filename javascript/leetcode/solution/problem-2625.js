/**
 * @param {Array} arr
 * @param {number} depth
 * @return {Array}
 */
var flat = function (arr, n) {
  if (n === 0) {
    return arr;
  }

  const result = [];

  for (const item of arr) {
    if (!Array.isArray(item)) {
      result.push(item);
      continue;
    }

    result.push(...flat(item, n - 1));
  }

  return result;
};

const test = () => {
  const testCases = [
    [[1, 2, 3, [4, 5, 6], [7, 8, [9, 10, 11], 12], [13, 14, 15]], 0],
    [[1, 2, 3, [4, 5, 6], [7, 8, [9, 10, 11], 12], [13, 14, 15]], 1],
    [[1, 2, 3, [4, 5, 6], [7, 8, [9, 10, 11], 12], [13, 14, 15]], 2],
  ];

  for (const [arr, n] of testCases) {
    const result = flat(arr, n);
    console.log(result);
  }
};

test();
