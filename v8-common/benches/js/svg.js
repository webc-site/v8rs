#!/usr/bin/env bun
// SVG 柱状图报告: 读取汇总结果 json, 按 操作分组 × 实现着色 绘制中位数耗时柱图,
// 生成 bench/bench.svg
import { writeFile } from "node:fs/promises";
import { join } from "node:path";

import { i18nRead, RESULT_DIR } from "./conf.js";
import { fmtNs, opLabel, readJson } from "./lib.js";

const PALETTE = ["#4C8BF5", "#9AA0A6", "#34A853", "#FB8C00", "#E91E63"];
const GEO = {
  w: 960,
  h: 540,
  pad_l: 70,
  pad_r: 20,
  pad_t: 70,
  pad_b: 90,
  bar_gap: 6,
  group_gap: 46,
};

const t = await i18nRead();
const { results } = await readJson(join(RESULT_DIR, "bench.json"));
const ops = [...new Set(results.map((r) => r.op))];
const impls = [...new Set(results.map((r) => r.impl))];
const cell = new Map(results.map((r) => [`${r.op}|${r.impl}`, r.median_ns]));
const max_ns = Math.max(...results.map((r) => r.median_ns));

const plot_w = GEO.w - GEO.pad_l - GEO.pad_r;
const plot_h = GEO.h - GEO.pad_t - GEO.pad_b;
const group_w = (plot_w - (ops.length - 1) * GEO.group_gap) / ops.length;
const bar_w = (group_w - (impls.length - 1) * GEO.bar_gap) / impls.length;
const scale = (ns) => (ns / max_ns) * plot_h;
const y = (ns) => GEO.pad_t + plot_h - scale(ns);
const color = (impl) => PALETTE[impls.indexOf(impl) % PALETTE.length];

const bars = ops.flatMap((op, gi) =>
  impls.flatMap((impl, bi) => {
    const ns = cell.get(`${op}|${impl}`);
    if (ns === undefined) return [];
    const bx = GEO.pad_l + gi * (group_w + GEO.group_gap) + bi * (bar_w + GEO.bar_gap);
    return [
      `<rect x="${bx.toFixed(1)}" y="${y(ns).toFixed(1)}" width="${bar_w.toFixed(1)}" height="${scale(ns).toFixed(1)}" fill="${color(impl)}"/>`,
      `<text x="${(bx + bar_w / 2).toFixed(1)}" y="${(y(ns) - 6).toFixed(1)}" font-size="11" text-anchor="middle">${fmtNs(ns)}</text>`,
    ];
  }),
);
const x_labels = ops.map((op, gi) => {
  const cx = GEO.pad_l + gi * (group_w + GEO.group_gap) + group_w / 2;
  return `<text x="${cx.toFixed(1)}" y="${GEO.h - GEO.pad_b + 24}" font-size="13" text-anchor="middle">${opLabel(t, op)}</text>`;
});
const legend = impls.map((impl, i) => {
  const lx = GEO.pad_l + i * 150;
  return `<g><rect x="${lx}" y="24" width="14" height="14" fill="${color(impl)}"/><text x="${lx + 20}" y="36" font-size="13">${impl}</text></g>`;
});

const svg = `<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="${GEO.w}" height="${GEO.h}" viewBox="0 0 ${GEO.w} ${GEO.h}" font-family="sans-serif">
<rect width="${GEO.w}" height="${GEO.h}" fill="#ffffff"/>
<text x="${GEO.w / 2}" y="56" font-size="16" text-anchor="middle">${t.SVG_TITLE}</text>
<line x1="${GEO.pad_l}" y1="${GEO.pad_t + plot_h}" x2="${GEO.pad_l + plot_w}" y2="${GEO.pad_t + plot_h}" stroke="#333"/>
<line x1="${GEO.pad_l}" y1="${GEO.pad_t}" x2="${GEO.pad_l}" y2="${GEO.pad_t + plot_h}" stroke="#333"/>
<text x="18" y="${GEO.pad_t + plot_h / 2}" font-size="12" transform="rotate(-90 18 ${GEO.pad_t + plot_h / 2})" text-anchor="middle">${t.SVG_YLABEL}</text>
<text x="${GEO.w - GEO.pad_r}" y="${GEO.pad_t - 8}" font-size="11" text-anchor="end">${fmtNs(max_ns)}</text>
${legend.join("\n")}
${bars.join("\n")}
${x_labels.join("\n")}
</svg>
`;
await writeFile(join(RESULT_DIR, "bench.svg"), svg);
