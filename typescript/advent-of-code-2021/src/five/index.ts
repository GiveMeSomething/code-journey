import { once } from "events";
import { getReader } from "../utils/file";
import { Vent } from "./vent";

export const executeFive = async () => {
  const vents = await readVentFromFile();

  const intersect = countIntersect(vents);
  console.log(`Intersect: ${intersect}`);

  const intersectWithDiagonal = countIntersectWithDiagonal(vents);
  console.log(`Intersect with diagonal: ${intersectWithDiagonal}`);
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

const countIntersect = (vents: Vent[]) => {
  const pointMap = new Map<string, number>();
  for (const vent of vents) {
    if (!vent.isStraight) {
      continue;
    }

    if (vent.start[0] === vent.end[0]) {
      const start = Math.min(vent.start[1], vent.end[1]);
      const end = Math.max(vent.start[1], vent.end[1]);
      for (let i = start; i <= end; i++) {
        const key = `${vent.start[0]}-${i}`;
        pointMap.set(key, (pointMap.get(key) ?? 0) + 1);
      }
      continue;
    }

    const start = Math.min(vent.start[0], vent.end[0]);
    const end = Math.max(vent.start[0], vent.end[0]);
    for (let i = start; i <= end; i++) {
      const key = `${i}-${vent.start[1]}`;
      pointMap.set(key, (pointMap.get(key) ?? 0) + 1);
    }
  }

  let count = 0;
  for (const point of pointMap.values()) {
    if (point >= 2) {
      count++;
    }
  }
  return count;
};

const countIntersectWithDiagonal = (vents: Vent[]) => {
  const pointMap = new Map<string, number>();
  for (const vent of vents) {
    if (!vent.isStraight) {
      const xDirection = vent.end[0] - vent.start[0] > 0 ? 1 : -1;
      const yDirection = vent.end[1] - vent.start[1] > 0 ? 1 : -1;
      const diff = Math.abs(vent.end[0] - vent.start[0]);

      for (let i = 0; i <= diff; i++) {
        const key = `${vent.start[0] + i * xDirection}-${vent.start[1] + i * yDirection}`;
        pointMap.set(key, (pointMap.get(key) ?? 0) + 1);
      }
      continue;
    }

    if (vent.start[0] === vent.end[0]) {
      const start = Math.min(vent.start[1], vent.end[1]);
      const end = Math.max(vent.start[1], vent.end[1]);
      for (let i = start; i <= end; i++) {
        const key = `${vent.start[0]}-${i}`;
        pointMap.set(key, (pointMap.get(key) ?? 0) + 1);
      }
      continue;
    }

    const start = Math.min(vent.start[0], vent.end[0]);
    const end = Math.max(vent.start[0], vent.end[0]);
    for (let i = start; i <= end; i++) {
      const key = `${i}-${vent.start[1]}`;
      pointMap.set(key, (pointMap.get(key) ?? 0) + 1);
    }
  }

  let count = 0;
  for (const point of pointMap.values()) {
    if (point >= 2) {
      count++;
    }
  }
  return count;
};
