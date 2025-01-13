/**
 * @param {string} s
 * @param {number} k
 * @return {number}
 */
var characterReplacement = function (s, k) {
  let start = 0;
  let end = 0;
  let maxLength = 0;

  let characterCount = Array(26).fill(0);
  let maxCharCount = 0;
  while (end < s.length) {
    let currentCount = ++characterCount[s.charCodeAt(end) - 65];
    maxCharCount = Math.max(maxCharCount, currentCount);
    end++;

    let currentLength = end - start;
    if (currentLength > maxCharCount + k) {
      characterCount[s.charCodeAt(start) - 65]--;
      maxCharCount = Math.max(...characterCount);
      start++;
    } else {
      maxLength = Math.max(maxLength, currentLength);
    }
  }

  return maxLength;
};

export const test424 = () => {
  let testCases = [
    ["ABAB", 2, 4],
    ["AABABBA", 1, 4],
  ];

  for (let [s, k, expected] of testCases) {
    let result = characterReplacement(s, k);
    if (result != expected) {
      throw new Error(
        `Test case: ${s} k = ${k}. Expected ${expected} but received ${result}`,
      );
    }
  }
  console.log("OK");
};
