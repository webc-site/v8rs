// 评测结果 json 解析/转换公共件 (table.js / md.js / svg.js 共用)。

import { SUMMARY_JSON, IMPL_ORDER, OPS_ORDER } from "./conf.js";

// 读取 bench.js 产出的汇总结果, 按 (op, impl, sample) 排序。
export const loadSummary = async () => {
  const { results } = await Bun.file(SUMMARY_JSON).json();
  return results.sort(
    (a, b) =>
      OPS_ORDER.indexOf(a.op) - OPS_ORDER.indexOf(b.op) ||
      IMPL_ORDER.indexOf(a.impl) - IMPL_ORDER.indexOf(b.impl) ||
      a.sample.localeCompare(b.sample),
  );
};

// op → 分组后的结果 Map。
export const groupByOp = (results) => {
  const by_op = new Map();
  for (const r of results) {
    if (!by_op.has(r.op)) by_op.set(r.op, []);
    by_op.get(r.op).push(r);
  }
  return by_op;
};

// 样本顺序 (首次出现序)。
export const sampleOrder = (rows) => [...new Set(rows.map((r) => r.sample))];

// 某 op+sample 下 std 基线均值, 无基线返回 0。
export const stdBase = (rows, sample) =>
  rows.find((r) => r.impl === "std" && r.sample === sample)?.ns_mean ?? 0;

// 纳秒格式化: >=1000 转微秒。
export const fmtNs = (v) => (v >= 1000 ? `${(v / 1000).toFixed(2)} us` : `${v.toFixed(1)} ns`);

// 相对 std 倍率。
export const fmtRatio = (v, base) => (base > 0 ? `${(v / base).toFixed(2)}x` : "-");
