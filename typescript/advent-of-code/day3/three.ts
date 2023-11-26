import { createReadStream } from "fs";
import path from "path";
import { createInterface } from "readline/promises";

interface Rucksack {
  first: string;
  second: string;
}

export const readRucksackFromFile = async (): Promise<Array<Rucksack>> => {
  const filePath = path.join(process.cwd(), "day3/rucksacks.txt");
  const fileStream = createReadStream(filePath);

  const rucksacks: Rucksack[] = [];
  const reader = createInterface(fileStream);

  for await (const line of reader) {
    const pivot = line.length / 2;
    rucksacks.push({
      first: line.slice(0, pivot),
      second: line.slice(pivot),
    });
  }

  return rucksacks;
};
