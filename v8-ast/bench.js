#! /usr/bin/env bun
// v8-ast genbench 主评测脚本 (步骤 8, 禁止后续改动):
// 用 zx 跑 `cargo bench -p v8-ast --features bench` (隔离 CARGO_TARGET_DIR=target/bench-ast),
// 解析 criterion 输出 json (criterion/<id>/new/{benchmark,estimates}.json),
// 生成 crate 根汇总结果 bench.json 供 benches/js/ 报告脚本消费

import { readdir, readFile, writeFile } from "node:fs/promises";
import { $ } from "zx";

const CRATE_DIR = import.meta.dirname;
const TARGET_DIR = `${CRATE_DIR}/../target/bench-ast`;
const CRITERION_DIR = `${TARGET_DIR}/criterion`;
const RESULT_FILE = `${CRATE_DIR}/bench.json`;

await $({
  cwd: CRATE_DIR,
  env: { ...process.env, CARGO_TARGET_DIR: TARGET_DIR },
  stdio: "inherit",
})`cargo bench -p v8-ast --features bench`;

const entries = await readdir(CRITERION_DIR, { withFileTypes: true });
const dirs = entries.filter((e) => e.isDirectory() && e.name !== "report");

const results = [];
for (const d of dirs) {
  const base = `${CRITERION_DIR}/${d.name}/new`;
  const meta = JSON.parse(await readFile(`${base}/benchmark.json`, "utf8"));
  const estimates = JSON.parse(await readFile(`${base}/estimates.json`, "utf8"));
  const [impl, op, param = ""] = String(meta.group_id).split("/");
  results.push({
    id: meta.group_id,
    impl,
    op,
    param,
    mean_ns: estimates.mean.point_estimate,
    median_ns: estimates.median.point_estimate,
    std_dev_ns: estimates.std_dev.point_estimate,
    ci95_ns: [
      estimates.mean.confidence_interval.lower_bound,
      estimates.mean.confidence_interval.upper_bound,
    ],
  });
}

results.sort((a, b) => (a.op === b.op ? a.param.localeCompare(b.param) : a.op.localeCompare(b.op)));
const impls = [...new Set(results.map((r) => r.impl))].sort();
const summary = {
  crate: "v8-ast",
  generated_at: new Date().toISOString(),
  impls,
  results,
};

await writeFile(RESULT_FILE, `${JSON.stringify(summary, null, 2)}\n`);

console.log(`已生成 ${RESULT_FILE}: ${results.length} 项评测, 实现 ${impls.join(", ")}`);
