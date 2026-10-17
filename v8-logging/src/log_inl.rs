//! 1:1 转写自 `v8/src/logging/log-inl.h`。
//!
//! Rust 无 `-inl.h` 头(内联由编译器决定), 本模块承载 log-inl.h 中**可脱离 Isolate**
//! 的实现体; 其余(`TimerEventScope<TimerEvent>::LogTimerEvent`, log-inl.h:31-35)
//! 依赖 `Isolate`/`V8FileLogger::CallEventLogger`, 【推迟登记】待 v8-execution 回收
//! (其名字/expose 语义已由 events.rs 的 `TIMER_EVENTS`/`TimerEventName` 承载)。

use crate::code_events::CodeTag;

/// 对应 `V8FileLogger::ToNativeByScript`(log.h:305 声明, log-inl.h:18-29 实现)。
///
/// 原版第一参判定 `script->type() != Script::Type::kNative` 即原样返回;
/// `Tagged<Script>` 属对象层, 此处解耦为布尔 `is_native_script`(调用方读出
/// `Script::Type::kNative`), 分支映射与原版逐条一致:
/// `kFunction → kNativeFunction`, `kScript → kNativeScript`, 其余原样。
#[inline]
pub fn to_native_by_script(tag: CodeTag, is_native_script: bool) -> CodeTag {
  if !is_native_script {
    return tag;
  }
  match tag {
    CodeTag::Function => CodeTag::NativeFunction,
    CodeTag::Script => CodeTag::NativeScript,
    other => other,
  }
}
