#!/usr/bin/env -S bun

// v8 C++ → Rust 一键转换流水线
import { $ } from "zx";
import { existsSync, mkdirSync, renameSync, writeFile } from "node:fs";
import { join } from "node:path";
import os from "node:os";
import { promisify } from "node:util";
import { preflight } from "./js/preflight.js";

const write_file = promisify(writeFile);

$.verbose = 1;

const root_dir = import.meta.dirname,
  cpp2rust_bin = join(root_dir, "cpp2rust/build/cpp2rust/cpp2rust"),
  nproc = os.cpus().length || 4,
  cmake_arg_li = ["-DCMAKE_BUILD_TYPE=Release"];

$.cwd = root_dir;

// 1. 环境预检（依赖、工具链、LLVM 定位）
const { llvm_prefix } = await preflight();
if (llvm_prefix) {
  cmake_arg_li.push("-DCMAKE_PREFIX_PATH=" + llvm_prefix);
}

// 2. 编译 cpp2rust（Release，避免 Debug 前端导致的转换超时）
if (
  !existsSync(join(root_dir, "cpp2rust/build/Makefile")) &&
  !existsSync(join(root_dir, "cpp2rust/build/build.ninja"))
) {
  await $`cmake -B cpp2rust/build -S cpp2rust ${cmake_arg_li}`;
}

await $`cmake --build cpp2rust/build -j${nproc}`;

if (!existsSync(cpp2rust_bin)) {
  console.error("Error: cpp2rust binary not found at " + cpp2rust_bin);
  process.exit(1);
}

// 3. 初始化 Rust 基础包（若未创建）
if (!existsSync(join(root_dir, "v8_macros"))) {
  await $`./sh/new.sh v8_macros`;
  await $`cargo add --package v8_macros proc-macro2 quote`;
  await $`cargo add --package v8_macros syn@2 --features full,visit-mut,extra-traits`;
  await $`cargo remove --package v8_macros thiserror`.nothrow();
}

if (!existsSync(join(root_dir, "v8rs"))) {
  await $`./sh/new.sh v8rs`;
  await $`cargo add --package v8rs --path ./v8_macros`;
  await $`cargo add --package v8rs libc jiff sprintf nix --features nix/socket,nix/net,nix/fs,nix/poll,nix/time,nix/user,nix/dir,nix/term,nix/event,nix/hostname`;
}

// 4. 同步宏库与运行时（--delete 保持与上游一致，避免残留死文件）
await $`rsync -a --checksum --update --delete cpp2rust/libcc2rs-macros/src/ v8_macros/src/`;

const rt_dir = join(root_dir, "v8rs/src/rt");
mkdirSync(rt_dir, { recursive: true });
await $`rsync -a --checksum --update --delete cpp2rust/libcc2rs/src/ ${rt_dir}/`;

const rt_lib_rs = join(rt_dir, "lib.rs"),
  rt_mod_rs = join(rt_dir, "mod.rs");

if (existsSync(rt_lib_rs)) {
  renameSync(rt_lib_rs, rt_mod_rs);
}

if (existsSync(rt_mod_rs)) {
  const content = await Bun.file(rt_mod_rs).text();
  if (content.includes("libcc2rs_macros")) {
    await write_file(rt_mod_rs, content.replaceAll("libcc2rs_macros", "v8_macros"));
  }
}

// 幂等确保 v8rs 入口声明齐全（仅补缺失，不覆盖手写内容）
const lib_rs = join(root_dir, "v8rs/src/lib.rs"),
  error_rs = join(root_dir, "v8rs/src/error.rs");

if (!existsSync(error_rs)) {
  await write_file(error_rs, 'use thiserror::Error;\n\n#[derive(Error, Debug)]\npub enum Error {\n  #[error("v8rs 未实现")]\n  NotImplemented,\n}\n\npub type Result<T> = std::result::Result<T, Error>;\n');
}

if (existsSync(lib_rs)) {
  let content = await Bun.file(lib_rs).text();
  for (const line of ["mod error;", "pub mod rt;", "pub mod v8;"]) {
    if (!content.includes(line)) content += "\n" + line + "\n";
  }
  await write_file(lib_rs, content);
}

// 5. 扫描并批量转写 C++ 文件（转译 → cargo check → mod.rs + 报告）
await $`./js/conv.js`;

// 6. 格式化（nightly 缺失或失败只警告，不阻断流水线）
try {
  await $`cargo +nightly fmt`;
} catch (e) {
  console.warn("警告: cargo +nightly fmt 失败: " + e.message);
}

await $`cargo check --all-targets`;

// 7. 跑复刻测试
await $`./js/test-converted.js`;

console.log("Conversion process completed successfully.");
