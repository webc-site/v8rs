#!/usr/bin/env bun
// tool/src/compiler/turboshaft/wasm-gc-typed-optimization-reducer.h.js
// 机械提取 v8/src/compiler/turboshaft/wasm-gc-typed-optimization-reducer.{h,cc}
// → v8-turboshaft/src/wasm_gc_typed_optimization_reducer.rs.skel
//
// 提取面: WasmGCTypeAnalyzer Process 面 (cc); reducer 的
// ReduceInputGraph 特化面; 类型侧表机制锚。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/wasm-gc-typed-optimization-reducer.h");
const cc = readV8("src/compiler/turboshaft/wasm-gc-typed-optimization-reducer.cc");

// 1. 类哨兵
for (const [re, name] of [
  [/class WasmGCTypeAnalyzer \{/, "分析器"],
  [/class WasmGCTypedOptimizationReducer : public Next/, "reducer 类"],
  [/TURBOSHAFT_REDUCER_BOILERPLATE\(WasmGCTypedOptimization\)/, "boilerplate"],
  [/using TypeSnapshotTable = SparseOpIndexSnapshotTable<wasm::ValueType>;/, "类型快照表"],
  [/FixedBlockSidetable<MaybeSnapshot> block_to_snapshot_/, "块快照侧表"],
  [/GetInputTypeOrSentinelType\(OpIndex op\) const/, "输入类型查询"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}

// 2. cc Process 面
const processes = [...cc.matchAll(/^void WasmGCTypeAnalyzer::Process(\w+)\(/gm)].map((m) => m[1]);
if (processes.length < 20) throw new Error(`Process 面 ${processes.length} < 20`);
for (const must of ["TypeCast", "TypeCheck", "AssertNotNull", "IsNull", "StructGet", "StructSet",
  "ArrayGet", "ArrayLength", "GlobalGet", "RefFunc", "AllocateArray"]) {
  if (!processes.includes(must)) throw new Error(`Process 缺 ${must}`);
}
const ccRun = /void WasmGCTypeAnalyzer::Run\(\)/.test(cc);

// 3. reducer 的 ReduceInputGraph 面
const rigOps = [...h.matchAll(/return Next::ReduceInputGraph(\w+)\(/g)].map((m) => m[1]);
const uniqueRig = [...new Set(rigOps)];
if (uniqueRig.length !== 9) throw new Error(`RIG 面 ${uniqueRig.length} != 9`);

// 4. REDUCE 特化面
const reduces = [...h.matchAll(/\bREDUCE\((\w+)\)\(/g)].map((m) => m[1]);

let rust = banner(
  "src/compiler/turboshaft/wasm-gc-typed-optimization-reducer.h.js",
  "src/compiler/turboshaft/wasm-gc-typed-optimization-reducer.{h,cc}",
  "wasm GC 类型优化: 输入图类型驱动化简",
);

rust += `
/// 原版 WasmGCTypeAnalyzer — 输入图上传播 wasm::ValueType:
/// types_table_ 为 SparseOpIndexSnapshotTable<wasm::ValueType>,
/// block_to_snapshot_ 为 FixedBlockSidetable<MaybeSnapshot>;
/// GetInputTypeOrSentinelType 返回输入类型或哨兵 (bottom = 必 trap)。
/// Process 分派面 (${processes.length} 个):
${processes.map((p) => `/// - Process${p}`).join("\n")}
/// v8: wasm-gc-typed-optimization-reducer.{h,cc} WasmGCTypeAnalyzer
pub struct WasmGCTypeAnalyzer<'g> {
  pub types: SparseOpIndexSnapshotTable<CanonicalValueType>,
}

impl<'g> WasmGCTypeAnalyzer<'g> {
  /// v8: cc Run — ${ccRun ? "全图块序扫描 + 快照合并" : ""}
  pub fn run(&mut self) { /* ... */ }
}

/// 原版 WasmGCTypedOptimizationReducer — 消费类型侧表做化简。
/// ReduceInputGraph 特化面 (${uniqueRig.length} 个):
${uniqueRig.map((o) => `/// - ReduceInputGraph${o}`).join("\n")}
/// 输出图 REDUCE 特化面 (${reduces.length} 个):
${reduces.map((r) => `/// - REDUCE(${r})`).join("\n")}
/// 典型化简: TypeCheck 已知命中/必败 -> 常量; TypeCast 源类型已兼容 ->
/// 透传; AssertNotNull 已知非空 -> 移除; StructGet/ArrayLength 用
/// 精确字段类型替换表示。
/// v8: wasm-gc-typed-optimization-reducer.h WasmGCTypedOptimizationReducer
pub struct WasmGCTypedOptimizationReducer;
`;

const p = writeSrc("v8-turboshaft", "wasm_gc_typed_optimization_reducer.rs.skel", rust);
console.log(`written ${p}`);
console.log(`Process=${processes.length} RIG=${uniqueRig.length} REDUCE=${reduces.length}`);
