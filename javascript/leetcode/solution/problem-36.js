/**
 * @param {character[][]} board
 * @return {boolean}
 */
var isValidSudoku = function (board) {
  // Check rows
  for (const row of board) {
    let checked = Array(10).fill(false);
    for (const number of row) {
      if (number === ".") {
        continue;
      }
      if (checked[Number(number)]) {
        return false;
      }
      checked[Number(number)] = true;
    }
  }

  // Check cols
  for (let i = 0; i < 9; i++) {
    let checked = Array(10).fill(false);
    for (let j = 0; j < 9; j++) {
      if (board[j][i] === ".") {
        continue;
      }
      if (checked[Number(board[j][i])]) {
        return false;
      }
      checked[Number(board[j][i])] = true;
    }
  }

  // Check cells
  for (let i = 0; i < 9; i += 3) {
    for (let j = 0; j < 9; j += 3) {
      let checked = Array(10).fill(false);
      for (let a = 0; a < 3; a++) {
        for (let b = 0; b < 3; b++) {
          const currentCell = board[a + i][b + j];
          if (currentCell === ".") {
            continue;
          }
          if (checked[Number(currentCell)]) {
            return false;
          }
          checked[Number(currentCell)] = true;
        }
      }
    }
  }

  return true;
};
