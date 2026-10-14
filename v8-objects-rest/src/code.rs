// 由 tool/src/objects/code.h.js 从 v8/src/objects/code.{h,cc,-inl.h} 生成; 修订就地改, 复跑覆盖
// 基准: 非压缩/沙箱关/恒常池关/跳转表关/gearbox 关

//! Code 对象族 (原版 code.{h,cc,-inl.h})
//!
//! Code 是 InstructionStream 的元数据容器 (InstructionStream 在写保护页上,
//! 头部不可变); 内建 Code 无 InstructionStream, 指令与元数据在离堆嵌入区.
//!
//! 推迟登记 (见 readme/zh.md 台账, Isolate/嵌入 blob/codegen 域就位即解锁):
//! - `metadata_start` 嵌入内建支 (EmbeddedData::FromBlob, 归 v8-snapshot);
//! - `source_position`/`source_statement_position` (SourcePositionTableIterator, 归 v8-codegen-core);
//! - `is_isolate_independent` / `clear_embedded_objects_and_js_dispatch_handles`
//!   (RelocIterator/WritableJitAllocation, 归 v8-codegen-core + heap);
//! - `sandboxing_mode` 内建支 (Builtins::SandboxingModeOf, 归 v8-init/builtins)。

use std::{
  mem::{offset_of, size_of},
  ops::Deref,
  ptr,
  sync::atomic::{AtomicU32, AtomicUsize, Ordering},
};

use v8_objects_core::{
  heap_object::HeapObject,
  instance_type_checker as itc,
  map::Map,
  objects::instance_type_of,
  smi::Smi,
  tagged::{Object, Tagged},
  visitors::{FullObjectSlot, Root, RootVisitor},
};

use crate::{
  code_kind::CodeKind,
  deoptimization_data_inl::{
    DeoptimizationData, DeoptimizationDataExt, DeoptimizationLiteralArrayExt,
  },
  error::{Error, Result},
  instruction_stream::{self, InstructionStream, InstructionStreamExt},
};

// ===== 布局 (CODE_DATA_FIELDS × DEFINE_FIELD_OFFSET_CONSTANTS, 23 项) =====
/// Code 字段偏移表 (基准配置求值; Size 列为字节, PAD 项 = OBJECT_POINTER_PADDING)
pub const CODE_FIELDS: [(&str, usize, usize, &str); 23] = [
  // (字段, 偏移, 尺寸, 原版尺寸表达式)
  (
    "kDeoptimizationDataOrInterpreterDataOffset",
    8,
    8,
    "kTaggedSize",
  ),
  ("kPositionTableOffset", 16, 8, "kTaggedSize"),
  ("kStartOfStrongFieldsOffset", 24, 0, "0"),
  ("kWrapperOffset", 24, 8, "kTaggedSize"),
  ("kEndOfStrongFieldsWithMainCageBaseOffset", 32, 0, "0"),
  ("kInstructionStreamOffset", 32, 8, "kTaggedSize"),
  ("kEndOfStrongFieldsOffset", 40, 0, "0"),
  ("kInstructionStartOffset", 40, 8, "kSystemPointerSize"),
  ("kDispatchHandleOffset", 48, 4, "kJSDispatchHandleSize"),
  ("kFlagsOffset", 52, 4, "kUInt32Size"),
  ("kInstructionSizeOffset", 56, 4, "kIntSize"),
  ("kMetadataSizeOffset", 60, 4, "kIntSize"),
  ("kInlinedBytecodeSizeOffset", 64, 4, "kIntSize"),
  ("kOsrOffsetOffset", 68, 4, "kInt32Size"),
  ("kHandlerTableOffsetOffset", 72, 4, "kIntSize"),
  ("kUnwindingInfoOffsetOffset", 76, 4, "kInt32Size"),
  (
    "kConstantPoolOffsetOffset",
    80,
    0,
    "V8_EMBEDDED_CONSTANT_POOL_BOOL ? kIntSize : 0",
  ),
  ("kCodeCommentsOffsetOffset", 80, 4, "kIntSize"),
  (
    "kJumpTableInfoOffsetOffset",
    84,
    0,
    "V8_JUMP_TABLE_INFO_BOOL ? kInt32Size : 0",
  ),
  ("kParameterCountOffset", 84, 2, "kUInt16Size"),
  ("kBuiltinIdOffset", 86, 2, "kInt16Size"),
  (
    "kUnalignedSize",
    88,
    0,
    "OBJECT_POINTER_PADDING(kUnalignedSize)",
  ),
  ("kSize", 88, 0, "0"),
];

/// 对象总大小 (基准配置 kSize)
pub const K_SIZE: usize = 88;

/// 尾部对齐前大小 (kUnalignedSize; 基准下恰已对齐, PAD=0)
pub const K_UNALIGNED_SIZE: usize = 88;

// ===== 标志位 (FLAGS_BIT_FIELDS × DEFINE_BIT_FIELDS, 7 段共 10 位) =====
/// flags 位域布局 (LSB 起)
pub const CODE_FLAG_BITS: [(&str, &str, u32, u32); 7] = [
  // (段名, 值类型, shift, width)
  ("KindField", "CodeKind", 0, 4),
  ("IsDisabledBuiltinField", "bool", 4, 1),
  ("IsTurbofannedField", "bool", 5, 1),
  ("IsContextSpecializedField", "bool", 6, 1),
  ("MarkedForDeoptimizationField", "bool", 7, 1),
  ("EmbeddedObjectsClearedField", "bool", 8, 1),
  ("CanHaveWeakObjectsField", "bool", 9, 1),
];

/// 标记反优化位 (生成代码直访, 原版 kMarkedForDeoptimizationBit = MarkedForDeoptimizationField::kShift)
pub const K_MARKED_FOR_DEOPTIMIZATION_BIT: u32 = 7;

/// TurboFan 位 (原版 kIsTurbofannedBit)
pub const K_IS_TURBOFANNED_BIT: u32 = 5;

/// 参数计数字宽 (原版 kArgumentsBits)
pub const K_ARGUMENTS_BITS: u32 = 16;

/// 最大参数数 (原版 kMaxArguments = 2^16 - 10, 预留隐式实参余量)
pub const K_MAX_ARGUMENTS: u32 = 65526;

/// 无内建哨兵 (原版 Builtin::kNoBuiltinId = -1, int16 存储靠符号扩展保持)
pub const K_NO_BUILTIN_ID: i16 = -1;

/// KindField::kShift/kWidth (code-inl.h DEFINE_BIT_FIELDS 展开)
const KIND_SHIFT: u32 = 0;
const KIND_MASK: u32 = 0xf << KIND_SHIFT;
/// IsDisabledBuiltinField::kShift
const IS_DISABLED_BUILTIN_SHIFT: u32 = 4;
/// IsTurbofannedField::kShift
const IS_TURBOFANNED_SHIFT: u32 = 5;
/// IsContextSpecializedField::kShift
const IS_CONTEXT_SPECIALIZED_SHIFT: u32 = 6;
/// MarkedForDeoptimizationField::kShift
const MARKED_FOR_DEOPT_SHIFT: u32 = 7;
/// EmbeddedObjectsClearedField::kShift
const EMBEDDED_OBJECTS_CLEARED_SHIFT: u32 = 8;
/// CanHaveWeakObjectsField::kShift
const CAN_HAVE_WEAK_OBJECTS_SHIFT: u32 = 9;

// ===== GcSafeCode (GC 中安全访问面, 原样转发 21 法) =====
/// GcSafeCode 转发清单 (GCSAFE_CODE_FWD_ACCESSOR X-macro 展开)
pub const GCSAFE_CODE_FORWARDERS: [&str; 21] = [
  "instruction_start",
  "instruction_end",
  "is_builtin",
  "builtin_id",
  "kind",
  "is_interpreter_trampoline_builtin",
  "is_baseline_trampoline_builtin",
  "is_baseline_leave_frame_builtin",
  "has_instruction_stream",
  "is_maglevved",
  "is_turbofanned",
  "has_tagged_outgoing_params",
  "marked_for_deoptimization",
  "raw_instruction_stream",
  "stack_slots",
  "parameter_count",
  "parameter_count_without_receiver",
  "wasm_js_tagged_parameter_count",
  "wasm_js_first_tagged_parameter",
  "constant_pool",
  "safepoint_table_address",
];

/// 对象总大小断言 (原版 static_assert(kSize == 88 于基准配置))
const _: () = assert!(size_of::<Code>() == K_SIZE);
const _: () = assert!(K_SIZE == K_UNALIGNED_SIZE);

/// Code 对象 (ExposedTrustedObject 头 8 + 数据区, 基准非压缩布局 88 字节)
#[repr(C)]
pub struct Code {
  /// ExposedTrustedObject→TrustedObject→HeapObject 头 (map 槽, 偏移 0)
  pub header: HeapObject,
  /// [kDeoptimizationDataOrInterpreterDataOffset] 受保护槽:
  /// 优化码 DeoptimizationData / baseline 码 BytecodeArray|InterpreterData / 其余 Smi::zero()
  pub deoptimization_data_or_interpreter_data: usize,
  /// [kPositionTableOffset] 受保护槽: baseline = 字节码偏移表, 其余 = 源位置表, 嵌入内建 = Smi::zero()
  pub position_table: usize,
  /// [kWrapperOffset] 沙箱内 CodeWrapper 强引用 (起始强字段)
  pub wrapper: Tagged<Object>,
  /// [kInstructionStreamOffset] 指令流回链 (code_cage_base 特判槽; 末强字段)
  pub instruction_stream: Tagged<Object>,
  /// [kInstructionStartOffset] 入口点 (沙箱开存 tag 异或值; 基线直存)
  pub instruction_start: usize,
  /// [kDispatchHandleOffset] JSDispatchHandle (u32 底层位)
  pub dispatch_handle: u32,
  /// [kFlagsOffset] 标志位域 (RELAXED_UINT32_ACCESSORS)
  pub flags: AtomicU32,
  /// [kInstructionSizeOffset] 指令体字节数
  pub instruction_size: i32,
  /// [kMetadataSizeOffset] 元数据区字节数
  pub metadata_size: i32,
  /// [kInlinedBytecodeSizeOffset] 内联字节码大小; wasm-js 包装复用为两个 u16 半区
  pub inlined_bytecode_size: u32,
  /// [kOsrOffsetOffset] OSR 字节码偏移 (BytecodeOffset 原值)
  pub osr_offset: i32,
  /// [kHandlerTableOffsetOffset] 处理表元数据偏移
  pub handler_table_offset: i32,
  /// [kUnwindingInfoOffsetOffset] unwinding 信息元数据偏移
  pub unwinding_info_offset: i32,
  /// [kCodeCommentsOffsetOffset] 代码注释元数据偏移 (恒常池关: constant_pool_offset 重定向至此)
  pub code_comments_offset: i32,
  /// [kParameterCountOffset] 形参计数 (含接收者)
  pub parameter_count: u16,
  /// [kBuiltinIdOffset] 内建号 (int16, 符号扩展保 -1)
  pub builtin_id: i16,
}

// 布局逐项断言 (与 CODE_FIELDS 偏移表全等)
const _: () = assert!(offset_of!(Code, header) == 0);
const _: () = assert!(offset_of!(Code, deoptimization_data_or_interpreter_data) == 8);
const _: () = assert!(offset_of!(Code, position_table) == 16);
const _: () = assert!(offset_of!(Code, wrapper) == 24);
const _: () = assert!(offset_of!(Code, instruction_stream) == 32);
const _: () = assert!(offset_of!(Code, instruction_start) == 40);
const _: () = assert!(offset_of!(Code, dispatch_handle) == 48);
const _: () = assert!(offset_of!(Code, flags) == 52);
const _: () = assert!(offset_of!(Code, instruction_size) == 56);
const _: () = assert!(offset_of!(Code, metadata_size) == 60);
const _: () = assert!(offset_of!(Code, inlined_bytecode_size) == 64);
const _: () = assert!(offset_of!(Code, osr_offset) == 68);
const _: () = assert!(offset_of!(Code, handler_table_offset) == 72);
const _: () = assert!(offset_of!(Code, unwinding_info_offset) == 76);
const _: () = assert!(offset_of!(Code, code_comments_offset) == 80);
const _: () = assert!(offset_of!(Code, parameter_count) == 84);
const _: () = assert!(offset_of!(Code, builtin_id) == 86);

impl Code {
  /// 构造 (map + 零值数据区; 原版 Factory::NewCode 分配后逐字段装配, 偏差登记:
  /// 分配面归 heap 席, 此处提供脱离堆构造入口)
  #[inline]
  pub fn new(map: Tagged<Map>) -> Self {
    Self {
      header: HeapObject::new(map),
      deoptimization_data_or_interpreter_data: 0,
      position_table: 0,
      wrapper: Tagged::NULL,
      instruction_stream: Smi::zero().cast(),
      instruction_start: 0,
      dispatch_handle: K_NULL_JS_DISPATCH_HANDLE,
      flags: AtomicU32::new(0),
      instruction_size: 0,
      metadata_size: 0,
      inlined_bytecode_size: 0,
      osr_offset: BYTECODE_OFFSET_NONE,
      handler_table_offset: 0,
      unwinding_info_offset: 0,
      code_comments_offset: 0,
      parameter_count: 0,
      builtin_id: K_NO_BUILTIN_ID,
    }
  }
}

/// 原版 JSDispatchHandle::kNullHandle (沙箱关配置句柄面恒空槽; 值 0)
pub const K_NULL_JS_DISPATCH_HANDLE: u32 = 0;

/// 原版 BytecodeOffset::None() (kBytecodeOffsetNone = -1)
pub const BYTECODE_OFFSET_NONE: i32 = -1;

/// Code 语义面 (原版 code.{h,cc,-inl.h} 方法族, 挂载于 Tagged&lt;Code&gt;)
pub trait CodeExt: Copy {
  /// 自身 `Tagged<Code>` 的裸地址 (剥 tag)
  fn address(&self) -> usize;

  // ===== 标志位域 =====

  /// v8: src/objects/code.cc RELAXED_UINT32_ACCESSORS(Code, flags, kFlagsOffset) 读
  fn flags(&self) -> u32;
  /// v8: src/objects/code.cc flags(kRelaxedStore) 写
  fn set_flags(&self, value: u32);
  /// v8: src/objects/code-inl.h Code::kind (KindField::decode(flags(kRelaxedLoad)))
  fn kind(&self) -> Result<CodeKind>;
  /// v8: src/objects/code-inl.h Code::is_disabled_builtin
  fn is_disabled_builtin(&self) -> bool;
  /// v8: src/objects/code-inl.h Code::set_is_disabled_builtin
  fn set_is_disabled_builtin(&self, value: bool);
  /// v8: src/objects/code-inl.h Code::is_turbofanned
  fn is_turbofanned(&self) -> bool;
  /// v8: src/objects/code-inl.h Code::is_context_specialized
  fn is_context_specialized(&self) -> bool;
  /// v8: src/objects/code-inl.h Code::marked_for_deoptimization
  fn marked_for_deoptimization(&self) -> bool;
  /// v8: src/objects/code-inl.h Code::set_marked_for_deoptimization
  /// (原版 DCHECK_IMPLIES(AllowDeoptimization::IsAllowed) 归 heap 席, 不拦)
  fn set_marked_for_deoptimization(&self, flag: bool);
  /// v8: src/objects/code-inl.h Code::embedded_objects_cleared
  fn embedded_objects_cleared(&self) -> bool;
  /// v8: src/objects/code-inl.h Code::set_embedded_objects_cleared
  fn set_embedded_objects_cleared(&self, flag: bool);
  /// v8: src/objects/code-inl.h Code::can_have_weak_objects
  fn can_have_weak_objects(&self) -> bool;
  /// v8: src/objects/code-inl.h Code::set_can_have_weak_objects
  fn set_can_have_weak_objects(&self, value: bool);

  // ===== 内建号 =====

  /// v8: src/objects/code-inl.h Code::builtin_id (int16 符号扩展保 kNoBuiltinId)
  fn builtin_id(&self) -> i16;
  /// v8: src/objects/code-inl.h Code::set_builtin_id (Relaxed_WriteField<int16_t>)
  fn set_builtin_id(&self, builtin_id: i16);
  /// v8: src/objects/code-inl.h Code::is_builtin
  fn is_builtin(&self) -> bool;

  // ===== 指令流与入口 =====

  /// v8: src/objects/code-inl.h Code::raw_instruction_stream (ExternalCodeField load)
  fn raw_instruction_stream(&self) -> Tagged<Object>;
  /// v8: src/objects/code-inl.h Code::raw_instruction_stream(RelaxedLoadTag)
  fn raw_instruction_stream_relaxed(&self) -> Tagged<Object>;
  /// v8: src/objects/code-inl.h Code::set_raw_instruction_stream (Release_Store + 屏障桩)
  fn set_raw_instruction_stream(&self, value: Tagged<Object>);
  /// v8: src/objects/code-inl.h Code::instruction_stream (DCHECK has_instruction_stream)
  fn instruction_stream(&self) -> Result<Tagged<InstructionStream>>;
  /// v8: src/objects/code-inl.h Code::instruction_stream(RelaxedLoadTag)
  fn instruction_stream_relaxed(&self) -> Result<Tagged<InstructionStream>>;
  /// v8: src/objects/code-inl.h Code::unchecked_instruction_stream (GC 免检)
  fn unchecked_instruction_stream(&self) -> Tagged<InstructionStream>;
  /// v8: src/objects/code-inl.h Code::has_instruction_stream
  fn has_instruction_stream(&self) -> bool;
  /// v8: src/objects/code-inl.h Code::has_instruction_stream(RelaxedLoadTag)
  fn has_instruction_stream_relaxed(&self) -> bool;
  /// v8: src/objects/code-inl.h DEF_GETTER(Code, instruction_start, Address) (沙箱关: 直读)
  fn instruction_start(&self) -> usize;
  /// v8: src/objects/code-inl.h Code::set_instruction_start (沙箱关: 直写, isolate 参数退化)
  fn set_instruction_start(&self, value: usize);
  /// v8: src/objects/code-inl.h Code::instruction_end
  fn instruction_end(&self) -> usize;
  /// v8: src/objects/code-inl.h Code::instruction_size 读
  fn instruction_size(&self) -> i32;
  /// v8: src/objects/code-inl.h Code::set_instruction_size
  fn set_instruction_size(&self, value: i32);
  /// v8: src/objects/code-inl.h Code::metadata_size 读
  fn metadata_size(&self) -> i32;
  /// v8: src/objects/code-inl.h Code::set_metadata_size
  fn set_metadata_size(&self, value: i32);
  /// v8: src/objects/code-inl.h Code::body_start
  fn body_start(&self) -> usize;
  /// v8: src/objects/code-inl.h Code::body_size
  fn body_size(&self) -> i32;
  /// v8: src/objects/code-inl.h Code::body_end
  fn body_end(&self) -> usize;
  /// v8: src/objects/code-inl.h Code::SetInstructionStreamAndInstructionStart
  /// (isolate 参数基线退化: 沙箱关无 CPT; 写屏障桩归 heap 席)
  fn set_instruction_stream_and_instruction_start(&self, istream: Tagged<InstructionStream>);
  /// v8: src/objects/code-inl.h Code::SetInstructionStartForOffHeapBuiltin
  fn set_instruction_start_for_off_heap_builtin(&self, entry: usize);
  /// v8: src/objects/code-inl.h Code::ClearInstructionStartForSerialization
  /// (基线沙箱关: self_indirect_pointer 字段不存在, 仅入口清零 + 流槽 Smi::zero)
  fn clear_instruction_start_for_serialization(&self);
  /// v8: src/objects/code-inl.h Code::UpdateInstructionStart
  fn update_instruction_start(&self, istream: Tagged<InstructionStream>) -> Result<()>;

  // ===== 元数据区 =====

  /// v8: src/objects/code-inl.h Code::metadata_start
  /// (内建支 EmbeddedData::FromBlob().MetadataStartOf 推迟: 台账)
  fn metadata_start(&self) -> usize;
  /// v8: src/objects/code-inl.h Code::metadata_end
  fn metadata_end(&self) -> usize;
  /// v8: src/objects/code-inl.h Code::safepoint_table_offset (恒 0)
  fn safepoint_table_offset(&self) -> i32 {
    0
  }
  /// v8: src/objects/code-inl.h Code::safepoint_table_address
  fn safepoint_table_address(&self) -> usize;
  /// v8: src/objects/code-inl.h Code::safepoint_table_size
  fn safepoint_table_size(&self) -> i32;
  /// v8: src/objects/code-inl.h Code::has_safepoint_table
  fn has_safepoint_table(&self) -> bool;
  /// v8: src/objects/code-inl.h Code::handler_table_address
  fn handler_table_address(&self) -> usize;
  /// v8: src/objects/code-inl.h Code::handler_table_offset 读
  fn handler_table_offset(&self) -> i32;
  /// v8: src/objects/code-inl.h Code::set_handler_table_offset
  fn set_handler_table_offset(&self, value: i32);
  /// v8: src/objects/code-inl.h Code::handler_table_size
  fn handler_table_size(&self) -> i32;
  /// v8: src/objects/code-inl.h Code::has_handler_table
  fn has_handler_table(&self) -> bool;
  /// v8: src/objects/code-inl.h Code::constant_pool_offset (恒常池关: 重定向 code_comments_offset)
  fn constant_pool_offset(&self) -> i32;
  /// v8: src/objects/code-inl.h Code::set_constant_pool_offset (恒常池关: 空操作)
  fn set_constant_pool_offset(&self, value: i32);
  /// v8: src/objects/code-inl.h Code::constant_pool_size
  fn constant_pool_size(&self) -> i32;
  /// v8: src/objects/code-inl.h Code::has_constant_pool
  fn has_constant_pool(&self) -> bool {
    self.constant_pool_size() > 0
  }
  /// v8: src/objects/code-inl.h Code::constant_pool
  fn constant_pool(&self) -> usize;
  /// v8: src/objects/code-inl.h Code::code_comments
  fn code_comments(&self) -> usize;
  /// v8: src/objects/code-inl.h Code::code_comments_offset 读
  fn code_comments_offset(&self) -> i32;
  /// v8: src/objects/code-inl.h Code::set_code_comments_offset
  fn set_code_comments_offset(&self, value: i32);
  /// v8: src/objects/code-inl.h Code::code_comments_size
  fn code_comments_size(&self) -> i32;
  /// v8: src/objects/code-inl.h Code::has_code_comments
  fn has_code_comments(&self) -> bool {
    self.code_comments_size() > 0
  }
  /// v8: src/objects/code-inl.h Code::jump_table_info_offset (跳转表关: 重定向 unwinding_info_offset)
  fn jump_table_info_offset(&self) -> i32;
  /// v8: src/objects/code-inl.h Code::set_jump_table_info_offset (跳转表关: 空操作)
  fn set_jump_table_info_offset(&self, value: i32);
  /// v8: src/objects/code-inl.h Code::jump_table_info
  fn jump_table_info(&self) -> usize;
  /// v8: src/objects/code-inl.h Code::jump_table_info_size
  fn jump_table_info_size(&self) -> i32;
  /// v8: src/objects/code-inl.h Code::has_jump_table_info
  fn has_jump_table_info(&self) -> bool {
    self.jump_table_info_size() > 0
  }
  /// v8: src/objects/code-inl.h Code::unwinding_info_start
  fn unwinding_info_start(&self) -> usize;
  /// v8: src/objects/code-inl.h Code::unwinding_info_end
  fn unwinding_info_end(&self) -> usize;
  /// v8: src/objects/code-inl.h Code::unwinding_info_offset 读
  fn unwinding_info_offset(&self) -> i32;
  /// v8: src/objects/code-inl.h Code::set_unwinding_info_offset
  fn set_unwinding_info_offset(&self, value: i32);
  /// v8: src/objects/code-inl.h Code::unwinding_info_size
  fn unwinding_info_size(&self) -> i32;
  /// v8: src/objects/code-inl.h Code::has_unwinding_info
  fn has_unwinding_info(&self) -> bool {
    self.unwinding_info_size() > 0
  }
  /// v8: src/objects/code-inl.h Code::uses_safepoint_table
  fn uses_safepoint_table(&self) -> bool;
  /// v8: src/objects/code-inl.h Code::stack_slots (safepoint 表首 u32 字段 LE)
  fn stack_slots(&self) -> u32;

  // ===== 参数与字节码尺寸 =====

  /// v8: src/objects/code-inl.h UINT16_ACCESSORS(Code, parameter_count) 读
  fn parameter_count(&self) -> u16;
  /// v8: src/objects/code-inl.h set_parameter_count
  fn set_parameter_count(&self, value: u16);
  /// v8: src/objects/code-inl.h Code::parameter_count_without_receiver
  fn parameter_count_without_receiver(&self) -> u16;
  /// v8: src/objects/code-inl.h Code::inlined_bytecode_size
  fn inlined_bytecode_size(&self) -> u32;
  /// v8: src/objects/code-inl.h Code::set_inlined_bytecode_size
  fn set_inlined_bytecode_size(&self, size: u32);
  /// v8: src/objects/code-inl.h Code::wasm_js_tagged_parameter_count 读
  /// (kInlinedBytecodeSizeOffset 低 u16 半区)
  fn wasm_js_tagged_parameter_count(&self) -> u16;
  /// v8: src/objects/code-inl.h Code::set_wasm_js_tagged_parameter_count
  fn set_wasm_js_tagged_parameter_count(&self, count: u16);
  /// v8: src/objects/code-inl.h Code::wasm_js_first_tagged_parameter 读 (+2 半区)
  fn wasm_js_first_tagged_parameter(&self) -> u16;
  /// v8: src/objects/code-inl.h Code::set_wasm_js_first_tagged_parameter
  fn set_wasm_js_first_tagged_parameter(&self, count: u16);
  /// v8: src/objects/code-inl.h Code::osr_offset (BytecodeOffset 原值)
  fn osr_offset(&self) -> i32;
  /// v8: src/objects/code-inl.h Code::set_osr_offset
  fn set_osr_offset(&self, offset: i32);

  // ===== 去优化数据 / baseline 数据 =====

  /// v8: src/objects/code-inl.h Code::uses_deoptimization_data
  fn uses_deoptimization_data(&self) -> bool;
  /// v8: src/objects/code-inl.h Code::deoptimization_data 读 (受保护槽)
  fn deoptimization_data_raw(&self) -> Tagged<DeoptimizationData>;
  /// v8: src/objects/code-inl.h Code::deoptimization_data (SBXCHECK uses_deoptimization_data)
  fn deoptimization_data(&self) -> Result<Tagged<DeoptimizationData>>;
  /// v8: src/objects/code-inl.h Code::set_deoptimization_data
  fn set_deoptimization_data(&self, value: Tagged<DeoptimizationData>) -> Result<()>;
  /// v8: src/objects/code-inl.h Code::unchecked_deoptimization_data (GC 免检)
  fn unchecked_deoptimization_data(&self) -> Tagged<DeoptimizationData>;
  /// v8: src/objects/code-inl.h Code::clear_deoptimization_data_and_interpreter_data
  fn clear_deoptimization_data_and_interpreter_data(&self);
  /// v8: src/objects/code-inl.h Code::has_deoptimization_data_or_interpreter_data
  fn has_deoptimization_data_or_interpreter_data(&self) -> bool;
  /// v8: src/objects/code-inl.cc Code::bytecode_or_interpreter_data 读 (SBXCHECK BASELINE)
  fn bytecode_or_interpreter_data_raw(&self) -> Result<Tagged<Object>>;
  /// v8: src/objects/code-inl.cc Code::set_bytecode_or_interpreter_data
  fn set_bytecode_or_interpreter_data(&self, value: Tagged<Object>) -> Result<()>;

  // ===== 位置表 =====

  /// v8: src/objects/code-inl.h Code::source_position_table 读 (DCHECK has_source_position_table)
  fn source_position_table_raw(&self) -> usize;
  /// v8: src/objects/code-inl.h Code::set_source_position_table
  fn set_source_position_table(&self, value: usize);
  /// v8: src/objects/code-inl.h Code::bytecode_offset_table 读
  fn bytecode_offset_table_raw(&self) -> usize;
  /// v8: src/objects/code-inl.h Code::set_bytecode_offset_table
  fn set_bytecode_offset_table(&self, value: usize);
  /// v8: src/objects/code-inl.h Code::has_source_position_table_or_bytecode_offset_table
  fn has_source_position_table_or_bytecode_offset_table(&self) -> bool;
  /// v8: src/objects/code-inl.h Code::has_source_position_table
  fn has_source_position_table(&self) -> bool;
  /// v8: src/objects/code-inl.h Code::has_bytecode_offset_table(&self)
  fn has_bytecode_offset_table(&self) -> bool;
  /// v8: src/objects/code-inl.h Code::clear_source_position_table_and_bytecode_offset_table
  fn clear_source_position_table_and_bytecode_offset_table(&self);

  // ===== 谓词族 =====

  /// v8: src/objects/code-inl.h Code::is_maglevved
  fn is_maglevved(&self) -> bool;
  /// v8: src/objects/code-inl.h Code::is_optimized_code
  fn is_optimized_code(&self) -> bool;
  /// v8: src/objects/code-inl.h Code::is_wasm_code
  fn is_wasm_code(&self) -> bool;
  /// v8: src/objects/code-inl.h CodeKindHasTaggedOutgoingParams + has_tagged_outgoing_params
  /// (WebAssembly 编译支 kWasmCompileLazy 排除项推迟: 台账)
  fn has_tagged_outgoing_params(&self) -> Result<bool>;
  // v8: src/objects/code.h Code::checks_tiering_state / trampoline 谓词族依赖
  // Builtins 枚举常量 (kCompileLazy 等), 归 v8-init 席, 本波不落地 — 见台账

  // ===== CodeWrapper / 句柄 =====

  /// v8: src/objects/code-inl.h ACCESSORS(Code, wrapper, Tagged<CodeWrapper>) 读
  fn wrapper(&self) -> Tagged<Object>;
  /// v8: src/objects/code-inl.h set_wrapper
  fn set_wrapper(&self, value: Tagged<Object>);
  /// v8: src/objects/code-inl.h Code::js_dispatch_handle
  fn js_dispatch_handle(&self) -> u32;
  /// v8: src/objects/code-inl.h Code::set_js_dispatch_handle (Relaxed_WriteField<u32>)
  fn set_js_dispatch_handle(&self, handle: u32);

  // ===== 装配 =====

  /// v8: src/objects/code-inl.h Code::initialize_flags
  fn initialize_flags(&self, kind: CodeKind, is_context_specialized: bool, is_turbofanned: bool);
  /// v8: src/objects/code-inl.h Code::clear_padding
  fn clear_padding(&self);

  // ===== 尺寸面 =====

  /// v8: src/objects/code-inl.h Code::InstructionStreamObjectSize
  fn instruction_stream_object_size(&self) -> i32;
  /// v8: src/objects/code-inl.h Code::relocation_size (内建支 = 0)
  fn relocation_size(&self) -> i32;
  /// v8: src/objects/code-inl.h Code::SizeIncludingMetadata
  /// (deopt 数据尺寸经 DeoptimizationData::size; 嵌入内建 metadata_start 支推迟)
  fn size_including_metadata(&self) -> Result<i32>;

  // ===== 沙箱与入口 tag =====

  /// v8: src/objects/code-inl.h Code::entrypoint_tag
  /// (沙箱关配置仅诊断用; UNREACHABLE 支 → Error::Invariant)
  fn entrypoint_tag(&self) -> Result<&'static str>;
  /// v8: src/objects/code-inl.h Code::sandboxing_mode
  /// (内建支 Builtins::SandboxingModeOf 推迟: 台账)
  fn sandboxing_mode(&self) -> Result<CodeSandboxingMode>;

  // ===== 弱对象判定与遍历 =====

  /// v8: src/objects/code.cc Code::CanContainWeakObjects
  fn can_contain_weak_objects(&self) -> bool;
  /// v8: src/objects/code.cc Code::IsWeakObject
  fn is_weak_object(&self, object: Tagged<HeapObjectView>) -> bool;
  /// v8: src/objects/code.cc Code::IterateDeoptimizationLiterals
  /// (v 逐弱项派发; GetHeapObject 失败 = 已清除/非堆, 跳过)
  fn iterate_deoptimization_literals<V: RootVisitor>(&self, v: &mut V) -> Result<()>;

  // ===== 源位置 =====

  /// v8: src/objects/code.cc Code::SourcePosition (迭代器推迟: 台账)
  fn source_position(&self, offset: i32) -> Result<i32>;
  /// v8: src/objects/code.cc Code::SourceStatementPosition (迭代器推迟: 台账)
  fn source_statement_position(&self, offset: i32) -> Result<i32>;
}

/// HeapObject 视图别名 (原版 Tagged<HeapObject> 参数位)
pub type HeapObjectView = HeapObject;

/// 沙箱运行模式 (原版 enum class CodeSandboxingMode, sandbox/code-entrypoint-tag.h)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CodeSandboxingMode {
  /// 受沙箱保护 (运行期生成码一律此值)
  Sandboxed = 0,
  /// 不受沙箱保护 (白名单内建)
  Unsandboxed = 1,
  /// 不受沙箱保护 + CFI (wasm 等)
  UnsandboxedWithCfi = 2,
}

/// 原版 CodeKindHasTaggedOutgoingParams (code-inl.h:435):
/// 排除 JS_TO_WASM_FUNCTION / C_WASM_ENTRY / WASM_FUNCTION
#[inline]
#[must_use]
pub const fn code_kind_has_tagged_outgoing_params(kind: CodeKind) -> bool {
  !matches!(
    kind,
    CodeKind::JS_TO_WASM_FUNCTION | CodeKind::C_WASM_ENTRY | CodeKind::WASM_FUNCTION
  )
}

/// 标记反优化位 (生成代码直访, 原版 kMarkedForDeoptimizationBit = MarkedForDeoptimizationField::kShift)
#[inline]
#[must_use]
pub const fn marked_for_deoptimization_bit() -> u32 {
  K_MARKED_FOR_DEOPTIMIZATION_BIT
}

/// 入口点标签 (原版 entrypoint_tag switch; 沙箱开时入口地址存 tag 异或值)
///
/// 映射表 (CodeKind → CodeEntrypointTag, BUILTIN 支按禁用位/内建表细分):
pub const ENTRYPOINT_TAGS: [(&str, &str); 12] = [
  ("BYTECODE_HANDLER", "kBytecodeHandlerEntrypointTag"),
  ("REGEXP", "kRegExpEntrypointTag"),
  ("WASM_FUNCTION", "kWasmEntrypointTag"),
  ("WASM_TO_CAPI_FUNCTION", "kWasmEntrypointTag"),
  ("WASM_TO_JS_FUNCTION", "kWasmEntrypointTag"),
  ("JS_TO_WASM_FUNCTION", "kJSEntrypointTag"),
  ("FOR_TESTING", "kCodeEntrypointTagForTesting"),
  ("FOR_TESTING_JS", "kJSEntrypointTag"),
  ("C_WASM_ENTRY", "kInvalidEntrypointTag"),
  ("BASELINE", "kJSEntrypointTag"),
  ("MAGLEV", "kJSEntrypointTag"),
  ("TURBOFAN_JS", "kJSEntrypointTag"),
];

/// 位域单 bit 更新 (base::BitField::update 1:1)
#[inline]
const fn bit_update(previous: u32, shift: u32, value: bool) -> u32 {
  let mask = 1u32 << shift;
  if value {
    previous | mask
  } else {
    previous & !mask
  }
}

/// 受保护槽直写 (原版 WriteProtectedPointerField(kDeoptimizationDataOrInterpreterDataOffset);
/// 基线非沙箱配置无写屏障桩, 桩归 heap 席)
///
/// v8: src/objects/code-inl.h Code::set_deoptimization_data / set_bytecode_or_interpreter_data
#[inline]
fn set_deopt_slot(code: &Tagged<Code>, value: usize) {
  // SAFETY: 契约同 flags — Tagged<Code> 指向有效 Code 对象内存
  unsafe {
    (*code.as_mut_ptr()).deoptimization_data_or_interpreter_data = value;
  }
}

impl CodeExt for Tagged<Code> {
  #[inline]
  fn address(&self) -> usize {
    Tagged::<Code>::address(self)
  }

  #[inline]
  fn flags(&self) -> u32 {
    // SAFETY: 调用方契约 — Tagged<Code> 指向已初始化 Code 对象内存 (同 FeedbackVector 先例)
    unsafe { (*self.as_ptr()).flags.load(Ordering::Relaxed) }
  }

  #[inline]
  fn set_flags(&self, value: u32) {
    // SAFETY: 同上契约; Relaxed 写对齐 RELAXED_UINT32_ACCESSORS
    unsafe { (*self.as_ptr()).flags.store(value, Ordering::Relaxed) }
  }

  #[inline]
  fn kind(&self) -> Result<CodeKind> {
    let raw = ((self.flags() & KIND_MASK) >> KIND_SHIFT) as u8;
    CodeKind::from_repr(raw).ok_or(Error::Invariant("Code::kind 位域解码越界"))
  }

  #[inline]
  fn is_disabled_builtin(&self) -> bool {
    (self.flags() >> IS_DISABLED_BUILTIN_SHIFT) & 1 != 0
  }

  #[inline]
  fn set_is_disabled_builtin(&self, value: bool) {
    debug_assert!(self.is_builtin(), "原版 DCHECK(is_builtin)");
    let updated = bit_update(self.flags(), IS_DISABLED_BUILTIN_SHIFT, value);
    self.set_flags(updated);
  }

  #[inline]
  fn is_turbofanned(&self) -> bool {
    (self.flags() >> IS_TURBOFANNED_SHIFT) & 1 != 0
  }

  #[inline]
  fn is_context_specialized(&self) -> bool {
    (self.flags() >> IS_CONTEXT_SPECIALIZED_SHIFT) & 1 != 0
  }

  #[inline]
  fn marked_for_deoptimization(&self) -> bool {
    (self.flags() >> MARKED_FOR_DEOPT_SHIFT) & 1 != 0
  }

  #[inline]
  fn set_marked_for_deoptimization(&self, flag: bool) {
    let updated = bit_update(self.flags(), MARKED_FOR_DEOPT_SHIFT, flag);
    self.set_flags(updated);
  }

  #[inline]
  fn embedded_objects_cleared(&self) -> bool {
    (self.flags() >> EMBEDDED_OBJECTS_CLEARED_SHIFT) & 1 != 0
  }

  #[inline]
  fn set_embedded_objects_cleared(&self, flag: bool) {
    debug_assert!(
      !flag || self.marked_for_deoptimization(),
      "原版 DCHECK_IMPLIES"
    );
    let updated = bit_update(self.flags(), EMBEDDED_OBJECTS_CLEARED_SHIFT, flag);
    self.set_flags(updated);
  }

  #[inline]
  fn can_have_weak_objects(&self) -> bool {
    (self.flags() >> CAN_HAVE_WEAK_OBJECTS_SHIFT) & 1 != 0
  }

  #[inline]
  fn set_can_have_weak_objects(&self, value: bool) {
    let updated = bit_update(self.flags(), CAN_HAVE_WEAK_OBJECTS_SHIFT, value);
    self.set_flags(updated);
  }

  #[inline]
  fn builtin_id(&self) -> i16 {
    // SAFETY: 契约同 flags; int16 直读, 符号扩展天然保持
    unsafe { (*self.as_ptr()).builtin_id }
  }

  #[inline]
  fn set_builtin_id(&self, builtin_id: i16) {
    // SAFETY: 契约同 flags; Relaxed_WriteField<int16_t> 等价直写
    unsafe {
      (*self.as_mut_ptr()).builtin_id = builtin_id;
    }
  }

  #[inline]
  fn is_builtin(&self) -> bool {
    self.builtin_id() != K_NO_BUILTIN_ID
  }

  #[inline]
  fn raw_instruction_stream(&self) -> Tagged<Object> {
    // SAFETY: 契约同 flags
    unsafe { (*self.as_ptr()).instruction_stream }
  }

  #[inline]
  fn raw_instruction_stream_relaxed(&self) -> Tagged<Object> {
    // SAFETY: 契约同 flags; 非压缩基线槽读写即原子 relaxed 同构
    // Tagged<Object> 为 repr(transparent) 全宽 usize 槽, 取字段裸址转 AtomicUsize 视图
    unsafe {
      Tagged::new(
        AtomicUsize::from_ptr(&raw const (*self.as_ptr()).instruction_stream as *mut usize)
          .load(Ordering::Relaxed),
      )
    }
  }

  #[inline]
  fn set_raw_instruction_stream(&self, value: Tagged<Object>) {
    // SAFETY: 契约同 flags; Release_Store 语义 (原版 ExternalCodeField::Release_Store);
    // 写屏障桩 (CONDITIONAL_WRITE_BARRIER) 归 heap 席
    unsafe {
      AtomicUsize::from_ptr(&raw const (*self.as_ptr()).instruction_stream as *mut usize)
        .store(value.ptr(), Ordering::Release);
    }
  }

  #[inline]
  fn instruction_stream(&self) -> Result<Tagged<InstructionStream>> {
    if !self.has_instruction_stream() {
      return Err(Error::Invariant(
        "Code::instruction_stream: 无指令流 (嵌入内建或已序列化清零)",
      ));
    }
    Ok(self.unchecked_instruction_stream())
  }

  #[inline]
  fn instruction_stream_relaxed(&self) -> Result<Tagged<InstructionStream>> {
    if !self.has_instruction_stream_relaxed() {
      return Err(Error::Invariant(
        "Code::instruction_stream(Relaxed): 无指令流",
      ));
    }
    Ok(self.raw_instruction_stream_relaxed().cast())
  }

  #[inline]
  fn unchecked_instruction_stream(&self) -> Tagged<InstructionStream> {
    self.raw_instruction_stream().cast()
  }

  #[inline]
  fn has_instruction_stream(&self) -> bool {
    // 原版读 uint64 字段判非 0 (Smi::zero 即 0)
    self.raw_instruction_stream().ptr() != 0
  }

  #[inline]
  fn has_instruction_stream_relaxed(&self) -> bool {
    self.raw_instruction_stream_relaxed().ptr() != 0
  }

  #[inline]
  fn instruction_start(&self) -> usize {
    // 沙箱关分支: 直读 (DEF_GETTER 的 ReadField<Address>)
    // SAFETY: 契约同 flags
    unsafe { (*self.as_ptr()).instruction_start }
  }

  #[inline]
  fn set_instruction_start(&self, value: usize) {
    // SAFETY: 契约同 flags
    unsafe {
      (*self.as_mut_ptr()).instruction_start = value;
    }
  }

  #[inline]
  fn instruction_end(&self) -> usize {
    self.instruction_start() + self.instruction_size() as usize
  }

  #[inline]
  fn instruction_size(&self) -> i32 {
    // SAFETY: 契约同 flags
    unsafe { (*self.as_ptr()).instruction_size }
  }

  #[inline]
  fn set_instruction_size(&self, value: i32) {
    // SAFETY: 契约同 flags
    unsafe {
      (*self.as_mut_ptr()).instruction_size = value;
    }
  }

  #[inline]
  fn metadata_size(&self) -> i32 {
    // SAFETY: 契约同 flags
    unsafe { (*self.as_ptr()).metadata_size }
  }

  #[inline]
  fn set_metadata_size(&self, value: i32) {
    // SAFETY: 契约同 flags
    unsafe {
      (*self.as_mut_ptr()).metadata_size = value;
    }
  }

  #[inline]
  fn body_start(&self) -> usize {
    self.instruction_start()
  }

  #[inline]
  fn body_size(&self) -> i32 {
    self.instruction_size() + self.metadata_size()
  }

  #[inline]
  fn body_end(&self) -> usize {
    self.body_start() + self.body_size() as usize
  }

  #[inline]
  fn set_instruction_stream_and_instruction_start(&self, istream: Tagged<InstructionStream>) {
    self.set_raw_instruction_stream(istream.cast());
    self.set_instruction_start(istream.instruction_start());
  }

  #[inline]
  fn set_instruction_start_for_off_heap_builtin(&self, entry: usize) {
    debug_assert!(!self.has_instruction_stream(), "原版 DCHECK");
    self.set_instruction_start(entry);
  }

  #[inline]
  fn clear_instruction_start_for_serialization(&self) {
    // 基线沙箱关: self_indirect_pointer 字段不存在 (沙箱支见台账口径)
    self.set_instruction_start(0);
    self.set_raw_instruction_stream(Smi::zero().cast());
  }

  #[inline]
  fn update_instruction_start(&self, istream: Tagged<InstructionStream>) -> Result<()> {
    if self.raw_instruction_stream() != istream.cast() {
      return Err(Error::Invariant(
        "Code::UpdateInstructionStart: 流槽与入参不一致 (原版 DCHECK_EQ)",
      ));
    }
    self.set_instruction_start(istream.instruction_start());
    Ok(())
  }

  #[inline]
  fn metadata_start(&self) -> usize {
    if self.has_instruction_stream() {
      // 原版 static_assert(kOnHeapBodyIsContiguous): 指令尾即元数据首
      return self.instruction_start() + self.instruction_size() as usize;
    }
    // v8: src/objects/code-inl.h Code::metadata_start
    // TODO(台账): 嵌入内建支 EmbeddedData::FromBlob().MetadataStartOf(builtin_id)
    // — 依赖 v8-snapshot 嵌入 blob 面, 就位即填。
    todo!("推迟登记(readme/zh.md): EmbeddedData::FromBlob().MetadataStartOf 归 v8-snapshot")
  }

  #[inline]
  fn metadata_end(&self) -> usize {
    self.metadata_start() + self.metadata_size() as usize
  }

  #[inline]
  fn safepoint_table_address(&self) -> usize {
    self.metadata_start() + self.safepoint_table_offset() as usize
  }

  #[inline]
  fn safepoint_table_size(&self) -> i32 {
    self.handler_table_offset() - self.safepoint_table_offset()
  }

  #[inline]
  fn has_safepoint_table(&self) -> bool {
    self.safepoint_table_size() > 0
  }

  #[inline]
  fn handler_table_address(&self) -> usize {
    self.metadata_start() + self.handler_table_offset() as usize
  }

  #[inline]
  fn handler_table_offset(&self) -> i32 {
    // SAFETY: 契约同 flags
    unsafe { (*self.as_ptr()).handler_table_offset }
  }

  #[inline]
  fn set_handler_table_offset(&self, value: i32) {
    // SAFETY: 契约同 flags
    unsafe {
      (*self.as_mut_ptr()).handler_table_offset = value;
    }
  }

  #[inline]
  fn handler_table_size(&self) -> i32 {
    self.constant_pool_offset() - self.handler_table_offset()
  }

  #[inline]
  fn has_handler_table(&self) -> bool {
    self.handler_table_size() > 0
  }

  #[inline]
  fn constant_pool_offset(&self) -> i32 {
    // V8_EMBEDDED_CONSTANT_POOL_BOOL=false: 字段不存在, 重定向 code_comments_offset
    self.code_comments_offset()
  }

  #[inline]
  fn set_constant_pool_offset(&self, value: i32) {
    // 恒常池关: 空操作 (原版 return); 断言仅恒常池开配置有意义
    let _ = value;
  }

  #[inline]
  fn constant_pool_size(&self) -> i32 {
    // 恒常池关: code_comments_offset - constant_pool_offset 恒 0 (原版 DCHECK_EQ(size, 0))
    0
  }

  #[inline]
  fn constant_pool(&self) -> usize {
    if !self.has_constant_pool() {
      return 0; // kNullAddress
    }
    self.metadata_start() + self.constant_pool_offset() as usize
  }

  #[inline]
  fn code_comments(&self) -> usize {
    self.metadata_start() + self.code_comments_offset() as usize
  }

  #[inline]
  fn code_comments_offset(&self) -> i32 {
    // SAFETY: 契约同 flags
    unsafe { (*self.as_ptr()).code_comments_offset }
  }

  #[inline]
  fn set_code_comments_offset(&self, value: i32) {
    // SAFETY: 契约同 flags
    unsafe {
      (*self.as_mut_ptr()).code_comments_offset = value;
    }
  }

  #[inline]
  fn code_comments_size(&self) -> i32 {
    self.jump_table_info_offset() - self.code_comments_offset()
  }

  #[inline]
  fn jump_table_info_offset(&self) -> i32 {
    // V8_JUMP_TABLE_INFO_BOOL=false: 字段不存在, 重定向 unwinding_info_offset
    self.unwinding_info_offset()
  }

  #[inline]
  fn set_jump_table_info_offset(&self, value: i32) {
    let _ = value; // 跳转表关: 空操作 (原版 return)
  }

  #[inline]
  fn jump_table_info(&self) -> usize {
    self.metadata_start() + self.jump_table_info_offset() as usize
  }

  #[inline]
  fn jump_table_info_size(&self) -> i32 {
    self.unwinding_info_offset() - self.jump_table_info_offset()
  }

  #[inline]
  fn unwinding_info_start(&self) -> usize {
    self.metadata_start() + self.unwinding_info_offset() as usize
  }

  #[inline]
  fn unwinding_info_end(&self) -> usize {
    self.metadata_end()
  }

  #[inline]
  fn unwinding_info_offset(&self) -> i32 {
    // SAFETY: 契约同 flags
    unsafe { (*self.as_ptr()).unwinding_info_offset }
  }

  #[inline]
  fn set_unwinding_info_offset(&self, value: i32) {
    // SAFETY: 契约同 flags
    unsafe {
      (*self.as_mut_ptr()).unwinding_info_offset = value;
    }
  }

  #[inline]
  fn unwinding_info_size(&self) -> i32 {
    (self.unwinding_info_end() - self.unwinding_info_start()) as i32
  }

  #[inline]
  fn uses_safepoint_table(&self) -> bool {
    self.is_turbofanned() || self.is_maglevved() || self.is_wasm_code()
  }

  #[inline]
  fn stack_slots(&self) -> u32 {
    // 原版: 表长 0 → 0; 否则读表首 SafepointTableStackSlotsField_t (uint32, kSafepointTableStackSlotsOffset=0)
    if self.safepoint_table_size() == 0 {
      return 0;
    }
    // SAFETY: safepoint 表位于指令流可读写入的元数据区 (调用方契约: 对象内存有效)
    unsafe { (self.safepoint_table_address() as *const u32).read_unaligned() }
  }

  #[inline]
  fn parameter_count(&self) -> u16 {
    // SAFETY: 契约同 flags
    unsafe { (*self.as_ptr()).parameter_count }
  }

  #[inline]
  fn set_parameter_count(&self, value: u16) {
    // SAFETY: 契约同 flags
    unsafe {
      (*self.as_mut_ptr()).parameter_count = value;
    }
  }

  #[inline]
  fn parameter_count_without_receiver(&self) -> u16 {
    self.parameter_count() - 1
  }

  #[inline]
  fn inlined_bytecode_size(&self) -> u32 {
    // SAFETY: 契约同 flags
    unsafe { (*self.as_ptr()).inlined_bytecode_size }
  }

  #[inline]
  fn set_inlined_bytecode_size(&self, size: u32) {
    // SAFETY: 契约同 flags
    unsafe {
      (*self.as_mut_ptr()).inlined_bytecode_size = size;
    }
  }

  #[inline]
  fn wasm_js_tagged_parameter_count(&self) -> u16 {
    // kInlinedBytecodeSizeOffset 低 u16 半区 (RELAXED_READ_UINT16_FIELD)
    (self.inlined_bytecode_size() & 0xffff) as u16
  }

  #[inline]
  fn set_wasm_js_tagged_parameter_count(&self, count: u16) {
    let cur = self.inlined_bytecode_size();
    self.set_inlined_bytecode_size((cur & 0xffff_0000) | u32::from(count));
  }

  #[inline]
  fn wasm_js_first_tagged_parameter(&self) -> u16 {
    // +2 半区
    (self.inlined_bytecode_size() >> 16) as u16
  }

  #[inline]
  fn set_wasm_js_first_tagged_parameter(&self, count: u16) {
    let cur = self.inlined_bytecode_size();
    self.set_inlined_bytecode_size((cur & 0x0000_ffff) | (u32::from(count) << 16));
  }

  #[inline]
  fn osr_offset(&self) -> i32 {
    // SAFETY: 契约同 flags
    unsafe { (*self.as_ptr()).osr_offset }
  }

  #[inline]
  fn set_osr_offset(&self, offset: i32) {
    // SAFETY: 契约同 flags
    unsafe {
      (*self.as_mut_ptr()).osr_offset = offset;
    }
  }

  #[inline]
  fn uses_deoptimization_data(&self) -> bool {
    self.kind().is_ok_and(|k| k.uses_deoptimization_data())
  }

  #[inline]
  fn deoptimization_data_raw(&self) -> Tagged<DeoptimizationData> {
    // SAFETY: 契约同 flags; 受保护槽 = 全宽指针 (原版 ReadProtectedPointerField)
    unsafe { Tagged::new((*self.as_ptr()).deoptimization_data_or_interpreter_data) }
  }

  #[inline]
  fn deoptimization_data(&self) -> Result<Tagged<DeoptimizationData>> {
    // 原版 SBXCHECK(uses_deoptimization_data()): 沙箱关配置降级 debug_assert + 类型面校验
    if !self.uses_deoptimization_data() {
      return Err(Error::Invariant(
        "Code::deoptimization_data: kind 不使用去优化数据 (原版 SBXCHECK)",
      ));
    }
    Ok(self.deoptimization_data_raw())
  }

  #[inline]
  fn set_deoptimization_data(&self, value: Tagged<DeoptimizationData>) -> Result<()> {
    if !self.uses_deoptimization_data() {
      return Err(Error::Invariant(
        "Code::set_deoptimization_data: kind 不使用去优化数据 (原版 SBXCHECK)",
      ));
    }
    set_deopt_slot(self, value.ptr());
    Ok(())
  }

  #[inline]
  fn unchecked_deoptimization_data(&self) -> Tagged<DeoptimizationData> {
    self.deoptimization_data_raw()
  }

  #[inline]
  fn clear_deoptimization_data_and_interpreter_data(&self) {
    // ClearProtectedPointerField: 置空槽
    set_deopt_slot(self, 0);
  }

  #[inline]
  fn has_deoptimization_data_or_interpreter_data(&self) -> bool {
    // IsProtectedPointerFieldEmpty: 空 = 0 槽 (非压缩无哨兵)
    self.deoptimization_data_raw().ptr() != 0
  }

  #[inline]
  fn bytecode_or_interpreter_data_raw(&self) -> Result<Tagged<Object>> {
    // 原版 SBXCHECK_EQ(kind(), BASELINE)
    if self.kind()? != CodeKind::BASELINE {
      return Err(Error::Invariant(
        "Code::bytecode_or_interpreter_data: 非 baseline 码 (原版 SBXCHECK_EQ)",
      ));
    }
    Ok(self.deoptimization_data_raw().cast())
  }

  #[inline]
  fn set_bytecode_or_interpreter_data(&self, value: Tagged<Object>) -> Result<()> {
    if self.kind()? != CodeKind::BASELINE {
      return Err(Error::Invariant(
        "Code::set_bytecode_or_interpreter_data: 非 baseline 码 (原版 SBXCHECK_EQ)",
      ));
    }
    set_deopt_slot(self, value.ptr());
    Ok(())
  }

  #[inline]
  fn source_position_table_raw(&self) -> usize {
    debug_assert!(self.has_source_position_table());
    // SAFETY: 契约同 flags
    unsafe { (*self.as_ptr()).position_table }
  }

  #[inline]
  fn set_source_position_table(&self, value: usize) {
    debug_assert!(!self.kind().is_ok_and(|k| k.uses_bytecode_offset_table()));
    // SAFETY: 契约同 flags; 受保护指针槽直写 (屏障桩归 heap 席)
    unsafe {
      (*self.as_mut_ptr()).position_table = value;
    }
  }

  #[inline]
  fn bytecode_offset_table_raw(&self) -> usize {
    debug_assert!(self.has_bytecode_offset_table());
    // SAFETY: 契约同 flags
    unsafe { (*self.as_ptr()).position_table }
  }

  #[inline]
  fn set_bytecode_offset_table(&self, value: usize) {
    debug_assert!(self.kind().is_ok_and(|k| k.uses_bytecode_offset_table()));
    // SAFETY: 契约同 flags
    unsafe {
      (*self.as_mut_ptr()).position_table = value;
    }
  }

  #[inline]
  fn has_source_position_table_or_bytecode_offset_table(&self) -> bool {
    // TaggedField load != Smi::zero() (Smi::zero 原值即 0)
    // SAFETY: 契约同 flags
    unsafe { (*self.as_ptr()).position_table != Smi::zero().ptr() }
  }

  #[inline]
  fn has_source_position_table(&self) -> bool {
    self.has_source_position_table_or_bytecode_offset_table()
      && !self.kind().is_ok_and(|k| k.uses_bytecode_offset_table())
  }

  #[inline]
  fn has_bytecode_offset_table(&self) -> bool {
    self.has_source_position_table_or_bytecode_offset_table()
      && self.kind().is_ok_and(|k| k.uses_bytecode_offset_table())
  }

  #[inline]
  fn clear_source_position_table_and_bytecode_offset_table(&self) {
    // SAFETY: 契约同 flags
    unsafe {
      (*self.as_mut_ptr()).position_table = Smi::zero().ptr();
    }
  }

  #[inline]
  fn is_maglevved(&self) -> bool {
    self.kind().is_ok_and(|k| k == CodeKind::MAGLEV)
  }

  #[inline]
  fn is_optimized_code(&self) -> bool {
    self.kind().is_ok_and(|k| k.is_optimized_js_function())
  }

  #[inline]
  fn is_wasm_code(&self) -> bool {
    self.kind().is_ok_and(|k| k == CodeKind::WASM_FUNCTION)
  }

  #[inline]
  fn has_tagged_outgoing_params(&self) -> Result<bool> {
    // V8_ENABLE_WEBASSEMBLY=false 分支 1:1 (kWasmCompileLazy 排除项见台账)
    Ok(code_kind_has_tagged_outgoing_params(self.kind()?))
  }

  #[inline]
  fn wrapper(&self) -> Tagged<Object> {
    // SAFETY: 契约同 flags
    unsafe { (*self.as_ptr()).wrapper }
  }

  #[inline]
  fn set_wrapper(&self, value: Tagged<Object>) {
    // SAFETY: 契约同 flags
    unsafe {
      (*self.as_mut_ptr()).wrapper = value;
    }
  }

  #[inline]
  fn js_dispatch_handle(&self) -> u32 {
    // SAFETY: 契约同 flags
    unsafe { (*self.as_ptr()).dispatch_handle }
  }

  #[inline]
  fn set_js_dispatch_handle(&self, handle: u32) {
    // SAFETY: 契约同 flags
    unsafe {
      (*self.as_mut_ptr()).dispatch_handle = handle;
    }
  }

  #[inline]
  fn initialize_flags(&self, kind: CodeKind, is_context_specialized: bool, is_turbofanned: bool) {
    debug_assert!(
      !kind.is_interpreted_js_function(),
      "原版 DCHECK(!CodeKindIsInterpretedJSFunction)"
    );
    let value = ((kind as u32) << KIND_SHIFT)
      | u32::from(is_context_specialized) << IS_CONTEXT_SPECIALIZED_SHIFT
      | u32::from(is_turbofanned) << IS_TURBOFANNED_SHIFT;
    self.set_flags(value);
  }

  #[inline]
  fn clear_padding(&self) {
    if K_SIZE - K_UNALIGNED_SIZE == 0 {
      return;
    }
    // SAFETY: 基线恒不进入; 若布局改动, [kUnalignedSize, kSize) 为本对象内填充区
    unsafe {
      ptr::write_bytes(
        (self.address() + K_UNALIGNED_SIZE) as *mut u8,
        0,
        K_SIZE - K_UNALIGNED_SIZE,
      );
    }
  }

  #[inline]
  fn instruction_stream_object_size(&self) -> i32 {
    instruction_stream::size_for(self.body_size() as usize, K_CODE_ALIGNMENT_BASELINE) as i32
  }

  #[inline]
  fn relocation_size(&self) -> i32 {
    // 原版: 有流 → istream->relocation_size(); 内建 → 0
    if self.has_instruction_stream() {
      self.unchecked_instruction_stream().relocation_size()
    } else {
      0
    }
  }

  #[inline]
  fn size_including_metadata(&self) -> Result<i32> {
    // v8: src/objects/code-inl.h Code::SizeIncludingMetadata
    let mut size = self.instruction_stream_object_size();
    size += self.relocation_size();
    if self.uses_deoptimization_data() {
      size += self.deoptimization_data()?.size_including_header();
    }
    Ok(size)
  }

  #[inline]
  fn entrypoint_tag(&self) -> Result<&'static str> {
    let kind = self.kind()?;
    Ok(match kind {
      CodeKind::BYTECODE_HANDLER => "kBytecodeHandlerEntrypointTag",
      CodeKind::BUILTIN => {
        if self.is_disabled_builtin() {
          "kDisabledBuiltinEntrypointTag"
        } else {
          // 原版 Builtins::EntrypointTagFor(builtin_id): 内建表归 v8-init 席 (台账)
          "kBuiltinEntrypointTagDefault"
        }
      }
      CodeKind::REGEXP => "kRegExpEntrypointTag",
      CodeKind::WASM_FUNCTION | CodeKind::WASM_TO_CAPI_FUNCTION | CodeKind::WASM_TO_JS_FUNCTION => {
        "kWasmEntrypointTag"
      }
      CodeKind::JS_TO_WASM_FUNCTION => "kJSEntrypointTag",
      CodeKind::FOR_TESTING => "kCodeEntrypointTagForTesting",
      CodeKind::FOR_TESTING_JS => "kJSEntrypointTag",
      CodeKind::C_WASM_ENTRY => "kInvalidEntrypointTag",
      CodeKind::BASELINE | CodeKind::MAGLEV | CodeKind::TURBOFAN_JS => "kJSEntrypointTag",
      // 原版 UNREACHABLE: 该 kind 不会出现在 Code 对象上 (解码即数据损坏 → Err)
      CodeKind::INTERPRETED_FUNCTION | CodeKind::WASM_STACK_ENTRY => {
        return Err(Error::Invariant("entrypoint_tag: 原版 UNREACHABLE kind 支"));
      }
    })
  }

  #[inline]
  fn sandboxing_mode(&self) -> Result<CodeSandboxingMode> {
    if self.is_builtin() {
      // v8: src/objects/code-inl.h Code::sandboxing_mode 内建支
      // TODO(台账): Builtins::SandboxingModeOf 内建表归 v8-init/builtins, 就位即填。
      todo!("推迟登记(readme/zh.md): Builtins::SandboxingModeOf 归 v8-init/builtins")
    } else {
      // 运行期生成码一律沙箱内
      Ok(CodeSandboxingMode::Sandboxed)
    }
  }

  #[inline]
  fn can_contain_weak_objects(&self) -> bool {
    self.is_optimized_code() && self.can_have_weak_objects()
  }

  #[inline]
  fn is_weak_object(&self, object: Tagged<HeapObjectView>) -> bool {
    self.can_contain_weak_objects() && is_weak_object_in_optimized_code(object)
  }

  #[inline]
  fn iterate_deoptimization_literals<V: RootVisitor>(&self, v: &mut V) -> Result<()> {
    if !self.uses_deoptimization_data() {
      return Ok(());
    }
    let deopt_data = self.deoptimization_data()?;
    if deopt_data.ulength()? == 0 {
      return Ok(());
    }
    let literals = deopt_data.literal_array();
    let len = literals.ulength()?;
    for i in 0..len {
      let maybe_literal = literals.get_raw(i)?;
      if let Some(heap_literal) = maybe_literal.get_heap_object() {
        // 原版 FullObjectSlot(&heap_literal): 访客可见可改的局部槽 (1:1 语义,
        // 更新不回写字面量数组本体)
        let mut literal_ptr = heap_literal.ptr();
        let slot_addr = &raw mut literal_ptr as usize;
        v.visit_root_pointer(
          Root::StackRoots,
          Some("deoptimization literal"),
          FullObjectSlot::new(slot_addr),
        )?;
      }
    }
    Ok(())
  }

  #[inline]
  fn source_position(&self, offset: i32) -> Result<i32> {
    if self.kind()? == CodeKind::BASELINE {
      return Err(Error::Invariant(
        "Code::SourcePosition: baseline 码不支持 (原版 CHECK_NE)",
      ));
    }
    // 原版: offset-- 后迭代源位置表取最后 code_offset <= offset 项
    let _offset = offset - 1;
    if !self.has_source_position_table() {
      return Ok(0);
    }
    // v8: src/objects/code.cc Code::SourcePosition
    // TODO(台账): SourcePositionTableIterator (src/codegen/source-position-table.cc)
    // 归 v8-codegen-core 面, 依赖决策后接入即填。
    todo!("推迟登记(readme/zh.md): SourcePositionTableIterator 归 v8-codegen-core")
  }

  #[inline]
  fn source_statement_position(&self, offset: i32) -> Result<i32> {
    if self.kind()? == CodeKind::BASELINE {
      return Err(Error::Invariant(
        "Code::source_statement_position: baseline 码不可调用 (原版 CHECK_NE)",
      ));
    }
    let _ = offset;
    todo!("推迟登记(readme/zh.md): SourcePositionTableIterator 归 v8-codegen-core")
  }
}

/// 原版 Code::IsWeakObjectInOptimizedCode(Tagged<HeapObject>) (code.cc:661)
///
/// Map(可迁移)/PropertyCell/JSReceiver/Context 四类弱资格;
/// 其余 (含不可迁移 Map) 强持有。
#[must_use]
pub fn is_weak_object_in_optimized_code(object: Tagged<HeapObject>) -> bool {
  // 原版: type = o->map()->instance_type(); 读不到 map 类型 (非堆对象) → 恒强持有
  let Some(ty) = instance_type_of(object.cast()) else {
    return false;
  };
  if itc::is_map(ty) {
    // Cast<Map>(object)->CanTransition()
    // SAFETY: 调用方契约 — object 为有效 Map 堆对象
    return unsafe { (*object.cast::<Map>().as_ptr()).can_transition() };
  }
  itc::is_property_cell(ty) || itc::is_js_receiver(ty) || itc::is_context(ty)
}

/// 原版 Code::IsWeakObjectInDeoptimizationLiteralArray (code.cc:677)
///
/// Map 必须强持有 — 去优化物化描述可能引用其代码。
#[must_use]
pub fn is_weak_object_in_deoptimization_literal_array(object: Tagged<Object>) -> bool {
  let Some(ty) = instance_type_of(object) else {
    // IsHeapObject(object) 为假 (Smi/非对象)
    return false;
  };
  !itc::is_map(ty) && is_weak_object_in_optimized_code(object.cast())
}

/// 原版 Code::IsWeakObject(JSDispatchHandle) — 派发句柄在优化码中恒弱 (code.cc:671)
#[must_use]
pub const fn is_weak_js_dispatch_handle() -> bool {
  true
}

/// 基线配置 kCodeAlignment (MemoryChunk::kCodeAlignment = CodeRange 页对齐;
/// 对象面仅 InstructionStream::SizeFor 用其倍数 — 由调用方传入, 此处导出常用值)
pub const K_CODE_ALIGNMENT_BASELINE: usize = 8;

// ===== GcSafeCode =====

/// GC 进行中安全访问的 Code 视图 (原版 GcSafeCode : HeapObject, code.h:540)
///
/// 底层指针保证为 Code; Code 与其 InstructionStream 均可能为转发指针,
/// 故免类型检查 (checked cast 不可用)。
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct GcSafeCode {
  /// 原始 Code 指针 (可转发态)
  pub tagged: Tagged<Code>,
}

impl GcSafeCode {
  /// 原版 UnsafeCastToCode: 剥离 GcSafe 语义回 Code
  #[inline]
  #[must_use]
  pub const fn unsafe_cast_to_code(&self) -> Tagged<Code> {
    self.tagged
  }
}

/// GCSAFE_CODE_FWD_ACCESSOR 转发面 (21 法中的可落地支;
/// is_interpreter_trampoline_builtin / is_baseline_trampoline_builtin /
/// is_baseline_leave_frame_builtin 依赖 Builtins 枚举常量, 归 v8-init 席 — 台账)
pub trait GcSafeCodeExt {
  /// instruction_start 转发
  fn instruction_start(&self) -> usize;
  /// instruction_end 转发
  fn instruction_end(&self) -> usize;
  /// is_builtin 转发
  fn is_builtin(&self) -> bool;
  /// builtin_id 转发
  fn builtin_id(&self) -> i16;
  /// kind 转发
  fn kind(&self) -> Result<CodeKind>;
  /// has_instruction_stream 转发
  fn has_instruction_stream(&self) -> bool;
  /// is_maglevved 转发
  fn is_maglevved(&self) -> bool;
  /// is_turbofanned 转发
  fn is_turbofanned(&self) -> bool;
  /// has_tagged_outgoing_params 转发
  fn has_tagged_outgoing_params(&self) -> Result<bool>;
  /// marked_for_deoptimization 转发
  fn marked_for_deoptimization(&self) -> bool;
  /// raw_instruction_stream 转发
  fn raw_instruction_stream(&self) -> Tagged<Object>;
  /// stack_slots 转发
  fn stack_slots(&self) -> u32;
  /// parameter_count 转发
  fn parameter_count(&self) -> u16;
  /// parameter_count_without_receiver 转发
  fn parameter_count_without_receiver(&self) -> u16;
  /// wasm_js_tagged_parameter_count 转发
  fn wasm_js_tagged_parameter_count(&self) -> u16;
  /// wasm_js_first_tagged_parameter 转发
  fn wasm_js_first_tagged_parameter(&self) -> u16;
  /// constant_pool 转发
  fn constant_pool(&self) -> usize;
  /// safepoint_table_address 转发
  fn safepoint_table_address(&self) -> usize;
}

impl GcSafeCodeExt for GcSafeCode {
  #[inline]
  fn instruction_start(&self) -> usize {
    self.tagged.instruction_start()
  }
  #[inline]
  fn instruction_end(&self) -> usize {
    self.tagged.instruction_end()
  }
  #[inline]
  fn is_builtin(&self) -> bool {
    self.tagged.is_builtin()
  }
  #[inline]
  fn builtin_id(&self) -> i16 {
    self.tagged.builtin_id()
  }
  #[inline]
  fn kind(&self) -> Result<CodeKind> {
    self.tagged.kind()
  }
  #[inline]
  fn has_instruction_stream(&self) -> bool {
    self.tagged.has_instruction_stream()
  }
  #[inline]
  fn is_maglevved(&self) -> bool {
    self.tagged.is_maglevved()
  }
  #[inline]
  fn is_turbofanned(&self) -> bool {
    self.tagged.is_turbofanned()
  }
  #[inline]
  fn has_tagged_outgoing_params(&self) -> Result<bool> {
    self.tagged.has_tagged_outgoing_params()
  }
  #[inline]
  fn marked_for_deoptimization(&self) -> bool {
    self.tagged.marked_for_deoptimization()
  }
  #[inline]
  fn raw_instruction_stream(&self) -> Tagged<Object> {
    self.tagged.raw_instruction_stream()
  }
  #[inline]
  fn stack_slots(&self) -> u32 {
    self.tagged.stack_slots()
  }
  #[inline]
  fn parameter_count(&self) -> u16 {
    self.tagged.parameter_count()
  }
  #[inline]
  fn parameter_count_without_receiver(&self) -> u16 {
    self.tagged.parameter_count_without_receiver()
  }
  #[inline]
  fn wasm_js_tagged_parameter_count(&self) -> u16 {
    self.tagged.wasm_js_tagged_parameter_count()
  }
  #[inline]
  fn wasm_js_first_tagged_parameter(&self) -> u16 {
    self.tagged.wasm_js_first_tagged_parameter()
  }
  #[inline]
  fn constant_pool(&self) -> usize {
    self.tagged.constant_pool()
  }
  #[inline]
  fn safepoint_table_address(&self) -> usize {
    self.tagged.safepoint_table_address()
  }
}

// ===== CodeWrapper =====

/// CodeWrapper 对象 (原版 code.h:584 Struct + TrustedPointerMember<Code>)
///
/// 沙箱内 Struct 壳: map 头 + 受信指针槽 (基线沙箱关 = 全宽 tagged 槽)。
#[repr(C)]
pub struct CodeWrapper {
  /// Struct→HeapObject 头
  pub header: HeapObject,
  /// code_ 受信指针槽 (TrustedPointerMember<Code, kCodeIndirectPointerTag>
  /// 非沙箱形态 = TaggedMember<Code>)
  pub code: Tagged<Object>,
}

const _: () = assert!(size_of::<CodeWrapper>() == 16);
const _: () = assert!(offset_of!(CodeWrapper, code) == 8);

impl CodeWrapper {
  /// 构造 (原版 CodeWrapper(witness): map 由调用方显式传入, 偏差登记同 struct_ 先例)
  #[inline]
  pub const fn new(map: Tagged<Map>) -> Self {
    Self {
      header: HeapObject::new(map),
      code: Tagged::NULL,
    }
  }
}

/// CodeWrapper 五法 (原版 code-inl.h:941-956)
pub trait CodeWrapperExt: Copy {
  /// v8: src/objects/code-inl.h CodeWrapper::code (load = acquire)
  fn code(&self) -> Tagged<Code>;
  /// v8: src/objects/code-inl.h CodeWrapper::code(isolate, kAcquireLoad)
  fn code_acquire(&self) -> Tagged<Code>;
  /// v8: src/objects/code-inl.h CodeWrapper::set_code (store, release + 屏障桩)
  fn set_code(&self, value: Tagged<Code>);
  /// v8: src/objects/code-inl.h CodeWrapper::set_code(value, kReleaseStore, mode)
  fn set_code_release(&self, value: Tagged<Code>);
  /// v8: src/objects/code-inl.h CodeWrapper::has_code (!code_.is_empty())
  fn has_code(&self) -> bool;
  /// v8: src/objects/code-inl.h CodeWrapper::clear_code
  fn clear_code(&self);
}

impl CodeWrapperExt for Tagged<CodeWrapper> {
  #[inline]
  fn code(&self) -> Tagged<Code> {
    self.code_acquire()
  }

  #[inline]
  fn code_acquire(&self) -> Tagged<Code> {
    // SAFETY: 调用方契约 — Tagged<CodeWrapper> 指向有效对象 (TrustedPointerMember load 即 acquire)
    // code 槽为 repr(transparent) 全宽 Tagged<Object>, 取字段裸址转 AtomicUsize 视图
    unsafe {
      Tagged::<Object>::new(
        AtomicUsize::from_ptr(&raw const (*self.as_ptr()).code as *mut usize)
          .load(Ordering::Acquire),
      )
      .cast()
    }
  }

  #[inline]
  fn set_code(&self, value: Tagged<Code>) {
    // SAFETY: 契约同 code_acquire; store 即 release
    unsafe {
      AtomicUsize::from_ptr(&raw const (*self.as_ptr()).code as *mut usize)
        .store(value.ptr(), Ordering::Release);
    }
  }

  #[inline]
  fn set_code_release(&self, value: Tagged<Code>) {
    self.set_code(value);
  }

  #[inline]
  fn has_code(&self) -> bool {
    !self.code_acquire().is_null()
  }

  #[inline]
  fn clear_code(&self) {
    // SAFETY: 契约同 code_acquire
    unsafe {
      AtomicUsize::from_ptr(&raw const (*self.as_ptr()).code as *mut usize)
        .store(Tagged::<Object>::NULL.ptr(), Ordering::Release);
    }
  }
}

// ===== Buf 构造面 (脱离堆, 供对数与行为测试) =====

/// 拥有独立内存块的 Code 缓冲区 (先例: FeedbackVectorBuf)
pub struct CodeBuf {
  _storage: Vec<u64>,
  tagged: Tagged<Code>,
}

impl CodeBuf {
  /// 分配并初始化 88 字节 Code 对象
  pub fn new(map: Tagged<Map>) -> Self {
    let mut storage = vec![0u64; K_SIZE.div_ceil(8)];
    let base = storage.as_mut_ptr() as usize;
    let header = Code::new(map);
    // SAFETY: base 指向 storage 首地址, u64 对齐且容量即 K_SIZE
    unsafe {
      ptr::write(base as *mut Code, header);
    }
    Self {
      _storage: storage,
      // SAFETY: base 为有效 Code 内存
      tagged: unsafe { Tagged::from_raw_address(base) },
    }
  }

  /// 句柄
  #[inline]
  #[must_use]
  pub const fn tagged(&self) -> Tagged<Code> {
    self.tagged
  }
}

impl Deref for CodeBuf {
  type Target = Tagged<Code>;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.tagged
  }
}

/// 拥有独立内存块的 CodeWrapper 缓冲区
pub struct CodeWrapperBuf {
  _storage: Vec<u64>,
  tagged: Tagged<CodeWrapper>,
}

impl CodeWrapperBuf {
  /// 分配并初始化 CodeWrapper (code 槽空)
  pub fn new(map: Tagged<Map>) -> Self {
    let mut storage = vec![0u64; 2];
    let base = storage.as_mut_ptr() as usize;
    // SAFETY: base 指向 storage 首地址且容量 16 字节
    unsafe {
      ptr::write(base as *mut CodeWrapper, CodeWrapper::new(map));
    }
    Self {
      _storage: storage,
      // SAFETY: base 为有效 CodeWrapper 内存
      tagged: unsafe { Tagged::from_raw_address(base) },
    }
  }

  /// 句柄
  #[inline]
  #[must_use]
  pub const fn tagged(&self) -> Tagged<CodeWrapper> {
    self.tagged
  }
}

impl Deref for CodeWrapperBuf {
  type Target = Tagged<CodeWrapper>;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.tagged
  }
}
