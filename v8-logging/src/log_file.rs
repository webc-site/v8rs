//! 1:1 转写自 `v8/src/logging/log-file.h/.cc` 的可移植部分。
//!
//! 转写要点:
//! - C++ `MessageBuilder` 直接写 `OFStream`(FILE*);此处写 `OutputHandle`
//!   (stdout / 命名文件 / 临时文件),转义语义逐条对齐。
//! - C++ 的 `NoGarbageCollectionMutexGuard`/`base::Mutex` 由 Rust 借用检查
//!   (`&mut LogFile`)排他访问替代。
//! - 输出经行缓冲(对应 C++ `OFStream`/FILE* 的 stdio 缓冲):逐段入内存,
//!   `WriteToLogFile` 行尾一次性落盘;`Close`/`Drop` 兜底刷净(对应 `fflush`)。
//! - `operator<<` 模板特化展开为显式方法:
//!   `<< const char*` → [`MessageBuilder::append_cstr`](转义),
//!   `<< char` → [`MessageBuilder::append_char`](转义),
//!   `<< void*` → [`MessageBuilder::append_pointer`](`0x` + 十六进制),
//!   `<< LogSeparator` → [`MessageBuilder::append_separator`](原始 `,`),
//!   其余算术类型经 `os_ <<` 直通(不转义)→ `append_raw_*`。
//! - C++ `format_buffer_`(kMessageBufferSize)用于 `VSNPrintF` 中转;
//!   Rust 以 `LogFile::format_buffer: Vec<u8>` 复用同一缓冲做整数格式化,
//!   截断语义(`length == -1 → kMessageBufferSize`)对齐。
//!
//! 【推迟登记】`AppendString(Tagged<String>)`(堆字符串版;其转义循环由
//! [`MessageBuilder::append_string_utf16`] 1:1 承载)、`AppendSymbolName` /
//! `AppendSymbolNameDetails`(依赖 Heap/Handle/Symbol)。
//! `LogFile` 的 `V8FileLogger*` 依赖以 `is_logging` 参数注入。

use std::{
  env::temp_dir,
  fs::{File, OpenOptions, remove_file},
  io,
  io::{ErrorKind, Write},
  mem::take,
};

use crate::{Error, error::Result, flags_lite};

/// 消息格式化缓冲区大小(对应 `LogFile::kMessageBufferSize`)。
pub const K_MESSAGE_BUFFER_SIZE: usize = 2048;

/// 对应 `LogFile::kLogToTemporaryFile`。
pub const K_LOG_TO_TEMPORARY_FILE: &str = "+";
/// 对应 `LogFile::kLogToConsole`。
pub const K_LOG_TO_CONSOLE: &str = "-";

/// 对应 `enum class LogSeparator { kSeparator }`。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogSeparator {
  /// 对应 `LogSeparator::kSeparator`(写一个原始 `,` 开新一列)。
  Separator,
}

/// 对应 `V8FileLogger::kNext`。
pub const K_NEXT: LogSeparator = LogSeparator::Separator;

/// `WriteLogHeader` 所需的版本信息。
/// 【待回收】v8-utils `Version::Get*` 与 v8-base OS 字符串常量。
#[derive(Debug, Clone, Copy)]
pub struct LogVersionInfo {
  pub major: i32,
  pub minor: i32,
  pub build: i32,
  pub patch: i32,
  pub embedder: &'static str,
  pub is_candidate: bool,
  /// 对应 `V8_OS_STRING`(如 "mac")。
  pub os_string: &'static str,
  /// 对应 `V8_TARGET_OS_STRING`(如 "macos")。
  pub target_os_string: &'static str,
}

/// 日志输出目标(对应 C++ `FILE*`/`OFStream`)。
pub enum OutputHandle {
  Stdout,
  File(File),
  TempFile(File),
}

impl OutputHandle {
  fn write_bytes(&mut self, bytes: &[u8]) -> io::Result<()> {
    match self {
      OutputHandle::Stdout => {
        let stdout = io::stdout();
        let mut lock = stdout.lock();
        lock.write_all(bytes)?;
        lock.flush()
      }
      OutputHandle::File(f) | OutputHandle::TempFile(f) => {
        f.write_all(bytes)?;
        f.flush()
      }
    }
  }
}

/// 对应 `LogFile::IsLoggingToConsole`。
#[inline]
pub fn is_logging_to_console(file_name: &str) -> bool {
  file_name == K_LOG_TO_CONSOLE
}

/// 对应 `LogFile::IsLoggingToTemporaryFile`。
#[inline]
pub fn is_logging_to_temporary_file(file_name: &str) -> bool {
  file_name == K_LOG_TO_TEMPORARY_FILE
}

/// 对应 `base::OS::OpenTemporaryFile()`(posix 实现即 `tmpfile()`):
/// O_CREAT|O_EXCL 语义循环创建, **创建成功后立即 unlink**(POSIX 下已打开的
/// fd 保持有效),复刻 tmpfile 匿名语义 —— 关闭句柄即自动回收,不留磁盘残留
/// (log-file.h:53-55: "temporary files are automatically deleted on close")。
pub fn open_temporary_file() -> Result<File> {
  let dir = temp_dir();
  for i in 0..10_000_u32 {
    let path = dir.join(format!("v8_log_test_env_var_{i}"));
    let attempt = OpenOptions::new()
      .create_new(true)
      .write(true)
      .read(true)
      .open(&path);
    match attempt {
      Ok(file) => {
        // 匿名化:unlink 后 fd 仍可读写(tmpfile 等价);失败不视为致命
        // (Windows 语义不同, 此处以 posix 主线为准)。
        let _ = remove_file(&path);
        return Ok(file);
      }
      Err(err) if err.kind() == ErrorKind::AlreadyExists => continue,
      Err(err) => return Err(err.into()),
    }
  }
  Err(Error::OutputUnavailable(
    "temporary file name space exhausted".to_owned(),
  ))
}

/// 对应 `LogFile::CreateOutputHandle`。
///
/// 打开失败**不视为致命错误**(1:1 对齐 log-file.cc:32-43): 原版
/// `base::OS::FOpen`/`OpenTemporaryFile` 失败返回 `nullptr` →
/// `output_handle_ == nullptr` → `os_` 降级 stdout(log-file.cc:59),
/// 且不写 header(log-file.cc:61);此处以 `Ok(None)` 承载同一状态,
/// 输出落 stdout 的行为见 [`LogFile::flush_line_buffer`]。
pub fn create_output_handle(file_name: &str) -> Result<Option<OutputHandle>> {
  // If we're logging anything, we need to open the log file.
  if !flags_lite::log() {
    Ok(None)
  } else if is_logging_to_console(file_name) {
    Ok(Some(OutputHandle::Stdout))
  } else if is_logging_to_temporary_file(file_name) {
    Ok(open_temporary_file().ok().map(OutputHandle::TempFile))
  } else {
    // C++ `base::OS::FOpen(file_name, LogFileOpenMode)`;Rust 等价为
    // 创建/截断读写文件,失败同态降级(见函数级注释)。
    let file = OpenOptions::new()
      .create(true)
      .read(true)
      .write(true)
      .truncate(true)
      .open(file_name);
    Ok(file.ok().map(OutputHandle::File))
  }
}

/// 对应 `class LogFile`(去掉 `V8FileLogger*` 依赖,`is_logging` 由调用方注入)。
pub struct LogFile {
  file_name: String,
  output_handle: Option<OutputHandle>,
  /// 对应 `format_buffer_`:kMessageBufferSize 单例格式化缓冲。
  format_buffer: Vec<u8>,
  /// 行缓冲:对应 C++ `OFStream`(FILE*)的 stdio 缓冲语义 —— 逐段追加在内存,
  /// `WriteToLogFile`(std::endl)行尾一次性落盘;超长行按块刷出防内存无界。
  line_buffer: Vec<u8>,
}

impl LogFile {
  /// 对应 C++ 构造函数:创建输出句柄;成功时写日志头。
  pub fn create(file_name: String, version: &LogVersionInfo) -> Result<Self> {
    let output_handle = create_output_handle(&file_name)?;
    let mut log_file = Self {
      file_name,
      output_handle,
      format_buffer: vec![0; K_MESSAGE_BUFFER_SIZE],
      line_buffer: Vec::new(),
    };
    if log_file.output_handle.is_some() {
      log_file.write_log_header(version);
    }
    Ok(log_file)
  }

  /// 对应 `LogFile::NewMessageBuilder`:未开启日志立即返回 `None`。
  /// C++ 中双重检查 `is_logging()`(无锁快速检查 + 加锁复查,避免后台线程
  /// 在日志关闭后继续写入);Rust 由 `&mut` 借用排他保证同一时刻仅一个 builder。
  pub fn new_message_builder(&mut self, is_logging: bool) -> Option<MessageBuilder<'_>> {
    if !is_logging {
      return None;
    }
    Some(MessageBuilder { log: self })
  }

  /// 对应 `LogFile::Close`:刷净行缓冲(对应 C++ `fflush`)后返回并脱离输出句柄
  /// (测试用临时文件场景)。
  pub fn close(&mut self) -> Option<OutputHandle> {
    self.flush_line_buffer();
    self.output_handle.take()
  }

  /// 对应 `LogFile::file_name`。
  #[inline]
  pub fn file_name(&self) -> &str {
    &self.file_name
  }

  /// 对应 `LogFile::WriteLogHeader`(两行:`v8-version,…` 与 `v8-platform,…`)。
  pub fn write_log_header(&mut self, version: &LogVersionInfo) {
    let mut msg = MessageBuilder { log: self };
    msg.append_cstr("v8-version");
    msg.append_separator();
    msg.append_raw_int(version.major as i64);
    msg.append_separator();
    msg.append_raw_int(version.minor as i64);
    msg.append_separator();
    msg.append_raw_int(version.build as i64);
    msg.append_separator();
    msg.append_raw_int(version.patch as i64);
    if !version.embedder.is_empty() {
      msg.append_separator();
      msg.append_cstr(version.embedder);
    }
    msg.append_separator();
    msg.append_raw_bool(version.is_candidate);
    msg.write_to_log_file();
    msg.append_cstr("v8-platform");
    msg.append_separator();
    msg.append_cstr(version.os_string);
    msg.append_separator();
    msg.append_cstr(version.target_os_string);
    msg.write_to_log_file();
  }

  /// 借用内部缓冲做整数格式化(对应 `FormatStringIntoBuffer`);
  /// 返回切片生命周期仅限本次调用(调用方立即转义/原始输出)。
  fn format_into_buffer<F>(&mut self, f: F) -> usize
  where
    F: FnOnce(&mut Vec<u8>),
  {
    self.format_buffer.clear();
    f(&mut self.format_buffer);
    // |length| is -1 if output was truncated.(截断时钳到缓冲区大小)
    if self.format_buffer.len() > K_MESSAGE_BUFFER_SIZE {
      self.format_buffer.truncate(K_MESSAGE_BUFFER_SIZE);
    }
    self.format_buffer.len()
  }

  /// 落盘行缓冲(对应 stdio 缓冲刷新);写失败仅 debug(禁 panic)。
  ///
  /// 输出目标语义对应 C++ 成员 `os_`(log-file.cc:59):
  /// 文件句柄直写文件;`Some(Stdout)`(logfile="-")与句柄缺失
  /// (`!v8_flags.log` / 打开失败)统一落 **stdout** —— 后者即原版
  /// 打开失败的降级路径,`is_logging` 由调用方决定是否继续产出消息。
  fn flush_line_buffer(&mut self) {
    if self.line_buffer.is_empty() {
      return;
    }
    let bytes = take(&mut self.line_buffer);
    match self.output_handle.as_mut() {
      Some(out @ OutputHandle::File(_)) | Some(out @ OutputHandle::TempFile(_)) => {
        // 日志写失败对引擎语义无影响;记录 debug,禁 panic。
        let _ = out.write_bytes(&bytes);
      }
      Some(OutputHandle::Stdout) | None => {
        let stdout = io::stdout();
        let mut lock = stdout.lock();
        let _ = lock.write_all(&bytes).and_then(|_| lock.flush());
      }
    }
    self.line_buffer = bytes;
    self.line_buffer.clear();
  }

  fn write_raw_bytes(&mut self, bytes: &[u8]) {
    // 不以 output_handle.is_some() 门控: C++ `os_` 在句柄缺失(log 未开由
    // is_logging 拦截)与打开失败(降级 stdout, log-file.cc:59)时均绑定 stdout,
    // 消息照写;分派由 flush_line_buffer 完成。
    self.line_buffer.extend_from_slice(bytes);
    // 超长行(如反汇编)按块刷出,防内存无界(C++ stdio 同样按缓冲块落盘)。
    if self.line_buffer.len() >= K_MESSAGE_BUFFER_SIZE * 8 {
      self.flush_line_buffer();
    }
  }
}

impl Drop for LogFile {
  fn drop(&mut self) {
    // 兜底刷净(对应 C++ fflush;Close() 已刷则此处为 no-op)。
    self.flush_line_buffer();
  }
}

/// 对应 `LogFile::MessageBuilder`:转义后追加内容。
///
/// 转义语义(1:1 对齐 C++):
/// - `,` → `\x2C`;`\` → `\\`;`\n` → `\n`(两字符转义);
/// - 其余不可打印字节 → `\xNN`(小写两位十六进制);可打印 ASCII 直通;
/// - 双字节字符(高字节非 0)→ `\uNNNN`(c1 低字节在前,c2 高字节在后);
/// - 堆字符串 >0xFF 字符 → `\u%04x`。
pub struct MessageBuilder<'a> {
  log: &'a mut LogFile,
}

impl<'a> MessageBuilder<'a> {
  /// 对应 `AppendRawCharacter`。
  pub fn append_raw_char(&mut self, c: u8) {
    self.log.write_raw_bytes(&[c]);
  }

  /// 对应 `AppendRawString`。
  pub fn append_raw_str(&mut self, s: &str) {
    self.log.write_raw_bytes(s.as_bytes());
  }

  /// 对应 `AppendCharacter`:先判断 C 语言 `isprint`(0x20..=0x7E),
  /// 再分流到逗号/反斜杠/换行/十六进制转义。
  pub fn append_char(&mut self, c: u8) {
    if c.is_ascii_graphic() || c == b' ' {
      if c == b',' {
        // Escape commas to avoid adding column separators.
        self.append_raw_str("\\x2C");
      } else if c == b'\\' {
        self.append_raw_str("\\\\");
      } else {
        // Safe, printable ascii character.
        self.append_raw_char(c);
      }
    } else if c == b'\n' {
      // Escape newlines to avoid adding row separators.
      self.append_raw_str("\\n");
    } else {
      // Escape non-printable characters. 对应 `AppendRawFormatString("\\x%02x")`。
      let len = self.log.format_into_buffer(|buf| {
        buf.extend_from_slice(b"\\x");
        write_hex_into(buf, c as u64, 2);
      });
      let bytes: Vec<u8> = self.log.format_buffer[..len].to_vec();
      for b in bytes {
        self.append_raw_char(b);
      }
    }
  }

  /// 对应 `AppendTwoByteCharacter`:c2==0 时按单字节转义,
  /// 否则输出 `\u` + c1 + c2(各 2 位十六进制,对应 `"\\u%02x%02x"`)。
  pub fn append_two_byte_character(&mut self, c1: u8, c2: u8) {
    if c2 == 0 {
      self.append_char(c1);
    } else {
      // Escape non-printable characters.
      let len = self.log.format_into_buffer(|buf| {
        buf.extend_from_slice(b"\\u");
        write_hex_into(buf, c1 as u64, 2);
        write_hex_into(buf, c2 as u64, 2);
      });
      let bytes: Vec<u8> = self.log.format_buffer[..len].to_vec();
      for b in bytes {
        self.append_raw_char(b);
      }
    }
  }

  /// 对应 `AppendString(const char*)`(逐字符转义)。
  pub fn append_cstr(&mut self, s: &str) {
    for &b in s.as_bytes() {
      self.append_char(b);
    }
  }

  /// 对应 `AppendString(base::Vector<const char> str)`:逐字节转义。
  pub fn append_string_vec(&mut self, s: &[u8]) {
    for &b in s {
      self.append_char(b);
    }
  }

  /// 对应 `AppendString(const char* str, size_t length, bool is_one_byte)`。
  pub fn append_string_len(&mut self, s: &[u8], is_one_byte: bool) {
    if is_one_byte {
      for &b in s {
        self.append_char(b);
      }
    } else {
      debug_assert!(s.len().is_multiple_of(2));
      let mut i = 0;
      while i + 1 < s.len() {
        self.append_two_byte_character(s[i], s[i + 1]);
        i += 2;
      }
    }
  }

  /// 对应 `AppendString(Tagged<String>)` 的转义循环:
  /// `c <= 0xFF` 走 `AppendCharacter`,否则 `\u%04x`。
  pub fn append_string_utf16(&mut self, units: &[u16], length_limit: Option<usize>) {
    let length = length_limit.map_or(units.len(), |l| l.min(units.len()));
    for &c in units.iter().take(length) {
      if c <= 0xFF {
        self.append_char(c as u8);
      } else {
        // Escape non-ascii characters.
        let len = self.log.format_into_buffer(|buf| {
          buf.extend_from_slice(b"\\u");
          write_hex_into(buf, c as u64, 4);
        });
        let bytes: Vec<u8> = self.log.format_buffer[..len].to_vec();
        for b in bytes {
          self.append_raw_char(b);
        }
      }
    }
  }

  /// 对应 `AppendFormatString("%" PRIdPTR, value)`(格式化后逐字符转义;
  /// 十进制输出不含需转义字符,转义后与原始输出一致)。
  pub fn append_format_string_intptr(&mut self, value: i64) {
    let len = self.log.format_into_buffer(|buf| {
      buf.extend_from_slice(&itoa_bytes(value));
    });
    let bytes: Vec<u8> = self.log.format_buffer[..len].to_vec();
    for b in bytes {
      self.append_char(b);
    }
  }

  /// 对应 `operator<<(void*)`:Windows 下 "0x" 前缀不一致,故手工格式化。
  pub fn append_pointer(&mut self, pointer: u64) {
    // os << "0x" << std::hex << reinterpret_cast<intptr_t>(pointer) << std::dec;
    self.append_raw_str("0x");
    self
      .log
      .format_into_buffer(|buf| write_hex_into(buf, pointer, 0));
    let bytes = &self.log.format_buffer;
    let len = bytes.len().min(K_MESSAGE_BUFFER_SIZE);
    let owned = bytes[..len].to_vec();
    for b in owned {
      self.append_raw_char(b);
    }
  }

  /// 对应 `operator<<(LogSeparator)`:跳过转义,直接开新列。
  pub fn append_separator(&mut self) {
    self.append_raw_char(b',');
  }

  /// 对应通用 `operator<<(T)` 经 `os_` 直通(i64 十进制)。
  pub fn append_raw_int(&mut self, value: i64) {
    self.append_raw_str(&value.to_string());
  }

  /// 对应通用 `operator<<(T)` 经 `os_` 直通(u64 十进制)。
  pub fn append_raw_uint(&mut self, value: u64) {
    self.append_raw_str(&value.to_string());
  }

  /// 对应通用 `operator<<(bool)` 经 `os_` 直通(std::ostream 默认 `1`/`0`)。
  pub fn append_raw_bool(&mut self, value: bool) {
    self.append_raw_str(if value { "1" } else { "0" });
  }

  /// 对应 `WriteToLogFile()`:行尾 `std::endl`(换行 + flush)。
  pub fn write_to_log_file(&mut self) {
    self.log.line_buffer.push(b'\n');
    self.log.flush_line_buffer();
  }
}

/// 小端无填充十六进制输出(std::ostream `std::hex` 默认小写、无 `0x`)。
/// 值为 0 时也至少输出 1 位,并按 `min_width` 补零 —— 对应 printf `%02x`
/// 的 0 → "00"(log-file.cc:170 `"\\u%02x%02x"` 低字节为 0 的场景)。
fn write_hex_into(buf: &mut Vec<u8>, value: u64, min_width: usize) {
  const DIGITS: &[u8; 16] = b"0123456789abcdef";
  let start = buf.len();
  let mut v = value;
  while v > 0 {
    buf.push(DIGITS[(v & 0xF) as usize]);
    v >>= 4;
  }
  buf[start..].reverse();
  while buf.len() - start < min_width.max(1) {
    buf.insert(start, b'0');
  }
}

/// `%x`(uint32_t 场景,如 symbol hash / append_hex)。
pub(crate) fn format_hex_u32(value: u32) -> String {
  let mut buf = Vec::new();
  write_hex_into(&mut buf, value as u64, 0);
  String::from_utf8_lossy(&buf).into_owned()
}

fn itoa_bytes(value: i64) -> Vec<u8> {
  value.to_string().into_bytes()
}
