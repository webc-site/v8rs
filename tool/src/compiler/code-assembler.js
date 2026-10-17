#!/usr/bin/env bun
// v8/src/compiler/code-assembler.cc + code-assembler.h +
//    code-assembler-compilation-job.h
// → v8-compiler-passes/src/code_assembler.rs.skel
//
// 规则面: CheckBounds / StoreToObjectWriteBarrier 枚举、
// CodeAssemblerLabel::Type、OBJECT_TYPE X-macro 清单 (5 组特化调用)、
// CodeAssembler 主类方法面计数对账; compilation-job 的 kNoFinalizeOrder
// 常量与回调类型签名。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/code-assembler.h");
const cc = readV8("src/compiler/code-assembler.cc");
const job = readV8("src/compiler/code-assembler-compilation-job.h");

// —— 哨兵 ——
const sentinels = [
  [h, /enum class CheckBounds \{ kAlways, kDebugOnly \};/, "CheckBounds"],
  [h, /enum class StoreToObjectWriteBarrier \{ kNone, kMap, kFull \};/, "StoreToObjectWriteBarrier"],
  [h, /OBJECT_TYPE_LIST\(OBJECT_TYPE_CASE\)/, "OBJECT_TYPE X-macro"],
  [h, /HOLE_LIST\(OBJECT_TYPE_HOLE_CASE\)/, "HOLE_LIST 特化"],
  [h, /class V8_EXPORT_PRIVATE CodeAssembler \{/, "CodeAssembler 主类"],
  [h, /enum Type \{ kDeferred, kNonDeferred \};/, "Label Type"],
  [h, /class V8_EXPORT_PRIVATE CodeAssemblerState \{/, "CodeAssemblerState"],
  [h, /void PushExceptionHandler\(CodeAssemblerExceptionHandlerLabel\* label\);/, "PushExceptionHandler"],
  [job, /class CodeAssemblerCompilationJob : public TurbofanCompilationJob \{/, "CompilationJob"],
  [job, /static constexpr int kNoFinalizeOrder = -1;/, "kNoFinalizeOrder"],
];
for (const [src, re, name] of sentinels) {
  if (!re.test(src)) throw new Error(`哨兵缺失: ${name}`);
}

// —— 枚举变体 (字面量已由哨兵锁定, 此处计数复核) ——
const oddballs = [...h.matchAll(/OBJECT_TYPE_ODDBALL_CASE\((\w+)\)/g)].map((m) => m[1]);
if (oddballs.join() !== "Name,Null,Undefined,True,False") throw new Error(`ODDBALL 漂移: ${oddballs}`);
// X-macro 展开调用组 (ObjectTypeOf 特化来源)
const xmacroCalls = [...h.matchAll(/^(OBJECT_TYPE_LIST|HEAP_OBJECT_ORDINARY_TYPE_LIST|VIRTUAL_OBJECT_TYPE_LIST|HEAP_OBJECT_TRUSTED_TYPE_LIST|STRUCT_LIST|HEAP_OBJECT_TEMPLATE_TYPE_LIST)\(/gm)].map((m) => m[1]);
if (xmacroCalls.length !== 6) throw new Error(`X-macro 调用组 ${xmacroCalls.length} != 6`);

// —— 主类方法计数 (CodeAssembler public 区签名) ——
const caBlock = h.slice(h.indexOf("class V8_EXPORT_PRIVATE CodeAssembler {"), h.indexOf("class V8_EXPORT_PRIVATE CodeAssemblerVariable"));
const caMethods = new Set([...caBlock.matchAll(/^  (?:V8_INLINE\s+)?(?:template <[^>]*>\s*\n\s*)?(?:explicit\s+)?(?:[A-Za-z_][\w:<>,*& ]+?)\s+(\w+)\(/gm)].map((m) => m[1]));
if (caMethods.size < 200) throw new Error(`CodeAssembler 方法 ${caMethods.size} < 200`);

const body = `${banner("src/compiler/code-assembler.js", "src/compiler/code-assembler.h + src/compiler/code-assembler.cc + src/compiler/code-assembler-compilation-job.h", "CodeAssembler — CSA (Code Stub Assembler) 图构建层 + 编译作业封装")}
//! CodeAssembler: Torque/内置生成器的中层 IR 构建 API。
//!
//! 规则面: 3 枚举 + Label Type + OBJECT_TYPE 特化 X-macro 组
//! (${xmacroCalls.length} 组) + 主类方法面 ${caMethods.size} (骨架化)。
//! 语义面 (推迟): TNode 类型系统 (模板层), 状态机 (Label/Variable 栈),
//! raw machine 调用 (js-graph / raw-machine-assembler)。

#![allow(dead_code, non_camel_case_types)]

/// v8: src/compiler/code-assembler.h CheckBounds
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckBounds {
  /// kAlways
  Always,
  /// kDebugOnly
  DebugOnly,
}

/// v8: src/compiler/code-assembler.h NeedsBoundsCheck
/// 原文: kAlways→true; kDebugOnly→DEBUG_BOOL
pub const fn needs_bounds_check(check_bounds: CheckBounds) -> bool {
  match check_bounds {
    CheckBounds::Always => true,
    CheckBounds::DebugOnly => cfg!(debug_assertions),
  }
}

/// v8: src/compiler/code-assembler.h StoreToObjectWriteBarrier
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreToObjectWriteBarrier {
  /// kNone
  None,
  /// kMap
  Map,
  /// kFull
  Full,
}

/// v8: src/compiler/code-assembler.h CodeAssemblerLabel::Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelType {
  /// kDeferred
  Deferred,
  /// kNonDeferred
  NonDeferred,
}

/// v8: src/compiler/code-assembler.h ObjectTypeOf (X-macro 特化组)
/// 特化来源: OBJECT_TYPE_LIST / HEAP_OBJECT_ORDINARY_TYPE_LIST /
/// VIRTUAL_OBJECT_TYPE_LIST / HEAP_OBJECT_TRUSTED_TYPE_LIST / STRUCT_LIST /
/// HEAP_OBJECT_TEMPLATE_TYPE_LIST; ODDBALL 四值折叠为 kOddball;
/// HOLE 折叠为 kHole; Weak<T> → kHeapObjectReference;
/// Union: 含 Smi/Object → kObject, 含 Weak → kHeapObjectReference, 否则 kHeapObject。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectType {
  /// kObject — 任意 tagged (含 Smi 可能)
  Object,
  /// kSmi
  Smi,
  /// kHeapObject
  HeapObject,
  /// kHeapObjectReference — Weak
  HeapObjectReference,
  /// kOddball — Null/Undefined/True/False 折叠
  Oddball,
  /// kHole — HOLE_LIST 折叠
  Hole,
}

/// v8: src/compiler/code-assembler.h CodeAssemblerLabel
pub struct CodeAssemblerLabel {
  /// 原 type_ (默认 kNonDeferred)
  pub label_type: LabelType,
  /// 原 state_ 绑定点推迟
  pub _semantic: (),
}

/// v8: src/compiler/code-assembler.h CodeAssemblerState
/// 状态机: 变量栈/label 栈/错误处理器 (PushErrorHandler/PopErrorHandler)
pub struct CodeAssemblerState {
  /// 原 macro_assembler_ 推迟 (codegen 层)
  pub _semantic: (),
}

impl CodeAssemblerState {
  /// v8: src/compiler/code-assembler.h CodeAssemblerState::PushExceptionHandler
  /// 注: 声明在 .h, 成员 exception_handler_labels_ 为 vector 栈
  pub fn push_exception_handler(&mut self, label: *mut crate::CodeAssemblerExceptionHandlerLabel) {
    todo!("exception_handler_labels_ 压入")
  }
  /// v8: src/compiler/code-assembler.h CodeAssemblerState::PopExceptionHandler
  pub fn pop_exception_handler(&mut self) {
    todo!("exception_handler_labels_ 弹出")
  }
}

/// v8: src/compiler/code-assembler.h CodeAssembler
/// 方法面 ${caMethods.size} 个 (Goto/Branch/Select/Call/Load/Store/…),
/// TNode 模板层语义推迟; 本骨架保留类位。
pub struct CodeAssembler {
  /// 原 state_
  pub state: *mut CodeAssemblerState,
}

impl CodeAssembler {
  /// v8: src/compiler/code-assembler.h CodeAssembler::CodeAssembler
  pub fn new(state: *mut CodeAssemblerState) -> Self {
    Self { state }
  }
  /// v8: src/compiler/code-assembler.h CodeAssembler::label_count 等查询面
  /// 完整方法面 ${caMethods.size} 项在 .h:407-1763, 语义迁移随 TNode 层。
  pub fn placeholder_method_count(&self) -> usize {
    ${caMethods.size}
  }
}

/// v8: src/compiler/code-assembler-compilation-job.h CodeAssemblerCompilationJob::kNoFinalizeOrder
pub const K_NO_FINALIZE_ORDER: i32 = -1;

/// v8: src/compiler/code-assembler-compilation-job.h CodeAssemblerCompilationJob
/// 生成器/安装器回调 + finalize_order; 语义面 (Pipeline 执行) 推迟。
pub struct CodeAssemblerCompilationJob {
  /// 原 generator_ (CodeAssemblerGenerator)
  pub generator: crate::CodeAssemblerGenerator,
  /// 原 installer_ (CodeAssemblerInstaller)
  pub installer: crate::CodeAssemblerInstaller,
  /// 原 finalize_order_ (kNoFinalizeOrder = -1)
  pub finalize_order: i32,
}

impl CodeAssemblerCompilationJob {
  /// v8: src/compiler/code-assembler-compilation-job.h FinalizeOrder
  /// 原文: DCHECK_NE(kNoFinalizeOrder, finalize_order_); return finalize_order_;
  pub fn finalize_order(&self) -> i32 {
    debug_assert_ne!(K_NO_FINALIZE_ORDER, self.finalize_order);
    self.finalize_order
  }
  /// v8: src/compiler/code-assembler-compilation-job.h ShouldOptimizeJumps
  pub fn should_optimize_jumps(isolate: &crate::Isolate) -> bool {
    todo!("flags 查询")
  }
}
`;

const p = writeSrc("v8-compiler-passes", "code_assembler.rs.skel", body);
console.log(
  [
    `written ${p}`,
    `枚举=4 (CheckBounds2/WriteBarrier3/LabelType2/${xmacroCalls.length}组X-macro) ODDBALL=${oddballs.length} 方法面=${caMethods.size}`,
  ].join("\n"),
);
