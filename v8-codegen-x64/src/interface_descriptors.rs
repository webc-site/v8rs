// 由 tool/src/codegen/x64/interface-descriptors-x64-inl.h.js 从 v8/src/codegen/x64/interface-descriptors-x64-inl.h 生成; 修订就地改, 复跑覆盖
// x64 调用描述符寄存器分配表

/// kMaxBuiltinRegisterParams (原版 static_assert 锚定 GP 默认阵列大小)
pub const K_MAX_BUILTIN_REGISTER_PARAMS: usize = 5;

/// 原版 CallInterfaceDescriptor::DefaultRegisterArray(): rax, rbx, rcx, rdx, rdi
pub const DEFAULT_REGISTER_ARRAY: [&str; 5] = ["rax", "rbx", "rcx", "rdx", "rdi"];

/// 原版 DefaultDoubleRegisterArray(): xmm0..xmm6
pub const DEFAULT_DOUBLE_REGISTER_ARRAY: [&str; 7] = ["xmm0", "xmm1", "xmm2", "xmm3", "xmm4", "xmm5", "xmm6"];

/// 原版 DefaultReturnRegisterArray(): kReturnRegister0/1/2
pub const DEFAULT_RETURN_REGISTER_ARRAY: [&str; 3] = ["kReturnRegister0", "kReturnRegister1", "kReturnRegister2"];

/// 原版 DefaultReturnDoubleRegisterArray(): kFPReturnRegister0 + 2 个 no_dreg 填充
pub const DEFAULT_RETURN_DOUBLE_REGISTER_ARRAY: [&str; 3] = ["kFPReturnRegister0", "no_dreg", "no_dreg"];

/// 原版 WriteBarrierDescriptor::registers() — 非 WIN 分支 (本目标生效): 6 寄存器
pub const WRITE_BARRIER_REGISTERS: [&str; 6] = ["rdi", "rbx", "rdx", "rcx", "rax", "rsi"];
/// 原版 WriteBarrierDescriptor::registers() — V8_TARGET_OS_WIN 分支: 7 寄存器
pub const WRITE_BARRIER_REGISTERS_WIN: [&str; 7] = ["rdi", "r8", "rcx", "rax", "r9", "rdx", "rsi"];

/// 描述符寄存器访问器总表: (描述符, 角色名, 寄存器名)。
/// 寄存器名保持原版字面 (rax/rcx/...; kInterpreterAccumulatorRegister = rax,
/// kCArgRegs[i] = C 调用参数寄存器组)。
#[rustfmt::skip]
pub const DESCRIPTOR_REGISTERS: &[(&str, &str, &str)] = &[
  ("LoadDescriptor", "ReceiverRegister", "rdx"),
  ("LoadDescriptor", "NameRegister", "rcx"),
  ("LoadDescriptor", "SlotRegister", "rax"),
  ("LoadWithVectorDescriptor", "VectorRegister", "rbx"),
  ("KeyedLoadBaselineDescriptor", "ReceiverRegister", "rdx"),
  ("KeyedLoadBaselineDescriptor", "NameRegister", "kInterpreterAccumulatorRegister"),
  ("KeyedLoadBaselineDescriptor", "SlotRegister", "rcx"),
  ("KeyedLoadWithVectorDescriptor", "VectorRegister", "rbx"),
  ("EnumeratedKeyedLoadBaselineDescriptor", "EnumIndexRegister", "rdi"),
  ("EnumeratedKeyedLoadBaselineDescriptor", "CacheTypeRegister", "r8"),
  ("EnumeratedKeyedLoadBaselineDescriptor", "SlotRegister", "rcx"),
  ("KeyedHasICBaselineDescriptor", "ReceiverRegister", "kInterpreterAccumulatorRegister"),
  ("KeyedHasICBaselineDescriptor", "NameRegister", "rdx"),
  ("KeyedHasICBaselineDescriptor", "SlotRegister", "rcx"),
  ("KeyedHasICWithVectorDescriptor", "VectorRegister", "rbx"),
  ("LoadWithReceiverAndVectorDescriptor", "LookupStartObjectRegister", "rdi"),
  ("StoreDescriptor", "ReceiverRegister", "rdx"),
  ("StoreDescriptor", "NameRegister", "rcx"),
  ("StoreDescriptor", "ValueRegister", "rax"),
  ("StoreDescriptor", "SlotRegister", "rdi"),
  ("StoreWithVectorDescriptor", "VectorRegister", "rbx"),
  ("DefineKeyedOwnDescriptor", "FlagsRegister", "r11"),
  ("StoreTransitionDescriptor", "MapRegister", "r11"),
  ("CallApiGetterDescriptor", "NameRegister", "kCArgRegs[0]"),
  ("CallApiGetterDescriptor", "CallbackRegister", "rbx"),
  ("GrowArrayElementsDescriptor", "ObjectRegister", "rax"),
  ("GrowArrayElementsDescriptor", "KeyRegister", "rbx"),
  ("BaselineLeaveFrameDescriptor", "ParamsSizeRegister", "rbx"),
  ("BaselineLeaveFrameDescriptor", "WeightRegister", "rcx"),
  ("MaglevOptimizeCodeOrTailCallOptimizedCodeSlotDescriptor", "FlagsRegister", "r8"),
  ("MaglevOptimizeCodeOrTailCallOptimizedCodeSlotDescriptor", "FeedbackVectorRegister", "r9"),
  ("MaglevOptimizeCodeOrTailCallOptimizedCodeSlotDescriptor", "TemporaryRegister", "r11"),
  ("TypeConversionDescriptor", "ArgumentRegister", "rax"),
  ("CallApiCallbackOptimizedDescriptor", "ApiFunctionAddressRegister", "rdx"),
  ("CallApiCallbackOptimizedDescriptor", "ActualArgumentsCountRegister", "rcx"),
  ("CallApiCallbackOptimizedDescriptor", "FunctionTemplateInfoRegister", "rbx"),
  ("CallApiCallbackGenericDescriptor", "ActualArgumentsCountRegister", "rcx"),
  ("CallApiCallbackGenericDescriptor", "FunctionTemplateInfoRegister", "rbx"),
  ("CallApiCallbackGenericDescriptor", "TopmostScriptHavingContextRegister", "rdx"),
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

// TSAN 分支 (V8_IS_TSAN): TSANStoreDescriptor / TSANLoadDescriptor::registers()
// 本目标不启用 TSAN — 登记: 未提取 (2 个描述符)。

// —— 提取不变式断言 ——
const _: () = {
  assert!(K_MAX_BUILTIN_REGISTER_PARAMS == 5);
  assert!(DEFAULT_REGISTER_ARRAY == ["rax", "rbx", "rcx", "rdx", "rdi"]);
  assert!(DEFAULT_DOUBLE_REGISTER_ARRAY.len() == 7);
  assert!(DESCRIPTOR_REGISTERS.len() == 39);
  assert!(descriptor_register("LoadDescriptor", "ReceiverRegister") == Some("rdx"));
  assert!(descriptor_register("LoadDescriptor", "NameRegister") == Some("rcx"));
  assert!(descriptor_register("LoadDescriptor", "SlotRegister") == Some("rax"));
  assert!(WRITE_BARRIER_REGISTERS.len() == 6);
  assert!(WRITE_BARRIER_REGISTERS_WIN.len() == 7);
};
