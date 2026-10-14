#!/usr/bin/env bun
// tool/src/codegen/arm64/constants-arm64.h.js — 从 v8/src/codegen/arm64/constants-arm64.h
// 提取 wave1 零耦合数据面: 标量常量、编码域字段(offset/width/mask)、Condition/StatusFlags/
// Shift/Extend/SystemHint/Barrier 枚举、SystemRegister 编码、Generic fields 位掩码。
// 逐值 golden 对数断言(真跑), missing/多余/漂移即 throw; 通过后直出 constants.rs 基线。
// 指令类 Fixed/FMask/Mask 大枚举(547 行后)随 decoder 留二梯队, 本脚本不触碰。
import { readV8, writeSrc, banner } from "../../../lib.js";

const V8REL = "src/codegen/arm64/constants-arm64.h";
const src = readV8(V8REL);
const allLines = src.split("\n");

function must(cond, msg) {
  if (!cond) throw new Error(`constants-arm64.h 对数核对失败: ${msg}`);
}

// ── 名称转换: C++ → Rust SCREAMING_SNAKE (kFoo → K_FOO, 与 v8-common 生成风格一致) ──
function rustUpper(name) {
  if (name === "CRn") return "CRN";
  if (name === "CRm") return "CRM";
  // 已是全大写带下划线风格 (NEON_8B / LS_NEON_4H / BTI_c): 直接大写
  if (/^[A-Z][A-Z0-9]*(_[A-Za-z0-9]+)+$/.test(name)) return name.toUpperCase();
  let s = name.replace(/NEON(?=[a-z0-9])/g, "NEON_");
  if (s.startsWith("NEON") && s.length > 4 && s[4] !== "_") s = "NEON_" + s.slice(4);
  return s
    .replace(/([a-z0-9])([A-Z])/g, "$1_$2")
    .replace(/([A-Z]+)([A-Z][a-z])/g, "$1_$2")
    .toUpperCase();
}

// ── 表达式求值: C++ 整型常量表达式 → BigInt ──
const env = new Map([
  ["KB", 1024n],
  ["MB", 1024n * 1024n],
  ["GB", 1024n * 1024n * 1024n],
]);

function evalExpr(raw, lookup) {
  let e = String(raw)
    .replace(/\/\/[^\n]*/g, "")
    .replace(/UINT64_C\(\s*(0x[0-9a-fA-F]+|\d+)\s*\)/g, "$1")
    .replace(/\b(0[xX][0-9a-fA-F]+|\d+)(?:[uUlL]+)(?=\b)/g, "$1")
    .trim();
  // 单遍词法替换: 十六进制/十进制字面量 → BigInt 串, 标识符 → env 值
  e = e.replace(/0[xX][0-9a-fA-F]+|[A-Za-z_][A-Za-z0-9_]*|[0-9]+/g, (tok) => {
    if (/^\d/.test(tok)) return (tok[1] === "x" || tok[1] === "X" ? BigInt(tok).toString() : tok) + "n";
    const v = (lookup ? lookup(tok) : undefined) ?? env.get(tok);
    if (v === undefined) throw new Error(`constants: 未求值的标识符 ${tok} in ${raw}`);
    return v.toString() + "n";
  });
  if (!/^[\s0-9n+\-*/%()&|^~<>]+$/.test(e)) throw new Error(`constants: 表达式含非法记号 ${e}`);
  const v = Function(`"use strict"; return (${e});`)();
  if (typeof v !== "bigint") throw new Error(`constants: 非 BigInt 结果 ${e}`);
  return v;
}

const A_TYPES = {
  size_t: "usize", uint8_t: "u8", uint64_t: "u64", int64_t: "i64",
  int32_t: "i32", int: "i32", unsigned: "u32",
};
const GF_TYPES = {
  uint32_t: "u32", GenericInstrField: "u32", NEONFormatField: "u32",
  NEONFPFormatField: "u32", NEONLSFormatField: "u32", NEONScalarFormatField: "u32",
  SVESizeField: "u32",
};

// 逐值 golden 表 (手工按 constants-arm64.h 核对)
const GOLD_A = {
  kMaxPCRelativeCodeRangeInMB: ["usize", 128n],
  kInstrSize: ["u8", 4n],
  kInstrSizeLog2: ["u8", 2n],
  kLoadLiteralScaleLog2: ["u8", 2n],
  kLoadLiteralScale: ["u8", 4n],
  kMaxLoadLiteralRange: ["i32", 0x100000n],
  kNumberOfRegisters: ["i32", 32n],
  kNumberOfVRegisters: ["i32", 32n],
  kNumberOfZRegisters: ["i32", 32n],
  kNumberOfCalleeSavedRegisters: ["i32", 10n],
  kNumberOfCalleeSavedDRegisters: ["i32", 8n],
  kNumberOfCalleeSavedVRegisters: ["i32", 0n],
  kWRegSizeInBits: ["i32", 32n],
  kWRegSizeInBitsLog2: ["i32", 5n],
  kWRegSize: ["i32", 4n],
  kWRegSizeLog2: ["i32", 2n],
  kXRegSizeInBits: ["i32", 64n],
  kXRegSizeInBitsLog2: ["i32", 6n],
  kXRegSize: ["i32", 8n],
  kXRegSizeLog2: ["i32", 3n],
  kSRegSizeInBits: ["i32", 32n],
  kSRegSizeInBitsLog2: ["i32", 5n],
  kSRegSize: ["i32", 4n],
  kSRegSizeLog2: ["i32", 2n],
  kDRegSizeInBits: ["i32", 64n],
  kDRegSizeInBitsLog2: ["i32", 6n],
  kDRegSize: ["i32", 8n],
  kDRegSizeLog2: ["i32", 3n],
  kDRegSizeInBytesLog2: ["i32", 3n],
  kBRegSizeInBits: ["i32", 8n],
  kBRegSize: ["i32", 1n],
  kHRegSizeInBits: ["i32", 16n],
  kHRegSize: ["i32", 2n],
  kQRegSizeInBits: ["i32", 128n],
  kQRegSizeInBitsLog2: ["i32", 7n],
  kQRegSize: ["i32", 16n],
  kQRegSizeLog2: ["i32", 4n],
  kVRegSizeInBits: ["i32", 128n],
  kVRegSize: ["i32", 16n],
  kWRegMask: ["i64", 0xffffffffn],
  kXRegMask: ["i64", 0xffffffffffffffffn],
  kSRegMask: ["i64", 0xffffffffn],
  kDRegMask: ["i64", 0xffffffffffffffffn],
  kDSignBit: ["i64", 63n],
  kDSignMask: ["i64", 0x8000000000000000n],
  kSSignBit: ["i64", 31n],
  kSSignMask: ["i64", 0x80000000n],
  kXSignBit: ["i64", 63n],
  kXSignMask: ["i64", 0x8000000000000000n],
  kWSignBit: ["i64", 31n],
  kWSignMask: ["i64", 0x80000000n],
  kDQuietNanBit: ["i64", 51n],
  kDQuietNanMask: ["i64", 0x8000000000000n],
  kSQuietNanBit: ["i64", 22n],
  kSQuietNanMask: ["i64", 0x400000n],
  kHQuietNanBit: ["i64", 9n],
  kHQuietNanMask: ["i64", 0x200n],
  kByteMask: ["i64", 0xffn],
  kHalfWordMask: ["i64", 0xffffn],
  kWordMask: ["i64", 0xffffffffn],
  kXMaxUInt: ["u64", 0xffffffffffffffffn],
  kWMaxUInt: ["u64", 0xffffffffn],
  kXMaxInt: ["i64", 0x7fffffffffffffffn],
  kXMinInt: ["i64", 0x8000000000000000n],
  kWMaxInt: ["i32", 0x7fffffffn],
  kWMinInt: ["i32", 0x80000000n],
  kIp0Code: ["i32", 16n],
  kIp1Code: ["i32", 17n],
  kFramePointerRegCode: ["i32", 29n],
  kLinkRegCode: ["i32", 30n],
  kZeroRegCode: ["i32", 31n],
  kSPRegInternalCode: ["i32", 63n],
  kRegCodeMask: ["u32", 0x1fn],
  kShiftAmountWRegMask: ["u32", 0x1fn],
  kShiftAmountXRegMask: ["u32", 0x3fn],
  kHalfWordSize: ["u32", 16n],
  kHalfWordSizeLog2: ["u32", 4n],
  kHalfWordSizeInBytes: ["u32", 2n],
  kHalfWordSizeInBytesLog2: ["u32", 1n],
  kWordSize: ["u32", 32n],
  kWordSizeLog2: ["u32", 5n],
  kWordSizeInBytes: ["u32", 4n],
  kWordSizeInBytesLog2: ["u32", 2n],
  kDoubleWordSize: ["u32", 64n],
  kDoubleWordSizeInBytes: ["u32", 8n],
  kQuadWordSize: ["u32", 128n],
  kQuadWordSizeInBytes: ["u32", 16n],
  kMaxLanesPerVector: ["i32", 16n],
  kAddressTagOffset: ["u32", 56n],
  kAddressTagWidth: ["u32", 8n],
  kAddressTagMask: ["u64", 0xff00000000000000n],
  kTTBRMask: ["u64", 0x80000000000000n],
  kDoubleMantissaBits: ["u32", 52n],
  kDoubleExponentBits: ["u32", 11n],
  kDoubleExponentBias: ["u32", 1023n],
  kFloatMantissaBits: ["u32", 23n],
  kFloatExponentBits: ["u32", 8n],
  kFloatExponentBias: ["u32", 127n],
  kFloat16MantissaBits: ["u32", 10n],
  kFloat16ExponentBits: ["u32", 5n],
  kFloat16ExponentBias: ["u32", 15n],
  kRootRegisterBias: ["i32", 256n],
};

const GOLD_GF = {
  kUnallocatedInstruction: ["u32", 0xffffffffn],
  SixtyFourBits: ["u32", 0x80000000n],
  ThirtyTwoBits: ["u32", 0x0n],
  FP32: ["u32", 0x0n],
  FP64: ["u32", 0x00400000n],
  NEONFormatFieldMask: ["u32", 0x40C00000n],
  NEON_Q: ["u32", 0x40000000n],
  NEON_sz: ["u32", 0x00400000n],
  NEON_8B: ["u32", 0x0n],
  NEON_16B: ["u32", 0x40000000n],
  NEON_4H: ["u32", 0x00400000n],
  NEON_8H: ["u32", 0x40400000n],
  NEON_2S: ["u32", 0x00800000n],
  NEON_4S: ["u32", 0x40800000n],
  NEON_1D: ["u32", 0x00C00000n],
  NEON_2D: ["u32", 0x40C00000n],
  NEONFPFormatFieldMask: ["u32", 0x40400000n],
  NEON_FP_4H: ["u32", 0x0n],
  NEON_FP_8H: ["u32", 0x40000000n],
  NEON_FP_2S: ["u32", 0x0n],
  NEON_FP_4S: ["u32", 0x40000000n],
  NEON_FP_2D: ["u32", 0x40400000n],
  NEONLSFormatFieldMask: ["u32", 0x40000C00n],
  LS_NEON_8B: ["u32", 0x0n],
  LS_NEON_16B: ["u32", 0x40000000n],
  LS_NEON_4H: ["u32", 0x400n],
  LS_NEON_8H: ["u32", 0x40000400n],
  LS_NEON_2S: ["u32", 0x800n],
  LS_NEON_4S: ["u32", 0x40000800n],
  LS_NEON_1D: ["u32", 0xC00n],
  LS_NEON_2D: ["u32", 0x40000C00n],
  NEONScalarFormatFieldMask: ["u32", 0x00C00000n],
  NEONScalar: ["u32", 0x10000000n],
  NEON_B: ["u32", 0x0n],
  NEON_H: ["u32", 0x400000n],
  NEON_S: ["u32", 0x800000n],
  NEON_D: ["u32", 0xC00000n],
  SVESizeFieldMask: ["u32", 0x00C00000n],
  SVE_B: ["u32", 0x0n],
  SVE_H: ["u32", 0x400000n],
  SVE_S: ["u32", 0x800000n],
  SVE_D: ["u32", 0xC00000n],
};

const RE_CONST = /constexpr\s+(\w+)\s+(\w+)\s*=\s*([\s\S]*?)\s*;/g;
function parseConsts(text) {
  return [...text.matchAll(RE_CONST)].map(([, ty, name, expr]) => [ty, name, expr]);
}

// 解析 + 求值 + 逐值对数; 返回 Map(name → [rustName, rustType, BigInt])
function checkGolden(parsed, gold, label, typeMap) {
  const seen = new Map();
  for (const [ty, name, expr] of parsed) {
    must(gold[name] !== undefined, `${label}: 意外常量 ${name}`);
    const [goldTy, goldVal] = gold[name];
    must(typeMap[ty] === goldTy, `${label}: ${name} 类型漂移 ${ty} != ${goldTy}`);
    const v = evalExpr(expr);
    must(v === goldVal, `${label}: ${name} 值漂移 src=${v} gold=${goldVal}`);
    must(!seen.has(name), `${label}: 重复定义 ${name}`);
    seen.set(name, [rustUpper(name), goldTy, v]);
    env.set(name, v); // 派生常量按声明顺序引用前值
  }
  const missing = Object.keys(gold).filter((k) => !seen.has(k));
  must(missing.length === 0, `${label}: 缺失 ${missing.join(",")}`);
  return seen;
}

// 区 A: 行 33-154 (0-based 30 起)
const parsedA = parseConsts(allLines.slice(30, 155).join("\n"));
must(parsedA.length === Object.keys(GOLD_A).length, `区A 对数 ${parsedA.length} != ${Object.keys(GOLD_A).length}`);
const constsA = checkGolden(parsedA, GOLD_A, "区A", A_TYPES);
for (const [, name, expr] of parsedA) env.set(name, evalExpr(expr));

// Generic fields: 行 491-545 + kUnallocatedInstruction(491)
const parsedGF = parseConsts(allLines.slice(488, 546).join("\n"));
must(parsedGF.length === Object.keys(GOLD_GF).length, `GenericFields 对数 ${parsedGF.length} != ${Object.keys(GOLD_GF).length}`);
const constsGF = checkGolden(parsedGF, GOLD_GF, "GenericFields", GF_TYPES);
for (const [, name, expr] of parsedGF) env.set(name, evalExpr(expr));

// ── 区 B: 编码域字段表 (续行宏) ──
function grabMacro(name) {
  const i = allLines.findIndex((l) => l.startsWith(`#define ${name}`));
  must(i >= 0, `宏缺失 ${name}`);
  const parts = [];
  let j = i;
  for (; j < allLines.length; j++) {
    const line = allLines[j];
    parts.push(line);
    if (!line.trimEnd().endsWith("\\")) break;
  }
  return parts.join(" ").replace(/\\\s*$/gm, " ").replace(/\/\*[^*]*\*\//g, "");
}

const fields = [...grabMacro("INSTRUCTION_FIELDS_LIST").matchAll(/V_\(([^)]*)\)/g)]
  .map((m) => m[1].split(",").map((s) => s.trim()));
const sysFields = [...grabMacro("SYSTEM_REGISTER_FIELDS_LIST").matchAll(/V_\(([^)]*)\)/g)]
  .map((m) => m[1].split(",").map((s) => s.trim()));
for (const f of [...fields, ...sysFields]) { f[1] = Number(f[1]); f[2] = Number(f[2]); }
must(fields.length === 79, `INSTRUCTION_FIELDS_LIST 对数 ${fields.length} != 79`);
must(sysFields.length === 9, `SYSTEM_REGISTER_FIELDS_LIST 对数 ${sysFields.length} != 9`);

// 逐字段 golden (高位/低位手工核对自 constants-arm64.h:158-291)
const FIELD_GOLD = {
  Rd: [4, 0], Rn: [9, 5], Rm: [20, 16], Ra: [14, 10], Rt: [4, 0], Rt2: [14, 10], Rs: [20, 16],
  PrefetchMode: [4, 0], SixtyFourBits: [31, 31], FlagsUpdate: [29, 29],
  ImmPCRelHi: [23, 5], ImmPCRelLo: [30, 29], ShiftDP: [23, 22], ImmDPShift: [15, 10],
  ImmAddSub: [21, 10], ShiftAddSub: [23, 22], ImmExtendShift: [12, 10], ExtendMode: [15, 13],
  ImmMoveWide: [20, 5], ShiftMoveWide: [22, 21], BitN: [22, 22], ImmRotate: [21, 16],
  ImmSetBits: [15, 10], ImmR: [21, 16], ImmS: [15, 10], ImmTestBranch: [18, 5],
  ImmTestBranchBit40: [23, 19], ImmTestBranchBit5: [31, 31], Condition: [15, 12],
  ConditionBranch: [3, 0], Nzcv: [3, 0], ImmCondCmp: [20, 16], ImmCondBranch: [23, 5],
  FPType: [23, 22], ImmFP: [20, 13], FPScale: [15, 10], ImmLS: [20, 12],
  ImmLSUnsigned: [21, 10], ImmLSPair: [21, 15], ImmShiftLS: [12, 12], LSOpc: [23, 22],
  LSVector: [26, 26], LSSize: [31, 30], NEONQ: [30, 30], NEONSize: [23, 22],
  NEONLSSize: [11, 10], NEONS: [12, 12], NEONL: [21, 21], NEONM: [20, 20], NEONH: [11, 11],
  ImmNEONExt: [14, 11], ImmNEON5: [20, 16], ImmNEON4: [14, 11], ImmUncondBranch: [25, 0],
  ImmCmpBranch: [23, 5], ImmLLiteral: [23, 5], ImmException: [20, 5], ImmHint: [11, 5],
  ImmBarrierDomain: [11, 10], ImmBarrierType: [9, 8], ImmSystemRegister: [19, 5],
  SysO0: [19, 19], SysOp1: [18, 16], SysOp2: [7, 5], CRn: [15, 12], CRm: [11, 8],
  LoadStoreXLoad: [22, 22], LoadStoreXNotExclusive: [23, 23], LoadStoreXAcquireRelease: [15, 15],
  LoadStoreXSizeLog2: [31, 30], LoadStoreXPair: [21, 21], NEONLoad: [22, 22],
  ImmNEONabc: [18, 16], ImmNEONdefgh: [9, 5], NEONModImmOp: [29, 29], NEONCmode: [15, 12],
  ImmNEONImmhImmb: [22, 16], ImmNEONImmh: [22, 19], ImmNEONImmb: [18, 16],
};
must(Object.keys(FIELD_GOLD).length === fields.length, "字段 golden 表对数不符");
const SYS_FIELD_GOLD = {
  Flags: [31, 28], N: [31, 31], Z: [30, 30], C: [29, 29], V: [28, 28],
  AHP: [26, 26], DN: [25, 25], FZ: [24, 24], RMode: [23, 22],
};
must(Object.keys(SYS_FIELD_GOLD).length === sysFields.length, "系统字段 golden 表对数不符");

const fieldDefs = []; // [rustName, offset, width, mask]
function emitField(base, hi, lo) {
  const width = hi - lo + 1;
  const mask = (((1 << width) - 1) << lo) >>> 0; // uint32_t 语义
  fieldDefs.push([rustUpper(base), lo, width, mask]);
  env.set(`${base}_offset`, BigInt(lo));
  env.set(`${base}_width`, BigInt(width));
  env.set(`${base}_mask`, BigInt(mask));
}
for (const [name, hi, lo] of fields) {
  const g = FIELD_GOLD[name];
  must(g && g[0] === hi && g[1] === lo, `字段漂移 ${name}: ${hi},${lo} != ${g}`);
  emitField(name, hi, lo);
}
for (const [name, hi, lo] of sysFields) {
  const g = SYS_FIELD_GOLD[name];
  must(g && g[0] === hi && g[1] === lo, `系统字段漂移 ${name}`);
  emitField(name, hi, lo);
}

must(src.includes("constexpr int ImmPCRel_mask = ImmPCRelLo_mask | ImmPCRelHi_mask;"), "ImmPCRel_mask 漂移");
const immPcRelMask = Number((env.get("ImmPCRelLo_mask") | env.get("ImmPCRelHi_mask")) & 0xffffffffn);

// ── 区 C: 枚举 ──
function grabEnum(name) {
  const i = src.indexOf(`enum ${name}`);
  must(i >= 0, `枚举缺失 ${name}`);
  const open = src.indexOf("{", i);
  return src.slice(open + 1, src.indexOf("};", open)).replace(/\/\/[^\n]*/g, "");
}
function parseEnumBody(body) {
  return body.split(",").map((s) => s.trim()).filter(Boolean).map((e) => {
    const [n, v] = e.split("=").map((s) => s.trim());
    return [n, v];
  });
}

const cond = parseEnumBody(grabEnum("Condition : int"));
must(cond.length === 32, `Condition 对数 ${cond.length} != 32`); // 16 码 + cs/cc 同值别名 + 14 跨平台别名
const condVals = new Map();
for (const [n, v] of cond) {
  const val = v === undefined ? undefined : /^\d/.test(v) ? BigInt(v) : condVals.get(v);
  must(val !== undefined, `Condition 未解析 ${n}=${v}`);
  must(!condVals.has(n), `Condition 重复 ${n}`);
  condVals.set(n, val);
}
must(condVals.get("eq") === 0n && condVals.get("nv") === 15n &&
  condVals.get("kEqual") === 0n && condVals.get("kNotZero") === 1n &&
  condVals.get("kUnsignedGreaterThanEqual") === 2n, "Condition 值锚点漂移");

const shift = parseEnumBody(grabEnum("Shift"));
must(shift.length === 6 && shift[0][0] === "NO_SHIFT" && shift[0][1] === "-1", "Shift 漂移");
const extend = parseEnumBody(grabEnum("Extend"));
must(extend.length === 9 && extend[0][0] === "NO_EXTEND" && extend[8][1] === "7", "Extend 漂移");
const hint = parseEnumBody(grabEnum("SystemHint"));
must(hint.length === 11 && hint.map(([, v]) => v).join() === "0,1,2,3,4,5,20,32,34,36,38", "SystemHint 漂移");
const bd = parseEnumBody(grabEnum("BarrierDomain"));
must(bd.length === 4 && bd.map(([, v]) => v).join() === "0,1,2,3", "BarrierDomain 漂移");
const bt = parseEnumBody(grabEnum("BarrierType"));
must(bt.length === 4 && bt.map(([, v]) => v).join() === "0,1,2,3", "BarrierType 漂移");

// FlagsUpdate (行内)
const fu = parseEnumBody(grabEnum("FlagsUpdate {"));
must(fu.length === 2 && fu[0][0] === "SetFlags" && fu[0][1] === "1" && fu[1][1] === "0", "FlagsUpdate 漂移");

const status = parseEnumBody(grabEnum("StatusFlags"));
must(status.length === 20, `StatusFlags 对数 ${status.length} != 20`);
const statusVals = new Map();
for (const [n, v] of status) {
  const val = evalExpr(v, (id) => statusVals.get(id) ?? env.get(id) ?? env.get(`${id}_mask`));
  statusVals.set(n, val);
}
must(statusVals.get("NoFlag") === 0n && statusVals.get("NFlag") === 0x80000000n &&
  statusVals.get("NZCVFlag") === 0xf0000000n && statusVals.get("FPEqualFlag") === statusVals.get("ZCFlag") &&
  statusVals.get("FPUnorderedFlag") === statusVals.get("CVFlag"), "StatusFlags 锚点漂移");

const sysreg = parseEnumBody(grabEnum("SystemRegister {"));
must(sysreg.length === 2 && sysreg[0][0] === "NZCV" && sysreg[1][0] === "FPCR", "SystemRegister 漂移");
must(sysreg.map(([, v]) => evalExpr(v)).join() === "23056,23072", "SystemRegister 值漂移 (NZCV=0x5A10 FPCR=0x5A20)");

const bti = parseEnumBody(grabEnum("class BranchTargetIdentifier")).map(([n]) => n.replace(/=.*$/, "").trim());
must(bti.length === 6 && bti[0] === "kNone" && bti[5] === "kPacibsp", "BranchTargetIdentifier 漂移");

// ── Rust 输出 ──
function lit(v, ty) {
  const bits = { u8: 8, u32: 32, u64: 64, i32: 32, i64: 64, usize: 64 }[ty];
  const hex = (x) => `0x${x.toString(16).toUpperCase().padStart(bits / 4, "0")}`;
  if (ty.startsWith("i")) {
    const max = (1n << BigInt(bits - 1)) - 1n;
    if (v > max) {
      const ut = ty === "i64" ? "u64" : "u32";
      return `${hex(v & ((1n << BigInt(bits)) - 1n))}_${ut} as ${ty}`;
    }
  }
  return hex(v & ((1n << BigInt(bits)) - 1n));
}
// C++ 表达式 → Rust 表达式 (仅标识符名转写, 数字字面量原样保留)
function translate(expr) {
  return expr.replace(/0[xX][0-9a-fA-F]+|[A-Za-z_][A-Za-z0-9_]*|\d+/g, (tok) => {
    if (/^\d/.test(tok)) return tok;
    const m = tok.match(/^(.*)_(offset|width|mask)$/);
    return m ? `${rustUpper(m[1])}_${m[2].toUpperCase()}` : rustUpper(tok);
  });
}

let out = banner("src/codegen/arm64/constants-arm64.h.js", V8REL,
  "v8-codegen-arm64 wave1 — constants-arm64.h 零耦合数据面 1:1; 指令类 Fixed/FMask/Mask 枚举随 decoder 留二梯队");
out += `
use core::mem::size_of;

// static_assert(LP64) 对齐 constants-arm64.h:12-20 (host=arm64 LP64)
const _: () = assert!(size_of::<i32>() == 4); // sizeof(int) == sizeof(int32_t)
const _: () = assert!(size_of::<i64>() == 8); // sizeof(long) == sizeof(int64_t)
const _: () = assert!(size_of::<*const u8>() == 8); // sizeof(void*) == sizeof(int64_t)

// ── 标量常量 (constants-arm64.h:33-154) ──
`;
for (const name of Object.keys(GOLD_A)) {
  const [rn, ty, v] = constsA.get(name);
  out += `pub const ${rn}: ${ty} = ${lit(v, ty)};\n`;
}
out += "\n// ── 编码域字段 offset/width/mask (INSTRUCTION_FIELDS_LIST + SYSTEM_REGISTER_FIELDS_LIST) ──\n";
for (const [rn, off, w, mask] of fieldDefs) {
  out += `pub const ${rn}_OFFSET: i32 = ${off};\npub const ${rn}_WIDTH: i32 = ${w};\npub const ${rn}_MASK: u32 = ${lit(BigInt(mask), "u32")};\n`;
}
out += `// ImmPCRel 复合域 (Lo|Hi, constants-arm64.h:307)\npub const IMM_PC_REL_MASK: u32 = ${lit(BigInt(immPcRelMask), "u32")};\n`;
out += "\n// ── Generic fields 位掩码 (constants-arm64.h:491-545) ──\n";
for (const name of Object.keys(GOLD_GF)) {
  const [rn, ty, v] = constsGF.get(name);
  out += `pub const ${rn}: ${ty} = ${lit(v, ty)};\n`;
}
out += "\n// ── Condition 码与跨平台别名 (constants-arm64.h:310-352); k别名 → K_前缀 保持可区分 ──\n";
for (const [n] of cond) {
  out += `pub const ${rustUpper(n)}: i32 = ${condVals.get(n)};\n`;
}
out += `
/// 原版 NegateCondition: cond ^ 1; al/nv 恒真不可取反
pub fn negate_condition(cond: i32) -> i32 {
  debug_assert!(cond != AL && cond != NV, "al/nv 无逆条件");
  cond ^ 1
}
`;
out += "\n// ── FlagsUpdate / Shift / Extend / SystemHint / Barrier (constants-arm64.h:354-455) ──\n";
for (const grp of [fu, shift, extend, hint, bd, bt]) {
  for (const [n, v] of grp) out += `pub const ${rustUpper(n)}: i32 = ${BigInt(v)};\n`;
}
out += "\n// ── StatusFlags (由系统字段掩码推导, 1:1 保留表达式形态) ──\n";
for (const [n, v] of status) {
  out += `pub const ${rustUpper(n)}: u32 = ${translate(v)};\n`;
}
out += "\n// ── SystemRegister 编码 (NZCV/FPCR, 多域拼接) ──\n";
for (const [n, v] of sysreg) {
  out += `pub const ${rustUpper(n)}: i32 = ${translate(v)};\n`;
}
const variant = (n) => {
  const s = n.replace(/^k/, "");
  return s[0].toUpperCase() + s.slice(1);
};
out += `
// ── BranchTargetIdentifier (enum class, 判别值 0..5) ──
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BranchTargetIdentifier {
${bti.map((n, i) => `  ${variant(n)} = ${i},`).join("\n")}
}

// static_assert(kAddressTagMask == 0xff00000000000000) 对齐 constants-arm64.h:136
const _: () = assert!(K_ADDRESS_TAG_MASK == 0xFF00_0000_0000_0000_u64);
`;

const path = writeSrc("v8-codegen-arm64", "constants.rs", out);
console.log(
  `constants-arm64.h.js: 校验通过! 标量=${constsA.size} generic域=${constsGF.size} ` +
  `字段=${fieldDefs.length}(主79+系统9) Condition=${cond.length} Status=${status.length} ` +
  `Shift=${shift.length} Extend=${extend.length} Hint=${hint.length} 输出 ${path}`,
);
