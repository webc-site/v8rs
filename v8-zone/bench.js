#! /usr/bin/env bun
// v8-zone 评测入口 (genbench 第 8 步, 本文件生成后禁止修改)
// 用 zx 跑 cargo bench (criterion), 解析 criterion 输出 json, 生成汇总结果 bench.json
// 报告生成脚本: benches/js/table.js / md.js / svg.js

import { join } from "path";
import { $ } from "zx";
import { buildSummary, findBenchmarkFiles, jsonWrite, toRecord } from "./benches/js/lib.js";

const crate_dir = import.meta.dirname;
const target_dir = join(crate_dir, "target/bench-zone");
const criterion_dir = join(target_dir, "criterion");

await $({
  cwd: crate_dir,
  env: { ...process.env, CARGO_TARGET_DIR: target_dir },
})`cargo bench -p v8-zone --features bench --bench main`;

const files = await findBenchmarkFiles(criterion_dir);
const records = [];
for (const file of files) records.push(await toRecord(file));
await jsonWrite(join(crate_dir, "bench.json"), buildSummary(records));
