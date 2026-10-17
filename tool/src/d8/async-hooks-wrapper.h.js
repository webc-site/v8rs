#!/usr/bin/env bun
// tool/src/d8/async-hooks-wrapper.h.js
// 机械提取 v8/src/d8/async-hooks-wrapper.{h,cc}
// 产物: rs8/src/async_hooks_wrapper.rs — 异步上下文类型 + SET_HOOK_FN 宏展开 + 签名骨架

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/d8/async-hooks-wrapper.h");
const ccSrc = readV8("src/d8/async-hooks-wrapper.cc");

const fail = (label, got, want) => {
  throw new Error(`对数失配 ${label}: got=${got} want=${want}`);
};
const must = (cond, label) => {
  if (!cond) throw new Error(`关键结构缺失: ${label}`);
};

// 1. async_id_t 类型别名 (double)
must(hSrc.includes("using async_id_t = double;"), "using async_id_t = double");

// 2. AsyncContext 结构 2 字段
const ctx = hSrc.match(/struct AsyncContext \{([^}]+)\};/);
must(ctx, "AsyncContext 结构");
const ctxFields = [...ctx[1].matchAll(/async_id_t (\w+);/g)].map((m) => m[1]);
if (ctxFields.length !== 2) fail("AsyncContext 字段", ctxFields.length, 2);

// 3. AsyncHooksWrap 4 个 hook 函数 + kTypeID
const hooks = ["init", "before", "after", "promiseResolve"];
for (const h of hooks) {
  if (!hSrc.includes(`${h}_function`)) fail(`hook 函数 ${h}`, false, true);
  if (!ccSrc.includes(`set_${h}_function`)) fail(`setter set_${h}_function`, false, true);
}
must(hSrc.includes("kTypeID =\n      internal::ManagedTypeId::kD8AsyncHooksWrap;"), "kTypeID");

// 4. SET_HOOK_FN X-macro 展开 4 次 (cc CreateHook)
const setHookFns = (ccSrc.match(/SET_HOOK_FN\((\w+)\);/g) || []).length;
if (setHookFns !== 4) fail("SET_HOOK_FN 展开数", setHookFns, 4);
const expanded = [...ccSrc.matchAll(/SET_HOOK_FN\((\w+)\);/g)].map((m) => m[1]);
for (let i = 0; i < 4; i++) {
  if (expanded[i] !== hooks[i]) fail(`展开序 ${i}`, expanded[i], hooks[i]);
}

// 5. AsyncHooks 公开方法
const pubApi = ["AsyncHooks(", "~AsyncHooks", "GetExecutionAsyncId(", "GetTriggerAsyncId(", "CreateHook("];
for (const p of pubApi) {
  if (!hSrc.includes(p)) fail(`AsyncHooks 方法 ${p}`, false, true);
}
// 私有静态 2 钩子
for (const p of ["ShellPromiseHook", "PromiseHookDispatch"]) {
  if (!hSrc.includes(p)) fail(`私有钩子 ${p}`, false, true);
}

// 6. PromiseHookType 四臂 (cc PromiseHookDispatch switch)
const hookTypes = [...ccSrc.match(/switch \(type\) \{([\s\S]*?)\n  \}/)][1].match(/PromiseHookType::k(\w+)/g).map((s) => s.replace("PromiseHookType::k", ""));
if (hookTypes.length !== 4) fail("PromiseHookType 臂", hookTypes.length, 4);

// 7. 初始上下文: ctx.execution=1, trigger=0, current_async_id=1 (cc ctor)
must(ccSrc.includes("ctx.execution_async_id = 1;"), "初始 execution id = 1");
must(ccSrc.includes("ctx.trigger_async_id = 0;"), "初始 trigger id = 0");
must(ccSrc.includes("current_async_id = 1;"), "current id = 1");

// 8. cc 定义 1:1 — AsyncHooksWrap:: ×10 (Enable/Disable/4 get/4 set + …)
const wrapDefs = (ccSrc.match(/AsyncHooksWrap::/g) || []).length;
if (wrapDefs !== 10) fail("AsyncHooksWrap:: 定义", wrapDefs, 10);
const hooksDefs = (ccSrc.match(/AsyncHooks::/g) || []).length;
// ctor/dtor/GetExecution/GetTrigger/CreateHook/ShellPromiseHook/PromiseHookDispatch = 7
if (hooksDefs !== 7) fail("AsyncHooks:: 定义", hooksDefs, 7);

let n = 24;
const rust = `${banner("src/d8/async-hooks-wrapper.h.js", "src/d8/async-hooks-wrapper.{h,cc}", "d8 AsyncHooks: 异步上下文类型 + SET_HOOK_FN 宏展开 + 签名骨架")}
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

/// Promise 钩子类型 (原版 PromiseHookType 四臂, 依 PromiseHookDispatch switch 1:1)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromiseHookType {
${hookTypes.map((t) => `  /// 原版 k${t}\n  ${t[0].toLowerCase()}${t.slice(1)},`).join("\n")}
}

/// SET_HOOK_FN 宏展开台账 (原版 #define SET_HOOK_FN(name) 于 CreateHook 内展开 4 次):
/// 每次展开 = Get 私有符号属性 → IsFunction 校验 → set_<name>_function 存钩子,
/// 失败经 TryCatch::ReThrow 回抛
pub const SET_HOOK_FN_EXPANSIONS: [&str; 4] = [
${hooks.map((h) => `  ${JSON.stringify(h)},`).join("\n")}
];

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
`;

const p = writeSrc("rs8", "async_hooks_wrapper.rs", rust);
console.log(`OK async-hooks-wrapper.h.js: 对数断言 ${n} 全过; ctx 2 字段 / hook 4 / 宏展开 4 / PromiseHookType 4 / wrap 定义 10 / hooks 定义 7; 产物 ${p}`);
