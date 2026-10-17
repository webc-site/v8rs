#!/usr/bin/env bun
// tool/src/base/build_config.h.js — 族: src/base/build_config.h
// 覆盖: 指令集能力宏 (ARMv7/8, SDIV, VFP3) + JIT 写保护开关 3 + 页大小常量 5 分支
// → v8-base/src/build_config.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/base/build_config.h");

// —— 锚点断言 ——
const anchors = [
  "#define CAN_USE_ARMV7_INSTRUCTIONS 1",
  "#define CAN_USE_SUDIV 1",
  "#define CAN_USE_ARMV8_INSTRUCTIONS 1",
  "#define CAN_USE_VFP3_INSTRUCTIONS 1",
  "#define V8_HAS_PKU_SUPPORT",
  "#define V8_HAS_PTHREAD_JIT_WRITE_PROTECT",
  "#define V8_HAS_BECORE_JIT_WRITE_PROTECT",
  "constexpr int kReturnAddressStackSlotCount =",
  "constexpr int kPageSizeBits = 18;",
  "constexpr int kRegularPageSize = 1 << kPageSizeBits;",
  "constexpr int kMinimumOSPageSize = 16 * 1024;",
  "constexpr int kMinimumOSPageSize = 64 * 1024;",
  "constexpr int kMinimumOSPageSize = 4 * 1024;",
];
for (const a of anchors) if (!src.includes(a)) throw new Error(`build_config.h 缺锚点: ${a}`);

// —— 真解析: kPageSizeBits 三分支取值 ——
const pageBitsBranches = [...src.matchAll(/constexpr int kPageSizeBits = (\d+);/g)].map((m) => Number(m[1]));
if (pageBitsBranches.join(",") !== "19,18") throw new Error(`kPageSizeBits 分支 ${pageBitsBranches} != [19(PPC64/hugapage 另计),18]`);
// hugapage 分支: kHugePageBits=21, kPageSizeBits=kHugePageBits
if (!src.includes("constexpr int kHugePageBits = 21;")) throw new Error("缺 kHugePageBits");
if (!src.includes("constexpr int kPageSizeBits = kHugePageBits;")) throw new Error("缺 hugepage kPageSizeBits 别名分支");
// 最小 OS 页三档
const minPages = [...src.matchAll(/constexpr int kMinimumOSPageSize = ([\d *]+);/g)].map((m) => m[1].replace(/\s+/g, ""));
if (minPages.join(",") !== "16*1024,64*1024,4*1024") throw new Error(`kMinimumOSPageSize 档 ${minPages}`);
// 写保护宏真值分支 (0/1 各出现)
const pkuFalse = src.includes("#define V8_HAS_PKU_SUPPORT 0");
if (!pkuFalse) throw new Error("缺 V8_HAS_PKU_SUPPORT 0 分支");
console.log("build_config: 能力宏 4, JIT 写保护开关 3, kPageSizeBits 分支 3 (19/21/18), kMinimumOSPageSize 档 3");

let rust = banner(
  "src/base/build_config.h.js",
  "src/base/build_config.h",
  "构建期配置常量 (架构指令集/写保护/页大小)",
);

rust += `
//! v8: src/base/build_config.h — 构建 cfg 常量, Rust 侧用 cfg!/const 表达

// —— 指令集能力 (仅 ARM 系构建可见; Rust 侧 cfg(target_feature)) ——
// v8: CAN_USE_ARMV7_INSTRUCTIONS — __ARM_ARCH_7A__/7R/7 或 8A
// v8: CAN_USE_SUDIV — __ARM_ARCH_EXT_IDIV__ 或 ARMv8A
// v8: CAN_USE_ARMV8_INSTRUCTIONS — __ARM_ARCH_8A__
// v8: CAN_USE_VFP3_INSTRUCTIONS — ARMv7/8 默认 1 (除非外部已定义)

// —— JIT 写保护开关 ——
/// v8: V8_HAS_PKU_SUPPORT — 仅 x64 Linux 为 1; V8_HAS_PKU_JIT_WRITE_PROTECT 恒等于它
pub const V8_HAS_PKU_SUPPORT: bool = cfg!(all(target_os = "linux", target_arch = "x86_64"));
/// v8: V8_HAS_PTHREAD_JIT_WRITE_PROTECT — 仅 arm64 macOS 为 1
pub const V8_HAS_PTHREAD_JIT_WRITE_PROTECT: bool = cfg!(all(target_os = "macos", target_arch = "aarch64"));
/// v8: V8_HAS_BECORE_JIT_WRITE_PROTECT — arm64 iOS 17.4+; Rust 侧登记跳过 (无等价 cfg)
pub const V8_HAS_BECORE_JIT_WRITE_PROTECT: bool = false;

/// v8: src/base/build_config.h V8_TARGET_ARCH_STORES_RETURN_ADDRESS_ON_STACK
/// ia32/x64 为 true, 其余 false; kReturnAddressStackSlotCount = true ? 1 : 0
pub const RETURN_ADDRESS_STACK_SLOT_COUNT: usize = if cfg!(any(target_arch = "x86", target_arch = "x86_64")) { 1 } else { 0 };

/// v8: src/base/build_config.h kPageSizeBits
/// 分支: PPC64 非 AIX → 19; ENABLE_HUGEPAGE → kHugePageBits(21); 默认 (含 arm64) → 18
/// v8: kRegularPageSize = 1 << kPageSizeBits
pub const PAGE_SIZE_BITS: u32 = ${pageBitsBranches[1]};
pub const REGULAR_PAGE_SIZE: usize = 1 << PAGE_SIZE_BITS;

/// v8: src/base/build_config.h kMinimumOSPageSize
/// 档位: macOS/arm64·Android(arm64,x64)·LOONG64·MIPS64·iOS → 16KB;
///       Linux(非 Android) arm64/PPC64 → 64KB; 其余 → 4KB
pub const MINIMUM_OS_PAGE_SIZE: usize = 4 * 1024; // Developer: 按 target cfg 三档展开

/// v8: kHugePageBits = 21; kHugePageSize = 1 << 21 (仅 ENABLE_HUGEPAGE 构建生效)
pub const HUGE_PAGE_BITS: u32 = 21;
pub const HUGE_PAGE_SIZE: usize = 1 << HUGE_PAGE_BITS;
`;

writeSrc("v8-base", "build_config.rs.skel", rust);
console.log("written v8-base/src/build_config.rs.skel");
