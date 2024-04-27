import { createReadStream } from "fs";
import { createInterface } from "readline/promises";
import events from "events";

export const readInputsFromFile = async (): Promise<string[][]> => {
  const opponentInput: string[] = [];
  const userInput: string[] = [];

  let reader = null;
  try {
    const filePath = process.cwd() + "/day2/items.txt";
    reader = createInterface({
      input: createReadStream(filePath, { encoding: "utf-8" }),
    });
  } catch (err: any) {
    console.log(err.message);
    return [];
  }

  reader.on("line", (line) => {
    const [opponent, user] = line.split(" ");
    opponentInput.push(opponent);
    userInput.push(user);
  });

  await events.once(reader, "close");

  return [opponentInput, userInput];
};

export const calculatePoint = (inputs: string[][]): number => {
  const [opponentInput, userInput] = inputs;

  const inputToPoint: Record<string, number> = {
    A: 1,
    B: 2,
    C: 3,
    X: 1,
    Y: 2,
    Z: 3,
  };

  let i = 0;
  let point = 0;

  while (i < opponentInput.length) {
    const opponentValue = inputToPoint[opponentInput[i]];
    const userValue = inputToPoint[userInput[i]];

    // console.log("oppo input", opponentInput[i], "point", opponentValue);
    // console.log("user input", userInput[i], "point", userValue);

    const winPosition = opponentValue === 2 ? 3 : (opponentValue + 1) % 3;
    const losePosition = opponentValue === 1 ? 3 : opponentValue - 1;
    if (userValue === winPosition) {
      // Win
      point += 6;
    } else if (userValue === losePosition) {
      // Lose
      point += 0;
    } else {
      // Draw
      point += 3;
    }

    point += userValue;
    i++;
  }

  return point;
};

export const calculatePointPartTwo = (inputs: string[][]): number => {
  const [opponentInput, userInput] = inputs;

  const inputToPoint: Record<string, number> = {
    A: 1,
    B: 2,
    C: 3,
    X: 1,
    Y: 2,
    Z: 3,
  };

  let point = 0;
  for (let i = 0; i < opponentInput.length; i++) {
    const opponentValue = inputToPoint[opponentInput[i]];

    // Need to lose
    if (userInput[i] === "X") {
      point += 0;
      point += opponentValue === 1 ? 3 : opponentValue - 1;
    }

    // Need to draw
    if (userInput[i] === "Y") {
      point += 3 + opponentValue;
    }

    // Need to win
    if (userInput[i] === "Z") {
      point += 6;
      point += opponentValue === 2 ? 3 : (opponentValue + 1) % 3;
    }
  }

  return point;
};
