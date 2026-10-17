#!/usr/bin/env bun
// table.js — 读取 bench.js 生成的汇总 json, 输出等宽文本表格 (stdout + bench-report.txt)

import { writeFile } from "node:fs/promises";
import { groupByOp, loadSummary, fmtNs, fmtRatio, renderTable } from "./lib.js";
import { BASELINE_GROUP, RESULT_JSON, TABLE_OUT, i18nRead } from "./conf.js";

const summary = await loadSummary(RESULT_JSON),
  ops = groupByOp(summary.benches, BASELINE_GROUP);

const headers = [
    i18nRead("COL_OP"),
    i18nRead("COL_IMPL"),
    i18nRead("COL_MEAN"),
    i18nRead("COL_MEDIAN"),
    i18nRead("COL_RATIO"),
    i18nRead("COL_ERR"),
  ],
  rows = [];

for (const { op, rows: impls, ratios } of ops) {
  impls.forEach((r, i) =>
    rows.push([
      op,
      r.group,
      fmtNs(r.mean_ns),
      fmtNs(r.median_ns),
      fmtRatio(ratios[i]),
      `±${fmtNs(r.std_err_ns)}`,
    ]),
  );
}

const table_text = `${i18nRead("TITLE")} (${summary.generated_at})\n\n${renderTable(headers, rows)}\n\n${i18nRead("TABLE_HINT")}\n`;

await writeFile(TABLE_OUT, table_text);
console.info(table_text);
