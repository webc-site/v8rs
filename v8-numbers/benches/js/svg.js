#!/usr/bin/env bun
// SVG 条形图报告 — 读取 bench.json, 生成 crate 根 bench.svg
import { writeFile } from "node:fs/promises";
import path from "node:path";
import { CRATE_DIR, i18nRead, pickLang } from "./conf.js";
import { fmtNs, loadRows } from "./lib.js";

const ROW_H = 22;
const OP_PAD = 34;
const LABEL_W = 150;
const VAL_W = 220;
const BAR_MAX = 520;
const WIDTH = LABEL_W + BAR_MAX + VAL_W + 40;

const COLORS = ["#3b82f6", "#9ca3af", "#f59e0b", "#10b981", "#ef4444"];

const esc = (s) => s.replaceAll("&", "&amp;").replaceAll("<", "&lt;").replaceAll(">", "&gt;");

const barRow = (y, name, ns, ratio, color) => {
  const w = Math.max(2, (ns / ratio.max) * BAR_MAX);
  return `
  <text x="${LABEL_W - 8}" y="${y + 15}" text-anchor="end" font-size="13">${esc(name)}</text>
  <rect x="${LABEL_W}" y="${y + 4}" width="${w.toFixed(1)}" height="14" fill="${color}" rx="2"/>
  <text x="${LABEL_W + w + 8}" y="${y + 15}" font-size="12">${fmtNs(ns)} · ${ratio.fixed}x</text>`;
};

const main = async () => {
  const i18n = await i18nRead(pickLang());
  const rows = await loadRows();
  let y = 46;
  const parts = [];
  for (const { op, results } of rows) {
    const max = Math.max(...results.map((r) => r.ns_per_elem));
    const base = results.find((r) => r.base)?.ns_per_elem ?? max;
    parts.push(`
  <text x="16" y="${y}" font-size="15" font-weight="bold">${esc(op)}</text>`);
    y += OP_PAD - 14;
    results.forEach((r, i) => {
      parts.push(
        barRow(
          y,
          r.name,
          r.ns_per_elem,
          { max, fixed: (base / r.ns_per_elem).toFixed(2) },
          COLORS[i % COLORS.length],
        ),
      );
      y += ROW_H;
    });
    y += 14;
  }
  const svg = `<svg xmlns="http://www.w3.org/2000/svg" width="${WIDTH}" height="${y + 20}" font-family="monospace">
  <rect width="100%" height="100%" fill="#ffffff"/>
  <text x="16" y="26" font-size="17" font-weight="bold">${esc(i18n.SVG_TITLE)}</text>${parts.join("")}
</svg>
`;
  const out = path.join(CRATE_DIR, "bench.svg");
  await writeFile(out, svg);
  console.log(out);
};

await main();
