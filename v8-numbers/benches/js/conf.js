// 公共配置 — 路径、语言选择与 i18n 加载
import { readFile } from "node:fs/promises";
import path from "node:path";

const js_dir = import.meta.dirname;

export const CRATE_DIR = path.dirname(path.dirname(js_dir));
export const BENCH_JSON = path.join(CRATE_DIR, "bench.json");

// zh 环境用中文, 否则英文
export const pickLang = () =>
  String(process.env.LANG ?? "")
    .toLowerCase()
    .startsWith("zh")
    ? "zh"
    : "en";

// 加载 benches/js/i18n/<lang>.js 语言模块 (export const 大写变量名)
export const i18nRead = async (lang) => await import(path.join(js_dir, "i18n", `${lang}.js`));

export const readJson = async (file) => JSON.parse(await readFile(file, "utf8"));

export default { CRATE_DIR, BENCH_JSON, pickLang, i18nRead, readJson };
