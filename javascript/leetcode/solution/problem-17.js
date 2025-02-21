const LETTERS = [
  [],
  [],
  ["a", "b", "c"],
  ["d", "e", "f"],
  ["g", "h", "i"],
  ["j", "k", "l"],
  ["m", "n", "o"],
  ["p", "q", "r", "s"],
  ["t", "u", "v"],
  ["w", "x", "y", "z"],
];

/**
 * @param {string} digits
 * @return {string[]}
 */
var letterCombinations = function (digits) {
  if (!digits.length) {
    return [];
  }

  const result = [];

  recurCombination(digits, result, "", digits.length);

  return result;
};

const recurCombination = (digits, result, currentCombination, n) => {
  if (currentCombination.length === n) {
    result.push(currentCombination);
    return;
  }

  const currentLetters = LETTERS[Number(digits[currentCombination.length])];
  for (const letter of currentLetters) {
    recurCombination(digits, result, currentCombination + letter, n);
  }
};

console.log(letterCombinations("23"));
