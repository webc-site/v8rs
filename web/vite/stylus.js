import stylus from "@3-/stylus";
import read from "@3-/read";
import path from "path";
import fs from "fs";

export const inlineSvg = (code, id) => {
  let has_changes = false;
  const clean_id = id.split("?")[0],
    url_re = /url\(\s*(\\?['"])?([^'"\\)]+?\.svg(?:[#?][^'"\\)]*)?)\\?['"]?\s*\)/g,
    new_code = code.replace(url_re, (match, quote = "", rel_path) => {
      const [path_part, hash_part = ""] = rel_path.split(/(?=[#?])/);
      let svg_path = path.resolve(path.dirname(clean_id), path_part);
      if (fs.existsSync(svg_path)) {
        has_changes = true;
        const raw = read(svg_path).trim().replaceAll('"', "'").replace(/\s+/g, " "),
          content =
            "data:image/svg+xml," +
            [..."%#<>,'"].reduce((r, c) => r.replaceAll(c, encodeURIComponent(c)), raw);
        return 'url("' + content + hash_part + '")';
      }
      return match;
    });
  return has_changes ? new_code : code;
};

export const stylusVitePlugin = () => ({
  name: "vite-plugin-custom-stylus",
  enforce: "pre",
  resolveId(source, importer) {
    if (source.endsWith(".styl")) {
      const base_dir = importer
        ? path.dirname(importer)
        : path.resolve(import.meta.dirname, "../src");
      const resolved = path.resolve(base_dir, source);
      return resolved + ".css";
    }
    return null;
  },
  load(id) {
    const clean_id = id.split("?")[0];
    if (clean_id.endsWith(".styl.css")) {
      const styl_path = clean_id.slice(0, -4);
      if (fs.existsSync(styl_path)) {
        this.addWatchFile(styl_path);
        const content = read(styl_path);
        const compiler = stylus(content, { filename: styl_path });
        const css = compiler.render();
        if (compiler.deps) {
          compiler.deps().forEach((dep) => this.addWatchFile(dep));
        }
        return {
          code: inlineSvg(css, styl_path),
          map: null,
        };
      }
    }
    return null;
  },
});

export const svelteStylusPreprocessor = () => ({
  style: ({ content, attributes, filename }) => {
    if (attributes.lang === "stylus" || attributes.lang === "styl") {
      const file_path = filename || "style.styl";
      const compiler = stylus(content, { filename: file_path });
      const css = compiler.render();
      return {
        code: inlineSvg(css, file_path),
      };
    }
  },
});
