#!/usr/bin/env bun
// 目的: criterion 输出 json 解析 / bench-result.json 转换的公共件 (table.js md.js svg.js 与 bench.js 共用)
import { fs, path } from "zx";

// 评测 id 结构字段顺序
export const K_OPS = ["add", "sub", "mul", "div", "rem", "to_string", "from_string"];
export const K_STRING_OPS = ["to_string", "from_string"];

// "{impl}.{op}[.{radix}].{bits}" → {impl, op, radix, bits}
export const break_id = (id) => {
  const parts = id.split(".");
  const bits = Number(parts.pop());
  const tail = parts.pop();
  // 字符串操作 id 为 {impl}.{op}.{radix}.{bits}, 末段弹出的是 radix
  const [radix, op] = Number.isNaN(Number(tail)) ? [null, tail] : [Number(tail), parts.pop()];
  return { impl: parts.join("."), op, radix, bits };
};

// 扫描 <target>/criterion/<id>/new/{estimates,benchmark}.json → 汇总行数组
export const collect_estimates = async (criterion_dir) => {
  const dirs = (await fs.readdir(criterion_dir, { withFileTypes: true })).filter((d) =>
    d.isDirectory(),
  );
  const rows = await Promise.all(
    dirs.map(async (dir) => {
      const est_path = path.join(criterion_dir, dir.name, "new", "estimates.json");
      if (!(await fs.pathExists(est_path))) return null;
      const est = await fs.readJson(est_path);
      const meta = await fs.readJson(path.join(criterion_dir, dir.name, "new", "benchmark.json"));
      const id = meta.full_id ?? dir.name;
      const ns = (v) => Math.round(v * 1e3) / 1e3;
      return {
        id,
        ...break_id(id),
        mean_ns: ns(est.mean.point_estimate),
        ci_low_ns: ns(est.mean.confidence_interval.lower_bound),
        ci_high_ns: ns(est.mean.confidence_interval.upper_bound),
      };
    }),
  );
  return rows
    .filter((r) => r)
    .sort(
      (a, b) =>
        K_OPS.indexOf(a.op) - K_OPS.indexOf(b.op) ||
        a.radix - b.radix ||
        a.bits - b.bits ||
        a.impl.localeCompare(b.impl),
    );
};

// 行按 (op, radix, bits) 归组, 各实现并列并计算对比倍率 (对比库 / 基准库, >1 表示基准更快)
export const group_rows = (rows, base_impl = "v8rs") => {
  const groups = new Map();
  for (const row of rows) {
    const key = `${row.op}.${row.radix ?? ""}.${row.bits}`;
    const g = groups.get(key) ?? {
      op: row.op,
      radix: row.radix,
      bits: row.bits,
      times: {},
      ratio: null,
    };
    g.times[row.impl] = row.mean_ns;
    groups.set(key, g);
  }
  return [...groups.values()].map((g) => {
    const others = Object.entries(g.times).filter(([impl]) => impl !== base_impl);
    const ratio =
      g.times[base_impl] && others.length
        ? Math.round((others[0][1] / g.times[base_impl]) * 100) / 100
        : null;
    return { ...g, ratio };
  });
};

// ns → 可读时间串
export const fmt_time = (ns) =>
  ns >= 1e9
    ? `${Math.round(ns / 1e6)} ms`
    : ns >= 1e6
      ? `${Math.round(ns / 1e3)} µs`
      : ns >= 1e3
        ? `${Math.round(ns / 100) / 10} µs`
        : `${Math.round(ns * 100) / 100} ns`;

// markdown 表格渲染 (纯函数)
export const md_table = (headers, lines) =>
  `${[headers, headers.map(() => "---"), ...lines].map((cols) => `| ${cols.join(" | ")} |`).join("\n")}\n`;

// 位宽标签 (64 / 256 / 1k / 4k / 16k / 64k)
export const bits_label = (bits) => (bits >= 1024 ? `${bits / 1024}k` : `${bits}`);

// 汇总表数据: 基准库 + 对比库并列, 附倍率
export const summary = (rows, base_impl = "v8rs") => {
  const peers = [...new Set(rows.map((r) => r.impl))].filter((impl) => impl !== base_impl);
  const groups = group_rows(rows, base_impl);
  const cells = groups.map((g) => [
    g.op,
    g.radix ?? "-",
    bits_label(g.bits),
    g.times[base_impl] ? fmt_time(g.times[base_impl]) : "-",
    ...peers.map((p) => (g.times[p] ? fmt_time(g.times[p]) : "-")),
    g.ratio ?? "-",
  ]);
  return { base_impl, peers, cells };
};
