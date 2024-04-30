import { once } from "events";
import { createReadStream } from "fs";
import { Interface, createInterface } from "readline";

export const executeThree = async () => {
  const bits = await readBitsFromFile();

  const powerConsumption = calculatePowerConsumption(bits);
  console.log(`Power consumption: ${powerConsumption}`);
};

const readBitsFromFile = async (): Promise<string[]> => {
  let reader: Interface | null = null;

  try {
    const filePath = `${process.cwd()}/src/three/input.txt`;
    reader = createInterface({
      input: createReadStream(filePath, "utf-8"),
    });
  } catch (error) {
    // Avoid file errors
    console.log(error);
    return [];
  }

  if (reader == null) {
    return [];
  }

  const result: string[] = [];
  reader.on("line", (line) => {
    result.push(line);
  });

  await once(reader, "close");

  return result;
};

const calculatePowerConsumption = (bits: string[]): number => {
  if (!bits.length) {
    return 0;
  }

  let gammaRate = "";
  let epsilonRate = "";

  const counter: number[] = Array(bits[0].length).fill(0);
  for (const bit of bits) {
    for (let i = 0; i < bit.length; i++) {
      if (bit.charAt(i) === "0") {
        counter[i]--;
        continue;
      }
      counter[i]++;
    }
  }

  for (let number of counter) {
    if (number > 0) {
      gammaRate += "1";
      epsilonRate += "0";
      continue;
    }
    gammaRate += "0";
    epsilonRate += "1";
  }

  const gamma = Number.parseInt(gammaRate, 2);
  const epsilon = Number.parseInt(epsilonRate, 2);

  if (Number.isNaN(gamma) || Number.isNaN(epsilon)) {
    return 0;
  }

  return gamma * epsilon;
};
