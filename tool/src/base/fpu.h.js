#!/usr/bin/env bun
// tool/src/base/fpu.h.js — 族: src/base/fpu.h + src/base/fpu.cc
// 覆盖: FPU (FTZ/DAZ 刷 Denormal 控制) + FlushDenormalsScope RAII; 四平台分支
// → v8-base/src/fpu.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/base/fpu.h");
const ccSrc = readV8("src/base/fpu.cc");

// —— 锚点断言 ——
if (!hSrc.includes("class FPU final {")) throw new Error("缺 FPU 类");
if (!hSrc.includes("class V8_BASE_EXPORT FlushDenormalsScope final")) throw new Error("缺 RAII Scope");
if (!hSrc.includes("V8_BASE_EXPORT static bool GetFlushDenormals();")) throw new Error("缺 Get 声明");
if (!hSrc.includes("V8_BASE_EXPORT static void SetFlushDenormals(bool);")) throw new Error("缺 Set 声明");
if (!ccSrc.includes("constexpr int kFlushDenormToZeroBits = 0x8040;")) throw new Error("缺 x64 MXCSR 掩码");
if (!ccSrc.includes("constexpr int kFlushDenormToZeroBit = (1 << 24);")) throw new Error("缺 ARM FPCR bit24");
if (!ccSrc.includes('asm volatile("stmxcsr %0"')) throw new Error("缺 stmxcsr 内联汇编");
if (!ccSrc.includes('asm volatile("mrs %x[result], FPCR"')) throw new Error("缺 mrs FPCR 内联汇编");
if (!ccSrc.includes("_controlfp_s(&csr, value ? _DN_FLUSH : _DN_SAVE, _MCW_DN);")) throw new Error("缺 MSVC 分支");
if (!ccSrc.includes("bool FPU::GetFlushDenormals() { return false; }")) throw new Error("缺兜底分支");

// —— 真解析: 平台分支计数 ——
const branches = [
  ["x64/ia32 GNU", /#if defined\(V8_HOST_ARCH_X64\) \|\| defined\(V8_HOST_ARCH_IA32\)/],
  ["MSVC", /#elif defined\(V8_CC_MSVC\)/],
  ["arm64/arm", /#elif defined\(V8_HOST_ARCH_ARM64\) \|\| defined\(V8_HOST_ARCH_ARM\)/],
  ["fallback", /bool FPU::GetFlushDenormals\(\) \{ return false; \}/],
];
for (const [name, re] of branches) if (!re.test(ccSrc)) throw new Error(`缺平台分支 ${name}`);
console.log("fpu: 类 2 (FPU + FlushDenormalsScope), 平台分支 4 (x64 GNU/MSVC, ARM, 兜底), 掩码常量 2");

let rust = banner(
  "src/base/fpu.h.js",
  "src/base/fpu.h + src/base/fpu.cc",
  "Denormal 刷零控制 (FTZ/DAZ) 与 RAII 恢复",
);

rust += `
//! v8: src/base/fpu.h/.cc

pub struct Fpu;

impl Fpu {
    /// v8: src/base/fpu.cc FPU::GetFlushDenormals
    /// x64: stmxcsr & 0x8040 (FTZ|DAZ); arm64: FPCR bit24; 其他平台恒 false
    pub fn get_flush_denormals() -> bool;
    /// v8: src/base/fpu.cc FPU::SetFlushDenormals
    /// 置/清对应位后回写 (ldmxcsr / msr FPCR / _controlfp_s(_MCW_DN))
    pub fn set_flush_denormals(value: bool);
}

// v8: src/base/fpu.cc 常量
// x64 (GNU): kFlushDenormToZeroBits = 0x8040 (FTZ bit15 | DAZ bit6)
// arm:       kFlushDenormToZeroBit  = 1 << 24 (FPCR FZ)
pub const FLUSH_DENORM_TO_ZERO_BITS_X64: u32 = 0x8040;
pub const FLUSH_DENORM_TO_ZERO_BIT_ARM: u32 = 1 << 24;

/// v8: src/base/fpu.h FlushDenormalsScope
/// 构造: 记录 old_flush_state_ 并 Set(value); 析构: 还原 — Rust Drop 对应
pub struct FlushDenormalsScope {
    old_flush_state: bool,
}

impl FlushDenormalsScope {
    pub fn new(value: bool) -> Self;
}
impl Drop for FlushDenormalsScope {
    /// v8: ~FlushDenormalsScope — SetFlushDenormals(old_flush_state_)
    fn drop(&mut self);
}
`;

writeSrc("v8-base", "fpu.rs.skel", rust);
console.log("written v8-base/src/fpu.rs.skel");
