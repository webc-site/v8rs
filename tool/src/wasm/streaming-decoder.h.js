#!/usr/bin/env bun
// tool/src/wasm/streaming-decoder.h.js
// 机械提取 v8/src/wasm/streaming-decoder.{h,cc} 的流状态枚举与头部布局常量

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/streaming-decoder.h");
const ccSrc = readV8("src/wasm/streaming-decoder.cc");

// 1. StreamState 枚举
const stateMatch = hSrc.match(/enum StreamState \{([^}]*)\};/);
if (!stateMatch) throw new Error("未找到 StreamState");
const states = stateMatch[1]
  .split(",")
  .map((s) => s.trim())
  .filter(Boolean);
if (states.join(",") !== "kReceivingBytes,kAborted,kFinished,kDiscarded") {
  throw new Error(`StreamState 内容不符: ${states}`);
}

// 2. cc 侧模块头大小
const headerSizeMatch = ccSrc.match(/static constexpr size_t kModuleHeaderSize = (\d+);/);
if (!headerSizeMatch) throw new Error("未找到 kModuleHeaderSize");
const moduleHeaderSize = parseInt(headerSizeMatch[1], 10);
if (moduleHeaderSize !== 8) throw new Error(`kModuleHeaderSize 不符: ${moduleHeaderSize}`);

console.log(`streaming-decoder: StreamState=${states.length} moduleHeaderSize=${moduleHeaderSize}`);

let rust = banner(
  "src/wasm/streaming-decoder.h.js",
  "src/wasm/streaming-decoder.{h,cc}",
  "流式解码状态机与模块头布局",
);

rust += `
/// 流式解码器状态 (对应原版 AsyncStreamingDecoder::StreamState)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum StreamState {
  /// kReceivingBytes — 正在接收字节
  ReceivingBytes = 0,
  /// kAborted — 已中止
  Aborted = 1,
  /// kFinished — 已完成
  Finished = 2,
  /// kDiscarded — 已丢弃
  Discarded = 3,
}

/// 模块头大小 (原版 streaming-decoder.cc kModuleHeaderSize; magic + version)
pub const MODULE_HEADER_SIZE: usize = ${moduleHeaderSize};

// —— 提取不变式断言 ——
const _: () = {
  assert!(MODULE_HEADER_SIZE == 8);
  assert!(StreamState::Discarded as u8 == 3);
};
`;

const p = writeSrc("v8-wasm-core", "streaming_decoder_consts.rs", rust);
console.log(`written ${p}`);
