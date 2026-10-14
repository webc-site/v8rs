// 由 tool/src/builtins/typed-array-slice.tq.js 从 v8/src/builtins/typed-array-slice.tq 生成; 修订就地改, 复跑覆盖
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

/// 原版顶层声明清单 (共 6 条)
pub const DECLS: &[TqDecl] = &[
  ("namespace", "typed_array", "namespace typed_array"),
  (
    "const",
    "kBuiltinNameSlice",
    "const kBuiltinNameSlice: constexpr string = '%TypedArray%.prototype.slice'",
  ),
  (
    "macro",
    "TypedArrayBuiltinsAssembler::CallCCopyTypedArrayElementsSlice",
    "macro TypedArrayBuiltinsAssembler::CallCCopyTypedArrayElementsSlice( JSTypedArray, JSTypedArray, uintptr, uintptr): void",
  ),
  (
    "macro",
    "FastCopy",
    "macro FastCopy( src: typed_array::AttachedJSTypedArray, dest: JSTypedArray, k: uintptr, count: uintptr): void labels IfSlow",
  ),
  (
    "macro",
    "SlowCopy",
    "macro SlowCopy( implicit context: Context)(src: JSTypedArray, dest: JSTypedArray, k: uintptr, final: uintptr): void",
  ),
  (
    "builtin",
    "TypedArrayPrototypeSlice",
    "builtin TypedArrayPrototypeSlice( js-implicit context: NativeContext, receiver: JSAny)(...arguments): JSAny",
  ),
];

/// 字面量常量直译 (原版 const <名>: <类型> = <字面量>)
pub const BUILTIN_NAME_SLICE: &'static str = "%TypedArray%.prototype.slice"; // 原版: kBuiltinNameSlice

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 6);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "const").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "macro").count() == 3);
  assert!(DECLS.iter().filter(|d| d.kind == "builtin").count() == 1);
};
