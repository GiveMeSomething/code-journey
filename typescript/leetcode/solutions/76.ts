export function minWindow(s: string, t: string): string {
  if (s === t) {
    return s;
  }
  if (s.length < t.length) {
    return "";
  }

  const sMap = new Map();
  for (let i = 0; i < s.length; i++) {
    const currentCount = sMap.get(s[i]) ?? 0;
    sMap.set(s[i], currentCount + 1);
  }

  const tMap = new Map();
  for (let i = 0; i < t.length; i++) {
    const currentCount = tMap.get(t[i]) ?? 0;
    if (currentCount + 1 > (sMap.get(t[i]) ?? 0)) {
      return "";
    }
    tMap.set(t[i], currentCount + 1);
  }

  let start = 0;
  let end = s.length - 1;
  let minimum = s;

  // console.log("sMap", sMap);
  // console.log("tMap", tMap);

  while (true) {
    const currentChar = s[start];
    const sCount = sMap.get(currentChar) ?? 0;
    const tCount = tMap.get(currentChar) ?? 0;

    if (sCount > tCount) {
      start += 1;
      sMap.set(currentChar, sCount - 1);

      if (end - start + 1 < minimum.length) {
        minimum = s.slice(start, end + 1);
      }
    } else {
      // If we reduce further, the current substring will be invalid
      break;
    }
  }

  while (start >= 0) {
    const currentChar = s[end];
    const sCount = sMap.get(currentChar) ?? 0;
    const tCount = tMap.get(currentChar) ?? 0;

    if (sCount > tCount) {
      end -= 1;
      sMap.set(currentChar, sCount - 1);
      if (end - start + 1 < minimum.length) {
        minimum = s.slice(start, end + 1);
      }
      continue;
    }

    if (start == 0) {
      break;
    }

    // Decrease start until it's valid again
    while (start > 0) {
      start -= 1;

      const startChar = s[start];
      if (startChar === currentChar) {
        end -= 1;
        if (end - start + 1 < minimum.length) {
          minimum = s.slice(start, end + 1);
        }
        break;
      } else {
        const startCount = sMap.get(startChar) ?? 0;
        sMap.set(startChar, startCount + 1);
      }
    }
  }

  return minimum;
}
