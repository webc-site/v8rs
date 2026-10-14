// 由 tool/src/deoptimizer/frame-translation-builder.h.js 从 v8/src/deoptimizer/{translated-state,frame-translation-builder,deoptimized-frame-info,materialized-object-store}.{h,cc} + 9 架构 deoptimizer-*.cc 生成; 修订就地改, 复跑覆盖
// 反优化状态族与架构分派骨架

/// TranslatedState 方法面 (原版 translated-state.h, 58 项, 顺序登记)
pub const TRANSLATED_STATE_METHODS: [&str; 58] = [
  "GetRawValue",
  "GetSmiValue",
  "GetValue",
  "IsMaterializedObject",
  "IsMaterializableByDebugger",
  "Handlify",
  "GetChildrenCount",
  "set_storage",
  "set_initialized_storage",
  "mark_finished",
  "mark_allocated",
  "storage",
  "ReplaceElementsArrayWithCopy",
  "raw_literal",
  "int32_value",
  "int64_value",
  "object_length",
  "object_index",
  "string_concat_index",
  "GetValueCount",
  "shared_info",
  "bytecode_array",
  "return_value_offset",
  "return_value_count",
  "formal_parameter_count",
  "raw_shared_info",
  "raw_bytecode_array",
  "wasm_function_index",
  "Add",
  "Handlify",
  "get_on_heap_literals",
  "Prepare",
  "StoreMaterializedValuesAndDeopt",
  "Init",
  "VerifyMaterializedObjects",
  "DoUpdateFeedback",
  "CreateNextTranslatedValue",
  "CreateArgumentsElementsTranslatedValues",
  "UpdateFromPreviouslyMaterializedObjects",
  "MaterializeFixedDoubleArray",
  "MaterializeHeapNumber",
  "MaterializeUninitializedHeapNumber",
  "EnsureObjectAllocatedAt",
  "SkipSlots",
  "AllocateStorageFor",
  "EnsureJSObjectAllocated",
  "EnsurePropertiesAllocatedAndMarked",
  "EnsureChildrenAllocated",
  "EnsureCapturedObjectAllocatedAt",
  "InitializeObjectAt",
  "InitializeCapturedObjectAt",
  "InitializeJSObjectAt",
  "InitializeFirstHeaderField",
  "InitializeObjectWithTaggedFieldsAt",
  "PrepareObjectForLayoutChange",
  "ResolveStringConcat",
  "ReadUpdateFeedback",
  "GetValueAndAdvance",
];

/// FrameTranslationBuilder 写入面 (原版 frame-translation-builder.h, 52 项)
pub const TRANSLATION_BUILDER_METHODS: [&str; 52] = [
  "BeginInterpretedFrame",
  "BeginInlinedExtraArguments",
  "BeginConstructCreateStubFrame",
  "BeginConstructInvokeStubFrame",
  "BeginBuiltinContinuationFrame",
  "BeginJSToWasmBuiltinContinuationFrame",
  "BeginWasmInlinedIntoJSFrame",
  "BeginLiftoffFrame",
  "BeginJavaScriptBuiltinContinuationFrame",
  "BeginJavaScriptBuiltinContinuationWithCatchFrame",
  "ArgumentsElements",
  "ArgumentsLength",
  "RestLength",
  "BeginCapturedObject",
  "AddUpdateFeedback",
  "DuplicateObject",
  "StringConcat",
  "StoreRegister",
  "StoreRegister",
  "StoreInt32Register",
  "StoreInt64Register",
  "StoreIntPtrRegister",
  "StoreSignedBigInt64Register",
  "StoreUnsignedBigInt64Register",
  "StoreUint32Register",
  "StoreBoolRegister",
  "StoreFloatRegister",
  "StoreDoubleRegister",
  "StoreHoleyDoubleRegister",
  "StoreSimd128Register",
  "StoreStackSlot",
  "StoreInt32StackSlot",
  "StoreInt64StackSlot",
  "StoreIntPtrStackSlot",
  "StoreSignedBigInt64StackSlot",
  "StoreUnsignedBigInt64StackSlot",
  "StoreUint32StackSlot",
  "StoreBoolStackSlot",
  "StoreFloatStackSlot",
  "StoreDoubleStackSlot",
  "StoreSimd128StackSlot",
  "StoreHoleyDoubleStackSlot",
  "StoreLiteral",
  "StoreDoubleLiteral",
  "StoreHoleyDoubleLiteral",
  "StoreOptimizedOut",
  "StoreJSFrameFunction",
  "Add",
  "AddRawToContents",
  "AddRawBegin",
  "FinishPendingInstructionIfNeeded",
  "ValidateBytes",
];

/// VLQ 编码锚点 (builder 以 VLQ 写操作数)
pub const VLQ_ANCHOR: &str = "WriteVLQ";

/// 结构锚点
pub const STRUCT_ANCHORS: [&str; 2] = ["DeoptimizedFrameInfo", "MaterializedObjectStore"];

/// 架构分派登记 (原版 deoptimizer/<arch>/deoptimizer-<arch>.cc)
/// x64+arm64 为当前目标; 其余架构跳过并在此留档
pub mod arch_dispatch {
  /// 当前目标 (x64 + arm64)
  pub const TARGET_ARCHES: [&str; 2] = ["x64", "arm64"];
  /// 登记跳过架构 (实现存在但本仓库不编译)
  pub const SKIPPED_ARCHES: [&str; 7] =
    ["arm", "ia32", "loong64", "mips64", "ppc", "riscv", "s390"];

  /// 各架构定义头函数数 (对数基准)
  pub const FN_COUNTS: [(&str, usize); 9] = [
    ("x64", 0),
    ("arm64", 0),
    ("arm", 0),
    ("ia32", 0),
    ("loong64", 0),
    ("mips64", 0),
    ("ppc", 0),
    ("riscv", 0),
    ("s390", 0),
  ];

  /// x64 退出尺寸常量 (原版 deoptimizer-x64.cc; 非沙箱/沙箱双值取非沙箱,
  /// 平台分支按当前目标展开)
  pub mod x64_consts {
    /// 原版 kEagerDeoptExitSize
    pub const EAGER_DEOPT_EXIT_SIZE: i32 = 4;
    /// 原版 kLazyDeoptExitSize
    pub const LAZY_DEOPT_EXIT_SIZE: i32 = 8;
    /// 原版 kLazyDeoptExitSize_Alt2
    pub const LAZY_DEOPT_EXIT_SIZE_ALT2: i32 = 4;
    /// 原版 kAdaptShadowStackOffsetToSubtract
    pub const ADAPT_SHADOW_STACK_OFFSET_TO_SUBTRACT: i32 = 7;
    /// 原版 kAdaptShadowStackOffsetToSubtract_Alt2
    pub const ADAPT_SHADOW_STACK_OFFSET_TO_SUBTRACT_ALT2: i32 = 0;
  }

  /// arm64 退出尺寸常量
  pub mod arm64_consts {
    /// 原版 kInstrSize (arm64 定长指令 4 字节, constants-arm64.h)
    pub const INSTR_SIZE: i32 = 4;
    /// 原版 kEagerDeoptExitSize = kInstrSize (deoptimizer-arm64.cc:15)
    pub const EAGER_DEOPT_EXIT_SIZE: i32 = 4;
    /// 原版 kLazyDeoptExitSize = 2*kInstrSize (V8_ENABLE_CONTROL_FLOW_INTEGRITY 分支, cc:17)
    pub const LAZY_DEOPT_EXIT_SIZE_CFI: i32 = 8;
    /// 原版 kLazyDeoptExitSize = 1*kInstrSize (目标配置 CFI 关, cc:19)
    pub const LAZY_DEOPT_EXIT_SIZE: i32 = 4;
    /// 原版 kAdaptShadowStackOffsetToSubtract
    pub const ADAPT_SHADOW_STACK_OFFSET_TO_SUBTRACT: i32 = 0;
  }
}

/// 签名骨架 (语义见 Developer 修订)
pub mod translated_state_skel {
  /// 读取翻译字节流并物化帧 (原版 TranslatedState::Init / UpdateFromPreviouslyMaterializedObjects)
  #[inline]
  pub fn init() {}
  /// 物化对象暂存 (原版 MaterializedObjectStore: 栈上物化对象跨 GC 存取)
  #[inline]
  pub fn store_materialized_objects() {}
  /// 帧信息快照 (原版 DeoptimizedFrameInfo: 调试器取反优化帧局部)
  #[inline]
  pub fn deoptimized_frame_info_skel() {}
}
