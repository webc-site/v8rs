#! /usr/bin/env bun
// Markdown 报告 (genbench 步骤 9, 9.4: 经 conf.js 的 i18nRead 加载语言文件):
// 读 bench.json, 输出 report/report.md

import { mkdir, writeFile } from "node:fs/promises";
import { DEFAULT_LANG, REPORT_DIR, i18n_read } from "./conf.js";
import { fmt_ratio, fmt_time, read_bench, to_groups } from "./lib.js";

const LANG = await i18n_read(DEFAULT_LANG);

const { results, impls, generated_at } = await read_bench();
const { groups, gain } = to_groups(results);

const op_section = (g) => {
  const op_label = LANG[`OP_${g.op.toUpperCase()}`];
  const rows = g.items
    .map(
      (i) =>
        `| ${i.impl}${i.impl === g.fastest.impl ? " ⭐" : ""} | ${fmt_time(i.mean_ns)} | ${fmt_time(i.median_ns)} | ${fmt_time(i.std_dev_ns)} | ${fmt_time(i.ci95_ns[0])} ~ ${fmt_time(i.ci95_ns[1])} | ${fmt_ratio(i.vs_fastest)} |`,
    )
    .join("\n");
  return `### ${op_label}${g.param ? ` (${g.param})` : ""}\n\n| ${LANG.COL_IMPL} | ${LANG.COL_MEAN} | ${LANG.COL_MEDIAN} | ${LANG.COL_STD_DEV} | ${LANG.COL_CI95} | ${LANG.COL_VS_FASTEST} |\n| --- | ---: | ---: | ---: | ---: | ---: |\n${rows}`;
};

const md = `# ${LANG.REPORT_TITLE}

${LANG.REPORT_INTRO}

> **${LANG.GAIN_TITLE}**: ${LANG.GAIN_SUMMARY} **${fmt_ratio(gain)}** ${LANG.GAIN_SUMMARY_SUFFIX}

${LANG.UNIT_NOTE}

## ${LANG.TABLE_CAPTION}

${groups.map(op_section).join("\n\n")}

---

${LANG.GENERATED_AT}: ${generated_at} · 实现: ${impls.join(", ")}

${LANG.FOOTER}
`;

await mkdir(REPORT_DIR, { recursive: true });
await writeFile(`${REPORT_DIR}/report.md`, md);
console.log(`${REPORT_DIR}/report.md`);
