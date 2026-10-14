// 由 tool/src/builtins/typed-array-from.tq.js 从 v8/src/builtins/typed-array-from.tq 生成; 修订就地改, 复跑覆盖
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

/// 原版顶层声明清单 (共 8 条)
pub const DECLS: &[TqDecl] = &[
  ("namespace", "typed_array", "namespace typed_array"),
  (
    "const",
    "kBuiltinNameFrom",
    "const kBuiltinNameFrom: constexpr string = '%TypedArray%.from'",
  ),
  (
    "const",
    "kTypedArrayPrototypeValues",
    "const kTypedArrayPrototypeValues: constexpr BuiltinsName generates 'Builtin::kTypedArrayPrototypeValues'",
  ),
  (
    "const",
    "kArrayPrototypeValues",
    "const kArrayPrototypeValues: constexpr BuiltinsName generates 'Builtin::kArrayPrototypeValues'",
  ),
  (
    "const",
    "kSetPrototypeValues",
    "const kSetPrototypeValues: constexpr BuiltinsName generates 'Builtin::kSetPrototypeValues'",
  ),
  (
    "builtin",
    "IterableToList",
    "builtin IterableToList(implicit context: Context)(JSAny, JSAny): JSArray",
  ),
  (
    "macro",
    "CanCopyElementsFastNumber",
    "macro CanCopyElementsFastNumber(source: JSArray): bool",
  ),
  (
    "builtin",
    "TypedArrayFrom",
    "builtin TypedArrayFrom( js-implicit context: NativeContext, receiver: JSAny)( ...arguments): JSTypedArray",
  ),
];

/// constexpr ... generates 'C++ 符号' 引用登记 (机械登记, 不猜值)
pub const CPP_GENERATES: &[(&str, &str)] = &[
  (
    "kTypedArrayPrototypeValues",
    "Builtin::kTypedArrayPrototypeValues",
  ),
  ("kArrayPrototypeValues", "Builtin::kArrayPrototypeValues"),
  ("kSetPrototypeValues", "Builtin::kSetPrototypeValues"),
];

/// 字面量常量直译 (原版 const <名>: <类型> = <字面量>)
pub const BUILTIN_NAME_FROM: &'static str = "%TypedArray%.from"; // 原版: kBuiltinNameFrom

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 8);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "const").count() == 4);
  assert!(DECLS.iter().filter(|d| d.kind == "builtin").count() == 2);
  assert!(DECLS.iter().filter(|d| d.kind == "macro").count() == 1);
};
