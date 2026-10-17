import extract_replace_include from "@3-/extract/extractReplaceInclude.js";
import pug2htm from "./toHtm.js";
import pug_hack from "./hack.js";

const render = (content, filename, locals = {}) =>
  pug2htm(
    pug_hack(content),
    filename,
    locals,
  )[0].replace(/="\{([^"]*)\}"/g, "={$1}");

export default () => {
  return {
    markup: (o) => {
      const code = extract_replace_include(
        '<template lang="pug">',
        "</template>",
        (pug) => "\n" + render(pug.slice(21, -11), o.filename),
        o.content,
      );
      if (code) {
        return {
          code,
          map: {
            version: 3,
            file: o.filename,
            sources: [o.filename],
            sourcesContent: [o.content],
            names: [],
            mappings: "",
          },
        };
      }
    },
  };
};
