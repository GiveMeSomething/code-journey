import { createReadStream } from "fs";
import path from "path";
import { Interface, createInterface } from "readline/promises";

interface Rucksack {
  first: string;
  second: string;
}

export const readRucksackFromFile = (): Interface => {
  const filePath = path.join(process.cwd(), "day3/rucksacks.txt");
  const fileStream = createReadStream(filePath);

  return createInterface(fileStream);
};

export const calculateLineSum = async (reader: Interface): Promise<number> => {
  let sum = 0;

  for await (const line of reader) {
    const charMap = new Map<string, boolean>();

    // Get rucksack's parts
    const pivot = line.length / 2;
    const first = line.slice(0, pivot);
    const second = line.slice(pivot);

    // Indexing parts' characters
    for (let char of first) {
      charMap.set(char, true);
    }

    for (let char of second) {
      if (charMap.get(char) == null) {
        continue;
      }

      sum += getCharacterValue(char);
      break;
    }
  }

  return sum;
};

export const calculateGroupSum = async (reader: Interface): Promise<number> => {
  for await (const line of reader) {
  }
  return 0;
};

const getCharacterValue = (char: string): number => {
  const charCode = char.charCodeAt(0);
  if (charCode >= 65 && charCode <= 90) {
    return charCode - 38;
  } else {
    return charCode - 96;
  }
};
