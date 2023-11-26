import { createReadStream } from "fs";
import { createInterface } from "readline/promises";
import events from "events";

export const readItemsFromFile = async (): Promise<Array<number>> => {
  const itemSum: number[] = [];

  let reader = null;

  try {
    const filePath = process.cwd() + "/day1/items.txt";
    reader = createInterface({
      input: createReadStream(filePath, { encoding: "utf-8" }),
    });
  } catch (err) {
    console.log(err);
    return [];
  }

  let sum = 0;
  reader.on("line", (currentLine) => {
    if (currentLine.trim() === "") {
      itemSum.push(sum);
      sum = 0;
      return;
    }
    sum += Number(currentLine);
  });

  await events.once(reader, "close");

  return itemSum;
};

export const getMaxSum = (items: number[]): number => {
  let max = 0;
  for (let item of items) {
    max = Math.max(max, item);
  }
  return max;
};

export const getTopThreeSum = (items: number[]): number => {
  items.sort((a, b) => b - a);
  return items[0] + items[1] + items[2];
};
