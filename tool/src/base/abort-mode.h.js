#!/usr/bin/env bun
// tool/src/base/abort-mode.h.js — 族: src/base/abort-mode.h + src/base/abort-mode.cc
// 覆盖: 枚举 AbortMode (5 值) + 全局 g_abort_mode 初值 + 3 个判定谓词签名
// → v8-base/src/abort_mode.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/base/abort-mode.h");
const ccSrc = readV8("src/base/abort-mode.cc");

// —— 锚点断言: 缺即 throw ——
if (!hSrc.includes("enum class AbortMode {")) throw new Error("abort-mode.h 缺 AbortMode 枚举");
if (!hSrc.includes("V8_BASE_EXPORT extern AbortMode g_abort_mode;")) throw new Error("缺 g_abort_mode 声明");
if (!ccSrc.includes("AbortMode g_abort_mode = AbortMode::kDefault;")) throw new Error("缺 g_abort_mode 默认初值");
for (const fn of [
  "bool ControlledCrashesAreHarmless()",
  "bool DcheckFailuresAreIgnored()",
  "bool FatalErrorsWithNoSecurityImpactShouldExit()",
]) {
  if (!hSrc.includes(fn)) throw new Error(`缺谓词签名 ${fn}`);
}

// —— 真解析: 提取枚举成员, 对数 ——
const enumBody = hSrc.match(/enum class AbortMode \{([\s\S]*?)\};/)[1];
const variants = [...enumBody.matchAll(/^\s{2}(k\w+),?$/gm)].map((m) => m[1]);
if (variants.length !== 5) throw new Error(`AbortMode 枚举数 ${variants.length} != 5`);
console.log(`abort-mode: AbortMode 枚举 ${variants.length} 值, 谓词 3, 全局 1`);

let rust = banner(
  "src/base/abort-mode.h.js",
  "src/base/abort-mode.h + src/base/abort-mode.cc",
  "abort 处理模式枚举与全局开关",
);

rust += `
//! v8: src/base/abort-mode.h — abort 模式 (OS::Abort 与 DCHECK 行为)

/// v8: src/base/abort-mode.h AbortMode
/// 原版为 enum class, 无显式赋值, 序数 0..=4
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbortMode {
${variants.map((v) => `    ${v},`).join("\n")}
}

/// v8: src/base/abort-mode.cc g_abort_mode
/// 原版初值 kDefault; Rust 侧由 Developer 定原子/线程模型
pub static G_ABORT_MODE: std::sync::atomic::AtomicU8 =
    std::sync::atomic::AtomicU8::new(AbortMode::KDefault as u8);

/// v8: src/base/abort-mode.h ControlledCrashesAreHarmless
pub fn controlled_crashes_are_harmless() -> bool;

/// v8: src/base/abort-mode.h DcheckFailuresAreIgnored
pub fn dcheck_failures_are_ignored() -> bool;

/// v8: src/base/abort-mode.h FatalErrorsWithNoSecurityImpactShouldExit
pub fn fatal_errors_with_no_security_impact_should_exit() -> bool;
`;

writeSrc("v8-base", "abort_mode.rs.skel", rust);
console.log("written v8-base/src/abort_mode.rs.skel");
