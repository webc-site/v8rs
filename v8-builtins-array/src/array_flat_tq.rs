// 由 tool/src/builtins/array-flat.tq.js 从 v8/src/builtins/array-flat.tq 生成; 修订就地改, 复跑覆盖
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

/// 原版顶层声明清单 (共 15 条)
pub const DECLS: &[TqDecl] = &[
  ("namespace", "array", "namespace array"),
  (
    "macro",
    "ArrayIsArray_Inline",
    "macro ArrayIsArray_Inline( implicit context: Context)(element: JSAny): Boolean",
  ),
  ("struct", "FlatVector", "struct FlatVector"),
  (
    "macro",
    "NewFlatVector",
    "macro NewFlatVector(implicit context: Context)(length: Smi): FlatVector",
  ),
  (
    "struct",
    "FlattenedLengthResult",
    "struct FlattenedLengthResult",
  ),
  (
    "const",
    "kMaxFlatFastStackEntries",
    "const kMaxFlatFastStackEntries: intptr = 3072",
  ),
  (
    "macro",
    "CalculateFlattenedLengthFast",
    "macro CalculateFlattenedLengthFast( implicit context: Context)(source: FastJSArray, sourceLength: Smi, depth: Smi): FlattenedLengthResult labels Bailout",
  ),
  (
    "macro",
    "TryFastFlat",
    "macro TryFastFlat( implicit context: Context)(receiver: JSReceiver, sourceLength: Number, depth: Smi): JSArray labels Bailout",
  ),
  (
    "macro",
    "FlattenIntoArrayFast",
    "macro FlattenIntoArrayFast( implicit context: Context)(target: JSReceiver, source: JSReceiver, sourceLength: Number, start: Number, depth: Smi, hasMapper: constexpr bool, mapfn: JSAny, thisArgs: JSAny): Number labels Bailout(Number, Number)",
  ),
  (
    "macro",
    "FlattenIntoArraySlow",
    "macro FlattenIntoArraySlow( implicit context: Context)(target: JSReceiver, source: JSReceiver, sourceIndex: Number, sourceLength: Number, start: Number, depth: Smi, hasMapper: constexpr bool, mapfn: JSAny, thisArgs: JSAny): Number",
  ),
  (
    "macro",
    "FlattenIntoArray",
    "macro FlattenIntoArray( implicit context: Context)(target: JSReceiver, source: JSReceiver, sourceLength: Number, start: Number, depth: Smi, hasMapper: constexpr bool, mapfn: JSAny, thisArgs: JSAny): Number",
  ),
  (
    "builtin",
    "FlattenIntoArrayWithoutMapFn",
    "builtin FlattenIntoArrayWithoutMapFn( implicit context: Context)(target: JSReceiver, source: JSReceiver, sourceLength: Number, start: Number, depth: Smi): Number",
  ),
  (
    "builtin",
    "FlattenIntoArrayWithMapFn",
    "builtin FlattenIntoArrayWithMapFn( implicit context: Context)(target: JSReceiver, source: JSReceiver, sourceLength: Number, start: Number, depth: Smi, mapfn: JSAny, thisArgs: JSAny): Number",
  ),
  (
    "builtin",
    "ArrayPrototypeFlat",
    "builtin ArrayPrototypeFlat( js-implicit context: NativeContext, receiver: JSAny)(...arguments): JSAny",
  ),
  (
    "builtin",
    "ArrayPrototypeFlatMap",
    "builtin ArrayPrototypeFlatMap( js-implicit context: NativeContext, receiver: JSAny)(...arguments): JSAny",
  ),
];

/// 字面量常量直译 (原版 const <名>: <类型> = <字面量>)
pub const MAX_FLAT_FAST_STACK_ENTRIES: i64 = 3072; // 原版: kMaxFlatFastStackEntries

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 15);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "macro").count() == 7);
  assert!(DECLS.iter().filter(|d| d.kind == "struct").count() == 2);
  assert!(DECLS.iter().filter(|d| d.kind == "const").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "builtin").count() == 4);
};
