#!/usr/bin/env bun
// tool/src/codegen/cpu-features.h.js
// 机械提取 v8/src/codegen/cpu-features.h 的 CpuFeature 枚举 (x64+arm64 分支展开,
// 其余架构登记) + CpuFeatures 静态面骨架; 附 flush-instruction-cache.{h,cc} 签名
// → v8-codegen-core/src/cpu_features.rs + flush_instruction_cache.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/codegen/cpu-features.h");
const flushH = readV8("src/codegen/flush-instruction-cache.h");
const flushCc = readV8("src/codegen/flush-instruction-cache.cc");

// 按平台分支切出枚举体: #if/#elif V8_TARGET_ARCH_X (…list…) 至下一个分支
function branchList(marker) {
  const esc = marker.replace(/\|/g, "\\|");
  const re = new RegExp(
    `(?:#if|#elif) V8_TARGET_ARCH_${esc}\\b([\\s\\S]*?)(?=#elif|#endif)`,
  );
  const m = src.match(re);
  if (!m) throw new Error(`未找到 ${marker} 分支`);
  return m[1]
    .split("\n")
    .map((l) => l.replace(/\/\/.*$/, "").trim())
    .filter((l) => l && !l.startsWith("#") && l !== ",")
    .join("\n");
}

// 枚举项: name 或 alias = value
function parseEntries(body) {
  return body
    .split(",")
    .map((s) => s.trim())
    .filter(Boolean)
    .map((e) => {
      const m = e.match(/^(\w+)\s*=\s*(\w+)$/);
      return m ? { name: m[1], alias: m[2] } : { name: e };
    });
}

const x64Body = branchList("IA32 || V8_TARGET_ARCH_X64");
const arm64Body = branchList("ARM64");
const x64 = parseEntries(x64Body);
const arm64 = parseEntries(arm64Body);

// 对数 (原版逐项): x64/ia32 20 项, arm64 11 项
const EXPECT_X64 = 20;
const EXPECT_ARM64 = 11;
if (x64.length !== EXPECT_X64) throw new Error(`x64 CpuFeature ${x64.length} != ${EXPECT_X64}`);
if (arm64.length !== EXPECT_ARM64) throw new Error(`arm64 CpuFeature ${arm64.length} != ${EXPECT_ARM64}`);
if (x64[0].name !== "SSE4_2" || x64.at(-1).name !== "AVX10_1") throw new Error("x64 特性表首尾不符");
if (arm64[0].name !== "JSCVT" || arm64.at(-1).name !== "SVEBITPERM") throw new Error("arm64 特性表首尾不符");

function snake(s) {
  return s.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase();
}

// flush-instruction-cache 对数
if (!flushH.includes("FlushInstructionCache")) throw new Error("flush-instruction-cache.h 缺 FlushInstructionCache");
const flushCcSigs = [...flushCc.matchAll(/^void FlushInstructionCache\(/gm)].length;
if (flushCcSigs < 1) throw new Error("flush-instruction-cache.cc 缺 FlushICache");

let rust = banner(
  "src/codegen/cpu-features.h.js",
  "src/codegen/cpu-features.h;src/codegen/flush-instruction-cache.{h,cc}",
  "CpuFeature 枚举 (x64/arm64 展开) + CpuFeatures 静态面骨架 + FlushInstructionCache 签名",
);

rust += `
// 平台分支: x64 与 arm64 两目标分别展开为独立枚举;
// arm/ia32/mips64/loong64/ppc/s390/riscv 分支未采用, 在此登记跳过。

/// x64/ia32 目标 CpuFeature (原版 #if V8_TARGET_ARCH_IA32 || X64 分支, ${x64.length} 项)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum CpuFeatureX64 {
${x64.map((e, i) => `  /// ${e.name}\n  ${snake(e.name)} = ${i},`).join("\n")},
  /// NUMBER_OF_CPU_FEATURES 哨兵
  NUMBER_OF_CPU_FEATURES = ${x64.length},
}

/// arm64 目标 CpuFeature (原版 #elif V8_TARGET_ARCH_ARM64 分支, ${arm64.length} 项)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum CpuFeatureArm64 {
${arm64.map((e, i) => `  /// ${e.name}\n  ${snake(e.name)} = ${i},`).join("\n")},
  /// NUMBER_OF_CPU_FEATURES 哨兵
  NUMBER_OF_CPU_FEATURES = ${arm64.length},
}

/// 原版 static_assert(NUMBER_OF_CPU_FEATURES <= kBitsPerInt) 的 u32 载体前提

/// CpuFeatures 静态面骨架 (AllStatic; 支持集必须先 Probe 再用):
/// 例: if assembler.is_supported(SSE3) { CpuFeatureScope ... }
pub struct CpuFeatures;

impl CpuFeatures {
  /// 原版 Probe(bool cross_compile): 幂等初始化; ProbeImpl 平台实现待 Developer
  pub fn probe(cross_compile: bool) {
    let _ = cross_compile;
    todo!("待 Developer 修订: 对齐 ProbeImpl 平台实现")
  }

  /// 原版 IsSupported(f) — supported_ 位集待接线; 骨架
  #[must_use]
  pub fn is_supported(_f: u8) -> bool {
    todo!("待 Developer 修订: supported_ EnumSet 接线")
  }

  /// 原版 SetSupported / SetUnsupported (骨架)
  pub fn set_supported(_f: u8) {
    todo!("待 Developer 修订")
  }

  pub fn set_unsupported(_f: u8) {
    todo!("待 Developer 修订")
  }

  /// 原版 SupportsSimd128 (平台相关, 骨架)
  #[must_use]
  pub fn supports_simd128() -> bool {
    todo!("待 Developer 修订")
  }

  /// 原版 SupportsOptimizer (平台相关, 骨架)
  #[must_use]
  pub fn supports_optimizer() -> bool {
    todo!("待 Developer 修订")
  }

  /// 原版 icache_line_size()
  #[must_use]
  pub fn icache_line_size() -> u32 {
    todo!("待 Developer 修订: 静态 icache_line_size_")
  }

  /// 原版 dcache_line_size()
  #[must_use]
  pub fn dcache_line_size() -> u32 {
    todo!("待 Developer 修订: 静态 dcache_line_size_")
  }

  /// 原版 FlushICache(start, size) — 平台实现待 Developer
  pub fn flush_i_cache(_start: *mut u8, _size: usize) {
    todo!("待 Developer 修订: 平台 FlushICache")
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(CpuFeatureX64::NUMBER_OF_CPU_FEATURES as u8 == ${x64.length});
  assert!(CpuFeatureArm64::NUMBER_OF_CPU_FEATURES as u8 == ${arm64.length});
  assert!(CpuFeatureX64::SSE4_2 as u8 == 0);
  assert!(CpuFeatureArm64::JSCVT as u8 == 0);
};
`;

const p1 = writeSrc("v8-codegen-core", "cpu_features.rs", rust);

let rust2 = banner(
  "src/codegen/cpu-features.h.js",
  "src/codegen/flush-instruction-cache.{h,cc}",
  "FlushInstructionCache 自由函数签名骨架",
);
rust2 += `
/// 原版 FlushInstructionCache(void* start, size_t size):
/// 刷新指令缓存区间 (x64 上通常为空操作/内存屏障; 平台实现待 Developer)
pub fn flush_instruction_cache(start: *mut u8, size: usize) {
  let _ = (start, size);
  todo!("待 Developer 修订: 对齐 flush-instruction-cache.cc 平台实现")
}
`;
const p2 = writeSrc("v8-codegen-core", "flush_instruction_cache.rs", rust2);

console.log(
  `cpu-features: x64 ${x64.length} / arm64 ${arm64.length} 项; flush 签名 1 (cc ${flushCcSigs}) → ${p1}; ${p2}`,
);
