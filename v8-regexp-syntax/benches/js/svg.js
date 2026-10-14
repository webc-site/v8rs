#!/usr/bin/env bun
// 生成 bench/report.svg：均值对数横向条形图（跨 ns~ms 量级）

import conf, { i18nRead } from "./conf.js";
import { fmt_num, fmt_time, group_rows, read_json, with_rel, write_text } from "./lib.js";

const COLORS = ["#4c72b0", "#dd8452"];
const BAR_X = 320;
const BAR_MAX_W = 460;
const ROW_H = 26;
const GROUP_GAP = 34;

const esc = (text) => text.replace(/&/g, "&amp;").replace(/</g, "&lt;");

const text_el = (x, y, content, fill = "#222", anchor = "start", weight = "400") =>
  `<text x="${x}" y="${y}" fill="${fill}" text-anchor="${anchor}" font-size="13" font-weight="${weight}">${esc(content)}</text>`;

const gen = async () => {
  const data = await read_json(conf.bench_json);
  const strings = await i18nRead(conf.lang);
  const rows = with_rel(data.results);
  const groups = group_rows(rows, data.ops);
  const log_ns = (row) => Math.log10(row.mean_ns);
  const logs = rows.map(log_ns);
  const [log_min, log_max] = [Math.min(...logs), Math.max(...logs)];
  const span = Math.max(log_max - log_min, 1);
  const bar_w = (row) => Math.max(((log_ns(row) - log_min) / span) * BAR_MAX_W, 24);
  let y = 40;
  const parts = [];
  for (const [op, g_rows] of groups) {
    parts.push(
      text_el(10, y + 4, strings[`OP_${op.toUpperCase()}`] ?? op, "#000", "start", "bold"),
    );
    y += 22;
    for (const [i, row] of g_rows.entries()) {
      const w = bar_w(row);
      parts.push(
        `<rect x="${BAR_X}" y="${y - 13}" width="${w.toFixed(1)}" height="16" fill="${COLORS[i % COLORS.length]}" />`,
      );
      parts.push(text_el(10, y + 3, row.target));
      parts.push(text_el(BAR_X + w + 8, y + 3, `${fmt_time(row.mean_ns)}（${fmt_num(row.rel)}x）`));
      y += ROW_H;
    }
    y += GROUP_GAP - 22;
  }
  const svg = `<svg xmlns="http://www.w3.org/2000/svg" width="${BAR_X + BAR_MAX_W + 220}" height="${y}" font-family="system-ui,sans-serif">
<rect width="100%" height="100%" fill="#ffffff" />
${text_el(10, 22, strings.TITLE, "#000", "start", "bold")}
${parts.join("\n")}
${text_el(10, y - 6, strings.NOTE_SAMPLES, "#666")}
</svg>
`;
  await write_text(conf.svg_path, svg);
  return conf.svg_path;
};

if (import.meta.main) await gen();

export default gen;
