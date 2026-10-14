#!/usr/bin/env bun
// 生成 bench/table.html：按操作分组的 HTML 汇总表

import conf, { i18nRead } from "./conf.js";
import { fmt_num, fmt_time, group_rows, read_json, with_rel, write_text } from "./lib.js";

const esc = (text) => text.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");

const cell = (content, tag = "td") => `<${tag}>${content}</${tag}>`;

const row_html = (row) =>
  `<tr>${[row.target, fmt_time(row.mean_ns), fmt_time(row.median_ns), fmt_time(row.slope_ns), `${fmt_time(row.ci_low_ns)} – ${fmt_time(row.ci_high_ns)}`, `${fmt_num(row.rel)}x`].map((v) => cell(esc(v))).join("")}</tr>`;

const table_html = (op, rows, strings) => {
  const headers = [
    strings.COL_TARGET,
    strings.COL_MEAN,
    strings.COL_MEDIAN,
    strings.COL_SLOPE,
    strings.COL_CI,
    strings.COL_REL,
  ];
  return `<h2>${esc(strings[`OP_${op.toUpperCase()}`] ?? op)}</h2>
<p>${esc(strings[`DESC_${op.toUpperCase()}`] ?? "")}</p>
<table>
<thead><tr>${headers.map((h) => cell(esc(h), "th")).join("")}</tr></thead>
<tbody>${rows.map(row_html).join("\n")}</tbody>
</table>`;
};

const gen = async () => {
  const data = await read_json(conf.bench_json);
  const strings = await i18nRead(conf.lang);
  const rows = with_rel(data.results);
  const groups = group_rows(rows, data.ops);
  const style = `<style>body{font-family:system-ui,sans-serif;max-width:56em;margin:2em auto}table{border-collapse:collapse;margin:1em 0}th,td{border:1px solid #bbb;padding:.35em .8em;text-align:right}th{background:#eee}td:first-child,th:first-child{text-align:left}h2{margin-top:2em}</style>`;
  const html = `<!doctype html>
<meta charset="utf-8">
<title>${esc(strings.TITLE)}</title>
${style}
<h1>${esc(strings.TITLE)}</h1>
<p>${esc(strings.SUBTITLE.replace("{baseline}", data.baseline))}</p>
${[...groups.keys()].map((op) => table_html(op, groups.get(op), strings)).join("\n")}
<p><small>${esc(strings.NOTE_ORACLE)}</small></p>
<p><small>${esc(strings.NOTE_SAMPLES)} — ${esc(strings.GENERATED_AT)}: ${esc(data.generated_at)}</small></p>
`;
  await write_text(conf.table_path, html);
  return conf.table_path;
};

if (import.meta.main) await gen();

export default gen;
