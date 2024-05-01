import { once } from "events";
import { createReadStream } from "fs";
import { Interface, createInterface } from "readline";

export const executeThree = async () => {
  const bits = await readBitsFromFile();

  const powerConsumption = calculatePowerConsumption(bits);
  console.log(`Power consumption: ${powerConsumption}`);

  const lifeSupportRating = calculateLifeSupportRating(bits);
  console.log(`Life support rating: ${lifeSupportRating}`);
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

const calculateLifeSupportRating = (bits: string[]): number => {
  const oxygenRating = calculateLifeRating([...bits], "oxygen");
  const co2Rating = calculateLifeRating([...bits], "co2");

  if (oxygenRating === -1 || co2Rating === -1) {
    return 0;
  }

  return oxygenRating * co2Rating;
};

const calculateLifeRating = (
  bits: string[],
  rating: "oxygen" | "co2",
): number => {
  let currentBit = 0;
  while (bits.length > 1) {
    let counter = 0;
    for (const bit of bits) {
      if (bit[currentBit] === "0") {
        counter--;
        continue;
      }
      counter++;
    }

    if (counter >= 0) {
      bits = bits.filter(
        (bit) => bit[currentBit] === (rating === "oxygen" ? "1" : "0"),
      );
    }

    if (counter < 0) {
      bits = bits.filter(
        (bit) => bit[currentBit] === (rating === "oxygen" ? "0" : "1"),
      );
    }

    currentBit++;
  }

  const value = Number.parseInt(bits[0], 2);
  if (Number.isNaN(value)) {
    return -1;
  }

  return value;
};
