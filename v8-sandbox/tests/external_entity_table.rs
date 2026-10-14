// ExternalEntityTable 测试: 对应 pointer-table-unittest.cc 的 freelist/space 语义面
// (EPT 特有的 compaction/mark 面属第二梯队, 此处覆盖通用表语义)

use std::{collections::BTreeSet, mem::size_of};

use aok::{OK, Void};
use v8_sandbox::{
  Error, ExternalEntityTable, ExternalPointerPayload, ExternalPointerTag, ExternalPointerTagRange,
  K_NUM_READ_ONLY_SEGMENTS, K_SEGMENT_SIZE, Space, TableEntry,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 测试条目: 以 ExternalPointerPayload 位编码承载
/// (freelist = next|free_tag<<49, mark = 1<<48), 8 字节
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TestEntry(pub u64);

impl TableEntry for TestEntry {
  const IS_WRITE_PROTECTED: bool = false;
  const NULL: Self = Self(0);
  fn make_freelist_entry(&mut self, next_entry_index: u32) {
    self.0 = ExternalPointerPayload::tag(
      next_entry_index as usize,
      ExternalPointerTag::ExternalPointerFreeEntryTag,
    );
  }
  fn next_freelist_entry(&self) -> Option<u32> {
    ExternalPointerPayload {
      encoded_word: self.0,
    }
    .extract_freelist_link()
  }
  fn is_marked(&self) -> bool {
    ExternalPointerPayload {
      encoded_word: self.0,
    }
    .has_mark_bit_set()
  }
  fn unmark(&mut self) {
    let mut p = ExternalPointerPayload {
      encoded_word: self.0,
    };
    p.clear_mark_bit();
    self.0 = p.encoded_word;
  }
}

impl TestEntry {
  fn set(&mut self, value: usize, tag: ExternalPointerTag) {
    self.0 = ExternalPointerPayload::tag(value, tag);
  }
  fn get(&self, tag: ExternalPointerTag) -> usize {
    ExternalPointerPayload {
      encoded_word: self.0,
    }
    .untag_disallow_null(ExternalPointerTagRange::of(tag))
    .unwrap_or(0)
  }
  fn mark(&mut self) {
    self.0 |= 1 << 48;
  }
}

/// 4 段表: 1 RO + 3 常规 (SIZE = 4 * 64KB); eps = 8192
type TestTable = ExternalEntityTable<TestEntry, { 4 * K_SEGMENT_SIZE }>;
const EPS: u32 = (K_SEGMENT_SIZE / size_of::<TestEntry>()) as u32;

fn init_table() -> TestTable {
  let mut table = TestTable::default();
  table.initialize().expect("table init");
  table
}

/// 初始化 + 首 space 生命周期: 首个常规段 = 段 1 (段 0 为 RO 预留)
#[test]
fn space_lifecycle() -> Void {
  let mut table = init_table();
  assert!(table.is_initialized());
  assert_eq!(K_NUM_READ_ONLY_SEGMENTS, 1);
  assert_eq!(EPS, 8192);

  let mut space = Space::new();
  table.initialize_space(&mut space);
  assert!(space.is_empty());
  assert_eq!(space.freelist_length(), 0);

  let first = table.allocate_entry(&mut space).expect("sandbox io");
  // 段 0 属 RO 预留 → 常规分配从段 1 首 entry 开始
  assert_eq!(first, EPS);
  assert_eq!(space.num_segments_for_testing(), 1);
  assert_eq!(space.freelist_length(), EPS - 1);
  assert!(space.contains(first, EPS));
  assert!(!space.contains(0, EPS));

  table.tear_down_space(&mut space).expect("sandbox io");
  assert!(space.is_empty());
  assert_eq!(space.freelist_length(), 0);
  table.tear_down().expect("sandbox io");
  assert!(!table.is_initialized());
  OK
}

/// 条目分配/写入/读回 (pointer-table-unittest AllocateEntry 面)
#[test]
fn entry_allocation_and_access() -> Void {
  let mut table = init_table();
  let mut space = Space::new();
  let mut handles = Vec::new();
  for _ in 0..10 {
    let index = table.allocate_entry(&mut space).expect("sandbox io");
    handles.push(index);
    // SAFETY: 已分配段内槽位。
    unsafe { table.table.at(index) }.set(
      0x1234_0000 + index as usize,
      ExternalPointerTag::CFunctionTag,
    );
  }
  let mut distinct = BTreeSet::new();
  for &h in &handles {
    assert!(distinct.insert(h));
  }
  for &h in &handles {
    // SAFETY: 同上。
    let v = unsafe { table.table.at(h) }.get(ExternalPointerTag::CFunctionTag);
    assert_eq!(v, 0x1234_0000 + h as usize);
  }
  assert_eq!(space.freelist_length(), EPS - 10);
  table.tear_down_space(&mut space).expect("sandbox io");
  OK
}

/// GenericSweep: 未标记条目回 freelist; 全空段回收; freelist 排序自顶向下
#[test]
fn sweep_frees_unmarked_and_empty_segment() -> Void {
  let mut table = init_table();
  let mut space = Space::new();

  // 填满段 1 (EPS 个) + 段 2 的 1 个
  for _ in 0..=EPS {
    let _ = table.allocate_entry(&mut space).expect("sandbox io");
    // 不 mark: 全部按死亡条目进入 sweep
  }
  assert_eq!(space.num_segments_for_testing(), 2);

  // 全部未标记 sweep → 两段全空 → 段 2 回收 (段 1 保留? 全空也回收, 均为常规段)
  let live = table.generic_sweep(&mut space).expect("sandbox io");
  assert_eq!(live, 0);
  assert_eq!(space.num_segments_for_testing(), 0, "全空段应被回收");
  assert_eq!(space.freelist_length(), 0);

  // 重新分配: 新段 (first-fit 回到段 1)
  let again = table.allocate_entry(&mut space).expect("sandbox io");
  assert_eq!(again, EPS);
  table.tear_down_space(&mut space).expect("sandbox io");
  OK
}

/// sweep 部分存活: 存活条目保留 + 计数正确
#[test]
fn sweep_partial_live() -> Void {
  let mut table = init_table();
  let mut space = Space::new();
  let mut keep = Vec::new();
  for i in 0..16 {
    let index = table.allocate_entry(&mut space).expect("sandbox io");
    // SAFETY: 已分配槽位。
    let entry = unsafe { table.table.at(index) };
    entry.set(i as usize, ExternalPointerTag::CFunctionTag);
    if i % 2 == 0 {
      entry.mark();
      keep.push(index);
    }
  }
  let live = table.generic_sweep(&mut space).expect("sandbox io");
  assert_eq!(live, 8);
  assert_eq!(space.freelist_length(), EPS - 8);
  // 存活条目值仍在 (mark 已被 unmark)
  for &h in &keep {
    // SAFETY: 存活槽位。
    let e = unsafe { table.table.at(h) };
    assert!(!e.is_marked(), "sweep 后应已 unmark");
    assert_eq!(
      e.get(ExternalPointerTag::CFunctionTag),
      h as usize - EPS as usize
    );
  }
  table.tear_down_space(&mut space).expect("sandbox io");
  OK
}

/// AllocateEntryBelow: 只取低于阈值的表项, 不扩段; 无可用返回 0
#[test]
fn allocate_entry_below_threshold() -> Void {
  let mut table = init_table();
  let mut space = Space::new();
  // 首次扩段后 freelist 头 = EPS (段 1 首)
  let head = table.allocate_entry(&mut space).expect("sandbox io");
  assert_eq!(head, EPS);

  // 阈值 = 当前头 (EPS+1): 下一个空闲 EPS+1 >= 阈值 → 0
  assert_eq!(table.allocate_entry_below(&mut space, EPS + 1), 0);
  // 阈值足够大: 取到 EPS+1
  let below = table.allocate_entry_below(&mut space, EPS + 10);
  assert_eq!(below, EPS + 1);
  table.tear_down_space(&mut space).expect("sandbox io");
  OK
}

/// IterateEntriesIn: 段数 × eps 全量枚举
#[test]
fn iterate_entries() -> Void {
  let mut table = init_table();
  let mut space = Space::new();
  let _ = table.allocate_entry(&mut space);
  let mut count = 0;
  table.iterate_entries_in(&mut space, |i| {
    assert!((EPS..2 * EPS).contains(&i));
    count += 1;
  });
  assert_eq!(count, EPS);
  table.tear_down_space(&mut space).expect("sandbox io");
  OK
}

/// 共享读访问器 at_read/segment_slice_read (原版 at 的 const 重载):
/// &self 不可变路径读回写入值; sweep 后以只读视角验证 freelist 链
#[test]
fn shared_read_accessors() -> Void {
  let mut table = init_table();
  let mut space = Space::new();
  let a = table.allocate_entry(&mut space).expect("sandbox io");
  let b = table.allocate_entry(&mut space).expect("sandbox io");
  // SAFETY: 已分配段内槽位。
  unsafe { table.table.at(a) }.set(0xABCD, ExternalPointerTag::CFunctionTag);
  // SAFETY: 同段槽位。
  unsafe { table.table.at(b) }.set(0xEF01, ExternalPointerTag::CFunctionTag);

  // 共享引用路径: 与可变引用路径读到相同编码
  let table_ref = &table.table;
  // SAFETY: 槽位已提交可读。
  let ra = unsafe { table_ref.at_read(a) };
  // SAFETY: 同上。
  let rb = unsafe { table_ref.at_read(b) };
  assert_eq!(ra.get(ExternalPointerTag::CFunctionTag), 0xABCD);
  assert_eq!(rb.get(ExternalPointerTag::CFunctionTag), 0xEF01);
  // 段切片只读视角: 首段 (段 1) 全量可读, 写入槽位编码一致
  let segment = v8_sandbox::SegmentBase { number: 1 };
  // SAFETY: 段已分配。
  let slice = unsafe { table_ref.segment_slice_read(segment) };
  assert_eq!(slice.len() as u32, EPS);
  let sa = &slice[(a - EPS) as usize];
  let sb = &slice[(b - EPS) as usize];
  assert_eq!(sa.get(ExternalPointerTag::CFunctionTag), 0xABCD);
  assert_eq!(sb.get(ExternalPointerTag::CFunctionTag), 0xEF01);

  // sweep (a/b 已 mark 存活) 后只读视角: 条目已 unmark (mark 位清零) 且值保留;
  // 若未 mark, 全空段会被回收为 PROT_NONE, 槽位不可再读 (death.rs 已覆盖该面)
  // SAFETY: 已分配槽位。
  unsafe { table.table.at(a) }.mark();
  // SAFETY: 同上。
  unsafe { table.table.at(b) }.mark();
  let live = table.generic_sweep(&mut space).expect("sandbox io");
  assert_eq!(live, 2);
  let table_ref = &table.table;
  // SAFETY: 存活槽位仍在已分配段内。
  assert!(!unsafe { table_ref.at_read(a) }.is_marked());
  // SAFETY: 同上。
  assert!(!unsafe { table_ref.at_read(b) }.is_marked());
  // SAFETY: 同上。
  assert_eq!(
    unsafe { table_ref.at_read(a) }.get(ExternalPointerTag::CFunctionTag),
    0xABCD
  );
  table.tear_down_space(&mut space).expect("sandbox io");
  OK
}

/// RO space 挂接: null 表项 = 0 + 黑分配; 分离清空
#[test]
fn attach_ro_space() -> Void {
  let mut table = init_table();
  let mut space = Space::new();
  table.initialize_space(&mut space);
  table
    .attach_space_to_read_only_segments(&mut space)
    .expect("sandbox io");
  assert!(space.is_internal_read_only_space);
  assert!(space.allocate_black);
  assert_eq!(space.num_segments_for_testing(), 1);
  assert_eq!(space.freelist_length(), EPS - 1);
  // null 表项 (索引 0) 已清零
  // SAFETY: RO 首段已提交。
  assert_eq!(unsafe { table.table.at(0) }.0, 0);
  // RO space 继续分配在段 0 内
  let next = table.allocate_entry(&mut space).expect("sandbox io");
  assert_eq!(next, 1);

  table.detach_space_from_read_only_segments(&mut space);
  assert!(space.is_empty());
  assert_eq!(table.table.base.read_only_segments_used(), 0);
  // RO 段生命周期归表: tear_down_space 不释放 RO 段 (由表 TearDown 统一释放)
  table.tear_down_space(&mut space).expect("sandbox io");
  table.tear_down().expect("sandbox io");
  OK
}

/// 表预留耗尽: 分配失败返回 OutOfTableSpace (原版 OOM → 错误)
#[test]
fn allocation_exhaustion() -> Void {
  // 2 段表: 1 RO + 1 常规
  type SmallTable = ExternalEntityTable<TestEntry, { 2 * K_SEGMENT_SIZE }>;
  let mut table = SmallTable::default();
  table.initialize().expect("sandbox io");
  let mut space = Space::new();
  // 填满唯一常规段
  for _ in 0..EPS {
    let _ = table.allocate_entry(&mut space).expect("sandbox io");
  }
  assert_eq!(space.num_segments_for_testing(), 1);
  // 扩段失败 → allocate_entry 两次尝试后 Err
  let err = table.allocate_entry(&mut space).unwrap_err();
  assert_eq!(err, Error::OutOfTableSpace);
  table.tear_down_space(&mut space).expect("sandbox io");
  table.tear_down().expect("sandbox io");
  OK
}
