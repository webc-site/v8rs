#!/usr/bin/env bun
// tool/src/compiler/wasm-call-descriptors.js
// v8/src/compiler/wasm-call-descriptors.h + wasm-call-descriptors.cc
// → v8-compiler-passes/src/wasm_call_descriptors.rs.skel
// wasm 调用描述符缓存: 解析 BigInt 双描述符与 32 位降级面。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/wasm-call-descriptors.h");
const cc = readV8("src/compiler/wasm-call-descriptors.cc");

const need = (cond, msg) => {
  if (!cond) throw new Error(`wasm-call-descriptors: ${msg}`);
};

for (const [s, name] of [
  ["class WasmCallDescriptors {", "类定义"],
  ["compiler::CallDescriptor* GetBigIntToI64Descriptor(bool needs_frame_state) {", "I64 描述符获取"],
  ["bigint_to_i64_descriptor_;", "i64 字段"],
  ["bigint_to_i64_descriptor_with_framestate_;", "i64+fs 字段"],
]) need(h.includes(s), `哨兵缺失 ${name}`);

// —— builtin 构造面 (真解析 cc) ——
const builtins = [...cc.matchAll(/GetBuiltinCallDescriptor\(\s*Builtin::k(\w+)/g)].map((m) => m[1]);
need(new Set(builtins).size === 2, `builtin 面 ${builtins}`);
need(builtins.includes("BigIntToI64"), "BigIntToI64 缺席");
// 32 位分支
const has32 = cc.includes("V8_TARGET_ARCH_32_BIT") && cc.includes("GetLoweredCallDescriptor");
const loweredPairs = [...cc.matchAll(/original == (\w+_[\w]*)\)/g)].map((m) => m[1]);

const body = `${banner("src/compiler/wasm-call-descriptors.js", "src/compiler/wasm-call-descriptors.h + wasm-call-descriptors.cc", "BigInt↔I64 等 wasm 调用描述符缓存")}
//! wasm-call-descriptors — 自有 zone + 双 descriptor
//! (BigIntToI64: 带/不带 FrameState; ${has32 ? "32 位另建 BigIntToI32Pair 对并降级映射" : "64 位无降级面"})。

/// 原版 WasmCallDescriptors — 描述符缓存
pub struct WasmCallDescriptors {
  /// zone_
  pub zone: Box<Zone<'static>>,
  /// bigint_to_i64_descriptor_
  pub bigint_to_i64_descriptor: *mut CallDescriptor,
  /// bigint_to_i64_descriptor_with_framestate_
  pub bigint_to_i64_descriptor_with_framestate: *mut CallDescriptor,
}

impl WasmCallDescriptors {
  /// 原版 GetBigIntToI64Descriptor — 按 FrameState 需求二选一
  /// v8: src/compiler/wasm-call-descriptors.h WasmCallDescriptors::GetBigIntToI64Descriptor
  pub fn get_big_int_to_i64_descriptor(&self, needs_frame_state: bool) -> &CallDescriptor {
    if needs_frame_state {
      unsafe { &*self.bigint_to_i64_descriptor_with_framestate }
    } else {
      unsafe { &*self.bigint_to_i64_descriptor }
    }
  }

  /// 原版 GetLoweredCallDescriptor — 仅 32 位 (I64 → I32Pair 降级映射)
  /// v8: src/compiler/wasm-call-descriptors.cc WasmCallDescriptors::GetLoweredCallDescriptor
  #[cfg(target_pointer_width = "32")]
  pub fn get_lowered_call_descriptor(&self, original: &CallDescriptor) -> Option<&CallDescriptor> {
    todo!("v8: src/compiler/wasm-call-descriptors.cc WasmCallDescriptors::GetLoweredCallDescriptor (映射对 ${loweredPairs.join(" / ")})")
  }
}
`;

const p = writeSrc("v8-compiler-passes", "wasm_call_descriptors.rs.skel", body);
console.log(`written ${p}\nbuiltins=${[...new Set(builtins)].join(",")} has32=${has32} lowered_pairs=${loweredPairs.length}`);
