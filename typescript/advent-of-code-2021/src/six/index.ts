import { once } from "events";
import { createReadStream } from "fs";
import { Interface, createInterface } from "readline";

export const executeSix = async () => {
  const code = await readCodeFromFile();

  const illegalPoint = calculateIllegalPoint(code);
  console.log("Illegal point", illegalPoint);

  const autocompletePoint = calculateAutocompletePoint(code);
  console.log("Autocomplete point", autocompletePoint);
};

const tokenMap: Record<string, number> = {
  ")": 3,
  "]": 57,
  "}": 1197,
  ">": 25137,
};
const tokenMap2: Record<string, number> = {
  "(": 1,
  "[": 2,
  "{": 3,
  "<": 4,
};

const openTokenList = ["(", "[", "{", "<"];

const readCodeFromFile = async (): Promise<string[]> => {
  const inputPath = `${process.cwd()}/src/six/input.txt`;
  let reader: Interface | null = null;

  try {
    reader = createInterface({
      input: createReadStream(inputPath, "utf-8"),
    });
  } catch (error) {
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

const calculateIllegalPoint = (lines: string[]) => {
  let points = 0;
  for (const line of lines) {
    points += calculateLineIllegalPoint(line);
  }
  return points;
};

export const calculateLineIllegalPoint = (line: string): number => {
  const queue: string[] = [];
  for (const token of line.split("")) {
    if (openTokenList.includes(token)) {
      queue.push(token);
      continue;
    }

    const lastToken = queue[queue.length - 1];
    if (
      (lastToken !== "(" && token === ")") ||
      (lastToken !== "[" && token === "]") ||
      (lastToken !== "{" && token === "}") ||
      (lastToken !== "<" && token === ">")
    ) {
      return tokenMap[token];
    }

    const data = queue.pop();
    if (!data) {
      return tokenMap[token];
    }
  }
  return 0;
};

export const calculateAutocompletePoint = (lines: string[]): number => {
  // Filter out all invalid lines
  const validLines = lines.filter(
    (line) => calculateLineIllegalPoint(line) === 0
  );

  let points = [];
  for (const line of validLines) {
    points.push(calculateLineAutocompletePoint(line));
  }

  // Find median
  points.sort((a, b) => a - b);
  const half = Math.floor(points.length / 2);
  return points[half];
};

export const calculateLineAutocompletePoint = (line: string): number => {
  const queue: string[] = [];
  for (const token of line.split("")) {
    if (openTokenList.includes(token)) {
      queue.push(token);
      continue;
    }

    // This is sure to be correct for valid line
    queue.pop();
  }

  let point = 0;
  while (queue.length > 0) {
    const token = queue.pop();
    if (!token) {
      continue;
    }
    point = point * 5 + tokenMap2[token];
  }
  return point;
};
