#!/usr/bin/env bun
// tool/src/codegen/arm64/instructions-arm64.h.js
// 机械提取 v8/src/codegen/arm64/instructions-arm64.{h,cc,-constants.cc} —
// NEONFormat/ImmBranchType/AddrMode/FPRounding/Reg31Mode/MemOp/CheckAlignment/
// PrintfArgPattern/DebugParameters 枚举 + NEONFormatMap 结构 + FP 位型常量。
// 字段掩码 (LoadStoreMask/ImmPCRel_mask 等) 由 constants-arm64 覆盖面提供。
// 产物: v8-codegen-arm64/src/instructions_arm64_generated.rs
//   (instructions.rs 为手写产物, 本脚本落 _generated 旁路, 禁覆写)

import { readV8, writeSrc, banner } from "../../../lib.js";

const H_REL = "src/codegen/arm64/instructions-arm64.h";
const CC_REL = "src/codegen/arm64/instructions-arm64.cc";
const CONST_REL = "src/codegen/arm64/instructions-arm64-constants.cc";
const h = readV8(H_REL);
readV8(CC_REL); // 家族覆盖锚
const konst = readV8(CONST_REL);

// —— 枚举提取器 (显式值优先, 否则自动 +1) ——
function parseEnum(body) {
  const clean = body.replace(/\/\/[^\n]*/g, "");
  const out = [];
  let next = 0;
  for (const raw of clean.split(",")) {
    const m = raw.trim().match(/^(\w+)\s*(?:=\s*([^,]+))?$/);
    if (!m) continue;
    let value;
    if (m[2] === undefined) {
      value = next;
    } else if (/^0[xX][0-9a-fA-F]+$|^\d+$/.test(m[2].trim())) {
      value = Number(m[2].trim());
    } else if (/^\d+\s*<<\s*\d+$/.test(m[2].trim())) {
      const parts = m[2].split("<<");
      value = Number(parts[0].trim()) << Number(parts[1].trim());
    } else {
      // A | B 形式: 按已登记条目按位或求值
      value = m[2].split("|").reduce((acc, part) => {
        const k = part.trim();
        const found = out.find((e) => e.name === k);
        if (!found) throw new Error(`枚举值表达式不支持: ${raw} (引用 ${k})`);
        return acc | found.value;
      }, 0);
    }
    if (Number.isNaN(value)) throw new Error(`枚举值非数字: ${raw}`);
    out.push({ name: m[1], value });
    next = value + 1;
  }
  return out;
}
function needEnum(name, re, expect) {
  const m = h.match(re);
  if (!m) throw new Error(`未找到 ${name}`);
  const items = parseEnum(m[1]);
  if (items.length !== expect) throw new Error(`${name} 条数不符: ${items.length} != ${expect}`);
  return items;
}

const neonFmt = needEnum("NEONFormat", /enum NEONFormat\s*\{([^}]+)\};/, 13);
if (neonFmt[0].name !== "NF_UNDEF" || neonFmt[1].value !== 1) throw new Error("NEONFormat 锚不符");
const branchTypes = needEnum("ImmBranchType", /enum ImmBranchType\s*\{([^}]+)\};/, 5);
const addrModes = needEnum("AddrMode", /enum AddrMode\s*\{([^}]+)\};/, 3);
const fpRounding = needEnum("FPRounding", /enum FPRounding\s*\{([^}]+)\};/, 6);
if (fpRounding[0].value !== 0x0 || fpRounding[3].value !== 0x3) throw new Error("FPRounding 编码锚不符");
const reg31Modes = needEnum("Reg31Mode", /enum Reg31Mode\s*\{([^}]+)\};/, 2);
const memOps = needEnum("MemOp", /enum class MemOp : uint8_t\s*\{([^}]+)\};/, 2);
const checkAlign = needEnum("CheckAlignment", /enum CheckAlignment\s*\{([^}]+)\};/, 2);
const printfArgs = needEnum("PrintfArgPattern", /enum PrintfArgPattern\s*\{([^}]+)\};/, 3);
const dbgParams = needEnum("DebugParameters", /enum DebugParameters\s*\{([^}]+)\};/, 13);

const maxBitsM = h.match(/static const unsigned kNEONFormatMaxBits = (\d+);/);
if (!maxBitsM || maxBitsM[1] !== "6") throw new Error("kNEONFormatMaxBits 锚不符");
const printfBitsM = h.match(/static const unsigned kPrintfArgPatternBits = (\d+);/);
if (!printfBitsM || printfBitsM[1] !== "2") throw new Error("kPrintfArgPatternBits 锚不符");

// FP 位型常量 (integer_constants)
function needFP(name) {
  const m = konst.match(new RegExp(`constexpr u?int(?:32|64)_t ${name} = (0x[0-9A-Fa-f']+)`));
  if (!m) throw new Error(`未找到 ${name}`);
  return BigInt(m[1].replace(/'/g, ""));
}
const fps = {
  fp32PosInf: needFP("kFP32PositiveInfinity"),
  fp32NegInf: needFP("kFP32NegativeInfinity"),
  fp64PosInf: needFP("kFP64PositiveInfinity"),
  fp64NegInf: needFP("kFP64NegativeInfinity"),
  fp64SigNaN: needFP("kFP64SignallingNaN"),
  fp32SigNaN: needFP("kFP32SignallingNaN"),
  fp64QuietNaN: needFP("kFP64QuietNaN"),
  fp32QuietNaN: needFP("kFP32QuietNaN"),
  fp64DefaultNaN: needFP("kFP64DefaultNaN"),
  fp32DefaultNaN: needFP("kFP32DefaultNaN"),
};
const fp16DefaultNaN = konst.match(/kFP16DefaultNaN = (0x[0-9A-Fa-f]+)/);
if (!fp16DefaultNaN) throw new Error("未找到 kFP16DefaultNaN");

let rust = banner("src/codegen/arm64/instructions-arm64.h.js", "src/codegen/arm64/instructions-arm64.{h,cc,-constants.cc}", "arm64 指令枚举与 FP 位型常量");
rust += `
// 字段掩码常量 (LoadStoreMask/ImmPCRel_mask/…) 由 constants-arm64 覆盖面提供;
// 指令解码语义 (Instruction::*) 登记跳过。

/// NEON 访存/运算格式 (原 enum NEONFormat)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum NeonFormat {
${neonFmt.map((e) => `  /// ${e.name}\n  ${e.name} = ${e.value},`).join("\n")}
}

/// NEON 格式索引位宽上限 (原 kNEONFormatMaxBits)
pub const NEON_FORMAT_MAX_BITS: usize = 6;

/// NEONFormatMap (原 struct): bits 位拼接索引 → 格式
#[derive(Debug, Clone, Copy)]
pub struct NeonFormatMap {
  /// 参与索引的指令位号
  pub bits: [u8; ${maxBitsM[1]}],
  /// 2^bits 查表
  pub map: [NeonFormat; 1 << ${maxBitsM[1]}],
}

/// 分支类型 (原 enum ImmBranchType)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ImmBranchType {
${branchTypes.map((e) => `  /// ${e.name}\n  ${e.name} = ${e.value},`).join("\n")}
}

/// 访存地址模式 (原 enum AddrMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AddrMode {
${addrModes.map((e, i) => `  /// ${e.name}\n  ${e.name} = ${i},`).join("\n")}
}

/// 浮点舍入模式 (原 enum FPRounding; 前 4 值可由 FPCR<RMode> 编码)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FPRounding {
${fpRounding.map((e) => `  /// ${e.name}\n  ${e.name} = 0x${e.value.toString(16)},`).join("\n")}
}

/// 31 号寄存器语义 (原 enum Reg31Mode)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Reg31Mode {
${reg31Modes.map((e, i) => `  /// ${e.name}\n  ${e.name} = ${i},`).join("\n")}
}

/// MOPS 内存操作 (原 enum class MemOp : uint8_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MemOp {
${memOps.map((e, i) => `  /// ${e.name}\n  ${e.name} = ${i},`).join("\n")}
}

/// 对齐检查 (原 enum CheckAlignment)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CheckAlignment {
${checkAlign.map((e, i) => `  /// ${e.name}\n  ${e.name} = ${i},`).join("\n")}
}

/// printf 参数模式 (原 enum PrintfArgPattern; 无 S 型 — float 一律转 double)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PrintfArgPattern {
${printfArgs.map((e) => `  /// ${e.name}\n  ${e.name} = ${e.value},`).join("\n")}
}

/// printf 参数模式位宽 (原 kPrintfArgPatternBits)
pub const PRINTF_ARG_PATTERN_BITS: usize = 2;

/// debug 参数 (原 enum DebugParameters)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DebugParameters {
${dbgParams.map((e, i) => `  /// ${e.name}\n  ${e.name} = ${i},`).join("\n")}
}

// —— FP 位型常量 (integer_constants, bit_cast 前的整型原值) ——

/// FP32 +∞ 位型
pub const FP32_POSITIVE_INFINITY_BITS: u32 = 0x${fps.fp32PosInf.toString(16).toUpperCase()};
/// FP32 -∞ 位型
pub const FP32_NEGATIVE_INFINITY_BITS: u32 = 0x${fps.fp32NegInf.toString(16).toUpperCase()};
/// FP64 +∞ 位型
pub const FP64_POSITIVE_INFINITY_BITS: u64 = 0x${fps.fp64PosInf.toString(16).toUpperCase()};
/// FP64 -∞ 位型
pub const FP64_NEGATIVE_INFINITY_BITS: u64 = 0x${fps.fp64NegInf.toString(16).toUpperCase()};
/// FP64 信号 NaN
pub const FP64_SIGNALLING_NAN_BITS: u64 = 0x${fps.fp64SigNaN.toString(16).toUpperCase()};
/// FP32 信号 NaN
pub const FP32_SIGNALLING_NAN_BITS: u32 = 0x${fps.fp32SigNaN.toString(16).toUpperCase()};
/// FP64 静默 NaN
pub const FP64_QUIET_NAN_BITS: u64 = 0x${fps.fp64QuietNaN.toString(16).toUpperCase()};
/// FP32 静默 NaN
pub const FP32_QUIET_NAN_BITS: u32 = 0x${fps.fp32QuietNaN.toString(16).toUpperCase()};
/// FP64 默认 NaN (FPCR.DN=1)
pub const FP64_DEFAULT_NAN_BITS: u64 = 0x${fps.fp64DefaultNaN.toString(16).toUpperCase()};
/// FP32 默认 NaN
pub const FP32_DEFAULT_NAN_BITS: u32 = 0x${fps.fp32DefaultNaN.toString(16).toUpperCase()};
/// FP16 默认 NaN
pub const FP16_DEFAULT_NAN_BITS: u16 = ${fp16DefaultNaN[1].toUpperCase()};

// —— 提取不变式断言 ——
const _: () = {
  assert!(NeonFormat::NF_UNDEF as u8 == 0);
  assert!(NeonFormat::NF_D as u8 == 12);
  assert!(ImmBranchType::CondBranchType as u8 == 1);
  assert!(FPRounding::FPTieEven as u8 == 0x0);
  assert!(FPRounding::FPZero as u8 == 0x3);
  assert!(FP64_DEFAULT_NAN_BITS == 0x7FF8000000000000);
  assert!(FP16_DEFAULT_NAN_BITS == 0x7E00);
  assert!(PRINTF_ARG_PATTERN_BITS == 2);
};
`;

const p = writeSrc("v8-codegen-arm64", "instructions_arm64_generated.rs", rust);
console.log(`written ${p} (枚举 9 组, FP 常量 11)`);
