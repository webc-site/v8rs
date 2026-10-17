#!/usr/bin/env bun
// markdown 报告: 读取汇总结果 json, 生成 bench/bench.md
// 语言字符串经 conf.js 的 i18nRead 加载 (9.4)
import { writeFile } from "node:fs/promises";
import { join } from "node:path";

import { i18nRead, RESULT_DIR } from "./conf.js";
import { fmtNs, opLabel, readJson } from "./lib.js";

const t = await i18nRead();
const summary = await readJson(join(RESULT_DIR, "bench.json"));
const { results } = summary;

const head = (cols) => `| ${cols.join(" | ")} |\n|${cols.map(() => "---|").join("")}`;
const abs_table = [
  `## ${t.TABLE_ABS}`,
  head([t.COL_OP, t.COL_IMPL, t.COL_MEDIAN, t.COL_MEAN, t.COL_CI]),
  ...results.map(
    (r) =>
      `| ${opLabel(t, r.op)} | ${r.impl} | ${fmtNs(r.median_ns)} | ${fmtNs(r.mean_ns)} | ${fmtNs(r.ci_low_ns)} ~ ${fmtNs(r.ci_high_ns)} |`,
  ),
].join("\n");
const rel_table = [
  `## ${t.TABLE_REL}`,
  head([t.COL_OP, t.COL_IMPL, t.COL_SPEEDUP]),
  ...results.map(
    (r) =>
      `| ${opLabel(t, r.op)} | ${r.impl} | ${r.speedup_vs_baseline === null ? "—" : `${r.speedup_vs_baseline.toFixed(2)}×`} |`,
  ),
].join("\n");

const md = `# ${t.TITLE}

${t.INTRO}

${t.GEN_AT}: ${summary.generated_at}

${abs_table}

${rel_table}

> ${t.NOTE_BASELINE}
>
> ${t.NOTE_METHOD}
`;
await writeFile(join(RESULT_DIR, "bench.md"), md);
