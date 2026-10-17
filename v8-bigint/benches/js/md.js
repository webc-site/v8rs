#!/usr/bin/env bun
// 目的: 读 bench-result.json, 生成完整 markdown 评测报告 bench-report.md
// 加载语言文件用 conf.js 的 i18n_read
import { echo, fs, path } from "zx";
import { crate_dir, i18n_read, result_json } from "./conf.js";
import { md_table, summary } from "./lib.js";

const run = async () => {
  const i18n = await i18n_read();
  const rows = await fs.readJson(result_json);
  const { base_impl, peers, cells } = summary(rows);
  const table = md_table(
    [i18n.COL_OP, i18n.COL_RADIX, i18n.COL_BITS, base_impl, ...peers, i18n.COL_RATIO],
    cells,
  );
  const md = `# ${i18n.TITLE}

${i18n.SUB}

## ${i18n.SEC_TABLE}

${table}
## ${i18n.SEC_SVG}

见 [bench-chart.svg](./bench-chart.svg)。

## ${i18n.SEC_NOTES}

- ${i18n.NOTE_MALACHITE}
- ${i18n.NOTE_BARRETT}
- ${i18n.NOTE_ALGO}
- ${i18n.NOTE_SAMPLES}
- ${i18n.NOTE_TIME}
`;
  await fs.outputFile(path.join(crate_dir, "bench-report.md"), md);
  echo(`${path.join(crate_dir, "bench-report.md")}`);
};

export default await run();
