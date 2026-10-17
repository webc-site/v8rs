// 由 tool/src/builtins/ic-callable.tq.js 从 v8/src/builtins/ic-callable.tq 生成; 修订就地改, 复跑覆盖
// Torque 声明骨架 — 声明清单/常量直译/C++ 符号引用, 函数体不展开
//! 任务映射: v8-builtins-misc; 聚合基线脚本另产同源模块 (并存不覆盖)

/// Torque 声明条目 (kind: namespace/type/const/macro/builtin/struct/enum/class/import)
#[derive(Debug, Clone, Copy)]
pub struct TqDecl {
  pub kind: &'static str,
  pub name: &'static str,
  /// 原版签名 (空白规整; 含 labels/js-implicit 等修饰)
  pub sig: &'static str,
}

/// 原版顶层声明清单 (共 18 条)
pub const DECLS: &[TqDecl] = &[
  ("namespace", "ic", "namespace ic"),
  ("namespace", "callable", "namespace callable"),
  (
    "macro",
    "IncrementCallCount",
    "macro IncrementCallCount(FeedbackVector, uintptr): void",
  ),
  (
    "const",
    "kCallFeedbackContentFieldMask",
    "const kCallFeedbackContentFieldMask: constexpr int32 generates 'FeedbackNexus::CallFeedbackContentField::kMask'",
  ),
  (
    "const",
    "kCallFeedbackContentFieldShift",
    "const kCallFeedbackContentFieldShift: constexpr uint32 generates 'FeedbackNexus::CallFeedbackContentField::kShift'",
  ),
  (
    "macro",
    "IsMonomorphic",
    "macro IsMonomorphic(feedback: MaybeObject, target: JSAny): bool",
  ),
  (
    "macro",
    "InSameNativeContext",
    "macro InSameNativeContext(lhs: Context, rhs: Context): bool",
  ),
  (
    "macro",
    "TryInitializeAsMonomorphic",
    "macro TryInitializeAsMonomorphic( implicit context: Context)(maybeTarget: JSAny, feedbackVector: FeedbackVector, slotId: uintptr): void labels TransitionToMegamorphic",
  ),
  (
    "macro",
    "TransitionToMegamorphic",
    "macro TransitionToMegamorphic( implicit context: Context)(feedbackVector: FeedbackVector, slotId: uintptr): void",
  ),
  (
    "macro",
    "TaggedEqualPrototypeApplyFunction",
    "macro TaggedEqualPrototypeApplyFunction( implicit context: Context)(target: JSAny): bool",
  ),
  (
    "macro",
    "FeedbackValueIsReceiver",
    "macro FeedbackValueIsReceiver( implicit context: Context)(feedbackVector: FeedbackVector, slotId: uintptr): bool",
  ),
  (
    "macro",
    "SetCallFeedbackContent",
    "macro SetCallFeedbackContent( implicit context: Context)(feedbackVector: FeedbackVector, slotId: uintptr, callFeedbackContent: constexpr CallFeedbackContent): void",
  ),
  (
    "macro",
    "CollectCallFeedback",
    "macro CollectCallFeedback( maybeTarget: JSAny, maybeReceiver: Lazy<JSAny>, context: Context, maybeFeedbackVector: Undefined|FeedbackVector, slotId: uintptr): void",
  ),
  (
    "macro",
    "CollectInstanceOfFeedback",
    "macro CollectInstanceOfFeedback( maybeTarget: JSAny, context: Context, maybeFeedbackVector: Undefined|FeedbackVector, slotId: uintptr): void",
  ),
  (
    "macro",
    "BothTaggedEqualArrayFunction",
    "macro BothTaggedEqualArrayFunction( implicit context: Context)(first: JSAny, second: JSAny): bool",
  ),
  (
    "macro",
    "CreateAllocationSiteInFeedbackVector",
    "macro CreateAllocationSiteInFeedbackVector(FeedbackVector, uintptr): AllocationSite",
  ),
  (
    "macro",
    "CastFeedbackVector",
    "macro CastFeedbackVector( maybeFeedbackVector: Undefined|FeedbackVector, updateFeedbackMode: constexpr UpdateFeedbackMode): FeedbackVector labels Fallback",
  ),
  (
    "macro",
    "CollectConstructFeedback",
    "macro CollectConstructFeedback( implicit context: Context)(target: JSAny, newTarget: JSAny, maybeFeedbackVector: Undefined|FeedbackVector, slotId: TaggedIndex, updateFeedbackMode: constexpr UpdateFeedbackMode): never labels ConstructGeneric…",
  ),
];

/// constexpr ... generates 'C++ 符号' 引用登记 (机械登记, 不猜值)
pub const CPP_GENERATES: &[(&str, &str)] = &[
  (
    "kCallFeedbackContentFieldMask",
    "FeedbackNexus::CallFeedbackContentField::kMask",
  ),
  (
    "kCallFeedbackContentFieldShift",
    "FeedbackNexus::CallFeedbackContentField::kShift",
  ),
];

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 18);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 2);
  assert!(DECLS.iter().filter(|d| d.kind == "macro").count() == 14);
  assert!(DECLS.iter().filter(|d| d.kind == "const").count() == 2);
};
