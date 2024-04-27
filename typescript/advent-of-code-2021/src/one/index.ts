import { createReadStream } from "fs";
import { Interface, createInterface } from "readline";
import events from "events";

const executeOne = async () => {
  const depths = await readDepthFromFile();

  const incresingDepth = countIncreasingDepth(depths);
  console.log("Increasing depth", incresingDepth);

  const increasingDepthWindow =countIncreasingDepthWindow(depths);
  console.log("Increasing depth window", increasingDepthWindow);
}

const readDepthFromFile = async (): Promise<number[]> => {
  let reader: Interface | null = null;

  try {
    const inputFile = `${process.cwd()}/src/one/input.txt`;
    reader = createInterface({
      input: createReadStream(inputFile, {encoding: "utf-8"})
    });
  } catch(error) {
    console.log(error);
    return [];
  }

  if(reader == null) {
    return [];
  }

  const result: number[] = [];
  reader.on("line", (line) => {
    const value = Number(line);
    if(Number.isNaN(value)) {
      return;
    }

    result.push(value);
  });

  await events.once(reader, "close");

  return result;
}

const countIncreasingDepth = (depths: number[]): number => {
  let count = 0;
  for(let i = 1; i < depths.length; i++) {
    if(depths[i] > depths[i-1]) {
      count++;
    }
  }
  return count;
}

const countIncreasingDepthWindow = (depths: number[]): number => {
  let count = 0;
  for(let i = 3; i < depths.length; i++) {
    if(depths[i-3] < depths[i]) {
      count++
    }
  }
  return count;
}

export {executeOne}