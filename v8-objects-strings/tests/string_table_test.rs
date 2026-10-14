// 迁移规约:
//   off-heap-hash-table-inl.h:74-115 容量策略 (HasSufficientCapacityToAdd/
//     ComputeCapacity/ComputeCapacityWithShrink)
//   off-heap-hash-table-inl.h:139-200 探测语义 (FindEntry/FindInsertionEntry/
//     FindEntryOrInsertionEntry 删除槽复用)
//   string-table.cc:169-897 表生命周期 (LookupKey/TryLookupKey/EnsureCapacity/
//     Resize 保链/DropOldData/NotifyElementsRemoved/反序列化/引导插入)
//   string-table-unittest.cc:17-48 HasString 规约 (foo×2/bar 内部化判定;
//     isolate/factory 依赖 → 以内容键 + 测试仲裁器承接, 规约强度守恒)
//   concurrent-string-unittest.cc / cctest/test-strings.cc: isolate 依赖 → 推迟
use std::{
  mem::size_of,
  sync::atomic::{AtomicUsize, Ordering},
};

use aok::{OK, Void};
use v8_objects_core::{
  name::{HASH_MASK, HASH_SHIFT, HashFieldType, Name},
  smi::Smi,
  tagged::{Object, Tagged},
  visitors::{FullObjectSlot, OffHeapObjectSlot, Root, RootVisitor},
};
use v8_objects_strings::string_table::{
  self, OffHeapStringHashSet, StringTable, StringTableKey, compute_capacity,
  compute_capacity_with_shrink, has_sufficient_capacity_to_add,
};

#[ctor::ctor(unsafe)]
fn init() {
  log_init::init();
}

// ── 测试仲裁器: 指针 → (哈希, 内容), 模拟堆对象哈希/长度/内容读取 ────────
#[derive(Default)]
struct Arena {
  entries: Vec<(usize, u32, &'static str)>,
}

/// 确定性内容哈希 (fnv1a) → 30 位原始哈希字段。
fn content_raw_hash_field(content: &str) -> u32 {
  let mut h: u32 = 0x811c_9dc5;
  for &b in content.as_bytes() {
    h = (h ^ u32::from(b)).wrapping_mul(0x0100_0193);
  }
  Name::create_hash_field_value(h & (HASH_MASK >> HASH_SHIFT), HashFieldType::Hash)
}

/// 原始哈希字段 → 30 位载荷 (HashBits::decode)。
fn decode_hash(raw_hash_field: u32) -> u32 {
  (raw_hash_field & HASH_MASK) >> HASH_SHIFT
}

impl Arena {
  fn intern(&mut self, content: &'static str) -> Tagged<Object> {
    if let Some(&(ptr, ..)) = self.entries.iter().find(|(_, _, c)| *c == content) {
      return Tagged::new(ptr);
    }
    // 堆对象样指针: 低位置 heap object tag, 非 Smi
    let ptr = 0x1_0001 + self.entries.len() * 16;
    let hash = decode_hash(content_raw_hash_field(content));
    self.entries.push((ptr, hash, content));
    Tagged::new(ptr)
  }

  fn lookup(&self, ptr: usize) -> Option<(u32, &'static str)> {
    self
      .entries
      .iter()
      .find(|(p, ..)| *p == ptr)
      .map(|(_, h, c)| (*h, *c))
  }

  fn lookup_by_content(&self, content: &str) -> Option<Tagged<Object>> {
    self
      .entries
      .iter()
      .find(|(_, _, c)| *c == content)
      .map(|(p, ..)| Tagged::new(*p))
  }
}

/// 内容键 (SequentialStringKey 家族的测试形态: 哈希 + 长度 + 内容比对)。
struct ContentKey<'a> {
  raw_hash_field: u32,
  length: u32,
  content: &'a str,
  arena: &'a Arena,
}

impl<'a> ContentKey<'a> {
  fn new(arena: &'a Arena, content: &'a str) -> Self {
    Self {
      raw_hash_field: content_raw_hash_field(content),
      length: content.len() as u32,
      content,
      arena,
    }
  }
}

impl StringTableKey for ContentKey<'_> {
  fn raw_hash_field(&self) -> u32 {
    self.raw_hash_field
  }
  fn set_raw_hash_field(&mut self, raw_hash_field: u32) {
    self.raw_hash_field = raw_hash_field;
  }
  fn length(&self) -> u32 {
    self.length
  }
  fn unwrap_thin_string(&self) -> Tagged<Object> {
    // 契约: 内容键恒非 ThinString (返回值无意义)
    Tagged::new(0)
  }
  fn get_handle_for_insertion(&self) -> Tagged<Object> {
    self
      .arena
      .lookup_by_content(self.content)
      .expect("插入前须先登记仲裁条目")
  }
  fn is_match(&self, element: Tagged<Object>, element_hash: u32) -> bool {
    // KeyIsMatch 剩余两行: 长度 + 内容 (哈希前置过滤由表完成)
    match self.arena.lookup(element.ptr()) {
      Some((hash, content)) => hash == element_hash && content == self.content,
      None => false,
    }
  }
}

/// 探测键 (指针相等; 反序列化/哨兵测试用)。
struct ProbeKey {
  element: Tagged<Object>,
  raw_hash: u32,
}

impl StringTableKey for ProbeKey {
  fn raw_hash_field(&self) -> u32 {
    self.raw_hash << HASH_SHIFT
  }
  fn set_raw_hash_field(&mut self, raw_hash_field: u32) {
    self.raw_hash = raw_hash_field >> HASH_SHIFT;
  }
  fn length(&self) -> u32 {
    0
  }
  fn unwrap_thin_string(&self) -> Tagged<Object> {
    self.element
  }
  fn get_handle_for_insertion(&self) -> Tagged<Object> {
    self.element
  }
  fn is_match(&self, element: Tagged<Object>, element_hash: u32) -> bool {
    element.ptr() == self.element.ptr() && element_hash == self.hash()
  }
}

/// 规约 (off-heap-hash-table-inl.h:74): 50% 空闲 + 删除槽约束的边界。
#[test]
fn has_sufficient_capacity_boundary() -> Void {
  // 充裕: 容量 8, 4 元素, 0 删除, 加 1 → nof=5, 需 2 空闲, 7 ≤ 8
  assert!(has_sufficient_capacity_to_add(8, 4, 0, 1));
  // 临界: nof=6 需 3 空闲 → 9 > 8 不够
  assert!(!has_sufficient_capacity_to_add(8, 5, 0, 1));
  // 删除槽约束: nof=3 时空闲 5, 删除槽上限 (8-3)/2=2
  assert!(!has_sufficient_capacity_to_add(8, 2, 4, 1));
  assert!(!has_sufficient_capacity_to_add(8, 2, 3, 1));
  assert!(has_sufficient_capacity_to_add(8, 2, 2, 1));
  // 满/溢出必拒
  assert!(!has_sufficient_capacity_to_add(8, 8, 0, 0));
  OK
}

/// 规约 (off-heap-hash-table-inl.h:91 ComputeCapacity):
/// 50% 松弛 + 2 的幂上取整 + 最小容量下限。
#[test]
fn compute_capacity_growth_ladder() -> Void {
  assert_eq!(compute_capacity(0), 2048);
  assert_eq!(compute_capacity(1), 2048);
  assert_eq!(compute_capacity(1365), 2048); // 1365+682=2047 → 2048
  assert_eq!(compute_capacity(1366), 4096); // 1366+683=2049 → 4096
  assert_eq!(compute_capacity(2731), 4096); // 2731+1365=4096 → 4096 (边界)
  assert_eq!(compute_capacity(2732), 8192); // 2732+1366=4098 → 8192
  OK
}

/// 规约 (off-heap-hash-table-inl.h:101 ComputeCapacityWithShrink):
/// 需求 ≤ capacity/4 才收缩; 不低于最小容量。
#[test]
fn compute_capacity_with_shrink_gate() -> Void {
  // 需求超过 capacity/4: 不收缩
  assert_eq!(compute_capacity_with_shrink(8192, 2049), 8192);
  // 过空: 收缩到重算容量
  assert_eq!(compute_capacity_with_shrink(8192, 100), 2048);
  // 最小容量下限: 不降到 2048 以下
  assert_eq!(compute_capacity_with_shrink(2048, 3), 2048);
  OK
}

/// 规约 (off-heap-hash-table-inl.h:139-200 + string-table.cc:469):
/// 插入/命中/同哈希冲突/删除槽复用语义 (直接驱动哈希集)。
#[test]
fn hash_set_insert_find_and_deleted_reuse() -> Void {
  let mut set = OffHeapStringHashSet::new(8);
  assert_eq!(set.capacity(), 8);

  let e1 = Tagged::<Object>::new(0x1_0001);
  let e2 = Tagged::<Object>::new(0x2_0001);
  let e3 = Tagged::<Object>::new(0x3_0001);

  // 空表查找: 未命中
  let key = ProbeKey {
    element: e1,
    raw_hash: 0x11,
  };
  assert_eq!(set.find_entry(&key, key.hash()), None);

  // 插入 + 命中
  let entry = set.find_insertion_entry(key.hash());
  set.add_at(entry, e1, key.hash());
  assert_eq!(set.number_of_elements(), 1);
  assert_eq!(set.find_entry(&key, key.hash()), Some(entry));

  // 同哈希不同元素: 探测序列推进不误命中
  let key2 = ProbeKey {
    element: e2,
    raw_hash: 0x11,
  };
  assert_eq!(set.find_entry(&key2, key2.hash()), None);
  let entry2 = set.find_entry_or_insertion_entry(&key2, key2.hash());
  assert_ne!(entry2, entry);
  set.add_at(entry2, e2, key2.hash());
  assert_eq!(set.find_entry(&key2, key2.hash()), Some(entry2));

  // 删除槽: GC 访客在槽地址写删除哨兵 (原版 StringTableCleaner 机制),
  // 随后计数回收 → FindEntryOrInsertionEntry 复用删除槽
  let mut deleter = DeleteSlotAt::new(3);
  set
    .iterate_elements(&mut deleter)
    .expect("遍历不可达段不存在");
  set.elements_removed(1);
  assert_eq!(set.number_of_deleted_elements(), 1);
  let key3 = ProbeKey {
    element: e3,
    raw_hash: 0x33,
  };
  // 0x33 & 7 = 3 → 探测首槽即删除槽, 应被复用而非跳过
  let entry3 = set.find_entry_or_insertion_entry(&key3, key3.hash());
  assert_eq!(entry3, 3);
  set.overwrite_deleted_at(entry3, e3, key3.hash());
  assert_eq!(set.number_of_deleted_elements(), 0);
  assert_eq!(set.number_of_elements(), 2);
  let key3 = ProbeKey {
    element: e3,
    raw_hash: 0x33,
  };
  assert_eq!(set.find_entry(&key3, key3.hash()), Some(3));
  OK
}

/// 单槽删除访客: 在指定槽地址原子写入删除哨兵 (模拟 GC StringTableCleaner)。
struct DeleteSlotAt {
  entry: usize,
  done: bool,
}

impl DeleteSlotAt {
  const fn new(entry: usize) -> Self {
    Self { entry, done: false }
  }
}

impl RootVisitor for DeleteSlotAt {
  fn visit_root_pointers(
    &mut self,
    _root: Root,
    _description: Option<&str>,
    _start: FullObjectSlot,
    _end: FullObjectSlot,
  ) -> v8_objects_core::Result<()> {
    unreachable!("字符串表遍历只走压缩槽路径")
  }

  fn visit_compressed_root_pointers(
    &mut self,
    _root: Root,
    _description: Option<&str>,
    start: OffHeapObjectSlot,
    end: OffHeapObjectSlot,
  ) -> v8_objects_core::Result<()> {
    debug_assert!(!self.done);
    // 槽宽 = AtomicUsize (本仓非压缩模型; OffHeapObjectSlot::next 的 4B 步进
    // 为压缩槽语义, 此处按实际布局推算, 登记见 string_table.rs 头注)
    let stride = size_of::<AtomicUsize>();
    let target = start.address + self.entry * stride;
    debug_assert!(target < end.address);
    // SAFETY: 地址由 iterate_elements 提供, 指向活表槽位; 原子写 = GC 清理语义
    unsafe {
      (&*(target as *const AtomicUsize)).store(Smi::from_int(1).ptr(), Ordering::Release);
    }
    self.done = true;
    Ok(())
  }
}

/// 规约 (string-table-unittest.cc:17 HasString, 规约先行迁移):
/// 初始皆不在表 → 内部化 foo → foo (两个同内容键) 与内部化句柄在, bar 不在
/// → 内部化 bar → 全部在。
#[test]
fn string_table_has_string_spec() -> Void {
  let mut arena = Arena::default();
  arena.intern("foo");
  arena.intern("bar");
  let table = StringTable::new();

  let has = |word: &str| {
    let key = ContentKey::new(&arena, word);
    table.has_string(&key)
  };

  // 初始: 均不在
  assert!(!has("foo"));
  assert!(!has("bar"));

  // 内部化 foo (LookupString 的表内语义 = LookupKey)
  let mut key = ContentKey::new(&arena, "foo");
  let foo_internalized = table.lookup_key(&mut key);
  assert_eq!(
    arena.lookup(foo_internalized.ptr()).map(|(_, c)| c),
    Some("foo")
  );

  // 同内容键 (原版 foo2) 命中; bar 不在
  assert!(has("foo"));
  assert!(!has("bar"));

  // 内部化 bar → 全部在
  let mut key = ContentKey::new(&arena, "bar");
  table.lookup_key(&mut key);
  assert!(has("foo"));
  assert!(has("bar"));
  assert_eq!(table.number_of_elements(), 2);
  OK
}

/// 规约 (string-table.cc:469 LookupKey 命中支): 同内容重复内部化返回同一元素
/// (引用相等 = 强一致性, string-table.cc:488-491)。
#[test]
fn lookup_key_returns_same_element_on_repeat() -> Void {
  let mut arena = Arena::default();
  arena.intern("dup");
  let table = StringTable::new();

  let mut k1 = ContentKey::new(&arena, "dup");
  let first = table.lookup_key(&mut k1);
  let mut k2 = ContentKey::new(&arena, "dup");
  let second = table.lookup_key(&mut k2);
  assert_eq!(first.ptr(), second.ptr());
  assert_eq!(table.number_of_elements(), 1);
  OK
}

/// 规约 (string-table.cc:583 EnsureCapacity + Data::Resize):
/// 越过 50% 阈值触发扩容, rehash 后全部键仍可命中, 计数守恒;
/// DropOldData 释放旧链不损语义。
#[test]
fn resize_preserves_all_elements() -> Void {
  let mut arena = Arena::default();
  let words: Vec<String> = (0..2000).map(|i| format!("key-{i:05}")).collect();
  for w in &words {
    // 静态仲裁器不可用动态串 → 换 Box::leak 固化生命周期
    let leaked: &'static str = Box::leak(w.clone().into_boxed_str());
    arena.intern(leaked);
  }
  let table = StringTable::new();
  assert_eq!(table.capacity(), 2048);

  for w in &words {
    let mut key = ContentKey::new(&arena, w);
    table.lookup_key(&mut key);
  }
  // 2000 元素: 2048 容量下 nof+nof/2 越限 → 已扩容
  assert!(table.capacity() >= 4096);
  assert_eq!(table.number_of_elements(), 2000);

  // rehash 后逐键命中
  for w in &words {
    let key = ContentKey::new(&arena, w);
    assert!(table.try_lookup_key(&key).is_some(), "扩容后失键: {w}");
  }

  // DropOldData 后旧链释放, 语义不变 (内存口径收缩)
  let usage_before = table.get_current_memory_usage();
  table.drop_old_data();
  assert!(table.get_current_memory_usage() < usage_before);
  let key = ContentKey::new(&arena, &words[0]);
  assert!(table.try_lookup_key(&key).is_some());
  OK
}

/// 规约 (string-table.cc:888 NotifyElementsRemoved): GC 删除计数回收
/// (槽位清删除哨兵归 GC 访客; 此处验证计数与后续插入的相互作用)。
#[test]
fn notify_elements_removed_accounting() -> Void {
  let mut arena = Arena::default();
  for i in 0..10 {
    arena.intern(Box::leak(format!("n{i}").into_boxed_str()));
  }
  let table = StringTable::new();
  for i in 0..10 {
    let word = format!("n{i}");
    let mut key = ContentKey::new(&arena, &word);
    table.lookup_key(&mut key);
  }
  assert_eq!(table.number_of_elements(), 10);
  table.notify_elements_removed(4);
  assert_eq!(table.number_of_elements(), 6);
  // 新插入仍可进行 (容量判定考虑删除槽)
  arena.intern("n10");
  let mut key = ContentKey::new(&arena, "n10");
  table.lookup_key(&mut key);
  assert_eq!(table.number_of_elements(), 7);
  OK
}

/// 规约 (string-table.cc:837/811): 引导插入 + 反序列化批量插入。
#[test]
fn bootstrapping_and_deserialization_insert() -> Void {
  let table = StringTable::new();
  let empty = Tagged::<Object>::new(0x1_0001);
  table.insert_empty_string_for_bootstrapping(empty, 0x77);
  assert_eq!(table.number_of_elements(), 1);
  let key = ProbeKey {
    element: empty,
    raw_hash: 0x77,
  };
  assert_eq!(
    table.try_lookup_key(&key).map(|o| o.ptr()),
    Some(empty.ptr())
  );

  // 反序列化插入要求空表 (原版 DCHECK_EQ(0, NumberOfElements)) → 独立新表
  let table = StringTable::new();
  let strings = [
    (Tagged::<Object>::new(0x2_0001), 0x01),
    (Tagged::<Object>::new(0x3_0001), 0x02),
    (Tagged::<Object>::new(0x4_0001), 0x03),
  ];
  table.insert_for_isolate_deserialization(&strings);
  assert_eq!(table.number_of_elements(), 3);
  for &(element, hash) in &strings {
    let key = ProbeKey {
      element,
      raw_hash: hash,
    };
    assert_eq!(
      table.try_lookup_key(&key).map(|o| o.ptr()),
      Some(element.ptr())
    );
  }
  OK
}

/// 规约 (string-table.h:53 空槽/删除哨兵 + globals.h:1963 ResultSentinel):
/// 查询失败判负值, 与非负数组索引可区分 (原版 static_assert 语义;
/// 编译期断言)。
const _: () = assert!(string_table::RESULT_NOT_FOUND < 0);
const _: () = assert!(string_table::RESULT_UNSUPPORTED < string_table::RESULT_NOT_FOUND);
const _: () = assert!(string_table::RESULT_NOT_FOUND != string_table::RESULT_UNSUPPORTED);

#[test]
fn sentinels_are_smi_zero_and_one() -> Void {
  assert_eq!(string_table::empty_element().ptr(), 0);
  assert_eq!(
    string_table::deleted_element().ptr(),
    Smi::from_int(1).ptr()
  );
  OK
}

/// 规约 (string-table.cc:469 LookupKey 并发论证):
/// 多线程并发插入互异键 → 最终元素数精确, 重复查询稳定同指针。
#[test]
fn concurrent_lookup_key_distinct_inserts() -> Void {
  use std::{sync::Arc, thread};

  const THREADS: usize = 8;
  const PER_THREAD: usize = 250;

  let mut arena = Arena::default();
  let words_by_thread: Vec<Vec<&'static str>> = (0..THREADS)
    .map(|t| {
      (0..PER_THREAD)
        .map(|i| {
          let word: &'static str = Box::leak(format!("t{t}-k{i:04}").into_boxed_str());
          arena.intern(word);
          word
        })
        .collect()
    })
    .collect();
  let arena = Arc::new(arena);
  let table = Arc::new(StringTable::new());

  let handles: Vec<_> = words_by_thread
    .into_iter()
    .map(|words| {
      let arena = Arc::clone(&arena);
      let table = Arc::clone(&table);
      thread::spawn(move || {
        for word in words {
          let mut key = ContentKey::new(&arena, word);
          let r1 = table.lookup_key(&mut key);
          // 二次查询须命中同一元素 (引用相等)
          let r2 = table.lookup_key(&mut key);
          assert_eq!(r1.ptr(), r2.ptr());
        }
      })
    })
    .collect();
  for h in handles {
    h.join().expect("工作线程不可 panic");
  }

  assert_eq!(table.number_of_elements(), THREADS * PER_THREAD);
  OK
}
