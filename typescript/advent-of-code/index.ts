import { getMaxSum, getTopThreeSum, readItemsFromFile } from "./dayOne/dayOne";

function main() {
  execDayOne();
}

main();

async function execDayOne() {
  const itemSums = await readItemsFromFile();

  const max = getMaxSum(itemSums);
  const topThree = getTopThreeSum(itemSums);

  console.log("Max sum", max);
  console.log("Top three sum", topThree);
}
