//! perf-jit (jit-<pid>.dump) 记录序列化行为面 (对应 perf-jit.cc 的 LogWrite* 纯逻辑段)。
//!
//! 时间戳/PID/TID/ELF machine 由调用方注入 (Isolate 耦合面推迟);
//! 记录布局由 perf_jit_data.rs 基线生成, 此处按字段小端序序列化, 与 C++ POD 内存布局逐字节一致。

use crate::{
  eh_frame::{EhFrameWriter, round_up},
  eh_frame_data::EH_FRAME_HDR_SIZE,
  perf_jit_data::{
    ELF_HEADER_SIZE_64, EVENT_DEBUGINFO, EVENT_LOAD, EVENT_UNWINDINGINFO, HEADER_RESERVED_FILL,
    PERF_JIT_HEADER_MAGIC, PERF_JIT_HEADER_VERSION, PerfJitCodeDebugInfo, PerfJitCodeLoad,
    PerfJitCodeUnwindingInfo, PerfJitDebugEntry, PerfJitHeader, RECORD_PAD,
  },
};

const STRING_TERMINATOR: u8 = 0;

fn write_u32(out: &mut Vec<u8>, v: u32) {
  out.extend_from_slice(&v.to_le_bytes());
}

fn write_u64(out: &mut Vec<u8>, v: u64) {
  out.extend_from_slice(&v.to_le_bytes());
}

fn write_base(out: &mut Vec<u8>, event: u32, size: u32, time_stamp: u64) {
  write_u32(out, event);
  write_u32(out, size);
  write_u64(out, time_stamp);
}

/// PerfJitHeader → 40 字节 (mirrors LogWriteHeader); 时间戳单位微秒
/// (上游 perf-jit.cc:555-557 CurrentClockTimeMillisecondsHighResolution() × kMicrosecondsPerMillisecond;
///  注意与记录条目的 GetTimestamp() 纳秒 (cc:220-222) 不同源)
pub fn header_bytes(elf_mach_target: u32, process_id: u32, time_stamp_us: u64) -> Vec<u8> {
  let mut out = Vec::with_capacity(size_of::<PerfJitHeader>());
  write_u32(&mut out, PERF_JIT_HEADER_MAGIC);
  write_u32(&mut out, PERF_JIT_HEADER_VERSION);
  write_u32(&mut out, size_of::<PerfJitHeader>() as u32);
  write_u32(&mut out, elf_mach_target);
  write_u32(&mut out, HEADER_RESERVED_FILL);
  write_u32(&mut out, process_id);
  write_u64(&mut out, time_stamp_us);
  write_u64(&mut out, 0); // flags_ = 0 (u64, 对齐 PerfJitHeader::flags)
  out
}

/// PerfJitCodeLoad 记录入参组 (对应上游 LogWriteJitCodeLoadEntry 形参列)
pub struct CodeLoadArgs<'a> {
  pub time_stamp: u64,
  pub process_id: u32,
  pub thread_id: u32,
  pub vma: u64,
  pub code_address: u64,
  pub code_size: u64,
  pub code_id: u64,
  pub name: &'a [u8],
  pub code: &'a [u8],
}

/// PerfJitCodeLoad 记录: 56 字节头 + name + '\0' + 代码字节 (mirrors WriteJitCodeLoadEntry)
pub fn code_load_bytes(a: &CodeLoadArgs<'_>) -> Vec<u8> {
  let size = size_of::<PerfJitCodeLoad>() + a.name.len() + 1 + a.code.len();
  let mut out = Vec::with_capacity(size);
  write_base(&mut out, EVENT_LOAD, size as u32, a.time_stamp);
  write_u32(&mut out, a.process_id);
  write_u32(&mut out, a.thread_id);
  write_u64(&mut out, a.vma);
  write_u64(&mut out, a.code_address);
  write_u64(&mut out, a.code_size);
  write_u64(&mut out, a.code_id);
  out.extend_from_slice(a.name);
  out.push(STRING_TERMINATOR);
  out.extend_from_slice(a.code);
  out
}

/// 源码位置条目: (调整后的地址, 行, 列, 脚本名)
pub struct SourceLine<'a> {
  pub address: u64,
  pub line: i32,
  pub column: i32,
  pub name: &'a [u8],
}

/// PerfJitCodeDebugInfo 记录 (mirrors LogWriteDebugInfo); 空条目返回 None (上游 entry_count==0 直接返回)
pub fn debug_info_bytes(
  time_stamp: u64,
  address: u64,
  entries: &[SourceLine<'_>],
) -> Option<Vec<u8>> {
  if entries.is_empty() {
    return None;
  }
  let head = size_of::<PerfJitCodeDebugInfo>();
  let names: usize = entries
    .iter()
    .map(|e| size_of::<PerfJitDebugEntry>() + e.name.len() + 1)
    .sum();
  let size = head + names;
  let padding = round_up(size, RECORD_PAD) - size;

  let mut out = Vec::with_capacity(size + padding);
  write_base(
    &mut out,
    EVENT_DEBUGINFO,
    (size + padding) as u32,
    time_stamp,
  );
  write_u64(&mut out, address);
  write_u64(&mut out, entries.len() as u64);
  for e in entries {
    write_u64(&mut out, e.address);
    write_u32(&mut out, e.line as u32);
    write_u32(&mut out, e.column as u32);
    out.extend_from_slice(e.name);
    out.push(STRING_TERMINATOR);
  }
  out.resize(out.len() + padding, 0);
  Some(out)
}

/// PerfJitCodeUnwindingInfo 记录 (mirrors LogWriteUnwindingInfo)。
/// `unwinding` 为 None 时落空 .eh_frame_hdr (上游无 unwinding_info 分支)。
pub fn unwinding_bytes(time_stamp: u64, unwinding: Option<&[u8]>) -> Vec<u8> {
  let (unwinding_size, mapped_size) = match unwinding {
    Some(data) => (data.len() as u64, data.len() as u64),
    None => (EH_FRAME_HDR_SIZE as u64, 0),
  };
  let content = size_of::<PerfJitCodeUnwindingInfo>() + unwinding_size as usize;
  let padding = round_up(content, RECORD_PAD) - content;

  let mut out = Vec::with_capacity(content + padding);
  write_base(
    &mut out,
    EVENT_UNWINDINGINFO,
    (content + padding) as u32,
    time_stamp,
  );
  write_u64(&mut out, unwinding_size);
  write_u64(&mut out, EH_FRAME_HDR_SIZE as u64);
  write_u64(&mut out, mapped_size);
  match unwinding {
    Some(data) => out.extend_from_slice(data),
    None => EhFrameWriter::write_empty_eh_frame(&mut out),
  }
  out.resize(out.len() + padding, 0);
  out
}

/// perf inject 把函数入口放到 ELF 头之后: 调试条目地址基准 = 代码起始 + kElfHeaderSize(64)
pub const fn debug_address_base(code_start: u64) -> u64 {
  code_start + ELF_HEADER_SIZE_64 as u64
}
