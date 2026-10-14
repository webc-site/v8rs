// conf.js — 报告脚本公共配置: 路径、语言、基线组; md.js 等加载语言文案用 i18nRead
import { resolve } from "node:path";

const ROOT_DIR = resolve(`${import.meta.dirname}/../..`),
  LANG = process.env.BENCH_LANG || "zh";

export const CRATE_NAME = "v8-date",
  BASELINE_GROUP = "v8-date",
  RESULT_JSON = `${ROOT_DIR}/bench-result.json`,
  TABLE_OUT = `${ROOT_DIR}/bench-report.txt`,
  MD_OUT = `${ROOT_DIR}/bench-report.md`,
  SVG_OUT = `${ROOT_DIR}/bench-report.svg`;

const texts = await import(`${import.meta.dirname}/i18n/${LANG}.js`);

// 按当前语言读取文案变量 (BENCH_LANG=en|zh, 默认 zh)
export const i18nRead = (key) => texts[key] ?? key;

export default { CRATE_NAME, BASELINE_GROUP, RESULT_JSON, TABLE_OUT, MD_OUT, SVG_OUT, i18nRead };
