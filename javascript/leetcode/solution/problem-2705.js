/**
 * @param {Object|Array} obj
 * @return {Object|Array}
 */
var compactObject = function (obj) {
  if (obj == null) {
    return null;
  }

  if (Array.isArray(obj)) {
    obj = obj.filter(Boolean).map(compactObject);
    return obj;
  }

  if (typeof obj !== "object") {
    return obj;
  }

  const compacted = {};
  for (const key in obj) {
    if (!obj[key]) {
      continue;
    }
    compacted[key] = compactObject(obj[key]);
  }

  return compacted;
};
