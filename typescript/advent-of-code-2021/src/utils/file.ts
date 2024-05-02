import { createReadStream } from "fs";
import { Interface, createInterface } from "readline";

export const getReader = (filePath: string): Interface | null => {
  try {
    return createInterface({
      input: createReadStream(filePath, "utf-8")
    });
  } catch(error) {
    console.error(error);
    return null;
  }
}