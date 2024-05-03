import { once } from "events";
import { getReader } from "../utils/file";
import { Vent } from "./vent";

export const executeFive = async () => {
  const result = await readVentFromFile();
  console.log(result);
};

const readVentFromFile = async () => {
  const reader = getReader("src/five/input.txt");
  if (reader == null) {
    throw new Error("Unable to create reader for input file");
  }

  const result: Vent[] = [];
  reader.on("line", (line) => {
    result.push(new Vent(line));
  });

  await once(reader, "close");
  return result;
};
