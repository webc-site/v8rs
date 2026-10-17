#!/usr/bin/env bun
// json 解析/转换公共件：criterion 原始输出收集、单位格式化、相对值计算

import { mkdir, readdir, readFile, writeFile } from "node:fs/promises";
import { dirname, join } from "node:path";

const read_json = async (path) => JSON.parse(await readFile(path, "utf8"));

const write_text = async (path, text) => {
  await mkdir(dirname(path), { recursive: true });
  await writeFile(path, text, "utf8");
};

// 扫描 criterion/<操作>/<目标>/<基线>/estimates.json 收集为结果行数组
const collect_estimates = async (criterion_dir, baseline) => {
  const rows = [];
  for (const op_entry of await readdir(criterion_dir, { withFileTypes: true })) {
    if (!op_entry.isDirectory()) continue;
    const op = op_entry.name;
    for (const target_entry of await readdir(join(criterion_dir, op), { withFileTypes: true })) {
      if (!target_entry.isDirectory()) continue;
      const target = target_entry.name;
      const bench_dir = join(criterion_dir, op, target, baseline);
      const subs = await readdir(join(criterion_dir, op, target));
      if (!subs.includes(baseline)) continue;
      const est = await read_json(join(bench_dir, "estimates.json"));
      rows.push({
        op,
        target,
        mean_ns: est.mean.point_estimate,
        median_ns: est.median.point_estimate,
        slope_ns: est.slope?.point_estimate ?? null,
        ci_low_ns: est.mean.confidence_interval.lower_bound,
        ci_high_ns: est.mean.confidence_interval.upper_bound,
      });
    }
  }
  return rows;
};

// 纳秒转可读单位（null 显示占位线）
const fmt_time = (ns) => {
  if (ns === null) return "—";
  const units = [
    ["s", 1e9],
    ["ms", 1e6],
    ["µs", 1e3],
    ["ns", 1],
  ];
  const [unit, scale] = units.find(([, s]) => ns >= s);
  return `${(ns / scale).toFixed(3)} ${unit}`;
};

const fmt_num = (value) => (value >= 100 ? value.toFixed(0) : value.toFixed(2));

// 同操作内相对最快实现的倍率
const with_rel = (rows) => {
  const best_by_op = new Map();
  for (const row of rows) {
    const best = best_by_op.get(row.op);
    if (best === undefined || row.mean_ns < best) best_by_op.set(row.op, row.mean_ns);
  }
  return rows.map((row) => ({ ...row, rel: row.mean_ns / best_by_op.get(row.op) }));
};

// 按操作分组（保持 OPS 顺序，未知操作排最后）
const group_rows = (rows, ops) => {
  const sorted = [...rows].sort((a, b) => {
    const ia = ops.indexOf(a.op);
    const ib = ops.indexOf(b.op);
    return (
      (ia < 0 ? 99 : ia) - (ib < 0 ? 99 : ib) ||
      a.op.localeCompare(b.op) ||
      a.target.localeCompare(b.target)
    );
  });
  const groups = new Map();
  for (const row of sorted) {
    groups.set(row.op, [...(groups.get(row.op) ?? []), row]);
  }
  return groups;
};

export { read_json, write_text, collect_estimates, fmt_time, fmt_num, with_rel, group_rows };
export default {
  read_json,
  write_text,
  collect_estimates,
  fmt_time,
  fmt_num,
  with_rel,
  group_rows,
};
