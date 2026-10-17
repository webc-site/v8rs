#!/usr/bin/env bun
// 终端表格报告: 按核心操作分组, 输出各实现 × 语料样本的均值纳秒与 std 倍率。

import { i18nRead } from "./conf.js";
import { loadSummary, groupByOp, sampleOrder, stdBase, fmtNs, fmtRatio } from "./lib.js";

const results = await loadSummary(),
  by_op = groupByOp(results),
  elems_of = new Map(results.map((r) => [r.sample, r.elems])),
  label_of = (sample) => `${sample} (${elems_of.get(sample) ?? 0})`,
  op_label = (op) => i18nRead(`OP_${op.toUpperCase()}`),
  width_of = (text) => String(text).length;

let out = `\n${i18nRead("TITLE")} — ${i18nRead("METHOD_NOTE")}\n`;

for (const op of by_op.keys()) {
  const rows = by_op.get(op),
    samples = sampleOrder(rows),
    impls = [...new Set(rows.map((r) => r.impl))],
    head = [i18nRead("COL_IMPL"), ...samples.map(label_of)],
    body = impls.map((impl) => [
      impl,
      ...samples.map((sample) => {
        const r = rows.find((x) => x.impl === impl && x.sample === sample);
        if (!r) return "-";
        return `${fmtNs(r.ns_mean)} (${fmtRatio(r.ns_mean, stdBase(rows, sample))})`;
      }),
    ]),
    widths = head.map((h, i) => Math.max(width_of(h), ...body.map((row) => width_of(row[i])))),
    line_of = (cells) => cells.map((c, i) => String(c).padEnd(widths[i])).join("|") + "\n";
  out += `\n== ${op} · ${op_label(op)} ==\n${line_of(head)}${body.map(line_of).join("")}`;
}

process.stdout.write(`${out}\n${i18nRead("RATIO_NOTE")}\n${i18nRead("CORPUS_NOTE")}\n`);
