// 由 tool/src/builtins/typed-array-set.tq.js 从 v8/src/builtins/typed-array-set.tq 生成; 修订就地改, 复跑覆盖
// Torque 声明骨架 — 声明清单/常量直译/C++ 符号引用, 函数体不展开
//! 任务映射: v8-builtins-typedarray; 聚合基线脚本另产同源模块 (并存不覆盖)

/// Torque 声明条目 (kind: namespace/type/const/macro/builtin/struct/enum/class/import)
#[derive(Debug, Clone, Copy)]
pub struct TqDecl {
  pub kind: &'static str,
  pub name: &'static str,
  /// 原版签名 (空白规整; 含 labels/js-implicit 等修饰)
  pub sig: &'static str,
}

/// 原版顶层声明清单 (共 7 条)
pub const DECLS: &[TqDecl] = &[
  ("namespace", "typed_array", "namespace typed_array"),
  (
    "const",
    "kBuiltinNameSet",
    "const kBuiltinNameSet: constexpr string = '%TypedArray%.prototype.set'",
  ),
  (
    "macro",
    "TypedArrayBuiltinsAssembler::CallCCopyFastNumberJSArrayElementsToTypedArray",
    "macro TypedArrayBuiltinsAssembler::CallCCopyFastNumberJSArrayElementsToTypedArray( Context, FastJSArray, // source AttachedJSTypedArray, // dest uintptr, // sourceLength uintptr // destOffset ): void",
  ),
  (
    "macro",
    "TypedArrayBuiltinsAssembler::CallCCopyTypedArrayElementsToTypedArray",
    "macro TypedArrayBuiltinsAssembler::CallCCopyTypedArrayElementsToTypedArray( AttachedJSTypedArray, // source AttachedJSTypedArray, // dest uintptr, // sourceLength uintptr // destOffset ): void",
  ),
  (
    "builtin",
    "TypedArrayPrototypeSet",
    "builtin TypedArrayPrototypeSet( js-implicit context: NativeContext, receiver: JSAny)(...arguments): JSAny",
  ),
  (
    "macro",
    "TypedArrayPrototypeSetArray",
    "macro TypedArrayPrototypeSetArray( implicit context: Context, receiver: JSAny)(target: JSTypedArray, targetLength: uintptr, arrayArg: JSAny, targetOffset: uintptr, targetOffsetOverflowed: bool): void labels IfOffsetOutOfBounds",
  ),
  (
    "macro",
    "TypedArrayPrototypeSetTypedArray",
    "macro TypedArrayPrototypeSetTypedArray( implicit context: Context, receiver: JSAny)( attachedTargetAndLength: ValidJSTypedArrayAndLength, attachedSourceAndLength: ValidJSTypedArrayAndLength, targetOffset: uintptr, targetOffsetOverflowed: bo…",
  ),
];

/// 字面量常量直译 (原版 const <名>: <类型> = <字面量>)
pub const BUILTIN_NAME_SET: &'static str = "%TypedArray%.prototype.set"; // 原版: kBuiltinNameSet

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 7);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "const").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "macro").count() == 4);
  assert!(DECLS.iter().filter(|d| d.kind == "builtin").count() == 1);
};
