// 由 tool/src/wasm/interpreter/wasm-interpreter-inl.h.js 从 v8/src/wasm/interpreter/wasm-interpreter-inl.h 生成; 修订就地改, 复跑覆盖
// Drumbrake inline 层: 槽宽规则与 wrapper 打包数组对齐规则

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
  Ref = 5,
}

/// 栈槽宽 (原版 kSlotSize = sizeof(int32_t))
pub const K_SLOT_SIZE: usize = 4;

/// 逐类别槽宽 (原版 GetValueSizeInSlots, 单位: 槽)
/// v8: src/wasm/interpreter/wasm-interpreter-inl.h GetValueSizeInSlots
#[must_use]
pub const fn value_size_in_slots(kind: SlotValueKind) -> u32 {
  match kind {
    SlotValueKind::I32 => 1,  // kI32: 4B / 4B
    SlotValueKind::I64 => 2,  // kI64: 8B / 4B
    SlotValueKind::F32 => 1,  // kF32: 4B / 4B
    SlotValueKind::F64 => 2,  // kF64: 8B / 4B
    SlotValueKind::S128 => 4, // kS128: 16B / 4B
    SlotValueKind::Ref => 2,  // kRef: 8B / 4B
  }
}

/// Ref 槽 64 位对齐掩码 (原版 args_size & 0x04)
pub const K_REF_ALIGN_MASK: u32 = 0x04;

/// JS↔Wasm wrapper 打包数组尺寸 (原版 JSToWasmWrapperPackedArraySize)
/// 规则: i32/f32 计 4B; i64/f64 计 8B; s128 计 16B;
/// ref 计 8B 且写入前对齐到 8B; 最终取 args/rets 较大者后再对齐到 8B。
/// v8: src/wasm/interpreter/wasm-interpreter-inl.h JSToWasmWrapperPackedArraySize
#[must_use]
pub fn js_to_wasm_wrapper_packed_array_size(
  arg_sizes: impl Iterator<Item = SlotValueKind>,
  ret_sizes: impl Iterator<Item = SlotValueKind>,
) -> u32 {
  fn one_side(sizes: impl Iterator<Item = SlotValueKind>) -> u32 {
    let mut size: u32 = 0;
    for kind in sizes {
      size += match kind {
        SlotValueKind::I32 | SlotValueKind::F32 => 4,
        SlotValueKind::I64 | SlotValueKind::F64 => 8,
        SlotValueKind::S128 => 16,
        // Ref 槽先补齐到 8 字节边界再写入 8B
        SlotValueKind::Ref => {
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
  "GetCode",
  "GetFunctionBytecode",
  "AddFunction",
  "GetIsolate",
  "StartActivation",
  "FinishActivation",
  "GetCurrentActivationFor",
  "BeginExecution",
  "BeginExecution",
  "GetReturnValue",
  "GetInterpretedStack",
  "GetFunctionIndex",
  "SetTrapFunctionIndex",
  "return_type",
  "arg_type",
  "local_type",
  "GetValueSizeInSlots",
  "ResetHandleScope",
  "ArgsSizeInSlots",
  "RetsSizeInSlots",
  "RefArgsCount",
  "RefRetsCount",
  "ContainsSimd",
  "HasRefOrSimdArgs",
  "JSToWasmWrapperPackedArraySize",
  "RefLocalsCount",
  "LocalsSizeInSlots",
  "InitializeSlots",
  "ToRegisterIsAllowed",
  "I32Push",
  "I64Push",
  "MemIndexPush",
  "F32Push",
  "F64Push",
  "S128Push",
  "RefPush",
  "Push",
  "PushCopySlot",
  "PushConstSlot",
  "HasVoidSignature",
  "ParamsCount",
  "GetParamType",
  "ReturnsCount",
  "GetReturnType",
  "GetGlobalType",
  "IsMemory64",
  "IsMultiMemory",
  "EmitGlobalIndex",
  "GetCurrentBranchDepth",
  "GetTargetBranch",
  "EmitBranchOffset",
  "EmitBranchTableOffset",
  "EmitIfElseBranchOffset",
  "EmitTryCatchBranchOffset",
  "BeginElseBlock",
  "GetFunctionSignature",
  "GetTopStackType",
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(INLINE_FNS.len() == 57);
  assert!(value_size_in_slots(SlotValueKind::I32) == 1);
  assert!(value_size_in_slots(SlotValueKind::I64) == 2);
  assert!(value_size_in_slots(SlotValueKind::S128) == 4);
  assert!(value_size_in_slots(SlotValueKind::Ref) == 2);
};
