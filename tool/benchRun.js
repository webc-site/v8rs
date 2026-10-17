#!/usr/bin/env -S bun
import { readdir } from "node:fs/promises";
import os from "node:os";
import { basename, join } from "node:path";
import { $ } from "zx";

$.verbose = false;

const ROOT_DIR = join(import.meta.dirname, ".."),
  BENCH_DIR = join(ROOT_DIR, "v8-benchmarks/benches/v8-v7"),
  CONF_DIR = join(import.meta.dirname, "benchRun"),
  V8RS_NAME_MAP = {
    Richards: "richards",
    DeltaBlue: "deltablue",
    Crypto: "crypto",
    RayTrace: "raytrace",
    EarleyBoyer: "earley-boyer",
    RegExp: "regexp",
    Splay: "splay",
    NavierStokes: "navier-stokes",
  };

const sysInfo = async () => {
  let cpu = os.cpus()[0]?.model ?? "Unknown CPU";
  if (process.platform === "darwin") {
    try {
      cpu = (await $`sysctl -n machdep.cpu.brand_string`).text().trim();
    } catch {}
  }
  const os_name = process.platform === "darwin" ? "macOS" : os.type(),
    os_ver = os.release(),
    arch = os.arch(),
    core_count = os.cpus().length;
  return { cpu, os_name, os_ver, arch, core_count };
};

const v8rsBench = async (root_dir) => {
  const { stdout } = await $({ cwd: root_dir })`cargo run --release -p v8-benchmarks -- --json`;
  return JSON.parse(stdout.trim());
};

const cppBench = async (bench_dir) => {
  const code = `
const fs = require("node:fs");
const path = require("node:path");
const vm = require("node:vm");
const dir = ${JSON.stringify(bench_dir)};
const files = ["base.js", "richards.js", "deltablue.js", "crypto.js", "raytrace.js", "earley-boyer.js", "regexp.js", "splay.js", "navier-stokes.js"];
for (const f of files) {
  vm.runInThisContext(fs.readFileSync(path.join(dir, f), "utf8"));
}
BenchmarkSuite.RunSuites({});
const score_map = {};
for (const s of BenchmarkSuite.suites) {
  const mean = BenchmarkSuite.GeometricMean(s.results);
  const score = Math.round(100 * s.reference / mean);
  const latency_ms = (mean / 1000).toFixed(2);
  score_map[s.name] = { score, latency_ms };
}
const total_score = Math.round(100 * BenchmarkSuite.GeometricMean(BenchmarkSuite.scores));
console.log(JSON.stringify({ score_map, total_score, v8_ver: process.versions.v8 }));
`;
  const { stdout } = await $`node -e ${code}`;
  return JSON.parse(stdout.trim());
};

const latencyFmt = (ns) => (ns < 1000 ? ns.toFixed(2) + " ns" : (ns / 1000).toFixed(2) + " µs");

const deltaFmt = (v8rs_val, cpp_val) => {
  if (!cpp_val) return "N/A";
  const diff = ((v8rs_val - cpp_val) / cpp_val) * 100,
    sign = diff >= 0 ? "+" : "";
  return sign + diff.toFixed(1) + "%";
};

const templateRender = (str, data) =>
  str.replaceAll(/\{(\w+)\}/g, (_, key) => data[key] ?? "");

const benchMdRender = (cfg, ctx) => {
  const { sys, cpp_data, v8rs_data, bin_size_mb, cpp_bin_mb, reduction_pct, overall_delta, latency_map } = ctx,
    env_str = templateRender(cfg.env, {
      cpu: sys.cpu,
      core_count: sys.core_count,
      os_name: sys.os_name,
      os_ver: sys.os_ver,
      arch: sys.arch,
      v8_ver: cpp_data.v8_ver,
    }),
    highlight_data = {
      v8rs_score: v8rs_data.suite.Score.toLocaleString(),
      cpp_score: cpp_data.total_score.toLocaleString(),
      delta: overall_delta,
      bin_size_mb,
      cpp_bin_mb: cpp_bin_mb.toFixed(1),
      reduction_pct,
      v8rs_startup: ctx.v8rs_startup,
      cpp_startup: ctx.cpp_startup,
      startup_speedup: ctx.startup_speedup,
      startup_reduction: ctx.startup_reduction,
    },
    startup_line = cfg.highlights.startup ? templateRender(cfg.highlights.startup, highlight_data) + "\n" : "",
    score_line = templateRender(cfg.highlights.score, highlight_data),
    bin_line = templateRender(cfg.highlights.binary_size, highlight_data),
    th_line = `| ${cfg.table.headers.join(" | ")} |`,
    sep_line = `| :--- | :--- | :---: | :---: | :---: | :---: |`,
    row_li = Object.entries(cfg.suites).map(([name, desc]) => {
      const v8rs_val = v8rs_data.suite[name] ?? 0,
        cpp_val = cpp_data.score_map[name]?.score ?? 0,
        delta = deltaFmt(v8rs_val, cpp_val),
        lat = latencyFmt(latency_map[V8RS_NAME_MAP[name]] ?? 0);
      return `| **${name}** | ${desc} | **${v8rs_val.toLocaleString()}** | ${cpp_val.toLocaleString()} | ${delta} | ${lat} |`;
    });

  return `## ${cfg.title}

${cfg.desc}

> ${env_str}

${th_line}
${sep_line}
${row_li.join("\n")}

### ${cfg.highlights.title}

${startup_line}${score_line}
${bin_line}
`;
};

const run = async () => {
  console.log("1. 采集系统与硬件信息...");
  const sys = await sysInfo();

  console.log("2. 实测 V8RS 性能基准...");
  const v8rs_data = await v8rsBench(ROOT_DIR);

  console.log("3. 实测 Native C++ V8 (Node v8-v7) 性能基准...");
  const cpp_data = await cppBench(BENCH_DIR);

  console.log("4. 采样实测进程冷启动耗时...");
  let cpp_startup_total = 0;
  for (let i = 0; i < 5; i++) {
    const t0 = performance.now();
    await $`node -e ""`;
    cpp_startup_total += (performance.now() - t0);
  }
  const cpp_startup = (cpp_startup_total / 5).toFixed(2);

  let v8rs_startup_total = 0;
  const cargo_meta = JSON.parse((await $({ cwd: ROOT_DIR })`cargo metadata --format-version 1 --no-deps`).stdout);
  let rs8_bin = join(cargo_meta.target_directory, "release/rs8");
  if (!(await Bun.file(rs8_bin).exists())) {
    await $({ cwd: ROOT_DIR })`cargo build --release -p rs8 --bin rs8`;
  }
  for (let i = 0; i < 5; i++) {
    const t0 = performance.now();
    await $`${rs8_bin} -e ""`;
    v8rs_startup_total += (performance.now() - t0);
  }
  const v8rs_startup = (v8rs_startup_total / 5).toFixed(2);
  const startup_speedup = (Number(cpp_startup) / Math.max(0.1, Number(v8rs_startup))).toFixed(1);
  const startup_reduction = (((Number(cpp_startup) - Number(v8rs_startup)) / Number(cpp_startup)) * 100).toFixed(1);

  const bin_size_mb = (v8rs_data.suite.BinarySize / (1024 * 1024)).toFixed(1),
    cpp_bin_mb = 32.0,
    reduction_pct = (((cpp_bin_mb - bin_size_mb) / cpp_bin_mb) * 100).toFixed(1),
    latency_map = Object.fromEntries(v8rs_data.benchmarks.map((b) => [b.name, b.ns_per_op])),
    overall_delta = deltaFmt(v8rs_data.suite.Score, cpp_data.total_score);

  const ctx = {
    sys,
    cpp_data,
    v8rs_data,
    bin_size_mb,
    cpp_bin_mb,
    reduction_pct,
    overall_delta,
    latency_map,
    v8rs_startup,
    cpp_startup,
    startup_speedup,
    startup_reduction,
  };

  console.log("5. 读取多语言配置并写入独立基准评测文档...");
  const conf_file_li = (await readdir(CONF_DIR)).filter((f) => f.endsWith(".yml")).sort();

  for (const file of conf_file_li) {
    const lang = basename(file, ".yml"),
      yaml_raw = await Bun.file(join(CONF_DIR, file)).text(),
      cfg = Bun.YAML.parse(yaml_raw),
      md_content = benchMdRender(cfg, ctx),
      out_path = join(ROOT_DIR, `readme/${lang}/bench.md`);
    await Bun.write(out_path, md_content);
    console.log(`    已写入 ${out_path}`);
  }

  console.log("6. 运行 bun x mdt . 递归级联渲染...");
  await $({ cwd: ROOT_DIR })`bun x mdt .`;

  console.log("完成: 性能报告与 README 已全量更新。");
};

await run();
