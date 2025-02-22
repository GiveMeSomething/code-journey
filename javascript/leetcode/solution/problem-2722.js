/**
 * @param {Array} arr1
 * @param {Array} arr2
 * @return {Array}
 */
var join = function (arr1, arr2) {
  const idMap = new Map();

  for (const item of arr1) {
    idMap.set(item.id, item);
  }

  for (const item of arr2) {
    const found = idMap.get(item.id) ?? {};
    idMap.set(item.id, { ...found, ...item });
  }

  const result = [];
  for (const item of idMap.values()) {
    result.push(item);
  }

  result.sort((a, b) => a.id - b.id);

  return result;
};
