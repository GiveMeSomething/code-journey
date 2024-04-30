import { once } from "events";
import { createReadStream } from "fs";
import { Interface, createInterface } from "readline";

export const executeThree = async () => {
  const bits = await readBitsFromFile();
  console.log(bits);
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
