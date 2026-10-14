// 由 tool/src/common.js 从 v8/src/common/globals.h 生成; 修订就地改, 复跑覆盖
//
// 修订 (Developer): 枚举变体 Rust 化 (kFoo → Foo, ALL_CAPS → CamelCase),
// 补 HeapObjectReferenceType / ShouldThrow / StubCallMode 三个缺失枚举,
// 修复 ExternalArrayType 判别值 / 非法 #[repr] / 重名 Type·TypeIndex,
// 236 条常量清单按目标配置落成真常量, 配置布尔移交 globals::config。
//
// 修订 (审查环 R1): kElidedFrameSlots 补非 x64/ia32 归零分支 (globals.h:466-470);
// 文末「助记函数/小类型补全」段 — 语言模式谓词/construct_repl_mode、flushing
// 4 函数 (EnumSet 替身)、ToString 族 as_str、KeyedAccess 判定、ObjectHash、
// 反馈编码计算 (Decode/Combine/TransitionMap/EncodeTable)、GCEpoch、
// AllocationHint、IeeeDoubleArchType、WasmCodePointer 表常量、Int31、
// StackFrameId 常量、对齐宏族、原子访问 tag。
//
// 修订 (审查环 R2): IeeeDoubleArchType 改 u64 载体 + to_bits/from_bits
// (整体去 unsafe); 补 11 族 operator<< as_str 打印串 (差分日志面对齐原版,
// 见「operator<< 打印串族」段) 与 StaticStringsEqual (globals.h:821-826)。
//
// 修订 (审查环 round1): 补遗漏 — TrustedPointerT 别名 (globals.h:710-713)、
// DefineKeyedOwnPropertyFlags / InLiteralFlags 位集替身 (globals.h:2904-2913,
// 判别值本身即位值); 裁剪登记增补 AsAtomicTagged / RuntimeArguments 族 /
// hash_value(LanguageMode) (见文末登记表)。

use core::mem::transmute_copy;
pub(crate) mod config;

use core::sync::atomic::AtomicU32;

use crate::internal::{
  Address, GB, K_API_TAGGED_SIZE, K_CPP_HEAP_POINTER_HANDLE_SIZE, K_DONT_THROW,
  K_EXTERNAL_ALLOCATION_SOFT_LIMIT as K_EXTERNAL_ALLOCATION_SOFT_LIMIT_INTERNAL,
  K_EXTERNAL_POINTER_HANDLE_SIZE, K_GARBAGE_COLLECTION_REASON_MAX_VALUE, K_HEAP_OBJECT_TAG,
  K_INDIRECT_POINTER_HANDLE_SIZE, K_PAGE_SIZE_BITS, K_SMI_SHIFT_SIZE, K_SMI_TAG_SIZE,
  K_SMI_VALUE_SIZE, K_THROW_ON_ERROR, K_WASM_CODE_POINTER_TABLE_RESERVATION_SIZE, KB, MB,
  SMI_VALUES_ARE_31_BITS, SMI_VALUES_ARE_32_BITS,
};

// ── 配置布尔 ────────────────────────────────────────────────────────────────
// 全部展开条目在 config 模块 (含原版 globals.h 行号依据), 此处不再重复表。

// ── 底层别名 (globals.h:585-586, 612-613) ─────────────────────────────────
/// 原版 Tagged_t — 压缩指针槽, COMPRESS_POINTERS 分支
pub type TaggedT = u32;
/// 原版 AtomicTagged_t (base::Atomic32) — 原子压缩指针槽
pub type AtomicTaggedT = AtomicU32;
/// 原版 JSDispatchHandle (base::StrongAlias<uint32_t>) — Rust 直取底层类型
pub type JsDispatchHandle = u32;
/// 原版 AdaptArguments (base::StrongAlias<bool>)
pub type AdaptArguments = bool;
/// 原版 NeedsContext (base::StrongAlias<bool>)
pub type NeedsContext = bool;
/// 原版 StackFrameId (globals.h:3253) — int 型 ID 空间, 转 i32 (登记偏差)
pub type StackFrameId = i32;

// ── 枚举 ───────────────────────────────────────────────────────────────────
// 变体命名登记: C++ kFoo → Rust Foo; C++ ALL_CAPS → Rust CamelCase。
// 判别值与原版 1:1, 未标注者按声明顺序 0..n。

/// 原版 globals.h:864 enum class LanguageMode : bool
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum LanguageMode {
  Sloppy = 0,
  Strict = 1,
}

/// 原版 LanguageMode2String (globals.h:871)
impl LanguageMode {
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Sloppy => "sloppy",
      Self::Strict => "strict",
    }
  }
}

/// 原版 globals.h:865 LanguageModeSize
pub const LANGUAGE_MODE_SIZE: usize = 2;

/// 原版 globals.h:913 enum StoreOrigin
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum StoreOrigin {
  MaybeKeyed,
  Named,
}

/// 原版 globals.h:915 enum TypeofMode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum TypeofMode {
  Inside,
  NotInside,
}

/// 原版 globals.h:917 enum ContextMode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum ContextMode {
  NoContextCells,
  HasContextCells,
}

/// 原版 globals.h:920 enum SaveFPRegsMode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum SaveFPRegsMode {
  Ignore,
  Save,
}

/// 原版 globals.h:923 enum IndirectPointerMode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum IndirectPointerMode {
  Strong,
  Custom,
}

/// 原版 globals.h:936 enum ArgvMode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum ArgvMode {
  Stack,
  Register,
}

/// 原版 globals.h:938 enum CallApiCallbackMode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum CallApiCallbackMode {
  Generic,
  OptimizedNoProfiling,
  Optimized,
}

/// 原版 globals.h:984 enum ExtendedMapKind : uint8_t
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum ExtendedMapKind {
  JSInterceptorMap,
}

/// 原版 globals.h:1019 enum DeoptimizeKind : uint8_t
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum DeoptimizeKind {
  Eager,
  LazyAfterFastCall,
  Lazy,
}

/// 原版 kFirstDeoptimizeKind / kLastDeoptimizeKind / kDeoptimizeKindCount
/// (globals.h:1024-1027)
pub const FIRST_DEOPTIMIZE_KIND: DeoptimizeKind = DeoptimizeKind::Eager;
pub const LAST_DEOPTIMIZE_KIND: DeoptimizeKind = DeoptimizeKind::Lazy;
pub const DEOPTIMIZE_KIND_COUNT: i32 = LAST_DEOPTIMIZE_KIND as i32 + 1;

/// 原版 ToString(DeoptimizeKind) (globals.h:1031)
impl DeoptimizeKind {
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Eager => "Eager",
      Self::LazyAfterFastCall => "LazyAfterfastCall",
      Self::Lazy => "Lazy",
    }
  }
}

/// 原版 globals.h:1048 enum LookupHoistingMode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum LookupHoistingMode {
  Normal,
  LegacySloppy,
}

/// 原版 globals.h:1197 enum HeapObjectReferenceType (脚本跳过, 修订补全)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum HeapObjectReferenceType {
  Weak,
  Strong,
}

/// 原版 globals.h:1202 enum ArgumentsType
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum ArgumentsType {
  Runtime,
  JS,
}

/// 原版 globals.h:1488 enum AllocationSpace
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum AllocationSpace {
  RoSpace = 0,
  NewSpace = 1,
  OldSpace = 2,
  CodeSpace = 3,
  SharedSpace = 4,
  TrustedSpace = 5,
  SharedTrustedSpace = 6,
  NewLoSpace = 7,
  LoSpace = 8,
  CodeLoSpace = 9,
  SharedLoSpace = 10,
  SharedTrustedLoSpace = 11,
  TrustedLoSpace = 12,
}

/// 原版 FIRST_SPACE..LAST_SWEEPABLE_SPACE (globals.h:1506-1513)
pub const FIRST_SPACE: AllocationSpace = AllocationSpace::RoSpace;
pub const LAST_SPACE: AllocationSpace = AllocationSpace::TrustedLoSpace;
pub const FIRST_MUTABLE_SPACE: AllocationSpace = AllocationSpace::NewSpace;
pub const LAST_MUTABLE_SPACE: AllocationSpace = AllocationSpace::TrustedLoSpace;
pub const FIRST_GROWABLE_PAGED_SPACE: AllocationSpace = AllocationSpace::OldSpace;
pub const LAST_GROWABLE_PAGED_SPACE: AllocationSpace = AllocationSpace::SharedTrustedSpace;
pub const FIRST_SWEEPABLE_SPACE: AllocationSpace = AllocationSpace::NewSpace;
pub const LAST_SWEEPABLE_SPACE: AllocationSpace = AllocationSpace::SharedTrustedSpace;

/// 原版 globals.h:1515 kSpaceTagSize
pub const K_SPACE_TAG_SIZE: i32 = 4;
// 原版 static_assert FIRST_SPACE == 0 (globals.h:1516)
const _: () = assert!(FIRST_SPACE as u8 == 0);

/// 原版 ToString(AllocationSpace) (globals.h:1536)
impl AllocationSpace {
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::RoSpace => "read_only_space",
      Self::NewSpace => "new_space",
      Self::OldSpace => "old_space",
      Self::CodeSpace => "code_space",
      Self::SharedSpace => "shared_space",
      Self::TrustedSpace => "trusted_space",
      Self::SharedTrustedSpace => "shared_trusted_space",
      Self::NewLoSpace => "new_large_object_space",
      Self::LoSpace => "large_object_space",
      Self::CodeLoSpace => "code_large_object_space",
      Self::SharedLoSpace => "shared_large_object_space",
      Self::SharedTrustedLoSpace => "shared_trusted_large_object_space",
      Self::TrustedLoSpace => "trusted_large_object_space",
    }
  }
}

/// 原版 IsAnyCodeSpace (globals.h:1518)
pub const fn is_any_code_space(space: AllocationSpace) -> bool {
  let s = space as u8;
  s == AllocationSpace::CodeSpace as u8 || s == AllocationSpace::CodeLoSpace as u8
}

/// 原版 IsAnyTrustedSpace (globals.h:1521)
pub const fn is_any_trusted_space(space: AllocationSpace) -> bool {
  let s = space as u8;
  s == AllocationSpace::TrustedSpace as u8
    || s == AllocationSpace::TrustedLoSpace as u8
    || s == AllocationSpace::SharedTrustedSpace as u8
    || s == AllocationSpace::SharedTrustedLoSpace as u8
}

/// 原版 IsAnyWritableSharedSpace (globals.h:1525)
pub const fn is_any_writable_shared_space(space: AllocationSpace) -> bool {
  let s = space as u8;
  s == AllocationSpace::SharedSpace as u8
    || s == AllocationSpace::SharedLoSpace as u8
    || s == AllocationSpace::SharedTrustedSpace as u8
    || s == AllocationSpace::SharedTrustedLoSpace as u8
}

/// 原版 IsAnyNewSpace (globals.h:1529)
pub const fn is_any_new_space(space: AllocationSpace) -> bool {
  let s = space as u8;
  s == AllocationSpace::NewSpace as u8 || s == AllocationSpace::NewLoSpace as u8
}

/// 原版 IsAnyLargeSpace (globals.h:1532)
pub const fn is_any_large_space(space: AllocationSpace) -> bool {
  let s = space as u8;
  s == AllocationSpace::NewLoSpace as u8
    || s == AllocationSpace::LoSpace as u8
    || s == AllocationSpace::CodeLoSpace as u8
    || s == AllocationSpace::SharedLoSpace as u8
    || s == AllocationSpace::TrustedLoSpace as u8
}

/// 原版 globals.h:1573 enum AllocationType : uint8_t
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum AllocationType {
  Young,
  Old,
  Code,
  Map,
  ReadOnly,
  SharedOld,
  SharedMap,
  SharedTrusted,
  Trusted,
}

/// 原版 globals.h:1620 enum RequestedGCKind : uint8_t
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum RequestedGCKind {
  Major = 1,
  LastResort = 1 << 1,
}

/// 原版 globals.h:1643 enum GarbageCollectionReason : int
/// NUM_REASONS 为计数哨兵, 按原版保留为变体 (登记)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum GarbageCollectionReason {
  Unknown = 0,
  AllocationFailure = 1,
  AllocationLimit = 2,
  ContextDisposal = 3,
  CountersExtension = 4,
  Debugger = 5,
  Deserializer = 6,
  ExternalMemoryPressure = 7,
  FinalizeMarkingViaStackGuard = 8,
  FinalizeMarkingViaTask = 9,
  FullHashtable = 10,
  HeapProfiler = 11,
  Task = 12,
  LastResort = 13,
  LowMemoryNotification = 14,
  MakeHeapIterable = 15,
  MemoryPressure = 16,
  MemoryReducer = 17,
  Runtime = 18,
  SamplingProfiler = 19,
  SnapshotCreator = 20,
  Testing = 21,
  ExternalFinalize = 22,
  GlobalAllocationLimit = 23,
  MeasureMemory = 24,
  BackgroundAllocationFailure = 25,
  FinalizeConcurrentMinorMS = 26,
  CppHeapAllocationFailure = 27,
  Frozen = 28,
  IdleContextDisposal = 29,
  FinalizeMinorMSForMajorGC = 30,
  NumReasons = 31,
}

// 原版 static_assert kGarbageCollectionReasonMaxValue == NUM_REASONS - 1
// (globals.h:1696, 依据 include/v8-internal.h:1598)
const _: () =
  assert!(K_GARBAGE_COLLECTION_REASON_MAX_VALUE == GarbageCollectionReason::NumReasons as i32 - 1);

/// 原版 globals.h:1758 enum HeapGrowingMode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum HeapGrowingMode {
  Slow,
  Conservative,
  Minimal,
  Default,
}

/// 原版 globals.h:1770 enum RecordYoungSlot : bool
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum RecordYoungSlot {
  No = 0,
  Yes = 1,
}

/// 原版 globals.h:1775 enum AllocationAlignment : uint8_t
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum AllocationAlignment {
  TaggedAligned,
  DoubleAligned,
  DoubleUnaligned,
}

/// 原版 globals.h:1795 enum AccessMode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum AccessMode {
  Atomic,
  NonAtomic,
}

/// 原版 globals.h:1797 enum TypedArrayAccessMode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum TypedArrayAccessMode {
  Read,
  Write,
}

/// 原版 globals.h:1808 enum MinimumCapacity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum MinimumCapacity {
  UseDefaultMinimumCapacity,
  UseCustomMinimumCapacity,
}

/// 原版 globals.h:1813 enum GarbageCollector
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum GarbageCollector {
  Scavenger,
  MarkCompactor,
  MinorMarkSweeper,
}

/// 原版 globals.h:1831 enum CompactionSpaceKind
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum CompactionSpaceKind {
  None,
  CompactionSpaceForScavenge,
  CompactionSpaceForMarkCompact,
  CompactionSpaceForMinorMarkSweep,
}

/// 原版 globals.h:1838 enum Executability
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum Executability {
  NotExecutable,
  Executable,
}

/// 原版 globals.h:1840 enum PageSize
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum PageSize {
  Regular,
  Large,
}

/// 原版 globals.h:1842 enum CodeFlushMode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum CodeFlushMode {
  FlushBytecode,
  FlushBaselineCode,
  ForceFlush,
}

/// 原版 globals.h:1848 enum NewJSObjectType : uint8_t
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum NewJSObjectType {
  MaybeEmbedderFieldsAndNoApiWrapper,
  NoEmbedderFieldsAndNoApiWrapper,
  MaybeEmbedderFieldsAndApiWrapper,
}

/// 原版 globals.h:1876 enum REPLMode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum REPLMode {
  Yes,
  No,
}

/// 原版 globals.h:1886 enum ParsingWhileDebugging
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum ParsingWhileDebugging {
  Yes,
  No,
}

/// 原版 globals.h:1892 enum NativesFlag
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum NativesFlag {
  NotNativesCode,
  ExtensionCode,
  InspectorCode,
}

/// 原版 globals.h:1896 enum ParseRestriction : bool
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum ParseRestriction {
  NoParseRestriction = 0,
  OnlySingleFunctionLiteral = 1,
}

/// 原版 globals.h:1901 enum ScriptEventType
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum ScriptEventType {
  ReserveId,
  Create,
  Deserialize,
  BackgroundCompile,
  StreamingCompileBackground,
  StreamingCompileForeground,
}

/// 原版 globals.h:1911 enum InlineCacheState
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum InlineCacheState {
  NoFeedback,
  Uninitialized,
  Monomorphic,
  RecomputeHandler,
  Polymorphic,
  Megadom,
  Homomorphic,
  Megamorphic,
  Generic,
}

/// 原版 globals.h:1961 enum WhereToStart
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum WhereToStart {
  StartAtReceiver,
  StartAtPrototype,
}

/// 原版 globals.h:1963 enum ResultSentinel
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum ResultSentinel {
  NotFound = -1,
  Unsupported = -2,
}

/// 原版 globals.h:1965 enum ShouldThrow — 依赖 Internals::kDontThrow/kThrowOnError,
/// 值由 internal 模块提供 (修订补全)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum ShouldThrow {
  DontThrow = K_DONT_THROW,
  ThrowOnError = K_THROW_ON_ERROR,
}

/// 原版 globals.h:1970 enum InterceptorKind
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum InterceptorKind {
  Named,
  Indexed,
}

/// 原版 globals.h:1974 enum InterceptorResult
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum InterceptorResult {
  False = 0,
  True = 1,
  NotIntercepted = 2,
}

/// 原版 globals.h:1980 enum ThreadKind
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum ThreadKind {
  Main,
  Background,
}

/// 原版 globals.h:2081 enum BranchHint : uint8_t
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum BranchHint {
  None,
  True,
  False,
}

/// 原版 globals.h:2084 enum GotoHint : uint8_t
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum GotoHint {
  None,
  Label,
  Fallthrough,
}

/// 原版 globals.h:2091 enum ConvertReceiverMode : unsigned
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u32)]
pub enum ConvertReceiverMode {
  NullOrUndefined = 0,
  NotNullOrUndefined = 1,
  Any = 2,
}

/// 原版 globals.h:2115 kLast (ConvertReceiverMode 域内)
pub const CONVERT_RECEIVER_MODE_LAST: ConvertReceiverMode = ConvertReceiverMode::Any;

/// 原版 globals.h:2117 enum OrdinaryToPrimitiveHint
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum OrdinaryToPrimitiveHint {
  Number,
  String,
}

/// 原版 globals.h:2121 enum ToPrimitiveHint
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum ToPrimitiveHint {
  Default,
  Number,
  String,
}

/// 原版 globals.h:2124 enum CreateArgumentsType : uint8_t
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum CreateArgumentsType {
  MappedArguments,
  UnmappedArguments,
  RestParameter,
}

/// 原版 globals.h:2151 enum ScopeType : uint8_t
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum ScopeType {
  ScriptScope,
  ReplModeScope,
  EvalScope,
  ModuleScope,
  ClassScope,
  FunctionScope,
  CatchScope,
  BlockScope,
  WithScope,
  ShadowRealmScope,
}

/// 原版 globals.h:2168 enum AllocationSiteMode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum AllocationSiteMode {
  DontTrackAllocationSite = 0,
  TrackAllocationSite = 1,
}

/// 原版 globals.h:2171 LAST_ALLOCATION_SITE_MODE
pub const LAST_ALLOCATION_SITE_MODE: AllocationSiteMode = AllocationSiteMode::TrackAllocationSite;

/// 原版 globals.h:2174 enum AllocationSiteUpdateMode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum AllocationSiteUpdateMode {
  Update,
  CheckOnly,
}

/// 原版 globals.h:2238 enum VariableMode : uint8_t
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum VariableMode {
  Let = 0,
  Const = 1,
  Using = 2,
  AwaitUsing = 3,
  Var = 4,
  Temporary = 5,
  Dynamic = 6,
  DynamicGlobal = 7,
  DynamicLocal = 8,
  PrivateMethod = 9,
  PrivateSetterOnly = 10,
  PrivateGetterOnly = 11,
  PrivateGetterAndSetter = 12,
}

/// 原版 kFirstImmutableLexicalVariableMode / kLastLexicalVariableMode
/// (globals.h:2304, 2312)
pub const FIRST_IMMUTABLE_LEXICAL_VARIABLE_MODE: VariableMode = VariableMode::Const;
pub const LAST_LEXICAL_VARIABLE_MODE: VariableMode = VariableMode::AwaitUsing;

impl VariableMode {
  /// 原版 IsDynamicVariableMode (globals.h:2351)
  pub const fn is_dynamic(self) -> bool {
    let v = self as u8;
    v >= Self::Dynamic as u8 && v <= Self::DynamicLocal as u8
  }

  /// 原版 IsDeclaredVariableMode (globals.h:2355)
  pub const fn is_declared(self) -> bool {
    self as u8 <= Self::Var as u8
  }

  /// 原版 IsPrivateAccessorVariableMode (globals.h:2366)
  pub const fn is_private_accessor(self) -> bool {
    let v = self as u8;
    v >= Self::PrivateSetterOnly as u8 && v <= Self::PrivateGetterAndSetter as u8
  }

  /// 原版 IsPrivateMethodVariableMode (globals.h:2370)
  pub const fn is_private_method(self) -> bool {
    self as u8 == Self::PrivateMethod as u8
  }

  /// 原版 IsPrivateMethodOrAccessorVariableMode (globals.h:2374)
  pub const fn is_private_method_or_accessor(self) -> bool {
    self.is_private_method() || self.is_private_accessor()
  }

  /// 原版 IsSerializableVariableMode (globals.h:2379)
  pub const fn is_serializable(self) -> bool {
    self.is_declared() || self.is_private_method_or_accessor()
  }

  /// 原版 IsImmutableLexicalVariableMode (globals.h:2383)
  pub const fn is_immutable_lexical(self) -> bool {
    let v = self as u8;
    v >= FIRST_IMMUTABLE_LEXICAL_VARIABLE_MODE as u8 && v <= LAST_LEXICAL_VARIABLE_MODE as u8
  }

  /// 原版 IsImmutableLexicalOrPrivateVariableMode (globals.h:2388)
  pub const fn is_immutable_lexical_or_private(self) -> bool {
    self.is_immutable_lexical() || self.is_private_method_or_accessor()
  }

  /// 原版 IsLexicalVariableMode (globals.h:2392)
  pub const fn is_lexical(self) -> bool {
    self as u8 <= LAST_LEXICAL_VARIABLE_MODE as u8
  }

  /// 原版 IsResourceManagedVariableMode (globals.h:2397)
  pub const fn is_resource_managed(self) -> bool {
    let v = self as u8;
    v == Self::Using as u8 || v == Self::AwaitUsing as u8
  }
}

// 原版 static_assert (uint8_t)VariableMode::kLet == 0 (globals.h:2357, 2394)
const _: () = assert!(VariableMode::Let as u8 == 0);

/// 原版 globals.h:2339 enum VariableKind : uint8_t
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum VariableKind {
  NormalVariable,
  ParameterVariable,
  ThisVariable,
  SloppyBlockFunctionVariable,
  SloppyFunctionNameVariable,
}

/// 原版 globals.h:2396 enum VariableLocation : uint8_t
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum VariableLocation {
  Unallocated = 0,
  Parameter = 1,
  Local = 2,
  Context = 3,
  Lookup = 4,
  Module = 5,
  ReplGlobal = 6,
}

/// 原版 kLastVariableLocation (globals.h:2450 附近)
pub const LAST_VARIABLE_LOCATION: VariableLocation = VariableLocation::ReplGlobal;

/// 原版 globals.h:2460 enum InitializationFlag : uint8_t
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum InitializationFlag {
  NeedsInitialization,
  CreatedInitialized,
}

/// 原版 globals.h:2464 enum IsStaticFlag : uint8_t
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum IsStaticFlag {
  NotStatic,
  Static,
}

/// 原版 globals.h:2466 enum MaybeAssignedFlag : uint8_t
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum MaybeAssignedFlag {
  NotAssigned,
  MaybeAssigned,
}

/// 原版 globals.h:2468 enum InterpreterPushArgsMode : unsigned
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u32)]
pub enum InterpreterPushArgsMode {
  ArrayFunction,
  WithFinalSpread,
  Other,
}

/// 原版 BinaryOperationFeedback::Type (globals.h:2528-2541)
/// C++ 嵌套枚举展平, 类名并入枚举名 (登记偏差)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u16)]
pub enum BinaryOperationFeedbackType {
  None = 0x0,
  SignedSmall = 0x1,
  SignedSmallInputs = 0x3,
  AdditiveSafeInteger = 0x7,
  Number = 0xF,
  NumberOrOddball = 0x1F,
  BigInt64 = 0x20,
  BigInt = 0x60,
  String = 0x80,
  StringWrapper = 0x100,
  StringOrStringWrapper = 0x180,
  Any = 0x1FF,
}

/// 原版 BinaryOperationFeedback::TypeIndex (globals.h:2544-2559)
/// BINARY_OPERATION_FEEDBACK_TYPES X-macro 展开, 顺序 = 列表顺序
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum BinaryOperationFeedbackTypeIndex {
  None,
  SignedSmall,
  SignedSmallInputs,
  AdditiveSafeInteger,
  Number,
  NumberOrOddball,
  BigInt64,
  BigInt,
  String,
  StringWrapper,
  StringOrStringWrapper,
  Any,
}

/// 原版 kFirstTypeIndex / kLastTypeIndex (globals.h:2556-2558)
pub const BINARY_OP_FIRST_TYPE_INDEX: BinaryOperationFeedbackTypeIndex =
  BinaryOperationFeedbackTypeIndex::None;
pub const BINARY_OP_LAST_TYPE_INDEX: BinaryOperationFeedbackTypeIndex =
  BinaryOperationFeedbackTypeIndex::Any;
/// 原版 kNumTypeIndices (globals.h:2563)
pub const BINARY_OP_NUM_TYPE_INDICES: u32 = BINARY_OP_LAST_TYPE_INDEX as u32 + 1;
/// 原版 kTransitionMapStride — round up to 2^x (globals.h:2565, bit_ceil(12) = 16)
pub const BINARY_OP_TRANSITION_MAP_STRIDE: u32 = BINARY_OP_NUM_TYPE_INDICES.next_power_of_two();

/// 原版 CompareOperationFeedback 匿名枚举位标志 (globals.h:2659-2671)
pub const COMPARE_OP_SIGNED_SMALL_FLAG: u16 = 1 << 0;
pub const COMPARE_OP_OTHER_NUMBER_FLAG: u16 = 1 << 1;
pub const COMPARE_OP_BOOLEAN_FLAG: u16 = 1 << 2;
pub const COMPARE_OP_NULL_OR_UNDEFINED_FLAG: u16 = 1 << 3;
pub const COMPARE_OP_INTERNALIZED_STRING_FLAG: u16 = 1 << 4;
pub const COMPARE_OP_OTHER_STRING_FLAG: u16 = 1 << 5;
pub const COMPARE_OP_SYMBOL_FLAG: u16 = 1 << 6;
pub const COMPARE_OP_BIG_INT64_FLAG: u16 = 1 << 7;
pub const COMPARE_OP_OTHER_BIG_INT_FLAG: u16 = 1 << 8;
pub const COMPARE_OP_RECEIVER_FLAG: u16 = 1 << 9;
pub const COMPARE_OP_ANY_MASK: u16 = 0x3FF;

/// 原版 CompareOperationFeedback::Type (globals.h:2674-2698)
/// C++ 判别式引用位标志, 此处展开为等值字面量 (登记换算)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u16)]
pub enum CompareOperationFeedbackType {
  None = 0,
  Boolean = COMPARE_OP_BOOLEAN_FLAG,
  NullOrUndefined = COMPARE_OP_NULL_OR_UNDEFINED_FLAG,
  Oddball = COMPARE_OP_BOOLEAN_FLAG | COMPARE_OP_NULL_OR_UNDEFINED_FLAG,
  SignedSmall = COMPARE_OP_SIGNED_SMALL_FLAG,
  Number = COMPARE_OP_SIGNED_SMALL_FLAG | COMPARE_OP_OTHER_NUMBER_FLAG,
  NumberOrBoolean =
    COMPARE_OP_SIGNED_SMALL_FLAG | COMPARE_OP_OTHER_NUMBER_FLAG | COMPARE_OP_BOOLEAN_FLAG,
  NumberOrOddball = COMPARE_OP_SIGNED_SMALL_FLAG
    | COMPARE_OP_OTHER_NUMBER_FLAG
    | COMPARE_OP_BOOLEAN_FLAG
    | COMPARE_OP_NULL_OR_UNDEFINED_FLAG,
  InternalizedString = COMPARE_OP_INTERNALIZED_STRING_FLAG,
  String = COMPARE_OP_INTERNALIZED_STRING_FLAG | COMPARE_OP_OTHER_STRING_FLAG,
  StringOrOddball = COMPARE_OP_INTERNALIZED_STRING_FLAG
    | COMPARE_OP_OTHER_STRING_FLAG
    | COMPARE_OP_BOOLEAN_FLAG
    | COMPARE_OP_NULL_OR_UNDEFINED_FLAG,
  Receiver = COMPARE_OP_RECEIVER_FLAG,
  ReceiverOrNullOrUndefined = COMPARE_OP_RECEIVER_FLAG | COMPARE_OP_NULL_OR_UNDEFINED_FLAG,
  BigInt64 = COMPARE_OP_BIG_INT64_FLAG,
  BigInt = COMPARE_OP_BIG_INT64_FLAG | COMPARE_OP_OTHER_BIG_INT_FLAG,
  Symbol = COMPARE_OP_SYMBOL_FLAG,
  Any = COMPARE_OP_ANY_MASK,
}

/// 原版 CompareOperationFeedback::TypeIndex (globals.h:2701-2712)
/// COMPARE_OPERATION_FEEDBACK_TYPES X-macro 展开
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum CompareOperationFeedbackTypeIndex {
  None,
  Boolean,
  NullOrUndefined,
  Oddball,
  SignedSmall,
  Number,
  NumberOrBoolean,
  NumberOrOddball,
  InternalizedString,
  String,
  StringOrOddball,
  Receiver,
  ReceiverOrNullOrUndefined,
  BigInt64,
  BigInt,
  Symbol,
  Any,
}

/// 原版 kFirstTypeIndex / kLastTypeIndex / kNumTypeIndices / kTransitionMapStride
/// (globals.h:2709-2724, bit_ceil(17) = 32)
pub const COMPARE_OP_FIRST_TYPE_INDEX: CompareOperationFeedbackTypeIndex =
  CompareOperationFeedbackTypeIndex::None;
pub const COMPARE_OP_LAST_TYPE_INDEX: CompareOperationFeedbackTypeIndex =
  CompareOperationFeedbackTypeIndex::Any;
pub const COMPARE_OP_NUM_TYPE_INDICES: u32 = COMPARE_OP_LAST_TYPE_INDEX as u32 + 1;
pub const COMPARE_OP_TRANSITION_MAP_STRIDE: u32 = COMPARE_OP_NUM_TYPE_INDICES.next_power_of_two();

/// 原版 TypeOfFeedback::Result (globals.h:2795-2803)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum TypeOfFeedbackResult {
  None = 0,
  Smi = 1,
  HeapNumber = 1 << 1,
  Function = 1 << 2,
  String = 1 << 3,
  Number = 3,
  Any = 15,
}

/// 原版 globals.h:2812 enum ForInFeedback : uint8_t
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum ForInFeedback {
  None = 0x0,
  EnumCacheKeysAndIndices = 0x1,
  EnumCacheKeys = 0x3,
  EnumeratorHolder = 0x6,
  Any = 0x7,
}

/// 原版 globals.h:2841 enum UnicodeEncoding : uint8_t
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum UnicodeEncoding {
  UTF16,
  UTF32,
}

/// 原版 globals.h:2861 enum IterationKind
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum IterationKind {
  Keys,
  Values,
  Entries,
}

/// 原版 globals.h:2875 enum CollectionKind
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum CollectionKind {
  Map,
  Set,
}

/// 原版 globals.h:2887 enum IsolateExecutionModeFlag : uint8_t
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum IsolateExecutionModeFlag {
  NoFlags = 0,
  IsProfiling = 1 << 0,
  CheckSideEffects = 1 << 1,
}

/// 原版 globals.h:2900 enum DefineKeyedOwnPropertyInLiteralFlag
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum DefineKeyedOwnPropertyInLiteralFlag {
  NoFlags = 0,
  SetFunctionName = 1 << 0,
}

/// 原版 globals.h:2908 enum DefineKeyedOwnPropertyFlag
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum DefineKeyedOwnPropertyFlag {
  NoFlags = 0,
  SetFunctionName = 1 << 0,
}

/// 原版 DefineKeyedOwnPropertyInLiteralFlags = base::Flags<...>
/// (globals.h:2904-2906) — u8 位集最小替身 (审查环 round1 补遗漏,
/// 形制对齐 CodeFlushModeSet)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DefineKeyedOwnPropertyInLiteralFlags(u8);

impl DefineKeyedOwnPropertyInLiteralFlags {
  /// 原版 base::Flags::contains — 枚举判别值本身即位值 (NoFlags=0,
  /// SetFunctionName=1<<0), 直接按位与
  #[inline]
  pub const fn contains(self, flag: DefineKeyedOwnPropertyInLiteralFlag) -> bool {
    self.0 & flag as u8 != 0
  }
  /// 原版 base::Flags 逐位或 (operator|)
  #[inline]
  pub const fn put(self, flag: DefineKeyedOwnPropertyInLiteralFlag) -> Self {
    Self(self.0 | flag as u8)
  }
  /// 原版 base::Flags::empty
  #[inline]
  pub const fn is_empty(self) -> bool {
    self.0 == 0
  }
}

/// 原版 DefineKeyedOwnPropertyFlags = base::Flags<...>
/// (globals.h:2912-2913) — 同上位集替身
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DefineKeyedOwnPropertyFlags(u8);

impl DefineKeyedOwnPropertyFlags {
  /// 原版 base::Flags::contains — 枚举判别值本身即位值, 直接按位与
  #[inline]
  pub const fn contains(self, flag: DefineKeyedOwnPropertyFlag) -> bool {
    self.0 & flag as u8 != 0
  }
  /// 原版 base::Flags 逐位或 (operator|)
  #[inline]
  pub const fn put(self, flag: DefineKeyedOwnPropertyFlag) -> Self {
    Self(self.0 | flag as u8)
  }
  /// 原版 base::Flags::empty
  #[inline]
  pub const fn is_empty(self) -> bool {
    self.0 == 0
  }
}

/// 原版 globals.h:2915 enum ExternalArrayType — 判别值 1..12 连续 (生成物错值已修)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum ExternalArrayType {
  ExternalInt8Array = 1,
  ExternalUint8Array,
  ExternalInt16Array,
  ExternalUint16Array,
  ExternalInt32Array,
  ExternalUint32Array,
  ExternalFloat16Array,
  ExternalFloat32Array,
  ExternalFloat64Array,
  ExternalUint8ClampedArray,
  ExternalBigInt64Array,
  ExternalBigUint64Array,
}

/// 原版 globals.h:2974 enum CachedTieringDecision : int32_t
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum CachedTieringDecision {
  Pending,
  EarlySparkplug,
  DelayMaglev,
  EarlyMaglev,
  EarlyTurbofan,
  Normal,
}

/// 原版 globals.h:3041 enum SpeculationMode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum SpeculationMode {
  AllowSpeculation = 0,
  DisallowBoundsCheckSpeculation = 1,
  DisallowSpeculation = 3,
}

/// 原版 globals.h:3046 enum CallFeedbackContent
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum CallFeedbackContent {
  Target,
  Receiver,
}

/// 原版 globals.h:3061 enum BlockingBehavior
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum BlockingBehavior {
  Block,
  DontBlock,
}

/// 原版 globals.h:3063 enum ConcurrencyMode : uint8_t
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum ConcurrencyMode {
  Synchronous,
  Concurrent,
}

/// 原版 globals.h:3093 enum AliasingKind
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum AliasingKind {
  Overlap,
  Combine,
  Independent,
}

/// 原版 FOREACH_WASM_TRAPREASON X-macro (globals.h:3103-3124)
/// 机械展开为枚举, 顺序 = 列表顺序
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u32)]
pub enum WasmTrapReason {
  TrapUnreachable,
  TrapMemOutOfBounds,
  TrapUnalignedAccess,
  TrapDivByZero,
  TrapDivUnrepresentable,
  TrapRemByZero,
  TrapFloatUnrepresentable,
  TrapNullFunc,
  TrapFuncSigMismatch,
  TrapDataSegmentOutOfBounds,
  TrapElementSegmentOutOfBounds,
  TrapTableOutOfBounds,
  TrapRethrowNull,
  TrapNullDereference,
  TrapIllegalCast,
  TrapArrayOutOfBounds,
  TrapArrayTooLarge,
  TrapResume,
  TrapSuspend,
  TrapSwitch,
  TrapStringOffsetOutOfBounds,
}

/// 原版 globals.h:3126 enum KeyedAccessLoadMode : uint8_t
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum KeyedAccessLoadMode {
  InBounds = 0b00,
  HandleOOB = 0b01,
  HandleHoles = 0b10,
  HandleOOBAndHoles = 0b11,
}

/// 原版 globals.h:3158 enum KeyedAccessStoreMode : uint8_t
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum KeyedAccessStoreMode {
  InBounds,
  GrowAndHandleCOW,
  IgnoreTypedArrayOOB,
  HandleCOW,
}

/// 原版 globals.h:3179 enum MutableMode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum MutableMode {
  Mutable,
  Immutable,
}

/// 原版 globals.h:3203 enum IcCheckType
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum IcCheckType {
  Element,
  Property,
}

/// 原版 globals.h:3213 enum StubCallMode (脚本跳过, 修订补全;
/// V8_ENABLE_WEBASSEMBLY 开 → 3 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum StubCallMode {
  CodeObject,
  WasmRuntimeStub,
  BuiltinPointer,
}

/// 原版 globals.h:3255 enum ExceptionStatus : bool
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum ExceptionStatus {
  Exception = 0,
  Success = 1,
}

/// 原版 globals.h:3263 enum VariableAllocationInfo
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum VariableAllocationInfo {
  None,
  Stack,
  Context,
  Unused,
}

/// 原版 globals.h:3313 enum PropertiesEnumerationMode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum PropertiesEnumerationMode {
  EnumerationOrder,
  PropertyAdditionOrder,
}

/// 原版 globals.h:3321 enum StringTransitionStrategy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum StringTransitionStrategy {
  Copy,
  InPlace,
  AlreadyTransitioned,
}

/// 原版 globals.h:3376 enum CallJumpMode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum CallJumpMode {
  Call,
  TailCall,
}

/// 原版 globals.h:3380 enum PrimitiveType
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum PrimitiveType {
  Boolean,
  Number,
  String,
  Symbol,
}

/// 原版 globals.h:3382 enum SilenceNanMode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum SilenceNanMode {
  SilenceUndefined,
  PreserveUndefined,
}

/// 原版 globals.h:3413 enum AddStringConstantAndInternalizeVariant : uint8_t
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum AddStringConstantAndInternalizeVariant {
  LhsIsStringConstant = 0,
  RhsIsStringConstant = 1,
}

// ── 常量 (globals.h 常量清单逐条落位; 原版行号随注释) ──────────────────────────
// 尺寸/对齐类统一 usize (Rust 指针运算惯例, 登记偏差); 值语义类保持原宽度。

/// 原版 globals.h:177-208 V8_DEFAULT_STACK_SIZE_KB — x64/arm64 非 ASan 分支;
/// ASan 宿主 960/64, ia32 472/40, arm32 864/160, 登记分支取舍
pub const V8_DEFAULT_STACK_SIZE_KB: usize = 984;
/// 原版 globals.h:208 V8_STACK_LIMIT_MARGIN_KB
pub const V8_STACK_LIMIT_MARGIN_KB: usize = 40;
// 原版 static_assert (globals.h:233) V8_DEFAULT_STACK_SIZE_KB * KB + slack <= MB
const _: () = assert!(
  V8_DEFAULT_STACK_SIZE_KB * KB + K_STACK_LIMIT_SLACK_FOR_DEOPTIMIZATION_IN_BYTES as usize <= MB
);

/// 原版 globals.h:224 kStackSpaceRequiredForCompilation
pub const K_STACK_SPACE_REQUIRED_FOR_COMPILATION: i32 = 40;
/// 原版 globals.h:230 kStackLimitSlackForDeoptimizationInBytes
pub const K_STACK_LIMIT_SLACK_FOR_DEOPTIMIZATION_IN_BYTES: i32 = 256;
/// 原版 globals.h:253 kShortBuiltinCallsOldSpaceSizeThreshold
pub const K_SHORT_BUILTIN_CALLS_OLD_SPACE_SIZE_THRESHOLD: usize = 2 * GB;

/// 原版 globals.h:424-435 kMaxInt 族
pub const K_MAX_INT: i32 = 0x7FFF_FFFF;
pub const K_MIN_INT: i32 = -K_MAX_INT - 1;
pub const K_MAX_INT8: i32 = (1 << 7) - 1;
pub const K_MIN_INT8: i32 = -(1 << 7);
pub const K_MAX_UINT8: i32 = (1 << 8) - 1;
pub const K_MIN_UINT8: i32 = 0;
pub const K_MAX_INT16: i32 = (1 << 15) - 1;
pub const K_MIN_INT16: i32 = -(1 << 15);
pub const K_MAX_UINT16: i32 = (1 << 16) - 1;
pub const K_MIN_UINT16: i32 = 0;
pub const K_MAX_INT31: i32 = K_MAX_INT / 2;
pub const K_MIN_INT31: i32 = K_MIN_INT / 2;

/// 原版 globals.h:437-441 kMaxUInt32 / kMaxUInt64
pub const K_MAX_UINT32: u32 = 0xFFFF_FFFF;
pub const K_MIN_UINT32: u32 = 0;
pub const K_MAX_UINT64: u64 = 0xFFFF_FFFF_FFFF_FFFF;
pub const K_MIN_UINT64: u64 = 0;

/// 原版 globals.h:443-464 sizeof 族 (64 位宿主)
pub const K_INT8_SIZE: usize = 1;
pub const K_UINT8_SIZE: usize = 1;
pub const K_BYTE_SIZE: usize = 1;
pub const K_CHAR_SIZE: usize = 1;
pub const K_SHORT_SIZE: usize = 2;
pub const K_INT16_SIZE: usize = 2;
pub const K_UINT16_SIZE: usize = 2;
pub const K_INT_SIZE: usize = 4;
pub const K_INT32_SIZE: usize = 4;
pub const K_UINT32_SIZE: usize = 4;
pub const K_INT64_SIZE: usize = 8;
pub const K_UINT64_SIZE: usize = 8;
pub const K_SIZET_SIZE: usize = 8;
/// 原版 kFloat16Size = sizeof(uint16_t)
pub const K_FLOAT16_SIZE: usize = 2;
pub const K_FLOAT_SIZE: usize = 4;
pub const K_DOUBLE_SIZE: usize = 8;
pub const K_INTPTR_SIZE: usize = 8;
pub const K_UINTPTR_SIZE: usize = 8;
pub const K_SYSTEM_POINTER_SIZE: usize = 8;
pub const K_SYSTEM_POINTER_HEX_DIGITS: usize = 12;
pub const K_PC_ON_STACK_SIZE: usize = K_SYSTEM_POINTER_SIZE;
pub const K_FP_ON_STACK_SIZE: usize = K_SYSTEM_POINTER_SIZE;

/// 原版 globals.h:466-470 kElidedFrameSlots — 仅 V8_TARGET_ARCH_X64/IA32
/// 计 PC 压栈槽 (=1), 其余架构 (arm64 等) 为 0
pub const K_ELIDED_FRAME_SLOTS: usize = if cfg!(any(target_arch = "x86_64", target_arch = "x86")) {
  K_PC_ON_STACK_SIZE / K_SYSTEM_POINTER_SIZE
} else {
  0
};

/// 原版 globals.h:472-489
pub const K_DOUBLE_SIZE_LOG2: u32 = 3;
pub const K_MAX_DOUBLE_STRING_LENGTH: usize = 25;

/// 原版 globals.h:495 kMaxCommittedWasmCodeMB
pub const K_MAX_COMMITTED_WASM_CODE_MB: u32 = 4095;
/// 原版 globals.h:500-512 kDefaultMaxWasmCodeSpaceSizeMb — arm64/loong64 128, 其余 1024
pub const K_DEFAULT_MAX_WASM_CODE_SPACE_SIZE_MB: u32 =
  if cfg!(any(target_arch = "aarch64", target_arch = "loongarch64")) {
    128
  } else {
    1024
  };

/// 原版 globals.h:515 kIsolateDataAlignment
pub const K_ISOLATE_DATA_ALIGNMENT: usize = 64;

/// 原版 globals.h:517-548 64 位宿主分支
pub const K_SYSTEM_POINTER_SIZE_LOG2: u32 = 3;
/// 原版 kIntptrSignBit — C++ 为 intptr_t 位型, Rust 落 usize 位型 (登记偏差)
pub const K_INTPTR_SIGN_BIT: usize = 0x8000_0000_0000_0000;
pub const K_PLATFORM_REQUIRES_CODE_RANGE: bool = true;
/// 原版 kMaximalCodeRangeSize — x64: CP&&!ECS ? 128MB : 512MB; arm64: 128/256MB
/// 目标配置 CP=true, ECS=true → x64 512MB, arm64 256MB
pub const K_MAXIMAL_CODE_RANGE_SIZE: usize = if cfg!(target_arch = "x86_64") {
  512 * MB
} else {
  256 * MB
};
/// 原版 kMinExpectedOSPageSize — x64/arm64 4KB
pub const K_MIN_EXPECTED_OS_PAGE_SIZE: usize = 4 * KB;
pub const K_MINIMUM_CODE_RANGE_SIZE: usize = 64 * MB;
/// 原版 kReservedCodeRangePages — Windows 1, 其余 0
pub const K_RESERVED_CODE_RANGE_PAGES: usize = if cfg!(target_os = "windows") { 1 } else { 0 };
pub const K_MAXIMAL_TRUSTED_RANGE_SIZE: usize = GB;
pub const K_MINIMUM_TRUSTED_RANGE_SIZE: usize = 512 * MB;

// 原版 static_assert (globals.h:573) kSystemPointerSize == (1 << kSystemPointerSizeLog2)
const _: () = assert!(K_SYSTEM_POINTER_SIZE == (1 << K_SYSTEM_POINTER_SIZE_LOG2));

/// 原版 globals.h:580-581 kTaggedSize — COMPRESS_POINTERS 分支, 值源于
/// include/v8-internal.h kApiTaggedSize
pub const K_TAGGED_SIZE: usize = K_API_TAGGED_SIZE;
pub const K_TAGGED_SIZE_LOG2: u32 = 2;

// 原版 static_assert (globals.h:663-670)
const _: () = assert!(K_TAGGED_SIZE == (1 << K_TAGGED_SIZE_LOG2));
const _: () = assert!(K_TAGGED_SIZE == K_API_TAGGED_SIZE);

/// 原版 globals.h:606 kPointerTableAddressSpaceName
pub const K_POINTER_TABLE_ADDRESS_SPACE_NAME: &str = "v8-pointer-table";

/// 原版 globals.h:615 kNullJSDispatchHandle
pub const K_NULL_JS_DISPATCH_HANDLE: JsDispatchHandle = 0;

/// 原版 globals.h:617-623 kJSDispatchTableEntrySize — 非 LA57 分支 (x64 默认无 LA57)
pub const K_JS_DISPATCH_TABLE_ENTRY_SIZE: usize = 16;
pub const K_JS_DISPATCH_TABLE_ENTRY_SIZE_LOG2: u32 = 4;
/// 原版 globals.h:635 — (V8_LOWER_LIMITS_MODE ? 1 : 16) * entry * MB
pub const K_JS_DISPATCH_TABLE_RESERVATION_SIZE: usize = 16 * K_JS_DISPATCH_TABLE_ENTRY_SIZE * MB;
/// 原版 globals.h:640 kMaxJSDispatchEntries
pub const K_MAX_JS_DISPATCH_ENTRIES: usize =
  K_JS_DISPATCH_TABLE_RESERVATION_SIZE / K_JS_DISPATCH_TABLE_ENTRY_SIZE;
/// 原版 globals.h:648 kJSDispatchHandleShift — 64 位非 iOS: V8_LOWER_LIMITS_MODE ? 12 : 8
pub const K_JS_DISPATCH_HANDLE_SHIFT: u32 = 8;

// 原版 static_assert (globals.h:651)
const _: () = assert!((1usize << (32 - K_JS_DISPATCH_HANDLE_SHIFT)) == K_MAX_JS_DISPATCH_ENTRIES);
const _: () = assert!(!config::V8_LOWER_LIMITS_MODE);

/// 原版 globals.h:688-693 kExternalPointerSlotSize — sandbox 开 → kTaggedSize
pub const K_EXTERNAL_POINTER_SLOT_SIZE: usize = K_EXTERNAL_POINTER_HANDLE_SIZE;
/// 原版 globals.h:697-702 kCppHeapPointerSlotSize — compress 开 → 4
pub const K_CPP_HEAP_POINTER_SLOT_SIZE: usize = K_CPP_HEAP_POINTER_HANDLE_SIZE;
/// 原版 globals.h:704 kIndirectPointerSize = sizeof(IndirectPointerHandle)
pub const K_INDIRECT_POINTER_SIZE: usize = K_INDIRECT_POINTER_HANDLE_SIZE;
/// 原版 globals.h:709 kTrustedPointerSize — sandbox 开 → kIndirectPointerSize
pub const K_TRUSTED_POINTER_SIZE: usize = K_INDIRECT_POINTER_SIZE;
/// 原版 TrustedPointer_t (globals.h:710-713, sandbox 开分支) =
/// TrustedPointerHandle (uint32); 审查环 round1 补遗漏
pub type TrustedPointerT = u32;
/// 原版 globals.h:715 kCodePointerSize
pub const K_CODE_POINTER_SIZE: usize = K_TRUSTED_POINTER_SIZE;
/// 原版 globals.h:720 kProtectedPointerSize = kTaggedSize
pub const K_PROTECTED_POINTER_SIZE: usize = K_TAGGED_SIZE;
/// 原版 globals.h:722 kJSDispatchHandleSize
pub const K_JS_DISPATCH_HANDLE_SIZE: usize = 4;

/// 原版 globals.h:732 kPlaceholderDispatchHandle / :736 kInvalidDispatchHandle
pub const K_PLACEHOLDER_DISPATCH_HANDLE: JsDispatchHandle = 0x0;
pub const K_INVALID_DISPATCH_HANDLE: JsDispatchHandle =
  0xFFFF_FFFFu32.wrapping_shl(K_JS_DISPATCH_HANDLE_SHIFT);

/// 原版 globals.h:739-743
pub const K_EMBEDDER_DATA_SLOT_SIZE: usize = 2 * K_TAGGED_SIZE;
pub const K_EMBEDDER_DATA_SLOT_SIZE_IN_TAGGED_SLOTS: usize =
  K_EMBEDDER_DATA_SLOT_SIZE / K_TAGGED_SIZE;
const _: () = assert!(K_EMBEDDER_DATA_SLOT_SIZE >= K_SYSTEM_POINTER_SIZE);

/// 原版 globals.h:745 kExternalAllocationSoftLimit — 值源于
/// include/v8-internal.h Internals::kExternalAllocationSoftLimit
pub const K_EXTERNAL_ALLOCATION_SOFT_LIMIT: usize = K_EXTERNAL_ALLOCATION_SOFT_LIMIT_INTERNAL;

/// 原版 globals.h:755 kMaxRegularHeapObjectSize = 1 << (kPageSizeBits - 1)
pub const K_MAX_REGULAR_HEAP_OBJECT_SIZE: usize = 1 << (K_PAGE_SIZE_BITS - 1);

/// 原版 globals.h:757-762
pub const K_BITS_PER_BYTE: usize = 8;
pub const K_BITS_PER_BYTE_LOG2: u32 = 3;
pub const K_BITS_PER_SYSTEM_POINTER: usize = K_SYSTEM_POINTER_SIZE * K_BITS_PER_BYTE;
pub const K_BITS_PER_SYSTEM_POINTER_LOG2: u32 = K_SYSTEM_POINTER_SIZE_LOG2 + K_BITS_PER_BYTE_LOG2;
pub const K_BITS_PER_INT: usize = K_INT_SIZE * K_BITS_PER_BYTE;

/// 原版 globals.h:765-772 IEEE 754 单精度位布局
pub const K_BINARY32_SIGN_MASK: u32 = 0x8000_0000;
pub const K_BINARY32_EXPONENT_MASK: u32 = 0x7F80_0000;
pub const K_BINARY32_MANTISSA_MASK: u32 = 0x007F_FFFF;
pub const K_BINARY32_EXPONENT_BIAS: i32 = 127;
pub const K_BINARY32_MAX_EXPONENT: i32 = 0xFE;
pub const K_BINARY32_MIN_EXPONENT: i32 = 0x01;
pub const K_BINARY32_MANTISSA_BITS: i32 = 23;
pub const K_BINARY32_EXPONENT_SHIFT: i32 = 23;

/// 原版 globals.h:776 kQuietNaNMask — 位 51..62
pub const K_QUIET_NAN_MASK: u64 = 0xFFFu64 << 51;

/// 原版 globals.h:778 kOneByteSize
pub const K_ONE_BYTE_SIZE: usize = K_CHAR_SIZE;

/// 原版 globals.h:781-790 SIMD 尺寸
pub const K_SIMD128_SIZE: usize = 16;
pub const K_SIMD128_HALF_SIZE: usize = K_SIMD128_SIZE / 2;
pub const K_SIMD128_QUARTER_SIZE: usize = K_SIMD128_SIZE / 4;
pub const K_SIMD256_SIZE: usize = 32;

// 原版 globals.h:864-865 LanguageModeSize 已随 LanguageMode 输出
/// 原版 globals.h:963-973 内嵌反馈
pub const K_FEEDBACK_IS_EMBEDDED: i32 = -1;
pub const K_UNINITIALIZED_EMBEDDED_FEEDBACK: i32 = 0;
pub const K_UNARY_EMBEDDED_FEEDBACK_OPERAND_INDEX: i32 = 0;
pub const K_EMBEDDED_FEEDBACK_OPERAND_INDEX: i32 = 1;

/// 原版 globals.h:976-981 拦截器常量
pub const K_INTERCEPTED_NO: u8 = 1;
pub const K_INTERCEPTED_YES: u8 = 0;
pub const K_INTERCEPTED_SIZE: usize = 4;
pub const K_NOT_INTERCEPTED_SENTINEL: u32 = K_HEAP_OBJECT_TAG as u32;

/// 原版 globals.h:1000-1011 位置/偏移哨兵
pub const K_NO_SOURCE_POSITION: i32 = -1;
pub const K_FUNCTION_ENTRY_BYTECODE_OFFSET: i32 = -1;
pub const K_FUNCTION_EXIT_BYTECODE_OFFSET: i32 = -1;
pub const K_NO_DEOPTIMIZATION_ID: i32 = -1;

/// 原版 globals.h:1061-1087 Smi 布局 — 值源于 internal 模块 (SmiTagging<4>)
pub const K_IS_SMI_VALUE_IN_UPPER_32_BITS: bool =
  (K_SMI_VALUE_SIZE + K_SMI_SHIFT_SIZE + K_SMI_TAG_SIZE) == 64;
pub const K_IS_SMI_VALUE_IN_LOWER_32_BITS: bool =
  (K_SMI_VALUE_SIZE + K_SMI_SHIFT_SIZE + K_SMI_TAG_SIZE) == 32;
// 原版 static_assert (globals.h:1071-1076) SmiValuesAre32Bits/31Bits 一致性
const _: () = assert!(!SMI_VALUES_ARE_32_BITS == SMI_VALUES_ARE_31_BITS);
const _: () = assert!(SMI_VALUES_ARE_32_BITS == K_IS_SMI_VALUE_IN_UPPER_32_BITS);
const _: () = assert!(SMI_VALUES_ARE_31_BITS == K_IS_SMI_VALUE_IN_LOWER_32_BITS);
const _: () = assert!(K_SMI_VALUE_SIZE <= 32);
const _: () = assert!((K_SMI_VALUE_SIZE + K_SMI_SHIFT_SIZE + K_SMI_TAG_SIZE).is_multiple_of(32));

/// 原版 globals.h:1079 kSmiSignMask
pub const K_SMI_SIGN_MASK: usize = 1 << (K_SMI_VALUE_SIZE + K_SMI_SHIFT_SIZE + K_SMI_TAG_SIZE - 1);
/// 原版 globals.h:1086 kSmiHashMask
pub const K_SMI_HASH_MASK: u32 = ((1u64 << (K_SMI_VALUE_SIZE - 1)) - 1) as u32;

/// 原版 globals.h:1090-1096 对象对齐
pub const K_OBJECT_ALIGNMENT_BITS: u32 = K_TAGGED_SIZE_LOG2;
pub const K_OBJECT_ALIGNMENT: usize = 1 << K_OBJECT_ALIGNMENT_BITS;
pub const K_OBJECT_ALIGNMENT_MASK: usize = K_OBJECT_ALIGNMENT - 1;
pub const K_OBJECT_ALIGNMENT_8GB_HEAP: usize = 8;
pub const K_OBJECT_ALIGNMENT_8GB_HEAP_MASK: usize = K_OBJECT_ALIGNMENT_8GB_HEAP - 1;

/// 原版 globals.h:1106-1111 指针/浮点对齐
pub const K_POINTER_ALIGNMENT: usize = 1 << K_SYSTEM_POINTER_SIZE_LOG2;
pub const K_POINTER_ALIGNMENT_MASK: usize = K_POINTER_ALIGNMENT - 1;
pub const K_DOUBLE_ALIGNMENT: usize = 8;
pub const K_DOUBLE_ALIGNMENT_MASK: usize = K_DOUBLE_ALIGNMENT - 1;

/// 原版 globals.h:1116-1131 kCodeAlignmentBits — x64 6, 其余 5 (ppc64/riscv 分支后置)
pub const K_CODE_ALIGNMENT_BITS: u32 = if cfg!(target_arch = "x86_64") { 6 } else { 5 };
pub const K_CODE_ALIGNMENT: usize = 1 << K_CODE_ALIGNMENT_BITS;
pub const K_CODE_ALIGNMENT_MASK: usize = K_CODE_ALIGNMENT - 1;

/// 原版 globals.h:1133 kWeakHeapObjectMask
pub const K_WEAK_HEAP_OBJECT_MASK: usize = 1 << 1;
/// 原版 globals.h:1147 kClearedWeakHeapObjectLower32
pub const K_CLEARED_WEAK_HEAP_OBJECT_LOWER32: u32 = 3;

/// 原版 globals.h:1151-1168 zap 值 — V8_HOST_ARCH_64_BIT 分支
pub const K_CLEARED_FREE_MEMORY_VALUE: u64 = 0;
pub const K_ZAP_VALUE: u64 = 0xdeadbeedbeadbeef;
pub const K_HANDLE_ZAP_VALUE: u64 = 0x1baddead0baddeaf;
pub const K_GLOBAL_HANDLE_ZAP_VALUE: u64 = 0x1baffed00baffedf;
pub const K_PERSISTENT_HANDLE_ZAP_VALUE: u64 = 0x1baffed66baffedf;
pub const K_TRACED_HANDLE_EAGER_RESET_ZAP_VALUE: u64 = 0x1beffedaabaffedf;
pub const K_TRACED_HANDLE_MINOR_GC_RESET_ZAP_VALUE: u64 = 0x1beffedeebaffedf;
pub const K_TRACED_HANDLE_MINOR_GC_WEAK_RESET_ZAP_VALUE: u64 = 0x1beffed11baffedf;
pub const K_TRACED_HANDLE_FULL_GC_RESET_ZAP_VALUE: u64 = 0x1beffed77baffedf;
pub const K_FROM_SPACE_ZAP_VALUE: u64 = 0x1beefdad0beefdaf;
pub const K_DEBUG_ZAP_VALUE: u64 = 0xbadbaddbbadbaddb;
pub const K_SLOTS_ZAP_VALUE: u64 = 0xbeefdeadbeefdeef;
pub const K_FREE_LIST_ZAP_VALUE: u64 = 0xfeed1eaffeed1eaf;

/// 原版 globals.h:1185-1186
pub const K_CODE_ZAP_VALUE: i32 = 0xbadc0de;
pub const K_PHANTOM_REFERENCE_ZAP: u32 = 0x0ca11bac;

/// 原版 globals.h:1191 PROCESSOR_CACHE_LINE_SIZE
pub const PROCESSOR_CACHE_LINE_SIZE: usize = 64;

/// 原版 globals.h:1195 kQuietNaNHighBitsMask
pub const K_QUIET_NAN_HIGH_BITS_MASK: u32 = 0xFFF << (51 - 32);

// 原版 globals.h:1515 kSpaceTagSize 已随 AllocationSpace 输出
/// 原版 globals.h:2015-2020 kIeeeDouble 双字偏移 — 小端 0/4, 大端 4/0
pub const K_IEEE_DOUBLE_MANTISSA_WORD_OFFSET: usize =
  if cfg!(target_endian = "little") { 0 } else { 4 };
pub const K_IEEE_DOUBLE_EXPONENT_WORD_OFFSET: usize =
  if cfg!(target_endian = "little") { 4 } else { 0 };

/// 原版 globals.h:2148 kScopeInfoMaxInlinedLocalNamesSize
pub const K_SCOPE_INFO_MAX_INLINED_LOCAL_NAMES_SIZE: i32 = 75;

/// 原版 globals.h:2179-2195 kHoleNan — 非 mips64r6 前置分支 (x64/arm64)
pub const K_HOLE_NAN_UPPER32: u32 = 0xFFF7_FFFF;
pub const K_HOLE_NAN_LOWER32: u32 = 0xFFF7_FFFF;
pub const K_HOLE_NAN_INT64: u64 = ((K_HOLE_NAN_UPPER32 as u64) << 32) | K_HOLE_NAN_LOWER32 as u64;
// 原版 V8_ENABLE_UNDEFINED_DOUBLE 关 → kUndefinedNan 族与 HoleNan()/IsUndefinedNan()
// 不落位 (globals.h:2182-2209, 登记裁剪)

/// 原版 globals.h:2213-2234 安全整数族
pub const K_MAX_SAFE_INTEGER_UINT64: u64 = (1u64 << 53) - 1;
pub const K_MAX_SAFE_INTEGER: f64 = K_MAX_SAFE_INTEGER_UINT64 as f64;
pub const K_MIN_SAFE_INTEGER: f64 = -K_MAX_SAFE_INTEGER;
pub const K_MAX_UINT32_DOUBLE: f64 = K_MAX_UINT32 as f64;
pub const K_MAX_ADDITIVE_SAFE_INTEGER: i64 = (1i64 << 52) - 1;
pub const K_MIN_ADDITIVE_SAFE_INTEGER: i64 = -(1i64 << 52) + 1;
// 原版 static_assert (globals.h:2223-2226)
const _: () =
  assert!(K_MAX_ADDITIVE_SAFE_INTEGER + K_MAX_ADDITIVE_SAFE_INTEGER <= K_MAX_SAFE_INTEGER as i64);
const _: () =
  assert!(K_MIN_ADDITIVE_SAFE_INTEGER + K_MIN_ADDITIVE_SAFE_INTEGER >= K_MIN_SAFE_INTEGER as i64);
pub const K_MAX_ADDITIVE_SAFE_INTEGER_FEEDBACK: i64 = (1i64 << 50) - 1;
pub const K_MIN_ADDITIVE_SAFE_INTEGER_FEEDBACK: i64 = -(1i64 << 50);
pub const K_ADDITIVE_SAFE_INTEGER_FEEDBACK_BIT_LENGTH: i32 = 51;
pub const K_ADDITIVE_SAFE_INTEGER_FEEDBACK_SHIFT: i32 =
  64 - K_ADDITIVE_SAFE_INTEGER_FEEDBACK_BIT_LENGTH;

/// 原版 globals.h:2950 kTieringStateInProgressBlocksTierup
pub const K_TIERING_STATE_IN_PROGRESS_BLOCKS_TIERUP: bool = true;

/// 原版 globals.h:3221-3236 参数计数族
pub const K_INVALID_INFO_ID: i32 = -1;
pub const K_FUNCTION_LITERAL_ID_TOP_LEVEL: i32 = 0;
pub const K_SWISS_NAME_DICTIONARY_INITIAL_CAPACITY: u32 = 4;
pub const K_SMALL_ORDERED_HASH_SET_MIN_CAPACITY: i32 = 4;
pub const K_SMALL_ORDERED_HASH_MAP_MIN_CAPACITY: i32 = 4;
/// 原版 kAdapt / kDontAdapt (globals.h:3232-3233)
pub const K_ADAPT: AdaptArguments = true;
pub const K_DONT_ADAPT: AdaptArguments = false;
pub const K_JS_ARGC_RECEIVER_SLOTS: i32 = 1;
pub const K_DONT_ADAPT_ARGUMENTS_SENTINEL: u16 = 0;

/// 原版 JSParameterCount (globals.h:3240)
#[inline]
pub const fn js_parameter_count(param_count_without_receiver: i32) -> i32 {
  param_count_without_receiver + K_JS_ARGC_RECEIVER_SLOTS
}

pub const K_JS_CALL_CLOSURE_PARAMETER_INDEX: i32 = -1;
pub const K_MIN_PARAMETER_INDEX: i32 = K_JS_CALL_CLOSURE_PARAMETER_INDEX;

/// 原版 globals.h:3374 kInvalidWasmSignatureHash
pub const K_INVALID_WASM_SIGNATURE_HASH: u64 = !0u64;
/// 原版 globals.h:3378 kPreallocatedNumberStringTableSize
pub const K_PREALLOCATED_NUMBER_STRING_TABLE_SIZE: u32 = 100;
/// 原版 globals.h:3411 kAdd_String_Constant_Internalize_CacheSlotOffset
pub const K_ADD_STRING_CONSTANT_INTERNALIZE_CACHE_SLOT_OFFSET: i32 = 1;

// ── 助记函数/小类型补全 (审查环 R1) ─────────────────────────────────────────
// 对照原版行号逐项转写; ToString 族落 as_str (const, 零开销), 判定谓词落
// 关联/自由 const fn, 命名随既有文件风格 (谓词入方法, 独立函数保留原名)。

// ── 语言模式 (globals.h:885-909, 1881-1883) ────────────────────────────────

/// 原版 is_sloppy (globals.h:885-887)
#[inline]
pub const fn is_sloppy(language_mode: LanguageMode) -> bool {
  language_mode as u8 == LanguageMode::Sloppy as u8
}

/// 原版 is_strict (globals.h:889-891) — 非 sloppy 即 strict
#[inline]
pub const fn is_strict(language_mode: LanguageMode) -> bool {
  language_mode as u8 != LanguageMode::Sloppy as u8
}

/// 原版 is_valid_language_mode (globals.h:893-895) — int 值域校验 (解析器边界)
#[inline]
pub const fn is_valid_language_mode(language_mode: i32) -> bool {
  language_mode == LanguageMode::Sloppy as i32 || language_mode == LanguageMode::Strict as i32
}

/// 原版 construct_language_mode (globals.h:897-899)
#[inline]
pub const fn construct_language_mode(strict_bit: bool) -> LanguageMode {
  if strict_bit {
    LanguageMode::Strict
  } else {
    LanguageMode::Sloppy
  }
}

/// 原版 stricter_language_mode (globals.h:903-909) — 任一 strict 即 strict
/// (bool 枚举判别值或运算)
#[inline]
pub const fn stricter_language_mode(mode1: LanguageMode, mode2: LanguageMode) -> LanguageMode {
  if (mode1 as u8 | mode2 as u8) == LanguageMode::Strict as u8 {
    LanguageMode::Strict
  } else {
    LanguageMode::Sloppy
  }
}

/// 原版 construct_repl_mode (globals.h:1881-1883)
#[inline]
pub const fn construct_repl_mode(is_repl_mode: bool) -> REPLMode {
  if is_repl_mode {
    REPLMode::Yes
  } else {
    REPLMode::No
  }
}

// ── code flushing (globals.h:1859-1873) ────────────────────────────────────

/// 原版 base::EnumSet<CodeFlushMode> 最小替身 — u8 位集 (三变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CodeFlushModeSet(u8);

impl CodeFlushModeSet {
  /// 原版 EnumSet::contains
  #[inline]
  pub const fn contains(self, mode: CodeFlushMode) -> bool {
    self.0 & (1 << mode as u8) != 0
  }

  /// 原版 EnumSet::put — 值语义并入
  #[inline]
  pub const fn put(self, mode: CodeFlushMode) -> Self {
    Self(self.0 | (1 << mode as u8))
  }

  /// 原版 EnumSet::empty 判定
  #[inline]
  pub const fn is_empty(self) -> bool {
    self.0 == 0
  }
}

/// 原版 IsBaselineCodeFlushingEnabled (globals.h:1859-1861)
#[inline]
pub const fn is_baseline_code_flushing_enabled(mode: CodeFlushModeSet) -> bool {
  mode.contains(CodeFlushMode::FlushBaselineCode)
}

/// 原版 IsByteCodeFlushingEnabled (globals.h:1864-1866)
#[inline]
pub const fn is_byte_code_flushing_enabled(mode: CodeFlushModeSet) -> bool {
  mode.contains(CodeFlushMode::FlushBytecode)
}

/// 原版 IsForceFlushingEnabled (globals.h:1869-1871)
#[inline]
pub const fn is_force_flushing_enabled(mode: CodeFlushModeSet) -> bool {
  mode.contains(CodeFlushMode::ForceFlush)
}

/// 原版 IsFlushingDisabled (globals.h:1873)
#[inline]
pub const fn is_flushing_disabled(mode: CodeFlushModeSet) -> bool {
  mode.is_empty()
}

// ── ToString 族 ────────────────────────────────────────────────────────────

impl AllocationType {
  /// 原版 IsSharedAllocationType (globals.h:1764-1768)
  #[inline]
  pub const fn is_shared(self) -> bool {
    matches!(
      self,
      Self::SharedOld | Self::SharedMap | Self::SharedTrusted
    )
  }

  /// 原版 ToString(AllocationType) (globals.h:1585-1607)
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Young => "Young",
      Self::Old => "Old",
      Self::Code => "Code",
      Self::Map => "Map",
      Self::ReadOnly => "ReadOnly",
      Self::SharedOld => "SharedOld",
      Self::SharedMap => "SharedMap",
      Self::SharedTrusted => "SharedTrusted",
      Self::Trusted => "Trusted",
    }
  }
}

impl GarbageCollector {
  /// 原版 ToString(GarbageCollector) (globals.h:1815-1825)
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Scavenger => "Scavenger",
      Self::MarkCompactor => "Mark-Sweep-Compact",
      Self::MinorMarkSweeper => "Minor Mark-Sweep",
    }
  }
}

impl GarbageCollectionReason {
  /// 原版 ToString(GarbageCollectionReason) (globals.h:1683-1751) —
  /// NUM_REASONS 为计数哨兵, 原版 UNREACHABLE, 禁 panic → 空串 (登记)
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::AllocationFailure => "allocation failure",
      Self::AllocationLimit => "allocation limit",
      Self::ContextDisposal => "context disposal",
      Self::CountersExtension => "counters extension",
      Self::Debugger => "debugger",
      Self::Deserializer => "deserialize",
      Self::ExternalMemoryPressure => "external memory pressure",
      Self::FinalizeMarkingViaStackGuard => "finalize incremental marking via stack guard",
      Self::FinalizeMarkingViaTask => "finalize incremental marking via task",
      Self::FullHashtable => "full hash-table",
      Self::HeapProfiler => "heap profiler",
      Self::Task => "task",
      Self::LastResort => "last resort",
      Self::LowMemoryNotification => "low memory notification",
      Self::MakeHeapIterable => "make heap iterable",
      Self::MemoryPressure => "memory pressure",
      Self::MemoryReducer => "memory reducer",
      Self::Runtime => "runtime",
      Self::SamplingProfiler => "sampling profiler",
      Self::SnapshotCreator => "snapshot creator",
      Self::Testing => "testing",
      Self::ExternalFinalize => "external finalize",
      Self::GlobalAllocationLimit => "global allocation limit",
      Self::MeasureMemory => "measure memory",
      Self::Unknown => "unknown",
      Self::BackgroundAllocationFailure => "background allocation failure",
      Self::FinalizeConcurrentMinorMS => "finalize concurrent MinorMS",
      Self::FinalizeMinorMSForMajorGC => "finalize MinorMS for major GC",
      Self::CppHeapAllocationFailure => "CppHeap allocation failure",
      Self::Frozen => "frozen",
      Self::IdleContextDisposal => "idle context disposal",
      Self::NumReasons => "",
    }
  }
}

impl ConcurrencyMode {
  /// 原版 IsSynchronous (globals.h:3065-3067)
  #[inline]
  pub const fn is_synchronous(self) -> bool {
    self as u8 == Self::Synchronous as u8
  }

  /// 原版 IsConcurrent (globals.h:3069-3070)
  #[inline]
  pub const fn is_concurrent(self) -> bool {
    self as u8 == Self::Concurrent as u8
  }

  /// 原版 ToString(ConcurrencyMode) (globals.h:3072-3080) — 全限定串
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Synchronous => "ConcurrencyMode::kSynchronous",
      Self::Concurrent => "ConcurrencyMode::kConcurrent",
    }
  }
}

impl InlineCacheState {
  /// 原版 InlineCacheState2String (globals.h:1937-1959)
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::NoFeedback => "NOFEEDBACK",
      Self::Uninitialized => "UNINITIALIZED",
      Self::Monomorphic => "MONOMORPHIC",
      Self::RecomputeHandler => "RECOMPUTE_HANDLER",
      Self::Polymorphic => "POLYMORPHIC",
      Self::Homomorphic => "HOMOMORPHIC",
      Self::Megamorphic => "MEGAMORPHIC",
      Self::Megadom => "MEGADOM",
      Self::Generic => "GENERIC",
    }
  }
}

impl ExtendedMapKind {
  /// 原版 ExtendedMapKindToString (globals.h:987-993)
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::JSInterceptorMap => "JSInterceptorMap",
    }
  }
}

impl VariableMode {
  /// 原版 VariableMode2String (globals.h:2293-2333, DEBUG-only; Rust 恒可用, 登记)
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Let => "LET",
      Self::Const => "CONST",
      Self::Using => "USING",
      Self::AwaitUsing => "AWAIT_USING",
      Self::Var => "VAR",
      Self::Temporary => "TEMPORARY",
      Self::Dynamic => "DYNAMIC",
      Self::DynamicGlobal => "DYNAMIC_GLOBAL",
      Self::DynamicLocal => "DYNAMIC_LOCAL",
      Self::PrivateMethod => "PRIVATE_METHOD",
      Self::PrivateSetterOnly => "PRIVATE_SETTER_ONLY",
      Self::PrivateGetterOnly => "PRIVATE_GETTER_ONLY",
      Self::PrivateGetterAndSetter => "PRIVATE_GETTER_AND_SETTER",
    }
  }
}

/// 原版 ImmutableLexicalVariableModeToString (globals.h:2334-2337) —
/// 定义域仅 immutable lexical, 越界原版 UNREACHABLE → None (登记)
#[inline]
pub const fn immutable_lexical_variable_mode_to_string(mode: VariableMode) -> Option<&'static str> {
  match mode {
    VariableMode::Const => Some("const"),
    VariableMode::Using => Some("using"),
    VariableMode::AwaitUsing => Some("await using"),
    _ => None,
  }
}

// ── operator<< 打印串族 (差分日志面对齐原版输出) ─────────────────────────────

impl LookupHoistingMode {
  /// 原版 operator<< (globals.h:1050-1059)
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Normal => "normal hoisting",
      Self::LegacySloppy => "legacy sloppy hoisting",
    }
  }
}

impl TypedArrayAccessMode {
  /// 原版 operator<< (globals.h:1798-1806)
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Read => "kRead",
      Self::Write => "kWrite",
    }
  }
}

impl ConvertReceiverMode {
  /// 原版 operator<< (globals.h:2103-2113)
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::NullOrUndefined => "NULL_OR_UNDEFINED",
      Self::NotNullOrUndefined => "NOT_NULL_OR_UNDEFINED",
      Self::Any => "ANY",
    }
  }
}

impl CreateArgumentsType {
  /// 原版 operator<< (globals.h:2134-2144)
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::MappedArguments => "MAPPED_ARGUMENTS",
      Self::UnmappedArguments => "UNMAPPED_ARGUMENTS",
      Self::RestParameter => "REST_PARAMETER",
    }
  }
}

impl InterpreterPushArgsMode {
  /// 原版 operator<< (globals.h:2478-2489)
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::ArrayFunction => "ArrayFunction",
      Self::WithFinalSpread => "WithFinalSpread",
      Self::Other => "Other",
    }
  }
}

impl UnicodeEncoding {
  /// 原版 operator<< (globals.h:2851-2859)
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::UTF16 => "UTF16",
      Self::UTF32 => "UTF32",
    }
  }
}

impl IterationKind {
  /// 原版 operator<< (globals.h:2863-2873) — 全限定串
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Keys => "IterationKind::kKeys",
      Self::Values => "IterationKind::kValues",
      Self::Entries => "IterationKind::kEntries",
    }
  }
}

impl CollectionKind {
  /// 原版 operator<< (globals.h:2877-2885) — 全限定串
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Map => "CollectionKind::kMap",
      Self::Set => "CollectionKind::kSet",
    }
  }
}

impl SpeculationMode {
  /// 原版 operator<< (globals.h:3048-3059) — 全限定串
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::AllowSpeculation => "SpeculationMode::kAllowSpeculation",
      Self::DisallowBoundsCheckSpeculation => "SpeculationMode::kDisallowBoundsCheckSpeculation",
      Self::DisallowSpeculation => "SpeculationMode::kDisallowSpeculation",
    }
  }
}

impl KeyedAccessStoreMode {
  /// 原版 operator<< (globals.h:3165-3177)
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::InBounds => "kInBounds",
      Self::GrowAndHandleCOW => "kGrowAndHandleCOW",
      Self::IgnoreTypedArrayOOB => "kIgnoreTypedArrayOOB",
      Self::HandleCOW => "kHandleCOW",
    }
  }
}

impl SilenceNanMode {
  /// 原版 operator<< (globals.h:3387-3395)
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::SilenceUndefined => "SilenceUndefined",
      Self::PreserveUndefined => "PreserveUndefined",
    }
  }
}

/// 原版 StaticStringsEqual (globals.h:821-826) — 静态串逐字节等值;
/// C++ 按 NUL 终止比较, &str 定长, 长度不等即不等, 无内嵌 NUL 时语义等价
#[inline]
pub const fn static_strings_equal(s1: &str, s2: &str) -> bool {
  let (a, b) = (s1.as_bytes(), s2.as_bytes());
  if a.len() != b.len() {
    return false;
  }
  let mut i = 0;
  while i < a.len() {
    if a[i] != b[i] {
      return false;
    }
    i += 1;
  }
  true
}

// ── KeyedAccess 模式判定 (globals.h:3133-3156, 3181-3201) ──────────────────

impl KeyedAccessLoadMode {
  /// 原版 CreateKeyedAccessLoadMode (globals.h:3133-3138)
  #[inline]
  pub const fn create(handle_oob: bool, handle_holes: bool) -> Self {
    match (handle_oob as u8) | ((handle_holes as u8) << 1) {
      0b01 => Self::HandleOOB,
      0b10 => Self::HandleHoles,
      0b11 => Self::HandleOOBAndHoles,
      _ => Self::InBounds,
    }
  }

  /// 原版 GeneralizeKeyedAccessLoadMode (globals.h:3140-3145) — 位或合并
  #[inline]
  pub const fn generalize(mode1: Self, mode2: Self) -> Self {
    match mode1 as u8 | mode2 as u8 {
      0b01 => Self::HandleOOB,
      0b10 => Self::HandleHoles,
      0b11 => Self::HandleOOBAndHoles,
      _ => Self::InBounds,
    }
  }

  /// 原版 LoadModeHandlesOOB (globals.h:3147-3151)
  #[inline]
  pub const fn handles_oob(self) -> bool {
    self as u8 & Self::HandleOOB as u8 != 0
  }

  /// 原版 LoadModeHandlesHoles (globals.h:3153-3156)
  #[inline]
  pub const fn handles_holes(self) -> bool {
    self as u8 & Self::HandleHoles as u8 != 0
  }
}

impl KeyedAccessStoreMode {
  /// 原版 StoreModeIsInBounds (globals.h:3181-3183)
  #[inline]
  pub const fn is_in_bounds(self) -> bool {
    self as u8 == Self::InBounds as u8
  }

  /// 原版 StoreModeHandlesCOW (globals.h:3185-3188)
  #[inline]
  pub const fn handles_cow(self) -> bool {
    let v = self as u8;
    v == Self::HandleCOW as u8 || v == Self::GrowAndHandleCOW as u8
  }

  /// 原版 StoreModeSupportsTypeArray (globals.h:3190-3193)
  #[inline]
  pub const fn supports_type_array(self) -> bool {
    let v = self as u8;
    v == Self::InBounds as u8 || v == Self::IgnoreTypedArrayOOB as u8
  }

  /// 原版 StoreModeIgnoresTypeArrayOOB (globals.h:3195-3197)
  #[inline]
  pub const fn ignores_type_array_oob(self) -> bool {
    self as u8 == Self::IgnoreTypedArrayOOB as u8
  }

  /// 原版 StoreModeCanGrow (globals.h:3199-3201)
  #[inline]
  pub const fn can_grow(self) -> bool {
    self as u8 == Self::GrowAndHandleCOW as u8
  }
}

/// 原版 ObjectHash (globals.h:2491-2495) — 对象至少指针对齐, 抹去低位零
#[inline]
pub const fn object_hash(address: usize) -> u32 {
  (address >> K_TAGGED_SIZE_LOG2) as u32
}

// ── 反馈编码计算 (globals.h:2562-2632, 2713-2789) ──────────────────────────

/// 原版 BinaryOperationFeedback::TransitionMap (globals.h:2599-2602)
pub type BinaryOpTransitionMap =
  [[u8; BINARY_OP_TRANSITION_MAP_STRIDE as usize]; BINARY_OP_NUM_TYPE_INDICES as usize];
/// 原版 BinaryOperationFeedback::FeedbackEncodeTable (globals.h:2604-2606)
pub type BinaryOpFeedbackEncodeTable = [u8; (BinaryOperationFeedbackType::Any as u32 + 1) as usize];
/// 原版 CompareOperationFeedback::TransitionMap (globals.h:2752-2755)
pub type CompareOpTransitionMap =
  [[u8; COMPARE_OP_TRANSITION_MAP_STRIDE as usize]; COMPARE_OP_NUM_TYPE_INDICES as usize];
/// 原版 CompareOperationFeedback::FeedbackEncodeTable (globals.h:2757-2759)
pub type CompareOpFeedbackEncodeTable =
  [u8; (CompareOperationFeedbackType::Any as u32 + 1) as usize];

impl BinaryOperationFeedbackTypeIndex {
  /// X-macro 序变体表 (calculate/transition 共用)
  const VARIANTS: [Self; BINARY_OP_NUM_TYPE_INDICES as usize] = [
    Self::None,
    Self::SignedSmall,
    Self::SignedSmallInputs,
    Self::AdditiveSafeInteger,
    Self::Number,
    Self::NumberOrOddball,
    Self::BigInt64,
    Self::BigInt,
    Self::String,
    Self::StringWrapper,
    Self::StringOrStringWrapper,
    Self::Any,
  ];

  /// 原版 DecodeTypeIndex (globals.h:2562-2572) — TypeIndex → Type 位标志
  #[inline]
  pub const fn decode_type_index(self) -> BinaryOperationFeedbackType {
    match self {
      Self::None => BinaryOperationFeedbackType::None,
      Self::SignedSmall => BinaryOperationFeedbackType::SignedSmall,
      Self::SignedSmallInputs => BinaryOperationFeedbackType::SignedSmallInputs,
      Self::AdditiveSafeInteger => BinaryOperationFeedbackType::AdditiveSafeInteger,
      Self::Number => BinaryOperationFeedbackType::Number,
      Self::NumberOrOddball => BinaryOperationFeedbackType::NumberOrOddball,
      Self::BigInt64 => BinaryOperationFeedbackType::BigInt64,
      Self::BigInt => BinaryOperationFeedbackType::BigInt,
      Self::String => BinaryOperationFeedbackType::String,
      Self::StringWrapper => BinaryOperationFeedbackType::StringWrapper,
      Self::StringOrStringWrapper => BinaryOperationFeedbackType::StringOrStringWrapper,
      Self::Any => BinaryOperationFeedbackType::Any,
    }
  }

  /// 原版 TypeIndexToString (globals.h:2574-2584)
  pub const fn type_index_to_string(self) -> &'static str {
    match self {
      Self::None => "None",
      Self::SignedSmall => "SignedSmall",
      Self::SignedSmallInputs => "SignedSmallInputs",
      Self::AdditiveSafeInteger => "AdditiveSafeInteger",
      Self::Number => "Number",
      Self::NumberOrOddball => "NumberOrOddball",
      Self::BigInt64 => "BigInt64",
      Self::BigInt => "BigInt",
      Self::String => "String",
      Self::StringWrapper => "StringWrapper",
      Self::StringOrStringWrapper => "StringOrStringWrapper",
      Self::Any => "Any",
    }
  }

  /// 原版 CombineTypeIndex (globals.h:2586-2587) — 位标志或后回落
  #[inline]
  pub const fn combine_type_index(a: Self, b: Self) -> Self {
    Self::calculate_type_index(a.decode_type_index() as u32 | b.decode_type_index() as u32)
  }

  /// 原版 CalculateTypeIndex (globals.h:2589-2600) — 按 X-macro 序首个
  /// (Type::kName & fv) == fv 的条目, 未中落 kAny
  const fn calculate_type_index(feedback_value: u32) -> Self {
    let mut i = 0;
    while i < Self::VARIANTS.len() {
      if Self::VARIANTS[i].decode_type_index() as u32 & feedback_value == feedback_value {
        return Self::VARIANTS[i];
      }
      i += 1;
    }
    Self::Any
  }

  /// 原版 BuildTransitionMap (globals.h:2606-2618) — map[i][j] = combine(i, j)
  pub const fn build_transition_map() -> BinaryOpTransitionMap {
    let mut map =
      [[0u8; BINARY_OP_TRANSITION_MAP_STRIDE as usize]; BINARY_OP_NUM_TYPE_INDICES as usize];
    let mut i = 0;
    while i < BINARY_OP_NUM_TYPE_INDICES as usize {
      let mut j = 0;
      while j < BINARY_OP_NUM_TYPE_INDICES as usize {
        map[i][j] = Self::combine_type_index(Self::VARIANTS[i], Self::VARIANTS[j]) as u8;
        j += 1;
      }
      i += 1;
    }
    map
  }

  /// 原版 BuildFeedbackEncodeTable (globals.h:2620-2632) — lut[i] = calculate(i)
  pub const fn build_feedback_encode_table() -> BinaryOpFeedbackEncodeTable {
    let mut lut = [0u8; (BinaryOperationFeedbackType::Any as u32 + 1) as usize];
    let mut i = 0u32;
    while i <= BinaryOperationFeedbackType::Any as u32 {
      lut[i as usize] = Self::calculate_type_index(i) as u8;
      i += 1;
    }
    lut
  }
}

impl CompareOperationFeedbackTypeIndex {
  /// X-macro 序变体表 (calculate/transition 共用)
  const VARIANTS: [Self; COMPARE_OP_NUM_TYPE_INDICES as usize] = [
    Self::None,
    Self::Boolean,
    Self::NullOrUndefined,
    Self::Oddball,
    Self::SignedSmall,
    Self::Number,
    Self::NumberOrBoolean,
    Self::NumberOrOddball,
    Self::InternalizedString,
    Self::String,
    Self::StringOrOddball,
    Self::Receiver,
    Self::ReceiverOrNullOrUndefined,
    Self::BigInt64,
    Self::BigInt,
    Self::Symbol,
    Self::Any,
  ];

  /// 原版 DecodeTypeIndex (globals.h:2713-2723)
  #[inline]
  pub const fn decode_type_index(self) -> CompareOperationFeedbackType {
    match self {
      Self::None => CompareOperationFeedbackType::None,
      Self::Boolean => CompareOperationFeedbackType::Boolean,
      Self::NullOrUndefined => CompareOperationFeedbackType::NullOrUndefined,
      Self::Oddball => CompareOperationFeedbackType::Oddball,
      Self::SignedSmall => CompareOperationFeedbackType::SignedSmall,
      Self::Number => CompareOperationFeedbackType::Number,
      Self::NumberOrBoolean => CompareOperationFeedbackType::NumberOrBoolean,
      Self::NumberOrOddball => CompareOperationFeedbackType::NumberOrOddball,
      Self::InternalizedString => CompareOperationFeedbackType::InternalizedString,
      Self::String => CompareOperationFeedbackType::String,
      Self::StringOrOddball => CompareOperationFeedbackType::StringOrOddball,
      Self::Receiver => CompareOperationFeedbackType::Receiver,
      Self::ReceiverOrNullOrUndefined => CompareOperationFeedbackType::ReceiverOrNullOrUndefined,
      Self::BigInt64 => CompareOperationFeedbackType::BigInt64,
      Self::BigInt => CompareOperationFeedbackType::BigInt,
      Self::Symbol => CompareOperationFeedbackType::Symbol,
      Self::Any => CompareOperationFeedbackType::Any,
    }
  }

  /// 原版 TypeIndexToString (globals.h:2725-2735)
  pub const fn type_index_to_string(self) -> &'static str {
    match self {
      Self::None => "None",
      Self::Boolean => "Boolean",
      Self::NullOrUndefined => "NullOrUndefined",
      Self::Oddball => "Oddball",
      Self::SignedSmall => "SignedSmall",
      Self::Number => "Number",
      Self::NumberOrBoolean => "NumberOrBoolean",
      Self::NumberOrOddball => "NumberOrOddball",
      Self::InternalizedString => "InternalizedString",
      Self::String => "String",
      Self::StringOrOddball => "StringOrOddball",
      Self::Receiver => "Receiver",
      Self::ReceiverOrNullOrUndefined => "ReceiverOrNullOrUndefined",
      Self::BigInt64 => "BigInt64",
      Self::BigInt => "BigInt",
      Self::Symbol => "Symbol",
      Self::Any => "Any",
    }
  }

  /// 原版 CombineTypeIndex (globals.h:2737-2739)
  #[inline]
  pub const fn combine_type_index(a: Self, b: Self) -> Self {
    Self::calculate_type_index(a.decode_type_index() as u32 | b.decode_type_index() as u32)
  }

  /// 原版 CalculateTypeIndex (globals.h:2740-2753)
  const fn calculate_type_index(feedback_value: u32) -> Self {
    let mut i = 0;
    while i < Self::VARIANTS.len() {
      if Self::VARIANTS[i].decode_type_index() as u32 & feedback_value == feedback_value {
        return Self::VARIANTS[i];
      }
      i += 1;
    }
    Self::Any
  }

  /// 原版 BuildTransitionMap (globals.h:2759-2771)
  pub const fn build_transition_map() -> CompareOpTransitionMap {
    let mut map =
      [[0u8; COMPARE_OP_TRANSITION_MAP_STRIDE as usize]; COMPARE_OP_NUM_TYPE_INDICES as usize];
    let mut i = 0;
    while i < COMPARE_OP_NUM_TYPE_INDICES as usize {
      let mut j = 0;
      while j < COMPARE_OP_NUM_TYPE_INDICES as usize {
        map[i][j] = Self::combine_type_index(Self::VARIANTS[i], Self::VARIANTS[j]) as u8;
        j += 1;
      }
      i += 1;
    }
    map
  }

  /// 原版 BuildFeedbackEncodeTable (globals.h:2773-2789)
  pub const fn build_feedback_encode_table() -> CompareOpFeedbackEncodeTable {
    let mut lut = [0u8; (CompareOperationFeedbackType::Any as u32 + 1) as usize];
    let mut i = 0u32;
    while i <= CompareOperationFeedbackType::Any as u32 {
      lut[i as usize] = Self::calculate_type_index(i) as u8;
      i += 1;
    }
    lut
  }
}

/// 原版 BinaryOperationFeedback::kTransitionMap 静态表
pub const BINARY_OP_TRANSITION_MAP: BinaryOpTransitionMap =
  BinaryOperationFeedbackTypeIndex::build_transition_map();
/// 原版 BinaryOperationFeedback::kFeedbackEncodeTable 静态表
pub const BINARY_OP_FEEDBACK_ENCODE_TABLE: BinaryOpFeedbackEncodeTable =
  BinaryOperationFeedbackTypeIndex::build_feedback_encode_table();
/// 原版 CompareOperationFeedback::kTransitionMap 静态表
pub const COMPARE_OP_TRANSITION_MAP: CompareOpTransitionMap =
  CompareOperationFeedbackTypeIndex::build_transition_map();
/// 原版 CompareOperationFeedback::kFeedbackEncodeTable 静态表
pub const COMPARE_OP_FEEDBACK_ENCODE_TABLE: CompareOpFeedbackEncodeTable =
  CompareOperationFeedbackTypeIndex::build_feedback_encode_table();

// GetTransitionMapAddress / GetFeedbackEncodeTableAddress (globals.h:2791-2794,
// 2807-2810) — 代码生成期取表地址, L4 codegen 承接 (登记)

// ── 小类型/常量 ────────────────────────────────────────────────────────────

/// 原版 GCEpoch (globals.h:1785) — base::StrongAlias<uint32_t>
pub type GCEpoch = u32;
/// 原版 kInitialGCEpoch (globals.h:1787)
pub const K_INITIAL_GC_EPOCH: GCEpoch = 0;

/// 原版 USE_ALLOCATION_ALIGNMENT_HEAP_NUMBER_BOOL (globals.h:1793) —
/// 支持平台均允许 tagged 对齐访问 double → false
pub const USE_ALLOCATION_ALIGNMENT_HEAP_NUMBER_BOOL: bool = false;

/// 原版 AllocationHint (globals.h:1622-1636) — 分配增长提示
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AllocationHint {
  may_grow: bool,
}

impl AllocationHint {
  /// 原版 WithMayGrow (globals.h:1627-1629)
  #[inline]
  pub const fn with_may_grow(self) -> Self {
    Self { may_grow: true }
  }

  /// 原版 MayGrow (globals.h:1631)
  #[inline]
  pub const fn may_grow(self) -> bool {
    self.may_grow
  }
}

/// 原版 int31_t (globals.h:3295-3311) — 值域限 int31 的 int32 载体,
/// 越界写原版 DCHECK (符号位与位 30 同值)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Int31(i32);

impl Int31 {
  pub const fn new(value: i32) -> Self {
    debug_assert!(((value as u32 & 0x8000_0000) != 0) == ((value as u32 & 0x4000_0000) != 0));
    Self(value)
  }

  /// 原版 value()
  pub const fn value(self) -> i32 {
    self.0
  }
}

/// 原版 IeeeDoubleLittleEndianArchType / IeeeDoubleBigEndianArchType
/// (globals.h:1985-2012) — C++ 双 union 按目标端序二选一; Rust 以 u64 载体 +
/// f64::to_bits/from_bits 承载 (from_f64 = d.to_bits()), 位域判读与端序无关
/// (登记)
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct IeeeDoubleArchType(u64);

impl IeeeDoubleArchType {
  /// 原版 double d 视图构造
  pub const fn from_f64(d: f64) -> Self {
    Self(d.to_bits())
  }

  pub const fn from_bits(bits: u64) -> Self {
    Self(bits)
  }

  /// 原版 ManLowField::decode — 位 0..32
  pub const fn man_low(&self) -> u32 {
    (self.0 & 0xFFFF_FFFF) as u32
  }

  /// 原版 ManHighField::decode — 位 32..52
  pub const fn man_high(&self) -> u32 {
    ((self.0 >> 32) & 0xF_FFFF) as u32
  }

  /// 原版 ExpField::decode — 位 52..63
  pub const fn exp(&self) -> u32 {
    ((self.0 >> 52) & 0x7FF) as u32
  }

  /// 原版 SignField::decode — 位 63
  pub const fn sign(&self) -> u32 {
    (self.0 >> 63) as u32
  }
}

/// 原版 WasmCodePointer::kWasmCodePointerTableEntrySize (globals.h:3331-3333) —
/// 表项 = 系统指针 + (sandbox 开 ? 8 字节签名 : 0)
pub const K_WASM_CODE_POINTER_TABLE_ENTRY_SIZE: u32 = (K_SYSTEM_POINTER_SIZE
  + if config::V8_ENABLE_SANDBOX {
    K_UINT64_SIZE
  } else {
    0
  }) as u32;

/// 原版 WasmCodePointer::kIndexSpaceSize (globals.h:3334-3339) —
/// 64 位分支: 预留空间 / 表项大小
pub const K_WASM_CODE_POINTER_INDEX_SPACE_SIZE: u32 = (K_WASM_CODE_POINTER_TABLE_RESERVATION_SIZE
  / K_WASM_CODE_POINTER_TABLE_ENTRY_SIZE as usize)
  as u32;

/// 原版 StackFrameId 枚举常量 (globals.h:3253)
pub const NO_ID: StackFrameId = 0;
pub const ID_MIN_VALUE: StackFrameId = K_MIN_INT;
pub const ID_MAX_VALUE: StackFrameId = K_MAX_INT;

// ── 对齐宏族 (globals.h:2040-2078) ─────────────────────────────────────────

/// 原版 OBJECT_POINTER_ALIGN — 按 kObjectAlignment (kTaggedSize) 对齐
#[inline]
pub const fn object_pointer_align(value: usize) -> usize {
  (value + K_OBJECT_ALIGNMENT_MASK) & !K_OBJECT_ALIGNMENT_MASK
}

/// 原版 OBJECT_POINTER_PADDING
#[inline]
pub const fn object_pointer_padding(value: usize) -> usize {
  object_pointer_align(value) - value
}

/// 原版 ALIGN_TO_ALLOCATION_ALIGNMENT (globals.h:2047-2056) — 8GB 压缩堆开时
/// 按 kObjectAlignment8GbHeap 对齐, 否则恒等 (目标配置 = false)
#[inline]
pub const fn align_to_allocation_alignment(value: usize) -> usize {
  if config::V8_COMPRESS_POINTERS_8GB {
    (value + K_OBJECT_ALIGNMENT_8GB_HEAP_MASK) & !K_OBJECT_ALIGNMENT_8GB_HEAP_MASK
  } else {
    value
  }
}

/// 原版 POINTER_SIZE_ALIGN — 按系统指针对齐
#[inline]
pub const fn pointer_size_align(value: usize) -> usize {
  (value + K_POINTER_ALIGNMENT_MASK) & !K_POINTER_ALIGNMENT_MASK
}

/// 原版 POINTER_SIZE_PADDING
#[inline]
pub const fn pointer_size_padding(value: usize) -> usize {
  pointer_size_align(value) - value
}

/// 原版 CODE_POINTER_ALIGN — 按生成代码段对齐
#[inline]
pub const fn code_pointer_align(value: usize) -> usize {
  (value + K_CODE_ALIGNMENT_MASK) & !K_CODE_ALIGNMENT_MASK
}

/// 原版 CODE_POINTER_PADDING
#[inline]
pub const fn code_pointer_padding(value: usize) -> usize {
  code_pointer_align(value) - value
}

/// 原版 DOUBLE_POINTER_ALIGN
#[inline]
pub const fn double_pointer_align(value: usize) -> usize {
  (value + K_DOUBLE_ALIGNMENT_MASK) & !K_DOUBLE_ALIGNMENT_MASK
}

// ── 原子访问 tag 派发 (globals.h:3400-3409) ────────────────────────────────

/// 原版 AcquireLoadTag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AcquireLoadTag;
/// 原版 RelaxedLoadTag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RelaxedLoadTag;
/// 原版 ReleaseStoreTag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReleaseStoreTag;
/// 原版 RelaxedStoreTag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RelaxedStoreTag;
/// 原版 SeqCstAccessTag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SeqCstAccessTag;

/// 原版 kAcquireLoad
pub const K_ACQUIRE_LOAD: AcquireLoadTag = AcquireLoadTag;
/// 原版 kRelaxedLoad
pub const K_RELAXED_LOAD: RelaxedLoadTag = RelaxedLoadTag;
/// 原版 kReleaseStore
pub const K_RELEASE_STORE: ReleaseStoreTag = ReleaseStoreTag;
/// 原版 kRelaxedStore
pub const K_RELAXED_STORE: RelaxedStoreTag = RelaxedStoreTag;
/// 原版 kSeqCstAccess
pub const K_SEQ_CST_ACCESS: SeqCstAccessTag = SeqCstAccessTag;

// ── 强类型布尔别名族 (globals.h:1609-1618, 3085, 审查环 R2 补登记) ─────────
// 原版 base::StrongAlias<Tag, bool> — 沿用本文件既有约定 (JsDispatchHandle /
// AdaptArguments / NeedsContext): Rust 直取底层类型, 区分交由调用方命名参数
// (登记)

/// 原版 AllowAllocation (globals.h:1609)
pub type AllowAllocation = bool;
/// 原版 PerformHeapLimitCheck (globals.h:1612-1613)
pub type PerformHeapLimitCheck = bool;
/// 原版 PerformIneffectiveMarkCompactCheck (globals.h:1614-1615)
pub type PerformIneffectiveMarkCompactCheck = bool;
/// 原版 SharedFlag (globals.h:3085)
pub type SharedFlag = bool;

// ── 汇编器调试信息 (globals.h:2930-2944) ──────────────────────────────────

/// 原版 AssemblerDebugInfo (globals.h:2930-2937) — 汇编器源位置标注
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssemblerDebugInfo {
  /// 原版 name — const char*
  pub name: &'static str,
  /// 原版 file — const char*
  pub file: &'static str,
  /// 原版 line — int
  pub line: i32,
}

/// 原版 FileAndLine (globals.h:2942) — (文件, 行号) 二元组
pub type FileAndLine = (&'static str, i32);

// ── 函数地址互转 (globals.h:793-805) ──────────────────────────────────────

/// 原版 FUNCTION_ADDR(f) (globals.h:793-794) — 函数指针取内部地址。
/// Rust 以宏承载: 泛型函数项无法经统一签名转型, 调用点 `$f as usize`
/// 对函数项/函数指针均成立
#[macro_export]
macro_rules! function_addr {
  ($f:expr) => {
    $f as usize
  };
}

/// 原版 FUNCTION_CAST<F>(addr) (globals.h:796-805) — 内部地址转 F 签名
/// 函数指针 (调生成代码用)
///
/// # SAFETY
/// 调用方必须保证 `addr` 指向与 `F` 签名兼容的有效可执行代码 (原版 reinterpret_cast)
#[inline]
pub unsafe fn function_cast<F>(addr: Address) -> F {
  assert_eq!(
    size_of::<F>(),
    size_of::<Address>(),
    "FUNCTION_CAST 目标类型须与地址等宽",
  );
  // SAFETY: 调用方保证 addr 指向 F 签名的可执行代码 (原版 reinterpret_cast);
  // transmute_copy 承载泛型 F 的依赖尺寸, 尺寸契约由上方 assert_eq 锁定
  unsafe { transmute_copy(&addr) }
}

// ── 配置性裁剪登记 (原版存在, 目标配置关闭未落位) ───────────────────────────
// - kContiguousReadOnlyReservationSize / kContiguousReadOnlySpaceMask
//   (globals.h:829-856, CONTIGUOUS_COMPRESSED_READ_ONLY_SPACE 关)
// - kPointerSize / kPointerSizeLog2 (globals.h:674-676, 压缩开时不定义)
// - kUndefinedNan 族 + HoleNan() / UndefinedNan() / IsUndefinedNan()
//   (globals.h:2182-2209, V8_ENABLE_UNDEFINED_DOUBLE 关)
// - 32 位宿主分支常量 (globals.h:552-570)
// - kJSDispatchTable* 的 iOS / LA57 变体 (globals.h:617-637, 645-650)
// - SlotHoldsTrustedPointerV (globals.h:1464-1481, 依赖 slot 类型层, L3 落位)
// - Union 类型别名族 (globals.h:1350-1390): MaybeWeak<T> / Zero / NaN /
//   Number / Numeric / JSPrimitive / JSAny / JSAnyNotSmi / JSAnyNotNumeric /
//   JSAnyNotNumber / JSCallable / JSAnyOrSharedFunctionInfo / JSPrototype /
//   MaybeObject / HeapObjectReference / JSObjectOrUndefined — 依赖
//   Union<T,...> / Weak<T> 与 Smi / HeapNumber / String / Symbol /
//   JSReceiver / SharedFunctionInfo 对象类型, L3 v8-objects-core 承接
// - SlotTraits + 槽别名族 (globals.h:1392-1462): TObjectSlot 等七槽位别名 +
//   WeakSlotCallback / WeakSlotCallbackWithHeap 函数指针类型 — 依赖
//   CompressedObjectSlot / FullObjectSlot 槽类型, L3 承接
// - 已落位 (审查环 R2): AllowAllocation / PerformHeapLimitCheck /
//   PerformIneffectiveMarkCompactCheck (globals.h:1609-1618) /
//   SharedFlag (globals.h:3085) / AssemblerDebugInfo + FileAndLine
//   (globals.h:2930-2944) / FUNCTION_ADDR + FUNCTION_CAST (globals.h:793-805)
//
// 裁剪登记 (审查环 round1 增补):
// - AsAtomicTagged (globals.h:666) = base::AsAtomicPointerImpl<AtomicTagged_t>
//   — 原子指针包装属 src/base 层, L3 v8-base 承接
// - RuntimeArguments / JavaScriptArguments (globals.h:1212-1214) =
//   Arguments<ArgumentsType::kRuntime/kJS> — Arguments 模板类仅前向声明
//   (实现在 src/execution/arguments.h), L3 execution 层承接; 枚举
//   ArgumentsType 已落位
// - hash_value(LanguageMode) (globals.h:867) — absl 哈希定制, Rust 由
//   derive(Hash) 等价覆盖 (bool 枚举判别值 0/1 一致)
