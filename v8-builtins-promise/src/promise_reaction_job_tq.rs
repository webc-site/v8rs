// 由 tool/src/builtins/promise-reaction-job.tq.js 从 v8/src/builtins/promise-reaction-job.tq 生成; 修订就地改, 复跑覆盖
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
    "macro",
    "RejectPromiseReactionJob",
    "macro RejectPromiseReactionJob( context: Context, promiseOrCapability: JSPromise|PromiseCapability|Undefined, reason: JSAny, reactionType: constexpr PromiseReactionType): JSAny",
  ),
  (
    "macro",
    "FuflfillPromiseReactionJob",
    "macro FuflfillPromiseReactionJob( context: Context, promiseOrCapability: JSPromise|PromiseCapability|Undefined, result: JSAny, reactionType: constexpr PromiseReactionType): JSAny",
  ),
  (
    "builtin",
    "ResumeGeneratorTrampoline",
    "builtin ResumeGeneratorTrampoline(Context, JSAny, JSGeneratorObject): JSAny",
  ),
  (
    "builtin",
    "AsyncGeneratorResumeNext",
    "builtin AsyncGeneratorResumeNext(Context, JSAsyncGeneratorObject): JSAny",
  ),
  (
    "macro",
    "ResumeGenerator",
    "macro ResumeGenerator( implicit context: Context)(generator: JSGeneratorObject, argument: JSAny, reactionType: constexpr PromiseReactionType): void",
  ),
  (
    "macro",
    "PromiseReactionJob",
    "macro PromiseReactionJob( context: Context, argument: JSAny, handler: Callable|JSGeneratorObject|Undefined, promiseOrCapability: JSPromise|PromiseCapability|Undefined, reactionType: constexpr PromiseReactionType): JSAny",
  ),
  (
    "macro",
    "PromiseFulfillReactionJobInline",
    "macro PromiseFulfillReactionJobInline( implicit context: Context)(value: JSAny, handler: JSAny, promiseOrCapability: HeapObject): void labels Slow",
  ),
  (
    "macro",
    "PromiseRejectReactionJobInline",
    "macro PromiseRejectReactionJobInline( implicit context: Context)(reason: JSAny, handler: JSAny, promiseOrCapability: HeapObject): void labels Slow",
  ),
  (
    "builtin",
    "PromiseFulfillReactionJob",
    "builtin PromiseFulfillReactionJob( implicit context: Context)(value: JSAny, handler: Callable|JSGeneratorObject|Undefined, promiseOrCapability: JSPromise|PromiseCapability|Undefined): JSAny",
  ),
  (
    "builtin",
    "PromiseRejectReactionJob",
    "builtin PromiseRejectReactionJob( implicit context: Context)(reason: JSAny, handler: Callable|JSGeneratorObject|Undefined, promiseOrCapability: JSPromise|PromiseCapability|Undefined): JSAny",
  ),
];

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 11);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "macro").count() == 6);
  assert!(DECLS.iter().filter(|d| d.kind == "builtin").count() == 4);
};
