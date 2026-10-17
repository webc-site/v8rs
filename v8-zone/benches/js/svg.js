#! /usr/bin/env bun
// 读取 bench.json 汇总结果, 生成 SVG 柱状图 report/chart.svg (吞吐相对基线倍率)

import { mkdir, writeFile } from "fs/promises";
import { join } from "path";
import { benchJsonPath, i18nRead, reportDir } from "./conf.js";
import { jsonRead } from "./lib.js";

const IMPL_COLORS = { "v8rs-zone": "#1a7f37", bumpalo: "#b07219", "std-vec": "#6f42c1" };
const BAR_W = 24;
const BAR_GAP = 5;
const GROUP_GAP = 30;
const PLOT_H = 150;
const MARGIN = 40;
const PANEL_H = PLOT_H + 64;
const opKeys = { alloc: "OP_ALLOC", aligned: "OP_ALIGNED", scoped: "OP_SCOPED", reset: "OP_RESET" };

const rect = (x, y, w, h, fill) =>
  `<rect x="${x.toFixed(1)}" y="${y.toFixed(1)}" width="${w.toFixed(1)}" height="${h.toFixed(1)}" fill="${fill}"/>`;

const text = (x, y, str, { anchor = "middle", size = 11, fill = "#222" } = {}) =>
  `<text x="${x.toFixed(1)}" y="${y.toFixed(1)}" font-size="${size}" text-anchor="${anchor}" fill="${fill}">${str}</text>`;

// 单个 op 面板: 按参数分组画倍率柱 (虚线为基线 ratio = 1)
const panel = (i18n, summary, op) => {
  const rows = summary.benchmarks.filter((b) => b.op === op);
  const params = [...new Set(rows.map((b) => b.param))];
  const group_w = summary.impls.length * (BAR_W + BAR_GAP) + GROUP_GAP;
  const plot_w = params.length * group_w;
  const max_ratio = Math.max(1.2, ...rows.map((r) => r.ratio ?? 0));
  const scale = (v) => (PLOT_H * v) / max_ratio;
  const base_y = PLOT_H + 24;
  const parts = [text(0, 14, i18n[opKeys[op]], { anchor: "start", size: 13 })];
  parts.push(
    `<line x1="0" y1="${(base_y - scale(1)).toFixed(1)}" x2="${plot_w}" y2="${(base_y - scale(1)).toFixed(1)}" stroke="#999" stroke-dasharray="4 3"/>`,
  );
  params.forEach((param, gi) => {
    summary.impls.forEach((impl, ii) => {
      const rec = rows.find((b) => b.param === param && b.impl === impl);
      if (!rec) return;
      const x = gi * group_w + ii * (BAR_W + BAR_GAP);
      const h = Math.max(2, scale(rec.ratio));
      parts.push(rect(x, base_y - h, BAR_W, h, IMPL_COLORS[impl]));
      parts.push(text(x + BAR_W / 2, base_y - h - 4, rec.ratio.toFixed(2), { size: 9 }));
    });
    parts.push(
      text(gi * group_w + (group_w - GROUP_GAP) / 2, base_y + 18, param, { size: 9, fill: "#555" }),
    );
  });
  parts.push(`<line x1="0" y1="${base_y}" x2="${plot_w}" y2="${base_y}" stroke="#222"/>`);
  return { parts, width: plot_w };
};

const render = async () => {
  const summary = await jsonRead(benchJsonPath);
  const i18n = await i18nRead();
  const panels = summary.ops.map((op) => panel(i18n, summary, op));
  const chart_w = Math.max(660, ...panels.map((p) => MARGIN + p.width + MARGIN));
  const legend = summary.impls
    .map((impl, i) => {
      const x = chart_w - 540 + i * 175;
      return `${rect(x, 8, 12, 12, IMPL_COLORS[impl])}${text(x + 17, 19, impl, { anchor: "start", size: 11 })}`;
    })
    .join("\n");
  let cy = 52;
  const groups = panels.map((p) => {
    const g = `<g transform="translate(${MARGIN} ${cy.toFixed(1)})">${p.parts.join("")}</g>`;
    cy += PANEL_H;
    return g;
  });
  return `<svg xmlns="http://www.w3.org/2000/svg" width="${chart_w}" height="${cy}" font-family="monospace">
<rect width="100%" height="100%" fill="#fff"/>
${text(MARGIN, 20, i18n.SVG_TITLE, { anchor: "start", size: 14 })}
${legend}
${groups.join("\n")}
</svg>
`;
};

const svg = await render();
await mkdir(reportDir, { recursive: true });
await writeFile(join(reportDir, "chart.svg"), svg);
await writeFile(1, `${svg.length} chars -> report/chart.svg\n`);
