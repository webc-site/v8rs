// 由 tool/src/builtins/promise-any.tq.js 从 v8/src/builtins/promise-any.tq 生成; 修订就地改, 复跑覆盖
// Torque 声明骨架 — 声明清单/常量直译/C++ 符号引用, 函数体不展开
//! 任务映射: v8-builtins-promise; 聚合基线脚本另产同源模块 (并存不覆盖)

/// Torque 声明条目 (kind: namespace/type/const/macro/builtin/struct/enum/class/import)
#[derive(Debug, Clone, Copy)]
pub struct TqDecl {
  pub kind: &'static str,
  pub name: &'static str,
  /// 原版签名 (空白规整; 含 labels/js-implicit 等修饰)
  pub sig: &'static str,
}

/// 原版顶层声明清单 (共 11 条)
pub const DECLS: &[TqDecl] = &[
  ("namespace", "promise", "namespace promise"),
  (
    "type",
    "PromiseAnyRejectElementContext",
    "type PromiseAnyRejectElementContext extends FunctionContext",
  ),
  (
    "enum",
    "PromiseAnyRejectElementContextSlots",
    "enum PromiseAnyRejectElementContextSlots extends intptr constexpr 'PromiseBuiltins::PromiseAnyRejectElementContextSlots'",
  ),
  (
    "macro",
    "CreatePromiseAnyRejectElementContext",
    "macro CreatePromiseAnyRejectElementContext( implicit context: Context)(capability: PromiseCapability, nativeContext: NativeContext): PromiseAnyRejectElementContext",
  ),
  (
    "const",
    "kPromiseAnyRejectElementClosureSharedFun",
    "const kPromiseAnyRejectElementClosureSharedFun: constexpr intptr generates 'RootIndex::PromiseAnyRejectElementClosureSharedFun'",
  ),
  (
    "macro",
    "CreatePromiseAnyRejectElementFunction",
    "macro CreatePromiseAnyRejectElementFunction( implicit context: Context)( rejectElementContext: PromiseAnyRejectElementContext, index: Smi, nativeContext: NativeContext): JSFunction",
  ),
  (
    "builtin",
    "PromiseAnyRejectElementClosure",
    "builtin PromiseAnyRejectElementClosure( js-implicit context: Context, receiver: JSAny, target: JSFunction)( value: JSAny): JSAny",
  ),
  (
    "macro",
    "PerformPromiseAny",
    "macro PerformPromiseAny( implicit context: Context)(nativeContext: NativeContext, iteratorRecord: iterator::IteratorRecord, constructor: Constructor, resultCapability: PromiseCapability, promiseResolveFunction: JSAny): JSAny labels Reject(J…",
  ),
  (
    "builtin",
    "PromiseAny",
    "builtin PromiseAny( js-implicit context: Context, receiver: JSAny)(iterable: JSAny): JSAny",
  ),
  (
    "macro",
    "ConstructAggregateError",
    "macro ConstructAggregateError( implicit context: Context)(errors: FixedArray): JSObject",
  ),
  (
    "macro",
    "PromiseAnyRejectElementSharedFunConstant",
    "macro PromiseAnyRejectElementSharedFunConstant(): SharedFunctionInfo",
  ),
];

/// constexpr ... generates 'C++ 符号' 引用登记 (机械登记, 不猜值)
pub const CPP_GENERATES: &[(&str, &str)] = &[(
  "kPromiseAnyRejectElementClosureSharedFun",
  "RootIndex::PromiseAnyRejectElementClosureSharedFun",
)];

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 11);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "type").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "enum").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "macro").count() == 5);
  assert!(DECLS.iter().filter(|d| d.kind == "const").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "builtin").count() == 2);
};
