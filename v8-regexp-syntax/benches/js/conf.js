#!/usr/bin/env bun
// 评测公共配置：路径、基线名、语言；bench.js 与 benches/js 脚本共用

import { join } from "node:path";

const js_dir = import.meta.dirname;
const crate_dir = join(js_dir, "../..");
// cargo 以 crate 目录为 bench 进程 cwd，target 固定用绝对路径隔离并发
const target_dir = join(crate_dir, "target/bench-regexp");
const out_dir = join(crate_dir, "bench");

const conf = {
  js_dir,
  crate_dir,
  target_dir,
  criterion_dir: join(target_dir, "criterion"),
  // criterion 基线目录名（cargo bench -- --save-baseline <baseline>）
  baseline: process.env.BENCH_BASELINE || "bench",
  bench_json: join(out_dir, "bench.json"),
  table_path: join(out_dir, "table.html"),
  md_path: join(out_dir, "report.md"),
  svg_path: join(out_dir, "report.svg"),
  lang: process.env.BENCH_LANG || "zh",
};

// 按语言名加载 benches/js/i18n/<语言名>.js 语言文件
const i18nRead = async (name) => import(join(js_dir, "i18n", `${name}.js`));

export default conf;
export { i18nRead };
