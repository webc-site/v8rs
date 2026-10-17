// 由 tool/src/builtins/function.tq.js 从 v8/src/builtins/function.tq 生成; 修订就地改, 复跑覆盖
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

/// 原版顶层声明清单 (共 10 条)
pub const DECLS: &[TqDecl] = &[
  ("namespace", "function", "namespace function"),
  (
    "macro",
    "OrdinaryHasInstance",
    "macro OrdinaryHasInstance(Context, Object, Object): JSAny",
  ),
  (
    "builtin",
    "FunctionPrototypeHasInstance",
    "builtin FunctionPrototypeHasInstance( js-implicit context: NativeContext, receiver: JSAny)(value: JSAny): JSAny",
  ),
  (
    "builtin",
    "FunctionPrototypeBind",
    "builtin FunctionPrototypeBind( js-implicit context: Context)(JSFunction, JSAny, int32, DispatchHandle): JSAny",
  ),
  (
    "const",
    "kLengthDescriptorIndex",
    "const kLengthDescriptorIndex: constexpr int32 generates 'JSFunctionOrBoundFunctionOrWrappedFunction::kLengthDescriptorIndex'",
  ),
  (
    "const",
    "kNameDescriptorIndex",
    "const kNameDescriptorIndex: constexpr int32 generates 'JSFunctionOrBoundFunctionOrWrappedFunction::kNameDescriptorIndex'",
  ),
  (
    "const",
    "kMinDescriptorsForFastBindAndWrap",
    "const kMinDescriptorsForFastBindAndWrap: constexpr int31 generates 'JSFunction::kMinDescriptorsForFastBindAndWrap'",
  ),
  (
    "const",
    "kCodeMaxArguments",
    "const kCodeMaxArguments: constexpr intptr generates 'Code::kMaxArguments'",
  ),
  (
    "macro",
    "CheckAccessor",
    "macro CheckAccessor( implicit context: Context)(array: DescriptorArray, index: constexpr int32, name: Name): void labels Slow",
  ),
  (
    "builtin",
    "FastFunctionPrototypeBind",
    "builtin FastFunctionPrototypeBind( js-implicit context: NativeContext, receiver: JSAny, newTarget: JSAny, target: JSFunction)(...arguments): JSAny",
  ),
];

/// constexpr ... generates 'C++ 符号' 引用登记 (机械登记, 不猜值)
pub const CPP_GENERATES: &[(&str, &str)] = &[
  (
    "kLengthDescriptorIndex",
    "JSFunctionOrBoundFunctionOrWrappedFunction::kLengthDescriptorIndex",
  ),
  (
    "kNameDescriptorIndex",
    "JSFunctionOrBoundFunctionOrWrappedFunction::kNameDescriptorIndex",
  ),
  (
    "kMinDescriptorsForFastBindAndWrap",
    "JSFunction::kMinDescriptorsForFastBindAndWrap",
  ),
  ("kCodeMaxArguments", "Code::kMaxArguments"),
];

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 10);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "macro").count() == 2);
  assert!(DECLS.iter().filter(|d| d.kind == "builtin").count() == 3);
  assert!(DECLS.iter().filter(|d| d.kind == "const").count() == 4);
};
