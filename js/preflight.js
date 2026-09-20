#!/usr/bin/env -S bun

// 转换流水线的环境预检：依赖、工具链、LLVM 定位
import { existsSync } from "node:fs";
import { join } from "node:path";

const root_dir = join(import.meta.dirname, "..");

// 定位 LLVM/Clang 安装前缀，失败返回空串
export const find_llvm = async () => {
  for (const p of ["/opt/homebrew/opt/llvm", "/usr/lib/llvm-23", "/usr/lib/llvm-22"]) {
    if (existsSync(p)) return p;
  }
  const proc = Bun.spawn(["brew", "--prefix", "llvm"], { stdout: "pipe", stderr: "ignore" });
  const out = (await new Response(proc.stdout).text()).trim();
  await proc.exited;
  return out && existsSync(out) ? out : "";
};

const check = async (name, argv) => {
  try {
    const proc = Bun.spawn([...argv], { stdout: "ignore", stderr: "ignore" });
    const code = await proc.exited;
    if (code !== 0) throw new Error(name + " 退出码 " + code);
    return true;
  } catch (e) {
    throw new Error("预检失败: 找不到可用命令 " + name + " (" + e.message + ")");
  }
};

export const preflight = async () => {
  const errors = [];

  try {
    await check("cmake", ["cmake", "--version"]);
  } catch (e) {
    errors.push(e.message);
  }

  try {
    await check("rsync", ["rsync", "--version"]);
  } catch (e) {
    errors.push(e.message);
  }

  try {
    await check("cargo", ["cargo", "--version"]);
  } catch (e) {
    errors.push(e.message);
  }

  // 格式化仅 nightly 支持部分选项，缺失只警告不阻断
  let nightly = false;
  try {
    const proc = Bun.spawn(["rustup", "toolchain", "list"], { stdout: "pipe", stderr: "ignore" });
    nightly = (await new Response(proc.stdout).text()).includes("nightly");
    await proc.exited;
  } catch {}
  if (!nightly) console.warn("警告: 未找到 nightly 工具链，cargo +nightly fmt 将被跳过");

  if (!existsSync(join(root_dir, "v8", "src"))) {
    errors.push("预检失败: v8/src 不存在，请先拉取 v8 源码");
  }

  // v8 的 third_party 依赖与生成头缺失会大量导致转换失败，只警告不阻断（失败会显式进报告）
  const missing = [];
  if (!existsSync(join(root_dir, "v8", "third_party", "abseil-cpp", "absl"))) missing.push("abseil-cpp");
  if (!existsSync(join(root_dir, "v8", "third_party", "googletest", "src", "googletest"))) missing.push("googletest");
  // gn 构建期生成头（metagen/builtins/torque 等）由 ./js/gen-headers.js 拷入 cpp2rust/build/gen
  const gen_headers_dir = join(root_dir, "cpp2rust", "build", "gen");
  if (
    !existsSync(join(gen_headers_dir, "metagen", "instance-types.h")) &&
    !existsSync(join(root_dir, "v8", "src", "builtins", "builtins-generated"))
  )
    missing.push("构建期生成头(需先运行 ./js/gen-headers.js)");
  if (missing.length > 0) {
    console.warn("警告: 依赖缺项: " + missing.join(", ") + " —— 大量 .cc 会因缺头文件转译失败，建议先 gclient sync 并运行 ./js/gen-headers.js");
  }

  const llvm_prefix = await find_llvm();
  if (!llvm_prefix) {
    errors.push("预检失败: 未定位到 LLVM（homebrew llvm 或 /usr/lib/llvm-*），cpp2rust 构建需要它");
  }

  if (!existsSync(join(root_dir, "node_modules", "zx"))) {
    console.log("node_modules 缺失，执行 bun install ...");
    const proc = Bun.spawn(["bun", "install"], { cwd: root_dir, stdout: "inherit", stderr: "inherit" });
    if ((await proc.exited) !== 0) errors.push("预检失败: bun install 失败");
  }

  if (errors.length > 0) {
    for (const msg of errors) console.error(msg);
    process.exit(1);
  }

  return { llvm_prefix };
};

if (import.meta.main) {
  const { llvm_prefix } = await preflight();
  console.log("预检通过, llvm_prefix=" + (llvm_prefix || "(未找到, 仅警告场景)"));
}
