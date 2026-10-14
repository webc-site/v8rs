#!/usr/bin/env bun
// 目的: 读 bench-result.json, 生成横向条形对比图 bench-chart.svg (对数刻度)
// 布局: 每个 (op, radix) 一个面板; 面板内每个位宽一组双色条, 颜色对应实现库 (见图例)
import { echo, fs, path } from "zx";
import { crate_dir, i18n_read, result_json } from "./conf.js";
import { bits_label, fmt_time, group_rows } from "./lib.js";

const K_W = 880;
const K_LABEL_W = 90;
const K_BAR_H = 11;
const K_BITS_GAP = 8;
const K_PANEL_GAP = 30;
const K_PLOT_W = K_W - K_LABEL_W - 250;
const K_COLORS = ["#2f81f7", "#e36f1e", "#3fb950", "#a371f7"];

// 按 (op, radix) 聚合位宽分组
const panels_of = (groups) => {
  const panels = new Map();
  for (const g of groups) {
    const key = `${g.op}.${g.radix ?? ""}`;
    panels.set(key, [...(panels.get(key) ?? []), g]);
  }
  return [...panels.values()];
};

// 单条: 对数刻度宽度 (10ns 起底)
const bar_w = (ns, max_ns) =>
  Math.max(
    2,
    Math.round(((Math.log10(Math.max(ns, 10)) - 1) / (Math.log10(max_ns) - 1)) * K_PLOT_W),
  );

const bar = (ns, max_ns, color, y) =>
  `<rect x="${K_LABEL_W}" y="${y}" width="${bar_w(ns, max_ns)}" height="${K_BAR_H}" fill="${color}"/><text x="${K_LABEL_W + bar_w(ns, max_ns) + 6}" y="${y + K_BAR_H - 2}" font-size="11" fill="#222">${fmt_time(ns)}</text>`;

// 面板标题: op + radix
const panel_name = (bits_rows) => {
  const { op, radix } = bits_rows[0];
  return radix ? `${op} · radix ${radix}` : op;
};

// 一个位宽组: 左侧位宽标签 + 各实现双色条
const bits_block = (g, max_ns, impls, colors, y) =>
  `<text x="4" y="${y + K_BAR_H + 2}" font-size="12" fill="#444">${bits_label(g.bits)}</text>` +
  impls
    .map((impl, idx) => bar(g.times[impl] ?? 0, max_ns, colors[idx], y + idx * (K_BAR_H + 2)))
    .join("");

const run = async () => {
  const [i18n, rows] = [await i18n_read(), await fs.readJson(result_json)];
  const impls = [...new Set(rows.map((r) => r.impl))];
  const colors = K_COLORS.slice(0, impls.length);
  const max_ns = Math.max(...rows.map((r) => r.mean_ns));
  const panels = panels_of(group_rows(rows));
  const block_h = impls.length * (K_BAR_H + 2) + K_BITS_GAP;
  const legend = impls
    .map(
      (impl, idx) =>
        `<rect x="${K_LABEL_W + idx * 190}" y="8" width="12" height="12" fill="${colors[idx]}"/><text x="${K_LABEL_W + idx * 190 + 17}" y="19" font-size="13" fill="#222">${impl}</text>`,
    )
    .join("");
  let y = 40;
  const parts = panels.map((bits_rows) => {
    const title = `<text x="0" y="${y + 12}" font-size="14" font-weight="bold" fill="#111">${panel_name(bits_rows)}</text>`;
    y += K_PANEL_GAP;
    const blocks = bits_rows.map((g) => {
      const b = bits_block(g, max_ns, impls, colors, y);
      y += block_h;
      return b;
    });
    return `${title}${blocks.join("")}`;
  });
  const svg = `<svg xmlns="http://www.w3.org/2000/svg" width="${K_W}" height="${y + 16}" font-family="ui-sans-serif, system-ui, sans-serif"><rect width="${K_W}" height="${y + 16}" fill="#fff"/><text x="0" y="22" font-size="16" font-weight="bold" fill="#111">${i18n.TITLE}</text>${legend}${parts.join("")}</svg>\n`;
  await fs.outputFile(path.join(crate_dir, "bench-chart.svg"), svg);
  echo(`${path.join(crate_dir, "bench-chart.svg")} (${panels.length} 面板)`);
};

export default await run();
