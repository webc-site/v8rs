// 由 tool/src/d8/async-hooks-wrapper.h.js 从 v8/src/d8/async-hooks-wrapper.{h,cc} 生成; 修订就地改, 复跑覆盖
// d8 AsyncHooks: 异步上下文类型 + SET_HOOK_FN 宏展开 + 签名骨架

//! d8 异步钩子包装 — 原版 namespace v8::AsyncHooks 族
//!
//! 语义: 以 PromiseHook 为底座模拟 Node 风格 async_hooks — createHook(init/before/
//! after/promiseResolve), async id 以 f64 承载。

/// 异步 id 类型 (原版 using async_id_t = double)
pub type AsyncId = f64;

/// 异步上下文 (原版 struct AsyncContext, 2 字段)
#[derive(Debug, Clone, Copy)]
pub struct AsyncContext {
  /// 原版 execution_async_id
  pub execution_async_id: AsyncId,
  /// 原版 trigger_async_id
  pub trigger_async_id: AsyncId,
}

impl AsyncContext {
  /// 创建新的初始异步上下文
  pub const fn initial() -> Self {
    Self {
      execution_async_id: INITIAL_EXECUTION_ASYNC_ID,
      trigger_async_id: INITIAL_TRIGGER_ASYNC_ID,
    }
  }

  /// 判断是否为有效异步执行 ID (严格为有限正数)
  pub fn is_valid_execution(&self) -> bool {
    self.execution_async_id.is_finite() && self.execution_async_id > 0.0
  }
}

/// Promise 钩子类型 (原版 PromiseHookType 四臂, 依 PromiseHookDispatch switch 1:1)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromiseHookType {
  /// 原版 kInit
  Init,
  /// 原版 kBefore
  Before,
  /// 原版 kAfter
  After,
  /// 原版 kResolve
  Resolve,
}

/// SET_HOOK_FN 宏展开台账 (原版 #define SET_HOOK_FN(name) 于 CreateHook 内展开 4 次):
/// 每次展开 = Get 私有符号属性 → IsFunction 校验 → set_<name>_function 存钩子,
/// 失败经 TryCatch::ReThrow 回抛
pub const SET_HOOK_FN_EXPANSIONS: [&str; 4] = ["init", "before", "after", "promiseResolve"];

/// 初始异步 id 常量 (原版 AsyncHooks::AsyncHooks ctor)
pub const INITIAL_EXECUTION_ASYNC_ID: AsyncId = 1.0;
pub const INITIAL_TRIGGER_ASYNC_ID: AsyncId = 0.0;

// —— AsyncHooksWrap 签名骨架 (h 6 方法 + 4 对 get/set 存取器, cc 定义 10 处 1:1) ——
// new(isolate) / enable() → enabled_ = true / disable() → enabled_ = false
// is_enabled() -> bool
// init_function()/before_function()/after_function()/promiseResolve_function()
// set_{init,before,after,promiseResolve}_function(Local<Function>)
// 常量: kTypeID = ManagedTypeId::kD8AsyncHooksWrap (CppGCManaged 包装识别)

// —— AsyncHooks 签名骨架 (h 5 公开 + 2 私有静态, cc 定义 7 处 1:1) ——
// new(v8_isolate) — 装配 ctor 模板/enable/disable/私有符号/ShellPromiseHook
// drop() — SetPromiseHook(nullptr) + async_wraps_.clear()
// get_execution_async_id() -> AsyncId — 栈顶
// get_trigger_async_id() -> AsyncId — 栈顶
// create_hook(info: &FunctionCallbackInfo) -> Local<Object> — SET_HOOK_FN ×4 展开
// ShellPromiseHook(type, promise, parent) — kInit 压栈+写私有符号 / kBefore 压栈 /
//   kAfter 弹栈; 终止后 skip_after_termination_ 熔断
// PromiseHookDispatch(type, promise, parent, wrap, hooks) — 按 wrap 使能分发 4 钩子,
//   init 带 4 参 (async_id, "PROMISE", trigger_id, promise)

// 语义承接点台账:
//   Persistent<Function>/Private 符号存取 → Rust Global 句柄承接
//   std::stack<AsyncContext> → Vec 栈式使用
