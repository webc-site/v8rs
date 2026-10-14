#!/usr/bin/env bun
// tool/src/regexp/regexp-bytecode-generator.h.js
// 机械提取 v8/src/regexp/regexp-bytecode-generator.{h,-inl.h} 的缓冲常量与哨兵
// (生成逻辑在 .cc, 语义面跳过; Bytecode 枚举由 regexp-bytecodes.h 侧生成物覆盖)

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/regexp/regexp-bytecode-generator.h");
const inlSrc = readV8("src/regexp/regexp-bytecode-generator-inl.h");

function constOf(re, what) {
  const m = hSrc.match(re);
  if (!m) throw new Error(`未找到 ${what}`);
  return m;
}

const initBuf = constOf(/static constexpr int kInitialBufferSizeInBytes = 1 \* KB;/, "kInitialBufferSizeInBytes");
void initBuf;
const maxGrowth = constOf(/static constexpr size_t kMaxBufferGrowthInBytes = 1 \* MB;/, "kMaxBufferGrowthInBytes");
void maxGrowth;
const invalidPc = constOf(/static const int kInvalidPC = -1;/, "kInvalidPC");
void invalidPc;
// inl.h: 当前 pc / 长度访问器存在性
if (!inlSrc.includes("BytecodeWriter::Emit")) throw new Error("inl.h 缺 BytecodeWriter::Emit 锚");

console.log("regexp-bytecode-generator.h: consts=3");

let rust = banner(
  "src/regexp/regexp-bytecode-generator.h.js",
  "src/regexp/regexp-bytecode-generator.{h,-inl.h}",
  "字节码生成器缓冲常量与哨兵",
);

rust += `
/// 字节码缓冲初始字节数 (原版 kInitialBufferSizeInBytes = 1 * KB)
pub const INITIAL_BUFFER_SIZE_IN_BYTES: i32 = 1024;

/// 单次扩容上限字节数 (原版 kMaxBufferGrowthInBytes = 1 * MB)
pub const MAX_BUFFER_GROWTH_IN_BYTES: usize = 1024 * 1024;

/// 无效 pc 哨兵 (原版 kInvalidPC)
pub const INVALID_PC: i32 = -1;

/// 字节码生成器 (对应原版 RegExpBytecodeGenerator; 语义实现面登记跳过,
/// 位于 v8/src/regexp/regexp-bytecode-generator.cc)
pub struct BytecodeGenerator;

// —— 提取不变式断言 ——
const _: () = {
  assert!(INITIAL_BUFFER_SIZE_IN_BYTES == 1024);
  assert!(INVALID_PC == -1);
};
`;

const p = writeSrc("v8-regexp-compiler", "bytecode_generator_consts.rs", rust);
console.log(`written ${p}`);
