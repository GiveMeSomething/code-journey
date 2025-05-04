export function containsNearbyAlmostDuplicate(
  nums: number[],
  indexDiff: number,
  valueDiff: number,
): boolean {
  const sortedNums = nums.map((num, i) => [num, i]);
  sortedNums.sort((a, b) => a[0] - b[0]);

  let limitStart = 0;
  let limitEnd = 0;
  while (limitEnd < nums.length) {
    limitEnd += 1;
  }

  // Check for index diff now
  const minQueue: number[] = [];
  const maxQueue: number[] = [];

  let start = 0;
  let end = 0;
  while (end <= limit) {
    while (
      minQueue.length > 0 &&
      sortedNums[end][1] < minQueue[minQueue.length - 1]
    ) {
      minQueue.pop();
    }
    minQueue.push(sortedNums[end][1]);

    while (
      maxQueue.length > 0 &&
      sortedNums[end][1] > maxQueue[maxQueue.length - 1]
    ) {
      maxQueue.pop();
    }
    maxQueue.push(sortedNums[end][1]);

    const currentMin = minQueue[0];
    const currentMax = maxQueue[0];

    if (Math.abs(currentMax - currentMin) <= indexDiff) {
      // Must be a different index to form a pair
      if (currentMax !== currentMin) {
        return true;
      }

      end += 1;
      continue;
    }

    while (Math.abs(maxQueue[0] - minQueue[0]) > indexDiff) {
      if (minQueue.length > 0 && minQueue[0] === sortedNums[start][1]) {
        // pop-front
        minQueue.splice(0, 1);
      }
      if (maxQueue.length > 0 && maxQueue[0] === sortedNums[start][1]) {
        // pop-front
        maxQueue.splice(0, 1);
      }

      start += 1;
    }

    end += 1;
  }

  return false;
}
