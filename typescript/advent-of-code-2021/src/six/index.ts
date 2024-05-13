import { readFile } from "fs/promises";

export const executeSix = async () => {
  const fishes = await readFishFromFile();

  const fishCount1 = countFish(fishes, 18);
  console.log("After 80 days", fishCount1);

  const fishCount2 = countFish(fishes, 256);
  console.log("After 256 days", fishCount2);
};

const readFishFromFile = async (): Promise<number[]> => {
  const inputPath = `${process.cwd()}/src/six/input.txt`;
  let data = "";
  try {
    data = await readFile(inputPath, "utf-8");
  } catch (error) {
    console.log(error);
    return [];
  }

  if (!data) {
    throw new Error("Cannot proceed with empty data");
  }

  return data
    .trim()
    .split(",")
    .map((part) => Number(part))
    .filter((part) => !Number.isNaN(part));
};

export const countFish = (fishes: number[], remaining: number): number => {
  const resultMap = new Map<string, number>();

  const countSingleFish = (start: number, remaining: number): number => {
    if (start < 0 || remaining < 0) {
      return 0;
    }

    if (remaining <= start) {
      return 1;
    }

    const key = `${start}-${remaining}`;
    if (resultMap.has(key)) {
      return resultMap.get(key) as number;
    }

    let count = 1;
    while (remaining > 0) {
      remaining = remaining - start - 1;
      start = 6;

      count += countSingleFish(8, remaining);
    }

    resultMap.set(key, count);
    return count;
  };

  let count = 0;
  for (const fish of fishes) {
    count += countSingleFish(fish, remaining);
  }
  return count;
};
