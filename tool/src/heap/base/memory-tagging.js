#!/usr/bin/env bun
// tool/src/heap/base/memory-tagging.js — memory_tagging.rs 初版生成
// 组→文件映射 (本脚本管辖):
//   v8/src/heap/base/memory-tagging.h + memory-tagging.cc → v8-heap-base/src/memory_tagging.rs
// 规则:
//   - MTE 支持条件 (linux + arm64) 机械提取
//   - PSTATE.TCO 置位/复位 asm 骨架; darwin/x64 目标为 no-op
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/heap/base/memory-tagging.h");
const cc = readV8("src/heap/base/memory-tagging.cc");

// ---- 条件提取 ----
const cond = cc.match(/#if defined\((V8_OS_\w+)\) && defined\((V8_HOST_ARCH_\w+)\)/);
if (!cond || cond[1] !== "V8_OS_LINUX" || cond[2] !== "V8_HOST_ARCH_ARM64")
  throw new Error(`MTE 条件应为 linux+arm64, 实得 ${cond}`);
// TCO 指令证据
if (!/msr tco, #1/.test(cc) || !/msr tco, #0/.test(cc)) throw new Error("cc: TCO 置位/复位指令缺失");

const body = `${banner("src/heap/base/memory-tagging.js", "src/heap/base/memory-tagging.h + memory-tagging.cc", "Arm MTE 标签检查挂起域: 仅 linux+arm64 生效, 其余 no-op")}
//! Memory Tagging Extension 辅助: 扫栈期间挂起标签检查。
//! 生效条件 (机械提取): ${cond[1]} && ${cond[2]}。
//! 偏差: 原版读 TCO 校验的 CHECK 裁剪 (禁 panic); CPU::has_mte() →
//! is_aarch64_feature_detected!("mte")。

/// 原 SuspendTagCheckingScope (memory-tagging.h:15-20):
/// 生存期内经 PSTATE.TCO 挂起 MTE 标签检查。
pub struct SuspendTagCheckingScope {
  /// 是否真正动过 TCO (非 MTE 平台/false 时析构不动寄存器)。
  suspended: bool,
}

#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
impl SuspendTagCheckingScope {
  pub fn new() -> Self { todo!("就地修订: has_mte 检测 + msr tco,#1") }
}

#[cfg(not(all(target_os = "linux", target_arch = "aarch64")))]
impl SuspendTagCheckingScope {
  /// 非 linux+arm64: no-op (对应原版 V8_HAS_MTE_SUPPORT 未定义分支)。
  pub fn new() -> Self { Self { suspended: false } }
}

impl Drop for SuspendTagCheckingScope {
  /// 原析构 (memory-tagging.cc:32-44): msr tco,#0 复位。
  fn drop(&mut self) { todo!("就地修订") }
}
`;

const p = writeSrc("v8-heap-base", "memory_tagging.rs", body);
console.log(`memory-tagging.js: ${p} (MTE 条件 ${cond[1]}+${cond[2]} + TCO 指令 2 处核对通过)`);
