#!/usr/bin/env bun
// v8-parsing 评测总入口 (本文件按 genbench 约定生成后不再改动)
// 用 zx 跑 cargo bench: --bench parsing_bench 指定 criterion 目标 (避免 lib bench harness 拒收参数),
// 短测量时间参数保证冒烟可重复; 解析 criterion 输出 json 汇总为 bench/result.json,
// 再依次调用 table.js / md.js / svg.js 生成报告.

import "zx/globals";
import { collect, write_json } from "./benches/js/lib.js";

$.verbose = false;

const CRATE = import.meta.dirname;
const ROOT = `${CRATE}/..`;
const TARGET = `${ROOT}/target/bench-parsing`;
const BENCH_DIR = `${CRATE}/bench`;
const RESULT = `${BENCH_DIR}/result.json`;
const JS_DIR = `${CRATE}/benches/js`;

const run_bench = async () => {
  await $({
    cwd: ROOT,
    env: { ...process.env, CARGO_TARGET_DIR: TARGET },
    stdio: "inherit",
  })`cargo bench -p v8-parsing --features bench --bench parsing_bench -- --sample-size 30 --warm-up-time 1 --measurement-time 2`;
};

const gen_reports = async () => {
  await $({ cwd: CRATE, stdio: "inherit" })`${`${JS_DIR}/table.js`}`;
  await $({ cwd: CRATE, stdio: "inherit" })`${`${JS_DIR}/md.js`}`;
  await $({ cwd: CRATE, stdio: "inherit" })`${`${JS_DIR}/svg.js`}`;
};

const main = async () => {
  await run_bench();
  write_json(RESULT, collect(`${TARGET}/criterion`));
  await gen_reports();
};

await main();
