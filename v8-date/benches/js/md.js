#!/usr/bin/env bun
// md.js — 读取 bench.js 生成的汇总 json, 生成 markdown 评测报告 bench-report.md
// 文案加载遵循 conf.js 的 i18nRead

import { writeFile } from "node:fs/promises";
import { groupByOp, loadSummary, fmtNs, fmtRatio, mdTable } from "./lib.js";
import { BASELINE_GROUP, CRATE_NAME, MD_OUT, RESULT_JSON, i18nRead } from "./conf.js";

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
      `\`${r.group}\``,
      fmtNs(r.mean_ns),
      fmtNs(r.median_ns),
      fmtRatio(ratios[i]),
      `±${fmtNs(r.std_err_ns)}`,
    ]),
  );
}

const report = `# ${i18nRead("TITLE")}

> ${i18nRead("SUBTITLE")}

- ${i18nRead("GENERATED_AT")}: ${summary.generated_at}
- ${i18nRead("BENCH_ARGS")}: \`${summary.bench_args.join(" ")}\`
- crate: \`${CRATE_NAME}\`

## ${i18nRead("SECTION_OPS")}

- ${i18nRead("OP_PARSE")}
- ${i18nRead("OP_YMD")}
- ${i18nRead("OP_TOD")}

${i18nRead("IMPL_NOTE")}

## ${i18nRead("RESULTS")}

${mdTable(headers, rows)}

${i18nRead("TABLE_HINT")}
`;

await writeFile(MD_OUT, report);
console.info(`${MD_OUT}`);
