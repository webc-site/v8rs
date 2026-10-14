#!/usr/bin/env bun
// genbench 总入口：跑 cargo bench（criterion），解析输出为 bench/bench.json 汇总，
// 再驱动 benches/js 的 table/md/svg 生成报告
// 本文件生成后禁止改动
// 环境变量：BENCH_BASELINE 基线名（默认 bench）、BENCH_LANG 报告语言（默认 zh）、
// SKIP_CARGO_BENCH=1 仅重新汇总已有 criterion 数据（调试报告脚本用）

import { $ } from "zx";

import conf from "./benches/js/conf.js";
import { collect_estimates, fmt_time, with_rel, write_text } from "./benches/js/lib.js";
import gen_md from "./benches/js/md.js";
import gen_svg from "./benches/js/svg.js";
import gen_table from "./benches/js/table.js";

// 评测操作固定顺序（与 benches/base.rs OPS 一致）
const OPS_ORDER = ["parse", "char_ranges", "format_ast"];

const run_cargo_bench = async () => {
  await $({
    cwd: conf.crate_dir,
    env: { ...process.env, CARGO_TARGET_DIR: conf.target_dir },
  })`cargo bench -p v8-regexp-syntax --features bench --bench main -- --save-baseline ${conf.baseline} --warm-up-time 0.2 --measurement-time 0.3 --sample-size 20`;
};

const gen = async () => {
  if (!process.env.SKIP_CARGO_BENCH) await run_cargo_bench();
  const rows = with_rel(await collect_estimates(conf.criterion_dir, conf.baseline));
  const summary = {
    generated_at: new Date().toISOString(),
    baseline: conf.baseline,
    ops: OPS_ORDER,
    results: rows,
  };
  await write_text(conf.bench_json, `${JSON.stringify(summary, null, 2)}\n`);
  const paths = await Promise.all([gen_table(), gen_md(), gen_svg()]);
  const report = [
    `v8-regexp-syntax bench（基线 ${conf.baseline}，样本 ${summary.results.length} 项）`,
    ...rows.map(
      (row) =>
        `${row.op}  ${row.target}  mean ${fmt_time(row.mean_ns)}  median ${fmt_time(row.median_ns)}  ${row.rel.toFixed(2)}x`,
    ),
    `汇总: ${conf.bench_json}`,
    ...paths,
  ].join("\n");
  console.log(report);
};

await gen();

export default gen;
