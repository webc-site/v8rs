#!/usr/bin/env bun
// benches/js/md.js — 生成 Markdown 评测报告: 指标表 (均值 + 相对基线) + svg 图引用
// 语言字符串按 i18n/<lang>.js 机制加载 (conf.js 的 i18nRead)

import { fmtNs, fmtRatio, measureOf, readJson, writeText } from "./lib.js";
import { i18nRead, md_report, result_json } from "./conf.js";

const BASELINE = "v8rs-phf";

const runMd = async () => {
  const result = await readJson(result_json);
  const i18n = await i18nRead();
  const { groups, impls } = result;
  const compared = impls.filter(impl => impl !== BASELINE);
  const labelOf = group => i18n[`${group.toUpperCase()}_LABEL`] ?? group;
  const meanOf = (group, impl) => measureOf(result, group, impl)?.mean_ns;

  const header = [
    i18n.COL_OPERATION,
    ...impls.map(impl => `${impl} (mean)`),
    ...compared.map(impl => `${impl} ${i18n.COL_RELATIVE}`),
  ];
  const rows = groups.map(group => {
    const base = meanOf(group, BASELINE);
    return [
      labelOf(group),
      ...impls.map(impl => {
        const value = meanOf(group, impl);
        return value ? fmtNs(value) : "—";
      }),
      ...compared.map(impl => {
        const value = meanOf(group, impl);
        return value && base ? fmtRatio(value / base) : "—";
      }),
    ].join(" | ");
  });

  const md = `# ${i18n.REPORT_TITLE}

${i18n.REPORT_INTRO}

${i18n.GENERATED_AT}: ${result.generated_at}

| ${header.join(" | ")} |
| ${header.map(() => "---").join(" | ")} |
${rows.join("\n")}

> ${i18n.BASELINE_NOTE}

![chart](bench-chart.svg)

${i18n.ARTIFACT_LINKS}
`;
  await writeText(md_report, md);
  return md_report;
};

export default runMd;

if (import.meta.main) await runMd();
