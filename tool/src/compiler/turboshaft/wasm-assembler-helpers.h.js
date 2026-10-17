#!/usr/bin/env bun
// tool/src/compiler/turboshaft/wasm-assembler-helpers.h.js
// 机械提取 v8/src/compiler/turboshaft/wasm-assembler-helpers.h
// → v8-turboshaft/src/wasm_assembler_helpers.rs.skel
//
// 提取面: LOAD_* 实例字段宏 (4 变体); WriteBarrier 换算函数对。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/wasm-assembler-helpers.h");

// 1. LOAD 宏清单
const loadMacros = [...h.matchAll(/^#define (LOAD_\w+)\(instance, name, (\w+)\)/gm)].map((m) => ({
  name: m[1], third: m[2],
}));
if (loadMacros.length !== 4) throw new Error(`LOAD 宏 ${loadMacros.length} != 4`);
for (const must of ["LOAD_INSTANCE_FIELD", "LOAD_PROTECTED_INSTANCE_FIELD", "LOAD_IMMUTABLE_INSTANCE_FIELD", "LOAD_IMMUTABLE_PROTECTED_INSTANCE_FIELD"]) {
  if (!loadMacros.some((m) => m.name === must)) throw new Error(`缺 ${must}`);
}

// 2. 宏体差异
for (const [re, name] of [
  [/LoadOp::Kind::TaggedBase\(\)/, "TaggedBase 基址"],
  [/\.Immutable\(\)/, "Immutable 变体"],
  [/LoadProtectedPointerField/, "受保护指针"],
  [/WasmTrustedInstanceData::k##name##Offset/, "字段偏移拼接"],
  [/WasmTrustedInstanceData::kProtected##name##Offset/, "受保护字段偏移拼接"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}

// 3. wasm 命名空间函数
for (const fn of ["ArrayIndexImmediateToWriteBarrier", "FieldImmediateToWriteBarrier"]) {
  if (!new RegExp(`WriteBarrierKind ${fn}\\(`).test(h)) throw new Error(`缺 ${fn}`);
}

let rust = banner(
  "src/compiler/turboshaft/wasm-assembler-helpers.h.js",
  "src/compiler/turboshaft/wasm-assembler-helpers.h",
  "wasm 汇编 helper: 实例字段载入宏与写屏障换算",
);

rust += `
/// 原版 4 个 LOAD 宏 — 从 WasmTrustedInstanceData 载字段:
${loadMacros.map((m) => `/// - ${m.name}(instance, name, ${m.third}): ${m.name.includes("PROTECTED") ? "LoadProtectedPointerField + V<type>::Cast" : "__ Load + representation"}${m.name.includes("IMMUTABLE") ? " (TaggedBase().Immutable())" : " (TaggedBase())"}, 偏移 WasmTrustedInstanceData::k${m.name.includes("PROTECTED") ? "Protected" : ""}<name>Offset`).join("\n")}
/// Rust 侧以泛型函数替代宏拼接。
/// v8: src/compiler/turboshaft/wasm-assembler-helpers.h
pub fn load_instance_field<T: InstanceField>(
  asm: &mut Assembler, instance: V<WasmTrustedInstanceData>, name: T,
) -> V<T::Repr> { todo!() }
pub fn load_protected_instance_field<T: InstanceField>(
  asm: &mut Assembler, instance: V<WasmTrustedInstanceData>, name: T,
) -> V<T::Repr> { todo!() }

/// 原版 (namespace v8::internal::wasm):
/// - ArrayIndexImmediateToWriteBarrier(array) — 数组元素写屏障判定
/// - FieldImmediateToWriteBarrier(field) — 结构字段写屏障判定
/// 由 wasm 管线与 wasm-in-JS 内联共用。
/// v8: wasm-assembler-helpers.h (wasm 命名空间)
pub fn array_index_immediate_to_write_barrier(array: &ArrayIndexImmediate) -> WriteBarrierKind { todo!() }
pub fn field_immediate_to_write_barrier(field: &FieldImmediate) -> WriteBarrierKind { todo!() }
`;

const p = writeSrc("v8-turboshaft", "wasm_assembler_helpers.rs.skel", rust);
console.log(`written ${p}`);
console.log(`LOAD宏=${loadMacros.length} 换算函数=2`);
