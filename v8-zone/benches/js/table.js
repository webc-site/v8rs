#! /usr/bin/env bun
// 读取 bench.json 汇总结果, 渲染文本表格: 写入 report/table.txt 并输出到终端

import { mkdir, writeFile } from "fs/promises";
import { join } from "path";
import { benchJsonPath, i18nRead, reportDir } from "./conf.js";
import { fmtNum, fmtRatio, jsonRead } from "./lib.js";

const PARAM_W = 22;
const IMPL_W = 18;
const opKeys = { alloc: "OP_ALLOC", aligned: "OP_ALIGNED", scoped: "OP_SCOPED", reset: "OP_RESET" };

const pad = (s, w) => s + " ".repeat(Math.max(0, w - s.length));

const render = async () => {
  const summary = await jsonRead(benchJsonPath);
  const i18n = await i18nRead();
  const lines = [`${i18n.REPORT_TITLE} (${i18n.GENERATED_LABEL}: ${summary.generated})`, ""];
  for (const op of summary.ops) {
    const rows = summary.benchmarks.filter((b) => b.op === op);
    const params = [...new Set(rows.map((b) => b.param))];
    const header = [
      pad(i18n.COL_PARAM, PARAM_W),
      ...(rows.length ? summary.impls.map((i) => pad(i, IMPL_W)) : []),
    ];
    lines.push(
      i18n[opKeys[op]],
      header.join("|"),
      "─".repeat(PARAM_W + (IMPL_W + 1) * summary.impls.length),
    );
    for (const param of params) {
      const cells = [pad(param, PARAM_W)];
      for (const impl of summary.impls) {
        const rec = rows.find((b) => b.param === param && b.impl === impl);
        cells.push(pad(rec ? `${fmtNum(rec.allocs_per_sec)} ${fmtRatio(rec.ratio)}` : "-", IMPL_W));
      }
      lines.push(cells.join("|"));
    }
    lines.push("");
  }
  return `${lines.join("\n")}\n`;
};

const text = await render();
await mkdir(reportDir, { recursive: true });
await writeFile(join(reportDir, "table.txt"), text);
await writeFile(1, text);
