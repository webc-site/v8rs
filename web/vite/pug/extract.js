const PAIR_LI = [
    ["(", ")"],
    ["{", "}"],
  ],
  OUT_SIDE = 0,
  IN_SIDE = 1;

export default (input) => {
  let in_quote = 0,
    state = OUT_SIDE,
    cur_pair = [],
    depth = 0,
    res = [],
    tmp = "",
    i = 0;

  const len = input.length,
    push = () => {
      res.push(tmp);
      tmp = "";
    };

  while (i < len) {
    const char = input[i++];
    if (in_quote) {
      tmp += char;
      if (char === "\\") {
        if (i < len) {
          tmp += input[i++];
        }
      } else if (char === in_quote) {
        push();
        in_quote = 0;
      }
      continue;
    }
    if (state === OUT_SIDE) {
      for (const pair of PAIR_LI) {
        if (char === pair[0]) {
          if (tmp) push();
          state = IN_SIDE;
          cur_pair = pair;
          depth = 1;
          break;
        }
      }
      if (state === OUT_SIDE) {
        if ("'\"".includes(char)) {
          if (tmp) push();
          in_quote = char;
        }
      }
      tmp += char;
    } else {
      tmp += char;
      const [open, close] = cur_pair;
      if (char === open) {
        ++depth;
      } else if (char === close) {
        --depth;
        if (depth === 0) {
          state = OUT_SIDE;
          push();
        }
      }
    }
  }
  if (tmp) res.push(tmp);
  return res;
};
