#!/usr/bin/env bun
// 读取 bench/result.json, 生成 SVG 条形图 bench/chart.svg (平均耗时, 每组内按组内最大值归一)

import { writeFileSync } from "node:fs";
import { BENCH_DIR, RESULT_PATH, i18nRead } from "./conf.js";
import { build_groups, fmt_ns, read_json } from "./lib.js";

const i18n = await i18nRead();

const data = read_json(RESULT_PATH);
const groups = build_groups(data.rows);

const V8_COLOR = "#4c82fb";
const NAIVE_COLOR = "#f0a020";
const TEXT_COLOR = "#333333";
const W = 920;
const PAD = 16;
const TITLE_H = 30;
const HEAD_H = 20;
const BAR_H = 20;
const BAR_GAP = 6;
const GROUP_PAD = 16;
const LABEL_X = 190;
const BAR_X = 200;
const BAR_MAX = W - BAR_X - 130;

const group_h = () => HEAD_H + 2 * (BAR_H + BAR_GAP) + GROUP_PAD;
const H = PAD * 2 + TITLE_H + groups.length * group_h();

const group_title = (g) => {
  const r = g[0];
  return `${r.op} · ${r.sample}${r.param ? ` · ${r.param}` : ""}`;
};

const bar_row = (r, y, max) => {
  const w = Math.max(2, (r.mean_ns / max) * BAR_MAX);
  const color = r.impl === "v8-scanner" ? V8_COLOR : NAIVE_COLOR;
  const ratio = r.ratio === null ? "" : `  (${r.ratio.toFixed(2)}×)`;
  return [
    `<text x="${LABEL_X}" y="${y + 14}" text-anchor="end" font-size="12" fill="${TEXT_COLOR}">${r.impl}</text>`,
    `<rect x="${BAR_X}" y="${y}" width="${w.toFixed(1)}" height="${BAR_H}" rx="3" fill="${color}"/>`,
    `<text x="${(BAR_X + w + 6).toFixed(1)}" y="${y + 14}" font-size="12" fill="${TEXT_COLOR}">${fmt_ns(r.mean_ns)}${ratio}</text>`,
  ].join("\n");
};

const panel = (g, y0) => {
  const max = Math.max(...g.map((r) => r.mean_ns));
  return [
    `<text x="${PAD}" y="${y0 + 14}" font-size="13" font-weight="bold" fill="${TEXT_COLOR}">${group_title(g)}</text>`,
    ...g.map((r, i) => bar_row(r, y0 + HEAD_H + i * (BAR_H + BAR_GAP), max)),
  ].join("\n");
};

const svg = `<svg xmlns="http://www.w3.org/2000/svg" width="${W}" height="${H}" viewBox="0 0 ${W} ${H}" font-family="system-ui, -apple-system, sans-serif">
<rect width="100%" height="100%" fill="#ffffff"/>
<text x="${PAD}" y="${PAD + 18}" font-size="15" font-weight="bold" fill="${TEXT_COLOR}">${i18n.CHART_TITLE}</text>
${groups.map((g, i) => panel(g, PAD + TITLE_H + i * group_h())).join("\n")}
</svg>
`;

writeFileSync(`${BENCH_DIR}/chart.svg`, svg);
process.stdout.write(`${BENCH_DIR}/chart.svg\n`);
