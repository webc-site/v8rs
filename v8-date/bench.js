#!/usr/bin/env bun
// bench.js — v8-date criterion 评测入口 (本文件禁止后续改动)
// 职责: 用 zx 运行 cargo bench -p v8-date --features bench, 解析 criterion 输出 json, 生成汇总 bench-result.json
// 用法: ./bench.js [criterion 附加参数...]  例: ./bench.js -- --warm-up-time 1 --measurement-time 2

import { readFile, readdir, writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import { $ } from "zx";

const CRATE_NAME = "v8-date",
  ROOT_DIR = import.meta.dirname,
  TARGET_DIR = resolve(`${ROOT_DIR}/../target/bench-date`),
  CRITERION_DIR = `${TARGET_DIR}/criterion`,
  RESULT_PATH = `${ROOT_DIR}/bench-result.json`,
  EXTRA_ARGS = process.argv.slice(2),
  BENCH_ARGS = EXTRA_ARGS.length
    ? EXTRA_ARGS
    : ["--warm-up-time", "2", "--measurement-time", "3", "--sample-size", "20"];

await $({
  cwd: ROOT_DIR,
  env: { ...process.env, CARGO_TARGET_DIR: TARGET_DIR },
})`cargo bench -p ${CRATE_NAME} --features bench --bench main -- ${BENCH_ARGS}`;

// 收集 <组名>/<评测id>/new/estimates.json, 汇总 mean/median 点估计与置信区间
const readFileText = async (file_path) => await readFile(file_path, "utf8");

const collect_benches = async () => {
  const entries = await readdir(CRITERION_DIR, { recursive: true, withFileTypes: true });
  const estimates = entries.filter(
    (e) => e.isFile() && e.name === "estimates.json" && e.parentPath.endsWith("/new"),
  );
  const benches = [];
  for (const item of estimates) {
    const rel_path = item.parentPath.slice(CRITERION_DIR.length + 1);
    const [group, bench] = rel_path.split("/");
    const est = JSON.parse(await readFileText(`${item.parentPath}/estimates.json`));
    benches.push({
      name: `${group}/${bench}`,
      group,
      bench,
      mean_ns: est.mean.point_estimate,
      median_ns: est.median.point_estimate,
      std_err_ns: est.mean.standard_error,
      ci_low_ns: est.mean.confidence_interval.lower_bound,
      ci_high_ns: est.mean.confidence_interval.upper_bound,
      slope_ns: est.slope?.point_estimate ?? null,
    });
  }
  return benches.sort((a, b) => a.name.localeCompare(b.name));
};

const summary = {
  crate: CRATE_NAME,
  generated_at: new Date().toISOString(),
  bench_args: BENCH_ARGS,
  criterion_dir: CRITERION_DIR,
  benches: await collect_benches(),
};

await writeFile(RESULT_PATH, `${JSON.stringify(summary, null, 2)}\n`);

console.info(`bench-result.json 已生成: ${summary.benches.length} 项评测 -> ${RESULT_PATH}`);
