// 由 tool/src/builtins/array.tq.js 从 v8/src/builtins/array.tq 生成; 修订就地改, 复跑覆盖
// Torque 声明骨架 — 声明清单/常量直译/C++ 符号引用, 函数体不展开
//! 任务映射: v8-builtins-array; 聚合基线脚本另产同源模块 (并存不覆盖)

/// Torque 声明条目 (kind: namespace/type/const/macro/builtin/struct/enum/class/import)
#[derive(Debug, Clone, Copy)]
pub struct TqDecl {
  pub kind: &'static str,
  pub name: &'static str,
  /// 原版签名 (空白规整; 含 labels/js-implicit 等修饰)
  pub sig: &'static str,
}

/// 原版顶层声明清单 (共 21 条)
pub const DECLS: &[TqDecl] = &[
  ("namespace", "array", "namespace array"),
  (
    "type",
    "GenericElementsAccessor",
    "type GenericElementsAccessor extends ElementsKind",
  ),
  (
    "type",
    "FastPackedSmiElements",
    "type FastPackedSmiElements extends ElementsKind",
  ),
  (
    "type",
    "FastPackedObjectElements",
    "type FastPackedObjectElements extends ElementsKind",
  ),
  (
    "type",
    "FastPackedDoubleElements",
    "type FastPackedDoubleElements extends ElementsKind",
  ),
  (
    "type",
    "FastSmiOrObjectElements",
    "type FastSmiOrObjectElements extends ElementsKind",
  ),
  (
    "type",
    "FastDoubleElements",
    "type FastDoubleElements extends ElementsKind",
  ),
  (
    "type",
    "DictionaryElements",
    "type DictionaryElements extends ElementsKind",
  ),
  (
    "macro",
    "EnsureWriteableFastElements",
    "macro EnsureWriteableFastElements( implicit context: Context)(array: JSArray): void",
  ),
  (
    "macro",
    "LoadElementOrUndefined",
    "macro LoadElementOrUndefined( implicit context: Context)(a: FixedArray, i: Smi): JSAny",
  ),
  (
    "macro",
    "LoadElementOrUndefined",
    "macro LoadElementOrUndefined(a: FixedDoubleArray, i: Smi): NumberOrUndefined",
  ),
  (
    "macro",
    "StoreArrayHole",
    "macro StoreArrayHole(elements: FixedDoubleArray, k: Smi): void",
  ),
  (
    "macro",
    "StoreArrayHole",
    "macro StoreArrayHole(elements: FixedArray, k: Smi): void",
  ),
  (
    "macro",
    "SetPropertyLength",
    "macro SetPropertyLength(implicit context: Context)(JSAny, Number): void",
  ),
  (
    "const",
    "kLengthDescriptorIndex",
    "const kLengthDescriptorIndex: constexpr int31 generates 'JSArray::kLengthDescriptorIndex'",
  ),
  (
    "const",
    "kAttributesReadOnlyMask",
    "const kAttributesReadOnlyMask: constexpr int31 generates 'PropertyDetails::kAttributesReadOnlyMask'",
  ),
  (
    "macro",
    "EnsureArrayLengthWritable",
    "macro EnsureArrayLengthWritable(implicit context: Context)(map: Map): void labels Bailout",
  ),
  (
    "macro",
    "CreateJSArrayWithElements",
    "macro CreateJSArrayWithElements( implicit context: Context)(array: FixedArray): JSArray",
  ),
  (
    "macro",
    "ArraySortContinueFromSnapshot",
    "macro ArraySortContinueFromSnapshot( implicit context: Context)(receiver: JSAny, comparefn: JSAny, tempArray: FixedArray, originalLength: Smi): JSAny",
  ),
  (
    "builtin",
    "ArraySortContinueFromSnapshotEagerDeoptContinuation",
    "builtin ArraySortContinueFromSnapshotEagerDeoptContinuation( js-implicit context: NativeContext, receiver: JSAny)(comparefn: JSAny, tempArray: JSAny, originalLength: JSAny): JSAny",
  ),
  (
    "builtin",
    "ArraySortContinueFromSnapshotLazyDeoptContinuation",
    "builtin ArraySortContinueFromSnapshotLazyDeoptContinuation( js-implicit context: NativeContext, receiver: JSAny)(comparefn: JSAny, tempArray: JSAny, originalLength: JSAny, _result: JSAny): JSAny",
  ),
];

/// constexpr ... generates 'C++ 符号' 引用登记 (机械登记, 不猜值)
pub const CPP_GENERATES: &[(&str, &str)] = &[
  ("kLengthDescriptorIndex", "JSArray::kLengthDescriptorIndex"),
  (
    "kAttributesReadOnlyMask",
    "PropertyDetails::kAttributesReadOnlyMask",
  ),
];

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 21);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "type").count() == 7);
  assert!(DECLS.iter().filter(|d| d.kind == "macro").count() == 9);
  assert!(DECLS.iter().filter(|d| d.kind == "const").count() == 2);
  assert!(DECLS.iter().filter(|d| d.kind == "builtin").count() == 2);
};
