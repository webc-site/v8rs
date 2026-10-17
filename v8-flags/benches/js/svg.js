#!/usr/bin/env bun
// benches/js/svg.js — 生成 SVG 柱状图: 每操作平均耗时 (对数纵轴), 多实现并列 + 图例

import { fmtNs, measureOf, readJson, writeText } from "./lib.js";
import { i18nRead, result_json, svg_chart } from "./conf.js";

const PALETTE = ["#4c78a8", "#f58518", "#54a24b", "#e45756"];
const M_LEFT = 100;
const M_RIGHT = 30;
const M_TOP = 64;
const M_BOTTOM = 190;
const PLOT_W = 1500;
const PLOT_H = 520;

const runSvg = async () => {
  const result = await readJson(result_json);
  const i18n = await i18nRead();
  const { groups, impls } = result;
  const values = groups.flatMap(group =>
    impls.map(impl => measureOf(result, group, impl)?.mean_ns).filter(value => value),
  );
  const log_min = Math.log10(Math.min(...values)) - 0.35;
  const log_max = Math.log10(Math.max(...values)) + 0.25;
  const ratioOf = value => (Math.log10(value) - log_min) / (log_max - log_min);
  const slot = PLOT_W / groups.length;
  const bar_w = (slot * 0.74) / impls.length;
  const bottom = M_TOP + PLOT_H;
  const width = M_LEFT + PLOT_W + M_RIGHT;
  const height = bottom + M_BOTTOM;

  const bars = groups.flatMap((group, gi) => [
    ...impls.flatMap((impl, ii) => {
      const value = measureOf(result, group, impl)?.mean_ns;
      if (!value) return [];
      const bar_h = Math.max(ratioOf(value) * PLOT_H, 2);
      const x = M_LEFT + gi * slot + slot * 0.13 + ii * bar_w;
      const y = bottom - bar_h;
      return [
        `<rect x="${x.toFixed(1)}" y="${y.toFixed(1)}" width="${(bar_w - 6).toFixed(1)}" height="${bar_h.toFixed(1)}" fill="${PALETTE[ii % PALETTE.length]}"/>`,
        `<text x="${(x + (bar_w - 6) / 2).toFixed(1)}" y="${(y - 6).toFixed(1)}" font-size="13" text-anchor="middle" fill="#333">${fmtNs(value)}</text>`,
      ].join("\n    ");
    }),
    (() => {
      const lx = M_LEFT + gi * slot + slot / 2;
      return `<text x="${lx.toFixed(1)}" y="${bottom + 18}" font-size="15" text-anchor="end" transform="rotate(-45 ${lx.toFixed(1)} ${bottom + 18})" fill="#333">${group}</text>`;
    })(),
  ]);

  const grid = [];
  for (let p = Math.ceil(log_min); p <= Math.floor(log_max); p += 1) {
    const value = 10 ** p;
    const y = bottom - ratioOf(value) * PLOT_H;
    grid.push(
      `<line x1="${M_LEFT}" y1="${y.toFixed(1)}" x2="${M_LEFT + PLOT_W}" y2="${y.toFixed(1)}" stroke="#dddddd"/>`,
      `<text x="${M_LEFT - 10}" y="${(y + 4).toFixed(1)}" font-size="12" text-anchor="end" fill="#666666">${fmtNs(value)}</text>`,
    );
  }

  const legend = impls.map((impl, ii) => {
    const x = M_LEFT + ii * 190;
    return `<rect x="${x}" y="${M_TOP - 36}" width="16" height="16" fill="${PALETTE[ii % PALETTE.length]}"/><text x="${x + 24}" y="${M_TOP - 23}" font-size="15" fill="#333333">${impl}</text>`;
  });

  const svg = `<svg xmlns="http://www.w3.org/2000/svg" width="${width}" height="${height}" viewBox="0 0 ${width} ${height}">
  <rect width="${width}" height="${height}" fill="#ffffff"/>
  <text x="${M_LEFT}" y="28" font-size="20" fill="#222222">${i18n.REPORT_TITLE} — mean (log scale)</text>
  <line x1="${M_LEFT}" y1="${bottom}" x2="${M_LEFT + PLOT_W}" y2="${bottom}" stroke="#999999"/>
  ${legend.join("\n  ")}
  ${grid.join("\n  ")}
  ${bars.join("\n  ")}
</svg>
`;
  await writeText(svg_chart, svg);
  return svg_chart;
};

export default runSvg;

if (import.meta.main) await runSvg();
