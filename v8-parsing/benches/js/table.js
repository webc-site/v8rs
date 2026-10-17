#!/usr/bin/env bun
// 读取 bench/result.json, 生成固定宽度终端对照表, 写入 bench/table.txt

import { writeFileSync } from "node:fs";
import { BENCH_DIR, RESULT_PATH, i18nRead } from "./conf.js";
import { build_groups, fmt_ns, fmt_thrpt, read_json, size_text } from "./lib.js";

const i18n = await i18nRead();

const data = read_json(RESULT_PATH);

const header = [
  i18n.COL_OP,
  i18n.COL_SAMPLE,
  i18n.COL_SIZE,
  i18n.COL_IMPL,
  i18n.COL_MEAN,
  i18n.COL_MEDIAN,
  i18n.COL_STD,
  i18n.COL_THRPT,
  i18n.COL_RATIO,
];

const fmt_ratio = (x) => (x === null ? "-" : `${x.toFixed(2)}×`);

const body = build_groups(data.rows).flatMap((g) =>
  g.map((r) => [
    r.op,
    `${r.sample}${r.param ? ` ${r.param}` : ""}`,
    size_text(r),
    r.impl,
    fmt_ns(r.mean_ns),
    fmt_ns(r.median_ns),
    fmt_ns(r.std_dev_ns),
    fmt_thrpt(r),
    fmt_ratio(r.ratio),
  ]),
);

const cells = [header, ...body];
const widths = header.map((_, i) => Math.max(...cells.map((row) => String(row[i]).length)));

const line = (row) =>
  row
    .map((c, i) => String(c).padEnd(widths[i]))
    .join("  ")
    .trimEnd();
const split = line(widths.map((w) => "─".repeat(w)));

const title = `${i18n.TITLE} (${i18n.GENERATED}: ${data.generated})`;
const text = `${title}\n\n${[line(header), split, ...body.map(line)].join("\n")}\n`;

writeFileSync(`${BENCH_DIR}/table.txt`, text);
process.stdout.write(`${BENCH_DIR}/table.txt\n`);
