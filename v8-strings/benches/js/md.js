#!/usr/bin/env bun
// Markdown 报告: 每个核心操作一节, 表格列出实现 × 样本的均值纳秒与 std 倍率;
// 语言字符串经 conf.js 的 i18nRead 加载。

import { MD_OUT, i18nRead } from "./conf.js";
import { loadSummary, groupByOp, sampleOrder, stdBase, fmtNs, fmtRatio } from "./lib.js";

const results = await loadSummary(),
  by_op = groupByOp(results),
  elems_of = new Map(results.map((r) => [r.sample, r.elems])),
  label_of = (sample) => `${sample} (${elems_of.get(sample) ?? 0})`;

const cell_of = (rows, impl, sample) => {
  const r = rows.find((x) => x.impl === impl && x.sample === sample);
  if (!r) return "-";
  return `${fmtNs(r.ns_mean)} (${fmtRatio(r.ns_mean, stdBase(rows, sample))})`;
};

let md = `# ${i18nRead("TITLE")}\n\n> ${i18nRead("METHOD_NOTE")}\n\n> ${i18nRead("CORPUS_NOTE")}\n\n`;

for (const op of by_op.keys()) {
  const rows = by_op.get(op),
    samples = sampleOrder(rows),
    impls = [...new Set(rows.map((r) => r.impl))];
  md += `## ${op} · ${i18nRead(`OP_${op.toUpperCase()}`)}\n\n`;
  md += `| ${i18nRead("COL_IMPL")} | ${samples.map(label_of).join(" | ")} |\n`;
  md += `| --- |${" --- |".repeat(samples.length)}\n`;
  md += impls
    .map((impl) => `| ${impl} | ${samples.map((s) => cell_of(rows, impl, s)).join(" | ")} |`)
    .join("\n");
  md += "\n\n";
}

md += `${i18nRead("RATIO_NOTE")}\n`;

await Bun.write(MD_OUT, md);
process.stdout.write(`${MD_OUT}\n`);
