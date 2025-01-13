/**
 * @param {string[]} timePoints
 * @return {number}
 */
var findMinDifference = function (timePoints) {
  const parsedTimePoints = [];
  for (const timePoint of timePoints) {
    let [hour, minute] = extractTime(timePoint);
    parsedTimePoints.push(hour * 60 + minute);
    parsedTimePoints.push((hour + 24) * 60 + minute);
  }

  parsedTimePoints.sort((a, b) => a - b);

  let minDiff = Number.MAX_SAFE_INTEGER;
  for (let i = 0; i < parsedTimePoints.length - 1; i++) {
    minDiff = Math.min(minDiff, parsedTimePoints[i + 1] - parsedTimePoints[i]);
  }
  return minDiff;
};

/**
 *
 * @param {string} timePoint
 */
let extractTime = (timePoint) => {
  return timePoint.split(":").map(Number);
};

export const test539 = () => {
  let testCases = [
    [["23:59", "00:00"], 1],
    [["00:00", "23:59", "00:00"], 0],
    [["01:01", "02:01"], 60],
  ];

  for (let [timePoints, expected] of testCases) {
    let result = findMinDifference(timePoints);
    if (result != expected) {
      throw new Error(
        `Test case: [${timePoints}]. Expected ${expected}, received ${result}`,
      );
    }

    console.log("OK");
  }
};
