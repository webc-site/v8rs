// json 解析/转换公共件: criterion 产物目录 → 汇总行; 数值格式化
import { mkdir, readdir, readFile, writeFile } from "node:fs/promises";
import { dirname } from "node:path";

export const OPS = ["tag_classify", "compress_roundtrip", "align_ops", "hash_probe"];
export const IMPL_ORDER = ["v8-common", "std", "gxhash"];

export const readJson = async (path) => JSON.parse(await readFile(path, "utf8"));

export const writeJson = async (path, data) => {
  await mkdir(dirname(path), { recursive: true });
  await writeFile(path, `${JSON.stringify(data, null, 2)}\n`);
};

// 扫描 <CARGO_TARGET_DIR>/criterion/<impl>/<op>/new/estimates.json,
// 提取 median/mean 点估计与 95% 置信区间 (单位 ns)
export const parseCriterion = async (criterion_dir) => {
  const impls = (await readdir(criterion_dir, { withFileTypes: true }))
    .filter((e) => e.isDirectory() && e.name !== "report")
    .map((e) => e.name)
    .sort((a, b) => rankImpl(a) - rankImpl(b) || a.localeCompare(b));
  const rows = [];
  for (const impl of impls) {
    const ops = (await readdir(`${criterion_dir}/${impl}`, { withFileTypes: true }))
      .filter((e) => e.isDirectory() && e.name !== "report")
      .map((e) => e.name)
      .sort((a, b) => rankOp(a) - rankOp(b) || a.localeCompare(b));
    for (const op of ops) {
      const est = await readJson(`${criterion_dir}/${impl}/${op}/new/estimates.json`);
      rows.push({
        impl,
        op,
        median_ns: est.median.point_estimate,
        mean_ns: est.mean.point_estimate,
        ci_low_ns: est.median.confidence_interval.lower_bound,
        ci_high_ns: est.median.confidence_interval.upper_bound,
      });
    }
  }
  return rows;
};

// 生成汇总结果对象 (bench.js 与报告脚本的中间数据契约)
export const buildSummary = (rows) => ({
  crate: "v8-common",
  generated_at: new Date().toISOString(),
  baseline_impl: "std",
  results: withSpeedup(rows, "std"),
});

// 相对基线实现的加速比 (>1 = 更快); 基线缺失时为 null
export const withSpeedup = (rows, baseline) => {
  const base_map = new Map(rows.filter((r) => r.impl === baseline).map((r) => [r.op, r.median_ns]));
  return rows.map((r) => ({
    ...r,
    speedup_vs_baseline: base_map.has(r.op) ? base_map.get(r.op) / r.median_ns : null,
  }));
};

export const rankImpl = (name) => {
  const i = IMPL_ORDER.indexOf(name);
  return i < 0 ? IMPL_ORDER.length : i;
};

export const rankOp = (name) => {
  const i = OPS.indexOf(name);
  return i < 0 ? OPS.length : i;
};

export const fmtNs = (ns) => {
  if (ns >= 1e6) return `${(ns / 1e6).toFixed(2)} ms`;
  if (ns >= 1e3) return `${(ns / 1e3).toFixed(2)} µs`;
  return `${ns.toFixed(1)} ns`;
};

// 操作描述取自 i18n 模块 (OP_<操作名大写>)
export const opLabel = (t, op) => t[`OP_${op.toUpperCase()}`] ?? op;
