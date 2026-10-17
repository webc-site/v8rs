// SegmentedTableBase 测试: 1:1 对应 v8/test/unittests/sandbox/segmented-table-unittest.cc
// (段属性 / 初始化 / 段分配释放 / 多段 / RO 段 / 解封作用域)

use std::{
  collections::BTreeSet,
  ptr::{read_volatile, write_volatile},
};

use aok::{OK, Void};
use v8_sandbox::{Error, K_SEGMENT_SIZE, SegmentBase, SegmentedTableBase};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 原 unittest TestSegmentedTable 便捷别名
const FOUR_SEGMENTS: usize = 4 * K_SEGMENT_SIZE;

fn read_memory(addr: usize) -> u8 {
  // SAFETY: 地址来自已提交段内 (测试语境保证); volatile 语义防优化。
  unsafe { read_volatile(addr as *const u8) }
}
fn write_memory(addr: usize, value: u8) {
  // SAFETY: 同上。
  unsafe { write_volatile(addr as *mut u8, value) }
}

/// SegmentBaseProperties: At/number/offset/排序
#[test]
fn segment_base_properties() -> Void {
  let seg0 = SegmentBase::at(0);
  assert_eq!(seg0.number(), 0);
  assert_eq!(seg0.offset(), 0);

  let seg1 = SegmentBase::at(K_SEGMENT_SIZE as u32);
  assert_eq!(seg1.number(), 1);
  assert_eq!(seg1.offset() as usize, K_SEGMENT_SIZE);

  let seg3 = SegmentBase::at((3 * K_SEGMENT_SIZE) as u32);
  assert_eq!(seg3.number(), 3);
  assert_eq!(seg3.offset() as usize, 3 * K_SEGMENT_SIZE);

  assert_eq!(seg0, seg0);
  assert_ne!(seg0, seg1);
  assert!(seg0 < seg1);
  assert!(!(seg1 < seg0));
  assert!(!(seg0 < seg0));
  OK
}

/// InitializationAndTearDown: 初始化 → base 非空 → teardown → 可重建
#[test]
fn initialization_and_tear_down() -> Void {
  let mut table = SegmentedTableBase::default();
  assert!(!table.is_initialized());

  table
    .initialize(FOUR_SEGMENTS, 0, false)
    .expect("sandbox io");
  assert!(table.is_initialized());
  assert!(!table.base().is_null());

  table.tear_down().expect("sandbox io");
  assert!(!table.is_initialized());

  // teardown 后可重新初始化
  table
    .initialize(FOUR_SEGMENTS, 0, false)
    .expect("sandbox io");
  assert!(table.is_initialized());
  assert!(!table.base().is_null());
  table.tear_down().expect("sandbox io");
  assert!(!table.is_initialized());
  OK
}

/// AllocateAndFreeSegment: 段内头/中/尾可写读; 释放成功
#[test]
fn allocate_and_free_segment() -> Void {
  let mut table = SegmentedTableBase::default();
  table
    .initialize(FOUR_SEGMENTS, 0, false)
    .expect("sandbox io");

  let segment = table
    .try_allocate_segment()
    .expect("sandbox io")
    .expect("4 段预留内首个分配必成功");
  assert_eq!(
    segment.offset() as usize,
    segment.number() as usize * K_SEGMENT_SIZE
  );
  assert!((segment.offset() as usize) < FOUR_SEGMENTS);

  let base_addr = table.base() as usize + segment.offset() as usize;
  write_memory(base_addr, 0xAA);
  write_memory(base_addr + K_SEGMENT_SIZE / 2, 0xBB);
  write_memory(base_addr + K_SEGMENT_SIZE - 1, 0xCC);
  assert_eq!(read_memory(base_addr), 0xAA);
  assert_eq!(read_memory(base_addr + K_SEGMENT_SIZE / 2), 0xBB);
  assert_eq!(read_memory(base_addr + K_SEGMENT_SIZE - 1), 0xCC);

  table.free_table_segment(segment).expect("sandbox io");
  OK
}

/// AllocateMultipleSegments: 段号互异; 预留耗尽返回 None;
/// 释放后重分配落在同一最低位段 (first-fit)
#[test]
fn allocate_multiple_segments() -> Void {
  let mut table = SegmentedTableBase::default();
  table
    .initialize(FOUR_SEGMENTS, 0, false)
    .expect("sandbox io");

  let mut segments = Vec::new();
  for _ in 0..4 {
    let seg = table
      .try_allocate_segment()
      .expect("sandbox io")
      .expect("4 段内分配");
    segments.push(seg);
  }
  let mut numbers = BTreeSet::new();
  for &s in &segments {
    assert!(numbers.insert(s.number()), "段号必须互异");
    write_memory(
      table.base() as usize + s.offset() as usize,
      s.number() as u8,
    );
  }
  for &s in &segments {
    assert_eq!(
      read_memory(table.base() as usize + s.offset() as usize),
      s.number() as u8
    );
  }

  // 连续大预留已满: 第 5 次分配失败
  assert!(table.try_allocate_segment().expect("sandbox io").is_none());
  // 释放段 0 后重分配: first-fit 落回同一槽位
  table.free_table_segment(segments[0]).expect("sandbox io");
  let new_seg = table
    .try_allocate_segment()
    .expect("sandbox io")
    .expect("释放后必有空位");
  assert_eq!(new_seg, segments[0]);
  for &s in &segments {
    let _ = table.free_table_segment(s);
  }
  OK
}

/// ReadOnlySegments: RO 段顺序发放/计数/复位; 常规段不与 RO 段重叠
#[test]
fn read_only_segments() -> Void {
  let mut table = SegmentedTableBase::default();
  let ro_size = 2 * K_SEGMENT_SIZE;
  table
    .initialize(6 * K_SEGMENT_SIZE, ro_size, false)
    .expect("sandbox io");

  assert_eq!(table.read_only_segments_used(), 0);
  let ro_seg0 = table.allocate_read_only_segment().expect("sandbox io");
  assert_eq!(ro_seg0.number(), 0);
  assert_eq!(ro_seg0.offset(), 0);
  assert_eq!(table.read_only_segments_used(), 1);

  let ro_seg1 = table.allocate_read_only_segment().expect("sandbox io");
  assert_eq!(ro_seg1.number(), 1);
  assert_eq!(ro_seg1.offset() as usize, K_SEGMENT_SIZE);
  assert_eq!(table.read_only_segments_used(), 2);

  assert_eq!(table.reset_read_only_segments(), 2);
  assert_eq!(table.read_only_segments_used(), 0);

  let ro_again = table.allocate_read_only_segment().expect("sandbox io");
  assert_eq!(ro_again.number(), 0);
  assert_eq!(table.read_only_segments_used(), 1);

  // 常规段分配不得与 RO 段重叠
  let normal = table
    .try_allocate_segment()
    .expect("sandbox io")
    .expect("常规段");
  assert!(normal.number() as usize >= 2);
  assert!(normal.offset() as usize >= ro_size);
  let _ = table.free_table_segment(normal);
  OK
}

/// AllocateReadOnlySegmentBeyondLimitCrashes 的错误化对应:
/// RO 配额耗尽返回 ReadOnlySegmentExhausted (原版 CHECK 崩溃 → Rust 错误)
#[test]
fn read_only_segment_exhaustion() -> Void {
  let mut table = SegmentedTableBase::default();
  table
    .initialize(4 * K_SEGMENT_SIZE, K_SEGMENT_SIZE, false)
    .expect("sandbox io");
  assert!(table.allocate_read_only_segment().is_ok());
  assert_eq!(
    table.allocate_read_only_segment().unwrap_err(),
    Error::ReadOnlySegmentExhausted
  );
  OK
}

/// UnsealReadOnlySegmentsScope: 解封期可写; 密封后读仍可见
#[test]
fn unseal_read_only_segments_scope() -> Void {
  let mut table = SegmentedTableBase::default();
  let ro_size = 2 * K_SEGMENT_SIZE;
  table
    .initialize(4 * K_SEGMENT_SIZE, ro_size, false)
    .expect("sandbox io");

  let ro_seg = table.allocate_read_only_segment().expect("sandbox io");
  let ro_addr = table.base() as usize + ro_seg.offset() as usize;

  {
    table.unseal_read_only_segments().expect("sandbox io");
    write_memory(ro_addr, 0x42);
    write_memory(ro_addr + 100, 0x43);
    assert_eq!(read_memory(ro_addr), 0x42);
    assert_eq!(read_memory(ro_addr + 100), 0x43);
    table.seal_read_only_segments().expect("sandbox io");
  }

  // 密封后: 读仍返回此前写入值 (Read 权限)
  assert_eq!(read_memory(ro_addr), 0x42);
  assert_eq!(read_memory(ro_addr + 100), 0x43);
  OK
}
