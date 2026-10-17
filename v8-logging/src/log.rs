//! 1:1 转写自 `v8/src/logging/log.cc` 中 `V8FileLogger` 的**核心日志门面**(纯格式化事件簇),
//! 与 events.rs(`StringEvent`/`TimerEvent`/`New`/`Delete`/`SharedLibrary`/`CodeCreateHeader`)
//! 互补, 不重叠。本模块承载 `--log-function-events` 事件族:
//! `ScriptEvent`(log.cc:1914)/ `FunctionEvent`(log.cc:1868)/ `CompilationCacheEvent`
//! (log.cc:1897)/ `CodeNameEvent`(log.cc:1835)/ `MoveEventInternal`(log.cc:1844)。
//!
//! 转写要点:
//! - C++ `MSG_BUILDER()` 宏(log.cc:1224)机械展开为 `new_message_builder(is_logging)`
//!   + 早退模式, 与 events.rs 完全一致(Rust 借用检查替代 `std::unique_ptr<MessageBuilder>`)。
//! - 原版门控读取全局 flag 位图 `v8_flags.*`。`log_function_events`/`log_code`/
//!   `predictable` 已补入 flags_lite, 本模块直接读
//!   [`crate::flags_lite`](对应 `v8_flags.log_function_events` 等, 不再参数下传)。
//! - `msg << double`(std::ostream 默认 `%g`, 精度 6)展开为 [`format_g`], 经
//!   `MessageBuilder::append_raw_str` 直通(不转义), 与 C++ `os_ << value` 一致。
//! - `msg << Event::x` → `event_name(CodeEvent::x)`(code_events.rs);
//!   `msg << (void*)` → `append_pointer`; `msg << int` → `append_raw_int`。
//!
//! 【推迟登记】`FunctionEvent` 的 `Tagged<String>` 重载(log.cc:1868, 走堆字符串转义)、
//! `ScriptDetails`/`EnsureLogScriptSource`(依赖 `Tagged<Script>`/`Isolate`)、
//! `VMStateIfMainThread<LOGGING>`(依赖 Isolate VM 状态)——待 v8-objects/v8-execution 回收。

use crate::{
  code_events::{CodeEvent, event_name},
  flags_lite,
  log_file::{LogFile, MessageBuilder},
};

/// 对应 `ScriptEventType`(`src/common/globals.h:1901`, 6 变体, 顺序 1:1)。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ScriptEventType {
  /// 对应 `kReserveId`。
  ReserveId = 0,
  /// 对应 `kCreate`。
  Create = 1,
  /// 对应 `kDeserialize`。
  Deserialize = 2,
  /// 对应 `kBackgroundCompile`。
  BackgroundCompile = 3,
  /// 对应 `kStreamingCompileBackground`。
  StreamingCompileBackground = 4,
  /// 对应 `kStreamingCompileForeground`。
  StreamingCompileForeground = 5,
}

/// 对应 `V8FileLogger::ScriptEvent` 内 switch 的字符串(log.cc:1919-1940)。
#[inline]
pub fn script_event_name(t: ScriptEventType) -> &'static str {
  match t {
    ScriptEventType::ReserveId => "reserve-id",
    ScriptEventType::Create => "create",
    ScriptEventType::Deserialize => "deserialize",
    ScriptEventType::BackgroundCompile => "background-compile",
    ScriptEventType::StreamingCompileBackground => "streaming-compile",
    ScriptEventType::StreamingCompileForeground => "streaming-compile-foreground",
  }
}

/// 对应 `V8FileLogger::ScriptEvent`(log.cc:1914)。
/// 门控 `v8_flags.log_function_events`(flags_lite);`time` 由调用方经
/// `file_logger_time` 求得(对应成员 `Time()`)。
/// 输出行: `script,<type>,<script_id>,<time>`。
pub fn script_event(
  log_file: &mut LogFile,
  is_logging: bool,
  t: ScriptEventType,
  script_id: i32,
  time: i64,
) {
  if !flags_lite::log_function_events() {
    return;
  }
  let Some(mut msg) = log_file.new_message_builder(is_logging) else {
    return;
  };
  // msg << "script" << kNext;
  msg.append_cstr("script");
  msg.append_separator();
  // switch(type) -> 字符串
  msg.append_cstr(script_event_name(t));
  msg.append_separator();
  // << script_id << kNext << Time()
  msg.append_raw_int(script_id as i64);
  msg.append_separator();
  msg.append_raw_int(time);
  msg.write_to_log_file();
}

/// 对应 log.cc:1852 匿名 `AppendFunctionMessage`。
/// 输出: `function,<reason>,<script_id>,<start>,<end>,<time_delta>,<time>,`(其后由调用方追加名字)。
/// 原版在 `v8_flags.predictable` 时写 `0.1`, 否则写 `time_delta`(均经 `os_ <<` 走 `%g`);
/// `predictable` 读 flags_lite。
// 签名 1:1 保留原版 `AppendFunctionMessage(msg, reason, script_id, time_delta,
// start_position, end_position, time)`, 不折叠。
pub fn append_function_message(
  msg: &mut MessageBuilder<'_>,
  reason: &str,
  script_id: i32,
  time_delta: f64,
  start_position: i32,
  end_position: i32,
  time: u64,
) {
  // msg << "function" << kNext << reason << kNext << script_id << kNext
  //     << start_position << kNext << end_position << kNext;
  msg.append_cstr("function");
  msg.append_separator();
  msg.append_cstr(reason);
  msg.append_separator();
  msg.append_raw_int(script_id as i64);
  msg.append_separator();
  msg.append_raw_int(start_position as i64);
  msg.append_separator();
  msg.append_raw_int(end_position as i64);
  msg.append_separator();
  // if (v8_flags.predictable) msg << 0.1; else msg << time_delta;
  let value = if flags_lite::predictable() {
    0.1
  } else {
    time_delta
  };
  msg.append_raw_str(&format_g(value));
  msg.append_separator();
  msg.append_raw_uint(time);
  msg.append_separator();
}

/// `FunctionEvent` 载荷 (原版 log.cc:1881 参数组打包, 消除参数展开)。
pub struct FunctionEventData<'a> {
  pub reason: &'a str,
  pub script_id: i32,
  pub time_delta: f64,
  pub start_position: i32,
  pub end_position: i32,
  pub time: u64,
  pub function_name: &'a [u8],
}

/// 对应 `V8FileLogger::FunctionEvent` 的 `const char*` 重载(log.cc:1881),
/// 以字节切片承载堆字符串版(`Tagged<String>` 重载推迟)。
/// 门控 `v8_flags.log_function_events`(flags_lite)。`time` 对应 `Time()`。
pub fn function_event(log_file: &mut LogFile, is_logging: bool, data: FunctionEventData<'_>) {
  if !flags_lite::log_function_events() {
    return;
  }
  let Some(mut msg) = log_file.new_message_builder(is_logging) else {
    return;
  };
  append_function_message(
    &mut msg,
    data.reason,
    data.script_id,
    data.time_delta,
    data.start_position,
    data.end_position,
    data.time,
  );
  // if (function_name_length > 0) msg.AppendString(function_name, len, is_one_byte);
  if !data.function_name.is_empty() {
    msg.append_string_vec(data.function_name);
  }
  msg.write_to_log_file();
}

/// `CompilationCacheEvent` 载荷 (原版 `Tagged<SharedFunctionInfo>` 解耦为
/// 整数入参组打包; 调用方从堆对象读出, script_id=-1 对应 `!IsScript` 分支)。
pub struct CompilationCacheEventData<'a> {
  pub action: &'a str,
  pub cache_type: &'a str,
  pub script_id: i32,
  pub start_position: i32,
  pub end_position: i32,
  pub time: i64,
}

/// 对应 `V8FileLogger::CompilationCacheEvent`(log.cc:1897)。
/// 门控 `v8_flags.log_function_events`(flags_lite)。`time` 对应 `Time()`。
/// 输出: `compilation-cache,<action>,<cache_type>,<script_id>,<start>,<end>,<time>`。
pub fn compilation_cache_event(
  log_file: &mut LogFile,
  is_logging: bool,
  data: CompilationCacheEventData<'_>,
) {
  if !flags_lite::log_function_events() {
    return;
  }
  let Some(mut msg) = log_file.new_message_builder(is_logging) else {
    return;
  };
  // msg << "compilation-cache" << kNext << action << kNext << cache_type
  //     << kNext << script_id << kNext << sfi->StartPosition() << kNext
  //     << sfi->EndPosition() << kNext << Time();
  msg.append_cstr("compilation-cache");
  msg.append_separator();
  msg.append_cstr(data.action);
  msg.append_separator();
  msg.append_cstr(data.cache_type);
  msg.append_separator();
  msg.append_raw_int(data.script_id as i64);
  msg.append_separator();
  msg.append_raw_int(data.start_position as i64);
  msg.append_separator();
  msg.append_raw_int(data.end_position as i64);
  msg.append_separator();
  msg.append_raw_int(data.time);
  msg.write_to_log_file();
}

/// 对应 `V8FileLogger::CodeNameEvent`(log.cc:1835)。
/// 原版门控 `code_name != nullptr` 与 `is_listening_to_code_events()`; 前者以空切片表达,
/// 后者以参数 `is_listening` 传入(log_file_->NewMessageBuilder 亦据此判定)。
/// 输出: `<snapshot-code-name>,<pos>,<code_name>`。
pub fn code_name_event(
  log_file: &mut LogFile,
  is_logging: bool,
  is_listening_to_code_events: bool,
  addr: u64,
  pos: i32,
  code_name: &str,
) {
  // if (code_name == nullptr) return;  (空名 → 不写)
  if code_name.is_empty() || !is_listening_to_code_events {
    return;
  }
  let Some(mut msg) = log_file.new_message_builder(is_logging) else {
    return;
  };
  // msg << Event::kSnapshotCodeName << kNext << pos << kNext << code_name;
  msg.append_raw_str(event_name(CodeEvent::SnapshotCodeName));
  msg.append_separator();
  msg.append_raw_int(pos as i64);
  msg.append_separator();
  msg.append_cstr(code_name);
  let _ = addr; // C++ 未在本事件写出 addr(仅用于 JIT logger), 保留入参签名对齐
  msg.write_to_log_file();
}

/// 对应 `V8FileLogger::MoveEventInternal`(log.cc:1844)。
/// 门控 `v8_flags.log_code`(flags_lite)。
/// 输出: `<event>,0x<from>,0x<to>`(`reinterpret_cast<void*>` → `append_pointer`)。
pub fn move_event_internal(
  log_file: &mut LogFile,
  is_logging: bool,
  event: CodeEvent,
  from: u64,
  to: u64,
) {
  if !flags_lite::log_code() {
    return;
  }
  let Some(mut msg) = log_file.new_message_builder(is_logging) else {
    return;
  };
  // msg << event << kNext << reinterpret_cast<void*>(from) << kNext
  //     << reinterpret_cast<void*>(to);
  msg.append_raw_str(event_name(event));
  msg.append_separator();
  msg.append_pointer(from);
  msg.append_separator();
  msg.append_pointer(to);
  msg.write_to_log_file();
}

/// 对应 `msg << double`(std::ostream 默认 `defaultfloat`, 精度 6, 等价 printf `%g`)。
/// 逐条复刻 `%g`: 有效数字 6; 若指数 `X < -4 || X >= 6` 用科学计数(尾数去尾零,
/// 指数 `e±DD` 至少 2 位), 否则定点(去尾零及孤立小数点)。非有限值回退 `to_string`。
/// 用于 function event 的 `time_delta` 输出, 与 message 测试比对强相关, 故 1:1 校验。
pub fn format_g(value: f64) -> String {
  // nan/inf: C++ locale 下 num_put 输出 "nan"/"inf"; 正常日志不出现, 保守回退。
  if !value.is_finite() {
    return value.to_string();
  }
  const P: usize = 6; // %g 默认有效数字
  if value == 0.0 {
    return "0".to_string();
  }
  let neg = value < 0.0;
  let av = value.abs();
  // 先取 `%e`(P-1 位小数 = P 位有效), 由 Rust 完成舍入并给出规范指数。
  let e_str = format!("{:.*e}", P - 1, av);
  let (mant, exp_s) = match e_str.split_once('e') {
    Some(pair) => pair,
    None => return value.to_string(),
  };
  let exp: i32 = match exp_s.parse() {
    Ok(v) => v,
    Err(_) => return value.to_string(),
  };
  let digits: String = mant.chars().filter(|c| *c != '.').collect();
  if digits.is_empty() {
    return "0".to_string();
  }
  let body = if exp < -4 || exp >= P as i32 {
    // 科学计数: d[.ddd]e±DD
    let d = strip_trailing_zeros(&digits);
    let mut s = String::new();
    s.push_str(&d[..1]);
    if d.len() > 1 {
      s.push('.');
      s.push_str(&d[1..]);
    }
    s.push('e');
    push_g_exponent(&mut s, exp);
    s
  } else {
    // 定点: 依 exp 定位小数点
    build_fixed(&digits, exp)
  };
  if neg { format!("-{body}") } else { body }
}

/// 去掉字符串尾部多余的 `0`, 至少保留一位。
fn strip_trailing_zeros(s: &str) -> String {
  let trimmed = s.trim_end_matches('0');
  if trimmed.is_empty() {
    "0".to_string()
  } else {
    trimmed.to_string()
  }
}

/// `%g` 指数部分: 强制符号 + 至少 2 位十进制(printf 行为)。
fn push_g_exponent(out: &mut String, exp: i32) {
  if exp < 0 {
    out.push('-');
  } else {
    out.push('+');
  }
  let a = exp.unsigned_abs();
  if a < 10 {
    out.push('0');
  }
  out.push_str(&a.to_string());
}

/// 由 6 位有效数字串 + 指数 `%g` 语义构造定点字符串(去尾零与孤立 `.`)。
fn build_fixed(digits: &str, exp: i32) -> String {
  let ds = strip_trailing_zeros(digits);
  let e = exp;
  let s = if e >= 0 {
    // 整数部分取前 e+1 位, 不足补零; 其余为小数。
    let int_len = (e as usize) + 1;
    if ds.len() <= int_len {
      let mut s = ds;
      for _ in 0..(int_len - s.len()) {
        s.push('0');
      }
      s
    } else {
      let (a, b) = ds.split_at(int_len);
      let frac = b.trim_end_matches('0');
      if frac.is_empty() {
        a.to_string()
      } else {
        format!("{a}.{frac}")
      }
    }
  } else {
    // 0.000ddd...
    let lead_zeros = (-(e + 1)) as usize;
    let frac = ds.trim_end_matches('0');
    if frac.is_empty() {
      "0".to_string()
    } else {
      format!("0.{}{frac}", "0".repeat(lead_zeros))
    }
  };
  // 若定点后仍残留 `.`(整数)或全零, 收敛。
  s.trim_end_matches('.').to_string()
}
