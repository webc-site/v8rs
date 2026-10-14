#!/usr/bin/env bun
// tool/src/wasm/string-builder.h.js
// 机械提取 v8/src/wasm/string-builder.h 与 string-builder-multiline.h 的容量常量

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/string-builder.h");
const mlSrc = readV8("src/wasm/string-builder-multiline.h");

// 1. OnGrowth 枚举
const growthMatch = hSrc.match(/enum OnGrowth : bool \{([^}]*)\};/);
if (!growthMatch) throw new Error("未找到 OnGrowth");
const growths = growthMatch[1]
  .split(",")
  .map((s) => s.trim())
  .filter(Boolean);
if (growths.join(",") !== "kKeepOldChunks,kReplacePreviousChunk") {
  throw new Error(`OnGrowth 内容不符: ${growths}`);
}

// 2. 容量常量
function mustConst(name, src, what) {
  const m = src.match(new RegExp(`static constexpr size_t ${name} = ([0-9 *]+);`));
  if (!m) throw new Error(`未找到 ${what}`);
  return eval(m[1].replace(/\s*\*\s*/g, "*").replace(/^\s*(\d+)$/, "$1"));
}
const stackSize = mustConst("kStackSize", hSrc, "StringBuilder::kStackSize");
const chunkSize = mustConst("kChunkSize", hSrc, "StringBuilder::kChunkSize");
const bufferSize = mustConst("kBufferSize", hSrc, "kBufferSize");
if (stackSize !== 256) throw new Error(`kStackSize 不符: ${stackSize}`);
if (chunkSize !== 1024 * 1024) throw new Error(`kChunkSize 不符: ${chunkSize}`);
if (bufferSize !== 10) throw new Error(`kBufferSize 不符: ${bufferSize}`);

// 3. multiline 的 uint32 缓冲宽
const mlBufMatch = mlSrc.match(/constexpr int kBufSize = (\d+);/);
if (!mlBufMatch) throw new Error("未找到 multiline kBufSize");
const mlBufSize = parseInt(mlBufMatch[1], 10);
if (mlBufSize !== 12) throw new Error(`multiline kBufSize 不符: ${mlBufSize}`);

console.log(`string-builder: stack=${stackSize} chunk=${chunkSize} buffer=${bufferSize} ml=${mlBufSize}`);

let rust = banner(
  "src/wasm/string-builder.h.js",
  "src/wasm/string-builder.h + string-builder-multiline.h",
  "Wasm StringBuilder 容量常量",
);

rust += `
/// 缓冲增长策略 (对应原版 StringBuilder::OnGrowth)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(bool)]
pub enum OnGrowth {
  /// kKeepOldChunks
  KeepOldChunks = false,
  /// kReplacePreviousChunk
  ReplacePreviousChunk = true,
}

/// 栈上内联缓冲字节数 (原版 StringBuilder::kStackSize)
pub const STACK_SIZE: usize = ${stackSize};

/// 堆块字节数 (原版 StringBuilder::kChunkSize)
pub const CHUNK_SIZE: usize = ${chunkSize};

/// uint32 十进制缓冲宽 (原版 kBufferSize; 恰容一个 uint32)
pub const BUFFER_SIZE: usize = ${bufferSize};

/// 多行分隔写入的 uint32 缓冲宽 (原版 string-builder-multiline.h kBufSize; 容 uint32 + '|')
pub const MULTILINE_BUF_SIZE: usize = ${mlBufSize};

// —— 提取不变式断言 ——
const _: () = {
  assert!(STACK_SIZE == 256);
  assert!(CHUNK_SIZE == 1024 * 1024);
  assert!(BUFFER_SIZE == 10);
  assert!(MULTILINE_BUF_SIZE == 12);
};
`;

const p = writeSrc("v8-wasm-core", "string_builder.rs", rust);
console.log(`written ${p}`);
