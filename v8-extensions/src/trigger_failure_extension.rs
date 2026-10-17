// 由 tool/src/extensions/trigger-failure-extension.h.js 从 v8/src/extensions/trigger-failure-extension.{h,cc} 生成; 修订就地改, 复跑覆盖
// TriggerFailure 扩展元数据 + 3 触发器签名骨架

//! TriggerFailure 扩展 — 原版 namespace v8::internal::TriggerFailureExtension
//!
//! 语义: 注入 3 个立即触发断言失败的 native 函数, 用于测试 CHECK/DCHECK 通路。

/// TriggerFailure 扩展注册名
pub const TRIGGER_FAILURE_EXTENSION_NAME: &str = "v8/trigger-failure";

/// 注入 JS 的源码 (原版 kSource, 3 条声明拼接)
pub const TRIGGER_FAILURE_SOURCE: &str = "native function triggerCheckFalse();native function triggerAssertFalse();native function triggerSlowAssertFalse();";

/// 触发器分发表 (原版 GetNativeFunctionTemplate if/else 链, 1:1)
pub const DISPATCH: [(&str, &str, &str); 3] = [
  // (JS 名, 方法名, 函数体断言)
  ("triggerCheckFalse", "TriggerCheckFalse", "CHECK(false)"),
  ("triggerAssertFalse", "TriggerAssertFalse", "DCHECK(false)"),
  (
    "triggerSlowAssertFalse",
    "TriggerSlowAssertFalse",
    "SLOW_DCHECK(false)",
  ),
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(DISPATCH.len() == 3);
};

// —— 签名骨架 (h 声明 3 + GetNativeFunctionTemplate, cc 定义 1:1) ——
// public: GetNativeFunctionTemplate(isolate, name) — 按 name 字符串三分发
// public static: TriggerCheckFalse(info) — CHECK(false)
// public static: TriggerAssertFalse(info) — DCHECK(false)
// public static: TriggerSlowAssertFalse(info) — SLOW_DCHECK(false)
