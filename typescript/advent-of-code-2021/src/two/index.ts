import { once } from "events";
import { createReadStream } from "fs";
import { Interface, createInterface } from "readline";

type Direction = "forward" | "down" | "up";

class Command {
  direction: Direction;
  move: number;

  constructor(direction: Direction, move: number) {
    this.direction = direction;
    this.move = move;
  }
}

export const executeTwo = async () => {
  const commands = await extractCommandFromFile();

  const [horizontal, vertical] = simulateCommands(commands);
  console.log(`Result part 1: ${horizontal * vertical}`);
};

const extractCommandFromFile = async (): Promise<Command[]> => {
  let reader: Interface | null = null;

  try {
    const filePath = `${process.cwd()}/src/two/input.txt`;
    reader = createInterface({ input: createReadStream(filePath, "utf-8") });
  } catch (error) {
    console.log(error);
    return [];
  }

  const result: Command[] = [];
  reader.on("line", (line) => {
    if (line.trim() === "") {
      return;
    }

    const commandRegex = /(\w+) ([0-9]+)/;
    const groups = line.match(commandRegex);
    if (groups == null) {
      return;
    }

    const direction = groups[1];
    if (!isDirection(direction)) {
      return;
    }

    const moveValue = Number(groups[2]);
    if (Number.isNaN(moveValue)) {
      return;
    }

    result.push(new Command(direction, moveValue));
  });

  await once(reader, "close");

  return result;
};

const simulateCommands = (commands: Command[]): [number, number] => {
  // for (const command of commands) {
  //   switch (command.direction) {
  //     case "forward":
  //       horizontal += command.move;
  //       break;
  //     case "down":
  //       vertical += command.move;
  //       break;
  //     case "up":
  //       vertical -= command.move;
  //       break;
  //   }
  // }

  // return [horizontal, vertical];

  return commands.reduce(
    (result, currentCommand) => {
      switch (currentCommand.direction) {
        case "forward":
          result[0] += currentCommand.move;
          return result;
        case "down":
          result[1] += currentCommand.move;
          return result;
        case "up":
          result[1] -= currentCommand.move;
          return result;
      }
    },
    [0, 0],
  );
};

const isDirection = (input: string): input is Direction => {
  return ["forward", "up", "down"].includes(input);
};
