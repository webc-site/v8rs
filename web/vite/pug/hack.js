import pugExtract from "./extract.js";

const CMD = new Set(
    "if else elif elseif key each await then catch html const debug snippet render".split(" "),
  ),
  split = (txt) => {
    let state = 0,
      depth = 0,
      t = [];
    const li = [];
    for (const i of txt) {
      if (state === 0) {
        if (i === '"') {
          state = 1;
          t.push(i);
          continue;
        }
        if (i === "{") {
          state = 2;
          depth = 1;
          t.push(i);
          continue;
        }
        if (i === " ") {
          li.push(t.join(""));
          t.length = 0;
        } else {
          t.push(i);
        }
      } else if (state === 1) {
        if (i === '"' && t[t.length - 1] !== "\\") {
          state = 0;
        }
        t.push(i);
      } else if (state === 2) {
        if (i === "{") depth++;
        else if (i === "}") depth--;
        t.push(i);
        if (depth === 0) state = 0;
      }
    }
    if (t.length) {
      li.push(t.join(""));
    }
    return li;
  },
  bind = (pug) =>
    pugExtract(pug).map((txt) => {
      const last = txt.length - 1;
      if (txt[0] !== "(" || txt[last] !== ")") {
        return txt;
      }
      const inner = txt.slice(1, last),
        splitted = split(inner)
          .map((line) => {
            if (!line.trim()) {
              return line;
            }
            let attr = line.trimStart();
            const begin_len = line.length - attr.length;
            attr = attr.trimEnd();
            const end_len = begin_len + attr.length,
              begin = line.slice(0, begin_len),
              end = line.slice(end_len),
              set = (txt) => (line = begin + txt + end),
              wrap = (txt, attr) => {
                if (!attr.startsWith("{")) {
                  attr = "{" + attr + "}";
                }
                if (txt.endsWith("=")) {
                  txt = txt.slice(0, -1) + "!=";
                }
                return set(txt + '"' + attr + '"');
              },
              replace = (key, to) => {
                const at_pos = attr.indexOf(key) + key.length,
                  pos = attr.indexOf("=", at_pos) + 1;
                return wrap(
                  attr.slice(0, at_pos - 1) + to + attr.slice(at_pos, pos),
                  attr.slice(pos),
                );
              };
            if (attr && attr.indexOf('="') < 0) {
              if (attr.endsWith(":")) {
                set("{" + attr.slice(0, attr.length - 1) + "}");
              } else {
                switch (attr.charAt(0)) {
                  case "@":
                    if (attr.charAt(1) === "&") {
                      wrap("bind:this=", attr.slice(2));
                    } else {
                      const eq_pos = attr.indexOf("=");
                      if (eq_pos < 0) {
                        attr = attr.slice(1);
                        if (attr !== "message") {
                          wrap("on" + attr + "=", attr.split("|", 1)[0]);
                        } else {
                          set("on" + attr);
                        }
                      } else {
                        replace("@", "on");
                      }
                    }
                    break;
                  case "&":
                    wrap("bind:value=", attr.slice(1));
                    break;
                  default:
                    const amp_pos = attr.indexOf("&"),
                      eq_pos = attr.indexOf("=");
                    if (eq_pos > 0 && attr.charAt(eq_pos + 1) === "{") {
                      const pre = attr.slice(0, eq_pos),
                        val = attr.slice(eq_pos + 1);
                      if (pre.endsWith("!")) {
                        set(pre + '="' + val + '"');
                      } else {
                        wrap(pre + "=", val);
                      }
                    } else if (amp_pos > 0 && eq_pos < 0) {
                      wrap("bind:" + attr.slice(0, amp_pos) + "=", attr.slice(amp_pos + 1));
                    } else {
                      const col_pos = attr.indexOf(":");
                      if (col_pos > 0 && eq_pos < 0) {
                        wrap(attr.slice(0, col_pos) + "=", attr.slice(col_pos + 1));
                      } else if (col_pos > 0 && col_pos < eq_pos) {
                        wrap(attr.slice(0, +eq_pos + 1 || 9e9), attr.slice(eq_pos + 1));
                      }
                    }
                }
              }
            }
            return line;
          })
          .join(" ");
      return "(" + splitted + ")";
    });

export default (pug) => {
  const li = [],
    pug_li = bind(pug).join(""),
    ref = pug_li.split("\n");
  for (let line of ref) {
    const ts = line.trimStart(),
      i = ts.trimEnd();
    if (i.charAt(0) === "+") {
      const pos = i.indexOf(" ", 2);
      if (pos > 0) {
        const cmd = i.slice(1, pos);
        if (CMD.has(cmd)) {
          let arg = i.slice(pos + 1);
          if ((cmd === "snippet" || cmd === "render") && !arg.endsWith(")")) {
            arg += "()";
          }
          line =
            "".padEnd(line.length - ts.length) +
            "+" +
            (cmd === "elif" ? "elseif" : cmd) +
            "('" +
            arg.replaceAll("'", "\\'") +
            "')";
        }
      }
    }
  }
  return li.join("\n");
};
