#!/usr/bin/env bun
// benches/js/conf.js — 评测报告的公共配置: 路径、语言选择、i18n 加载
// 引用: 语言文件在 ./i18n/<lang>.js 机制下由 i18nRead 动态 import
// 路径全部 normalize 为绝对路径: bench 子进程工作目录是 crate 目录 (cargo 约定),
// 相对 CARGO_TARGET_DIR 会漂移

import { resolve } from "node:path";

const js_dir = import.meta.dirname;

export const crate_dir = resolve(`${js_dir}/../..`);
export const root_dir = resolve(`${crate_dir}/..`);
export const target_dir = `${root_dir}/target/bench-flags`;
export const criterion_dir = `${target_dir}/criterion`;
export const result_json = `${crate_dir}/bench-result.json`;
export const table_csv = `${crate_dir}/bench-table.csv`;
export const md_report = `${crate_dir}/bench-report.md`;
export const svg_chart = `${crate_dir}/bench-chart.svg`;

export const lang = process.env.BENCH_LANG ?? "zh";

const i18n_cache = {};

export const i18nRead = async () => {
  i18n_cache[lang] ??= await import(`${js_dir}/i18n/${lang}.js`);
  return i18n_cache[lang];
};

export default i18nRead;
