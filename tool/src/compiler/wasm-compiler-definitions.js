#!/usr/bin/env bun
// tool/src/compiler/wasm-compiler-definitions.js
// v8/src/compiler/wasm-compiler-definitions.h + wasm-compiler-definitions.cc
// → v8-compiler-passes/src/wasm_compiler_definitions.rs.skel
// wasm 编译定义: 解析 SubtypeCheckExactness / WasmTypeCheckConfig /
// NullCheckStrategy / BoundsCheckResult / CheckForNull / WasmCallKind 枚举。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/wasm-compiler-definitions.h");
const cc = readV8("src/compiler/wasm-compiler-definitions.cc");

const need = (cond, msg) => {
  if (!cond) throw new Error(`wasm-compiler-definitions: ${msg}`);
};

for (const [s, name] of [
  ["enum SubtypeCheckExactness : uint8_t {", "SubtypeCheckExactness"],
  ["struct WasmTypeCheckConfig {", "WasmTypeCheckConfig"],
  ["static constexpr int kCharWidthBailoutSentinel = 3;", "kCharWidthBailoutSentinel"],
  ["enum class NullCheckStrategy { kExplicit, kTrapHandler };", "NullCheckStrategy"],
  ["enum class BoundsCheckResult {", "BoundsCheckResult"],
  ["enum CheckForNull : bool { kWithoutNullCheck, kWithNullCheck };", "CheckForNull"],
  ["enum WasmCallKind {", "WasmCallKind"],
]) need(h.includes(s), `哨兵缺失 ${name}`);

// —— 枚举成员 (真解析, 顺序敏感; 先到 }; 再剥注释, 避免 "checked" 假阳性) ——
const enumRegion = (start) => {
  const from = h.indexOf(start);
  return h.slice(from, h.indexOf("};", from)).replace(/\/\/[^\n]*/g, "");
};
const exactness = [...enumRegion("enum SubtypeCheckExactness").matchAll(/\bk(\w+),/g)].map((m) => m[1]).slice(0, 3);
need(exactness.length === 3 && exactness[0] === "MayBeSubtype" && exactness[2] === "ExactMatchLastSupertype", `exactness ${exactness}`);

const bounds = [...enumRegion("enum class BoundsCheckResult").matchAll(/\bk(\w+)/g)].map((m) => m[1]).slice(0, 3);
need(bounds.length === 3 && bounds.join(",") === "DynamicallyChecked,TrapHandler,InBounds", `bounds ${bounds}`);

const callKinds = [...enumRegion("enum WasmCallKind").matchAll(/\bk(\w+),?/g)].map((m) => m[1]).slice(0, 5);
need(callKinds.length === 5 && callKinds[0] === "WasmFunction" && callKinds[4] === "WasmContinuation", `callKind ${callKinds}`);

// —— WasmTypeCheckConfig 字段 ——
const wtc = h.slice(h.indexOf("struct WasmTypeCheckConfig"));
const wtcFields = [...wtc.slice(0, wtc.indexOf("bool is_valid")).matchAll(/^\s{2}(?:const )?(?:wasm::ValueType|SubtypeCheckExactness) (\w+)/gm)].map((m) => m[1]);
need(wtcFields.length >= 3, `WTC 字段 ${wtcFields}`);

// —— cc 定义面 ——
const ccDefs = ["GetExactness", "GetTrapIdForTrap", "GetDebugName", "operator<<"];
for (const d of ccDefs) need(cc.includes(d) || h.includes(d), `cc 缺 ${d}`);

const body = `${banner("src/compiler/wasm-compiler-definitions.js", "src/compiler/wasm-compiler-definitions.h + wasm-compiler-definitions.cc", "wasm 编译公共定义 (类型检查配置/调用类别/空检查策略)")}
//! wasm-compiler-definitions — 跨 wasm 编译器的轻量契约
//!
//! SubtypeCheckExactness 3 值 (${exactness.join("/")});
//! BoundsCheckResult 3 值 (${bounds.join("/")});
//! WasmCallKind 5 值 (${callKinds.join("/")})。

/// 原版 SubtypeCheckExactness — 子类型检查精确度
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubtypeCheckExactness {
  /// kMayBeSubtype
  MayBeSubtype,
  /// kExactMatchOnly
  ExactMatchOnly,
  /// kExactMatchLastSupertype
  ExactMatchLastSupertype,
}

/// 原版 WasmTypeCheckConfig — 类型检查/转换参数 (${wtcFields.join("/")})
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WasmTypeCheckConfig {
  /// from
  pub from: wasm::ValueType,
  /// to
  pub to: wasm::ValueType,
  /// exactness
  pub exactness: SubtypeCheckExactness,
}

impl WasmTypeCheckConfig {
  /// 原版 is_valid — from/to 均引用且同类型层级
  /// v8: src/compiler/wasm-compiler-definitions.h WasmTypeCheckConfig::is_valid
  pub fn is_valid(&self) -> bool {
    todo!("v8: src/compiler/wasm-compiler-definitions.h WasmTypeCheckConfig::is_valid")
  }
}

/// 原版 kCharWidthBailoutSentinel — 字符宽度逃逸哨兵
pub const K_CHAR_WIDTH_BAILOUT_SENTINEL: i32 = 3;

/// 原版 NullCheckStrategy — 空检查策略
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NullCheckStrategy {
  /// kExplicit — 显式分支
  Explicit,
  /// kTrapHandler — trap handler
  TrapHandler,
}

/// 原版 EnforceBoundsCheck — 强类型 bool (StrongAlias)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnforceBoundsCheck(pub bool);

/// 原版 BoundsCheckResult — 越界检查结果
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoundsCheckResult {
  /// kDynamicallyChecked — 1-2 条件分支
  DynamicallyChecked,
  /// kTrapHandler — trap handler 承接
  TrapHandler,
  /// kInBounds — 静态界内
  InBounds,
}

/// 原版 CheckForNull — 空检查需求 (强类型 bool)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckForNull {
  /// kWithoutNullCheck
  WithoutNullCheck,
  /// kWithNullCheck
  WithNullCheck,
}

/// 原版 WasmCallKind — wasm 调用类别
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WasmCallKind {
  /// kWasmFunction — 同模块直调
  WasmFunction,
  /// kWasmIndirectFunction — 表间接调用
  WasmIndirectFunction,
  /// kWasmImportWrapper — 导入包装
  WasmImportWrapper,
  /// kWasmCapiFunction — C-API 宿主函数
  WasmCapiFunction,
  /// kWasmContinuation — 续体 (stack switching)
  WasmContinuation,
}

/// 原版 GetExactness — 模块目标类型的检查精确度
/// v8: src/compiler/wasm-compiler-definitions.cc GetExactness
pub fn get_exactness(module: &wasm::WasmModule, target: wasm::HeapType) -> SubtypeCheckExactness {
  todo!("v8: src/compiler/wasm-compiler-definitions.cc GetExactness")
}

/// 原版 GetTrapIdForTrap — TrapReason → TrapId
/// v8: src/compiler/wasm-compiler-definitions.cc GetTrapIdForTrap
pub fn get_trap_id_for_trap(reason: wasm::TrapReason) -> TrapId {
  todo!("v8: src/compiler/wasm-compiler-definitions.cc GetTrapIdForTrap")
}

/// 原版 GetDebugName — wasm 函数调试名
/// v8: src/compiler/wasm-compiler-definitions.cc GetDebugName
pub fn get_debug_name<'z>(zone: &'z Zone<'z>, module: &wasm::WasmModule, wire_bytes: &wasm::WireBytesStorage, index: i32) -> &'z str {
  todo!("v8: src/compiler/wasm-compiler-definitions.cc GetDebugName")
}
`;

const p = writeSrc("v8-compiler-passes", "wasm_compiler_definitions.rs.skel", body);
console.log(`written ${p}\nexactness=${exactness.length} bounds=${bounds.length} call_kinds=${callKinds.length} wtc_fields=${wtcFields.length}`);
