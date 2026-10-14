#!/usr/bin/env bun
// v8-common 评测入口 (本文件生成后禁止改动):
// 1) zx 跑 cargo bench -p v8-common --features bench (CARGO_TARGET_DIR=target/bench-common)
// 2) 解析 criterion 输出 json → 汇总结果 bench/bench.json
// 3) 驱动 table.js / md.js / svg.js 生成报告
import { join } from "node:path";

import { $, cd, echo, within } from "zx";

import { CRATE_DIR, CRITERION_DIR, RESULT_DIR } from "./benches/js/conf.js";
import { buildSummary, parseCriterion, writeJson } from "./benches/js/lib.js";

await within(async () => {
  cd(CRATE_DIR);
  $.env.CARGO_TARGET_DIR = join(CRATE_DIR, "..", "target", "bench-common");
  await $`cargo bench -p v8-common --features bench`;
});

const results = await parseCriterion(CRITERION_DIR);
await writeJson(join(RESULT_DIR, "bench.json"), buildSummary(results));

await within(async () => {
  cd(CRATE_DIR);
  await $`bun benches/js/table.js`;
  await $`bun benches/js/md.js`;
  await $`bun benches/js/svg.js`;
});

echo(`bench done: ${join(RESULT_DIR, "bench.json")}`);
