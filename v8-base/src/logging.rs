//! 原版 v8/src/base/logging.cc/h — base 层 CHECK/DCHECK/FATAL 基础设施
//!
//! 红线 (一章): 日志统一走 `log` crate, 本 crate 禁自造日志门面. 但 v8-base 目前的
//! Cargo.toml 未引入 log (禁 cargo add), 因此 base 层的 fatal/dcheck 输出直接落 stderr,
//! 与原版 `OS::PrintError` 语义一致; **上层 `v8-logging` crate 负责把 `log::log!` 桥接
//! 到本模块的 `set_fatal_function`/`set_dcheck_function` 钩子**, 见 readme/zh.md 系统杂项族.
//!
//! 原版 API 映射:
//! - `SetPrintStackTrace` → [`set_print_stack_trace`]
//! - `PrintStackTraceIfAvailable` → [`print_stack_trace_if_available`]
//! - `SetDcheckFunction` → [`set_dcheck_function`]
//! - `SetFatalFunction` → [`set_fatal_function`]
//! - `V8_Dcheck` → [`v8_dcheck`] / [`v8_dcheck_loc`]
//! - `V8_Fatal` → [`v8_fatal`] / [`v8_fatal_loc`] (noreturn, 等价 `OS::Abort`)
//! - `FatalOOM` → [`fatal_oom`]
//! - `FatalNoSecurityImpact` → [`fatal_no_security_impact`]
//! - `PrettyPrintChar` (匿名 namespace) → [`pretty_print_char`] (导出以便测试)
//! - `MakeCheckOpString` 泛型 → [`make_check_op_string`] (受限 Rust 版本)
//!
//! V8_LOGGING_LEVEL == 2 分支 (携带 file/line) 与 == 0/1 (不携带) 通过不同入口承接,
//! 官方发布构建 (== 0) 的 FATAL 折叠宏由调用方按 cfg 决定, 本模块只提供 == 1/== 2 语义.

use core::ffi::c_int;
use std::{
  cmp::{PartialEq, PartialOrd},
  fmt::{self as core_fmt, Arguments},
  io,
  io::{Write as _, stdout},
  process::{abort, exit},
  sync::{
    Mutex,
    atomic::{AtomicBool, Ordering},
  },
};

use crate::abort_mode::{
  controlled_crashes_are_harmless, dcheck_failures_are_ignored,
  fatal_errors_with_no_security_impact_should_exit,
};

/// 原版 `kUnimplementedCodeMessage`
pub const UNIMPLEMENTED_CODE_MESSAGE: &str = "unimplemented code";
/// 原版 `kUnreachableCodeMessage`
pub const UNREACHABLE_CODE_MESSAGE: &str = "unreachable code";

/// 原版 `enum class OOMType`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum OOMType {
  /// JS 堆耗尽
  JavaScript = 0,
  /// 进程耗尽
  Process = 1,
}

/// V8_LOGGING_LEVEL == 2 变体: 带 file/line 的 fatal/dcheck 回调类型.
pub type FatalFn = fn(file: &str, line: c_int, message: &str);
/// 栈打印回调类型 (原版 `void (*)()`).
pub type PrintStackTraceFn = fn();

/// 全局栈打印回调槽位; 对应 `g_print_stack_trace`.
static PRINT_STACK_TRACE: Mutex<Option<PrintStackTraceFn>> = Mutex::new(None);
/// 全局 dcheck 覆写槽位; 对应 `g_dcheck_function`.
static DCHECK_FUNCTION: Mutex<Option<FatalFn>> = Mutex::new(None);
/// 全局 fatal 覆写槽位; 对应 `g_fatal_function`.
static FATAL_FUNCTION: Mutex<Option<FatalFn>> = Mutex::new(None);
/// `V8_LOGGING_LEVEL` 编译期档位 (0=release, 1=verbose-release, 2=debug).
const LOGGING_LEVEL: u32 = if cfg!(debug_assertions) { 2 } else { 1 };
static DCHECK_ENABLED: AtomicBool = AtomicBool::new(cfg!(debug_assertions));

/// 原版 `V8::base::DCHECK_ENABLED` 语义: Debug 构建为 true.
#[must_use]
pub fn dcheck_enabled() -> bool {
  DCHECK_ENABLED.load(Ordering::Relaxed)
}

/// 原版 `SetPrintStackTrace(void (*)())`.
pub fn set_print_stack_trace(f: Option<PrintStackTraceFn>) {
  if let Ok(mut slot) = PRINT_STACK_TRACE.lock() {
    *slot = f;
  }
}

/// 原版 `PrintStackTraceIfAvailable()`.
pub fn print_stack_trace_if_available() {
  if let Ok(guard) = PRINT_STACK_TRACE.lock()
    && let Some(f) = *guard
  {
    f();
  }
}

/// 原版 `SetDcheckFunction`; 传 `None` 恢复默认处理 (等价原版传 nullptr).
pub fn set_dcheck_function(f: Option<FatalFn>) {
  if let Ok(mut slot) = DCHECK_FUNCTION.lock() {
    *slot = f;
  }
}

/// 原版 `SetFatalFunction`.
pub fn set_fatal_function(f: Option<FatalFn>) {
  if let Ok(mut slot) = FATAL_FUNCTION.lock() {
    *slot = f;
  }
}

/// 原版 `OS::PrintError`: 直写 stderr, 无格式化 (由调用点 `format_args!` 完成).
fn print_error(args: Arguments<'_>) {
  let stderr = io::stderr();
  let mut lock = stderr.lock();
  // 写入 stderr 的错误无法在此恢复, 与原版一致 (失败即静默).
  let _ = lock.write_fmt(args);
  let _ = lock.flush();
}

/// 原版匿名 `PrettyPrintChar(int ch)`.
///
/// 逐条对齐 `logging.cc:28-55`:
/// - 特殊控制字符走 `#ch` 字面串化输出 (源码形式, e.g. `'\0'`)
/// - 其他可打印: `'\'' << ch << '\''`, **注意 C++ 侧 ch 是 int, 会打印数字**, 例 `'a'` → `'97'`
/// - 非可打印: `std::hex << "\\x" << (unsigned)ch`
#[must_use]
pub fn pretty_print_char(ch: i32) -> String {
  // 特殊字符映射到"源码字面串"形式 (即 `#ch` 展开的 4 字符)
  let literal: Option<&str> = match ch {
    0 => Some("'\\0'"),
    7 => Some("'\\a'"),
    8 => Some("'\\b'"),
    9 => Some("'\\t'"),
    10 => Some("'\\n'"),
    11 => Some("'\\v'"),
    12 => Some("'\\f'"),
    13 => Some("'\\r'"),
    39 => Some("'\\''"),
    92 => Some("'\\\\'"),
    _ => None,
  };
  if let Some(s) = literal {
    return s.to_string();
  }
  // `std::isprint` 定义: [0x20, 0x7e]
  if (0x20..0x7f).contains(&ch) {
    // 与原版一致: 数字输出 (int), 不是字符
    format!("'{ch}'")
  } else {
    // `std::hex << (unsigned)ch`
    format!("\\x{:x}", ch as u32)
  }
}

/// 原版 `MakeCheckOpString<Lhs, Rhs>` — Rust 侧泛型限制到 Display 类型, 输出格式一致:
/// - `msg (lhs vs. rhs)` (若两侧短 <=50)
/// - `msg\n   lhs\n vs.\n   rhs\n` (超长)
///
/// 不可打印侧使用 `<unprintable>`; Rust Display 默认可打印, 因此调用方要么提供
/// `Display` 实现要么显式走 [`make_check_op_string_unprintable`].
#[must_use]
pub fn make_check_op_string<Lhs: core_fmt::Display, Rhs: core_fmt::Display>(
  lhs: &Lhs,
  rhs: &Rhs,
  msg: &str,
) -> String {
  build_check_op_string(msg, lhs.to_string(), rhs.to_string())
}

/// 单侧不可打印的版本 (对应原版的 `if constexpr (!kLhsIsPrintable)` 分支).
#[must_use]
pub fn make_check_op_string_unprintable(msg: &str) -> String {
  build_check_op_string(
    msg,
    "<unprintable>".to_string(),
    "<unprintable>".to_string(),
  )
}

fn build_check_op_string(msg: &str, lhs_str: String, rhs_str: String) -> String {
  const MAX_INLINE: usize = 50;
  if lhs_str.len().max(rhs_str.len()) <= MAX_INLINE {
    format!("{msg} ({lhs_str} vs. {rhs_str})")
  } else {
    format!("{msg}\n   {lhs_str}\n vs.\n   {rhs_str}\n")
  }
}

/// 原版 `V8_Dcheck(message)` (LOGGING_LEVEL != 2).
///
/// - 若 `DcheckFailuresAreIgnored()` 为真 (AbortMode), 仅打印 "Ignoring" 并返回.
/// - 否则调用注册的 dcheck_function (默认 `DefaultDcheckHandler` → `v8_fatal`).
pub fn v8_dcheck(message: &str) {
  v8_dcheck_loc("", 0, message);
}

/// 原版 `V8_Dcheck(file, line, message)` (LOGGING_LEVEL == 2).
pub fn v8_dcheck_loc(file: &str, line: c_int, message: &str) {
  if dcheck_failures_are_ignored() {
    if LOGGING_LEVEL == 2 {
      print_error(format_args!(
        "# Ignoring debug check failure in {file}, line {line}: {message}\n"
      ));
    } else {
      print_error(format_args!("# Ignoring debug check failure: {message}\n"));
    }
    return;
  }
  // 默认 dcheck handler: 走 v8_fatal, 与 C++ `DefaultDcheckHandler` 一致.
  let handler = DCHECK_FUNCTION.lock().ok().and_then(|g| *g);
  match handler {
    Some(f) => f(file, line, message),
    None => default_dcheck_handler(file, line, message),
  }
}

fn default_dcheck_handler(file: &str, line: c_int, message: &str) {
  if LOGGING_LEVEL == 2 {
    v8_fatal_loc(file, line, format_args!("Debug check failed: {message}."));
  } else {
    v8_fatal(format_args!("Debug check failed: {message}."));
  }
}

/// 原版 `V8_Fatal(format, ...)` (LOGGING_LEVEL != 2) — **noreturn**.
///
/// Rust 侧 `Arguments` 承接 format_args!; 打印行为与原版逐条对齐, 最终以 `abort()` 终止.
pub fn v8_fatal(args: Arguments<'_>) -> ! {
  v8_fatal_loc_inner("", 0, args)
}

/// 原版 `V8_Fatal(file, line, format, ...)` (LOGGING_LEVEL == 2) — **noreturn**.
pub fn v8_fatal_loc(file: &str, line: c_int, args: Arguments<'_>) -> ! {
  v8_fatal_loc_inner(file, line, args)
}

fn v8_fatal_loc_inner(file: &str, line: c_int, args: Arguments<'_>) -> ! {
  // 格式化消息到本地缓冲 (原版 FailureMessage kMessageBufferSize = 512).
  // 与原版一致: 格式化仅用于传给注册的 fatal_function, 打印时再次展开 args.
  let mut msg_buf = Vec::<u8>::with_capacity(512);
  {
    struct Sink<'a>(&'a mut Vec<u8>);
    impl core_fmt::Write for Sink<'_> {
      fn write_str(&mut self, s: &str) -> core_fmt::Result {
        self.0.extend_from_slice(s.as_bytes());
        Ok(())
      }
    }
    let mut sink = Sink(&mut msg_buf);
    let _ = core_fmt::Write::write_fmt(&mut sink, args);
  }
  let message = String::from_utf8_lossy(&msg_buf);

  if let Some(f) = FATAL_FUNCTION.lock().ok().and_then(|g| *g) {
    f(file, line, &message);
  }

  let _ = stdout().flush();
  let _ = io::stderr().flush();

  if controlled_crashes_are_harmless() {
    if LOGGING_LEVEL == 2 {
      print_error(format_args!(
        "\n\n#\n# Safely terminating process due to error in {file}, line {line}\n# "
      ));
    } else {
      print_error(format_args!("\n\n#\n# Safely terminating process\n# "));
    }
    print_error(format_args!(
      "The following harmless error was encountered: "
    ));
  } else if LOGGING_LEVEL == 2 {
    print_error(format_args!(
      "\n\n#\n# Fatal error in {file}, line {line}\n# "
    ));
  } else {
    print_error(format_args!("\n\n#\n# Fatal error\n# "));
  }

  print_error(format_args!("{message}"));
  // 原版还会打印 FailureMessage 对象地址; Rust 无稳定栈地址, 保留消息结构但地址置 0.
  print_error(format_args!("\n#\n#\n#\n#FailureMessage Object: 0x0"));

  print_stack_trace_if_available();
  let _ = io::stderr().flush();
  // 与原版 OS::Abort() 一致: 不走 panic 路径.
  abort()
}

/// 原版 `FatalOOM(OOMType, msg)`.
pub fn fatal_oom(t: OOMType, msg: &str) -> ! {
  let type_str = match t {
    OOMType::Process => "process",
    OOMType::JavaScript => "JavaScript",
  };
  print_error(format_args!(
    "\n\n#\n# Fatal {type_str} out of memory: {msg}\n#"
  ));
  print_stack_trace_if_available();
  let _ = io::stderr().flush();
  if fatal_errors_with_no_security_impact_should_exit() {
    exit(-1);
  } else {
    abort()
  }
}

/// 原版 `FatalNoSecurityImpact(format, ...)` — **noreturn**.
pub fn fatal_no_security_impact(args: Arguments<'_>) -> ! {
  print_error(format_args!(
    "\n\n#\n# Fatal error with no security impact:\n# "
  ));
  print_error(args);
  print_error(format_args!("\n#\n"));
  print_stack_trace_if_available();
  let _ = io::stderr().flush();
  if fatal_errors_with_no_security_impact_should_exit() {
    exit(-1);
  } else {
    abort()
  }
}

/// `CHECK_WITH_MSG` 语义的 Rust 版: 条件失败即调用 `v8_fatal`; 成功返回 ().
/// 用于迁移 gtest 中 `CHECK_*` 断言到 Rust 用例.
#[inline]
pub fn check_with_msg(cond: bool, msg: &str) {
  if !cond {
    v8_fatal(format_args!("Check failed: {msg}."));
  }
}

/// `CHECK_EQ/NE/LT/LE/GT/GE` 通用比较: 返回 `Option<String>` — None 即通过.
///
/// 原版通过 `Check##NAME##Impl` 模板返回 `std::string*` 或 nullptr, Rust 用 Option 承接.
#[must_use]
pub fn check_op_eq<Lhs: PartialEq<Rhs> + core_fmt::Display, Rhs: core_fmt::Display>(
  lhs: &Lhs,
  rhs: &Rhs,
  msg: &str,
) -> Option<String> {
  if *lhs == *rhs {
    None
  } else {
    Some(make_check_op_string(lhs, rhs, msg))
  }
}

/// 与原版 `CHECK_NE` 一致.
#[must_use]
pub fn check_op_ne<Lhs: PartialEq<Rhs> + core_fmt::Display, Rhs: core_fmt::Display>(
  lhs: &Lhs,
  rhs: &Rhs,
  msg: &str,
) -> Option<String> {
  if *lhs != *rhs {
    None
  } else {
    Some(make_check_op_string(lhs, rhs, msg))
  }
}

/// 与原版 `CHECK_LT/LE/GT/GE` 共用 `PartialOrd` 通道.
#[must_use]
pub fn check_op_partial_ord<Lhs: PartialOrd<Rhs> + core_fmt::Display, Rhs: core_fmt::Display>(
  lhs: &Lhs,
  rhs: &Rhs,
  msg: &str,
  which: CmpOp,
) -> Option<String> {
  let ok = match which {
    CmpOp::Lt => *lhs < *rhs,
    CmpOp::Le => *lhs <= *rhs,
    CmpOp::Gt => *lhs > *rhs,
    CmpOp::Ge => *lhs >= *rhs,
  };
  if ok {
    None
  } else {
    Some(make_check_op_string(lhs, rhs, msg))
  }
}

/// `check_op_partial_ord` 的算子枚举.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CmpOp {
  Lt,
  Le,
  Gt,
  Ge,
}
