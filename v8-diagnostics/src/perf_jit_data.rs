// 由 tool/src/diagnostics/perf-jit.js 从 v8/src/diagnostics/perf-jit.h;src/diagnostics/perf-jit.cc 生成; 修订就地改, 复跑覆盖

//! perf-jit (jit-<pid>.dump) 记录纯定义面 (基线生成; 序列化流程见 perf_jit.rs)。
//! Isolate/文件 IO 面推迟, 时间戳/PID/TID 由调用方注入。

pub const PERF_JIT_HEADER_MAGIC: u32 = 0x4A695444;
pub const PERF_JIT_HEADER_VERSION: u32 = 1;
/// 上游 `PerfJitHeader` (8 字段, repr(C) 与 C++ POD 布局一致)
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PerfJitHeader {
  pub magic: u32,
  pub version: u32,
  pub size: u32,
  pub elf_mach_target: u32,
  pub reserved: u32,
  pub process_id: u32,
  pub time_stamp: u64,
  pub flags: u64,
}

/// 上游 `PerfJitBase` (3 字段, repr(C) 与 C++ POD 布局一致)
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PerfJitBase {
  pub event: u32,
  pub size: u32,
  pub time_stamp: u64,
}

/// 上游 `PerfJitCodeLoad : PerfJitBase` (6 字段, repr(C) 与 C++ POD 布局一致)
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PerfJitCodeLoad {
  pub base: PerfJitBase,
  pub process_id: u32,
  pub thread_id: u32,
  pub vma: u64,
  pub code_address: u64,
  pub code_size: u64,
  pub code_id: u64,
}

/// 上游 `PerfJitDebugEntry` (3 字段, repr(C) 与 C++ POD 布局一致)
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PerfJitDebugEntry {
  pub address: u64,
  pub line_number: i32,
  pub column: i32,
}

/// 上游 `PerfJitCodeDebugInfo : PerfJitBase` (2 字段, repr(C) 与 C++ POD 布局一致)
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PerfJitCodeDebugInfo {
  pub base: PerfJitBase,
  pub address: u64,
  pub entry_count: u64,
}

/// 上游 `PerfJitCodeUnwindingInfo : PerfJitBase` (3 字段, repr(C) 与 C++ POD 布局一致)
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PerfJitCodeUnwindingInfo {
  pub base: PerfJitBase,
  pub unwinding_size: u64,
  pub eh_frame_hdr_size: u64,
  pub mapped_size: u64,
}

// ── PerfJitEvent (5 项) ──
pub const EVENT_LOAD: u32 = 0;
pub const EVENT_MOVE: u32 = 1;
pub const EVENT_DEBUGINFO: u32 = 2;
pub const EVENT_CLOSE: u32 = 3;
pub const EVENT_UNWINDINGINFO: u32 = 4;

// ── ELF machine 常量 (9 项) ──
pub const ELF_MACH_IA32: u32 = 3;
pub const ELF_MACH_X64: u32 = 62;
pub const ELF_MACH_ARM: u32 = 40;
pub const ELF_MACH_MIPS64: u32 = 8;
pub const ELF_MACH_LOONG64: u32 = 258;
pub const ELF_MACH_ARM64: u32 = 183;
pub const ELF_MACH_S390X: u32 = 22;
pub const ELF_MACH_PPC64: u32 = 21;
pub const ELF_MACH_RISCV: u32 = 243;

// ── ELF 头尺寸 / 文件与缓冲常量 ──
pub const ELF_HEADER_SIZE_32: usize = 52;
pub const ELF_HEADER_SIZE_64: usize = 64;
pub const HEADER_RESERVED_FILL: u32 = 0xDEADBEEF;
pub const FILENAME_FORMAT: &str = "%s/jit-%d.dump";
pub const FILENAME_BUFFER_PADDING: usize = 16;
pub const LOG_BUFFER_SIZE: usize = 2 * 1024 * 1024;
pub const UNKNOWN_SCRIPT_NAME: &str = "<unknown>";
pub const RECORD_PAD: usize = 8;
