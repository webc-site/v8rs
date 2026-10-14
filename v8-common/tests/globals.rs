//! globals 枚举判别值与常量关系校验
//!
//! 只测跨字段关系与判别值对数 (对应原版 static_assert 族与 X-macro 展开序),
//! 不做常量等于字面量的废话断言。

use core::mem::size_of;

use aok::{OK, Void};
use v8_common::{
  AllocationSpace, BINARY_OP_NUM_TYPE_INDICES, BINARY_OP_TRANSITION_MAP_STRIDE,
  COMPARE_OP_ANY_MASK, COMPARE_OP_BOOLEAN_FLAG, COMPARE_OP_INTERNALIZED_STRING_FLAG,
  COMPARE_OP_NULL_OR_UNDEFINED_FLAG, COMPARE_OP_NUM_TYPE_INDICES, COMPARE_OP_OTHER_NUMBER_FLAG,
  COMPARE_OP_OTHER_STRING_FLAG, COMPARE_OP_SIGNED_SMALL_FLAG, COMPARE_OP_TRANSITION_MAP_STRIDE,
  CollectionKind, CompareOperationFeedbackType, ConvertReceiverMode, CreateArgumentsType,
  DEOPTIMIZE_KIND_COUNT, DefineKeyedOwnPropertyFlag, DefineKeyedOwnPropertyFlags,
  DefineKeyedOwnPropertyInLiteralFlag, DefineKeyedOwnPropertyInLiteralFlags, DeoptimizeKind,
  ExternalArrayType, FIRST_SPACE, ForInFeedback, GarbageCollectionReason, IeeeDoubleArchType,
  InterpreterPushArgsMode, IterationKind, K_API_TAGGED_SIZE, K_DONT_THROW,
  K_EMBEDDER_DATA_SLOT_SIZE, K_GARBAGE_COLLECTION_REASON_MAX_VALUE,
  K_IS_SMI_VALUE_IN_LOWER_32_BITS, K_IS_SMI_VALUE_IN_UPPER_32_BITS, K_JS_ARGC_RECEIVER_SLOTS,
  K_JS_DISPATCH_HANDLE_SHIFT, K_JS_DISPATCH_TABLE_ENTRY_SIZE, K_JS_DISPATCH_TABLE_RESERVATION_SIZE,
  K_MAX_ADDITIVE_SAFE_INTEGER, K_MAX_JS_DISPATCH_ENTRIES, K_MAX_SAFE_INTEGER,
  K_MIN_ADDITIVE_SAFE_INTEGER, K_MIN_SAFE_INTEGER, K_SYSTEM_POINTER_SIZE,
  K_SYSTEM_POINTER_SIZE_LOG2, K_TAGGED_SIZE, K_TAGGED_SIZE_LOG2, K_THROW_ON_ERROR,
  K_TRUSTED_POINTER_SIZE, KeyedAccessLoadMode, KeyedAccessStoreMode, LANGUAGE_MODE_SIZE,
  LAST_GROWABLE_PAGED_SPACE, LAST_SPACE, LookupHoistingMode, RequestedGCKind, ResultSentinel,
  SMI_VALUES_ARE_31_BITS, SMI_VALUES_ARE_32_BITS, ShouldThrow, SilenceNanMode, SpeculationMode,
  TAGGED_SIZE_8_BYTES, TrustedPointerT, TypedArrayAccessMode, UnicodeEncoding, VariableMode,
  function_cast, js_parameter_count, static_strings_equal,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 原版 static_assert 族: Smi 布局 / tagged 尺寸 / dispatch 表自洽
#[test]
fn globals_static_asserts() -> Void {
  // globals.h:651 — (1 << (32 - shift)) == entries
  assert_eq!(
    1usize << (32 - K_JS_DISPATCH_HANDLE_SHIFT),
    K_MAX_JS_DISPATCH_ENTRIES
  );
  assert_eq!(
    K_JS_DISPATCH_TABLE_RESERVATION_SIZE / K_JS_DISPATCH_TABLE_ENTRY_SIZE,
    K_MAX_JS_DISPATCH_ENTRIES
  );
  // globals.h:573/663 — size == (1 << log2)
  assert_eq!(K_SYSTEM_POINTER_SIZE, 1 << K_SYSTEM_POINTER_SIZE_LOG2);
  assert_eq!(K_TAGGED_SIZE, 1 << K_TAGGED_SIZE_LOG2);
  // globals.h:670 — kTaggedSize == kApiTaggedSize
  assert_eq!(K_TAGGED_SIZE, K_API_TAGGED_SIZE);
  // globals.h:664 — (kTaggedSize == 8) == TAGGED_SIZE_8_BYTES
  assert_eq!(K_TAGGED_SIZE == 8, TAGGED_SIZE_8_BYTES);
  // globals.h:743 — 编译期自洽, 常量块承载
  const { assert!(K_EMBEDDER_DATA_SLOT_SIZE >= K_SYSTEM_POINTER_SIZE) }
  // globals.h:1071-1076 — SmiValuesAre32Bits/31Bits 一致性
  assert_ne!(SMI_VALUES_ARE_32_BITS, SMI_VALUES_ARE_31_BITS);
  assert_eq!(SMI_VALUES_ARE_32_BITS, K_IS_SMI_VALUE_IN_UPPER_32_BITS);
  assert_eq!(SMI_VALUES_ARE_31_BITS, K_IS_SMI_VALUE_IN_LOWER_32_BITS);
  // globals.h:1696 — kGarbageCollectionReasonMaxValue == NUM_REASONS - 1
  assert_eq!(
    K_GARBAGE_COLLECTION_REASON_MAX_VALUE,
    GarbageCollectionReason::NumReasons as i32 - 1
  );
  // globals.h:2223-2226 — additive safe integer 与 safe integer 的双倍自洽
  assert!(2.0 * K_MAX_ADDITIVE_SAFE_INTEGER as f64 <= K_MAX_SAFE_INTEGER);
  assert!(2.0 * K_MIN_ADDITIVE_SAFE_INTEGER as f64 >= K_MIN_SAFE_INTEGER);
  OK
}

/// 枚举判别值对数: 显式标注/位标志类逐个校验
#[test]
fn globals_enum_discriminants() -> Void {
  // ResultSentinel (globals.h:1963)
  assert_eq!(ResultSentinel::NotFound as i32, -1);
  assert_eq!(ResultSentinel::Unsupported as i32, -2);
  // ShouldThrow (globals.h:1965, 值来自 Internals)
  assert_eq!(ShouldThrow::DontThrow as i32, K_DONT_THROW);
  assert_eq!(ShouldThrow::ThrowOnError as i32, K_THROW_ON_ERROR);
  // RequestedGCKind (globals.h:1620)
  assert_eq!(RequestedGCKind::Major as u8, 1);
  assert_eq!(RequestedGCKind::LastResort as u8, 1 << 1);
  // KeyedAccessLoadMode (globals.h:3126)
  assert_eq!(KeyedAccessLoadMode::HandleOOBAndHoles as u8, 0b11);
  // SpeculationMode (globals.h:3041)
  assert_eq!(SpeculationMode::DisallowSpeculation as u8, 3);
  // ForInFeedback (globals.h:2812)
  assert_eq!(ForInFeedback::EnumeratorHolder as u8, 0x6);
  assert_eq!(ForInFeedback::Any as u8, 0x7);
  // ExternalArrayType (globals.h:2915) — 连续 1..12
  assert_eq!(ExternalArrayType::ExternalInt8Array as u8, 1);
  assert_eq!(ExternalArrayType::ExternalBigUint64Array as u8, 12);
  // DeoptimizeKind 计数 (globals.h:1027)
  assert_eq!(DEOPTIMIZE_KIND_COUNT, 3);
  assert_eq!(DeoptimizeKind::Eager as u8, 0);
  assert_eq!(DeoptimizeKind::Lazy as u8, 2);
  // AllocationSpace 哨兵 (globals.h:1506-1513)
  assert_eq!(FIRST_SPACE as u8, 0);
  assert_eq!(LAST_SPACE as u8, 12);
  assert_eq!(
    LAST_GROWABLE_PAGED_SPACE as u8,
    AllocationSpace::SharedTrustedSpace as u8
  );
  OK
}

/// 位标志反馈枚举: CompareOperationFeedback::Type 的位或换算
#[test]
fn globals_feedback_bit_flags() -> Void {
  // globals.h:2674-2698 — kNumber = kSignedSmall | kOtherNumberFlag
  assert_eq!(
    CompareOperationFeedbackType::Number as u16,
    COMPARE_OP_SIGNED_SMALL_FLAG | COMPARE_OP_OTHER_NUMBER_FLAG
  );
  assert_eq!(
    CompareOperationFeedbackType::StringOrOddball as u16,
    COMPARE_OP_INTERNALIZED_STRING_FLAG
      | COMPARE_OP_OTHER_STRING_FLAG
      | COMPARE_OP_BOOLEAN_FLAG
      | COMPARE_OP_NULL_OR_UNDEFINED_FLAG
  );
  assert_eq!(COMPARE_OP_ANY_MASK, 0x3FF);
  assert_eq!(
    CompareOperationFeedbackType::Any as u16,
    COMPARE_OP_ANY_MASK
  );
  // TypeIndex 对数: 列表序 = X-macro 序
  assert_eq!(BINARY_OP_NUM_TYPE_INDICES, 12);
  assert_eq!(COMPARE_OP_NUM_TYPE_INDICES, 17);
  assert_eq!(BINARY_OP_TRANSITION_MAP_STRIDE, 16);
  assert_eq!(COMPARE_OP_TRANSITION_MAP_STRIDE, 32);
  OK
}

/// VariableMode 判定谓词 (globals.h:2351-2401)
#[test]
fn globals_variable_mode_predicates() -> Void {
  assert!(VariableMode::Dynamic.is_dynamic());
  assert!(VariableMode::DynamicLocal.is_dynamic());
  assert!(!VariableMode::Var.is_dynamic());
  assert!(VariableMode::Let.is_declared());
  assert!(VariableMode::Var.is_declared());
  assert!(!VariableMode::Temporary.is_declared());
  assert!(VariableMode::PrivateGetterOnly.is_private_accessor());
  assert!(VariableMode::Using.is_resource_managed());
  assert!(VariableMode::AwaitUsing.is_resource_managed());
  assert!(VariableMode::Const.is_immutable_lexical());
  assert!(VariableMode::AwaitUsing.is_lexical());
  assert!(!VariableMode::Var.is_lexical());
  assert_eq!(VariableMode::Let as u8, 0); // 原版 static_assert (globals.h:2357)
  OK
}

/// js_parameter_count 与语言模式尺寸
#[test]
fn globals_helpers() -> Void {
  assert_eq!(js_parameter_count(0), K_JS_ARGC_RECEIVER_SLOTS);
  assert_eq!(LANGUAGE_MODE_SIZE, 2);
  assert_eq!(AllocationSpace::RoSpace.as_str(), "read_only_space");
  assert_eq!(
    DeoptimizeKind::LazyAfterFastCall.as_str(),
    "LazyAfterfastCall"
  );
  OK
}

/// operator<< 打印串族: 与原版 globals.h 输出串一致 (差分日志面契约,
/// 同 message_template_strings 先例 — 字面量为原版转录非常量自比)
#[test]
fn globals_operator_str_families() -> Void {
  // LookupHoistingMode (globals.h:1050-1059)
  assert_eq!(LookupHoistingMode::Normal.as_str(), "normal hoisting");
  assert_eq!(
    LookupHoistingMode::LegacySloppy.as_str(),
    "legacy sloppy hoisting"
  );
  // TypedArrayAccessMode (globals.h:1798-1806)
  assert_eq!(TypedArrayAccessMode::Read.as_str(), "kRead");
  assert_eq!(TypedArrayAccessMode::Write.as_str(), "kWrite");
  // ConvertReceiverMode (globals.h:2103-2113)
  assert_eq!(
    ConvertReceiverMode::NullOrUndefined.as_str(),
    "NULL_OR_UNDEFINED"
  );
  assert_eq!(
    ConvertReceiverMode::NotNullOrUndefined.as_str(),
    "NOT_NULL_OR_UNDEFINED"
  );
  assert_eq!(ConvertReceiverMode::Any.as_str(), "ANY");
  // CreateArgumentsType (globals.h:2134-2144)
  assert_eq!(
    CreateArgumentsType::MappedArguments.as_str(),
    "MAPPED_ARGUMENTS"
  );
  assert_eq!(
    CreateArgumentsType::UnmappedArguments.as_str(),
    "UNMAPPED_ARGUMENTS"
  );
  assert_eq!(
    CreateArgumentsType::RestParameter.as_str(),
    "REST_PARAMETER"
  );
  // InterpreterPushArgsMode (globals.h:2478-2489)
  assert_eq!(
    InterpreterPushArgsMode::ArrayFunction.as_str(),
    "ArrayFunction"
  );
  assert_eq!(
    InterpreterPushArgsMode::WithFinalSpread.as_str(),
    "WithFinalSpread"
  );
  assert_eq!(InterpreterPushArgsMode::Other.as_str(), "Other");
  // UnicodeEncoding (globals.h:2851-2859)
  assert_eq!(UnicodeEncoding::UTF16.as_str(), "UTF16");
  assert_eq!(UnicodeEncoding::UTF32.as_str(), "UTF32");
  // IterationKind (globals.h:2863-2873)
  assert_eq!(IterationKind::Keys.as_str(), "IterationKind::kKeys");
  assert_eq!(IterationKind::Values.as_str(), "IterationKind::kValues");
  assert_eq!(IterationKind::Entries.as_str(), "IterationKind::kEntries");
  // CollectionKind (globals.h:2877-2885)
  assert_eq!(CollectionKind::Map.as_str(), "CollectionKind::kMap");
  assert_eq!(CollectionKind::Set.as_str(), "CollectionKind::kSet");
  // SpeculationMode (globals.h:3048-3059)
  assert_eq!(
    SpeculationMode::AllowSpeculation.as_str(),
    "SpeculationMode::kAllowSpeculation"
  );
  assert_eq!(
    SpeculationMode::DisallowBoundsCheckSpeculation.as_str(),
    "SpeculationMode::kDisallowBoundsCheckSpeculation"
  );
  assert_eq!(
    SpeculationMode::DisallowSpeculation.as_str(),
    "SpeculationMode::kDisallowSpeculation"
  );
  // KeyedAccessStoreMode (globals.h:3165-3177)
  assert_eq!(KeyedAccessStoreMode::InBounds.as_str(), "kInBounds");
  assert_eq!(
    KeyedAccessStoreMode::GrowAndHandleCOW.as_str(),
    "kGrowAndHandleCOW"
  );
  assert_eq!(
    KeyedAccessStoreMode::IgnoreTypedArrayOOB.as_str(),
    "kIgnoreTypedArrayOOB"
  );
  assert_eq!(KeyedAccessStoreMode::HandleCOW.as_str(), "kHandleCOW");
  // SilenceNanMode (globals.h:3387-3395)
  assert_eq!(
    SilenceNanMode::SilenceUndefined.as_str(),
    "SilenceUndefined"
  );
  assert_eq!(
    SilenceNanMode::PreserveUndefined.as_str(),
    "PreserveUndefined"
  );
  OK
}

/// IeeeDoubleArchType: f64 ↔ 位载体往返与位域判读 (globals.h:1985-2012)
#[test]
fn globals_ieee_double_arch_type() -> Void {
  // 1.5 = 0x3FF8000000000000: exp 0x3FF, 尾数位 [32..52) = 0x80000, man_low/sign 0
  let v = IeeeDoubleArchType::from_f64(1.5);
  assert_eq!(v.exp(), 0x3FF);
  assert_eq!(v.man_high(), 0x80000);
  assert_eq!(v.man_low(), 0);
  assert_eq!(v.sign(), 0);
  // 负数: -1.0 = 0xBFF0000000000000 → sign 1, exp 0x3FF, 尾数 0
  let neg = IeeeDoubleArchType::from_f64(-1.0);
  assert_eq!(neg.sign(), 1);
  assert_eq!(neg.exp(), 0x3FF);
  assert_eq!(neg.man_high(), 0);
  // from_f64 与 from_bits 同一载体
  let bits = IeeeDoubleArchType::from_f64(1.5);
  let direct = IeeeDoubleArchType::from_bits(0x3FF8_0000_0000_0000);
  assert_eq!(bits.exp(), direct.exp());
  assert_eq!(bits.man_high(), direct.man_high());
  assert_eq!(bits.man_low(), direct.man_low());
  assert_eq!(bits.sign(), direct.sign());
  OK
}

/// StaticStringsEqual (globals.h:821-826): 逐字节等值与不等路径
#[test]
fn globals_static_strings_equal() -> Void {
  assert!(static_strings_equal("kNormal", "kNormal"));
  assert!(!static_strings_equal("kNormal", "kLegacySloppy"));
  assert!(!static_strings_equal("abc", "abcd"));
  assert!(static_strings_equal("", ""));
  OK
}

/// DefineKeyedOwnProperty*Flags 位集 (globals.h:2900-2913, round1 补遗漏):
/// 判别值即位值, put/contains/is_empty 语义对齐 base::Flags
#[test]
fn globals_define_keyed_own_property_flags() -> Void {
  assert!(DefineKeyedOwnPropertyFlags::default().is_empty());
  let flags =
    DefineKeyedOwnPropertyFlags::default().put(DefineKeyedOwnPropertyFlag::SetFunctionName);
  assert!(flags.contains(DefineKeyedOwnPropertyFlag::SetFunctionName));
  assert!(!flags.is_empty());
  // 幂等并入
  assert_eq!(
    flags,
    flags.put(DefineKeyedOwnPropertyFlag::SetFunctionName)
  );

  let lit = DefineKeyedOwnPropertyInLiteralFlags::default()
    .put(DefineKeyedOwnPropertyInLiteralFlag::SetFunctionName);
  assert!(lit.contains(DefineKeyedOwnPropertyInLiteralFlag::SetFunctionName));
  assert!(!lit.is_empty());
  OK
}

/// TrustedPointer_t (globals.h:708-713, sandbox 开分支) = TrustedPointerHandle
/// (uint32); 尺寸与 kTrustedPointerSize 互证
#[test]
fn globals_trusted_pointer_t() -> Void {
  assert_eq!(size_of::<TrustedPointerT>(), K_TRUSTED_POINTER_SIZE);
  OK
}

fn sample_callee(a: i32, b: i32) -> i32 {
  a + b
}

/// FUNCTION_ADDR / FUNCTION_CAST 往返 (globals.h:793-805)
#[test]
fn globals_function_cast_roundtrip() -> Void {
  let addr = v8_common::function_addr!(sample_callee);
  assert_ne!(addr, 0);
  let callee: fn(i32, i32) -> i32 = unsafe { function_cast(addr) };
  assert_eq!(callee(40, 2), 42);
  OK
}

/// FUNCTION_CAST 目标类型尺寸不匹配必须 panic 拦截
#[test]
#[should_panic(expected = "FUNCTION_CAST 目标类型须与地址等宽")]
fn globals_function_cast_size_mismatch_panics() {
  let addr = v8_common::function_addr!(sample_callee);
  let _bad: u8 = unsafe { function_cast(addr) };
}

/// [round3] 跨枚举重名变体判别值锚 — 全量 diff (原版 globals.h 491 名逐条
/// 程序化比对) 中脚本易混槽的同名变体固化:
/// VariableLocation (globals.h:2396) / ConvertReceiverMode (globals.h:2091) /
/// HeapGrowingMode (globals.h:1758) / StoreOrigin (globals.h:913) /
/// RecordYoungSlot (globals.h:1770) / ArgumentsType (globals.h:1202)
#[test]
fn globals_round3_name_collision_anchors() -> Void {
  use v8_common::{ArgumentsType, HeapGrowingMode, RecordYoungSlot, StoreOrigin, VariableLocation};
  // VariableLocation: UNALLOCATED.. 按声明序
  assert_eq!(VariableLocation::Unallocated as u8, 0);
  assert_eq!(VariableLocation::Context as u8, 3);
  assert_eq!(VariableLocation::ReplGlobal as u8, 6);
  // ConvertReceiverMode: kLast = kAny
  assert_eq!(ConvertReceiverMode::Any as u32, 2);
  assert_eq!(ConvertReceiverMode::NullOrUndefined as u32, 0);
  // HeapGrowingMode: 4 变体声明序
  assert_eq!(HeapGrowingMode::Default as u8, 3);
  // StoreOrigin: kMaybeKeyed=0, kNamed=1
  assert_eq!(StoreOrigin::MaybeKeyed as u8, 0);
  assert_eq!(StoreOrigin::Named as u8, 1);
  // RecordYoungSlot: bool 语义 kNo=0, kYes=1
  assert_eq!(RecordYoungSlot::No as u8, 0);
  assert_eq!(RecordYoungSlot::Yes as u8, 1);
  // ArgumentsType: kRuntime=0, kJS=1
  assert_eq!(ArgumentsType::Runtime as u8, 0);
  assert_eq!(ArgumentsType::JS as u8, 1);
  OK
}
