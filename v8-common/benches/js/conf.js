// 报告脚本公共配置: 目录常量、语言选择、i18n 加载入口
import { join, resolve } from "node:path";

import yargs from "yargs";
import { hideBin } from "yargs/helpers";

const JS_DIR = import.meta.dirname;
const CRATE_DIR = resolve(JS_DIR, "../..");
const ROOT_DIR = resolve(CRATE_DIR, "..");
const TARGET_DIR = join(ROOT_DIR, "target", "bench-common");
const CRITERION_DIR = join(TARGET_DIR, "criterion");
const RESULT_DIR = join(CRATE_DIR, "bench");

export { CRATE_DIR, CRITERION_DIR, RESULT_DIR, ROOT_DIR, TARGET_DIR };

const argv = await yargs(hideBin(process.argv)).option("lang", { type: "string" }).argv;

export const LANG = argv.lang ?? process.env.BENCH_LANG ?? "zh";

// 加载 benches/js/i18n/<LANG>.js 语言模块 (export const 大写变量名)
export const i18nRead = async () => await import(`./i18n/${LANG}.js`);
