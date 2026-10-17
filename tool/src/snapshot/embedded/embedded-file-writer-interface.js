#!/usr/bin/env bun
// tool/src/snapshot/embedded/embedded-file-writer-interface.js
// 机械提取 v8/src/snapshot/embedded/embedded-file-writer-interface.h 的
// 默认变体常量与接口面

import { readV8, writeSrc, banner } from "../../../lib.js";

const src = readV8("src/snapshot/embedded/embedded-file-writer-interface.h");

// 1. 默认嵌入变体常量
const variantM = src.match(/static constexpr char kDefaultEmbeddedVariant\[\] = "(\w+)";/);
if (!variantM) throw new Error("未找到 kDefaultEmbeddedVariant");

// 2. 接口方法面 (纯虚)
const clsM = src.match(/class EmbeddedFileWriterInterface \{([\s\S]*?)\n\};/);
if (!clsM) throw new Error("未找到 EmbeddedFileWriterInterface 类体");
const methods = [...clsM[1].matchAll(/virtual\s+(?:int|const char\*|void)\s+(\w+)\(/g)].map((m) => m[1]);
if (methods.length !== 5) {
  throw new Error(`接口方法数不符: 期望 5, 实得 ${methods.length}: ${methods.join(",")}`);
}
// 3. V8_OS_WIN64 分支
const winBranch = src.includes("#if defined(V8_OS_WIN64)");
if (!winBranch) throw new Error("未找到 V8_OS_WIN64 分支");
console.log(`常量 1 + 接口方法 ${methods.length}`);

let rust = banner(
  "src/snapshot/embedded/embedded-file-writer-interface.js",
  "src/snapshot/embedded/embedded-file-writer-interface.h",
  "嵌入文件写入器接口",
);
rust += `
/// 默认嵌入变体名 (原版 kDefaultEmbeddedVariant)
pub const DEFAULT_EMBEDDED_VARIANT: &str = "${variantM[1]}";

/// EmbeddedFileWriterInterface (原版纯虚接口, isolate 编译期内建源码位置登记):
pub trait EmbeddedFileWriterInterface {
`;
for (const m of methods) rust += `  /// 原版: \`${m}\`\n`;
rust += `  fn lookup_or_add_externally_compiled_filename(&mut self, filename: &str) -> i32;
  fn get_externally_compiled_filename(&self, index: i32) -> &str;
  fn get_externally_compiled_filename_count(&self) -> i32;
  /// isolate 在以内建跳板替换已编译 InstructionStream 前调用
  fn prepare_builtin_source_position_map(&mut self);
}

/// LabelInfo (原版 struct): 偏移 + 符号名, 由 Developer 落型。
/// 分支登记: #if defined(V8_OS_WIN64) — SetBuiltinUnwindData 仅 Win64 构建。
`;

const p = writeSrc("v8-snapshot", "embedded_file_writer_interface.rs", rust);
console.log(`written ${p}`);
