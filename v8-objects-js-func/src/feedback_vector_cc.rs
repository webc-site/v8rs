// 由 tool/src/objects/feedback-vector.cc.js 从 v8/src/objects/feedback-vector.cc 生成; 修订就地改, 复跑覆盖
// FeedbackSlotKind 显示名表 + cc 方法清单骨架 (h/inl/tq 差集归 feedback-vector.h.js 席)

//! 差集骨架: feedback-vector.cc 的规则代码。
//! IC 状态机 / megamorphic 处理 / OSR 语义面留 Developer。

/// v8: src/objects/feedback-vector.cc FeedbackMetadata::Kind2String 显示名表
///   (FeedbackSlotKind::kXxx → 字符串); kSetKeyedSloppy/Strict 显示 Store* 为原版刻意别名
pub const FEEDBACK_SLOT_KIND_NAMES: &[(&str, &str); 26] = &[
  ("kInvalid", "Invalid"),
  ("kCall", "Call"),
  ("kLoadProperty", "LoadProperty"),
  ("kLoadGlobalInsideTypeof", "LoadGlobalInsideTypeof"),
  ("kLoadGlobalNotInsideTypeof", "LoadGlobalNotInsideTypeof"),
  ("kLoadKeyed", "LoadKeyed"),
  ("kHasKeyed", "HasKeyed"),
  ("kSetNamedSloppy", "SetNamedSloppy"),
  ("kSetNamedStrict", "SetNamedStrict"),
  ("kDefineNamedOwn", "DefineNamedOwn"),
  ("kDefineKeyedOwn", "DefineKeyedOwn"),
  ("kStoreGlobalSloppy", "StoreGlobalSloppy"),
  ("kStoreGlobalStrict", "StoreGlobalStrict"),
  ("kSetKeyedSloppy", "StoreKeyedSloppy"),
  ("kSetKeyedStrict", "StoreKeyedStrict"),
  ("kStoreInArrayLiteral", "StoreInArrayLiteral"),
  ("kBinaryOp", "BinaryOp"),
  ("kCompareOp", "CompareOp"),
  (
    "kDefineKeyedOwnPropertyInLiteral",
    "DefineKeyedOwnPropertyInLiteral",
  ),
  ("kLiteral", "Literal"),
  ("kForIn", "ForIn"),
  ("kInstanceOf", "InstanceOf"),
  ("kTypeOf", "TypeOf"),
  ("kCloneObject", "CloneObject"),
  ("kJumpLoop", "JumpLoop"),
  ("kStringAddAndInternalize", "StringAddAndInternalize"),
];

// ── cc 方法清单 (66 个定义, oracle: clang -E 尾部区段) ──
/// v8: src/objects/feedback-vector.cc
pub const FEEDBACK_VECTOR_CC_METHODS: &[&str] = &[
  "IC::update_lookup_start_object_map",
  "IC::IsHandler",
  "IC::vector_needs_update",
  "FeedbackVectorSpec::AddSlot",
  "FeedbackMetadata::GetKind",
  "FeedbackMetadata::SetKind",
  "FeedbackMetadata::GetCreateClosureParameterCount",
  "FeedbackMetadata::SetCreateClosureParameterCount",
  "FeedbackMetadata::New",
  "FeedbackMetadata::SpecDiffersFrom",
  "FeedbackMetadata::Kind2String",
  "FeedbackVector::GetKind",
  "ClosureFeedbackCellArray::New",
  "FeedbackVector::New",
  "FeedbackVector::NewForTesting",
  "FeedbackVector::NewWithOneBinarySlotForTesting",
  "FeedbackVector::NewWithOneCompareSlotForTesting",
  "FeedbackVector::AddToVectorsForProfilingTools",
  "FeedbackVector::set_tiering_in_progress",
  "FeedbackVector::SetOptimizedOsrCode",
  "FeedbackVector::osr_tiering_in_progress",
  "FeedbackVector::set_osr_tiering_in_progress",
  "FeedbackVector::ClearSlots",
  "NexusConfig::NewHandle",
  "NexusConfig::SetFeedbackPair",
  "NexusConfig::SetFeedbackExtra",
  "FeedbackNexus::CreateArrayOfSize",
  "FeedbackNexus::ConfigureUninitialized",
  "FeedbackNexus::Clear",
  "FeedbackNexus::ConfigureMegamorphic",
  "FeedbackNexus::ConfigureHomomorphic",
  "FeedbackNexus::ConfigureMegaDOM",
  "FeedbackNexus::GetFirstMap",
  "FeedbackNexus::ic_state",
  "FeedbackNexus::ic_handler",
  "FeedbackNexus::ConfigurePropertyCellMode",
  "FeedbackNexus::ConfigureLexicalVarMode",
  "FeedbackNexus::ConfigureHandlerMode",
  "FeedbackNexus::ConfigureCloneObject",
  "FeedbackNexus::ConfigureStringAddInternalizeCache",
  "FeedbackNexus::GetCallCount",
  "FeedbackNexus::SetSpeculationMode",
  "FeedbackNexus::NextSpeculationMode",
  "FeedbackNexus::GetSpeculationMode",
  "FeedbackNexus::GetOptimizedOsrCode",
  "FeedbackNexus::GetCallFeedbackContent",
  "FeedbackNexus::ComputeCallFrequency",
  "FeedbackNexus::ConfigureMonomorphic",
  "FeedbackNexus::ConfigurePolymorphic",
  "FeedbackNexus::ExtractMaps",
  "FeedbackNexus::ExtractMegaDOMHandler",
  "FeedbackNexus::ExtractHomomorphicHandler",
  "FeedbackNexus::ExtractMapsAndHandlers",
  "FeedbackNexus::FindHandlerForMap",
  "FeedbackNexus::GetName",
  "FeedbackNexus::GetKeyedAccessLoadMode",
  "FeedbackNexus::GetKeyedAccessStoreMode",
  "FeedbackNexus::IsOneMapManyNames",
  "FeedbackNexus::GetKeyType",
  "FeedbackNexus::GetBinaryOperationFeedback",
  "FeedbackNexus::GetCompareOperationFeedback",
  "FeedbackNexus::GetTypeOfFeedback",
  "FeedbackNexus::GetForInFeedback",
  "FeedbackNexus::GetConstructorFeedback",
  "FeedbackIterator::Advance",
  "FeedbackIterator::AdvancePolymorphic",
];

/// v8: src/objects/feedback-vector.cc FeedbackNexus — 槽位反馈读写的中心抽象
///   (ic_state / Configure* / ComputeUnreliableReceiverMap 语义面留锚)
/// v8: src/objects/feedback-vector.cc FeedbackVector::ClearSlots — 逐槽重置 (含 weak 引用清弱)
/// v8: src/objects/feedback-vector.cc FeedbackVectorSpec::AddSlot — GetSlotSize(kind) 决定槽宽补 kInvalid
// ── 对数不变式 ──
const fn str_eq(a: &str, b: &str) -> bool {
  let (a, b) = (a.as_bytes(), b.as_bytes());
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

const fn contains_pair(t: &[(&str, &str)], k: &str, n: &str) -> bool {
  let mut i = 0;
  while i < t.len() {
    if str_eq(t[i].0, k) && str_eq(t[i].1, n) {
      return true;
    }
    i += 1;
  }
  false
}

const _: () = {
  assert!(FEEDBACK_SLOT_KIND_NAMES.len() == 26);
  let n0 = FEEDBACK_SLOT_KIND_NAMES[0];
  assert!(str_eq(n0.0, "kInvalid") && str_eq(n0.1, "Invalid"));
  let n1 = FEEDBACK_SLOT_KIND_NAMES[1];
  assert!(str_eq(n1.0, "kCall") && str_eq(n1.1, "Call"));
  assert!(contains_pair(
    FEEDBACK_SLOT_KIND_NAMES,
    "kSetKeyedSloppy",
    "StoreKeyedSloppy"
  ));
  assert!(contains_pair(
    FEEDBACK_SLOT_KIND_NAMES,
    "kDefineKeyedOwnPropertyInLiteral",
    "DefineKeyedOwnPropertyInLiteral"
  ));
  assert!(FEEDBACK_VECTOR_CC_METHODS.len() >= 60);
};
