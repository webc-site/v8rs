// lib.js — benches/js 公共件: 汇总 json 解析/转换与表格渲染
import { readFile } from "node:fs/promises";

export const loadJson = async (file_path) => JSON.parse(await readFile(file_path, "utf8"));

export const loadSummary = async (json_path) => await loadJson(json_path);

// 按评测操作分组, ratios 相对 baseline_group 的 mean_ns 倍率 (越大表示越慢)
export const groupByOp = (benches, baseline_group) => {
  const ops = [...new Set(benches.map((b) => b.bench))].sort();
  return ops.map((op) => {
    const rows = benches
      .filter((b) => b.bench === op)
      .sort((a, b) => a.group.localeCompare(b.group));
    const base = rows.find((r) => r.group === baseline_group) ?? rows[0];
    return { op, rows, ratios: rows.map((r) => r.mean_ns / base.mean_ns) };
  });
};

export const fmtNs = (ns) =>
  ns >= 1e6
    ? `${(ns / 1e6).toFixed(3)} ms`
    : ns >= 1e3
      ? `${(ns / 1e3).toFixed(2)} µs`
      : `${ns.toFixed(1)} ns`;

export const fmtRatio = (ratio) => `${ratio.toFixed(2)}x`;

// 等宽文本表格 (东亚宽字符按 2 列计算显示宽度)
const displayWidth = (text) =>
  [...String(text)].reduce((acc, ch) => acc + (ch.codePointAt() > 0x2e7f ? 2 : 1), 0);

export const renderTable = (headers, rows) => {
  const widths = headers.map((h, i) =>
    Math.max(displayWidth(h), ...rows.map((r) => displayWidth(r[i]))),
  );
  const line = (cells) =>
    cells
      .map((c, i) => String(c) + " ".repeat(Math.max(0, widths[i] - displayWidth(c))))
      .join("  ")
      .trimEnd();
  const sep = widths.map((w) => "─".repeat(w)).join("──");
  return [line(headers), sep, ...rows.map((r) => line(r))].join("\n");
};

// markdown 表格
export const mdTable = (headers, rows) => {
  const head = `| ${headers.join(" | ")} |`;
  const split = `| ${headers.map(() => "---").join(" | ")} |`;
  return [head, split, ...rows.map((r) => `| ${r.join(" | ")} |`)].join("\n");
};

export default { loadJson, loadSummary, groupByOp, fmtNs, fmtRatio, renderTable, mdTable };
