#! /usr/bin/env bun
// SVG 柱状对比图 (genbench 步骤 9): 读 bench.json, 每个 (op, param) 一组横向条, 输出 report/chart.svg

import { mkdir, writeFile } from "node:fs/promises";
import { DEFAULT_LANG, REPORT_DIR, i18n_read } from "./conf.js";
import { fmt_ratio, fmt_time, read_bench, to_groups } from "./lib.js";

const LANG = await i18n_read(DEFAULT_LANG);

const COLORS = ["#2e8b57", "#9aa5b1", "#c9a227", "#5b8db8"];
const LABEL_W = 250;
const BAR_H = 22;
const BAR_GAP = 6;
const GROUP_GAP = 20;
const PLOT_W = 560;
const TOP_H = 56;

const { results } = await read_bench();
const { groups, gain } = to_groups(results);

let cursor_y = TOP_H;
const labels = [];
const bars = [];
for (const g of groups) {
  const max = Math.max(...g.items.map((i) => i.mean_ns));
  labels.push(
    `<text x="${LABEL_W - 10}" y="${cursor_y + BAR_H - 6}" class="label">${g.param ? `${g.op} · ${g.param}` : g.op}</text>`,
  );
  for (const [idx, i] of g.items.entries()) {
    const w = Math.max(3, (i.mean_ns / max) * PLOT_W);
    const note = `${i.impl} — ${fmt_time(i.mean_ns)}${i.vs_fastest === 1 ? "" : ` · ${fmt_ratio(i.vs_fastest)}`}`;
    bars.push(`<rect x="${LABEL_W}" y="${cursor_y}" width="${w.toFixed(1)}" height="${BAR_H}" fill="${COLORS[idx % COLORS.length]}" rx="3"/>
<text x="${(LABEL_W + w + 10).toFixed(1)}" y="${cursor_y + BAR_H - 6}" class="val">${note}</text>`);
    cursor_y += BAR_H + BAR_GAP;
  }
  cursor_y += GROUP_GAP;
}

const total_h = cursor_y + 36;
const svg = `<svg xmlns="http://www.w3.org/2000/svg" width="${LABEL_W + PLOT_W + 260}" height="${total_h}" font-family="system-ui, sans-serif">
<style>
.title { font-size: 18px; font-weight: 600; }
.sub { font-size: 12px; fill: #555; }
.label { font-size: 13px; fill: #333; text-anchor: end; }
.val { font-size: 12px; fill: #333; }
</style>
<text x="16" y="26" class="title">${LANG.CHART_TITLE}</text>
<text x="16" y="44" class="sub">${LANG.GAIN_TITLE}: ${LANG.GAIN_SUMMARY} ${fmt_ratio(gain)} ${LANG.GAIN_SUMMARY_SUFFIX}</text>
${labels.join("\n")}
${bars.join("\n")}
<text x="16" y="${total_h - 10}" class="sub">${LANG.UNIT_NOTE}</text>
</svg>
`;

await mkdir(REPORT_DIR, { recursive: true });
await writeFile(`${REPORT_DIR}/chart.svg`, svg);
console.log(`${REPORT_DIR}/chart.svg`);
