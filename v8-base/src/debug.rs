//! 原版 v8/src/base/debug/{stack_trace.h/cc, stack_trace_posix.cc} — 栈回溯
//!
//! 原版清单 (v8/src/base/debug/):
//! - `stack_trace.h`   — 抽象 API (StackTrace 类 + EnableInProcessStackDumping)
//! - `stack_trace.cc`  — 跨平台部分 (拷贝构造、Addresses、ToString)
//! - `stack_trace_posix.cc` — POSIX 实现: `backtrace()` + `backtrace_symbols()`, 信号安装
//! - `stack_trace_win.cc`   — Windows 实现, 本移植不支持
//! - `stack_trace_android.cc/fuchsia.cc/zos.cc` — 平台专属实现, 本移植不支持
//!
//! 落地范围:
//! - 完整承接 `stack_trace.cc` 的可移植部分 (结构 + 从地址数组构造 + `addresses()`)
//! - POSIX 平台 (`linux`/`android`/`macos`) 使用 libc `backtrace`/`backtrace_symbols` 落地
//!   `capture`/`output_to_stream`/`print`/`to_string`
//! - **推迟**: `EnableInProcessStackDumping` / `DisableSignalStackDump` — 依赖 sigaction +
//!   sigaltstack + 崩溃处理器, 应归上层 v8-base-platform/v8-diagnostics, 见 readme/zh.md
//!   "系统杂项族"节.

use core::ffi::c_void;
#[cfg(any(target_os = "linux", target_os = "android", target_os = "macos"))]
use std::ffi::{CStr, c_int};
use std::{io, io::Write as _, ptr::null};

/// 原版 `StackTrace::kMaxTraces = 62`.
pub const MAX_TRACES: usize = 62;

/// 原版 `class v8::base::debug::StackTrace`.
///
/// C++ 侧持有 `void* trace_[kMaxTraces]` 与 `count_`, Rust 保持同一布局: 帧地址用
/// `*const c_void` 数组 (拷贝语义与原版一致, 无 Drop 副作用).
#[derive(Clone, Copy)]
pub struct StackTrace {
  trace: [*const c_void; MAX_TRACES],
  count: usize,
}

impl StackTrace {
  /// 空栈 (对应 count_=0 的默认构造在 capture 前的中间态).
  #[must_use]
  pub const fn empty() -> Self {
    Self {
      trace: [null(); MAX_TRACES],
      count: 0,
    }
  }

  /// 原版 `StackTrace(const void* const* trace, size_t count)` — 拷贝并截断到 MAX_TRACES.
  #[must_use]
  pub fn from_addresses(trace: &[*const c_void]) -> Self {
    let count = trace.len().min(MAX_TRACES);
    let mut buf = [null(); MAX_TRACES];
    buf[..count].copy_from_slice(&trace[..count]);
    Self { trace: buf, count }
  }

  /// 原版 `Addresses(size_t* count)` — 返回帧切片; 空则返回空切片 (原版返回 nullptr).
  #[must_use]
  pub fn addresses(&self) -> &[*const c_void] {
    &self.trace[..self.count]
  }

  /// 原版默认构造 `StackTrace()` — 调用平台 `backtrace()` 抓取当前栈.
  ///
  /// 未支持平台返回空栈 (count=0), 与原版 `count_=0` 兜底路径一致.
  #[must_use]
  pub fn capture() -> Self {
    #[cfg(any(target_os = "linux", target_os = "android", target_os = "macos"))]
    {
      let mut this = Self::empty();
      // SAFETY: 缓冲区为本栈数组, size 匹配; backtrace 契约只写入 [0, return).
      let n = unsafe {
        libc::backtrace(
          this.trace.as_mut_ptr() as *mut *mut c_void,
          MAX_TRACES as c_int,
        )
      };
      if n > 0 {
        this.count = n as usize;
      }
      this
    }
    #[cfg(not(any(target_os = "linux", target_os = "android", target_os = "macos")))]
    {
      Self::empty()
    }
  }

  /// 原版 `OutputToStream(std::ostream*)` — POSIX 分支: 用 `backtrace_symbols` 解析帧.
  ///
  /// Rust 侧写入任意实现 `std::io::Write` 的 sink; 失败时逐帧输出十六进制地址 (原版
  /// `syslog`/无符号化路径的退化形式).
  #[cfg(any(target_os = "linux", target_os = "android", target_os = "macos"))]
  pub fn output_to_stream<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
    if self.count == 0 {
      return w.write_all(b"Stack trace (empty)\n");
    }
    // SAFETY: backtrace_symbols 只读取 [0, count) 帧地址数组, 返回的字符指针数组
    // 由 libc 分配, 我们随后 backtrace_symbols_free 归还, 生命周期在本函数体内.
    unsafe {
      let symbols = libc::backtrace_symbols(
        self.trace.as_ptr() as *const *mut c_void,
        self.count as c_int,
      );
      if symbols.is_null() {
        // 无符号化能力: 逐帧输出地址, 与原版兜底一致.
        for frame in self.addresses() {
          writeln!(w, "  {frame:p}")?;
        }
        return Ok(());
      }
      for i in 0..self.count {
        let sym_ptr = *symbols.add(i);
        if sym_ptr.is_null() {
          writeln!(w, "  <unknown>")?;
          continue;
        }
        // SAFETY: libc 保证返回 NUL 结尾 C 字符串数组元素.
        let cstr = CStr::from_ptr(sym_ptr);
        let bytes = cstr.to_bytes();
        w.write_all(bytes)?;
        w.write_all(b"\n")?;
      }
      // backtrace_symbols 使用 malloc, 用 libc::free 逐元素 + 数组释放 (libc crate 未导出
      // backtrace_symbols_free).
      for i in 0..self.count {
        let p = *symbols.add(i);
        if !p.is_null() {
          libc::free(p as *mut c_void);
        }
      }
      libc::free(symbols as *mut c_void);
    }
    Ok(())
  }

  /// 未支持平台: 逐帧输出地址兜底, 保持函数签名一致.
  #[cfg(not(any(target_os = "linux", target_os = "android", target_os = "macos")))]
  pub fn output_to_stream<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
    for frame in self.addresses() {
      writeln!(w, "  {frame:p}")?;
    }
    Ok(())
  }

  /// 原版 `Print() const` — 写 stderr.
  pub fn print(&self) {
    let stderr = io::stderr();
    let mut lock = stderr.lock();
    let _ = self.output_to_stream(&mut lock);
    let _ = lock.flush();
  }

  /// 原版 `ToString() const` — 汇总为 String (非 Display 承接, 避免与 std
  /// inherent to_string 二义)
  #[must_use]
  pub fn to_string_lossy(&self) -> String {
    let mut buf: Vec<u8> = Vec::new();
    // 内存写入不会失败
    let _ = self.output_to_stream(&mut buf);
    String::from_utf8_lossy(&buf).into_owned()
  }

  /// 有效帧数.
  #[must_use]
  pub const fn count(&self) -> usize {
    self.count
  }
}

impl Default for StackTrace {
  /// 原版默认构造 = `StackTrace::StackTrace()` (自动 capture).
  fn default() -> Self {
    Self::capture()
  }
}

/// 原版 `EnableInProcessStackDumping()` / `DisableSignalStackDump()`.
///
/// **推迟登记**: 依赖 sigaction + sigaltstack + SIGSEGV/SIGBUS/SIGFPE/SIGABRT 崩溃处理器,
/// 与 `v8-base-platform` 的信号设施耦合; 待 platform 层落地后由 `v8-diagnostics` 桥接.
/// 此处仅导出稳定 no-op 保持调用点符号.
pub fn enable_in_process_stack_dumping() -> bool {
  false
}

/// 见 [`enable_in_process_stack_dumping`].
pub fn disable_signal_stack_dumping() {}
