#! /usr/bin/env bun
// 表格报告 (genbench 步骤 9): 读 bench.json, 输出 HTML 对比表到 report/table.html

import { mkdir, writeFile } from "node:fs/promises";
import { DEFAULT_LANG, REPORT_DIR, i18n_read } from "./conf.js";
import { fmt_ratio, fmt_time, read_bench, to_groups } from "./lib.js";

const LANG = await i18n_read(DEFAULT_LANG);

const { results, impls, generated_at } = await read_bench();
const { groups, gain } = to_groups(results);

const head_cells = [
  LANG.COL_OPERATION,
  LANG.COL_IMPL,
  LANG.COL_MEAN,
  LANG.COL_MEDIAN,
  LANG.COL_STD_DEV,
  LANG.COL_CI95,
  LANG.COL_VS_FASTEST,
]
  .map((h) => `<th>${h}</th>`)
  .join("");

const body_rows = groups
  .map((g) => {
    const op_label = LANG[`OP_${g.op.toUpperCase()}`];
    return g.items
      .map((i, idx) => {
        const label =
          idx === 0
            ? `<td class="op" rowspan="${g.items.length}">${op_label}${g.param ? ` (${g.param})` : ""}</td>`
            : "";
        const best = i.impl === g.fastest.impl ? ` class="best"` : "";
        const ratio = i.vs_fastest === 1 ? `1.00× (${LANG.FASTEST})` : fmt_ratio(i.vs_fastest);
        return `<tr>${label}<td>${i.impl}</td><td${best}>${fmt_time(i.mean_ns)}</td><td>${fmt_time(i.median_ns)}</td><td>${fmt_time(i.std_dev_ns)}</td><td>${fmt_time(i.ci95_ns[0])} ~ ${fmt_time(i.ci95_ns[1])}</td><td>${ratio}</td></tr>`;
      })
      .join("\n");
  })
  .join("\n");

const html = `<!doctype html>
<html lang="${DEFAULT_LANG}">
<head>
<meta charset="utf-8">
<title>${LANG.REPORT_TITLE}</title>
<style>
body { font-family: system-ui, sans-serif; margin: 2rem; }
table { border-collapse: collapse; width: 100%; }
th, td { border: 1px solid #bbb; padding: 4px 10px; text-align: right; }
th { background: #f0f0f0; }
td.op, td:first-child { text-align: left; }
td.best { color: #0a7d32; font-weight: 600; }
caption { caption-side: top; font-weight: 600; padding: 8px; }
footer { margin-top: 1rem; color: #777; font-size: 0.85rem; }
</style>
</head>
<body>
<h1>${LANG.REPORT_TITLE}</h1>
<p>${LANG.REPORT_INTRO}</p>
<p><b>${LANG.GAIN_TITLE}</b>: ${LANG.GAIN_SUMMARY} ${fmt_ratio(gain)} ${LANG.GAIN_SUMMARY_SUFFIX}</p>
<table>
<caption>${LANG.TABLE_CAPTION}</caption>
<thead><tr>${head_cells}</tr></thead>
<tbody>
${body_rows}
</tbody>
</table>
<p>${LANG.UNIT_NOTE}</p>
<footer>${LANG.GENERATED_AT}: ${generated_at} · ${LANG.FOOTER} · impls: ${impls.join(", ")}</footer>
</body>
</html>
`;

await mkdir(REPORT_DIR, { recursive: true });
await writeFile(`${REPORT_DIR}/table.html`, html);
console.log(`${REPORT_DIR}/table.html`);
