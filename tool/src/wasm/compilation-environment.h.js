#!/usr/bin/env bun
// tool/src/wasm/compilation-environment.h.js
// 机械提取 v8/src/wasm/compilation-environment.h 的编译事件枚举与回调释放策略

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/compilation-environment.h");

// 1. CompilationEvent
const evMatch = hSrc.match(/enum class CompilationEvent : uint8_t \{([\s\S]*?)\n\};/);
if (!evMatch) throw new Error("未找到 CompilationEvent");
const events = evMatch[1]
  .split(",")
  .map((s) => s.trim())
  .filter(Boolean);
if (events.join(",") !== "kFinishedBaselineCompilation,kFinishedCompilationChunk,kFailedCompilation") {
  throw new Error(`CompilationEvent 内容不符: ${events}`);
}

// 2. ReleaseAfterFinalEvent
const relMatch = hSrc.match(/enum ReleaseAfterFinalEvent : bool \{\s*kReleaseAfterFinalEvent = true,\s*kKeepAfterFinalEvent = false\s*\};/s);
if (!relMatch) throw new Error("未找到 ReleaseAfterFinalEvent");

// 3. WireBytesStorage 接口存在性 (骨架登记)
if (!/class WireBytesStorage \{/.test(hSrc)) throw new Error("未找到 WireBytesStorage");

console.log(`compilation-environment.h: CompilationEvent=${events.length}`);

let rust = banner(
  "src/wasm/compilation-environment.h.js",
  "src/wasm/compilation-environment.h",
  "编译状态事件与回调释放策略",
);

rust += `
/// 编译状态事件 (对应原版 enum class CompilationEvent : uint8_t)
/// 回调只会收到 kFinishedBaselineCompilation 或 kFinishedCompilationChunk
/// 与 kFailedCompilation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CompilationEvent {
  /// kFinishedBaselineCompilation — 基线编译完成
  FinishedBaselineCompilation = 0,
  /// kFinishedCompilationChunk — 流式编译块完成
  FinishedCompilationChunk = 1,
  /// kFailedCompilation — 编译失败
  FailedCompilation = 2,
}

/// 最终事件后是否释放回调 (对应原版 CompilationEventCallback::ReleaseAfterFinalEvent)
pub type ReleaseAfterFinalEvent = bool;

/// 最终事件后释放回调 (默认行为)
pub const RELEASE_AFTER_FINAL_EVENT: ReleaseAfterFinalEvent = true;

/// 最终事件后保留回调 (动态 tier-up 的 code caching 回调需要)
pub const KEEP_AFTER_FINAL_EVENT: ReleaseAfterFinalEvent = false;

/// wire bytes 存储位置抽象 (原版 WireBytesStorage 接口; 语义面待 Developer 修订)
pub trait WireBytesStorage {
  /// 原版 GetCode: 返回 [ref 偏移, 长度) 对应的字节
  fn get_code(&self, offset: u32, length: u32) -> &[u8];
  /// 原版 GetModuleBytes: 流式编译期不可用, 返回 None
  fn get_module_bytes(&self) -> Option<&[u8]>;
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(CompilationEvent::FailedCompilation as u8 == 2);
  assert!(RELEASE_AFTER_FINAL_EVENT);
  assert!(!KEEP_AFTER_FINAL_EVENT);
};
`;

const p = writeSrc("v8-wasm-core", "compilation_environment_consts.rs", rust);
console.log(`written ${p}`);
