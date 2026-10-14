#!/usr/bin/env bun
// 目的: 报告脚本公共配置 — 路径常量 / 语言选择 / i18n 加载
import { path } from "zx";

export const js_dir = import.meta.dirname;
export const crate_dir = path.join(js_dir, "..", "..");
export const result_json = path.join(crate_dir, "bench-result.json");
export const lang = process.env.BENCH_LANG || "en";

// 加载 ./i18n/<语言>.js 语言文件 (export const 大写变量名)
export const i18n_read = async (name = lang) =>
  await import(path.join(js_dir, "i18n", `${name}.js`));
