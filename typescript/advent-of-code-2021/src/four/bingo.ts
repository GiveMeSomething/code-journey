export default class Bingo {
  numbers: number[][];

  constructor(inputLines: string[]) {
    const numbers: number[][] = [];
    for (const line of inputLines) {
      // Extract all number in a line using regex
      // const values: number[] = [];
      // for(const match of line.matchAll(/(\d+)/g)) {
      //   const value = Number(match[0]);
      //   if(Number.isNaN(value)) {
      //     throw new Error(`Invalid input. Failed at ${line}`);
      //   }
      //   values.push(value);
      // }

      // Extract all number in a line using normal operations
      const values = line
        .split(" ")
        // Ignore empty parts (caused by spliting double space)
        .filter((part) => part !== "")
        .map((value) => Number(value.trim()))
        .filter((value) => !Number.isNaN(value));

      if (!values || values.length !== 5) {
        throw new Error(`Invalid input. Failed at ${line}`);
      }
      numbers.push(values);
    }

    this.numbers = numbers;
  }

  toString() {
    return this.numbers.toString();
  }

  winScore(bingoNumbers: number[]): { winAt: number; score: number } {
    const checked: boolean[][] = [];
    for(let i = 0; i < this.numbers.length; i++) {
      checked.push(Array(this.numbers.length).fill(false));
    }

    for (let a = 0; a < bingoNumbers.length; a++) {
      const number = bingoNumbers[a];
      for (let i = 0; i < this.numbers.length; i++) {
        for (let j = 0; j < this.numbers.length; j++) {
          if (number !== this.numbers[i][j]) {
            continue;
          }

          checked[i][j] = true;
          if (!isWinRow(checked, i) && !isWinCol(checked, j)) {
            continue;
          }

          // Calculate score
          const score = checked.reduce(
            (sum, currentRow, currentRowIndex) =>
              sum +
              currentRow.reduce(
                (rowSum, current, currentIndex) =>
                  rowSum +
                  (checked[currentRowIndex][currentIndex]
                    ? 0
                    : this.numbers[currentRowIndex][currentIndex]),
                0,
              ),
            0,
          );
          return {
            winAt: a,
            score: score * bingoNumbers[a],
          };
        }
      }
    }

    return {
      winAt: -1,
      score: 0,
    };
  }
}

function isWinRow(checked: boolean[][], rowIndex: number): boolean {
  for (let i = 0; i < checked.length; i++) {
    if (!checked[rowIndex][i]) {
      return false;
    }
  }
  return true;
}

function isWinCol(checked: boolean[][], colIndex: number): boolean {
  for (let i = 0; i < checked.length; i++) {
    if (!checked[i][colIndex]) {
      return false;
    }
  }
  return true;
}
