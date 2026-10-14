#!/usr/bin/env bun
// tool/src/compiler/turboshaft/wasm-lowering-reducer.h.js
// 机械提取 v8/src/compiler/turboshaft/wasm-lowering-reducer.h
// → v8-turboshaft/src/wasm_lowering_reducer.rs.skel
//
// 提取面: 全部 REDUCE 面 (~25 个 wasm GC 操作); GlobalMode 枚举;
// kMaybeSandboxedPointer; 字符串 prepare 决策位。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/wasm-lowering-reducer.h");

// 1. REDUCE 面
const reduces = [...h.matchAll(/\bREDUCE\((\w+)\)\(/g)].map((m) => m[1]);
if (reduces.length < 24) throw new Error(`REDUCE ${reduces.length} < 24`);
for (const must of ["GlobalGet", "GlobalSet", "Null", "IsNull", "AssertNotNull", "RttCanon",
  "WasmTypeCheck", "WasmTypeCast", "AnyConvertExtern", "ExternConvertAny", "StructGet",
  "StructSet", "ArrayGet", "ArraySet", "ArrayLength", "WasmAllocateArray", "WasmAllocateStruct",
  "WasmRefFunc", "StringAsWtf16", "StringPrepareForGetCodeUnit"]) {
  if (!reduces.includes(must)) throw new Error(`缺 REDUCE(${must})`);
}

// 2. 枚举/常量
const gmM = h.match(/enum class GlobalMode \{ ([^}]+)\}/);
if (!gmM) throw new Error("未找到 GlobalMode");
const gm = [...gmM[1].replace(/\/\/[^\n]*/g, "").matchAll(/k(\w+)/g)].map((m) => m[1]);
if (gm.length !== 2) throw new Error(`GlobalMode ${gm.length} != 2`);
const sandboxed = h.match(/kMaybeSandboxedPointer =\s*\n?\s*V8_ENABLE_SANDBOX_BOOL \? MemoryRepresentation::(\w+)\(\)\s*\n?\s*: MemoryRepresentation::(\w+)\(\);/);
if (!sandboxed) throw new Error("未找到 kMaybeSandboxedPointer");

// 3. 决策位哨兵
for (const [re, name] of [
  [/static constexpr int kIsDirectStringTag = 0;/, "直接字符串决策位"],
  [/class WasmLoweringReducer : public Next/, "reducer 类"],
  [/TURBOSHAFT_REDUCER_BOILERPLATE\(WasmLowering\)/, "boilerplate"],
  [/REDUCE\(StringPrepareForGetCodeUnit\)/, "字符串 prepare 复合结果"],
  [/WasmTrustedInstanceData/, "实例数据类型引用 (helpers 宏面)"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}

let rust = banner(
  "src/compiler/turboshaft/wasm-lowering-reducer.h.js",
  "src/compiler/turboshaft/wasm-lowering-reducer.h",
  "wasm 降低: GC 操作到机器操作",
);

rust += `
/// 原版 WasmLoweringReducer — wasm GC 全操作降低
/// REDUCE 面 (${reduces.length} 个):
${reduces.map((r) => `/// - REDUCE(${r})`).join("\n")}
/// 要点:
/// - GlobalGet/Set 按 GlobalMode (${gm.join("/")}): 实例字段直载 / 反射表
/// - 指针经 kMaybeSandboxedPointer: sandbox 开 -> ${sandboxed[1]},
///   否则 ${sandboxed[2]}
/// - WasmTypeCheck/Cast: null 检查 + map 判 + 子类型判组合
/// - Struct/Array 存取: 写屏障经 wasm-assembler-helpers 换算
/// - WasmAllocate*: Map + 长度 (kMaxUInt32 饱和) + 分配折叠友好运
/// - StringPrepareForGetCodeUnit: 返回 (String, offset, charwidth_shift)
///   复合结果, kIsDirectStringTag 等决策位打包
/// v8: src/compiler/turboshaft/wasm-lowering-reducer.h
pub struct WasmLoweringReducer;

/// 原版 GlobalMode — 全局变量存取模式
/// v8: wasm-lowering-reducer.h WasmLoweringReducer::GlobalMode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GlobalMode {
  /// kLoad: 只读全局 (实例字段直接载入)
  ${gm[0]},
  /// kStore: 反射表路径
  ${gm[1]},
}
`;

const p = writeSrc("v8-turboshaft", "wasm_lowering_reducer.rs.skel", rust);
console.log(`written ${p}`);
console.log(`REDUCE=${reduces.length} GlobalMode=${gm.length}`);
