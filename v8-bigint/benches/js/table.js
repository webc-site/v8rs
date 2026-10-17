#!/usr/bin/env bun
// 目的: 读 bench-result.json, 生成 markdown 汇总表 bench-table.md
import { echo, fs, path } from "zx";
import { crate_dir, i18n_read, result_json } from "./conf.js";
import { md_table, summary } from "./lib.js";

const run = async () => {
  const i18n = await i18n_read();
  const rows = await fs.readJson(result_json);
  const { base_impl, peers, cells } = summary(rows);
  const md = md_table(
    [i18n.COL_OP, i18n.COL_RADIX, i18n.COL_BITS, base_impl, ...peers, i18n.COL_RATIO],
    cells,
  );
  await fs.outputFile(path.join(crate_dir, "bench-table.md"), `# ${i18n.TITLE}\n\n${md}`);
  echo(`${path.join(crate_dir, "bench-table.md")} (${cells.length} 行)`);
};

export default await run();
