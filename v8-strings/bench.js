#!/usr/bin/env bun
// v8-strings 评测入口 (genbench 第 8 步): zx 跑 cargo bench --features bench (criterion, 短测量
// 时间冒烟口径), 解析 criterion 输出 json 与 bench_manifest.json, 生成汇总 bench-result.json,
// 最后调用 benches/js 下 table.js / md.js / svg.js 生成报告。本文件创建后禁止改动。

import { resolve } from "node:path";
import { $ } from "zx";

const CRATE_DIR = import.meta.dirname,
  TARGET_DIR = resolve(CRATE_DIR, "../target/bench-strings"),
  CRITERION_DIR = resolve(TARGET_DIR, "criterion");

process.env.CARGO_TARGET_DIR = TARGET_DIR;
$.verbose = false;
$.cwd = CRATE_DIR;

await $`cargo bench -p v8-strings --features bench --bench main -- --sample-size 10 --warm-up-time 1 --measurement-time 2 --quiet`;

const { entries } = await Bun.file(`${CRITERION_DIR}/bench_manifest.json`).json(),
  read_entry = async (entry) => {
    const estimates = await Bun.file(
      `${CRITERION_DIR}/${entry.op}_${entry.impl}_${entry.sample}/new/estimates.json`,
    ).json();
    return {
      op: entry.op,
      impl: entry.impl,
      sample: entry.sample,
      elems: entry.elems,
      ns_mean: estimates.mean.point_estimate,
      ns_median: estimates.median.point_estimate,
      ci_low: estimates.mean.confidence_interval.lower_bound,
      ci_high: estimates.mean.confidence_interval.upper_bound,
    };
  };

const results = [];
for (const entry of entries) results.push(await read_entry(entry));

const summary = { crate: "v8-strings", generated: new Date().toISOString(), results };
await Bun.write(`${CRATE_DIR}/bench-result.json`, `${JSON.stringify(summary, null, 2)}\n`);

await $`bun ${CRATE_DIR}/benches/js/table.js`.pipe(process.stdout);
await $`bun ${CRATE_DIR}/benches/js/md.js`.pipe(process.stdout);
await $`bun ${CRATE_DIR}/benches/js/svg.js`.pipe(process.stdout);

process.stdout.write(`${results.length} 项评测已写入 bench-result.json\n`);
