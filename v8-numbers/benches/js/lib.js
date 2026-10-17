// bench.json 解析/转换公共件 — 报告脚本 (table/md/svg) 共用
import { readJson, BENCH_JSON } from "./conf.js";

export const BASE_NAME = "v8-numbers";

// ns → 人类可读耗时
export const fmtNs = (ns) => {
  if (ns == null) {
    return "-";
  }
  if (ns < 1e3) {
    return `${ns.toFixed(1)} ns`;
  }
  if (ns < 1e6) {
    return `${(ns / 1e3).toFixed(2)} µs`;
  }
  if (ns < 1e9) {
    return `${(ns / 1e6).toFixed(2)} ms`;
  }
  return `${(ns / 1e9).toFixed(2)} s`;
};

// 相对 v8-numbers 的倍速 (慢于基准为 <1, 快于基准为 >1)
export const vsBase = (ns, base_ns) => (base_ns == null || ns == null ? null : base_ns / ns);

// 汇总 json → 按操作分组的行数据
export const loadRows = async () => {
  const bench = await readJson(BENCH_JSON);
  return bench.ops
    .filter(({ results }) => results.length)
    .map(({ op, results }) => {
      const base = results.find((r) => r.name === BASE_NAME)?.ns_per_elem ?? null;
      return {
        op,
        results: results.map((r) => ({
          ...r,
          base: r.name === BASE_NAME,
          ratio: vsBase(r.ns_per_elem, base),
        })),
      };
    });
};

// 单元格文本: 单位样本耗时 + 相对基准倍速
export const cellText = (row) =>
  row.ratio == null
    ? fmtNs(row.ns_per_elem)
    : `${fmtNs(row.ns_per_elem)} (${row.ratio.toFixed(2)}x)`;

export default { BASE_NAME, fmtNs, vsBase, loadRows, cellText };
