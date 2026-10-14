#!/usr/bin/env bun
// v8-numbers 评测主入口 — 用 zx 跑 cargo bench, 解析 criterion 输出 json,
// 生成汇总 bench.json, 再调用 benches/js 下报告脚本。
// 本文件生成后禁止改动 (genbench 步骤 5)。

import { $, cd, path } from "zx";
import { readdir, readFile, writeFile } from "node:fs/promises";

const CRITERION_DIR = "target/bench-numbers/criterion";
const BENCH_JSON = "bench.json";

const read_json = async (file) => JSON.parse(await readFile(file, "utf8"));

// 遍历 criterion/<op>/<name>/new/{benchmark.json,estimates.json}
const collect = async (criterion_dir) => {
  const ops = [];
  for (const op of await readdir(criterion_dir, { withFileTypes: true })) {
    if (!op.isDirectory()) {
      continue;
    }
    const results = [];
    for (const imp of await readdir(path.join(criterion_dir, op.name), { withFileTypes: true })) {
      const new_dir = path.join(criterion_dir, op.name, imp.name, "new");
      if (!imp.isDirectory() || !(await readdir(new_dir)).includes("estimates.json")) {
        continue;
      }
      const { throughput } = await read_json(path.join(new_dir, "benchmark.json"));
      const { mean, median } = await read_json(path.join(new_dir, "estimates.json"));
      const elements = throughput?.Elements ?? 0;
      results.push({
        name: imp.name,
        ns_per_iter: mean.point_estimate,
        std_err: mean.standard_error,
        ci: [mean.confidence_interval.lower_bound, mean.confidence_interval.upper_bound],
        median_ns: median.point_estimate,
        elements,
        ns_per_elem: elements ? mean.point_estimate / elements : null,
      });
    }
    results.sort((a, b) => a.ns_per_iter - b.ns_per_iter);
    if (results.length) {
      ops.push({ op: op.name, results });
    }
  }
  ops.sort((a, b) => a.op.localeCompare(b.op));
  return ops;
};

const main = async () => {
  cd(import.meta.dirname);
  global.process.env.CARGO_TARGET_DIR ??= "target/bench-numbers";
  await $`cargo bench -p v8-numbers --features bench --bench main`;

  const bench = {
    crate: "v8-numbers",
    generated_at: new Date().toISOString(),
    ops: await collect(CRITERION_DIR),
  };
  await writeFile(BENCH_JSON, `${JSON.stringify(bench, null, 2)}\n`);

  await $`bun benches/js/table.js`;
  await $`bun benches/js/md.js`;
  await $`bun benches/js/svg.js`;

  console.log(`bench 汇总: ${path.resolve(BENCH_JSON)}
报告: ${path.resolve("bench.md")}
图表: ${path.resolve("bench.svg")}`);
};

await main();
