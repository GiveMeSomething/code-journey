import { once } from "events";
import { createReadStream } from "fs";
import { Interface, createInterface } from "readline";

export const executeSix = async () => {
  const code = await readCodeFromFile();
  console.log(code);
};

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
