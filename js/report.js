#!/usr/bin/env -S bun

// 流水线第三阶段：汇总转译 meta 与 check 结果，生成显式的转换报告
import { existsSync, mkdirSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root_dir = join(import.meta.dirname, ".."),
  gen_dir = join(root_dir, "v8rs", "src", "v8"),
  build_dir = join(root_dir, "cpp2rust", "build"),
  check_result = join(build_dir, "check-result.json"),
  run_stats = join(build_dir, "run-stats.json"),
  report_json = join(build_dir, "conversion-report.json"),
  report_md = join(build_dir, "conversion-report.md");

export const build_report = () => {
  const files = existsSync(gen_dir) ? readdirSync(gen_dir) : [];
  const meta_li = files
    .filter((f) => f.endsWith(".rs.meta.json"))
    .map((f) => {
      try {
        return JSON.parse(readFileSync(join(gen_dir, f), "utf8"));
      } catch {
        return null;
      }
    })
    .filter(Boolean);

  let check = { kept: [], dropped: {} };
  if (existsSync(check_result)) {
    try {
      check = JSON.parse(readFileSync(check_result, "utf8"));
    } catch {}
  }

  let run = { total_found: 0, skipped_up_to_date: 0 };
  if (existsSync(run_stats)) {
    try {
      run = { ...run, ...JSON.parse(readFileSync(run_stats, "utf8")) };
    } catch {}
  }

  const kept = new Set(check.kept);
  const stats = {
    ok_kept: 0,
    check_dropped: 0,
    timeout: 0,
    error: 0,
    skipped_up_to_date: run.skipped_up_to_date,
    total_found: run.total_found || meta_li.length,
    total_meta: meta_li.length,
  };
  const modules = [];

  for (const m of meta_li) {
    let final_state = m.state;
    if (m.state === "ok") {
      final_state = kept.has(m.mod) ? "ok_kept" : "check_dropped";
    }
    stats[final_state] = (stats[final_state] || 0) + 1;
    modules.push({
      file: m.file,
      mod: m.mod,
      state: final_state,
      reason: final_state === "check_dropped" ? check.dropped[m.mod] || "" : m.error || "",
      log: m.state === "ok" ? "" : "cpp2rust/build/logs/" + m.mod + ".log",
    });
  }

  const report = {
    generated_at: new Date().toISOString(),
    sampled: !!process.env.CONV_LIMIT,
    limit: process.env.CONV_LIMIT ? Number(process.env.CONV_LIMIT) : null,
    stats,
    coverage: stats.total_meta > 0 ? (stats.ok_kept / stats.total_meta * 100).toFixed(2) + "%" : "0%",
    modules: modules.sort((a, b) => a.file.localeCompare(b.file)),
  };

  mkdirSync(build_dir, { recursive: true });
  writeFileSync(report_json, JSON.stringify(report, null, 2));

  let md =
    "# v8 → rs 转换报告\n\n" +
    "- 生成时间: " + report.generated_at + "\n" +
    (report.sampled ? "- **注意: 本次为 CONV_LIMIT=" + report.limit + " 抽样运行，未覆盖全部源文件**\n" : "") +
    "- 扫描文件: " + stats.total_found + "（有 meta 记录 " + stats.total_meta + "）\n" +
    "- 转译成功且通过编译: " + stats.ok_kept + " (" + report.coverage + ")\n" +
    "- 转译成功但编译不通过: " + stats.check_dropped + "\n" +
    "- 转译超时: " + stats.timeout + "\n" +
    "- 转译失败: " + stats.error + "\n" +
    "- 增量跳过(未重转): " + stats.skipped_up_to_date + "\n\n" +
    "## 编译不通过模块\n\n" +
    modules
      .filter((m) => m.state === "check_dropped")
      .map((m) => "- `" + m.mod + "`: " + (m.reason || "").split("\n")[0])
      .join("\n") +
    "\n\n## 转译失败/超时模块\n\n" +
    modules
      .filter((m) => m.state === "error" || m.state === "timeout")
      .map((m) => "- `" + m.file + "` [" + m.state + "]: " + (m.reason || "").split("\n")[0] + " (日志: " + m.log + ")")
      .join("\n") +
    "\n";
  writeFileSync(report_md, md);
  return report;
};

if (import.meta.main) {
  const report = build_report();
  console.log(
    "报告已生成: " +
      report_json +
      "\n统计: " +
      JSON.stringify(report.stats) +
      " 覆盖率 " +
      report.coverage
  );
}
