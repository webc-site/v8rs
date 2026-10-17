// 由 tool/src/builtins/ic.tq.js 从 v8/src/builtins/ic.tq 生成; 修订就地改, 复跑覆盖
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

/// 原版顶层声明清单 (共 16 条)
pub const DECLS: &[TqDecl] = &[
  ("namespace", "ic", "namespace ic"),
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
    "CollectConstructFeedback",
    "macro CollectConstructFeedback( implicit context: Context)(target: JSAny, newTarget: JSAny, maybeFeedbackVector: Undefined|FeedbackVector, slotId: TaggedIndex, updateFeedbackMode: constexpr UpdateFeedbackMode): never labels ConstructGeneric…",
  ),
  (
    "macro",
    "MegamorphicSymbolConstant",
    "macro MegamorphicSymbolConstant(): Symbol",
  ),
  (
    "macro",
    "UninitializedSymbolConstant",
    "macro UninitializedSymbolConstant(): Symbol",
  ),
  (
    "const",
    "kMegamorphicSymbol",
    "const kMegamorphicSymbol: Symbol = MegamorphicSymbolConstant()",
  ),
  (
    "const",
    "kUninitializedSymbol",
    "const kUninitializedSymbol: Symbol = UninitializedSymbolConstant()",
  ),
  (
    "macro",
    "IsMegamorphic",
    "macro IsMegamorphic(feedback: MaybeObject): bool",
  ),
  (
    "macro",
    "IsUninitialized",
    "macro IsUninitialized(feedback: MaybeObject): bool",
  ),
  (
    "macro",
    "LoadFeedbackVectorSlot",
    "macro LoadFeedbackVectorSlot(FeedbackVector, uintptr): MaybeObject",
  ),
  (
    "macro",
    "LoadFeedbackVectorSlot",
    "macro LoadFeedbackVectorSlot(FeedbackVector, uintptr, constexpr int32): MaybeObject",
  ),
  (
    "macro",
    "StoreFeedbackVectorSlot",
    "macro StoreFeedbackVectorSlot(FeedbackVector, uintptr, MaybeObject): void",
  ),
  (
    "macro",
    "StoreFeedbackVectorSlot",
    "macro StoreFeedbackVectorSlot( FeedbackVector, uintptr, MaybeObject, constexpr WriteBarrierMode, constexpr int32): void",
  ),
  (
    "macro",
    "StoreWeakReferenceInFeedbackVector",
    "macro StoreWeakReferenceInFeedbackVector( FeedbackVector, uintptr, HeapObject): MaybeObject",
  ),
  (
    "macro",
    "ReportFeedbackUpdate",
    "macro ReportFeedbackUpdate(FeedbackVector, uintptr, constexpr string): void",
  ),
];

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 16);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "macro").count() == 13);
  assert!(DECLS.iter().filter(|d| d.kind == "const").count() == 2);
};
