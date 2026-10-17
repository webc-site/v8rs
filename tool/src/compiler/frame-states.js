#!/usr/bin/env bun
// v8/src/compiler/frame-states.cc + frame-states.h
// → v8-compiler-passes/src/frame_states.rs.skel
//
// 规则面: OutputFrameStateCombine (kInvalidIndex=SIZE_MAX, Ignore/PokeAt/
// ConsumedOutputCount 语义)、FrameStateType 枚举 (10 变体, WASM 条件 3)、
// FrameStateFunctionInfo (IsJSFunctionType 判据 3 项) + operator== 静态断言。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/frame-states.h");
const cc = readV8("src/compiler/frame-states.cc");

// —— 哨兵 ——
const sentinels = [
  [h, /class OutputFrameStateCombine \{/, "OutputFrameStateCombine"],
  [h, /static const size_t kInvalidIndex = SIZE_MAX;/, "kInvalidIndex"],
  [h, /enum class FrameStateType \{/, "FrameStateType"],
  [h, /class FrameStateFunctionInfo \{/, "FrameStateFunctionInfo"],
  [h, /static bool IsJSFunctionType\(FrameStateType type\) \{/, "IsJSFunctionType"],
  [cc, /bool operator==\(FrameStateFunctionInfo const& lhs,/, "operator== 实现"],
  [cc, /size_t hash_value\(FrameStateInfo const& info\) \{/, "hash_value 实现"],
  [cc, /static_assert\(sizeof\(FrameStateInfo\) == 24\);/, "sizeof 断言"],
];
for (const [src, re, name] of sentinels) {
  if (!re.test(src)) throw new Error(`哨兵缺失: ${name}`);
}

// —— FrameStateType 变体 (WASM 条件 3 项也计入) ——
const fst = h.match(/enum class FrameStateType \{([\s\S]*?)\n\};/)[1];
const variants = [...fst.matchAll(/^\s*k(\w+),?[ \t]*(?=\/\/|$)/gm)].map((m) => m[1]);
if (variants.join() !== "UnoptimizedFunction,InlinedExtraArguments,ConstructCreateStub,ConstructInvokeStub,BuiltinContinuation,JSToWasmBuiltinContinuation,WasmInlinedIntoJS,LiftoffFunction,JavaScriptBuiltinContinuation,JavaScriptBuiltinContinuationWithCatch") {
  throw new Error(`FrameStateType 漂移: ${variants}`);
}
const wasmVariants = variants.filter((v) => ["JSToWasmBuiltinContinuation", "WasmInlinedIntoJS", "LiftoffFunction"].includes(v));
// IsJSFunctionType 判据
if (!/type == FrameStateType::kUnoptimizedFunction \|\|\n\s*type == FrameStateType::kJavaScriptBuiltinContinuation \|\|\n\s*type == FrameStateType::kJavaScriptBuiltinContinuationWithCatch;/.test(h)) {
  throw new Error("IsJSFunctionType 判据漂移");
}

const body = `${banner("src/compiler/frame-states.js", "src/compiler/frame-states.h + src/compiler/frame-states.cc", "FrameState 族 — OutputFrameStateCombine / FrameStateType / FrameStateFunctionInfo")}
//! 惰性 bailout 帧状态描述。
//!
//! 规则面: 枚举 ${variants.length} 变体 (WASM 条件 ${wasmVariants.length} 项) +
//! Combine 语义 + IsJSFunctionType 判据 (与 TRANSLATION_JS_FRAME_OPCODE_LIST
//! 同步约束)。语义面 (推迟): FrameStateInfo operator==/hash (cc:79-101)。

#![allow(dead_code)]

/// v8: src/compiler/frame-states.h OutputFrameStateCombine
/// 参数索引 (kInvalidIndex = SIZE_MAX 即忽略输出)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OutputFrameStateCombine {
  /// 原 parameter_
  parameter: usize,
}

/// v8: src/compiler/frame-states.h OutputFrameStateCombine::kInvalidIndex
pub const K_INVALID_INDEX: usize = usize::MAX;

#[allow(non_snake_case)]
impl OutputFrameStateCombine {
  /// v8: src/compiler/frame-states.h OutputFrameStateCombine::Ignore
  pub const fn Ignore() -> Self {
    Self { parameter: K_INVALID_INDEX }
  }
  /// v8: src/compiler/frame-states.h OutputFrameStateCombine::PokeAt
  pub const fn PokeAt(index: usize) -> Self {
    Self { parameter: index }
  }
  /// v8: src/compiler/frame-states.h OutputFrameStateCombine::GetOffsetToPokeAt
  /// 原文: DCHECK_NE(parameter_, kInvalidIndex)
  pub fn get_offset_to_poke_at(&self) -> usize {
    debug_assert_ne!(self.parameter, K_INVALID_INDEX);
    self.parameter
  }
  /// v8: src/compiler/frame-states.h OutputFrameStateCombine::IsOutputIgnored
  pub const fn is_output_ignored(&self) -> bool {
    self.parameter == K_INVALID_INDEX
  }
  /// v8: src/compiler/frame-states.h OutputFrameStateCombine::ConsumedOutputCount
  /// 原文: IsOutputIgnored() ? 0 : 1
  pub const fn consumed_output_count(&self) -> usize {
    if self.is_output_ignored() { 0 } else { 1 }
  }
}

/// v8: src/compiler/frame-states.h FrameStateType
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FrameStateType {
  /// kUnoptimizedFunction — UnoptimizedJSFrame
  UnoptimizedFunction,
  /// kInlinedExtraArguments
  InlinedExtraArguments,
  /// kConstructCreateStub
  ConstructCreateStub,
  /// kConstructInvokeStub
  ConstructInvokeStub,
  /// kBuiltinContinuation
  BuiltinContinuation,
  /// kJSToWasmBuiltinContinuation (V8_ENABLE_WEBASSEMBLY)
  JSToWasmBuiltinContinuation,
  /// kWasmInlinedIntoJS (V8_ENABLE_WEBASSEMBLY)
  WasmInlinedIntoJS,
  /// kLiftoffFunction (V8_ENABLE_WEBASSEMBLY)
  LiftoffFunction,
  /// kJavaScriptBuiltinContinuation
  JavaScriptBuiltinContinuation,
  /// kJavaScriptBuiltinContinuationWithCatch
  JavaScriptBuiltinContinuationWithCatch,
}

/// v8: src/compiler/frame-states.h FrameStateFunctionInfo
pub struct FrameStateFunctionInfo {
  /// 原 type_
  pub ty: FrameStateType,
  /// 原 parameter_count_ (uint16)
  pub parameter_count: u16,
  /// 原 max_arguments_ (uint16)
  pub max_arguments: u16,
  /// 原 local_count_ (int)
  pub local_count: i32,
  /// 原 wasm_liftoff_frame_size_ (uint32, 默认 0)
  pub wasm_liftoff_frame_size: u32,
  /// 原 wasm_function_index_ (uint32, 默认 -1 截断)
  pub wasm_function_index: u32,
  /// 原 shared_info_
  pub shared_info: crate::handles::IndirectHandle<crate::objects::SharedFunctionInfo>,
  /// 原 bytecode_array_
  pub bytecode_array: Option<crate::handles::IndirectHandle<crate::objects::BytecodeArray>>,
}

impl FrameStateFunctionInfo {
  /// v8: src/compiler/frame-states.h FrameStateFunctionInfo::local_count 等 getter 面 8 项
  pub fn local_count(&self) -> i32 {
    self.local_count
  }
  /// v8: src/compiler/frame-states.h parameter_count_without_receiver
  /// 原文: DCHECK_GT(parameter_count_, 0); return parameter_count_ - 1;
  pub fn parameter_count_without_receiver(&self) -> u16 {
    debug_assert!(self.parameter_count > 0);
    self.parameter_count - 1
  }
  /// v8: src/compiler/frame-states.h FrameStateFunctionInfo::IsJSFunctionType
  /// 判据 3 项 (须与 TRANSLATION_JS_FRAME_OPCODE_LIST 同步)
  pub fn is_js_function_type(ty: FrameStateType) -> bool {
    matches!(
      ty,
      FrameStateType::UnoptimizedFunction
        | FrameStateType::JavaScriptBuiltinContinuation
        | FrameStateType::JavaScriptBuiltinContinuationWithCatch
    )
  }
}

// cc:79-101: operator== / hash_value / sizeof(FrameStateInfo)==24 断言
// 语义面: FrameStateInfo 相等 = 各字段 + function_info 内容相等
`;

const p = writeSrc("v8-compiler-passes", "frame_states.rs.skel", body);
console.log(`written ${p}\nFrameStateType=${variants.length} (WASM条件=${wasmVariants.length}) getter面=8 IsJS判据=3`);
