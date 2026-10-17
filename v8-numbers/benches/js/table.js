#!/usr/bin/env bun
// 终端表格报告 — 读取 bench.json, 按操作输出对比表
import { i18nRead, pickLang } from "./conf.js";
import { cellText, loadRows } from "./lib.js";

const padEnd = (s, w) => `${s}${" ".repeat(Math.max(0, w - s.length))}`;

const main = async () => {
  const i18n = await i18nRead(pickLang());
  const rows = await loadRows();
  const widths = rows.map(({ op, results }) => ({
    op,
    name_w: Math.max(i18n.IMPL_HEADER.length, ...results.map((r) => r.name.length)),
    time_w: Math.max(i18n.TIME_HEADER.length, ...results.map((r) => cellText(r).length)),
    err_w: Math.max(
      i18n.ERR_HEADER.length,
      ...results.map((r) => `${((r.std_err / r.ns_per_iter) * 100).toFixed(1)}%`.length),
    ),
    iter_w: Math.max(
      i18n.ITER_HEADER.length,
      ...results.map((r) => Math.round(r.ns_per_iter).toLocaleString("en-US").length),
    ),
  }));
  const name_w = Math.max(...widths.map((w) => w.name_w));
  const time_w = Math.max(...widths.map((w) => w.time_w));
  const err_w = Math.max(...widths.map((w) => w.err_w));
  const iter_w = Math.max(...widths.map((w) => w.iter_w));

  const line = (a, b, c, d) =>
    `${padEnd(a, name_w)}  ${padEnd(b, time_w)}  ${padEnd(c, err_w)}  ${padEnd(d, iter_w)}`;
  console.log(`${i18n.TITLE}\n${i18n.DESC}`);
  for (const { op, results } of rows) {
    console.log(`\n[${op}]`);
    console.log(line(i18n.IMPL_HEADER, i18n.TIME_HEADER, i18n.ERR_HEADER, i18n.ITER_HEADER));
    for (const r of results) {
      console.log(
        line(
          r.name,
          cellText(r),
          `${((r.std_err / r.ns_per_iter) * 100).toFixed(1)}%`,
          Math.round(r.ns_per_iter).toLocaleString("en-US"),
        ),
      );
    }
  }
};

await main();
