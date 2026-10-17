//! 多线程分配/释放压力测试 (round3 审查任务 3)。
//!
//! 覆盖面: RegionAllocator (经 BoundedPageAllocator) 与 EmulatedVirtualAddressSubspace
//! 的并发分配释放。固定 seed: 每线程 fastrand 显式播种 (差分确定性约定);
//! 断言全部为交错不变量, 不依赖线程序:
//! 1. 无重叠: 同一地址区间不会被并发签发两次 (共享区间账本逐次校验);
//! 2. 无双重释放: free 的地址必为本线程先前成功分配的地址;
//! 3. 完整归还: 全部释放后 free_size 精确还原 / mock 账本清空。
//!
//! 注: BoundedPageAllocator 未自动获得 Sync (PageAllocator trait 无 Send+Sync 上界),
//! 原版为线程安全 (内部 mutex_, bounded-page-allocator.h:31), 测试以 SharedAllocator
//! 包装显式承担该 SAFETY 契约。

use std::{
  collections::BTreeMap,
  sync::{Barrier, Mutex},
  thread,
};

use aok::{OK, Void};
use fastrand::Rng;
use v8_base::{
  bounded_page_allocator::{BoundedPageAllocator, PageFreeingMode, PageInitializationMode},
  emulated_virtual_address_subspace::EmulatedVirtualAddressSubspace,
  page_allocator::{PageAllocator, Permission},
  virtual_address_space::{NO_HINT, PagePermissions, VirtualAddressSpace},
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 每线程独立播种 (固定 seed 基 + 线程号派生, 序列可复现)
const SEED_BASE: u64 = 0xA11C_E570_5E5A_0001;
const THREADS: usize = 8;

/// 共享区间账本: insert 校验无重叠, remove 校验精确匹配
struct AddrLedger(Mutex<BTreeMap<usize, usize>>);

impl AddrLedger {
  fn new() -> Self {
    Self(Mutex::new(BTreeMap::new()))
  }

  /// 记录一笔分配; 与既有区间重叠即测试失败
  fn insert(&self, address: usize, size: usize) {
    let mut map = self.0.lock().expect("ledger mutex 不应中毒");
    if let Some((&begin, &len)) = map.range(..address).next_back() {
      assert!(
        begin + len <= address,
        "区间重叠: [{begin:#x},{:#x}) vs [{address:#x},{:#x})",
        begin + len,
        address + size
      );
    }
    if let Some((&begin, _)) = map.range(address..).next() {
      assert!(
        begin >= address + size,
        "区间重叠: [{address:#x},{:#x}) vs 后继 {begin:#x}",
        address + size
      );
    }
    assert!(map.insert(address, size).is_none(), "双重分配 {address:#x}");
  }

  /// 记录一笔释放; 地址必须是先前成功分配的区间
  fn remove(&self, address: usize, size: usize) {
    let mut map = self.0.lock().expect("ledger mutex 不应中毒");
    assert_eq!(
      map.remove(&address),
      Some(size),
      "释放了未分配/账目不符的区间 {address:#x}"
    );
  }
}

/// BPA 并发压力: 8 线程 × 4000 次混合分配/释放 (16MB 空间, 4K/8K 对齐)
#[test]
fn bounded_page_allocator_concurrent_stress() -> Void {
  const START: usize = 0x0000_1000_0000_0000;
  const SIZE: usize = 1 << 24;
  const PAGE: usize = 4096;
  const OPS: usize = 4000;

  let mock = MockBacking;
  let allocator = BoundedPageAllocator::new(
    &mock,
    START,
    SIZE,
    PAGE,
    PageInitializationMode::AllocatedPagesCanBeUninitialized,
    PageFreeingMode::MakeInaccessible,
  )
  .expect("BPA 参数合法");
  let shared = SharedAllocator(&allocator);
  let ledger = AddrLedger::new();
  let barrier = Barrier::new(THREADS);
  let (ledger_ref, barrier_ref, shared_ref) = (&ledger, &barrier, &shared);

  thread::scope(|scope| {
    for tid in 0..THREADS {
      scope.spawn(move || {
        let mut rng = Rng::new();
        rng.seed(SEED_BASE + tid as u64 * 0x9E37_79B9);
        let mut mine: Vec<(usize, usize)> = Vec::new();
        barrier_ref.wait();
        for i in 0..OPS {
          let do_alloc = mine.len() < 64 && (rng.bool() || mine.is_empty());
          if do_alloc {
            let size = PAGE * (1 + rng.u64(..16) as usize);
            let alignment = PAGE << (rng.u64(..2) as usize);
            if let Some(addr) =
              shared_ref
                .0
                .allocate_pages(0, size, alignment, Permission::ReadWrite)
            {
              ledger_ref.insert(addr, size);
              mine.push((addr, size));
            }
          } else if let Some((addr, size)) = mine.pop() {
            ledger_ref.remove(addr, size);
            shared_ref.0.free_pages(addr, size);
          }
          if i % 512 == 0 {
            // 统计面: 空闲量不超总空间 (防 free_size 漂移)
            assert!(shared_ref.0.record_stats().free_size <= SIZE);
          }
        }
        // 尾扫: 归还全部剩余
        for (addr, size) in mine {
          ledger_ref.remove(addr, size);
          shared_ref.0.free_pages(addr, size);
        }
      });
    }
  });
  // 完整归还: free_size 还原为全区
  let stats = allocator.record_stats();
  assert_eq!(stats.free_size, SIZE, "全部释放后 free_size 必须还原");
  assert_eq!(stats.largest_free_region, SIZE);
  OK
}

/// EmulatedVirtualAddressSubspace 并发压力: 8 线程 × 2000 次分配/释放
/// (64MB 背书 + 256MB 模拟空间; 未映射区分配走 mock 父空间 hint 路径)
#[test]
fn emulated_subspace_concurrent_stress() -> Void {
  const BASE: usize = 0x0000_7000_0000_0000;
  const MAPPED: usize = 1 << 26;
  const TOTAL: usize = 1 << 28;
  const PAGE: usize = 4096;
  const OPS: usize = 2000;

  let parent = MockVirtualAddressSpace::new(BASE - MAPPED, 1 << 30, PAGE);
  // 背书映射由调用方先在父空间预留 (原版构造注释 "takes ownership of the page
  // allocation (if any)"), 析构时归还 → mock 账本先登记
  parent.pre_register(BASE, MAPPED);
  let subspace = unsafe {
    // SAFETY: parent 活过 subspace 整个生命周期 (同作用域, subspace 先 Drop)
    EmulatedVirtualAddressSubspace::new(&parent, BASE, MAPPED, TOTAL)
  }
  .expect("模拟子空间参数合法");
  subspace.set_random_seed(0x5EED_5EED_5EED_5EED);

  let barrier = Barrier::new(THREADS);
  let subspace_ref = &subspace;
  let barrier_ref = &barrier;
  thread::scope(|scope| {
    for tid in 0..THREADS {
      scope.spawn(move || {
        let mut rng = Rng::new();
        rng.seed(SEED_BASE + 0x77 + tid as u64);
        let mut mine: Vec<(usize, usize)> = Vec::new();
        barrier_ref.wait();
        for _ in 0..OPS {
          let do_alloc = mine.len() < 32 && (rng.bool() || mine.is_empty());
          if do_alloc {
            // 未映射区分配上限: unmapped_size / 2 (96MB), 取小值以保高成功率
            let size = PAGE * (1 + rng.u64(..128) as usize);
            let alignment = PAGE << (rng.u64(..3) as usize);
            if let Some(addr) =
              subspace_ref.allocate_pages(NO_HINT, size, alignment, PagePermissions::READ_WRITE)
            {
              // 签发地址必须落在模拟空间且与本线程已持区间无重叠
              assert!(addr >= BASE && addr + size <= BASE + TOTAL);
              for &(a, s) in mine.iter() {
                assert!(
                  addr + size <= a || a + s <= addr,
                  "子空间重复签发: {addr:#x} vs {a:#x}"
                );
              }
              mine.push((addr, size));
            }
          } else if let Some(&(addr, size)) = mine.last() {
            subspace_ref.free_pages(addr, size);
            mine.pop();
          }
        }
        for (addr, size) in mine {
          subspace_ref.free_pages(addr, size);
        }
      });
    }
  });
  // 背书区无遗留分配, 未映射区无遗留映射 (Drop 后由 mock 账本验证)
  drop(subspace);
  assert_eq!(parent.live_count(), 0, "父空间存在遗留映射 (泄漏)");
  OK
}

// ---------------------------------------------------------------------------
// Mock: 逻辑页分配器 (不触真实 mmap)
// ---------------------------------------------------------------------------

struct MockBacking;

impl PageAllocator for MockBacking {
  fn allocate_page_size(&self) -> usize {
    4096
  }
  fn commit_page_size(&self) -> usize {
    4096
  }
  fn set_random_mmap_seed(&self, _seed: i64) {}
  fn get_random_mmap_addr(&self) -> usize {
    0
  }
  fn allocate_pages(
    &self,
    _hint: usize,
    _length: usize,
    _alignment: usize,
    _permissions: Permission,
  ) -> Option<usize> {
    None
  }
  fn free_pages(&self, _address: usize, _length: usize) -> bool {
    true
  }
  fn release_pages(&self, _address: usize, _length: usize, _new_length: usize) -> bool {
    true
  }
  fn set_permissions(&self, _address: usize, _length: usize, _permissions: Permission) -> bool {
    true
  }
  fn recommit_pages(&self, _address: usize, _length: usize, _permissions: Permission) -> bool {
    true
  }
  fn discard_system_pages(&self, _address: usize, _size: usize) -> bool {
    true
  }
  fn decommit_pages(&self, _address: usize, _size: usize) -> bool {
    true
  }
  fn seal_pages(&self, _address: usize, _length: usize) -> bool {
    false
  }
}

/// BPA 的线程安全共享包装
/// SAFETY: BPA 全部可变状态在内部 Mutex 内串行化 (原版同款线程安全),
/// 共享 &BPA 跨线程调用其 pub API 与原版多线程用法一致
struct SharedAllocator<'a>(&'a BoundedPageAllocator<'a>);
unsafe impl Send for SharedAllocator<'_> {}
unsafe impl Sync for SharedAllocator<'_> {}

/// 逻辑父空间: 真实账本语义 (hint 起线性探测空档, 无空档返回 None)
struct MockVirtualAddressSpace {
  base: usize,
  size: usize,
  page_size: usize,
  live: Mutex<BTreeMap<usize, usize>>,
}

impl MockVirtualAddressSpace {
  fn new(base: usize, size: usize, page_size: usize) -> Self {
    Self {
      base,
      size,
      page_size,
      live: Mutex::new(BTreeMap::new()),
    }
  }

  fn live_count(&self) -> usize {
    self.live.lock().expect("mock mutex 不应中毒").len()
  }

  /// 预登记区间 (调用方已在父空间预留的背书映射, 模拟 mmap 返回)
  fn pre_register(&self, address: usize, size: usize) {
    let old = self
      .live
      .lock()
      .expect("mock mutex 不应中毒")
      .insert(address, size);
    assert!(old.is_none(), "预登记区间已被占用 {address:#x}: {old:?}");
  }
}

impl VirtualAddressSpace for MockVirtualAddressSpace {
  fn page_size(&self) -> usize {
    self.page_size
  }
  fn allocation_granularity(&self) -> usize {
    self.page_size
  }
  fn base(&self) -> usize {
    self.base
  }
  fn size(&self) -> usize {
    self.size
  }
  fn max_page_permissions(&self) -> PagePermissions {
    PagePermissions::READ_WRITE_EXECUTE
  }
  fn set_random_seed(&self, _seed: i64) {}
  fn random_page_address(&self) -> usize {
    self.base
  }
  fn allocate_pages(
    &self,
    hint: usize,
    size: usize,
    alignment: usize,
    _permissions: PagePermissions,
  ) -> Option<usize> {
    let mut live = self.live.lock().expect("mock mutex 不应中毒");
    // 自 hint 起线性探测 (最多 256 档), 找无重叠空档
    let start = if hint >= self.base { hint } else { self.base };
    let mut address = (start / alignment) * alignment;
    for _ in 0..256 {
      let end = address.checked_add(size)?;
      if end <= self.base + self.size {
        let overlap = live
          .range(..address)
          .next_back()
          .is_some_and(|(&b, &l)| b + l > address)
          || live.range(address..).next().is_some_and(|(&b, &_)| b < end);
        if !overlap {
          live.insert(address, size);
          return Some(address);
        }
      }
      address += alignment;
    }
    None
  }
  fn free_pages(&self, address: usize, size: usize) {
    let mut live = self.live.lock().expect("mock mutex 不应中毒");
    assert_eq!(
      live.remove(&address),
      Some(size),
      "父空间双重释放 {address:#x}"
    );
  }
  fn set_page_permissions(
    &self,
    address: usize,
    size: usize,
    _permissions: PagePermissions,
  ) -> bool {
    let live = self.live.lock().expect("mock mutex 不应中毒");
    live.get(&address) == Some(&size)
  }
  fn allocate_guard_region(&self, address: usize, size: usize) -> bool {
    self
      .live
      .lock()
      .expect("mock mutex 不应中毒")
      .insert(address, size)
      .is_none()
  }
  fn free_guard_region(&self, address: usize, size: usize) {
    let mut live = self.live.lock().expect("mock mutex 不应中毒");
    assert_eq!(live.remove(&address), Some(size));
  }
  fn can_allocate_subspaces(&self) -> bool {
    false
  }
  fn allocate_subspace(
    &self,
    _hint: usize,
    _size: usize,
    _alignment: usize,
    _max_page_permissions: PagePermissions,
  ) -> Option<Box<dyn VirtualAddressSpace>> {
    None
  }
  fn recommit_pages(&self, address: usize, size: usize, _permissions: PagePermissions) -> bool {
    let live = self.live.lock().expect("mock mutex 不应中毒");
    live.get(&address) == Some(&size)
  }
  fn discard_system_pages(&self, _address: usize, _size: usize) -> bool {
    true
  }
  fn decommit_pages(&self, _address: usize, _size: usize) -> bool {
    true
  }
  fn free_subspace(&self, _base: usize, _size: usize) {}
}
