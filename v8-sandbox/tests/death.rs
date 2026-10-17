// 释放段不可访问死亡断言: 对应 segmented-table-unittest.cc 的 DeathTest 族
// (WriteToFreedSegmentCrashes / ReadFromFreedSegmentCrashes)
// Rust 侧以子进程崩溃信号断言 (PROT_NONE 访问 → SIGSEGV/SIGBUS)

use std::{
  env::{current_exe, var},
  process::{Command, exit},
  ptr::{read_volatile, write_volatile},
};

use aok::{OK, Void};
use v8_sandbox::{K_SEGMENT_SIZE, SegmentedTableBase};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

const CASE_ENV: &str = "V8SBX_DEATH_CASE";
const FOUR_SEGMENTS: usize = 4 * K_SEGMENT_SIZE;

fn write_memory(addr: usize, value: u8) {
  // SAFETY: 地址语义由各用例定义 (含预期崩溃路径)。
  unsafe { write_volatile(addr as *mut u8, value) }
}
fn read_memory(addr: usize) -> u8 {
  // SAFETY: 同上。
  unsafe { read_volatile(addr as *const u8) }
}

/// 子进程执行体: 分配段 → 写入验证 → 释放 → 访问 (预期崩溃)
fn child_write_after_free() {
  let mut table = SegmentedTableBase::default();
  let _ = table.initialize(FOUR_SEGMENTS, 0, false);
  let Ok(Some(segment)) = table.try_allocate_segment() else {
    return;
  };
  let addr = table.base() as usize + segment.offset() as usize;
  write_memory(addr, 0x42);
  assert_eq!(read_memory(addr), 0x42);
  let _ = table.free_table_segment(segment);
  // 释放段必须不可访问: 此写应崩溃
  write_memory(addr, 0x43);
}

fn child_read_after_free() {
  let mut table = SegmentedTableBase::default();
  let _ = table.initialize(FOUR_SEGMENTS, 0, false);
  let Ok(Some(segment)) = table.try_allocate_segment() else {
    return;
  };
  let addr = table.base() as usize + segment.offset() as usize + K_SEGMENT_SIZE / 2;
  write_memory(addr, 0x42);
  let _ = table.free_table_segment(segment);
  // 释放段中间字节读应崩溃
  let _ = read_memory(addr);
}

/// 父进程: 以当前测试二进制 + 用例过滤重执行子用例, 断言非零退出 (信号)
fn assert_crashes(case: &str, test_name: &str) {
  let exe = current_exe().expect("current_exe");
  let status = Command::new(exe)
    .arg(test_name)
    .arg("--exact")
    .env(CASE_ENV, case)
    .status()
    .expect("spawn child");
  assert!(
    !status.success(),
    "子用例 {case} 应崩溃退出 (释放段访问必须不可达), 实得 {status:?}"
  );
}

#[test]
fn death_write_to_freed_segment_crashes() -> Void {
  match var(CASE_ENV).as_deref() {
    Ok("write_freed") => {
      child_write_after_free();
      exit(0); // 不应到达
    }
    _ => assert_crashes("write_freed", "death_write_to_freed_segment_crashes"),
  }
  OK
}

#[test]
fn death_read_from_freed_segment_crashes() -> Void {
  match var(CASE_ENV).as_deref() {
    Ok("read_freed") => {
      child_read_after_free();
      exit(0); // 不应到达
    }
    _ => assert_crashes("read_freed", "death_read_from_freed_segment_crashes"),
  }
  OK
}
