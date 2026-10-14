#!/usr/bin/env bun
// svg.js — 读取 bench.js 生成的汇总 json, 生成各操作平均耗时横向条形对比图 bench-report.svg

import { writeFile } from "node:fs/promises";
import { groupByOp, loadSummary, fmtNs, fmtRatio } from "./lib.js";
import { BASELINE_GROUP, RESULT_JSON, SVG_OUT, i18nRead } from "./conf.js";

const summary = await loadSummary(RESULT_JSON),
  ops = groupByOp(summary.benches, BASELINE_GROUP),
  groups = [...new Set(summary.benches.map((b) => b.group))].sort(),
  palette = ["#2e7d32", "#1565c0", "#ef6c00", "#6a1b9a", "#455a64"],
  label_w = 330,
  area_w = 520,
  bar_h = 18,
  row_gap = 6,
  group_gap = 26,
  top_h = 64,
  bottom_h = 48,
  max_ns = Math.max(...summary.benches.map((b) => b.mean_ns));

const width = label_w + area_w + 60,
  height =
    top_h +
    ops.reduce((acc, op) => acc + op.rows.length * (bar_h + row_gap) + group_gap, 0) +
    bottom_h,
  parts = [];

const push = (line) => parts.push(line);

push(
  `<svg xmlns="http://www.w3.org/2000/svg" width="${width}" height="${height}" viewBox="0 0 ${width} ${height}" font-family="ui-sans-serif, system-ui, sans-serif" font-size="12">`,
);
push(`<rect width="100%" height="100%" fill="white"/>`);
push(`<text x="20" y="28" font-size="16" font-weight="bold">${i18nRead("SVG_TITLE")}</text>`);

for (let i = 0; i <= 4; i++) {
  const tick_ns = (max_ns * i) / 4,
    x = label_w + (area_w * i) / 4;
  push(
    `<line x1="${x.toFixed(1)}" y1="${top_h - 10}" x2="${x.toFixed(1)}" y2="${height - bottom_h + 4}" stroke="#e0e0e0"/>`,
  );
  push(
    `<text x="${x.toFixed(1)}" y="${height - bottom_h + 20}" text-anchor="middle" fill="#555">${fmtNs(tick_ns)}</text>`,
  );
}
push(
  `<text x="${(label_w + area_w / 2).toFixed(1)}" y="${height - 10}" text-anchor="middle" fill="#333">${i18nRead("AXIS_LABEL")}</text>`,
);

let y = top_h;
for (const { op, rows, ratios } of ops) {
  rows.forEach((r, i) => {
    const color = palette[groups.indexOf(r.group) % palette.length],
      bar_w = Math.max(2, (r.mean_ns / max_ns) * area_w);
    push(
      `<text x="${label_w - 10}" y="${y + bar_h - 4}" text-anchor="end" fill="#222">${op} · ${r.group}</text>`,
    );
    push(
      `<rect x="${label_w}" y="${y}" width="${bar_w.toFixed(1)}" height="${bar_h}" fill="${color}" rx="2"/>`,
    );
    push(
      `<text x="${(label_w + bar_w + 8).toFixed(1)}" y="${y + bar_h - 4}" fill="#333">${fmtNs(r.mean_ns)} (${fmtRatio(ratios[i])})</text>`,
    );
    y += bar_h + row_gap;
  });
  y += group_gap;
}

push("</svg>");

await writeFile(SVG_OUT, `${parts.join("\n")}\n`);
console.info(`${SVG_OUT}`);
