#!/usr/bin/env bun
// 生成 bench/report.md：markdown 评测报告（语言文件经 conf.js i18nRead 加载）

import conf, { i18nRead } from "./conf.js";
import { fmt_num, fmt_time, group_rows, read_json, with_rel, write_text } from "./lib.js";

const md_table = (rows, strings) => {
  const headers = [
    strings.COL_TARGET,
    strings.COL_MEAN,
    strings.COL_MEDIAN,
    strings.COL_SLOPE,
    strings.COL_CI,
    strings.COL_REL,
  ];
  return [
    `| ${headers.join(" | ")} |`,
    `| ${headers.map(() => "---:").join(" | ")} |`,
    ...rows.map(
      (row) =>
        `| ${row.target} | ${fmt_time(row.mean_ns)} | ${fmt_time(row.median_ns)} | ${fmt_time(row.slope_ns)} | ${fmt_time(row.ci_low_ns)} – ${fmt_time(row.ci_high_ns)} | ${fmt_num(row.rel)}x |`,
    ),
  ].join("\n");
};

const gen = async () => {
  const data = await read_json(conf.bench_json);
  const strings = await i18nRead(conf.lang);
  const rows = with_rel(data.results);
  const groups = group_rows(rows, data.ops);
  const sections = [...groups.keys()]
    .map(
      (op) => `## ${strings[`OP_${op.toUpperCase()}`] ?? op}

${strings[`DESC_${op.toUpperCase()}`] ?? ""}

${md_table(groups.get(op), strings)}`,
    )
    .join("\n\n");
  const md = `# ${strings.TITLE}

> ${strings.SUBTITLE.replace("{baseline}", data.baseline)}

${sections}

## 说明

- ${strings.NOTE_ORACLE}
- ${strings.NOTE_SAMPLES}
- ${strings.GENERATED_AT}: ${data.generated_at}
`;
  await write_text(conf.md_path, md);
  return conf.md_path;
};

if (import.meta.main) await gen();

export default gen;
