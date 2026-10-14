// 由 tool/src/builtins/array-splice.tq.js 从 v8/src/builtins/array-splice.tq 生成; 修订就地改, 复跑覆盖
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
    "Extract",
    "macro Extract( implicit context: Context)(source: FixedArray, startIndex: Smi, count: Smi, resultCapacity: Smi): FixedArray",
  ),
  (
    "macro",
    "Extract",
    "macro Extract( implicit context: Context)(source: FixedDoubleArray|EmptyFixedArray, startIndex: Smi, count: Smi, resultCapacity: Smi): FixedDoubleArray |EmptyFixedArray",
  ),
  (
    "macro",
    "DoMoveElements",
    "macro DoMoveElements<FixedArrayType : type extends FixedArrayBase>( elements: FixedArrayType, dstIndex: Smi, srcIndex: Smi, count: Smi): void",
  ),
  (
    "macro",
    "StoreHoles",
    "macro StoreHoles<FixedArrayType : type extends FixedArrayBase>( elements: FixedArrayType, holeStartIndex: Smi, holeEndIndex: Smi): void",
  ),
  (
    "macro",
    "DoCopyElements",
    "macro DoCopyElements<FixedArrayType : type extends FixedArrayBase>( dstElements: FixedArrayType, dstIndex: Smi, srcElements: FixedArrayType, srcIndex: Smi, count: Smi): void",
  ),
  (
    "macro",
    "InsertArgumentsIntoFastPackedArray",
    "macro InsertArgumentsIntoFastPackedArray< FixedArrayType : type extends FixedArrayBase, ElementType: type>( dst: JSArray, dstStart: Smi, args: Arguments, argsStart: constexpr IntegerLiteral, insertCount: Smi): void",
  ),
  (
    "macro",
    "FastSplice",
    "macro FastSplice<FixedArrayType : type extends FixedArrayBase, ElementType: type>( implicit context: Context)(args: Arguments, a: JSArray, length: Smi, newLength: Smi, actualStart: Smi, insertCount: Smi, actualDeleteCount: Smi): void",
  ),
  (
    "macro",
    "TransitionElementsKindForInsertionIfNeeded",
    "macro TransitionElementsKindForInsertionIfNeeded( context: Context, a: JSArray, originalElementsKind: ElementsKind, args: Arguments, argsStart: constexpr IntegerLiteral): ElementsKind",
  ),
  (
    "macro",
    "FastArraySplice",
    "macro FastArraySplice( context: Context, args: Arguments, o: JSReceiver, originalLengthNumber: Number, actualStartNumber: Number, insertCount: Smi, actualDeleteCountNumber: Number): JSAny labels Bailout",
  ),
  (
    "macro",
    "FillDeletedElementsArray",
    "macro FillDeletedElementsArray( context: Context, o: JSReceiver, actualStart: Number, actualDeleteCount: Number, a: JSReceiver): JSAny",
  ),
  (
    "macro",
    "HandleForwardCase",
    "macro HandleForwardCase( context: Context, o: JSReceiver, len: Number, itemCount: Number, actualStart: Number, actualDeleteCount: Number): void",
  ),
  (
    "macro",
    "HandleBackwardCase",
    "macro HandleBackwardCase( context: Context, o: JSReceiver, len: Number, itemCount: Number, actualStart: Number, actualDeleteCount: Number): void",
  ),
  (
    "macro",
    "SlowSplice",
    "macro SlowSplice( context: Context, arguments: Arguments, o: JSReceiver, len: Number, actualStart: Number, insertCount: Smi, actualDeleteCount: Number): JSAny",
  ),
  (
    "builtin",
    "ArrayPrototypeSplice",
    "builtin ArrayPrototypeSplice( js-implicit context: NativeContext, receiver: JSAny)(...arguments): JSAny",
  ),
];

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 15);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "macro").count() == 13);
  assert!(DECLS.iter().filter(|d| d.kind == "builtin").count() == 1);
};
