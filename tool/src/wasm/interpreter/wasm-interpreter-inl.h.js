#!/usr/bin/env bun
// tool/src/wasm/interpreter/wasm-interpreter-inl.h.js
// 机械提取 v8/src/wasm/interpreter/wasm-interpreter-inl.h 的
// inline 函数清单与槽宽规则 (GetValueSizeInSlots / JSToWasmWrapperPackedArraySize 对齐规则)

import { readV8, writeSrc, banner } from "../../../lib.js";

const hSrc = readV8("src/wasm/interpreter/wasm-interpreter-inl.h");

function must(re, what) {
  const m = hSrc.match(re);
  if (!m) throw new Error(`未找到 ${what}`);
  return m;
}

// 1. inline 函数清单 (签名面, 按源顺序)
const fnNames = [];
{
  const re = /\binline\s+[\w:<>,&*\s]+?\b([A-Za-z_]\w*)\s*\(/gs;
  for (const m of hSrc.matchAll(re)) fnNames.push(m[1]);
}
const EXPECTED_FNS = [
  "GetCode", "GetFunctionBytecode", "AddFunction", "GetIsolate",
  "StartActivation", "FinishActivation", "GetCurrentActivationFor",
  "BeginExecution", "BeginExecution", "GetReturnValue", "GetInterpretedStack",
  "GetFunctionIndex", "SetTrapFunctionIndex", "return_type", "arg_type",
  "local_type", "GetValueSizeInSlots", "ResetHandleScope", "ArgsSizeInSlots",
  "RetsSizeInSlots", "RefArgsCount", "RefRetsCount", "ContainsSimd",
  "HasRefOrSimdArgs", "JSToWasmWrapperPackedArraySize", "RefLocalsCount",
  "LocalsSizeInSlots", "InitializeSlots", "ToRegisterIsAllowed", "I32Push",
  "I64Push", "MemIndexPush", "F32Push", "F64Push", "S128Push", "RefPush",
  "Push", "PushCopySlot", "PushConstSlot", "HasVoidSignature", "ParamsCount",
  "GetParamType", "ReturnsCount", "GetReturnType", "GetGlobalType",
  "IsMemory64", "IsMultiMemory", "EmitGlobalIndex", "GetCurrentBranchDepth",
  "GetTargetBranch", "EmitBranchOffset", "EmitBranchTableOffset",
  "EmitIfElseBranchOffset", "EmitTryCatchBranchOffset", "BeginElseBlock",
  "GetFunctionSignature", "GetTopStackType",
];
if (fnNames.length !== EXPECTED_FNS.length || fnNames.join(",") !== EXPECTED_FNS.join(",")) {
  throw new Error(`inline 函数清单不符:\n实际 ${fnNames.length}: ${fnNames.join(",")}\n期望 ${EXPECTED_FNS.length}`);
}

// 2. GetValueSizeInSlots 槽宽规则 (sizeof(x)/kSlotSize, kSlotSize=4)
const K_SLOT_SIZE = 4;
const slotSizes = [
  ["I32", 4], ["I64", 8], ["F32", 4], ["F64", 8], ["S128", 16], ["Ref", 8],
];
must(/case kI32:\s*return sizeof\(int32_t\) \/ kSlotSize;/, "kI32 槽宽");
must(/case kI64:\s*return sizeof\(int64_t\) \/ kSlotSize;/, "kI64 槽宽");
must(/case kF32:\s*return sizeof\(float\) \/ kSlotSize;/, "kF32 槽宽");
must(/case kF64:\s*return sizeof\(double\) \/ kSlotSize;/, "kF64 槽宽");
must(/case kS128:\s*return sizeof\(Simd128\) \/ kSlotSize;/, "kS128 槽宽");
must(/case kRef:\s*case kRefNull:\s*return sizeof\(WasmRef\) \/ kSlotSize;/, "kRef 槽宽");
// sizeof(Simd128)=16, sizeof(WasmRef)=8 (指针宽引用)
must(/static_assert\(kSystemPointerSize == 8\);/, "指针宽 8 锚 (Ref 对齐前提)");

// 3. JSToWasmWrapperPackedArraySize 对齐规则
// Ref 槽 64 位对齐: args_size += (args_size & 0x04); 总长再对齐一次
for (const v of ["args_size", "rets_size"]) {
  must(new RegExp(`${v} \\+= \\(${v} & 0x04\\);`), `${v} Ref 对齐`);
}
must(/size = std::max\(args_size, rets_size\);/, "args/rets 取 max");
must(/size = size \+ \(size & 0x04\)|size \+= \(size & 0x04\);/, "总长 64 位对齐");

console.log(`wasm-interpreter-inl.h: inline 函数=${fnNames.length} 槽宽映射=${slotSizes.length}`);

// 4. 生成 Rust
let rust = banner(
  "src/wasm/interpreter/wasm-interpreter-inl.h.js",
  "src/wasm/interpreter/wasm-interpreter-inl.h",
  "Drumbrake inline 层: 槽宽规则与 wrapper 打包数组对齐规则",
);

rust += `
//! 内联薄层规则; 其余 inline 函数为转发/工具, 语义面由 Developer 就地修订。

/// 槽宽计算用的值类别 (本模块局部, 避免上游类型依赖)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SlotValueKind {
  /// kI32
  I32 = 0,
  /// kI64
  I64 = 1,
  /// kF32
  F32 = 2,
  /// kF64
  F64 = 3,
  /// kS128
  S128 = 4,
  /// kRef / kRefNull
  REF = 5,
}

/// 栈槽宽 (原版 kSlotSize = sizeof(int32_t))
pub const K_SLOT_SIZE: usize = ${K_SLOT_SIZE};

/// 逐类别槽宽 (原版 GetValueSizeInSlots, 单位: 槽)
/// v8: src/wasm/interpreter/wasm-interpreter-inl.h GetValueSizeInSlots
#[must_use]
pub const fn value_size_in_slots(kind: SlotValueKind) -> u32 {
  match kind {
`;

const kindMap = { I32: "I32", I64: "I64", F32: "F32", F64: "F64", S128: "S128", Ref: "REF" };
for (const [k, bytes] of slotSizes) {
  rust += `    SlotValueKind::${kindMap[k]} => ${bytes / K_SLOT_SIZE}, // k${k}: ${bytes}B / ${K_SLOT_SIZE}B\n`;
}
rust += `  }
}

/// Ref 槽 64 位对齐掩码 (原版 args_size & 0x04)
pub const K_REF_ALIGN_MASK: u32 = 0x04;

/// JS↔Wasm wrapper 打包数组尺寸 (原版 JSToWasmWrapperPackedArraySize)
/// 规则: i32/f32 计 4B; i64/f64 计 8B; s128 计 16B;
/// ref 计 8B 且写入前对齐到 8B; 最终取 args/rets 较大者后再对齐到 8B。
/// v8: src/wasm/interpreter/wasm-interpreter-inl.h JSToWasmWrapperPackedArraySize
#[must_use]
pub fn js_to_wasm_wrapper_packed_array_size(
  arg_sizes: impl ExactSizeIterator<Item = (SlotValueKind)>,
  ret_sizes: impl ExactSizeIterator<Item = (SlotValueKind)>,
) -> u32 {
  fn one_side(sizes: impl Iterator<Item = SlotValueKind>) -> u32 {
    let mut size: u32 = 0;
    for kind in sizes {
      size += match kind {
        SlotValueKind::I32 | SlotValueKind::F32 => 4,
        SlotValueKind::I64 | SlotValueKind::F64 => 8,
        SlotValueKind::S128 => 16,
        // Ref 槽先补齐到 8 字节边界再写入 8B
        SlotValueKind::REF => {
          size += size & K_REF_ALIGN_MASK;
          8
        }
      };
    }
    size
  }
  let size = one_side(arg_sizes).max(one_side(ret_sizes));
  // 总长对齐到 8 字节
  size + (size & K_REF_ALIGN_MASK)
}

/// inline 函数清单 (签名面对账; 重载按源序展开)
/// v8: src/wasm/interpreter/wasm-interpreter-inl.h (全部 inline 定义)
pub const INLINE_FNS: &[&str] = &[
`;

for (const n of fnNames) rust += `  "${n}",\n`;
rust += `];

// —— 提取不变式断言 ——
const _: () = {
  assert!(INLINE_FNS.len() == ${fnNames.length});
  assert!(value_size_in_slots(SlotValueKind::I32) == 1);
  assert!(value_size_in_slots(SlotValueKind::I64) == 2);
  assert!(value_size_in_slots(SlotValueKind::S128) == 4);
  assert!(value_size_in_slots(SlotValueKind::REF) == 2);
  // [i32, ref]: 4 + (4&4)=4 补齐 + 8 = 16
  assert!(js_to_wasm_wrapper_packed_array_size(
    [SlotValueKind::I32, SlotValueKind::REF].into_iter(), [].into_iter()) == 16);
  // [i32] vs [f64]: max(4,8)=8, 已对齐
  assert!(js_to_wasm_wrapper_packed_array_size(
    [SlotValueKind::I32].into_iter(), [SlotValueKind::F64].into_iter()) == 8);
};
`;

const p = writeSrc("v8-wasm-interp", "wasm_interpreter_inl.rs", rust);
console.log(`written ${p}`);
