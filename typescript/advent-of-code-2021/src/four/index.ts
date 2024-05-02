import { getReader } from "../utils/file";
import { once } from "events";
import Bingo from "./bingo";

export const executeFour = async () => {
  const { numbers, bingos } = await readBingoFromFile();

  const firstWin = getFirstWin(bingos, numbers);
  console.log(firstWin);
};

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

const getFirstWin = (bingos: Bingo[], bingoNumbers: number[]) => {
  const result = bingos
    .map((bingo) => bingo.winScore(bingoNumbers))
    .filter((result) => result.winAt !== -1);

  result.sort((a, b) => a.winAt - b.winAt);
  return result[0];
};
