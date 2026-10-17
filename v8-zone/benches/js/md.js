#! /usr/bin/env bun
// 读取 bench.json 汇总结果, 生成 Markdown 报告 report/report.md
// 语言文案经 conf.js 的 i18nRead 加载 (genbench 9.4)

import { mkdir, writeFile } from "fs/promises";
import { join } from "path";
import { benchJsonPath, i18nRead, reportDir } from "./conf.js";
import { fmtNum, fmtRatio, jsonRead } from "./lib.js";

const opKeys = { alloc: "OP_ALLOC", aligned: "OP_ALIGNED", scoped: "OP_SCOPED", reset: "OP_RESET" };

const mdTable = (head_cells, body_rows) => {
  const header = `| ${head_cells.join(" | ")} |`;
  const sep = `| :--- |${" ---: |".repeat(head_cells.length - 1)}`;
  return [header, sep, ...body_rows].join("\n");
};

const render = async () => {
  const summary = await jsonRead(benchJsonPath);
  const i18n = await i18nRead();
  const blocks = [];
  for (const op of summary.ops) {
    const rows = summary.benchmarks.filter((b) => b.op === op);
    const params = [...new Set(rows.map((b) => b.param))];
    const heads = [
      i18n.COL_PARAM,
      ...summary.impls.map((impl) => `${impl} (${i18n.COL_ALLOCS} / ${i18n.COL_RATIO})`),
    ];
    const body = params.map((param) => {
      const cells = summary.impls.map((impl) => {
        const rec = rows.find((b) => b.param === param && b.impl === impl);
        return rec ? `${fmtNum(rec.allocs_per_sec)} ${fmtRatio(rec.ratio)}` : "-";
      });
      return `| ${param} | ${cells.join(" | ")} |`;
    });
    blocks.push([`### ${i18n[opKeys[op]]}`, mdTable(heads, body)].join("\n\n"));
  }
  return `${i18n.MD_INTRO}\n${i18n.MD_IMPL_NOTE}\n\n${blocks.join("\n\n")}\n`;
};

const md = await render();
await mkdir(reportDir, { recursive: true });
await writeFile(join(reportDir, "report.md"), md);
await writeFile(1, `${md.length} chars -> report/report.md\n`);
