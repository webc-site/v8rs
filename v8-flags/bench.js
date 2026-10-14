#!/usr/bin/env bun
// v8-flags 评测入口 (本文件生成后禁止改动):
// 用 zx 跑 `cargo bench -p v8-flags --features bench` (CARGO_TARGET_DIR 隔离),
// 解析 criterion 输出 json 生成汇总 bench-result.json, 再驱动三个报告脚本产出
// table/csv、markdown、svg

import { $ } from "zx";

import { criterion_dir, result_json, root_dir, target_dir } from "./benches/js/conf.js";
import { collectMeasures, writeText } from "./benches/js/lib.js";
import runMd from "./benches/js/md.js";
import runSvg from "./benches/js/svg.js";
import runTable from "./benches/js/table.js";

// CARGO_TARGET_DIR 用绝对路径: cargo 以 crate 目录为 cwd 运行 bench 可执行文件,
// 相对路径会让 criterion 结果落进 crate 目录
const bench_opts = {
  cwd: root_dir,
  env: { ...process.env, CARGO_TARGET_DIR: target_dir },
};

await $({ ...bench_opts, quiet: true })`rm -rf ${criterion_dir}`;
await $(bench_opts)`cargo bench -p v8-flags --features bench`;

const result = await collectMeasures(criterion_dir);
await writeText(result_json, JSON.stringify(result, null, 2));

const artifacts = [result_json, await runTable(), await runMd(), await runSvg()];
const measure_lines = result.measures.map(
  ({ group, impl, mean_ns }) => `${group.padEnd(14)} ${impl.padEnd(12)} ${mean_ns.toFixed(1)} ns`,
);
process.stdout.write(
  `v8-flags bench done: ${result.measures.length} measures × [${result.impls.join(", ")}]
${measure_lines.join("\n")}
artifacts:
${artifacts.join("\n")}
`,
);
