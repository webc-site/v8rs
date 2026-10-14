// 评测报告公共配置: 路径常量与 i18n 加载 (md.js / svg.js / table.js 按此取数)。

import { resolve } from "node:path";

const CRATE_DIR = resolve(import.meta.dirname, "../.."),
  LANG = process.env.BENCH_LANG ?? "zh",
  I18N_MODS = {
    en: () => import(`./i18n/en.js`),
    zh: () => import(`./i18n/zh.js`),
  };

const strings = await I18N_MODS[LANG]();

// 按大写变量名读取当前语言的字符串, 缺失时原样返回键名。
export const i18nRead = (key) => strings[key] ?? key;

export const [SUMMARY_JSON, MD_OUT, SVG_OUT] = [
  `${CRATE_DIR}/bench-result.json`,
  `${CRATE_DIR}/bench-report.md`,
  `${CRATE_DIR}/bench-report.svg`,
];

export const IMPL_ORDER = ["v8rs", "memchr", "unicode-ident", "unicode-normalization", "std"];

export const OPS_ORDER = [
  "transcode_encode",
  "transcode_decode",
  "copy_chars",
  "scan_identifier",
  "case_lower",
  "find_byte",
  "find_substring",
  "normalize",
];
