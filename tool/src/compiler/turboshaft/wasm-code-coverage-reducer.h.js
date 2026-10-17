#!/usr/bin/env bun
// tool/src/compiler/turboshaft/wasm-code-coverage-reducer.h.js
// 机械提取 v8/src/compiler/turboshaft/wasm-code-coverage-reducer.h
// → v8-turboshaft/src/wasm_code_coverage_reducer.rs.skel
//
// 提取面: REDUCE(WasmIncCoverageCounter) 的四步计数序列。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/wasm-code-coverage-reducer.h");

// 1. 哨兵
for (const [re, name] of [
  [/template <class Next>\s*\n?class WasmCodeCoverageReducer : public Next/, "reducer 类"],
  [/TURBOSHAFT_REDUCER_BOILERPLATE\(WasmCodeCoverageReducer\)/, "boilerplate"],
  [/V<None> REDUCE\(WasmIncCoverageCounter\)\(Address address\)/, "REDUCE 签名"],
  [/reinterpret_cast<uintptr_t>\(address\)/, "地址转 WordPtr 常量"],
  [/LoadOffHeap\(counter_address, 0, MemoryRepresentation::Uint32\(\)\)/, "OffHeap 读"],
  [/Word32Add\(value, 1\)/, "自增"],
  [/StoreOffHeap\(counter_address, incremented_value,\s*\n\s*MemoryRepresentation::Uint32\(\), 0\)/, "OffHeap 写"],
  [/return V<None>::Invalid\(\);/, "无返回值"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}

let rust = banner(
  "src/compiler/turboshaft/wasm-code-coverage-reducer.h.js",
  "src/compiler/turboshaft/wasm-code-coverage-reducer.h",
  "wasm 代码覆盖: 计数器自增",
);

rust += `
/// 原版 WasmCodeCoverageReducer — 在给定地址上原子意义的三步计数
/// (非原子: LoadOffHeap -> +1 -> StoreOffHeap, Uint32)。
/// v8: src/compiler/turboshaft/wasm-code-coverage-reducer.h
pub struct WasmCodeCoverageReducer;

impl WasmCodeCoverageReducer {
  /// v8: REDUCE(WasmIncCoverageCounter)(address)
  /// 1. counter_address = WordPtrConstant(address as uintptr)
  /// 2. value = LoadOffHeap(counter_address, 0, Uint32)
  /// 3. incremented = Word32Add(value, 1)
  /// 4. StoreOffHeap(counter_address, incremented, Uint32, 0)
  /// 返回 V<None>::Invalid()
  pub fn reduce_wasm_inc_coverage_counter(&mut self, address: usize) -> V<None> { todo!() }
}
`;

const p = writeSrc("v8-turboshaft", "wasm_code_coverage_reducer.rs.skel", rust);
console.log(`written ${p}`);
console.log(`REDUCE=1 步骤=4`);
