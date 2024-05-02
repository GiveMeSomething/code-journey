import { getReader } from "../utils/file";
import { once } from "events";

export const executeFour = async () => {
  const { numbers, bingos } = await readBingoFromFile();
  console.log(numbers);
  console.log(bingos.map((bingo) => bingo.toString()));
};

class Bingo {
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
}

const readBingoFromFile = async () => {
  const reader = getReader("src/four/input.txt");
  if (reader == null) {
    throw new Error("Unable to create reader for input file");
  }

  const bingos: Bingo[] = [];
  let bingoNumbers: number[] = [];
  let bingoBuffer: string[] = [];
  let isFirstLine = true;

  reader.on("line", (line) => {
    if (isFirstLine) {
      isFirstLine = false;
      bingoNumbers = line.split(",").map((value) => Number(value));
      return;
    }

    // Empty line mean new bingo begin
    if (line === "") {
      // Avoid first empty line
      if (!bingoBuffer.length) {
        return;
      }

      bingos.push(new Bingo(bingoBuffer));
      bingoBuffer = [];
      return;
    }

    bingoBuffer.push(line);
  });

  await once(reader, "close");

  // If the buffer is not empty at the end, try to add the last bingo
  if (bingoBuffer.length) {
    bingos.push(new Bingo(bingoBuffer));
  }

  return {
    numbers: bingoNumbers,
    bingos,
  };
};
