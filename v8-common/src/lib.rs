#![cfg_attr(docsrs, feature(doc_cfg))]
//! v8-common — V8 src/common 的 Rust 移植 (L0 地基层)
//!
//! 原版 ~8k 行: globals / message-template / assert-scope / ptr-compr /
//! code-memory-access 等。目标配置: x64 + arm64, COMPRESS_POINTERS +
//! SHARED_CAGE + SANDBOX + EXTERNAL_CODE_SPACE 开 (见 `config` 模块)。
//! 原版 CHECK 失败语义 → `Result` 落 `Error` (禁 panic, 每处登记见模块注释)。

mod assert_scope;
mod checks;
mod code_memory_access;
mod error;
mod globals;
mod high_allocation_throughput_scope;
mod internal;
mod message_template;
mod operation;
mod ptr_compr;
mod scoped_modification;
mod simd128;
mod synchronization_point_support;
mod thread_local_storage;

// ── assert-scope ───────────────────────────────────────────────────────────
pub use assert_scope::{
  AllowCodeAllocation, AllowCodeDependencyChange, AllowCompilationDebugOnly,
  AllowDeoptimizationDebugOnly, AllowExceptionsDebugOnly, AllowGarbageCollection,
  AllowGarbageCollectionInRelease, AllowHandleAllocation, AllowHandleDereference,
  AllowHandleUsageOnAllThreads, AllowHeapAccess, AllowHeapAllocation, AllowHeapAllocationInRelease,
  AllowJavascriptExecutionDebugOnly, AllowPositionInfoSlow, AllowSafepoints, DisableGCMole,
  DisallowCodeAllocation, DisallowCodeDependencyChange, DisallowCompilationDebugOnly,
  DisallowDeoptimizationDebugOnly, DisallowExceptionsDebugOnly, DisallowGarbageCollection,
  DisallowGarbageCollectionInRelease, DisallowHandleAllocation, DisallowHandleDereference,
  DisallowHeapAccess, DisallowHeapAccessIf, DisallowHeapAllocation,
  DisallowJavascriptExecutionDebugOnly, DisallowPositionInfoSlow, DisallowSafepoints,
  K_CLEARED_VALUE, K_INITIAL_VALUE, NoGarbageCollectionMutexGuard, PER_ISOLATE_ASSERT_TYPES,
  PerIsolateAssertScope, PerIsolateAssertState, PerIsolateAssertType, PerThreadAssertScope,
  PerThreadAssertScopeDebugOnly, PerThreadAssertScopeEmpty, PerThreadAssertType, PerThreadAsserts,
  per_thread_asserts_allow,
};
// ── 其余模块 ───────────────────────────────────────────────────────────────
pub use checks::{
  dcheck_size_tag_aligned, dcheck_tag_aligned, dcheck_with_sandbox_access,
  dcheck_with_sandbox_access_and_msg, slow_dcheck, slow_dcheck_implies,
};
// ── code-memory-access ─────────────────────────────────────────────────────
// flags 为 pub(crate) 模块, 仅经此处平铺导出 (禁 pub mod)
pub use code_memory_access::flags::{
  force_memory_protection_keys, jitless, memory_protection_keys, set_force_memory_protection_keys,
  set_jitless, set_memory_protection_keys,
};
pub use code_memory_access::{
  CFIMetadataWriteScope, DiscardSealedMemoryScope, JitAllocation, JitAllocationType, JitPageRef,
  K_SPLIT_THRESHOLD, NopRwxMemoryWriteScope, PagePermissions, RwxMemoryWriteScope,
  RwxMemoryWriteScopeForTesting, THREAD_ISOLATION_ALIGN_SZ, ThreadIsolatedAllocator,
  ThreadIsolation, WritableFreeSpace, WritableJitAllocation, WritableJitPage,
  WritableJumpTablePair, thread_isolation_fill_page_sz,
};
pub use error::{Error, Result};
// ── globals::config (配置布尔展开) ─────────────────────────────────────────
pub use globals::config::{
  COMPRESS_POINTERS, COMPRESS_POINTERS_IN_MULTIPLE_CAGES, COMPRESS_POINTERS_IN_SHARED_CAGE,
  CONTIGUOUS_COMPRESSED_READ_ONLY_SPACE, DEBUG_BOOL, DECOMPRESS_POINTER_BY_ADDRESSING_MODE,
  ENABLE_CONTROL_FLOW_INTEGRITY, SHORT_BUILTIN_CALLS, TAGGED_SIZE_8_BYTES, USE_SIMULATOR,
  USES_FUNCTION_DESCRIPTORS, V8_CAN_CREATE_SHARED_HEAP, V8_COMPRESS_POINTERS_8GB,
  V8_DICT_PROPERTY_CONST_TRACKING, V8_EMBEDDED_CONSTANT_POOL, V8_ENABLE_FP_PARAMS_IN_C_LINKAGE,
  V8_ENABLE_GEARBOX, V8_ENABLE_NEAR_CODE_RANGE, V8_ENABLE_SANDBOX, V8_ENABLE_SWISS_NAME_DICTIONARY,
  V8_EXPERIMENTAL_TQ_TO_TSA, V8_EXPERIMENTAL_TSA_BUILTINS, V8_EXTERNAL_CODE_SPACE,
  V8_HEAP_USE_BECORE_JIT_WRITE_PROTECT, V8_HEAP_USE_PKU_JIT_WRITE_PROTECT,
  V8_HEAP_USE_PTHREAD_JIT_WRITE_PROTECT, V8_JS_LINKAGE_INCLUDES_DISPATCH_HANDLE,
  V8_JUMP_TABLE_INFO, V8_LOWER_LIMITS_MODE, V8_MAP_PACKING, V8_OS_WIN_ARM64, V8_OS_WIN_X64,
  V8_OS_WIN64, V8_STATIC_DISPATCH_HANDLES, V8_STATIC_ROOTS_GENERATION, V8_UNDEFINED_DOUBLE,
  V8_X64_16BYTE_STACK_ALIGNMENT,
};
// ── globals (函数地址互转, 审查环 R2) ──────────────────────────────────────
pub use globals::function_cast;
// ── globals (枚举 + 别名) ──────────────────────────────────────────────────
pub use globals::{
  AccessMode, AcquireLoadTag, AdaptArguments, AddStringConstantAndInternalizeVariant, AliasingKind,
  AllocationAlignment, AllocationHint, AllocationSiteMode, AllocationSiteUpdateMode,
  AllocationSpace, AllocationType, AllowAllocation, ArgumentsType, ArgvMode, AssemblerDebugInfo,
  AtomicTaggedT, BinaryOpFeedbackEncodeTable, BinaryOpTransitionMap, BinaryOperationFeedbackType,
  BinaryOperationFeedbackTypeIndex, BlockingBehavior, BranchHint, CachedTieringDecision,
  CallApiCallbackMode, CallFeedbackContent, CallJumpMode, CodeFlushMode, CodeFlushModeSet,
  CollectionKind, CompactionSpaceKind, CompareOpFeedbackEncodeTable, CompareOpTransitionMap,
  CompareOperationFeedbackType, CompareOperationFeedbackTypeIndex, ConcurrencyMode, ContextMode,
  ConvertReceiverMode, CreateArgumentsType, DefineKeyedOwnPropertyFlag,
  DefineKeyedOwnPropertyFlags, DefineKeyedOwnPropertyInLiteralFlag,
  DefineKeyedOwnPropertyInLiteralFlags, DeoptimizeKind, ExceptionStatus, Executability,
  ExtendedMapKind, ExternalArrayType, FileAndLine, ForInFeedback, GCEpoch, GarbageCollectionReason,
  GarbageCollector, GotoHint, HeapGrowingMode, HeapObjectReferenceType, IcCheckType,
  IeeeDoubleArchType, IndirectPointerMode, InitializationFlag, InlineCacheState, Int31,
  InterceptorKind, InterceptorResult, InterpreterPushArgsMode, IsStaticFlag,
  IsolateExecutionModeFlag, IterationKind, JsDispatchHandle, KeyedAccessLoadMode,
  KeyedAccessStoreMode, LanguageMode, LookupHoistingMode, MaybeAssignedFlag, MinimumCapacity,
  MutableMode, NativesFlag, NeedsContext, NewJSObjectType, OrdinaryToPrimitiveHint, PageSize,
  ParseRestriction, ParsingWhileDebugging, PerformHeapLimitCheck,
  PerformIneffectiveMarkCompactCheck, PrimitiveType, PropertiesEnumerationMode, REPLMode,
  RecordYoungSlot, RelaxedLoadTag, RelaxedStoreTag, ReleaseStoreTag, RequestedGCKind,
  ResultSentinel, SaveFPRegsMode, ScopeType, ScriptEventType, SeqCstAccessTag, SharedFlag,
  ShouldThrow, SilenceNanMode, SpeculationMode, StackFrameId, StoreOrigin,
  StringTransitionStrategy, StubCallMode, TaggedT, ThreadKind, ToPrimitiveHint, TrustedPointerT,
  TypeOfFeedbackResult, TypedArrayAccessMode, TypeofMode, UnicodeEncoding, VariableAllocationInfo,
  VariableKind, VariableLocation, VariableMode, WasmTrapReason, WhereToStart,
};
// ── globals (常量) ─────────────────────────────────────────────────────────
pub use globals::{
  BINARY_OP_FEEDBACK_ENCODE_TABLE, BINARY_OP_FIRST_TYPE_INDEX, BINARY_OP_LAST_TYPE_INDEX,
  BINARY_OP_NUM_TYPE_INDICES, BINARY_OP_TRANSITION_MAP, BINARY_OP_TRANSITION_MAP_STRIDE,
  COMPARE_OP_ANY_MASK, COMPARE_OP_BIG_INT64_FLAG, COMPARE_OP_BOOLEAN_FLAG,
  COMPARE_OP_FEEDBACK_ENCODE_TABLE, COMPARE_OP_FIRST_TYPE_INDEX,
  COMPARE_OP_INTERNALIZED_STRING_FLAG, COMPARE_OP_LAST_TYPE_INDEX,
  COMPARE_OP_NULL_OR_UNDEFINED_FLAG, COMPARE_OP_NUM_TYPE_INDICES, COMPARE_OP_OTHER_BIG_INT_FLAG,
  COMPARE_OP_OTHER_NUMBER_FLAG, COMPARE_OP_OTHER_STRING_FLAG, COMPARE_OP_RECEIVER_FLAG,
  COMPARE_OP_SIGNED_SMALL_FLAG, COMPARE_OP_SYMBOL_FLAG, COMPARE_OP_TRANSITION_MAP,
  COMPARE_OP_TRANSITION_MAP_STRIDE, CONVERT_RECEIVER_MODE_LAST, DEOPTIMIZE_KIND_COUNT,
  FIRST_DEOPTIMIZE_KIND, FIRST_GROWABLE_PAGED_SPACE, FIRST_IMMUTABLE_LEXICAL_VARIABLE_MODE,
  FIRST_MUTABLE_SPACE, FIRST_SPACE, FIRST_SWEEPABLE_SPACE, ID_MAX_VALUE, ID_MIN_VALUE,
  K_ACQUIRE_LOAD, K_ADAPT, K_ADD_STRING_CONSTANT_INTERNALIZE_CACHE_SLOT_OFFSET,
  K_ADDITIVE_SAFE_INTEGER_FEEDBACK_BIT_LENGTH, K_ADDITIVE_SAFE_INTEGER_FEEDBACK_SHIFT,
  K_BINARY32_EXPONENT_BIAS, K_BINARY32_EXPONENT_MASK, K_BINARY32_EXPONENT_SHIFT,
  K_BINARY32_MANTISSA_BITS, K_BINARY32_MANTISSA_MASK, K_BINARY32_MAX_EXPONENT,
  K_BINARY32_MIN_EXPONENT, K_BINARY32_SIGN_MASK, K_BITS_PER_BYTE, K_BITS_PER_BYTE_LOG2,
  K_BITS_PER_INT, K_BITS_PER_SYSTEM_POINTER, K_BITS_PER_SYSTEM_POINTER_LOG2, K_BYTE_SIZE,
  K_CHAR_SIZE, K_CLEARED_FREE_MEMORY_VALUE, K_CLEARED_WEAK_HEAP_OBJECT_LOWER32, K_CODE_ALIGNMENT,
  K_CODE_ALIGNMENT_BITS, K_CODE_ALIGNMENT_MASK, K_CODE_POINTER_SIZE, K_CODE_ZAP_VALUE,
  K_CPP_HEAP_POINTER_SLOT_SIZE, K_DEBUG_ZAP_VALUE, K_DEFAULT_MAX_WASM_CODE_SPACE_SIZE_MB,
  K_DONT_ADAPT, K_DONT_ADAPT_ARGUMENTS_SENTINEL, K_DOUBLE_ALIGNMENT, K_DOUBLE_ALIGNMENT_MASK,
  K_DOUBLE_SIZE, K_DOUBLE_SIZE_LOG2, K_ELIDED_FRAME_SLOTS, K_EMBEDDED_FEEDBACK_OPERAND_INDEX,
  K_EMBEDDER_DATA_SLOT_SIZE, K_EMBEDDER_DATA_SLOT_SIZE_IN_TAGGED_SLOTS,
  K_EXTERNAL_ALLOCATION_SOFT_LIMIT, K_EXTERNAL_POINTER_SLOT_SIZE, K_FEEDBACK_IS_EMBEDDED,
  K_FLOAT_SIZE, K_FLOAT16_SIZE, K_FP_ON_STACK_SIZE, K_FREE_LIST_ZAP_VALUE, K_FROM_SPACE_ZAP_VALUE,
  K_FUNCTION_ENTRY_BYTECODE_OFFSET, K_FUNCTION_EXIT_BYTECODE_OFFSET,
  K_FUNCTION_LITERAL_ID_TOP_LEVEL, K_GLOBAL_HANDLE_ZAP_VALUE, K_HANDLE_ZAP_VALUE, K_HOLE_NAN_INT64,
  K_HOLE_NAN_LOWER32, K_HOLE_NAN_UPPER32, K_IEEE_DOUBLE_EXPONENT_WORD_OFFSET,
  K_IEEE_DOUBLE_MANTISSA_WORD_OFFSET, K_INDIRECT_POINTER_SIZE, K_INITIAL_GC_EPOCH, K_INT_SIZE,
  K_INT8_SIZE, K_INT16_SIZE, K_INT32_SIZE, K_INT64_SIZE, K_INTERCEPTED_NO, K_INTERCEPTED_SIZE,
  K_INTERCEPTED_YES, K_INTPTR_SIGN_BIT, K_INTPTR_SIZE, K_INVALID_DISPATCH_HANDLE,
  K_INVALID_INFO_ID, K_INVALID_WASM_SIGNATURE_HASH, K_IS_SMI_VALUE_IN_LOWER_32_BITS,
  K_IS_SMI_VALUE_IN_UPPER_32_BITS, K_ISOLATE_DATA_ALIGNMENT, K_JS_ARGC_RECEIVER_SLOTS,
  K_JS_CALL_CLOSURE_PARAMETER_INDEX, K_JS_DISPATCH_HANDLE_SHIFT, K_JS_DISPATCH_HANDLE_SIZE,
  K_JS_DISPATCH_TABLE_ENTRY_SIZE, K_JS_DISPATCH_TABLE_ENTRY_SIZE_LOG2,
  K_JS_DISPATCH_TABLE_RESERVATION_SIZE, K_MAX_ADDITIVE_SAFE_INTEGER,
  K_MAX_ADDITIVE_SAFE_INTEGER_FEEDBACK, K_MAX_COMMITTED_WASM_CODE_MB, K_MAX_DOUBLE_STRING_LENGTH,
  K_MAX_INT, K_MAX_INT8, K_MAX_INT16, K_MAX_INT31, K_MAX_JS_DISPATCH_ENTRIES,
  K_MAX_REGULAR_HEAP_OBJECT_SIZE, K_MAX_SAFE_INTEGER, K_MAX_SAFE_INTEGER_UINT64, K_MAX_UINT8,
  K_MAX_UINT16, K_MAX_UINT32, K_MAX_UINT32_DOUBLE, K_MAX_UINT64, K_MAXIMAL_CODE_RANGE_SIZE,
  K_MAXIMAL_TRUSTED_RANGE_SIZE, K_MIN_ADDITIVE_SAFE_INTEGER, K_MIN_ADDITIVE_SAFE_INTEGER_FEEDBACK,
  K_MIN_EXPECTED_OS_PAGE_SIZE, K_MIN_INT, K_MIN_INT8, K_MIN_INT16, K_MIN_INT31,
  K_MIN_PARAMETER_INDEX, K_MIN_SAFE_INTEGER, K_MIN_UINT8, K_MIN_UINT16, K_MIN_UINT32, K_MIN_UINT64,
  K_MINIMUM_CODE_RANGE_SIZE, K_MINIMUM_TRUSTED_RANGE_SIZE, K_NO_DEOPTIMIZATION_ID,
  K_NO_SOURCE_POSITION, K_NOT_INTERCEPTED_SENTINEL, K_NULL_JS_DISPATCH_HANDLE, K_OBJECT_ALIGNMENT,
  K_OBJECT_ALIGNMENT_8GB_HEAP, K_OBJECT_ALIGNMENT_8GB_HEAP_MASK, K_OBJECT_ALIGNMENT_BITS,
  K_OBJECT_ALIGNMENT_MASK, K_ONE_BYTE_SIZE, K_PC_ON_STACK_SIZE, K_PERSISTENT_HANDLE_ZAP_VALUE,
  K_PHANTOM_REFERENCE_ZAP, K_PLACEHOLDER_DISPATCH_HANDLE, K_PLATFORM_REQUIRES_CODE_RANGE,
  K_POINTER_ALIGNMENT, K_POINTER_ALIGNMENT_MASK, K_POINTER_TABLE_ADDRESS_SPACE_NAME,
  K_PREALLOCATED_NUMBER_STRING_TABLE_SIZE, K_PROTECTED_POINTER_SIZE, K_QUIET_NAN_HIGH_BITS_MASK,
  K_QUIET_NAN_MASK, K_RELAXED_LOAD, K_RELAXED_STORE, K_RELEASE_STORE, K_RESERVED_CODE_RANGE_PAGES,
  K_SCOPE_INFO_MAX_INLINED_LOCAL_NAMES_SIZE, K_SEQ_CST_ACCESS,
  K_SHORT_BUILTIN_CALLS_OLD_SPACE_SIZE_THRESHOLD, K_SHORT_SIZE, K_SIMD128_HALF_SIZE,
  K_SIMD128_QUARTER_SIZE, K_SIMD128_SIZE, K_SIMD256_SIZE, K_SIZET_SIZE, K_SLOTS_ZAP_VALUE,
  K_SMALL_ORDERED_HASH_MAP_MIN_CAPACITY, K_SMALL_ORDERED_HASH_SET_MIN_CAPACITY, K_SMI_HASH_MASK,
  K_SMI_SIGN_MASK, K_SPACE_TAG_SIZE, K_STACK_LIMIT_SLACK_FOR_DEOPTIMIZATION_IN_BYTES,
  K_STACK_SPACE_REQUIRED_FOR_COMPILATION, K_SWISS_NAME_DICTIONARY_INITIAL_CAPACITY,
  K_SYSTEM_POINTER_HEX_DIGITS, K_SYSTEM_POINTER_SIZE, K_SYSTEM_POINTER_SIZE_LOG2, K_TAGGED_SIZE,
  K_TAGGED_SIZE_LOG2, K_TIERING_STATE_IN_PROGRESS_BLOCKS_TIERUP,
  K_TRACED_HANDLE_EAGER_RESET_ZAP_VALUE, K_TRACED_HANDLE_FULL_GC_RESET_ZAP_VALUE,
  K_TRACED_HANDLE_MINOR_GC_RESET_ZAP_VALUE, K_TRACED_HANDLE_MINOR_GC_WEAK_RESET_ZAP_VALUE,
  K_TRUSTED_POINTER_SIZE, K_UINT8_SIZE, K_UINT16_SIZE, K_UINT32_SIZE, K_UINT64_SIZE,
  K_UINTPTR_SIZE, K_UNARY_EMBEDDED_FEEDBACK_OPERAND_INDEX, K_UNINITIALIZED_EMBEDDED_FEEDBACK,
  K_WASM_CODE_POINTER_INDEX_SPACE_SIZE, K_WASM_CODE_POINTER_TABLE_ENTRY_SIZE,
  K_WEAK_HEAP_OBJECT_MASK, K_ZAP_VALUE, LANGUAGE_MODE_SIZE, LAST_ALLOCATION_SITE_MODE,
  LAST_DEOPTIMIZE_KIND, LAST_GROWABLE_PAGED_SPACE, LAST_LEXICAL_VARIABLE_MODE, LAST_MUTABLE_SPACE,
  LAST_SPACE, LAST_SWEEPABLE_SPACE, LAST_VARIABLE_LOCATION, NO_ID, PROCESSOR_CACHE_LINE_SIZE,
  USE_ALLOCATION_ALIGNMENT_HEAP_NUMBER_BOOL, V8_DEFAULT_STACK_SIZE_KB, V8_STACK_LIMIT_MARGIN_KB,
};
// ── globals (助记函数, 审查环 R1) ──────────────────────────────────────────
pub use globals::{
  align_to_allocation_alignment, code_pointer_align, code_pointer_padding, construct_language_mode,
  construct_repl_mode, double_pointer_align, immutable_lexical_variable_mode_to_string,
  is_any_code_space, is_any_large_space, is_any_new_space, is_any_trusted_space,
  is_any_writable_shared_space, is_baseline_code_flushing_enabled, is_byte_code_flushing_enabled,
  is_flushing_disabled, is_force_flushing_enabled, is_sloppy, is_strict, is_valid_language_mode,
  js_parameter_count, object_hash, object_pointer_align, object_pointer_padding,
  pointer_size_align, pointer_size_padding, static_strings_equal, stricter_language_mode,
};
pub use high_allocation_throughput_scope::{
  HighAllocationThroughputObserver, HighAllocationThroughputScope,
};
// ── internal (v8-internal.h 常量层) ────────────────────────────────────────
pub use internal::{
  Address, GB, K_API_DOUBLE_SIZE, K_API_INT32_SIZE, K_API_INT64_SIZE, K_API_SYSTEM_POINTER_SIZE,
  K_API_TAGGED_SIZE, K_CPP_HEAP_POINTER_HANDLE_SIZE, K_DISALLOW_GARBAGE_COLLECTION_ALIGN,
  K_DISALLOW_GARBAGE_COLLECTION_SIZE, K_DONT_THROW, K_EXTERNAL_POINTER_HANDLE_SIZE,
  K_GARBAGE_COLLECTION_REASON_MAX_VALUE, K_HEAP_OBJECT_REFERENCE_TAG_MASK, K_HEAP_OBJECT_TAG,
  K_HEAP_OBJECT_TAG_MASK, K_HEAP_OBJECT_TAG_SIZE, K_INDIRECT_POINTER_HANDLE_SIZE, K_NULL_ADDRESS,
  K_PAGE_SIZE_BITS, K_PTR_COMPR_CAGE_BASE_ALIGNMENT, K_PTR_COMPR_CAGE_RESERVATION_SIZE,
  K_REGULAR_PAGE_SIZE, K_SMI_SHIFT_SIZE, K_SMI_TAG, K_SMI_TAG_MASK, K_SMI_TAG_SIZE,
  K_SMI_VALUE_SIZE, K_THROW_ON_ERROR, K_WASM_CODE_POINTER_TABLE_RESERVATION_SIZE,
  K_WEAK_HEAP_OBJECT_TAG, KB, MB, SMI_VALUES_ARE_31_BITS, SMI_VALUES_ARE_32_BITS, V8_INFINITY,
};
// ── message-template ───────────────────────────────────────────────────────
pub use message_template::{
  JS_IMMUTABLE_ARRAYBUFFER, K_MESSAGE_COUNT, MESSAGE_TEMPLATE_STRINGS, MessageTemplate,
  message_template_from_int, template_string,
};
// ── operation ──────────────────────────────────────────────────────────────
pub use operation::Operation;
// ── ptr-compr ──────────────────────────────────────────────────────────────
pub use ptr_compr::{
  CompressionCage, ExternalCodeCompressionScheme, MainCage, PtrComprCageAccessScope,
  PtrComprCageBase, SmiCompressionScheme, TrustedCage, TrustedSpaceCompressionScheme,
  V8HeapCompressionScheme, V8HeapCompressionSchemeImpl, get_ptr_compr_cage_base,
  get_ptr_compr_cage_base_for_object, get_ptr_compr_cage_base_from_on_heap_address,
  has_heap_object_tag, has_smi_tag, has_weak_heap_object_tag, read_maybe_unaligned_value,
  write_maybe_unaligned_value,
};
pub use scoped_modification::ScopedModification;
pub use simd128::{Float32x4, Float64x2, Int8x16, Int16x8, Int32x4, Int64x2, Simd128};
pub use synchronization_point_support::{
  SynchronizationPointSupport, TEST_ONLY_SYNC_POINTS, synchronization_point,
  synchronization_point_test_only,
};
pub use thread_local_storage::{V8_TLS_LIBRARY_MODE, V8_TLS_MODEL};
