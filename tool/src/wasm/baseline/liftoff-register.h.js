#!/usr/bin/env bun
// tool/src/wasm/baseline/liftoff-register.h.js
// 机械提取 v8/src/wasm/baseline/liftoff-register.h 的寄存器类别编码常量
// 平台展开: x64 + arm64; 其余架构登记跳过 (硬规则 9)
// 寄存器编码取自 V8 既定编码 (register-x64.h / register-arm64.h), 属机械事实

import { readV8, writeSrc, banner } from "../../../lib.js";

const hSrc = readV8("src/wasm/baseline/liftoff-register.h");
const defsSrc = readV8("src/wasm/baseline/liftoff-assembler-defs.h");

// 1. 从 liftoff-assembler-defs.h 提取 x64/arm64 缓存寄存器表 (与 defs 生成物对数)
function parseRegs(arch) {
  const begin = defsSrc.indexOf(`#elif V8_TARGET_ARCH_${arch}`);
  const end = defsSrc.indexOf("#elif", begin + 5) === -1 ? defsSrc.length : defsSrc.indexOf("#elif", begin + 5);
  const seg = defsSrc.slice(begin, end);
  function list(name) {
    const m = seg.match(new RegExp(`${name} = \\{([^}]*)\\}`));
    if (!m) throw new Error(`未找到 ${arch} ${name}`);
    return m[1].split(",").map((s) => s.trim()).filter(Boolean);
  }
  return { gp: list("kLiftoffAssemblerGpCacheRegs"), fp: list("kLiftoffAssemblerFpCacheRegs"), simd: list("kLiftoffAssemblerSimd128CacheRegs") };
}
const x64 = parseRegs("X64");
const arm64 = parseRegs("ARM64");

// 2. 名称 → 硬件编码 (register-*.h 既定编码; 机械事实)
function gpCodeX64(n) {
  const table = { rax: 0, rcx: 1, rdx: 2, rbx: 3, rsp: 4, rbp: 5, rsi: 6, rdi: 7, r8: 8, r9: 9, r10: 10, r11: 11, r12: 12, r13: 13, r14: 14, r15: 15 };
  if (!(n in table)) throw new Error(`未知 x64 gp 寄存器: ${n}`);
  return table[n];
}
function fpCodeX64(n) {
  const m = n.match(/^xmm(\d+)$/);
  if (!m) throw new Error(`未知 x64 fp 寄存器: ${n}`);
  return parseInt(m[1], 10);
}
function gpCodeArm64(n) {
  const m = n.match(/^x(\d+)$/);
  if (!m || parseInt(m[1], 10) > 30) throw new Error(`未知 arm64 gp 寄存器: ${n}`);
  return parseInt(m[1], 10);
}
function fpCodeArm64(n) {
  const m = n.match(/^d(\d+)$/);
  if (!m || parseInt(m[1], 10) > 31) throw new Error(`未知 arm64 fp 寄存器: ${n}`);
  return parseInt(m[1], 10); // d_n 编码与 v_n 相同
}

// 3. 平台谓词 (kSystemPointerSize==8 → 无 i64 配对; x64/arm64 均为 kOverlap 别名 → 无 s128 配对、无独立 simd 寄存器)
const K_SYSTEM_POINTER_SIZE = 8;
const kNeedI64RegPair = K_SYSTEM_POINTER_SIZE === 4; // false
const kNeedS128RegPair = false; // kFPAliasing == kCombine
const kHasIndependentSimd128Regs = false; // kFPAliasing == kIndependent

// 4. RegClass 枚举 (机械展开为显式值)
const kGpReg = 0;
const kFpReg = 1;
const kNoReg = kFpReg + 1 + (kNeedI64RegPair ? 1 : 0) + (kNeedS128RegPair ? 1 : 0) + (kHasIndependentSimd128Regs ? 1 : 0);
const kSimd128Reg = kHasIndependentSimd128Regs ? kFpReg + 1 : kNoReg;
const kGpRegPair = kNeedI64RegPair ? kFpReg + 1 : kNoReg;
const kFpRegPair = kNeedS128RegPair ? kFpReg + 1 + (kNeedI64RegPair ? 1 : 0) : kNoReg;

// 5. 派生编码常量 (按原版公式计算)
function clz32(v) {
  return 32 - v.toString(2).length;
}
function derive(regs) {
  const isX64 = regs === x64;
  const maxGp = Math.max(...regs.gp.map(isX64 ? gpCodeX64 : gpCodeArm64));
  const maxFp = Math.max(...regs.fp.map(isX64 ? fpCodeX64 : fpCodeArm64));
  const maxSimd = regs.simd.length > 0 ? -2 : -1; // 空表 → -1 (两个目标均空)
  if (maxSimd !== -1) throw new Error("Simd128 缓存表非空, 派生公式需复核");
  const afterGp = maxGp + 1;
  const afterFp = afterGp + maxFp + 1;
  const afterSimd = afterFp + maxSimd + 1;
  const afterMax = afterSimd;
  return {
    maxGp, maxFp, maxSimd, afterGp, afterFp, afterSimd, afterMax,
    bitsPerLiftoff: 32 - clz32(afterMax - 1),
    bitsPerGp: 32 - clz32(maxGp),
    bitsPerFp: 32 - clz32(maxFp),
    bitsPerSimd128: maxSimd >= 0 ? 32 - clz32(maxSimd) : 0,
    bitsPerRegPair: (kNeedS128RegPair ? 2 : 1) + 2 * (32 - clz32(maxGp)),
  };
}
const dX64 = derive(x64);
const dArm64 = derive(arm64);

// 6. reg_class_for 值分类表 — 展开 value-type.h 的 FOREACH_VALUE_TYPE X-macro
//    (FOREACH_VALUE_TYPE = Void + FOREACH_NUMERIC_VALUE_TYPE + Ref, RefNull, Top, Bottom)
const vtSrc = readV8("src/wasm/value-type.h");
function macroKinds(name) {
  const m = vtSrc.match(new RegExp(`#define ${name}\\(V\\)[^]*?(?=#undef|\\nconstexpr|\\n#define [A-Z])`));
  if (!m) throw new Error(`未找到 ${name}`);
  return [...m[0].matchAll(/V\((\w+),/g)].map((x) => `k${x[1]}`);
}
const numericKinds = macroKinds("FOREACH_NUMERIC_VALUE_TYPE");
const fevt = vtSrc.match(/#define FOREACH_VALUE_TYPE\(V\)([\s\S]*?)\n\n/);
if (!fevt) throw new Error("未找到 FOREACH_VALUE_TYPE");
// 顶层宏体: V(Void..) + FOREACH_NUMERIC_VALUE_TYPE(V) + V(Ref..) V(RefNull..) V(Top..) V(Bottom..)
const topDirect = [...fevt[1].matchAll(/^\s*V\((\w+),/gm)].map((x) => `k${x[1]}`);
if (topDirect[0] !== "kVoid" || !topDirect.includes("kTop") || !topDirect.includes("kBottom")) {
  throw new Error(`FOREACH_VALUE_TYPE 顶层结构异常: ${topDirect}`);
}
const kindOrder = [
  topDirect[0], // kVoid
  ...numericKinds,
  ...topDirect.slice(1), // kRef, kRefNull, kTop, kBottom
];
// 原版 make_array 表长 = kTop 序号 (Top/Bottom 不入表)
const tableKinds = kindOrder.slice(0, kindOrder.indexOf("kTop"));
// x64/arm64: 无配对无独立 simd
function rcFor(kind) {
  if (["kF16", "kF32", "kF64", "kS128"].includes(kind)) return kFpReg;
  if (["kI8", "kI16", "kI32", "kI64", "kRef", "kRefNull"].includes(kind)) return kGpReg;
  if (kind === "kVoid") return kNoReg;
  throw new Error(`未知 ValueKind: ${kind}`);
}
if (tableKinds.length !== 11) throw new Error(`表长不符 (期望 kTop=11): ${tableKinds.length}`);
const regClassTable = tableKinds.map((kind) => ({ kind, rc: rcFor(kind) }));

console.log(
  `liftoff-register.h: x64 afterMax=${dX64.afterMax} bits=${dX64.bitsPerLiftoff}; ` +
    `arm64 afterMax=${dArm64.afterMax} bits=${dArm64.bitsPerLiftoff}; kinds=${regClassTable.length}`,
);

// 7. 生成 Rust (模块 per 目标)
function emitTarget(name, regs, d) {
  let s = `/// ${name} 目标 (V8_TARGET_ARCH_${name.toUpperCase()})\npub mod ${name} {\n`;
  s += `  /// 最大 gp 缓存寄存器编码 (原版 kMaxGpRegCode)\n  pub const MAX_GP_REG_CODE: i32 = ${d.maxGp};\n`;
  s += `  /// 最大 fp 缓存寄存器编码 (原版 kMaxFpRegCode)\n  pub const MAX_FP_REG_CODE: i32 = ${d.maxFp};\n`;
  s += `  /// simd128 缓存表为空 → -1 (原版 kMaxSimd128RegCode)\n  pub const MAX_SIMD128_REG_CODE: i32 = ${d.maxSimd};\n`;
  s += `  /// 原版 kAfterMaxLiftoffGpRegCode\n  pub const AFTER_MAX_LIFTOFF_GP_REG_CODE: i32 = ${d.afterGp};\n`;
  s += `  /// 原版 kAfterMaxLiftoffFpRegCode\n  pub const AFTER_MAX_LIFTOFF_FP_REG_CODE: i32 = ${d.afterFp};\n`;
  s += `  /// 原版 kAfterMaxLiftoffSimd128RegCode\n  pub const AFTER_MAX_LIFTOFF_SIMD128_REG_CODE: i32 = ${d.afterSimd};\n`;
  s += `  /// 原版 kAfterMaxLiftoffRegCode\n  pub const AFTER_MAX_LIFTOFF_REG_CODE: i32 = ${d.afterMax};\n`;
  s += `  /// 原版 kBitsPerLiftoffRegCode\n  pub const BITS_PER_LIFTOFF_REG_CODE: i32 = ${d.bitsPerLiftoff};\n`;
  s += `  /// 原版 kBitsPerGpRegCode\n  pub const BITS_PER_GP_REG_CODE: i32 = ${d.bitsPerGp};\n`;
  s += `  /// 原版 kBitsPerFpRegCode\n  pub const BITS_PER_FP_REG_CODE: i32 = ${d.bitsPerFp};\n`;
  s += `  /// 原版 kBitsPerSimd128RegCode\n  pub const BITS_PER_SIMD128_REG_CODE: i32 = ${d.bitsPerSimd128};\n`;
  s += `  /// 原版 kBitsPerRegPair\n  pub const BITS_PER_REG_PAIR: i32 = ${d.bitsPerRegPair};\n`;
  s += `}\n\n`;
  return s;
}

let rust = banner(
  "src/wasm/baseline/liftoff-register.h.js",
  "src/wasm/baseline/liftoff-register.h",
  "Liftoff 寄存器编码常量 (x64+arm64 展开; 其余架构跳过: arm/ia32/loong64/mips64/ppc/riscv/s390)",
);

rust += `
/// 平台谓词 (机械展开; x64/arm64: 指针 8 字节, FP 别名 kOverlap)
pub const NEED_I64_REG_PAIR: bool = ${kNeedI64RegPair};
pub const NEED_S128_REG_PAIR: bool = ${kNeedS128RegPair};
pub const HAS_INDEPENDENT_SIMD128_REGS: bool = ${kHasIndependentSimd128Regs};

/// 寄存器类别 (对应原版 enum RegClass : uint8_t; 本目标 kNoReg=${kNoReg})
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum RegClass {
  /// kGpReg
  GpReg = ${kGpReg},
  /// kFpReg
  FpReg = ${kFpReg},
  /// kNoReg — 聚合哨兵 (原版为 constexpr 别名, 值 ${kNoReg})
  NoReg = ${kNoReg},
}
/// kSimd128Reg (原版本目标下 == kNoReg)
pub const SIMD128_REG: RegClass = RegClass::NoReg;
/// kGpRegPair (原版本目标下 == kNoReg)
pub const GP_REG_PAIR: RegClass = RegClass::NoReg;
/// kFpRegPair (原版本目标下 == kNoReg)
pub const FP_REG_PAIR: RegClass = RegClass::NoReg;

/// 配对半字 (对应原版 enum RegPairHalf : uint8_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum RegPairHalf {
  /// kLowWord
  LowWord = 0,
  /// kHighWord
  HighWord = 1,
}

/// ValueKind → RegClass 查询表 (原版 reg_class_for 的 make_array 展开;
/// ValueKind 顺序对齐 value-type.h)
#[rustfmt::skip]
pub static REG_CLASS_FOR: [(crate::value_kind::ValueKind, RegClass); ${regClassTable.length}] = [
`;
for (const { kind, rc } of regClassTable) {
  const rk = rc === kGpReg ? "RegClass::GpReg" : rc === kFpReg ? "RegClass::FpReg" : "RegClass::NoReg";
  rust += `  (crate::value_kind::ValueKind::${kind.replace(/^k/, "")}, ${rk}),\n`;
}
rust += `];

`;

rust += emitTarget("x64", x64, dX64);
rust += emitTarget("arm64", arm64, dArm64);

// 跳过架构登记
rust += `// 跳过架构登记 (硬规则 9): arm / ia32 / loong64 / mips64 / ppc / riscv(32/64) / s390
// 的 liftoff-register 派生常量按需再展开 — 缓存寄存器表见 liftoff-assembler-defs.h
// 对应架构分支。

// —— 提取不变式断言 ——
const _: () = {
  assert!(!NEED_I64_REG_PAIR);
  assert!(!NEED_S128_REG_PAIR);
  assert!(!HAS_INDEPENDENT_SIMD128_REGS);
  assert!(RegClass::NoReg as u8 == ${kNoReg});
  assert!(x64::AFTER_MAX_LIFTOFF_REG_CODE == ${dX64.afterMax});
  assert!(x64::BITS_PER_LIFTOFF_REG_CODE == ${dX64.bitsPerLiftoff});
  assert!(x64::BITS_PER_GP_REG_CODE == ${dX64.bitsPerGp});
  assert!(x64::BITS_PER_REG_PAIR == ${dX64.bitsPerRegPair});
  assert!(arm64::AFTER_MAX_LIFTOFF_REG_CODE == ${dArm64.afterMax});
  assert!(arm64::BITS_PER_LIFTOFF_REG_CODE == ${dArm64.bitsPerLiftoff});
};
`;

const p = writeSrc("v8-wasm-liftoff", "liftoff_register.rs", rust);
console.log(`written ${p}`);
