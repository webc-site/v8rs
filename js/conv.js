#!/usr/bin/env -S bun

// 流水线主体：阶段1 转译 → 阶段2 cargo check 剔除 → 阶段3 报告
// 失败一律显式记录（meta + 日志 + 报告），绝不静默丢弃
import { existsSync, mkdirSync, readdirSync, readFileSync, statSync, unlinkSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import os from "node:os";
import { Glob } from "bun";
import { SingleBar, Presets } from "cli-progress";
import { check_and_filter, gen_dir, build_dir, root_dir } from "./check-modules.js";
import { build_report } from "./report.js";

const v8_src_dir = join(root_dir, "v8", "src"),
  cpp2rust = join(build_dir, "cpp2rust", "cpp2rust"),
  rules_dir = join(build_dir, "rules"),
  logs_dir = join(build_dir, "logs"),
  // 权威编译标志（由 ./js/gen-headers.js 从 gn compile_commands.json 导出，逐参数一行）
  gn_flags_path = join(build_dir, "gen", "v8_cxxflags.txt"),
  gn_flags = existsSync(gn_flags_path)
    ? readFileSync(gn_flags_path, "utf8").split("\n").filter(Boolean)
    : null,
  meta_path = (mod) => join(gen_dir, mod + ".rs.meta.json"),
  run_stats = join(build_dir, "run-stats.json"),
  cxxflag_li = gn_flags
    ? gn_flags
    : [
        // 回退：无 scratch 工作区时的手工路径集（覆盖度受限，报告如实呈现失败）
        "-I" + join(root_dir, "v8"),
        "-I" + join(root_dir, "v8", "include"),
        "-I" + join(root_dir, "v8", "third_party", "abseil-cpp"),
        "-I" + join(root_dir, "v8", "third_party", "googletest", "custom"),
        "-I" + join(root_dir, "v8", "third_party", "googletest", "src", "googletest", "include"),
        "-I" + join(root_dir, "v8", "third_party", "googletest", "src", "googlemock", "include"),
        "-I" + join(root_dir, "v8", "third_party", "highway", "src"),
        "-I" + join(root_dir, "v8", "third_party", "simdutf"),
        "-I" + join(root_dir, "v8", "third_party", "fp16", "src", "include"),
        "-I" + join(build_dir, "gen"),
        "-std=c++20",
        "-DV8_ENABLE_CHECKS",
      ],
  concurrency = os.cpus().length || 8,
  timeout_ms = Number(process.env.CONV_TIMEOUT || 120000),
  limit = Number(process.env.CONV_LIMIT || 0);

if (!existsSync(cpp2rust)) {
  console.error("Error: cpp2rust binary not found at " + cpp2rust);
  process.exit(1);
}
if (!existsSync(rules_dir)) {
  console.error("Error: rules 目录不存在: " + rules_dir + "（应由 cmake 构建生成）");
  process.exit(1);
}

mkdirSync(gen_dir, { recursive: true });
mkdirSync(logs_dir, { recursive: true });

// 指纹：任一输入变化即重转
const walk_fingerprint = (dir, acc) => {
    for (const name of readdirSync(dir)) {
      const p = join(dir, name);
      if (statSync(p).isDirectory()) walk_fingerprint(p, acc);
      else {
        const st = statSync(p);
        acc.push(name + ":" + st.size + ":" + st.mtimeMs);
      }
    }
    return acc;
  },
  sha256 = (data) => new Bun.CryptoHasher("sha256").update(data).digest("hex"),
  bin_sha = sha256(readFileSync(cpp2rust)),
  rules_fp = sha256(walk_fingerprint(rules_dir, []).sort().join("\n")),
  flags_fp = sha256(cxxflag_li.join(" "));

const read_meta = (mod) => {
  try {
    return JSON.parse(readFileSync(meta_path(mod), "utf8"));
  } catch {
    return null;
  }
};

const glob = new Glob("**/*.cc"),
  all_files = Array.from(glob.scanSync({ cwd: v8_src_dir })).sort(),
  file_li = limit > 0 ? all_files.slice(0, limit) : all_files,
  is_tty = process.stdout.isTTY;

console.log(
  "Found " + all_files.length + " C++ files in v8/src" +
  (limit > 0 ? "，CONV_LIMIT=" + limit + " 抽样转译前 " + file_li.length + " 个" : "")
);

let bar = null;
if (is_tty) {
  bar = new SingleBar(
    {
      format:
        "Translating [{bar}] {percentage}% | ETA: {eta}s | {value}/{total} | {file}",
      barCompleteChar: "\u2588",
      barIncompleteChar: "\u2591",
      hideCursor: true,
    },
    Presets.shades_classic
  );
  bar.start(file_li.length, 0, { file: "starting..." });
}

let skipped = 0;

const convertFile = async (rel_path) => {
  const full_path = join(v8_src_dir, rel_path),
    mod_name = rel_path.replace(/\.cc$/, "").replaceAll(/[\/\.-]/g, "_"),
    out_file = join(gen_dir, mod_name + ".rs"),
    src_mtime = statSync(full_path).mtimeMs,
    meta = read_meta(mod_name);

  // 增量：ok 与 unsupported 都是同指纹下的确定性结果（unsupported 重跑只是
  // 再次撞到同一条不可转译语句），跳过；crash/error/timeout 可能与环境相关，重试
  if (
    meta &&
    (meta.state === "ok" || meta.state === "unsupported") &&
    meta.src_mtime === src_mtime &&
    meta.bin_sha === bin_sha &&
    meta.rules_fp === rules_fp &&
    meta.flags_fp === flags_fp &&
    (meta.state === "unsupported" || existsSync(out_file))
  ) {
    skipped++;
    // ok 才进入 stage-2 候选；unsupported 无产物可检
    return [meta.state === "ok", mod_name, true];
  }

  const tmp_out = join(os.tmpdir(), mod_name + "_" + Date.now() + "_" + Math.random().toString(36).slice(2) + ".rs"),
    arg_li = [
      cpp2rust,
      "--file=" + full_path,
      "-o=" + tmp_out,
      "--rules=" + rules_dir,
      ...cxxflag_li.map((f) => "--cxxflags=" + f),
    ];

  const write_fail_meta = (state, error) => {
    writeFileSync(
      meta_path(mod_name),
      JSON.stringify({ file: rel_path, mod: mod_name, state, src_mtime, bin_sha, rules_fp, flags_fp, error }, null, 2)
    );
    try {
      if (existsSync(tmp_out)) unlinkSync(tmp_out);
    } catch {}
    // 失败不残留伪成功产物
    try {
      if (existsSync(out_file)) unlinkSync(out_file);
    } catch {}
  };

  let proc, timer;
  try {
    proc = Bun.spawn(arg_li, { stdout: "ignore", stderr: "pipe" });
    const timed_out = await Promise.race([
      proc.exited.then(() => false),
      new Promise((resolve) => {
        timer = setTimeout(() => resolve(true), timeout_ms);
      }),
    ]);
    if (timed_out) {
      try {
        proc.kill(9);
      } catch {}
      await proc.exited;
      write_fail_meta("timeout", "转换超时 (" + timeout_ms + "ms)");
      return [false, mod_name, false];
    }
    clearTimeout(timer);
    const stderr = await new Response(proc.stderr).text();

    if (proc.exitCode === 0 && existsSync(tmp_out)) {
      let code = readFileSync(tmp_out, "utf8");
      code = code
        .replaceAll(/extern crate libcc2rs;\s*/g, "")
        .replaceAll(/use libcc2rs::\*;\s*/g, "")
        .replaceAll("libcc2rs::", "");
      writeFileSync(out_file, "use crate::*;\n" + code);
      unlinkSync(tmp_out);
      if (stderr.trim()) writeFileSync(join(logs_dir, mod_name + ".log"), stderr);
      writeFileSync(
        meta_path(mod_name),
        JSON.stringify({ file: rel_path, mod: mod_name, state: "ok", src_mtime, bin_sha, rules_fp, flags_fp, error: "" }, null, 2)
      );
      return [true, mod_name, false];
    }

    writeFileSync(join(logs_dir, mod_name + ".log"), stderr);
    // 三分类：UNSUPPORTED（exit=3，语句确实未转译）≠ crash（信号杀死，转换器 bug）
    // ≠ error（其他非零退出）。全部显式入报告，绝不静默丢弃
    if (proc.exitCode === null) {
      write_fail_meta("crash", "cpp2rust 崩溃，信号: " + (proc.signalCode || "unknown"));
      return [false, mod_name, false];
    }
    if (proc.exitCode === 3 || stderr.includes("UNSUPPORTED:")) {
      const line = stderr.split("\n").find((l) => l.startsWith("UNSUPPORTED:"));
      write_fail_meta("unsupported", (line || "UNSUPPORTED (exit=3 但无标记行)").slice(13).trim());
      return [false, mod_name, false];
    }
    write_fail_meta("error", (stderr.trim().split("\n").find((l) => l.includes("error")) || "cpp2rust 退出码 " + proc.exitCode));
    return [false, mod_name, false];
  } catch (e) {
    clearTimeout(timer);
    write_fail_meta("error", "启动 cpp2rust 失败: " + e.message);
    return [false, mod_name, false];
  }
};

// ---------- 阶段 1: 转译 ----------
let file_index = 0,
  processed_count = 0;

const ok_mods = new Map();

const worker = async () => {
  while (file_index < file_li.length) {
    const i = file_index++,
      file = file_li[i],
      [ok, mod_name, was_skipped] = await convertFile(file);

    if (ok) ok_mods.set(mod_name, file);

    const count = ++processed_count;
    if (bar) {
      bar.increment(1, { file: file.length > 35 ? "..." + file.slice(-32) : file });
    } else if (count % 25 === 0 || count === file_li.length || !ok) {
      const pct = ((count / file_li.length) * 100).toFixed(1);
      console.log("[" + count + "/" + file_li.length + " " + pct + "%] " + file + (ok ? " (ok)" : " (FAIL)"));
    }
  }
};

await Promise.all(Array.from({ length: concurrency }, () => worker()));
if (bar) bar.stop();

writeFileSync(
  run_stats,
  JSON.stringify(
    { total_found: all_files.length, attempted: file_li.length, skipped_up_to_date: skipped, converted_ok: ok_mods.size },
    null,
    2
  )
);

// ---------- 阶段 2: cargo check 显式剔除 ----------
// 候选 = 本次 ok/跳过的 + 产物目录中现存的全部模块（含历史遗留，未过检者会被显式剔除）
const candidates = new Set(ok_mods.keys());
for (const f of readdirSync(gen_dir)) {
  if (f.endsWith(".rs") && f !== "mod.rs") candidates.add(f.replace(/\.rs$/, ""));
}

console.log("Stage 2: cargo check " + candidates.size + " 个转译成功模块 ...");
const { kept, dropped } = await check_and_filter(Array.from(candidates).sort());
writeFileSync(
  join(build_dir, "check-result.json"),
  JSON.stringify({ kept, dropped: Object.fromEntries(dropped) }, null, 2)
);

// ---------- 阶段 3: 报告 ----------
const report = build_report();
console.log(
  "Stage 3: 报告已生成 cpp2rust/build/conversion-report.md\n" +
  "  转译成功且过检 " + report.stats.ok_kept +
  " / 编译剔除 " + report.stats.check_dropped +
  " / 未转译(unsupported) " + report.stats.unsupported +
  " / 超时 " + report.stats.timeout +
  " / 失败 " + report.stats.error +
  " / 崩溃 " + report.stats.crash +
  " / 覆盖率 " + report.coverage
);
console.log("Pipeline completed.");
