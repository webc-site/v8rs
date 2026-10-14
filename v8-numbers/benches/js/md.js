#!/usr/bin/env bun
// Markdown 报告 — 读取 bench.json, 生成 crate 根 bench.md
import { writeFile } from "node:fs/promises";
import path from "node:path";
import { BENCH_JSON, CRATE_DIR, i18nRead, pickLang, readJson } from "./conf.js";
import { fmtNs, loadRows } from "./lib.js";

const main = async () => {
  const [i18n, bench, rows] = await Promise.all([
    i18nRead(pickLang()),
    readJson(BENCH_JSON),
    loadRows(),
  ]);
  const md = [];
  md.push(`# ${i18n.TITLE}`, "", `> ${i18n.DESC}`, "");
  for (const { op, results } of rows) {
    md.push(
      `## ${op}`,
      "",
      `| ${i18n.IMPL_HEADER} | ${i18n.TIME_HEADER} | ${i18n.REL_HEADER} | ${i18n.ERR_HEADER} |`,
      "|---|---:|---:|---:|",
    );
    for (const r of results) {
      const rel = r.ratio == null ? "-" : `${r.ratio.toFixed(2)}x`;
      md.push(
        `| ${r.name} | ${fmtNs(r.ns_per_elem)} | ${rel} | ${((r.std_err / r.ns_per_iter) * 100).toFixed(1)}% |`,
      );
    }
    md.push("");
  }
  md.push(`---`, "", `${i18n.GENERATED}: ${bench.generated_at}`, i18n.CRITERION_HINT, "");
  await writeFile(path.join(CRATE_DIR, "bench.md"), md.join("\n"));
  console.log(path.join(CRATE_DIR, "bench.md"));
};

await main();
