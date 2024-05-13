import { readFile } from "fs/promises";

export const executeSix = async () => {
  const fishes = await readFishFromFile();
  console.log("Fishes", fishes);
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
