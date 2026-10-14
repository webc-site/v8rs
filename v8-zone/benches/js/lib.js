// v8-zone bench 公共件: criterion json 解析/转换与汇总结果生成 (genbench 9.6)

import { readdir, readFile, writeFile } from "fs/promises";
import { join } from "path";

// criterion 输出目录: <CARGO_TARGET_DIR>/criterion/<op>/<impl>/<param>/new/benchmark.json
export const findBenchmarkFiles = async (dir) => {
  const out = [];
  const walk = async (d) => {
    for (const entry of await readdir(d, { withFileTypes: true })) {
      const p = join(d, entry.name);
      if (entry.isDirectory()) await walk(p);
      // 只取 new/ (base/ 是上次基线副本)
      else if (p.endsWith(join("new", "benchmark.json"))) out.push(p);
    }
  };
  await walk(dir);
  return out.sort();
};

export const jsonRead = async (path) => JSON.parse(await readFile(path, "utf8"));

export const jsonWrite = async (path, data) => await writeFile(path, JSON.stringify(data, null, 2));

// "size16_n200" -> { size: 16, n: 200 }
export const parseKv = (str) => {
  const params = {};
  for (const kv of str.matchAll(/([a-z]+)(\d+)/g)) params[kv[1]] = Number(kv[2]);
  return params;
};

// 单个 benchmark.json + estimates.json -> 记录
export const toRecord = async (benchmarkPath) => {
  const bench = await jsonRead(benchmarkPath);
  const est = await jsonRead(benchmarkPath.replace("benchmark.json", "estimates.json"));
  const [op, impl, param] = bench.full_id.split("/");
  const params = parseKv(param);
  const mean_ns = est.mean.point_estimate;
  // 每轮分配对象数: 作用域按深度展开
  const count = (params.depth ?? 1) * (params.n ?? 0);
  const bytes = bench.throughput?.Bytes ?? 0;
  return {
    op,
    impl,
    param,
    params,
    id: bench.full_id,
    mean_ns,
    ci_ns: [est.mean.confidence_interval.lower_bound, est.mean.confidence_interval.upper_bound],
    std_dev_ns: est.std_dev.point_estimate,
    allocs_per_sec: (count * 1e9) / mean_ns,
    bytes_per_sec: (bytes * 1e9) / mean_ns,
  };
};

export const BASELINE = "v8rs-zone";

// 汇总: 按 op/param/impl 排序, 附相对基线倍率
export const buildSummary = (records) => {
  const order = [BASELINE, "bumpalo", "std-vec"];
  records.sort(
    (a, b) =>
      a.op.localeCompare(b.op) ||
      (a.params.size ?? 0) - (b.params.size ?? 0) ||
      (a.params.depth ?? 0) - (b.params.depth ?? 0) ||
      order.indexOf(a.impl) - order.indexOf(b.impl),
  );
  const ops = [...new Set(records.map((r) => r.op))];
  const impls = order.filter((i) => records.some((r) => r.impl === i));
  const withRatio = records.map((r) => {
    const base = records.find((b) => b.op === r.op && b.param === r.param && b.impl === BASELINE);
    return { ...r, ratio: base ? base.mean_ns / r.mean_ns : null };
  });
  return {
    crate: "v8-zone",
    baseline: BASELINE,
    generated: new Date().toISOString(),
    ops,
    impls,
    benchmarks: withRatio,
  };
};

// 数字缩写: 1.2G / 34.5M / 780K
export const fmtNum = (v) => {
  const units = ["G", "M", "K"];
  let scaled = v;
  for (const u of units) {
    scaled /= 1000;
    if (Math.abs(scaled) < 1000) return `${scaled.toFixed(scaled >= 100 ? 0 : 1)}${u}`;
  }
  return scaled.toFixed(1);
};

export const fmtRatio = (v) => (v === null ? "-" : `×${v.toFixed(2)}`);
