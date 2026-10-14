#!/usr/bin/env bun
// 目的: 用 zx 跑 `cargo bench -p v8-bigint --features bench` (criterion),
// 解析 criterion 输出 json, 汇总生成 crate 根目录 bench-result.json
// 本文件为评测固定入口, 禁止后续改动 (参数可通过环境变量 BENCH_ARGS 覆盖, 不改本文件)
// --bench bigint 限定只跑 criterion 目标, 避免 libtest 目标拒绝 criterion 参数
import { $, cd, echo, fs, path } from "zx";
import { collect_estimates } from "./benches/js/lib.js";

const crate_dir = import.meta.dirname;
// 并发隔离: 固定绝对路径私有 target, 避免与其他 worktree/agent 抢锁
const target_dir = path.join(crate_dir, "..", "target", "bench-bigint");
const criterion_dir = path.join(target_dir, "criterion");
const bench_args = (process.env.BENCH_ARGS ?? "--warm-up-time 1 --measurement-time 2").split(" ");

cd(crate_dir);
await $({
  env: { ...process.env, CARGO_TARGET_DIR: target_dir },
  stdio: "inherit",
})`cargo bench -p v8-bigint --features bench --bench bigint -- ${bench_args}`;

const rows = await collect_estimates(criterion_dir);
await fs.outputFile(
  path.join(crate_dir, "bench-result.json"),
  `${JSON.stringify(rows, null, 2)}\n`,
);
echo(`bench-result.json: ${rows.length} 条`);
