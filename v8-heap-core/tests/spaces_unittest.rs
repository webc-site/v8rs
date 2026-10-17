//! tests/spaces_unittest.rs — v8-heap-core/src/spaces.rs 金标准用例 (HCR1-F1, R5H4 波)。
//!
//! 原版测试勘定实测: v8/test/unittests/heap/spaces-unittest.cc (1628 行, vendored 树) 的
//! TEST_F 面 (grep 实测起于 :350 CompactionSpaceMerge / :585 AllocationObserver /
//! :665 MutablePage 等) 全部依赖真 Isolate/Heap/PagedSpace 运行态 → 不可直迁。本档按
//! HCR1 判「现行 mock 即可落」口径, 对 spaces.rs 已落地面复刻其可观测语义, 逐断言注释
//! 指原版 file:line 算例 (spaces.h / base-space.h / list.h / spaces-inl.h / spaces.cc),
//! 禁零断言冒烟:
//!   PageList 序语义      → page_list_ordering_gold / page_list_remove_missing_panics
//!   PageIteratorImpl 快照序 → page_iterator_advance_snapshot_gold
//!   for_all 覆盖         → for_all_enum_coverage_gold
//!   Account{,Un}committed 累计与回绕 → account_committed_uncommitted_gold + 双 should_panic
//!   first_page_address env 注入 → first_page_address_env_gold / 空表 should_panic
//!   Space 基类默认槽     → space_base_default_slots_gold / heap 判据 should_panic
//! 回绕/下溢四类 DCHECK 违例以 #[should_panic] 承接 (本仓 tests 无 catch_unwind 先例,
//! 门禁允许测试断言 panic; debug_assert 面依赖 dev profile — nextest 默认 ✓)。
//! should_panic 形按 rustc 约束返回 `()`, 常规用例保持本仓 Void/OK 尾 (memory_pool_unittest
//! 先例)。

use std::{ptr::null_mut, sync::atomic::Ordering};

use aok::{OK, Void};
use log::info;
use v8_common::{Address, AllocationSpace};
use v8_heap_core::{
  memory_allocator::{Heap, MutablePage, NormalPage},
  spaces::*,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

// ══════════════════════════════════════════════════════════════════════════
// 测试替身
// ══════════════════════════════════════════════════════════════════════════

/// 假页句柄 (MutablePage 为空 enum 承接位, 从不解引用; memory_pool_unittest.rs:111-112
/// iso(tag) 假指针先例同型)。
fn page(tag: usize) -> *mut MutablePage {
  tag as *mut MutablePage
}

/// 假 Heap (base-space.h:65 heap_ 位型; h:28 DCHECK 仅判非空)。
fn heap_fake() -> *mut Heap {
  0x9000 as *mut Heap
}

/// for_all 承载枚举 (原版 h:57-58 `template <typename Enum, ...>` 的 Enum 侧约束替身;
/// strum 型枚举 kNumValues 哨兵语义以 K_NUM_VALUES 承接)。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TestKind {
  A,
  B,
  C,
}

impl ForAllEnum for TestKind {
  /// 原版 h:59 `static_cast<int>(Enum::kNumValues)`。
  const K_NUM_VALUES: i32 = 3;
  /// 原版 h:60 `static_cast<Enum>(i)`。
  fn from_index(i: i32) -> Self {
    match i {
      0 => Self::A,
      1 => Self::B,
      _ => Self::C,
    }
  }
}

/// 零值枚举 (h:59 `0..kNumValues` 空区间边界支)。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TestNone {
  Z,
}

impl ForAllEnum for TestNone {
  const K_NUM_VALUES: i32 = 0;
  fn from_index(_i: i32) -> Self {
    Self::Z
  }
}

/// PageChunkAddressEnv 替身 (h:107-110 消费的 MutablePage::ChunkAddress() 判据宿主;
/// base-page.h `address_t ChunkAddress() const { return address_; }` → tag*0x10 映射)。
struct TagChunkEnv;

impl PageChunkAddressEnv for TagChunkEnv {
  fn chunk_address(&self, page: *mut MutablePage) -> Address {
    (page as usize) * 0x10
  }
}

/// Space trait 最小宿主 (h:64-120 默认槽承接面; 纯虚三槽 available/CommittedPhysical/
/// Size 以平凡实现满足 trait 完备 — h:81-82/h:38-39/base-space.h:41-42 覆写责任在
/// 具体空间, 本档只验默认槽)。
struct TestSpace {
  base: SpaceBase<NoFreeList>,
  size_val: usize,
}

impl Space for TestSpace {
  type FreeListKind = NoFreeList;

  fn space_base(&self) -> &SpaceBase<NoFreeList> {
    &self.base
  }
  fn space_base_mut(&mut self) -> &mut SpaceBase<NoFreeList> {
    &mut self.base
  }
  fn available(&self) -> usize {
    0
  }
  fn committed_physical_memory(&self) -> usize {
    self.space_base().committed_memory()
  }
  fn size(&self) -> usize {
    self.size_val
  }
}

// ══════════════════════════════════════════════════════════════════════════
// PageList 序语义 (list.h 消费面承接, h:118 memory_chunk_list_)
// ══════════════════════════════════════════════════════════════════════════

/// front/back 空表哨兵 (list.h:86-90 读出 + h:130-131 front_/back_ 初值 nullptr)、
/// PushBack 尾插入列序 (list.h:30-39; large-spaces.cc:193 消费)、Remove 按指针值摘除
/// 且余序不变 (list.h:51-57)、Empty (list.h:80; large-spaces.cc:63 循环条件)。
#[test]
fn page_list_ordering_gold() -> Void {
  let mut list = PageList::<MutablePage>::default();
  // list.h:130-131: front_/back_ 初值 nullptr → 空表双哨兵 (spaces.h:86-87 first/last_page 消费)。
  assert!(list.front().is_null());
  assert!(list.back().is_null());
  assert!(list.is_empty()); // list.h:80 Empty()
  assert_eq!(0, list.len());

  // list.h:30-39 PushBack: 序 = 入列序 (尾插)。
  list.push_back(page(0x10));
  list.push_back(page(0x20));
  list.push_back(page(0x30));
  assert_eq!(3, list.len());
  assert_eq!(page(0x10), list.front()); // h:86 front = 首入
  assert_eq!(page(0x30), list.back()); // h:88 back = 尾入
  assert_eq!(
    vec![page(0x10), page(0x20), page(0x30)],
    list.iter().collect::<Vec<_>>()
  ); // begin()/end() 序 (h:78-79)

  // list.h:51-57 Remove(中间项): 按指针值摘除, 余序保持入列序。
  list.remove(page(0x20));
  assert_eq!(2, list.len());
  assert_eq!(
    vec![page(0x10), page(0x30)],
    list.iter().collect::<Vec<_>>()
  );
  assert_eq!(page(0x10), list.front());
  assert_eq!(page(0x30), list.back());

  // 摘首项: front 换手 (back 不变)。
  list.remove(page(0x10));
  assert_eq!(page(0x30), list.front());
  assert_eq!(page(0x30), list.back());

  // 摘空: 回复空表哨兵 (list.h:80 + h:130-131 语义域)。
  list.remove(page(0x30));
  assert!(list.is_empty());
  assert!(list.front().is_null());
  assert!(list.back().is_null());
  info!("page_list_ordering_gold ok");
  OK
}

/// list.h:52 `DCHECK(ListContains(...))` — Remove 未命中页违例 (承接形: 移除命中即
/// 断言, spaces.rs PageList::remove 落点)。
#[test]
#[should_panic(expected = "list.h:Remove 未命中页")]
fn page_list_remove_missing_panics() {
  let mut list = PageList::<MutablePage>::default();
  list.push_back(page(0x10));
  list.remove(page(0x99)); // 不在表中 → DCHECK 违例
}

// ══════════════════════════════════════════════════════════════════════════
// PageIteratorImpl 推进快照序 (h:138-158 + spaces-inl.h:23-34)
// ══════════════════════════════════════════════════════════════════════════

/// 链上闭包注入 next_page (spaces-inl.h:24-27 `p_ = p_->next_page()` 承接形, 台账 6):
/// 前置 ++ 就地推进; 后置 ++(int) 快照旧值后推进 (inl:29-34 `tmp = this; ++this; tmp`)。
fn chain_next(p: *mut MutablePage) -> *mut MutablePage {
  match p as usize {
    0x10 => page(0x20),
    0x20 => page(0x30),
    _ => null_mut(),
  }
}

#[test]
fn page_iterator_advance_snapshot_gold() -> Void {
  // h:142 explicit ctor + h:146 operator* 读出 p_。
  let mut it = PageIteratorImpl::new(page(0x10));
  assert_eq!(page(0x10), it.deref());
  // h:147-149 operator== (p_ 指针值比较)。
  assert!(it.equals(&PageIteratorImpl::new(page(0x10))));
  // h:150-152 operator!=。
  assert!(it.not_equals(&PageIteratorImpl::new(page(0x20))));

  // spaces-inl.h:29-34 后置 ++: 返回快照 = 推进前旧 p_, 自身已推进。
  let snapshot = it.post_advance_with(chain_next);
  assert_eq!(page(0x10), snapshot.deref()); // 快照旧值 (inl:31 `auto tmp = *this`)
  assert_eq!(page(0x20), it.deref()); // 已推进 (inl:32 `++(*this)`)

  // spaces-inl.h:23-27 前置 ++: 推进后返回自身。
  it.advance_with(chain_next);
  assert_eq!(page(0x30), it.deref());
  it.advance_with(chain_next); // 尾项 → nullptr (end 哨兵, h:77 消费)
  assert!(it.equals(&PageIteratorImpl::<*mut MutablePage>::new(null_mut())));

  // h:160-163 别名同型: PageIterator/LargePageIterator 仅页指针型差, 推进面共享。
  let mut normal: PageIterator = PageIterator::new(null_mut());
  assert!(normal.deref().is_null());
  normal.advance_with(|_| null_mut());
  assert!(normal.deref().is_null());
  info!("page_iterator_advance_snapshot_gold ok");
  OK
}

// ══════════════════════════════════════════════════════════════════════════
// for_all 枚举遍历 (h:57-62)
// ══════════════════════════════════════════════════════════════════════════

/// h:59-61 `for (int i = 0; i < kNumValues; ++i) callback(static_cast<Enum>(i), i)`:
/// (值, 序号) 对 0..kNumValues 全覆盖且仅一次; 零值枚举空区间不回调。
#[test]
fn for_all_enum_coverage_gold() -> Void {
  let mut seen: Vec<(TestKind, i32)> = Vec::new();
  for_all::<TestKind>(|kind, idx| seen.push((kind, idx)));
  assert_eq!(
    vec![
      (TestKind::A, 0),
      (TestKind::B, 1),
      (TestKind::C, 2) // static_cast 逐值还原 (h:60)
    ],
    seen
  );
  assert_eq!(TestKind::K_NUM_VALUES as usize, seen.len()); // h:59 区间界含 kNumValues 哨兵前

  // h:59 `0..0` 空区间: 零值枚举回调 0 次。
  let mut none_seen = Vec::new();
  for_all::<TestNone>(|kind, idx| none_seen.push((kind, idx)));
  assert!(none_seen.is_empty());
  info!("for_all_enum_coverage_gold ok");
  OK
}

// ══════════════════════════════════════════════════════════════════════════
// AccountCommitted/AccountUncommitted (base-space.h:54-62)
// ══════════════════════════════════════════════════════════════════════════

/// h:69 committed_ 初值 0 → h:54-57 累计 + h:59-62 回退对偶; 零增量与清零边界。
#[test]
fn account_committed_uncommitted_gold() -> Void {
  let mut base = SpaceBase::<NoFreeList>::new(null_mut(), AllocationSpace::LoSpace, None);
  // base-space.h:69 `std::atomic<size_t> committed_{0}`。
  assert_eq!(0, base.committed_memory());

  // h:56 `committed_ += bytes` 累计。
  base.account_committed(5);
  base.account_committed(7);
  assert_eq!(12, base.committed_memory());

  // h:61 `committed_ -= bytes` 回退。
  base.account_uncommitted(4);
  assert_eq!(8, base.committed_memory());

  // 零增量边界: h:55 DCHECK_GE 恒成立 (checked 形对 0 无违例)。
  base.account_committed(0);
  assert_eq!(8, base.committed_memory());

  // 全量回退到下界 0 (h:60 DCHECK_GE(committed_, committed_ - bytes) 等号位)。
  base.account_uncommitted(8);
  assert_eq!(0, base.committed_memory());
  info!("account_committed_uncommitted_gold ok");
  OK
}

/// h:55 `DCHECK_GE(committed_ + bytes, committed_)` — 回绕违例 (HCR1-F2 整改后 checked_add
/// 形具备观测力; saturating 恒真形已清除)。
#[test]
#[should_panic(expected = "base-space.h:55 DCHECK_GE")]
fn account_committed_wrap_panics() {
  let mut base = SpaceBase::<NoFreeList>::new(null_mut(), AllocationSpace::LoSpace, None);
  base.committed.store(usize::MAX, Ordering::SeqCst); // MAX + 1 回绕域
  base.account_committed(1);
}

/// h:60 `DCHECK_GE(committed_, committed_ - bytes)` — 下溢违例 (checked_sub 同形)。
#[test]
#[should_panic(expected = "base-space.h:60 DCHECK_GE")]
fn account_uncommitted_underflow_panics() {
  let mut base = SpaceBase::<NoFreeList>::new(null_mut(), AllocationSpace::LoSpace, None);
  base.committed.store(3, Ordering::SeqCst);
  base.account_uncommitted(4); // 3 - 4 下溢 → DCHECK 违例
}

// ══════════════════════════════════════════════════════════════════════════
// FirstPageAddress (h:107-110) + 基类默认槽 (h:77-105)
// ══════════════════════════════════════════════════════════════════════════

/// h:109-110 `return first_page()->ChunkAddress()` — 判据经 PageChunkAddressEnv 注入
/// (base-page.h 实体他票); front 换表首后取值随动。
#[test]
fn first_page_address_env_gold() -> Void {
  let mut base = SpaceBase::<NoFreeList>::new(heap_fake(), AllocationSpace::LoSpace, None);
  base.memory_chunk_list.push_back(page(0x20));
  // h:107-110: 取 front (list.h:86) → env 判据 (0x20 * 0x10)。
  assert_eq!(0x200, base.first_page_address(&TagChunkEnv));
  // 尾插不改 front (list.h:30-39) → 取值仍首入页。
  base.memory_chunk_list.push_back(page(0x10));
  assert_eq!(0x200, base.first_page_address(&TagChunkEnv));
  // 摘首 → front 换手 (list.h:51-57) → 取值随动。
  base.memory_chunk_list.remove(page(0x20));
  assert_eq!(0x100, base.first_page_address(&TagChunkEnv));
  info!("first_page_address_env_gold ok");
  OK
}

/// h:108 `DCHECK_NOT_NULL(first_page())` — 空表取值违例。
#[test]
#[should_panic(expected = "h:108 DCHECK_NOT_NULL")]
fn first_page_address_empty_panics() {
  let base = SpaceBase::<NoFreeList>::new(heap_fake(), AllocationSpace::LoSpace, None);
  let _ = base.first_page_address(&TagChunkEnv);
}

/// base-space.h:28 `DCHECK_NOT_NULL(heap_)` — 空 heap 字段读出违例。
#[test]
#[should_panic(expected = "base-space.h:28 DCHECK_NOT_NULL")]
fn space_base_heap_null_panics() {
  let base = SpaceBase::<NoFreeList>::new(null_mut(), AllocationSpace::LoSpace, None);
  let _ = base.heap();
}

/// Space 基类默认槽组 (h:77-105): SizeOfObjects→Size 默认转发 (h:77-79)、first/last_page
/// → list front/back (h:86-94)、memory_chunk_list (h:96-98)、committed_memory 字段默认
/// (base-space.h:34-36)、InitializePage UNREACHABLE→null 承接 (h:100, 落点申报)、
/// NotifyBlackArea 双空默认 (h:102-103)、free_list_ nullptr 语义 (h:105 +
/// large-spaces.cc:53-54)。
#[test]
fn space_base_default_slots_gold() -> Void {
  let mut ts = TestSpace {
    base: SpaceBase::new(heap_fake(), AllocationSpace::LoSpace, None),
    size_val: 100,
  };
  // base-space.h:27-30/32 字段读出。
  assert!(ts.space_base().heap() == heap_fake());
  assert_eq!(AllocationSpace::LoSpace, ts.space_base().identity());
  // h:77-79 `SizeOfObjects() { return Size(); }` 默认转发 (LO 覆写对照 large_spaces)。
  assert_eq!(100, ts.size_of_objects());
  // base-space.h:34-36 committed 默认读字段 + h:54-57 入账联动。
  assert_eq!(0, ts.committed_memory());
  ts.space_base_mut().account_committed(4096);
  assert_eq!(4096, ts.committed_memory());

  // h:86-94 first/last_page 默认: 空表双 nullptr → 入列后首/尾。
  assert!(ts.first_page().is_null());
  assert!(ts.last_page().is_null());
  ts.memory_chunk_list().push_back(page(0x10)); // h:96-98 list 槽可改
  ts.memory_chunk_list().push_back(page(0x20));
  assert_eq!(page(0x10), ts.first_page());
  assert_eq!(page(0x20), ts.last_page());
  assert_eq!(2, ts.space_base().memory_chunk_list.len()); // 同表回读 (双存储单源)

  // h:100 InitializePage 原版 UNREACHABLE → 基类空指针承接 (PagedSpace 覆写他票, 台账 5)。
  let initialized: *mut NormalPage = ts.initialize_page(page(0x30));
  assert!(initialized.is_null());

  // h:102-103 NotifyBlackArea{Created,Destroyed} 基类空默认: 状态零扰动。
  ts.notify_black_area_created(8);
  ts.notify_black_area_destroyed(8);
  assert_eq!(2, ts.space_base().memory_chunk_list.len());
  assert_eq!(4096, ts.committed_memory());

  // h:105 free_list() + cc:53-54 nullptr: 无链空间恒 None (NoFreeList 占位从不消费)。
  assert!(ts.space_base_mut().free_list().is_none());
  info!("space_base_default_slots_gold ok");
  OK
}
