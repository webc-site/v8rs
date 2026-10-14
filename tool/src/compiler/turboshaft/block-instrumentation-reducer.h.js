#!/usr/bin/env bun
// tool/src/compiler/turboshaft/block-instrumentation-reducer.h.js
// 机械提取 v8/src/compiler/turboshaft/block-instrumentation-reducer.{h,cc}
// → v8-turboshaft/src/block_instrumentation_reducer.rs.skel
//
// 提取面: BlockInstrumentationReducer (块级计数插桩) — REDUCE 3 方法 +
// ReduceOperation 全 opcode 钩子 (跳过块首保留操作 static_assert ×3) +
// Bind 重置逻辑 + detail::CreateCountersArray (cc)。语义实现禁编造。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/block-instrumentation-reducer.h");
const cc = readV8("src/compiler/turboshaft/block-instrumentation-reducer.cc");

// 1. 类 + boilerplate
if (!/class BlockInstrumentationReducer\s*:\s*public UniformReducerAdapter<BlockInstrumentationReducer, Next>/.test(h)) {
  throw new Error("类基类链不匹配 (UniformReducerAdapter)");
}
if (!h.includes("TURBOSHAFT_REDUCER_BOILERPLATE(BlockInstrumentation)")) {
  throw new Error("未找到 TURBOSHAFT_REDUCER_BOILERPLATE(BlockInstrumentation)");
}

// 2. REDUCE 方法 (3)
const reduceNames = [...h.matchAll(/\bREDUCE\((\w+)\)\s*\(/g)].map((m) => m[1]);
if (JSON.stringify(reduceNames) !== JSON.stringify(["Parameter", "CatchBlockBegin", "DidntThrow"])) {
  throw new Error(`REDUCE 名单 ${JSON.stringify(reduceNames)} != [Parameter, CatchBlockBegin, DidntThrow]`);
}

// 3. ReduceOperation 钩子的三个跳过 static_assert + Bind 重置 + 插桩触发
for (const [re, label] of [
  [/template <Opcode opcode, typename Continuation, typename\.\.\. Args>\s*OpIndex ReduceOperation\(Args\.\.\. args\)/, "ReduceOperation 模板"],
  [/static_assert\(opcode != Opcode::kCatchBlockBegin\);/, "跳过 kCatchBlockBegin"],
  [/static_assert\(opcode != Opcode::kDidntThrow\);/, "跳过 kDidntThrow"],
  [/static_assert\(opcode != Opcode::kParameter\);/, "跳过 kParameter"],
  [/void Bind\(Block\* new_block\)/, "Bind 重置计数"],
  [/if \(0 == operations_emitted_in_current_block_\+\+\)/, "块首操作触发插桩"],
  [/EmitBlockInstrumentation\(block_number\)/, "EmitBlockInstrumentation"],
  [/data_->SetBlockId\(block_number, block_number\);/, "SetBlockId"],
  [/counters_array_handle_ = detail::CreateCountersArray\(isolate_\);/, "on_heap_counters_ 分支"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${label}`);
}

// 4. cc: detail::CreateCountersArray 定义 (basic_block_counters_marker)
for (const [re, label] of [
  [/Handle<HeapObject> CreateCountersArray\(Isolate\* isolate\)/, "CreateCountersArray 定义"],
  [/ReadOnlyRoots\(isolate\)\.basic_block_counters_marker\(\)/, "basic_block_counters_marker 根"],
]) {
  if (!re.test(cc)) throw new Error(`cc 哨兵缺失: ${label}`);
}

// 5. 生成 Rust
let rust = banner(
  "src/compiler/turboshaft/block-instrumentation-reducer.h.js",
  "src/compiler/turboshaft/block-instrumentation-reducer.{h,cc}",
  "基本块计数插桩 reducer (d8 --trace-turbo 计数骨架)",
);
rust += `
/// v8: src/compiler/turboshaft/block-instrumentation-reducer.h BlockInstrumentationReducer
///
/// 策略 (原版保留):
/// - kCatchBlockBegin/kDidntThrow/kParameter 保留在块首, 不插桩
/// - 其余块内首个操作前 EmitBlockInstrumentation(block_number)
/// - Bind 时 SetBlockId + 计数清零; on_heap_counters_ 下预建计数数组
pub struct BlockInstrumentationReducer {
  /// operations_emitted_in_current_block_
  pub operations_emitted_in_current_block: i32,
  /// counters_array_handle_ (on_heap_counters_)
  pub counters_array_handle: Option<()>,
}

impl BlockInstrumentationReducer {
`;
const methods = [
  ["bind", "void Bind(Block* new_block)", "块切换: SetBlockId + 计数清零"],
  ["reduce_operation", "template <Opcode, Continuation, Args...> OpIndex ReduceOperation(Args...)", "全 opcode 钩子: 块首触发插桩"],
  ["emit_block_instrumentation", "EmitBlockInstrumentation(int block_number)", "插桩调用 (CountersArray 写入)"],
];
for (const [rustName, sig, comment] of methods) {
  rust += `  /// 原版 ${sig} — ${comment}
  pub fn ${rustName}(&mut self) {
    unimplemented!("签名骨架, Developer 修订")
  }
`;
}
for (const name of reduceNames) {
  const snakeName = name.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();
  rust += `  /// 原版 REDUCE(${name}) — 块首保留操作直通 Next
  /// v8: src/compiler/turboshaft/block-instrumentation-reducer.h BlockInstrumentationReducer::REDUCE(${name})
  pub fn reduce_${snakeName}(&mut self) {
    unimplemented!("签名骨架, Developer 修订")
  }
`;
}
rust += `}

/// v8: src/compiler/turboshaft/block-instrumentation-reducer.cc detail::CreateCountersArray
pub fn create_counters_array(isolate: &mut crate::phase::Isolate) -> Option<()> {
  // 原版: ReadOnlyRoots(isolate).basic_block_counters_marker()
  unimplemented!("签名骨架, Developer 修订")
}
`;

const p = writeSrc("v8-turboshaft", "block_instrumentation_reducer.rs.skel", rust);
console.log(`written ${p}`);
console.log(`类=BlockInstrumentationReducer REDUCE=${reduceNames.length} 钩子=3 方法面=${methods.length} cc锚=2`);
