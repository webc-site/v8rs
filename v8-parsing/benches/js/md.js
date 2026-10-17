#!/usr/bin/env bun
// 读取 bench/result.json, 生成 markdown 评测报告 bench/report.md

import { writeFileSync } from "node:fs";
import { BENCH_DIR, RESULT_PATH, i18nRead } from "./conf.js";
import { build_groups, fmt_ns, fmt_thrpt, read_json, size_text } from "./lib.js";

const i18n = await i18nRead();

const data = read_json(RESULT_PATH);

const fmt_ratio = (x) => (x === null ? "-" : `${x.toFixed(2)}×`);

const desc = (op) => i18n[`DESC_${op.toUpperCase()}`] ?? op;

const table_md = (rows) => {
  const head = [
    i18n.COL_IMPL,
    i18n.COL_SAMPLE,
    i18n.COL_SIZE,
    i18n.COL_MEAN,
    i18n.COL_MEDIAN,
    i18n.COL_STD,
    i18n.COL_THRPT,
    i18n.COL_RATIO,
  ];
  const body = rows.map((r) => [
    r.impl,
    `${r.sample}${r.param ? ` ${r.param}` : ""}`,
    size_text(r),
    fmt_ns(r.mean_ns),
    fmt_ns(r.median_ns),
    fmt_ns(r.std_dev_ns),
    fmt_thrpt(r),
    fmt_ratio(r.ratio),
  ]);
  return [
    `| ${head.join(" | ")} |`,
    `|${head.map(() => " --- |").join("")}`,
    ...body.map((row) => `| ${row.join(" | ")} |`),
  ].join("\n");
};

const groups = build_groups(data.rows);
const ops = groups.reduce((acc, g) => (acc.includes(g[0].op) ? acc : [...acc, g[0].op]), []);

const sections = ops
  .map((op) => {
    const rows = groups.filter((g) => g[0].op === op).flat();
    return `## ${op}\n\n${desc(op)}\n\n${table_md(rows)}`;
  })
  .join("\n\n");

const text = `# ${i18n.TITLE}

> ${i18n.SUMMARY}

${i18n.GENERATED}: ${data.generated}

${sections}
`;

writeFileSync(`${BENCH_DIR}/report.md`, text);
process.stdout.write(`${BENCH_DIR}/report.md\n`);
