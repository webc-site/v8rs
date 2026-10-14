// 报告脚本配置: 路径常量与语言文件加载 (genbench 9.4)

import { join } from "path";

export const jsDir = import.meta.dirname;
export const crateDir = join(jsDir, "..", "..");
export const benchJsonPath = join(crateDir, "bench.json");
export const reportDir = join(crateDir, "report");
export const LANG = process.env.BENCH_LANG ?? "zh";

// 加载 i18n/<LANG>/<name>.js 语言模块; name 省略时取 i18n/<LANG>.js
export const i18nRead = async (name = "") =>
  name
    ? await import(join(jsDir, "i18n", LANG, `${name}.js`))
    : await import(join(jsDir, "i18n", `${LANG}.js`));
