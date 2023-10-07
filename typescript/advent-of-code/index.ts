import { getMaxSum, getTopThreeSum, readItemsFromFile } from "./dayOne/dayOne";
import {
  calculatePoint,
  calculatePointPartTwo,
  readInputsFromFile,
} from "./dayTwo/dayTwo";

function main() {
  execDayTwo();
}

main();

async function execDayOne() {
  const itemSums = await readItemsFromFile();

  const max = getMaxSum(itemSums);
  const topThree = getTopThreeSum(itemSums);

  console.log("Max sum", max);
  console.log("Top three sum", topThree);
}

async function execDayTwo() {
  const inputs = await readInputsFromFile();

  const rpcPoints = calculatePoint(inputs);

  const rpcPointsPartTwo = calculatePointPartTwo(inputs);

  console.log(
    "Point after playing rounds of rock, paper, scissor is:",
    rpcPoints,
  );

  console.log(
    "Point after playing rounds of rock, paper, scissor after changing strategy is:",
    rpcPointsPartTwo,
  );
}
