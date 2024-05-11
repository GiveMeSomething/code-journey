import { once } from "events";
import { createReadStream } from "fs";
import { Interface, createInterface } from "readline";

export const executeSix = async () => {
  const code = await readCodeFromFile();

  const illegalPoint = calculateIllegalPoint(code);
  console.log("Illegal point", illegalPoint);
};

const tokenMap: Record<string, number> = {
  ")": 3,
  "]": 57,
  "}": 1197,
  ">": 25137,
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

const calculateLineIllegalPoint = (line: string): number => {
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
