type Point = [number, number];

export class Vent {
  start: Point;
  end: Point;
  isStraight: boolean;

  constructor(line: string) {
    const lineRegex = /(\d+),(\d+) -> (\d+),(\d+)/;
    const match = line.match(lineRegex);
    if (match == null) {
      throw new Error("Cannot create new Vent: Invalid input");
    }

    const values = match
      .map((value) => Number(value))
      .filter((value) => !Number.isNaN(value));
    if (values.length != 4) {
      throw new Error("Cannot create new Vent: Invalid input");
    }

    this.start = [values[0], values[1]];
    this.end = [values[2], values[3]];

    this.isStraight =
      this.start[0] === this.end[0] || this.start[1] === this.end[1];
  }
}
