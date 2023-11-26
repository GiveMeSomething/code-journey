import { readItemsFromFile, getMaxSum, getTopThreeSum } from "./day1/one";
import {
  readInputsFromFile,
  calculatePoint,
  calculatePointPartTwo,
} from "./day2/two";
import {
  calculateGroupSum,
  calculateLineSum,
  readRucksackFromFile,
} from "./day3/three";

function main() {
  // execOne();
  // execTwo();
  execThree();
}

main();

async function execOne() {
  const itemSums = await readItemsFromFile();

  const max = getMaxSum(itemSums);
  const topThree = getTopThreeSum(itemSums);

  console.log("Max sum", max);
  console.log("Top three sum", topThree);
}

async function execTwo() {
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

async function execThree() {
  const singleSum = await calculateLineSum();
  const groupSum = await calculateGroupSum();

  console.log("Priority sum of all line is", singleSum);
  console.log("Priority sum of all group is", groupSum);
}
