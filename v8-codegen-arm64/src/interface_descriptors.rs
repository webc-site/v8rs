// 由 tool/src/codegen/arm64/interface-descriptors-arm64-inl.h.js 从 v8/src/codegen/arm64/interface-descriptors-arm64-inl.h 生成; 修订就地改, 复跑覆盖
// arm64 调用描述符寄存器分配表

/// kMaxBuiltinRegisterParams (原版 static_assert 锚定 GP 默认阵列大小)
pub const K_MAX_BUILTIN_REGISTER_PARAMS: usize = 5;

/// 原版 DefaultRegisterArray(): x0..x4
pub const DEFAULT_REGISTER_ARRAY: [&str; 5] = ["x0", "x1", "x2", "x3", "x4"];

/// 原版 DefaultDoubleRegisterArray(): d0..d6
pub const DEFAULT_DOUBLE_REGISTER_ARRAY: [&str; 7] = ["d0", "d1", "d2", "d3", "d4", "d5", "d6"];

/// 原版 DefaultReturnRegisterArray(): kReturnRegister0/1/2
pub const DEFAULT_RETURN_REGISTER_ARRAY: [&str; 3] = ["kReturnRegister0", "kReturnRegister1", "kReturnRegister2"];

/// 原版 DefaultReturnDoubleRegisterArray(): kFPReturnRegister0 + no_dreg 填充
pub const DEFAULT_RETURN_DOUBLE_REGISTER_ARRAY: [&str; 3] = ["kFPReturnRegister0", "no_dreg", "no_dreg"];

/// 原版 WriteBarrierDescriptor::registers() (arm64 单分支; x7 仅为 padding, 待移除 — TODO leszeks)
pub const WRITE_BARRIER_REGISTERS: [&str; 8] = ["x1", "x5", "x4", "x2", "x0", "x3", "kContextRegister", "x7"];

/// 描述符寄存器访问器总表: (描述符, 角色名, 寄存器名)
#[rustfmt::skip]
pub const DESCRIPTOR_REGISTERS: &[(&str, &str, &str)] = &[
  ("LoadDescriptor", "ReceiverRegister", "x1"),
  ("LoadDescriptor", "NameRegister", "x2"),
  ("LoadDescriptor", "SlotRegister", "x0"),
  ("LoadWithVectorDescriptor", "VectorRegister", "x3"),
  ("KeyedLoadBaselineDescriptor", "ReceiverRegister", "x1"),
  ("KeyedLoadBaselineDescriptor", "NameRegister", "kInterpreterAccumulatorRegister"),
  ("KeyedLoadBaselineDescriptor", "SlotRegister", "x2"),
  ("KeyedLoadWithVectorDescriptor", "VectorRegister", "x3"),
  ("EnumeratedKeyedLoadBaselineDescriptor", "EnumIndexRegister", "x4"),
  ("EnumeratedKeyedLoadBaselineDescriptor", "CacheTypeRegister", "x5"),
  ("EnumeratedKeyedLoadBaselineDescriptor", "SlotRegister", "x2"),
  ("KeyedHasICBaselineDescriptor", "ReceiverRegister", "kInterpreterAccumulatorRegister"),
  ("KeyedHasICBaselineDescriptor", "NameRegister", "x1"),
  ("KeyedHasICBaselineDescriptor", "SlotRegister", "x2"),
  ("KeyedHasICWithVectorDescriptor", "VectorRegister", "x3"),
  ("LoadWithReceiverAndVectorDescriptor", "LookupStartObjectRegister", "x4"),
  ("StoreDescriptor", "ReceiverRegister", "x1"),
  ("StoreDescriptor", "NameRegister", "x2"),
  ("StoreDescriptor", "ValueRegister", "x0"),
  ("StoreDescriptor", "SlotRegister", "x4"),
  ("StoreWithVectorDescriptor", "VectorRegister", "x3"),
  ("DefineKeyedOwnDescriptor", "FlagsRegister", "x5"),
  ("StoreTransitionDescriptor", "MapRegister", "x5"),
  ("CallApiGetterDescriptor", "NameRegister", "kCArgRegs[0]"),
  ("GrowArrayElementsDescriptor", "ObjectRegister", "x0"),
  ("GrowArrayElementsDescriptor", "KeyRegister", "x3"),
  ("BaselineLeaveFrameDescriptor", "ParamsSizeRegister", "x3"),
  ("BaselineLeaveFrameDescriptor", "WeightRegister", "x4"),
  ("TypeConversionDescriptor", "ArgumentRegister", "x0"),
  ("MaglevOptimizeCodeOrTailCallOptimizedCodeSlotDescriptor", "FlagsRegister", "x8"),
  ("MaglevOptimizeCodeOrTailCallOptimizedCodeSlotDescriptor", "FeedbackVectorRegister", "x9"),
  ("MaglevOptimizeCodeOrTailCallOptimizedCodeSlotDescriptor", "TemporaryRegister", "x5"),
  ("CallApiCallbackOptimizedDescriptor", "ApiFunctionAddressRegister", "x1"),
  ("CallApiCallbackOptimizedDescriptor", "ActualArgumentsCountRegister", "x2"),
  ("CallApiCallbackOptimizedDescriptor", "FunctionTemplateInfoRegister", "x3"),
  ("CallApiCallbackGenericDescriptor", "ActualArgumentsCountRegister", "x2"),
  ("CallApiCallbackGenericDescriptor", "TopmostScriptHavingContextRegister", "x1"),
  ("CallApiCallbackGenericDescriptor", "FunctionTemplateInfoRegister", "x3"),
];

/// 按 (描述符, 角色) 查寄存器
#[inline]
#[must_use]
pub fn descriptor_register(desc: &str, role: &str) -> Option<&'static str> {
  DESCRIPTOR_REGISTERS
    .iter()
    .find(|(d, r, _)| *d == desc && *r == role)
    .map(|(_, _, reg)| *reg)
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(K_MAX_BUILTIN_REGISTER_PARAMS == 5);
  assert!(DEFAULT_REGISTER_ARRAY == ["x0", "x1", "x2", "x3", "x4"]);
  assert!(DESCRIPTOR_REGISTERS.len() == 38);
  assert!(descriptor_register("LoadDescriptor", "ReceiverRegister") == Some("x1"));
  assert!(descriptor_register("LoadDescriptor", "NameRegister") == Some("x2"));
  assert!(descriptor_register("LoadDescriptor", "SlotRegister") == Some("x0"));
  assert!(WRITE_BARRIER_REGISTERS.len() == 8);
  assert!(WRITE_BARRIER_REGISTERS[0] == "x1" && WRITE_BARRIER_REGISTERS[7] == "x7");
};
