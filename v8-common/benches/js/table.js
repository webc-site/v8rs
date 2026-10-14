#!/usr/bin/env bun
// 控制台表格: 读取汇总结果 json, 输出 操作×实现 的耗时与相对加速比文本表
import { join } from "node:path";

import { echo } from "zx";

import { i18nRead, RESULT_DIR } from "./conf.js";
import { fmtNs, opLabel, readJson } from "./lib.js";

const t = await i18nRead();
const { results } = await readJson(join(RESULT_DIR, "bench.json"));

const HEADS = [t.COL_OP, t.COL_IMPL, t.COL_MEDIAN, t.COL_MEAN, t.COL_CI, t.COL_SPEEDUP];
const cells = results.map((r) => [
  opLabel(t, r.op),
  r.impl,
  fmtNs(r.median_ns),
  fmtNs(r.mean_ns),
  `${fmtNs(r.ci_low_ns)} ~ ${fmtNs(r.ci_high_ns)}`,
  r.speedup_vs_baseline === null ? "—" : `${r.speedup_vs_baseline.toFixed(2)}×`,
]);
const widths = HEADS.map((h, i) => Math.max(h.length, ...cells.map((c) => c[i].length)));
const line = (cols) => `│ ${cols.map((c, i) => c.padEnd(widths[i])).join(" │ ")} │`;
const split = "─".repeat(widths.reduce((a, w) => a + w + 3, 1));

echo(`${line(HEADS)}\n├${split}┤\n${cells.map(line).join("\n")}`);
