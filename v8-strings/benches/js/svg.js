#!/usr/bin/env bun
// SVG 条形图报告: 每个核心操作一个面板, 面板内按样本分组, 条长为均值纳秒 (相对 std 基线归一)。

import { SVG_OUT, i18nRead } from "./conf.js";
import { loadSummary, groupByOp, sampleOrder } from "./lib.js";

const ROW_H = 18,
  BAR_H = 12,
  LABEL_W = 320,
  CHART_W = 520,
  PANEL_W = LABEL_W + CHART_W + 80,
  COLORS = [
    ["v8rs", "#d62728"],
    ["memchr", "#2ca02c"],
    ["unicode-ident", "#1f77b4"],
    ["unicode-normalization", "#9467bd"],
    ["std", "#7f7f7f"],
  ],
  color_of = (impl) => COLORS.find(([name]) => name === impl)?.[1] ?? "#000";

const results = await loadSummary(),
  by_op = groupByOp(results);

let y = 40,
  body = "";

for (const op of by_op.keys()) {
  const rows = by_op.get(op),
    samples = sampleOrder(rows),
    base_of = (sample) => rows.find((r) => r.impl === "std" && r.sample === sample)?.ns_mean ?? 0,
    max_ratio = Math.max(
      1,
      ...rows.map((r) => (base_of(r.sample) > 0 ? r.ns_mean / base_of(r.sample) : 1)),
    ),
    title = `${op} · ${i18nRead(`OP_${op.toUpperCase()}`)}`;
  body += `<text x="10" y="${y}" font-size="15" font-weight="bold">${title}</text>`;
  y += ROW_H + 4;
  for (const sample of samples) {
    const group = rows.filter((r) => r.sample === sample),
      base = base_of(sample);
    for (const r of group) {
      const ratio = base > 0 ? r.ns_mean / base : 1,
        w = Math.max(2, Math.round((ratio / max_ratio) * CHART_W));
      body +=
        `<text x="20" y="${y + BAR_H - 2}" font-size="12">${r.impl} (${sample})</text>` +
        `<rect x="${LABEL_W}" y="${y}" width="${w}" height="${BAR_H}" fill="${color_of(r.impl)}"/>` +
        `<text x="${LABEL_W + w + 6}" y="${y + BAR_H - 2}" font-size="12">${r.ns_mean.toFixed(1)} ns</text>`;
      y += ROW_H;
    }
    y += 4;
  }
  y += 14;
}

const svg =
  `<svg xmlns="http://www.w3.org/2000/svg" width="${PANEL_W}" height="${y}" font-family="monospace">` +
  `<rect width="${PANEL_W}" height="${y}" fill="white"/>` +
  `<text x="10" y="22" font-size="16" font-weight="bold">${i18nRead("TITLE")}</text>${body}</svg>\n`;

await Bun.write(SVG_OUT, svg);
process.stdout.write(`${SVG_OUT}\n`);
