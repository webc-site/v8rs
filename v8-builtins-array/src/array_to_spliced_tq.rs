// 由 tool/src/builtins/array-to-spliced.tq.js 从 v8/src/builtins/array-to-spliced.tq 生成; 修订就地改, 复跑覆盖
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

/// 原版顶层声明清单 (共 5 条)
pub const DECLS: &[TqDecl] = &[
  ("namespace", "array", "namespace array"),
  (
    "macro",
    "CopyFastPackedArrayForToSpliced",
    "macro CopyFastPackedArrayForToSpliced( implicit context: Context)(kind: constexpr ElementsKind, array: JSArray, newLenSmi: Smi, actualStartSmi: Smi, insertCountSmi: Smi, actualDeleteCountSmi: Smi): JSArray",
  ),
  (
    "macro",
    "TryFastArrayToSpliced",
    "macro TryFastArrayToSpliced( implicit context: Context)(args: Arguments, o: JSReceiver, originalLenNumber: Number, newLenNumber: Number, actualStartNumber: Number, insertCount: Smi, actualDeleteCountNumber: Number): JSArray labels Slow",
  ),
  (
    "macro",
    "GenericArrayToSpliced",
    "macro GenericArrayToSpliced( implicit context: Context)(args: Arguments, o: JSReceiver, newLen: Number, actualStart: Number, actualDeleteCount: Number): JSArray",
  ),
  (
    "builtin",
    "ArrayPrototypeToSpliced",
    "builtin ArrayPrototypeToSpliced( js-implicit context: NativeContext, receiver: JSAny)(...arguments): JSAny",
  ),
];

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 5);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "macro").count() == 3);
  assert!(DECLS.iter().filter(|d| d.kind == "builtin").count() == 1);
};
