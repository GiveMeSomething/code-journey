import { readFile } from "fs/promises";

export const executeSeven = async () => {
  const crabs = await readCrabFromFile();

  const minFuel = minCrabFuel(crabs);
  console.log("Min fuel", minFuel);
};

const readCrabFromFile = async (): Promise<number[]> => {
  const inputFile = `${process.cwd()}/src/seven/input.txt`;

  let inputLine = "";

  try {
    inputLine = await readFile(inputFile, "utf-8");
  } catch (error) {
    // Avoid file or OS errors
    console.error((error as Error).message);
    return [];
  }

  if (!inputLine) {
    return [];
  }

  return inputLine
    .trim()
    .split(",")
    .map((value) => Number(value))
    .filter((value) => !Number.isNaN(value));
};

const minCrabFuel = (crabs: number[]): number => {
  // Sort all crabs's position ASC
  const tempCrabs = [...crabs];
  tempCrabs.sort((a, b) => a - b);

  const calculateFuel = (crabs: number[], pivot: number): number => {
    let sum = 0;
    for (const crab of crabs) {
      sum += Math.abs(crab - pivot);
    }
    return sum;
  };

  if (tempCrabs.length % 2 !== 0) {
    return calculateFuel(crabs, tempCrabs[Math.floor(crabs.length / 2)]);
  }

  return Math.min(
    calculateFuel(crabs, tempCrabs[Math.floor(crabs.length / 2)]),
    calculateFuel(crabs, tempCrabs[Math.floor(crabs.length / 2) - 1])
  );
};
