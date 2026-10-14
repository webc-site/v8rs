#!/usr/bin/env bun
// 评测配置: 路径约定与 i18n 语言加载 (BENCH_LANG=zh|en, 默认 zh)

export const JS_DIR = import.meta.dirname;
export const CRATE_DIR = `${JS_DIR}/../..`;
export const BENCH_DIR = `${CRATE_DIR}/bench`;
export const RESULT_PATH = `${BENCH_DIR}/result.json`;
export const LANG = process.env.BENCH_LANG ?? "zh";

export const i18nRead = async (lang = LANG) =>
  await import(new URL(`./i18n/${lang}.js`, import.meta.url).href);

export default { JS_DIR, CRATE_DIR, BENCH_DIR, RESULT_PATH, LANG, i18nRead };
