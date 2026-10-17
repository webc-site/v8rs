//! tests/memory_pool_unittest.rs — v8-heap-core/src/memory_pool.rs 金标准用例 (HC-23R)。
//!
//! 原版测试勘定实测: v8/test/unittests/heap 无 memory-pool / pooled-page 专属 unittest
//! (find 实测 0 命中; heap-unittest.cc 的 MemoryPool 消费面 (BlackAllocatedPages 等) 全
//! 依赖 Isolate/Heap 运行态, 不可直迁)。按票面口径: 基于原版 memory-pool.h/.cc 的
//! CHECK/DCHECK 断言语义与注释契约自造等价覆盖:
//!   池静态表区 cc:106-113 (PoolImpl::PutLocal/Get) + cc:112-144 取池优先级 →
//!     pool_impl_get_priority_gold;
//!   ReleaseUpTo cc:202-234 (cc:209 epoch 界 + cc:228-230 emptied 条件) → epoch 界用例;
//!   MoveLocalToShared cc:146-158 / Size 族 cc:174-200 / TearDown cc:97-103 + cc:330
//!     (CHECK 面只在空池调用, 违例走 v8 fatal 不可测) → 对应金标准;
//!   LargePagePoolImpl Add/Remove cc:236-288 (cc:240/256/264/286 total_size DCHECK 恒等 +
//!     best-fit cc:266-277) → budget/best-fit 用例;
//!   PooledPage ToResult cc:87-95 + dtor cc:76-85 → to_result 用例;
//!   MemoryPool Add/Remove/Zone cc:401-443 + ReleaseOnTearDown cc:346-355 双分支 +
//!     PostDelayedReleaseTaskIfNeeded cc:514-529 + kTimeSlack cc:517 → pool 级用例;
//!   Config 默认 h:82-86; 打点文案 cc:472-475。
//! 页面对象/VirtualMemory 实体属他票 (台账 1/6) → Reservation 窄 trait 测试替身承接。

use std::ptr::null_mut;

use aok::{OK, Void};
use log::info;
use v8_base::address_region::AddressRegion;
use v8_base_platform::TimeDelta;
use v8_common::MB;
use v8_heap_core::memory_pool::*;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

// ══════════════════════════════════════════════════════════════════════════
// 测试替身
// ══════════════════════════════════════════════════════════════════════════

/// VirtualMemory 替身 (台账 6 面): 预留位型 (address,size) + reserved 位开关。
/// free/reset 均翻 reserved (原版语义: free 归还系统 / reset 弃所有权不还 — 观测面在本档
/// 仅需 is_reserved 状态转移); metadata 恒以 null 交出的 Drop 路径由 libc free(null) 兜住。
#[derive(Clone, Copy, Debug)]
struct TestRes {
  address: usize,
  size: usize,
  reserved: bool,
}

impl TestRes {
  fn reserved(address: usize, size: usize) -> Self {
    Self {
      address,
      size,
      reserved: true,
    }
  }
}

impl Reservation for TestRes {
  fn is_reserved(&self) -> bool {
    self.reserved
  }
  fn size(&self) -> usize {
    self.size
  }
  fn region(&self) -> AddressRegion {
    AddressRegion::new(self.address, self.size)
  }
  fn free(&mut self) {
    self.reserved = false;
  }
  fn reset(&mut self) {
    self.reserved = false;
  }
}

/// PoolImpl<E> 通用条目 (epoch 界判据 = cc:209)。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Mark {
  epoch: Epoch,
  id: usize,
}

impl PooledEntry for Mark {
  fn epoch(&self) -> Epoch {
    self.epoch
  }
}

/// ReleaseTaskHost 替身: 记录投递 (任务快照+延迟秒) 与打点文案。
#[derive(Default)]
struct TestHost {
  posts: Vec<(ReleasePooledChunksTask, TimeDelta)>,
  printed: Vec<String>,
}

impl ReleaseTaskHost for TestHost {
  fn post_delayed_release_task(
    &mut self,
    task: &ReleasePooledChunksTask,
    _pool_ctm: &v8_tasks::CancelableTaskManager,
    delay: TimeDelta,
  ) {
    self.posts.push((*task, delay));
  }
  fn print(&mut self, msg: &str) {
    self.printed.push(msg.to_string());
  }
}

/// 假 isolate (原版仅透传指针 + map 键; 台账 4 面宿主不可达)。
fn iso(tag: usize) -> *mut Isolate {
  (tag * 0x1000) as *mut Isolate
}

fn page(address: usize, size: usize, epoch: Epoch) -> PooledPage<TestRes> {
  PooledPage::new(null_mut(), TestRes::reserved(address, size), epoch)
}

fn timeout_cfg() -> Config {
  Config {
    timeout_in_sec: 8,
    ..Config::default()
  }
}

// ══════════════════════════════════════════════════════════════════════════
// Config / 统计结构默认值 (h:82-86, h:156-159, h:228-233)
// ══════════════════════════════════════════════════════════════════════════

/// Config 默认 5 项金标准 (h:82-86 类内初始化)。
#[test]
fn config_defaults_gold() -> Void {
  let c = Config::default();
  assert!(!c.single_threaded); // h:82
  assert!(c.share_memory_on_teardown); // h:83
  assert!(!c.trace_gc_nvp); // h:84
  assert_eq!(32, c.max_large_page_pool_size); // h:85 (单位 MB, cc:242 消费)
  assert_eq!(8, c.timeout_in_sec); // h:86
  info!("config_defaults_gold ok");
  OK
}

/// ReleaseStats 默认 + trace_gc_nvp 打点文案 (%zu ×3 逐位对应 + 结尾换行, cc:472-475)。
#[test]
fn release_stats_message_gold() -> Void {
  let s = ReleaseStats::default();
  assert_eq!(0, s.pages_removed);
  assert!(!s.pool_emptied); // h:232 默认 false
  let s = ReleaseStats {
    pages_removed: 1,
    large_pages_removed: 2,
    zone_reservations_removed: 3,
    pool_emptied: true,
  };
  assert_eq!(
    "Memory pool: Removed pages: 1, removed large pages: 2, removed zone reservations: 3\n",
    s.message()
  );
  // PoolReleaseStats 默认 (h:157-158)
  let p = PoolReleaseStats::default();
  assert_eq!(0, p.removed_entries);
  assert!(!p.pool_emptied);
  info!("release_stats_message_gold ok");
  OK
}

// ══════════════════════════════════════════════════════════════════════════
// PoolImpl 池静态表区 (cc:105-144: PutLocal 尾插 + Get 三级优先 + 桶空删键)
// ══════════════════════════════════════════════════════════════════════════

/// cc:105-110 PutLocal 尾插 + cc:112-144 Get 优先级: 本 isolate 本地桶尾 →
/// 共享池尾 (cc:120-126) → 窃取任一本地桶 (cc:128-129) → 空返回 nullopt (cc:143);
/// cc:137-139 桶空即删键; cc:132-141 弹出序 = 尾后进先出。
#[test]
fn pool_impl_get_priority_gold() -> Void {
  let mut pool = PoolImpl::<Mark>::new(Config::default());

  // cc:105-110: 同 isolate 尾插 [m1, m2]; cc:134 DCHECK 桶非空 → 尾弹 (LIFO)
  pool.put_local(iso(1), Mark { epoch: 0, id: 1 });
  pool.put_local(iso(1), Mark { epoch: 0, id: 2 });
  assert_eq!(2, pool.local_size(iso(1)));
  assert_eq!(2, pool.size()); // cc:174-181 local + shared 合计
  let got = pool.get(iso(1)).unwrap();
  assert_eq!(2, got.id); // 尾元素
  let got = pool.get(iso(1)).unwrap();
  assert_eq!(1, got.id);
  assert!(!pool.local_pools.contains_key(&iso(1))); // cc:137-139 桶空删键

  // cc:146-158 MoveLocalToShared: 整桶搬入共享池 + 桶删 + 返回共享池非空
  pool.put_local(iso(2), Mark { epoch: 0, id: 3 });
  assert!(pool.move_local_to_shared(iso(2)));
  assert_eq!(1, pool.shared_size()); // cc:196-200
  assert_eq!(0, pool.local_size(iso(2)));

  // 本地命中优先于共享尾 (cc:118 find 命中 → cc:132 分支, 不触共享池)
  pool.put_local(iso(3), Mark { epoch: 0, id: 4 });
  let got = pool.get(iso(3)).unwrap();
  assert_eq!(4, got.id);
  assert_eq!(1, pool.shared_size()); // 共享池未动

  // 无桶 isolate → 共享尾优先 (cc:120-126)
  let got = pool.get(iso(9)).unwrap();
  assert_eq!(3, got.id);
  assert!(pool.get(iso(9)).is_none()); // cc:143 两级皆空

  // 窃取面 (cc:128-131): 无桶且共享空 → 从任意他 isolate 本地桶偷
  pool.put_local(iso(4), Mark { epoch: 0, id: 5 });
  let got = pool.get(iso(9)).unwrap();
  assert_eq!(5, got.id);
  assert!(!pool.local_pools.contains_key(&iso(4))); // 偷空即删键
  assert!(pool.get(iso(9)).is_none());
  // Drop DCHECK (h:167-168): 两桶皆空态安全落释
  info!("pool_impl_get_priority_gold ok");
  OK
}

/// cc:202-234 ReleaseUpTo: `entry.epoch() <= release_epoch` (cc:209) 界内冲刷 (先共享
/// cc:217-218 后本地 cc:219-227); 空桶键移除; 两级皆空置 pool_emptied (cc:228-230)。
#[test]
fn pool_release_up_to_epoch_boundary_gold() -> Void {
  let mut pool = PoolImpl::<Mark>::new(Config::default());
  pool.put_local(iso(1), Mark { epoch: 1, id: 1 });
  pool.put_local(iso(1), Mark { epoch: 5, id: 2 });
  pool.put_local(iso(2), Mark { epoch: 3, id: 3 });
  assert!(pool.move_local_to_shared(iso(2))); // 共享池 [epoch3]

  // 界: 3 <= 3 含 (cc:209 <=); 1 也在界内 → 共享 1 + 本地 1 = 2 移除
  let stats = pool.release_up_to(3);
  assert_eq!(2, stats.removed_entries);
  assert!(!stats.pool_emptied); // epoch5 残留 (cc:228 条件不满足)
  assert_eq!(1, pool.size());

  // 界 5: 最后条目冲净 → emptied
  let stats = pool.release_up_to(5);
  assert_eq!(1, stats.removed_entries);
  assert!(stats.pool_emptied);
  assert_eq!(0, pool.size());

  // 空池再冲: 0 移除仍 emptied (cc:228-230 恒真)
  let stats = pool.release_up_to(5);
  assert_eq!(0, stats.removed_entries);
  assert!(stats.pool_emptied);

  // 下界: release_epoch=0 无条目达标 (epoch ≥ 1)
  pool.put_local(iso(1), Mark { epoch: 1, id: 9 });
  let stats = pool.release_up_to(0);
  assert_eq!(0, stats.removed_entries);
  assert!(!stats.pool_emptied);
  pool.release_local(); // cc:166-170 全清 (Drop DCHECK 前置)
  assert_eq!(0, pool.size());
  info!("pool_release_up_to_epoch_boundary_gold ok");
  OK
}

/// cc:97-103 TearDown: 删 "nullptr-isolate" 桶 (h:175-176 注释用途) → DCHECK 本地全空 →
/// 清共享池; 前置条件: 心跳任务已取消 (cc:372-373), 此处以空池+哨兵桶实测。
#[test]
fn pool_tear_down_clears_sentinel_and_shared_gold() -> Void {
  let mut pool = PoolImpl::<Mark>::new(Config::default());
  pool.put_local(null_mut(), Mark { epoch: 0, id: 1 }); // "nullptr-isolate" 桶
  pool.put_local(null_mut(), Mark { epoch: 0, id: 2 });
  assert!(pool.move_local_to_shared(null_mut())); // 哨兵桶搬共享
  pool.put_local(iso(1), Mark { epoch: 0, id: 3 });
  pool.get(iso(1)).unwrap(); // 清本地桶 — cc:101 DCHECK(local_pools_.empty()) 前置

  pool.tear_down();
  assert!(pool.local_pools.is_empty());
  assert!(pool.shared_pool.is_empty()); // cc:102 shared_pool_.clear()
  assert_eq!(0, pool.size());
  info!("pool_tear_down_clears_sentinel_and_shared_gold ok");
  OK
}

/// cc:160-176 ReleaseShared/ReleaseLocal (两重载) 金标准。
#[test]
fn pool_release_shared_local_gold() -> Void {
  let mut pool = PoolImpl::<Mark>::new(Config::default());
  pool.put_local(iso(1), Mark { epoch: 0, id: 1 });
  pool.put_local(iso(2), Mark { epoch: 0, id: 2 });
  assert!(pool.move_local_to_shared(iso(1)));
  assert!(pool.move_local_to_shared(iso(2)));
  assert_eq!(2, pool.shared_size());

  pool.release_shared(); // cc:160-164
  assert_eq!(0, pool.shared_size());

  pool.put_local(iso(1), Mark { epoch: 0, id: 3 });
  pool.put_local(iso(2), Mark { epoch: 0, id: 4 });
  pool.release_local_of(iso(1)); // cc:172-176 指定 isolate
  assert_eq!(0, pool.local_size(iso(1)));
  assert_eq!(1, pool.local_size(iso(2)));
  pool.release_local(); // cc:166-170 全量
  assert_eq!(0, pool.size());
  info!("pool_release_shared_local_gold ok");
  OK
}

// ══════════════════════════════════════════════════════════════════════════
// LargePagePoolImpl (cc:236-330): 预算 + best-fit + total_size DCHECK 恒等
// ══════════════════════════════════════════════════════════════════════════

/// cc:236-258 Add: max_total_size = config.max_large_page_pool_size * MB (cc:242);
/// 预算内入池并从入参移除 (erase_if 语义), 超预算原地留下由调用方释放; 返回值
/// = 是否有条目入池 (cc:238/257)。
#[test]
fn large_pool_budget_erase_if_gold() -> Void {
  let cfg = Config {
    max_large_page_pool_size: 1, // 1 MB
    ..Config::default()
  };
  let mut pool = LargePagePoolImpl::<TestRes>::new(cfg);
  let mut pages = vec![page(0x1000, 400 * 1024, 0), page(0x2000, 700 * 1024, 0)];
  let added = pool.add(&mut pages);
  assert!(added); // cc:257 return added_to_pool
  assert_eq!(1, pool.pages.len()); // 400K 入池
  assert_eq!(1, pages.len()); // 700K 超预算 (400+700 > 1024K) 原地留
  assert_eq!(400 * 1024, pool.compute_total_size()); // cc:320-328 重算恒等 (cc:256 DCHECK)

  // 全超预算: 无条目入池 → false (cc:238 初值不翻转)
  let mut pages = vec![page(0x3000, 2 * MB, 0)];
  assert!(!pool.add(&mut pages));
  assert_eq!(1, pages.len());

  // 超预算留下的条目出作用域即被释放 (原版调用方 delete 面) — 此处显式清池保 Drop DCHECK
  pool.release_all(); // cc:290-294
  assert_eq!(0, pool.compute_total_size());
  assert!(pool.pages.is_empty());
  info!("large_pool_budget_erase_if_gold ok");
  OK
}

/// cc:260-288 Remove: best-fit — size >= chunk_size 中最小者, 同值取先 (cc:274 严格 <);
/// 取走即减预算 (cc:284); 无满足者 nullopt (cc:279-281)。
#[test]
fn large_pool_remove_best_fit_gold() -> Void {
  let mut pool = LargePagePoolImpl::<TestRes>::new(Config::default()); // 32MB 预算
  let sizes = [300 * 1024usize, 64 * 1024, 128 * 1024, 300 * 1024];
  let mut pages = vec![
    page(0x1000, sizes[0], 0),
    page(0x2000, sizes[1], 0),
    page(0x3000, sizes[2], 0),
    page(0x4000, sizes[3], 0),
  ];
  assert!(pool.add(&mut pages));
  assert!(pages.is_empty());
  assert_eq!(792 * 1024, pool.compute_total_size());

  // 要 100K → 128K (大于需求的最小者, cc:266-268 注释)
  let r = pool.remove(iso(1), 100 * 1024).unwrap();
  assert_eq!(128 * 1024, r.uninitialized_chunk.size());
  assert_eq!(0x3000, r.uninitialized_chunk.begin());
  assert_eq!(664 * 1024, pool.compute_total_size()); // cc:286 DCHECK 恒等

  // 要 300K → 首个 300K (同值取先, cc:274 严格小于比较不翻位)
  let r = pool.remove(iso(1), 300 * 1024).unwrap();
  assert_eq!(0x1000, r.uninitialized_chunk.begin());

  // 再要 300K → 余下一块 300K
  let r = pool.remove(iso(1), 300 * 1024).unwrap();
  assert_eq!(0x4000, r.uninitialized_chunk.begin());
  assert_eq!(64 * 1024, pool.compute_total_size());

  // 无满足者 → None (cc:279-281)
  assert!(pool.remove(iso(1), MB).is_none());
  // 精确命中现存唯一块
  let r = pool.remove(iso(1), 64 * 1024).unwrap();
  assert_eq!(0x2000, r.uninitialized_chunk.begin());
  assert!(pool.pages.is_empty());
  // cc:330 TearDown CHECK(pages_.empty()) — 空池态合法调用
  pool.tear_down();
  info!("large_pool_remove_best_fit_gold ok");
  OK
}

/// cc:296-318 大页版 ReleaseUpTo: epoch 界冲净 + total_size 同步减 (cc:304);
/// pages_ 空即 emptied (cc:310-313)。
#[test]
fn large_pool_release_up_to_gold() -> Void {
  let mut pool = LargePagePoolImpl::<TestRes>::new(Config::default());
  let mut pages = vec![page(0x1000, 64 * 1024, 3), page(0x2000, 128 * 1024, 7)];
  assert!(pool.add(&mut pages));

  let s = pool.release_up_to(5); // epoch<=5 仅首块
  assert_eq!(1, s.removed_entries);
  assert!(!s.pool_emptied);
  assert_eq!(128 * 1024, pool.compute_total_size()); // cc:304 减账

  let s = pool.release_up_to(7);
  assert_eq!(1, s.removed_entries);
  assert!(s.pool_emptied);
  assert_eq!(0, pool.compute_total_size());
  info!("large_pool_release_up_to_gold ok");
  OK
}

// ══════════════════════════════════════════════════════════════════════════
// PooledPage (cc:20-24/87-95 + h:45-46)
// ══════════════════════════════════════════════════════════════════════════

/// ToResult cc:87-95: region/metadata 二元组搬出 → reservation Reset (不归还) →
/// metadata 置空 → 后续 Drop cc:77-79 早退 (调用方接管内存)。
#[test]
fn pooled_page_to_result_ownership_transfer_gold() -> Void {
  let mut p = PooledPage::new(null_mut(), TestRes::reserved(0x8000, 256 * 1024), 9);
  assert_eq!(256 * 1024, p.size()); // h:45 reservation_.size()
  assert_eq!(9, p.epoch()); // h:46
  let r = p.to_result();
  assert_eq!(0x8000, r.uninitialized_chunk.begin());
  assert_eq!(256 * 1024, r.uninitialized_chunk.size());
  assert_eq!(0, r.uninitialized_metadata); // cc:89 取副本 (null 入料)
  assert!(p.uninitialized_metadata.is_null()); // cc:93 置空
  assert!(!p.reservation.is_reserved()); // cc:92 Reset — Drop 不再归还 (cc:77-79)
  drop(p); // 未 reserved → 无 free 路径
  info!("pooled_page_to_result_ownership_transfer_gold ok");
  OK
}

/// 未转移所有权的 PooledPage 落释走 cc:76-85 归还分支 (free + libc free(null));
/// PooledEntry 面 (h:161-190 模板约束, cc:209 消费) 与 PooledVirtualMemory::epoch 同源。
#[test]
fn pooled_entry_epoch_and_drop_gold() -> Void {
  let p = page(0x9000, 4096, 42);
  assert_eq!(42, <PooledPage<TestRes> as PooledEntry>::epoch(&p));
  let mut vm = PooledVirtualMemory::new(TestRes::reserved(0xA000, 8192), 7);
  assert_eq!(7, vm.epoch());
  assert_eq!(8192, vm.size()); // h:221 memory_.size()
  assert_eq!(0xA000, vm.virtual_memory().region().begin()); // h:219
  let vm2 = PooledVirtualMemory::new(TestRes::reserved(0xB000, 8192), 7);
  assert_eq!(
    7,
    <PooledVirtualMemory<TestRes> as PooledEntry>::epoch(&vm2)
  );
  drop(p); // reserved → free 分支执行 (TestRes::free 翻位; metadata null → libc free 无害)
  let r = vm.into_virtual_memory(); // cc:441 搬出面
  assert_eq!(0xA000, r.address);
  info!("pooled_entry_epoch_and_drop_gold ok");
  OK
}

// ══════════════════════════════════════════════════════════════════════════
// MemoryPool 门面 (cc:337-536)
// ══════════════════════════════════════════════════════════════════════════

/// cc:401-443 Add/Remove/AddLarge/RemoveLarge/AddZoneReservation/RemoveZoneReservation +
/// cc:393-399 GetCount/GetSharedCount/GetTotalCount; zone 允许 nullptr isolate (h:112-115)。
#[test]
fn memory_pool_facade_counts_gold() -> Void {
  let mut host = TestHost::default();
  // timeout=0: add 不再触发心跳投递 (cc:518 if pool_timeout == 0 return), 门面纯净
  let cfg = Config {
    timeout_in_sec: 0,
    ..Config::default()
  };
  let mut pool = MemoryPool::<TestRes>::new(cfg);
  assert_eq!(0, pool.current_epoch()); // h:250 初值
  assert_eq!(0, pool.get_count(iso(1)));
  assert_eq!(0, pool.get_shared_count());
  assert_eq!(0, pool.get_total_count());

  // 页池: add → count → remove (ToResult 搬出)
  pool.add(iso(1), page(0x1000, 4096, 0), &mut host);
  assert_eq!(1, pool.get_count(iso(1))); // cc:393-395 LocalSize
  assert_eq!(1, pool.get_total_count()); // cc:399 Size
  let r = pool.remove(iso(1)).unwrap(); // cc:410-417
  assert_eq!(4096, r.uninitialized_chunk.size());
  assert_eq!(0, pool.get_total_count());
  assert!(pool.remove(iso(1)).is_none()); // 空池取 → nullopt (cc:413-415)

  // 大页池: add_large (台账 1 入参形) → remove_large best-fit → 计数面不涉大页
  let mut pages = vec![page(0x2000, 128 * 1024, 0)];
  pool.add_large(iso(1), &mut pages, &mut host);
  assert!(pages.is_empty());
  let r = pool.remove_large(iso(1), 100 * 1024).unwrap(); // cc:423-426
  assert_eq!(128 * 1024, r.uninitialized_chunk.size());
  assert!(pool.remove_large(iso(1), MB).is_none());

  // zone: nullptr isolate 合法 (h:112-115 注释)
  pool.add_zone_reservation(null_mut(), TestRes::reserved(0x3000, 8192), &mut host);
  let vm = pool.remove_zone_reservation(null_mut()).unwrap(); // cc:437-443
  assert_eq!(8192, vm.size());
  assert_eq!(0, pool.get_shared_count()); // cc:397 页共享池独立记账
  // cc:371-377 TearDown: 取消心跳 + 三池拆除 (CTM Drop 的 CHECK(canceled_) 前置,
  // 原版 ~CancelableTaskManager 亦要求已取消)
  pool.tear_down();
  info!("memory_pool_facade_counts_gold ok");
  OK
}

/// cc:346-369 ReleaseOnTearDown 双分支 + ReleaseAllImmediately:
/// share_memory_on_teardown=true → 本地转共享 + 大页清空 (给他 isolate 复用机会);
/// false → ReleaseImmediately (本地直释)。
#[test]
fn release_on_teardown_branches_gold() -> Void {
  let mut host = TestHost::default();
  // 分支 1: share=true
  let cfg = Config {
    timeout_in_sec: 0,
    share_memory_on_teardown: true,
    ..Config::default()
  };
  let mut pool = MemoryPool::<TestRes>::new(cfg);
  pool.add(iso(1), page(0x1000, 4096, 0), &mut host);
  pool.add_zone_reservation(iso(1), TestRes::reserved(0x2000, 512), &mut host);
  pool.add_large(iso(1), &mut vec![page(0x3000, 64 * 1024, 0)], &mut host);
  pool.release_on_teardown(iso(1));
  assert_eq!(0, pool.get_count(iso(1))); // 本地清空
  assert_eq!(1, pool.get_shared_count()); // 页转共享 (cc:352)
  // 其他 isolate 可复用共享页 (cc:120-126 共享尾优先)
  let r = pool.remove(iso(2)).unwrap();
  assert_eq!(0x1000, r.uninitialized_chunk.begin());
  assert_eq!(0, pool.get_shared_count());
  // zone 池同步走 cc:352-353 共享化 → 他 isolate 复用同路径 (Drop DCHECK 前置清池)
  let vm = pool.remove_zone_reservation(iso(2)).unwrap();
  assert_eq!(512, vm.size());
  pool.tear_down(); // cc:371-377: 取消心跳 + 三池拆除 (CTM Drop 前置 canceled)

  // 分支 2: share=false → 直释 (cc:347-350)
  let cfg = Config {
    timeout_in_sec: 0,
    share_memory_on_teardown: false,
    ..Config::default()
  };
  let mut pool = MemoryPool::<TestRes>::new(cfg);
  pool.add(iso(1), page(0x4000, 4096, 0), &mut host);
  pool.add_zone_reservation(iso(1), TestRes::reserved(0x5000, 512), &mut host);
  pool.release_on_teardown(iso(1));
  assert_eq!(0, pool.get_total_count());
  assert_eq!(0, pool.get_shared_count());

  // ReleaseAllImmediately (cc:363-369): 本地+共享双清
  pool.add(iso(1), page(0x6000, 4096, 0), &mut host);
  pool.add(iso(2), page(0x7000, 4096, 0), &mut host);
  assert_eq!(2, pool.get_total_count());
  pool.release_all_immediately();
  assert_eq!(0, pool.get_total_count());
  pool.tear_down(); // cc:371-377 同分支 1 收口
  info!("release_on_teardown_branches_gold ok");
  OK
}

/// cc:491-512 PostDelayedReleaseTask: epoch `fetch_add` 取旧值 (cc:498-499) +
/// posted_time 落戳 (cc:511); cc:514-529 IfNeeded 的 timeout/`kTimeSlack(4s)` 门槛 —
/// 距上次投递未超 timeout+slack 不补投。
#[test]
fn post_delayed_release_task_epoch_and_slack_gold() -> Void {
  let mut host = TestHost::default();
  let mut pool = MemoryPool::<TestRes>::new(timeout_cfg()); // timeout=8, slack=4

  // cc:524-525: 初投 — posted_time 默认 0 → now > 0+12s 恒成立 (cc:517 kTimeSlack)
  pool.post_delayed_release_task_if_needed(iso(1), &mut host);
  assert_eq!(1, host.posts.len());
  assert_eq!(0, host.posts[0].0.release_epoch); // fetch_add 取旧值
  assert_eq!(1, pool.current_epoch()); // 已 +1
  assert!(pool.posted_time().to_internal_value() > 0); // cc:511 落戳

  // 即刻复检: now <= deadline (posted+12s) → 不补投 (cc:527 条件不成立)
  pool.post_delayed_release_task_if_needed(iso(1), &mut host);
  assert_eq!(1, host.posts.len());

  // 显式投递: 第二任务 epoch=1 (cc:498-499) + 延迟透传
  pool.post_delayed_release_task(iso(1), TimeDelta::from_seconds(3), &mut host);
  assert_eq!(2, host.posts.len());
  assert_eq!(1, host.posts[1].0.release_epoch);
  assert_eq!(2, pool.current_epoch());

  // task_disabled (h:239): Cancel 面置位后 IfNeeded 直返 (cc:519) + 显式投递 DCHECK 面
  // 由调用侧禁入 (cc:494), 此处验证 IfNeeded 分支
  pool.cancel_and_wait_for_task_to_finish_for_testing(); // cc:531-534
  pool.post_delayed_release_task_if_needed(iso(1), &mut host);
  assert_eq!(2, host.posts.len());
  pool.reenable_task_for_testing(); // cc:536
  // 重开后 posted_time 已过 12s? 未过 → 仍不投 (门槛同源复证)
  pool.post_delayed_release_task_if_needed(iso(1), &mut host);
  assert_eq!(2, host.posts.len());
  info!("post_delayed_release_task_epoch_and_slack_gold ok");
  OK
}

/// cc:447-457 任务挂点判据 `config.single_threaded && isolate` (cc:451) 三态。
#[test]
fn task_attaches_to_isolate_matrix_gold() -> Void {
  let c_multi = timeout_cfg();
  let mut c_single = timeout_cfg();
  c_single.single_threaded = true;
  // 多线程配置: 恒挂池 CTM (cc:453 else 分支)
  assert!(!ReleasePooledChunksTask::new(c_multi, iso(1), 0).attaches_to_isolate());
  // 单线程 + isolate: 挂 isolate CTM (cc:451-452, isolate 将先于 task_runner 死亡)
  assert!(ReleasePooledChunksTask::new(c_single, iso(1), 0).attaches_to_isolate());
  // 单线程 + nullptr isolate: 回退池 CTM
  assert!(!ReleasePooledChunksTask::new(c_single, null_mut(), 0).attaches_to_isolate());
  info!("task_attaches_to_isolate_matrix_gold ok");
  OK
}

/// cc:464-483 RunInternal 全链: ReleaseUpTo → trace_gc_nvp 打点 (宿主 print 通道, 台账 5)
/// → 池未空则按 timeout 延迟自续投 (cc:478-482)。
#[test]
fn release_task_run_repost_gold() -> Void {
  let cfg = Config {
    trace_gc_nvp: true,
    ..timeout_cfg()
  };
  let mut host = TestHost::default();
  let mut pool = MemoryPool::<TestRes>::new(cfg);

  // 入池 epoch=5 (条目 epoch 由 Create 侧载, 台账 1; 此处直供 PooledPage)
  pool.add(iso(1), page(0x1000, 4096, 5), &mut host);
  let baseline = host.posts.len(); // add → IfNeeded 首投 1 次
  assert_eq!(1, baseline);
  assert_eq!(0, host.posts[0].0.release_epoch); // cc:498-499 fetch_add 取旧值
  assert_eq!(1, pool.current_epoch());

  // run(release_epoch=5): 5<=5 冲净 → emptied → 打点 + 不续投 (cc:478-482)
  let task = ReleasePooledChunksTask::new(cfg, iso(1), 5);
  task.run(&mut pool, &mut host);
  assert_eq!(baseline, host.posts.len()); // 未续投
  assert_eq!(1, host.printed.len());
  assert_eq!(
    "Memory pool: Removed pages: 1, removed large pages: 0, removed zone reservations: 0\n",
    host.printed[0]
  );
  assert_eq!(0, pool.get_total_count());

  // 未冲净: epoch 高于界 → 续投一次, 延迟 = FromSeconds(timeout) (cc:481);
  // 本轮 add 的 IfNeeded 因 posted_time+12s 门槛未到而不投 (cc:524-525)
  pool.add(iso(1), page(0x2000, 4096, 9), &mut host);
  assert_eq!(baseline, host.posts.len()); // 门槛未到, add 未追加投递
  let task = ReleasePooledChunksTask::new(cfg, iso(1), 8);
  task.run(&mut pool, &mut host);
  assert_eq!(baseline + 1, host.posts.len()); // 自续投 (cc:478-481)
  assert_eq!(1, host.posts[1].0.release_epoch); // 续投任务 epoch = fetch_add 旧值 1
  assert_eq!(2, pool.current_epoch());
  assert_eq!(8, host.posts[1].1.in_seconds()); // cc:481 FromSeconds(timeout_in_sec)
  assert_eq!(
    "Memory pool: Removed pages: 0, removed large pages: 0, removed zone reservations: 0\n",
    host.printed[1]
  );
  // run 只冲 release_epoch=8 以下 → epoch 9 条目残留 (cc:209 界判据)
  assert_eq!(1, pool.get_total_count());
  // Drop DCHECK (h:167-168) 前置: 三池冲净后落释
  pool.release_all_immediately(); // cc:363-369
  assert_eq!(0, pool.get_total_count());
  pool.tear_down(); // cc:371-377 取消心跳 + 三池拆除 (CTM Drop 前置 canceled)
  info!("release_task_run_repost_gold ok");
  OK
}
