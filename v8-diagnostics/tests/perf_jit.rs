//! perf-jit 记录序列化金标: 各记录逐字段偏移与小端字节流比对 (jit-<pid>.dump 面)。

use v8_diagnostics::{
  CodeLoadArgs, EVENT_DEBUGINFO, EVENT_LOAD, EVENT_UNWINDINGINFO, PERF_JIT_HEADER_MAGIC,
  PerfJitCodeDebugInfo, PerfJitCodeLoad, PerfJitCodeUnwindingInfo, PerfJitHeader, SourceLine,
  code_load_bytes, debug_address_base, debug_info_bytes, header_bytes, unwinding_bytes,
};

fn u32_at(b: &[u8], o: usize) -> u32 {
  u32::from_le_bytes(b[o..o + 4].try_into().unwrap())
}
fn u64_at(b: &[u8], o: usize) -> u64 {
  u64::from_le_bytes(b[o..o + 8].try_into().unwrap())
}
fn i32_at(b: &[u8], o: usize) -> i32 {
  i32::from_le_bytes(b[o..o + 4].try_into().unwrap())
}

/// PerfJitHeader → 40 字节, 偏移取自 repr(C) 结构
#[test]
fn header_record() {
  let b = header_bytes(62, 4321, 99);
  assert_eq!(b.len(), size_of::<PerfJitHeader>());
  assert_eq!(u32_at(&b, 0), PERF_JIT_HEADER_MAGIC);
  assert_eq!(u32_at(&b, 4), 1, "version");
  assert_eq!(u32_at(&b, 8), 40, "size");
  assert_eq!(u32_at(&b, 12), 62, "elf_mach_target");
  assert_eq!(u32_at(&b, 16), 0xDEAD_BEEF, "reserved");
  assert_eq!(u32_at(&b, 20), 4321, "process_id");
  assert_eq!(u64_at(&b, 24), 99, "time_stamp");
  assert_eq!(u64_at(&b, 32), 0, "flags");
}

/// CodeLoad: 56 字节头 + name + NUL + 代码, size 字段含变长体
#[test]
fn code_load_record() {
  let rec = CodeLoadArgs {
    time_stamp: 7,
    process_id: 1,
    thread_id: 2,
    vma: 0x1000,
    code_address: 0x2000,
    code_size: 2,
    code_id: 42,
    name: b"fn",
    code: &[9, 8],
  };
  let b = code_load_bytes(&rec);
  let head = size_of::<PerfJitCodeLoad>();
  assert_eq!(head, 56);
  assert_eq!(b.len(), head + 2 + 1 + 2);
  assert_eq!(u32_at(&b, 0), EVENT_LOAD);
  assert_eq!(u32_at(&b, 4), b.len() as u32, "size = 记录全长");
  assert_eq!(u64_at(&b, 8), 7, "time_stamp");
  assert_eq!(u32_at(&b, 16), 1);
  assert_eq!(u32_at(&b, 20), 2);
  assert_eq!(u64_at(&b, 24), 0x1000, "vma");
  assert_eq!(u64_at(&b, 32), 0x2000, "code_address");
  assert_eq!(u64_at(&b, 40), 2, "code_size");
  assert_eq!(u64_at(&b, 48), 42, "code_id");
  assert_eq!(&b[56..59], b"fn\0");
  assert_eq!(&b[59..], &[9, 8]);
}

/// DebugInfo: 空条目 None (上游 entry_count==0 早退); 非空按 8 字节补零
#[test]
fn debug_info_record() {
  assert_eq!(debug_info_bytes(1, 0x1000, &[]), None);

  let entries = [SourceLine {
    address: 0x1004,
    line: 3,
    column: 7,
    name: b"a.js",
  }];
  let b = debug_info_bytes(5, 0x1000, &entries).unwrap();
  let head = size_of::<PerfJitCodeDebugInfo>();
  assert_eq!(head, 32);
  let unpadded = head + 16 + 4 + 1;
  assert_eq!(b.len(), 56, "53 → 8 对齐 56");
  assert_eq!(u32_at(&b, 0), EVENT_DEBUGINFO);
  assert_eq!(u32_at(&b, 4), b.len() as u32);
  assert_eq!(u64_at(&b, 8), 5);
  assert_eq!(u64_at(&b, 16), 0x1000, "address");
  assert_eq!(u64_at(&b, 24), 1, "entry_count");
  assert_eq!(u64_at(&b, 32), 0x1004, "entry.address");
  assert_eq!(i32_at(&b, 40), 3, "line");
  assert_eq!(i32_at(&b, 44), 7, "column");
  assert_eq!(&b[48..53], b"a.js\0");
  assert_eq!(&b[53..], &[0, 0, 0], "补零");
  assert_eq!(unpadded, 53);
}

/// Unwinding: 有 CIE+FDE 数据直存; 无数据落空 .eh_frame_hdr (20 字节) hack
#[test]
fn unwinding_record() {
  let data = [0xAAu8; 12];
  let b = unwinding_bytes(9, Some(&data));
  let head = size_of::<PerfJitCodeUnwindingInfo>();
  assert_eq!(head, 40);
  assert_eq!(u32_at(&b, 0), EVENT_UNWINDINGINFO);
  assert_eq!(b.len(), 56, "40+12=52 → 8 对齐 56");
  assert_eq!(u64_at(&b, 16), 12, "unwinding_size");
  assert_eq!(u64_at(&b, 24), 20, "eh_frame_hdr_size");
  assert_eq!(u64_at(&b, 32), 12, "mapped_size");
  assert_eq!(&b[40..52], &data[..]);
  assert_eq!(&b[52..], &[0, 0, 0, 0], "记录补零");

  let b = unwinding_bytes(9, None);
  assert_eq!(b.len(), 64, "40+20 → 64");
  assert_eq!(u64_at(&b, 16), 20);
  assert_eq!(u64_at(&b, 32), 0, "mapped_size=0");
  assert_eq!(&b[40..44], &[1, 0x1B, 3, 0x3B], "空 hdr 版本/编码字节");
  assert!(b[44..60].iter().all(|&v| v == 0), "hdr 载荷零");
  assert_eq!(&b[60..], &[0, 0, 0, 0], "记录补零");
}

/// perf inject 入口基准 = 代码起始 + 64 (ELF64 头)
#[test]
fn debug_address_base_offset() {
  assert_eq!(debug_address_base(0x1000), 0x1040);
}
