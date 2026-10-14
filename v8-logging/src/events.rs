//! 1:1 转写自 `v8/src/logging/log.cc` 中不依赖 Isolate/Heap 的部分:
//! `CodeEventLogger::NameBuffer`、`TIMER_EVENTS_LIST` 表、
//! `V8FileLogger` 的纯格式化事件函数与 `Time()`。
//!
//! 【推迟登记】`V8FileLogger` 的 Start/Stop/Drain、JitLogger(ETW/Perf/JitCodeEvent
//! 缓冲)、`DebugServer` socket、`CodeEvent`(依赖 Isolate/Handle)、
//! `LogEventListener` 虚接口家族、`PROFILE()` 宏 —— 待 v8-heap/v8-objects/v8-init
//! 可用后回收。

use crate::{
  code_events::{CodeEvent, CodeTag, code_tag_name, event_name},
  flags_lite,
  log_file::{LogFile, MessageBuilder, format_hex_u32},
  time_base::ElapsedTimer,
};

/// 对应 `LogEventListener::CodeEvent` 的状态机 API 枚举
/// (`include/v8-callbacks.h` `enum LogEventStatus : int`)。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum LogEventStatus {
  /// 对应 `kStart = 0`。
  Start = 0,
  /// 对应 `kEnd = 1`。
  End = 1,
  /// 对应 `kLog = 2`。
  Log = 2,
}

/// 对应 `CodeEventLogger::NameBuffer::kUtf8BufferSize`。
pub const K_UTF8_BUFFER_SIZE: usize = 4096;
/// 对应 `CodeEventLogger::NameBuffer::kUtf16BufferSize`(保留常量)。
pub const K_UTF16_BUFFER_SIZE: usize = K_UTF8_BUFFER_SIZE;

/// `CodeEventLogger::NameBuffer`:定长 UTF-8 名字缓冲。
///
/// 与 C++ 一致:`kUtf8BufferSize = 4096`;`AppendBytes` 越界钳制(不 panic);
/// `AppendInt`/`AppendHex` 走 `SNPrintF("%d"/"%x")` 语义并要求完整写入才推进。
/// 【推迟登记】`AppendName`/`AppendString(Tagged<String>)`(依赖堆字符串)。
pub struct NameBuffer {
  utf8_pos: usize,
  utf8_buffer: [u8; K_UTF8_BUFFER_SIZE],
}

impl NameBuffer {
  pub fn new() -> Self {
    Self {
      utf8_pos: 0,
      utf8_buffer: [0; K_UTF8_BUFFER_SIZE],
    }
  }

  /// 对应 `Reset()`。
  #[inline]
  pub fn reset(&mut self) {
    self.utf8_pos = 0;
  }

  /// 对应 `Init(CodeTag tag)`:写入标签名 + ':'。
  pub fn init(&mut self, tag: CodeTag) {
    self.reset();
    self.append_bytes(code_tag_name(tag).as_bytes());
    self.append_byte(b':');
  }

  /// 对应 `AppendBytes(const char* bytes, size_t size)`(钳制截断)。
  pub fn append_bytes(&mut self, bytes: &[u8]) {
    let size = bytes.len().min(K_UTF8_BUFFER_SIZE - self.utf8_pos);
    self.utf8_buffer[self.utf8_pos..self.utf8_pos + size].copy_from_slice(&bytes[..size]);
    self.utf8_pos += size;
  }

  /// 对应 `AppendByte(char c)`。
  #[inline]
  pub fn append_byte(&mut self, c: u8) {
    if self.utf8_pos >= K_UTF8_BUFFER_SIZE {
      return;
    }
    self.utf8_buffer[self.utf8_pos] = c;
    self.utf8_pos += 1;
  }

  /// 对应 `AppendInt(int n)`:`SNPrintF("%d")`,要求完整容纳才推进。
  pub fn append_int(&mut self, n: i32) {
    if self.utf8_pos >= K_UTF8_BUFFER_SIZE {
      return;
    }
    let s = n.to_string();
    let bytes = s.as_bytes();
    let space = K_UTF8_BUFFER_SIZE - self.utf8_pos;
    // int size = SNPrintF(buffer, "%d", n); 若 size>0 且容纳则推进。
    let size = bytes.len();
    if size > 0 && self.utf8_pos + size <= K_UTF8_BUFFER_SIZE && size <= space {
      self.utf8_buffer[self.utf8_pos..self.utf8_pos + size].copy_from_slice(bytes);
      self.utf8_pos += size;
    }
  }

  /// 对应 `AppendHex(uint32_t n)`:`SNPrintF("%x")`(小写,无填充)。
  pub fn append_hex(&mut self, n: u32) {
    if self.utf8_pos >= K_UTF8_BUFFER_SIZE {
      return;
    }
    let s = format_hex_u32(n);
    let bytes = s.as_bytes();
    let space = K_UTF8_BUFFER_SIZE - self.utf8_pos;
    let size = bytes.len();
    if size > 0 && self.utf8_pos + size <= K_UTF8_BUFFER_SIZE && size <= space {
      self.utf8_buffer[self.utf8_pos..self.utf8_pos + size].copy_from_slice(bytes);
      self.utf8_pos += size;
    }
  }

  /// 对应 `get()` + `size()`:返回已写入内容。
  #[inline]
  pub fn get(&self) -> &[u8] {
    &self.utf8_buffer[..self.utf8_pos]
  }

  /// 对应 `size()`。
  #[inline]
  pub fn size(&self) -> usize {
    self.utf8_pos
  }
}

impl Default for NameBuffer {
  fn default() -> Self {
    Self::new()
  }
}

/// 对应 `TIMER_EVENTS_LIST(V)` 的机械展开
/// (`V(TimerName, expose)`,名称为 `"V8." #TimerName`)。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerEventName {
  RecompileSynchronous,
  RecompileConcurrent,
  CompileIgnition,
  CompileFullCode,
  OptimizeCode,
  CompileCode,
  CompileCodeBackground,
  DeoptimizeCode,
  Execute,
}

/// 表数据:(TimerName, "V8.TimerName", expose)。
pub static TIMER_EVENTS: [(TimerEventName, &str, bool); 9] = [
  (
    TimerEventName::RecompileSynchronous,
    "V8.RecompileSynchronous",
    true,
  ),
  (
    TimerEventName::RecompileConcurrent,
    "V8.RecompileConcurrent",
    true,
  ),
  (TimerEventName::CompileIgnition, "V8.CompileIgnition", true),
  (TimerEventName::CompileFullCode, "V8.CompileFullCode", true),
  (TimerEventName::OptimizeCode, "V8.OptimizeCode", true),
  (TimerEventName::CompileCode, "V8.CompileCode", true),
  (
    TimerEventName::CompileCodeBackground,
    "V8.CompileCodeBackground",
    true,
  ),
  (TimerEventName::DeoptimizeCode, "V8.DeoptimizeCode", true),
  (TimerEventName::Execute, "V8.Execute", true),
];

impl TimerEventName {
  /// 对应 `TimerEvent##TimerName::name()`。
  pub fn name(self) -> &'static str {
    for &(n, full, _) in TIMER_EVENTS.iter() {
      if n == self {
        return full;
      }
    }
    ""
  }

  /// 对应 `TimerEvent##TimerName::expose_to_api()`。
  pub fn expose_to_api(self) -> bool {
    for &(n, _, expose) in TIMER_EVENTS.iter() {
      if n == self {
        return expose;
      }
    }
    false
  }
}

/// 对应 `V8FileLogger::Time()`:可预测模式取单调毫秒×1000(微秒),
/// 否则取内部 `ElapsedTimer` 的微秒。
/// `timer` 对应 `timer_`(成员,调用方持有);时间源函数可注入以便测试。
pub fn file_logger_time(timer: &ElapsedTimer, monotonic_ms: fn() -> f64) -> i64 {
  if flags_lite::verify_predictable() {
    (monotonic_ms() * 1000.0) as i64
  } else {
    timer.elapsed().in_microseconds()
  }
}

/// 对应 `V8FileLogger::ProfilerBeginEvent()`(gate:无 flag,由 `is_logging` 决定)。
pub fn profiler_begin_event(log_file: &mut LogFile, is_logging: bool) {
  let Some(mut msg) = log_file.new_message_builder(is_logging) else {
    return;
  };
  msg.append_cstr("profiler");
  msg.append_separator();
  msg.append_cstr("begin");
  msg.append_separator();
  msg.append_raw_int(flags_lite::prof_sampling_interval());
  msg.write_to_log_file();
}

/// 对应 `V8FileLogger::StringEvent`(`if (v8_flags.log) UncheckedStringEvent(...)`)。
pub fn string_event(log_file: &mut LogFile, is_logging: bool, name: &str, value: &str) {
  if flags_lite::log() {
    unchecked_string_event(log_file, is_logging, name, value);
  }
}

/// 对应 `V8FileLogger::UncheckedStringEvent`。
///
/// # Safety 前置条件(对应原版 unchecked 语义,由参数显式化)
/// - 不检查 `v8_flags.log`(`string_event` 才做该 gate);调用方必须自行
///   决定日志开关,否则会在 `--log` 关闭时产生输出。
/// - `is_logging` 必须来自当前 `LogFile` 的日志激活状态(对应 C++
///   `NewMessageBuilder` 的双重检查),否则违背"关闭后不得再写"的约定。
pub fn unchecked_string_event(log_file: &mut LogFile, is_logging: bool, name: &str, value: &str) {
  let Some(mut msg) = log_file.new_message_builder(is_logging) else {
    return;
  };
  msg.append_cstr(name);
  msg.append_separator();
  msg.append_cstr(value);
  msg.write_to_log_file();
}

/// 对应 `V8FileLogger::IntPtrTEvent`(`if (!v8_flags.log) return;`)。
pub fn intptr_t_event(log_file: &mut LogFile, is_logging: bool, name: &str, value: i64) {
  if !flags_lite::log() {
    return;
  }
  let Some(mut msg) = log_file.new_message_builder(is_logging) else {
    return;
  };
  msg.append_cstr(name);
  msg.append_separator();
  // msg.AppendFormatString("%" V8PRIdPTR, value);
  msg.append_format_string_intptr(value);
  msg.write_to_log_file();
}

/// 对应 `V8FileLogger::SharedLibraryEvent`(`if (!v8_flags.prof_cpp) return;`)。
pub fn shared_library_event(
  log_file: &mut LogFile,
  is_logging: bool,
  library_path: &str,
  start: u64,
  end: u64,
  aslr_slide: i64,
) {
  if !flags_lite::prof_cpp() {
    return;
  }
  let Some(mut msg) = log_file.new_message_builder(is_logging) else {
    return;
  };
  msg.append_cstr("shared-library");
  msg.append_separator();
  msg.append_cstr(library_path);
  msg.append_separator();
  msg.append_pointer(start);
  msg.append_separator();
  msg.append_pointer(end);
  msg.append_separator();
  msg.append_raw_int(aslr_slide);
  msg.write_to_log_file();
}

/// 对应 `V8FileLogger::SharedLibraryEnd`。
pub fn shared_library_end_event(log_file: &mut LogFile, is_logging: bool) {
  if !flags_lite::prof_cpp() {
    return;
  }
  let Some(mut msg) = log_file.new_message_builder(is_logging) else {
    return;
  };
  msg.append_cstr("shared-library-end");
  msg.write_to_log_file();
}

/// 对应 `V8FileLogger::CurrentTimeEvent`(`DCHECK(v8_flags.log_timer_events)`)。
/// `timer`/时间源同 `file_logger_time`。
pub fn current_time_event(
  log_file: &mut LogFile,
  is_logging: bool,
  timer: &ElapsedTimer,
  monotonic_ms: fn() -> f64,
) {
  debug_assert!(flags_lite::log_timer_events());
  let Some(mut msg) = log_file.new_message_builder(is_logging) else {
    return;
  };
  msg.append_cstr("current-time");
  msg.append_separator();
  msg.append_raw_int(file_logger_time(timer, monotonic_ms));
  msg.write_to_log_file();
}

/// 对应 `V8FileLogger::TimerEvent(se, name)`(`DCHECK(log_timer_events)`)。
pub fn timer_event(
  log_file: &mut LogFile,
  is_logging: bool,
  se: LogEventStatus,
  name: &str,
  timer: &ElapsedTimer,
  monotonic_ms: fn() -> f64,
) {
  debug_assert!(flags_lite::log_timer_events());
  let Some(mut msg) = log_file.new_message_builder(is_logging) else {
    return;
  };
  match se {
    LogEventStatus::Start => msg.append_cstr("timer-event-start"),
    LogEventStatus::End => msg.append_cstr("timer-event-end"),
    LogEventStatus::Log => msg.append_cstr("timer-event"),
  }
  msg.append_separator();
  msg.append_cstr(name);
  msg.append_separator();
  msg.append_raw_int(file_logger_time(timer, monotonic_ms));
  msg.write_to_log_file();
}

/// 对应 `V8FileLogger::NewEvent`(`if (!v8_flags.log) return;`)。
pub fn new_event(log_file: &mut LogFile, is_logging: bool, name: &str, object: u64, size: u64) {
  if !flags_lite::log() {
    return;
  }
  let Some(mut msg) = log_file.new_message_builder(is_logging) else {
    return;
  };
  msg.append_cstr("new");
  msg.append_separator();
  msg.append_cstr(name);
  msg.append_separator();
  msg.append_pointer(object);
  msg.append_separator();
  // static_cast<unsigned int>(size) → os_ 直通(u32 十进制截断)。
  msg.append_raw_uint((size as u32) as u64);
  msg.write_to_log_file();
}

/// 对应 `V8FileLogger::DeleteEvent`。
pub fn delete_event(log_file: &mut LogFile, is_logging: bool, name: &str, object: u64) {
  if !flags_lite::log() {
    return;
  }
  let Some(mut msg) = log_file.new_message_builder(is_logging) else {
    return;
  };
  msg.append_cstr("delete");
  msg.append_separator();
  msg.append_cstr(name);
  msg.append_separator();
  msg.append_pointer(object);
  msg.write_to_log_file();
}

/// 对应 `AppendCodeCreateHeader(msg, tag, kind, address, size, time)`:
/// `code-creation,<tag>,<kind>,<time>,<addr>,<size>,`(其后由调用方追加名字)。
pub fn append_code_create_header(
  msg: &mut MessageBuilder<'_>,
  tag: CodeTag,
  kind: i32,
  address: u64,
  size: i32,
  time: u64,
) {
  // msg << CodeEvent::kCodeCreation << kNext << tag << kNext << int(kind)
  //     << kNext << time << kNext << ptr(address) << kNext << size << kNext
  msg.append_raw_str(event_name(CodeEvent::CodeCreation));
  msg.append_separator();
  msg.append_raw_str(code_tag_name(tag));
  msg.append_separator();
  msg.append_raw_int(kind as i64);
  msg.append_separator();
  msg.append_raw_uint(time);
  msg.append_separator();
  msg.append_pointer(address);
  msg.append_separator();
  msg.append_raw_int(size as i64);
  msg.append_separator();
}
