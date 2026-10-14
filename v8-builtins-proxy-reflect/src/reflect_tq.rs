// 由 tool/src/builtins/reflect.tq.js 从 v8/src/builtins/reflect.tq 生成; 修订就地改, 复跑覆盖
// Torque 声明骨架 — 声明清单/常量直译/C++ 符号引用, 函数体不展开
//! 任务映射: v8-builtins-proxy-reflect; 聚合基线脚本另产同源模块 (并存不覆盖)

/// Torque 声明条目 (kind: namespace/type/const/macro/builtin/struct/enum/class/import)
#[derive(Debug, Clone, Copy)]
pub struct TqDecl {
  pub kind: &'static str,
  pub name: &'static str,
  /// 原版签名 (空白规整; 含 labels/js-implicit 等修饰)
  pub sig: &'static str,
}

/// 原版顶层声明清单 (共 14 条)
pub const DECLS: &[TqDecl] = &[
  ("namespace", "reflect", "namespace reflect"),
  (
    "builtin",
    "ReflectIsExtensible",
    "builtin ReflectIsExtensible( js-implicit context: NativeContext)(object: JSAny): JSAny",
  ),
  (
    "builtin",
    "ReflectPreventExtensions",
    "builtin ReflectPreventExtensions( js-implicit context: NativeContext)(object: JSAny): JSAny",
  ),
  (
    "builtin",
    "ReflectGetPrototypeOf",
    "builtin ReflectGetPrototypeOf( js-implicit context: NativeContext)(object: JSAny): JSAny",
  ),
  (
    "builtin",
    "ReflectSetPrototypeOf",
    "builtin ReflectSetPrototypeOf( js-implicit context: NativeContext)(object: JSAny, proto: JSAny): JSAny",
  ),
  (
    "type",
    "OnNonExistent",
    "type OnNonExistent constexpr 'OnNonExistent'",
  ),
  (
    "const",
    "kReturnUndefined",
    "const kReturnUndefined: constexpr OnNonExistent generates 'OnNonExistent::kReturnUndefined'",
  ),
  (
    "macro",
    "SmiConstant",
    "macro SmiConstant(constexpr OnNonExistent): Smi",
  ),
  (
    "builtin",
    "GetPropertyWithReceiver",
    "builtin GetPropertyWithReceiver( implicit context: Context)(JSAny, Name, JSAny, Smi): JSAny",
  ),
  (
    "builtin",
    "ReflectGet",
    "builtin ReflectGet( js-implicit context: NativeContext)(...arguments): JSAny",
  ),
  (
    "builtin",
    "ReflectDeleteProperty",
    "builtin ReflectDeleteProperty( js-implicit context: NativeContext)(object: JSAny, key: JSAny): JSAny",
  ),
  (
    "builtin",
    "ReflectHas",
    "builtin ReflectHas( js-implicit context: NativeContext)(object: JSAny, key: JSAny): JSAny",
  ),
  (
    "builtin",
    "GetOwnPropertyDescriptor",
    "builtin GetOwnPropertyDescriptor( implicit context: Context)(JSAny, Name): JSAny",
  ),
  (
    "builtin",
    "ReflectGetOwnPropertyDescriptor",
    "builtin ReflectGetOwnPropertyDescriptor( js-implicit context: NativeContext)(target: JSAny, propertyKey: JSAny): JSAny",
  ),
];

/// constexpr ... generates 'C++ 符号' 引用登记 (机械登记, 不猜值)
pub const CPP_GENERATES: &[(&str, &str)] =
  &[("kReturnUndefined", "OnNonExistent::kReturnUndefined")];

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 14);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "builtin").count() == 10);
  assert!(DECLS.iter().filter(|d| d.kind == "type").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "const").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "macro").count() == 1);
};
