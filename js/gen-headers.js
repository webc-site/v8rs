#!/usr/bin/env -S bun

// 阶段2 生成头构建：在 scratch 工作区（默认 ../v8-gen）内用 gn/ninja
// 构建 v8 的构建期生成头（metagen/instance-types.h、builtins-generated/bytecodes-builtins-list.h、
// torque-generated/*、inspector protocol 等），并拷回 cpp2rust/build/gen/ 供转译 include 使用；
// 同时从 gn 导出的 compile_commands.json 提取权威编译标志写入 v8_cxxflags.txt。
// 用法: ./js/gen-headers.js [ninja目标...]（不带参数时构建默认目标清单）
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root_dir = join(import.meta.dir, ".."),
  gen_ws = process.env.V8_GEN_WS || join(root_dir, "..", "v8-gen"),
  out_dir = process.env.V8_GEN_OUT || "out/Gen",
  v8_dir = join(gen_ws, "v8"),
  src = join(v8_dir, out_dir, "gen"),
  dst = join(root_dir, "cpp2rust", "build", "gen"),
  depot_tools = process.env.DEPOT_TOOLS || "/tmp/depot_tools";

const DEFAULT_TARGETS = [
  "gen/metagen/instance-types.h",
  "gen/builtins-generated/bytecodes-builtins-list.h",
  "run_torque",
  "inspector",
];

const env = {
  ...process.env,
  PATH: `${depot_tools}:${process.env.PATH}`,
  DEPOT_TOOLS_UPDATE: "0",
  DONTELEMETRY: "1",
};

const run = async (cmd_li, cwd) => {
  const proc = Bun.spawn(cmd_li, { cwd, env, stdout: "inherit", stderr: "inherit" });
  const code = await proc.exited;
  if (code !== 0) {
    console.error("失败 (" + code + "): " + cmd_li.join(" "));
    process.exit(code);
  }
};

if (!existsSync(join(gen_ws, ".gclient"))) {
  console.error(`Error: 工作区不存在: ${gen_ws}（需先按 v8/DEPS 完成 gclient sync）`);
  process.exit(1);
}

if (!existsSync(join(v8_dir, out_dir, "build.ninja"))) {
  console.log("gn gen ...");
  await run(["gn", "gen", out_dir, "--args=is_debug=false"], v8_dir);
}

const targets = process.argv.slice(2).length ? process.argv.slice(2) : DEFAULT_TARGETS;
console.log("ninja:", targets.join(" "));
await run(["ninja", "-C", out_dir, ...targets], v8_dir);

mkdirSync(dst, { recursive: true });
await run(["rsync", "-a", "--checksum", src + "/", dst + "/"], root_dir);

// 提取权威编译标志：逐参数一行，相对路径展开为绝对路径
const cc_path = join(v8_dir, out_dir, "compile_commands.json");
if (!existsSync(cc_path)) {
  await run(["gn", "gen", out_dir, "--export-compile-commands"], v8_dir);
}
const cc = JSON.parse(readFileSync(cc_path, "utf8"));
const sample = cc.find((e) => e.file.includes("/src/objects/abstract-code.cc")) || cc[0];
const abs = (p) => (p.startsWith("/") ? p : join(v8_dir, out_dir, p)); // join 会规范化 ../ 段
const PAIRED_VAL = new Set(["-target", "-std", "-arch"]);
// 目录类选项：gn 命令里既有 "-isystem <路径>" 分开写法，也有 "-isystem<路径>" 内联写法；
// 内联判断必须早于 startsWith("-I")，否则 -isystem/-isysroot 会被 -I 分支错误截断。
const INLINE_DIRS = ["-isysroot", "-internal-isystem", "-isystem", "-iquote", "-idirafter", "-iframework"];
const pairs = []; // [选项, 值?] 成对存放，避免按单串去重拆散 "-isystem <路径>" 这类组合
{
  const toks = sample.command.match(/(?:[^\s"']+|"[^"]*"|'[^']*')+/g) || [];
  for (let i = 1; i < toks.length; i++) {
    const t = toks[i];
    if (t === "-c" || t === "-o" || t === "-MD" || t === "-MF" || t === "-MT" || t === "-MMD") {
      i++;
      continue;
    }
    if (t === "-I" || t === "-F" || INLINE_DIRS.includes(t)) pairs.push([t, abs(toks[++i])]);
    else if (PAIRED_VAL.has(t)) pairs.push([t, toks[++i]]);
    else if (t === "-nostdinc++" || t === "-fno-exceptions" || t === "-fno-rtti") pairs.push([t]);
    else if (INLINE_DIRS.some((p) => t.startsWith(p))) {
      const p = INLINE_DIRS.find((q) => t.startsWith(q));
      pairs.push([p, abs(t.slice(p.length))]);
    } else if (t.startsWith("-I") || t.startsWith("-F")) pairs.push([t.slice(0, 2) + abs(t.slice(2))]);
    else if (/^(-D|-U|-std=|--target=|--sysroot|-mmacos-version-min=)/.test(t)) pairs.push([t]);
  }
}
// 不再手工附加 -DABSL_OPTION_USE_STD_FORMAT_EXTENSION=0：gn 原命令并不定义它，
// 强制置 0 会退回 absl 自带的 format 检查器，在 clang 上产生 FormatSpecTemplate<void> unavailable 误报。
const seen = new Set();
const uniq = [];
for (const [k, v] of pairs) {
  const key = v === undefined ? k : k + "\0" + v;
  if (seen.has(key)) continue;
  seen.add(key);
  uniq.push(v === undefined ? k : k + "\n" + v);
}
// 单一样本文件的 -I 集不覆盖全部 target（如 ieee754 需要 third_party/llvm-libc/src）：
// 对所有 compile_commands 条目的包含目录取并集，追加样本里没有的目录。
// 仅并 -I/-F（附加搜索路径只会让解析更宽松），其余标志保持样本原样。
{
  const sample_dirs = new Set();
  for (const [k, v] of pairs) if ((k === "-I" || k === "-F") && v) sample_dirs.add(v);
  const extra = [];
  for (const e of cc) {
    const toks = (e.command || "").match(/(?:[^\s"']+|"[^"]*"|'[^']*')+/g) || [];
    for (let i = 1; i < toks.length; i++) {
      const t = toks[i];
      let dir = null;
      if (t === "-I" || t === "-F") dir = abs(toks[++i]);
      else if (t.startsWith("-I") || t.startsWith("-F")) dir = abs(t.slice(2));
      if (dir && !sample_dirs.has(dir) && !extra.includes(dir)) extra.push(dir);
    }
  }
  for (const d of extra) uniq.push("-I" + d);
}
writeFileSync(join(dst, "v8_cxxflags.txt"), uniq.join("\n") + "\n");
console.log(`生成头已拷入: ${dst}（cxxflags ${uniq.length} 条）`);
