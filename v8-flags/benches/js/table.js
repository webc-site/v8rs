#!/usr/bin/env bun
// benches/js/table.js — 汇总 bench-result.json 为 CSV 数据表 (机器可读, 固定英文表头)

import { readJson, writeText } from "./lib.js";
import { result_json, table_csv } from "./conf.js";

const HEADER = "operation,impl,mean_ns,median_ns,std_dev_ns,ci_lower_ns,ci_upper_ns";

const runTable = async () => {
  const result = await readJson(result_json);
  const rows = result.measures.map(
    ({ group, impl, mean_ns, median_ns, std_dev_ns, ci_lower_ns, ci_upper_ns }) =>
      [group, impl, mean_ns, median_ns, std_dev_ns, ci_lower_ns, ci_upper_ns].join(","),
  );
  await writeText(table_csv, `${[HEADER, ...rows].join("\n")}\n`);
  return table_csv;
};

export default runTable;

if (import.meta.main) await runTable();
