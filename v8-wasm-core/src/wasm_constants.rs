//! v8/src/wasm/wasm-constants.h 的 1:1 转写
//! 登记差异:
//! - kOSRTargetOffset 仅 V8_TARGET_ARCH_X64 → 推迟 (arch 梯队)。
//! - kInvalidCanonicalIndex 的 Smi 往返 static_assert 依赖 Internals → 跳过
//!   (Rust 侧无 Smi 表示), 值本身 1:1。
//! - WasmCodePosition = i32 类型别名; enum 保持原判别值。
// 由 tool/src/wasm/wasm-constants.h.js 从 v8/src/wasm/wasm-constants.h 生成; 修订就地改, 复跑覆盖
// 常量与枚举表; 对数见脚本运行输出
// --- 模块头二进制编码 ---
pub const K_WASM_MAGIC: u32 = 0x6d736100; // "\0asm"
pub const K_WASM_VERSION: u32 = 0x01;

// --- 值/堆类型二进制编码 (wire 字节) ---
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValueTypeCode {
  Void = 0x40,
  I32 = 0x7f,
  I64 = 0x7e,
  F32 = 0x7d,
  F64 = 0x7c,
  S128 = 0x7b,
  I8 = 0x78,
  I16 = 0x77,
  F16 = 0x76,
  NoExn = 0x74,
  NoFunc = 0x73,
  NoExtern = 0x72,
  None = 0x71,
  FuncRef = 0x70,
  ExternRef = 0x6f,
  AnyRef = 0x6e,
  EqRef = 0x6d,
  I31Ref = 0x6c,
  StructRef = 0x6b,
  ArrayRef = 0x6a,
  Ref = 0x64,
  RefNull = 0x63,
  Exact = 0x62,
  ExnRef = 0x69,
  ContRef = 0x68,
  NoCont = 0x75,
  StringRef = 0x67,
  StringViewWtf8 = 0x66,
  StringViewWtf16 = 0x60,
  StringViewIter = 0x61,
  WaitqueueRef = 0x5c,
  NoWaitqueue = 0x5b,
}

impl ValueTypeCode {
  /// wire 字节 → 枚举 (未分配码 → None)
  pub const fn from_u8(v: u8) -> Option<Self> {
    match v {
      0x40 => Some(Self::Void),
      0x7f => Some(Self::I32),
      0x7e => Some(Self::I64),
      0x7d => Some(Self::F32),
      0x7c => Some(Self::F64),
      0x7b => Some(Self::S128),
      0x78 => Some(Self::I8),
      0x77 => Some(Self::I16),
      0x76 => Some(Self::F16),
      0x74 => Some(Self::NoExn),
      0x73 => Some(Self::NoFunc),
      0x72 => Some(Self::NoExtern),
      0x71 => Some(Self::None),
      0x70 => Some(Self::FuncRef),
      0x6f => Some(Self::ExternRef),
      0x6e => Some(Self::AnyRef),
      0x6d => Some(Self::EqRef),
      0x6c => Some(Self::I31Ref),
      0x6b => Some(Self::StructRef),
      0x6a => Some(Self::ArrayRef),
      0x64 => Some(Self::Ref),
      0x63 => Some(Self::RefNull),
      0x62 => Some(Self::Exact),
      0x69 => Some(Self::ExnRef),
      0x68 => Some(Self::ContRef),
      0x75 => Some(Self::NoCont),
      0x67 => Some(Self::StringRef),
      0x66 => Some(Self::StringViewWtf8),
      0x60 => Some(Self::StringViewWtf16),
      0x61 => Some(Self::StringViewIter),
      0x5c => Some(Self::WaitqueueRef),
      0x5b => Some(Self::NoWaitqueue),
      _ => None,
    }
  }
}

/// 解码用堆类型码界 (wasm-constants.h:66-67): [kFirstHeapTypeCode, kLastHeapTypeCode]
pub const FIRST_HEAP_TYPE_CODE: u8 = 0x5b; // kNoWaitqueueCode, 最低分配码
pub const LAST_HEAP_TYPE_CODE: u8 = 0x75; // kNoContCode, 最高分配码

// --- 类型定义二进制编码 ---
pub const K_SHARED_FLAG_CODE: u8 = 0x65;
pub const K_WASM_FUNCTION_TYPE_CODE: u8 = 0x60;
pub const K_WASM_STRUCT_TYPE_CODE: u8 = 0x5f;
pub const K_WASM_ARRAY_TYPE_CODE: u8 = 0x5e;
pub const K_WASM_CONT_TYPE_CODE: u8 = 0x5d;
pub const K_WASM_SUBTYPE_CODE: u8 = 0x50;
pub const K_WASM_SUBTYPE_FINAL_CODE: u8 = 0x4f;
pub const K_WASM_RECURSIVE_TYPE_GROUP_CODE: u8 = 0x4e;
pub const K_WASM_DESCRIPTOR_CODE: u8 = 0x4d;
pub const K_WASM_DESCRIBES_CODE: u8 = 0x4c;

// --- 导入/导出种类二进制编码 ---
pub const K_EXTERNAL_EXACT_BIT: u8 = 0x20;
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImportExportKindCode {
  ExternalFunction = 0,
  ExternalTable = 1,
  ExternalMemory = 2,
  ExternalGlobal = 3,
  ExternalTag = 4,
  ExternalExactFunction = 32,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompactImportCode {
  CompactImportByModule = 127,
  CompactImportByModuleAndType = 126,
}

// --- limits 结构 (内存与表 limits 通用) ---
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LimitsFlags {
  NoMaximum = 0,
  WithMaximum = 1,
  SharedNoMaximum = 2,
  SharedWithMaximum = 3,
  Memory64NoMaximum = 4,
  Memory64WithMaximum = 5,
  Memory64SharedNoMaximum = 6,
  Memory64SharedWithMaximum = 7,
}

// --- 数据/元素段标志 ---
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SegmentFlags {
  ActiveNoIndex = 0,
  Passive = 1,
  ActiveWithIndex = 2,
}

// --- 段标识符编码 ---
// 0-14 为已知段; 15+ 为自定义段 (以字符串标识, 枚举值不保证稳定)。
#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SectionCode {
  UnknownSection = 0,
  TypeSection = 1,
  ImportSection = 2,
  FunctionSection = 3,
  TableSection = 4,
  MemorySection = 5,
  GlobalSection = 6,
  ExportSection = 7,
  StartSection = 8,
  ElementSection = 9,
  CodeSection = 10,
  DataSection = 11,
  DataCountSection = 12,
  TagSection = 13,
  StringRefSection = 14,
  NameSection = 15,
  SourceMappingURLSection = 16,
  DebugInfoSection = 17,
  ExternalDebugInfoSection = 18,
  BuildIdSection = 19,
  InstTraceSection = 20,
  BranchHintsSection = 21,
  CompilationPrioritySection = 22,
  InstFrequenciesSection = 23,
  CallTargetsSection = 24,
  DescriptorsSection = 25,
}

impl SectionCode {
  /// wire 字节 → 枚举 (未定义段码 → None)
  pub const fn from_i8(v: i8) -> Option<Self> {
    match v {
      0 => Some(Self::UnknownSection),
      1 => Some(Self::TypeSection),
      2 => Some(Self::ImportSection),
      3 => Some(Self::FunctionSection),
      4 => Some(Self::TableSection),
      5 => Some(Self::MemorySection),
      6 => Some(Self::GlobalSection),
      7 => Some(Self::ExportSection),
      8 => Some(Self::StartSection),
      9 => Some(Self::ElementSection),
      10 => Some(Self::CodeSection),
      11 => Some(Self::DataSection),
      12 => Some(Self::DataCountSection),
      13 => Some(Self::TagSection),
      14 => Some(Self::StringRefSection),
      15 => Some(Self::NameSection),
      16 => Some(Self::SourceMappingURLSection),
      17 => Some(Self::DebugInfoSection),
      18 => Some(Self::ExternalDebugInfoSection),
      19 => Some(Self::BuildIdSection),
      20 => Some(Self::InstTraceSection),
      21 => Some(Self::BranchHintsSection),
      22 => Some(Self::CompilationPrioritySection),
      23 => Some(Self::InstFrequenciesSection),
      24 => Some(Self::CallTargetsSection),
      25 => Some(Self::DescriptorsSection),
      _ => None,
    }
  }
  /// kFirstSectionInModule = kTypeSectionCode
  pub const FIRST_SECTION_IN_MODULE: Self = Self::TypeSection;
  /// kLastKnownModuleSection = kStringRefSectionCode
  pub const LAST_KNOWN_MODULE_SECTION: Self = Self::StringRefSection;
  /// kFirstUnorderedSection = kDataCountSectionCode
  pub const FIRST_UNORDERED_SECTION: Self = Self::DataCountSection;
}

// --- 自定义段名 ---
pub const K_NAME_STRING: &str = "name";
pub const K_SOURCE_MAPPING_URL_STRING: &str = "sourceMappingURL";
pub const K_INST_TRACE_STRING: &str = "metadata.code.trace_inst";
pub const K_BRANCH_HINTS_STRING: &str = "metadata.code.branch_hint";
pub const K_COMPILATION_PRIORITY_STRING: &str = "metadata.code.compilation_priority";
pub const K_INSTRUCTION_FREQUENCIES_STRING: &str = "metadata.code.instr_freq";
pub const K_CALL_TARGETS_STRING: &str = "metadata.code.call_targets";
pub const K_DEBUG_INFO_STRING: &str = ".debug_info";
pub const K_EXTERNAL_DEBUG_INFO_STRING: &str = "external_debug_info";
pub const K_BUILD_ID_STRING: &str = "build_id";

// --- name section 子段种类编码 ---
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NameSectionKindCode {
  Module = 0,
  Function = 1,
  Local = 2,
  Label = 3,
  Type = 4,
  Table = 5,
  Memory = 6,
  Global = 7,
  ElementSegment = 8,
  DataSegment = 9,
  Field = 10,
  Tag = 11,
}

// --- catch 子句种类 ---
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CatchKind {
  Catch = 0,
  CatchRef = 1,
  CatchAll = 2,
  CatchAllRef = 3,
}

impl CatchKind {
  /// kLastCatchKind = kCatchAllRef
  pub const LAST: Self = Self::CatchAllRef;
}

// --- switch 子句种类 ---
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SwitchKind {
  OnSuspend = 0,
  Switch = 1,
}

impl SwitchKind {
  /// kLastSwitchKind = kSwitch
  pub const LAST: Self = Self::Switch;
}

// --- 原子内存序 ---
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemoryOrdering {
  AtomicSeqCst = 0,
  AtomicAcqRel = 1,
}

// --- 页大小 ---
pub const WASM_PAGE_SIZE: usize = 65536; // 0x10000
pub const WASM_PAGE_SIZE_LOG2: u32 = 16;
const _: () = assert!(WASM_PAGE_SIZE == 1usize << WASM_PAGE_SIZE_LOG2);

/// 代码位置 (wasm 二进制内字节偏移); TODO(wasm): 原版注释保留 — 待包装为结构
pub type WasmCodePosition = i32;
pub const K_NO_CODE_POSITION: WasmCodePosition = -1;

pub const K_EXCEPTION_ATTRIBUTE: u32 = 0;
pub const K_ANONYMOUS_FUNC_INDEX: i32 = -1;
pub const K_INVALID_CANONICAL_INDEX: u32 = 4294967295;
pub const K_GENERIC_WRAPPER_BUDGET: u32 = 1000;
pub const K_MINIMUM_SUPERTYPE_ARRAY_SIZE: u32 = 3;
pub const K_MAX_POLYMORPHISM: i32 = 4;
pub const K_MAX_STRUCT_FIELD_INDEX_FOR_IMPLICIT_NULL_CHECK: i32 = 4000;
pub const K_OPTIMIZATION_PRIORITY_EXECUTED_ONCE_SENTINEL: i32 = 127;
pub const K_OPTIMIZATION_PRIORITY_NOT_SPECIFIED_SENTINEL: i32 = -1;
pub const K_INT31_MAX_VALUE: i32 = 1073741823;
pub const K_INT31_MIN_VALUE: i32 = -1073741824;

// --- CWasmEntry 参数槽位 (kNumParameters 为计数标记) ---
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CWasmEntryParameters {
  CodeEntry = 0,
  ObjectRef = 1,
  ArgumentsBuffer = 2,
  CEntryFp = 3,
  NumParameters = 4,
}
