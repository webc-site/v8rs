#!/usr/bin/env bun
// criterion 输出 json 解析/转换公共件: 扫描 target/criterion 下 benchmark.json + estimates.json, 汇总为行数组, 并提供格式化/读写工具

import { mkdirSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { dirname } from "node:path";

const OPS = ["scan_tokens", "scan_regexp", "lookahead", "literal_fill"];

const walk = (dir) => {
  const files = [];
  for (const e of readdirSync(dir, { withFileTypes: true })) {
    const p = `${dir}/${e.name}`;
    if (e.isDirectory()) files.push(...walk(p));
    else if (e.name === "benchmark.json") files.push(p);
  }
  return files;
};

export const read_json = (p) => JSON.parse(readFileSync(p, "utf8"));

export const collect = (criterion_dir) => {
  const rows = walk(criterion_dir)
    .filter((p) => p.includes("/new/"))
    .map((p) => {
      const meta = read_json(p);
      const est = read_json(`${p.slice(0, -"benchmark.json".length)}estimates.json`);
      const [impl, op, ...tail] = String(meta.full_id).split("/");
      const thrpt = meta.throughput ?? {};
      return {
        impl,
        op,
        sample: meta.function_id ?? tail[0] ?? "",
        param: meta.value_str ?? "",
        bytes: thrpt.Bytes ?? 0,
        elements: thrpt.Elements ?? 0,
        mean_ns: est.mean.point_estimate,
        median_ns: est.median.point_estimate,
        std_dev_ns: est.std_dev.point_estimate,
      };
    });
  rows.sort(
    (a, b) =>
      OPS.indexOf(a.op) - OPS.indexOf(b.op) ||
      String(a.sample).localeCompare(String(b.sample)) ||
      String(a.param).localeCompare(String(b.param)) ||
      String(a.impl).localeCompare(String(b.impl)),
  );
  return rows;
};

export const write_json = (p, rows) => {
  mkdirSync(dirname(p), { recursive: true });
  writeFileSync(
    p,
    `${JSON.stringify({ generated: new Date().toISOString(), ops: OPS, rows }, null, 2)}\n`,
  );
};

export const fmt_ns = (ns) => {
  if (ns < 1e3) return `${ns.toFixed(1)} ns`;
  if (ns < 1e6) return `${(ns / 1e3).toFixed(2)} µs`;
  return `${(ns / 1e6).toFixed(2)} ms`;
};

export const fmt_size = (bytes) => {
  if (!bytes) return "-";
  const units = ["B", "KiB", "MiB"];
  let v = bytes,
    i = 0;
  while (v >= 1024 && i < 2) {
    v /= 1024;
    i += 1;
  }
  return `${i ? v.toFixed(2) : v} ${units[i]}`;
};

export const fmt_thrpt = (r) => {
  const sec = r.mean_ns / 1e9;
  if (r.bytes) return `${(r.bytes / sec / 1048576).toFixed(1)} MiB/s`;
  if (r.elements) return `${(r.elements / sec / 1e6).toFixed(0)} Melem/s`;
  return "-";
};

export const group_key = (r) => `${r.op}|${r.sample}|${r.param}`;

export const group_rows = (rows) => {
  const groups = [];
  const index = new Map();
  for (const r of rows) {
    const k = group_key(r);
    if (!index.has(k)) {
      index.set(k, groups.length);
      groups.push([]);
    }
    groups[index.get(k)].push(r);
  }
  return groups;
};

// 每个 v8 行附带语义成本 ratio (v8 平均耗时 / naive 平均耗时)
export const build_groups = (rows) =>
  group_rows(rows).map((g) => {
    const naive = g.find((r) => r.impl === "naive-tokenizer");
    return g.map((r) => ({
      ...r,
      ratio: naive && r.impl === "v8-scanner" ? r.mean_ns / naive.mean_ns : null,
    }));
  });

export const size_text = (r) =>
  r.bytes ? fmt_size(r.bytes) : r.elements ? `${r.elements} elem` : "-";

export default {
  collect,
  read_json,
  write_json,
  fmt_ns,
  fmt_size,
  fmt_thrpt,
  group_key,
  group_rows,
  build_groups,
  size_text,
};
