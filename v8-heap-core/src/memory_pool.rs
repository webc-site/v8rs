//! 内存池 (MemoryPool) 与池化页 (PooledPage)。
//! 对应原版 v8/src/heap/memory-pool.h (259 行) + memory-pool.cc (538 行)。
//!
//! 目标配置 (与 tool/src/heap/memory-pool.*.js clang -E 展开侧同源):
//!   V8_COMPRESS_POINTERS + V8_ENABLE_SANDBOX + V8_EXTERNAL_CODE_SPACE, DEBUG/TSAN 关。
//!   展开侧实测: DCHECK/CHECK 宏全部剥除 (源码 28 DCHECK + 1 CHECK → 展开 0 残留),
//!   V8_NOEXCEPT → noexcept, V8_EXPORT_PRIVATE → 0 残留 (声明记号消失, 访问面以 pub 承接)。
//!
//! 票面实测差异 (原版 259+538 行内无 kMaxExtentSize / RecommendExtentSize / Allocate /
//! Release 族 — v8/src 全树 grep 0 命中, 属旧版叙事)。现版 memory-pool 为
//!   epoch 版本化池: PooledPage (池化条目) + PoolImpl<E> (本地池 map + 共享池 vec) +
//!   LargePagePoolImpl (best-fit 大页池) + PooledVirtualMemory (zone 预留) +
//!   ReleaseUpTo(release_epoch) 心跳回收 (延迟任务, [timeout, 2*timeout) 秒内可回收)。
//!
//! 消费的真实耦合面 (本仓实测已落地, 直接复用):
//! - [`v8_common::DiscardSealedMemoryScope`] ← cc:81 (释还 RW-X 密封窗口)。
//! - [`v8_common::AddressRegion`] ← h:35 `base::AddressRegion`。
//! - [`v8_base_platform::Mutex`] / [`v8_base_platform::TimeDelta`] / `TimeTicks` ← h:189/209/251。
//! - [`v8_base_platform::free`] ← cc:84 `free(uninitialized_metadata_)`。
//! - [`v8_tasks::CancelableTaskManager`] ← h:252/cc:342/373/532 (池自持 CTM)。
//!
//! 未落地耦合以窄 trait 承接 (推迟双行号见模块尾台账): [`Reservation`] (原版 VirtualMemory)、
//! [`ReleaseTaskHost`] (原版 Isolate task_runner / Platform worker 投递 + IsolateGroup 打点选点)。

use std::{
  collections::HashMap,
  ffi::c_void,
  mem::size_of,
  ptr::null_mut,
  sync::atomic::{AtomicBool, AtomicI64, AtomicUsize, Ordering},
};

use v8_base::{address_region::AddressRegion, logging::check_with_msg};
use v8_base_platform::{Mutex, TimeDelta, TimeTicks, free};
use v8_common::{Address, DiscardSealedMemoryScope, MB};
use v8_tasks::CancelableTaskManager;

/// 原版 h:31 `using Epoch = size_t` (PooledPage 内) + h:75 `using Epoch = PooledPage::Epoch`
/// (MemoryPool 内别名) — 内存释放的世代号。
pub type Epoch = usize;

/// 原版 h:16 `class Isolate;` 前向声明的 opaque 承接 (实体 src/execution/isolate.h 属他票)。
/// C++ 侧 memory-pool 只把 isolate 当指针透传与 map 键 → 空 enum 承接 (同 memory_chunk.rs:26
/// BasePage 先例); zone 预留允许 nullptr (h:112-115 注释)。
pub enum Isolate {}

/// 原版 utils/allocation.h:193-306 `class VirtualMemory final` 的窄承接面。
///
/// 实测本仓 VirtualMemory 未落地 (推迟登记见 v8-utils/src/allocation.rs:5 与
/// v8-zone/src/zone/accounting.rs:5), 而其析构/系统预留属 PageAllocator 平台票;
/// memory-pool 只消费下列 5 面, 故以 trait 承接 — 真实 VirtualMemory 落地后直接作
/// [`MemoryPool<R>`] 的类型参数注入, 本模块逻辑零改动。
/// 原版 `address()` (allocation.h:249) 仅在 Create 族的 DCHECK 里消费 → 不入本 trait (台账 1)。
pub trait Reservation {
  /// 原版 allocation.h:236 `bool IsReserved() const { return region_.begin() != kNullAddress; }`。
  fn is_reserved(&self) -> bool;

  /// 原版 allocation.h:263 `size_t size() const { return region_.size(); }`。
  fn size(&self) -> usize;

  /// 原版 allocation.h:243 `const base::AddressRegion& region() const`。
  fn region(&self) -> AddressRegion;

  /// 原版 allocation.h:293 `V8_EXPORT_PRIVATE void free()` — 归还预留 (释还后仍需 Reset)。
  fn free(&mut self);

  /// 原版 allocation.h:239 `V8_EXPORT_PRIVATE void Reset()` — 放弃所有权但不归还系统内存。
  fn reset(&mut self);
}

/// 原版 h:33-36 `struct PooledPage::Result` — 移交给调用方去初始化 chunk 与 metadata 的裸内存对。
#[derive(Clone, Copy, Debug)]
pub struct PooledPageResult {
  /// h:34 `void* uninitialized_metadata` — 未初始化的 metadata 裸块。
  pub uninitialized_metadata: Address,
  /// h:35 `base::AddressRegion uninitialized_chunk` — 未初始化的 chunk 预留区间。
  pub uninitialized_chunk: AddressRegion,
}

/// 原版 h:161-190 模板形参 `typename PoolEntry` 的约束面: 池只读 entry 的 epoch (cc:209)。
pub trait PooledEntry {
  /// h:46 / h:220 `Epoch epoch() const`。
  fn epoch(&self) -> Epoch;
}

/// 原版 h:28-67 `class PooledPage final` + cc:20-95。
///
/// 把 metadata 对象转成池化条目: 接管 MemoryChunk 与 metadata 的内存预留 (h:22-27 注释),
/// 目的是让析构与释还时点显式化。原版 h:40-41 move ctor/assign = default、h:42-43 copy = delete
/// → Rust 移动语义天然承接, 不派生 Clone (台账 8)。
#[derive(Debug)]
pub struct PooledPage<R: Reservation> {
  /// h:63 `void* uninitialized_metadata_` — cc:84 由 `free` 归还 (置空即放弃所有权)。
  pub uninitialized_metadata: *mut c_void,
  /// h:64-65 `VirtualMemory reservation_` — 原 MemoryChunk 的预留 (h:64 注释)。
  pub reservation: R,
  /// h:66 `Epoch epoch_`。
  pub epoch: Epoch,
}

impl<R: Reservation> PooledPage<R> {
  /// 原版 h:60-61 私有 ctor 声明 + cc:20-24 实体 (三个字段按声明序搬移)。
  ///
  /// 这是 h:57-58 `Create(NormalPage*)/Create(LargePage*)` 的落点承接位: Create 需
  /// metadata->reserved_memory() 搬移 + `chunk->~MemoryChunk()`/`metadata->~NormalPage()`
  /// placement 析构 (cc:49-50), 页面对象属他票 → 台账 1。
  pub fn new(uninitialized_metadata: *mut c_void, reservation: R, epoch: Epoch) -> Self {
    Self {
      uninitialized_metadata,
      reservation,
      epoch,
    }
  }

  /// 原版 h:45 `size_t size() const { return reservation_.size(); }`。
  pub fn size(&self) -> usize {
    self.reservation.size()
  }

  /// 原版 h:46 `size_t epoch() const { return epoch_; }`。
  pub fn epoch(&self) -> Epoch {
    self.epoch
  }

  /// 原版 h:50 声明 + cc:87-95 `ToResult()`: 把所有权交给调用方, 自身复位为"空"。
  ///
  /// 逐行对应: cc:88 region → cc:89 metadata 取副本 → cc:92 `reservation_.Reset()`
  /// (复位但不归还, 内存已归调用方) → cc:93 metadata 置空 → cc:94 返回二元组。
  pub fn to_result(&mut self) -> PooledPageResult {
    let uninitialized_chunk = self.reservation.region();
    let uninitialized_metadata = self.uninitialized_metadata as Address;
    // 复位但不归还内存 — 该内存现由 Release 的调用方持有 (cc:90-91 注释)。
    self.reservation.reset();
    self.uninitialized_metadata = null_mut();
    PooledPageResult {
      uninitialized_metadata,
      uninitialized_chunk,
    }
  }
}

/// 原版 cc:76-85 `~PooledPage()`: 预留仍在时 (未被 ToResult 搬走) 归还系统内存并 free metadata。
impl<R: Reservation> Drop for PooledPage<R> {
  fn drop(&mut self) {
    if !self.reservation.is_reserved() {
      return;
    }
    {
      let _scope = DiscardSealedMemoryScope::new("Deleting MemoryChunk reservation");
      self.reservation.free();
    }
    // SAFETY: cc:84 `free(uninitialized_metadata_)` — 指针由 Create 侧从 malloc 系 metadata
    // 交出; null 由分配器语义兜住 (且 ToResult 路径已在 cc:77-79 提前返回)。
    unsafe { free(self.uninitialized_metadata) };
  }
}

impl<R: Reservation> PooledEntry for PooledPage<R> {
  fn epoch(&self) -> Epoch {
    PooledPage::epoch(self)
  }
}

/// 原版 h:214-226 `class MemoryPool::PooledVirtualMemory final` (无 .cc 实体, 全内联)。
/// zone 预留的池化包装 (h:112-115: 可不关联 isolate)。
#[derive(Debug)]
pub struct PooledVirtualMemory<R> {
  /// h:224 `VirtualMemory memory_`。
  pub memory: R,
  /// h:225 `Epoch epoch_`。
  pub epoch: Epoch,
}

impl<R: Reservation> PooledVirtualMemory<R> {
  /// 原版 h:216-217 ctor 初始化列表 (memory_ 搬移 + epoch_)。
  pub fn new(memory: R, epoch: Epoch) -> Self {
    Self { memory, epoch }
  }

  /// 原版 h:219 `VirtualMemory& virtual_memory()`。
  pub fn virtual_memory(&mut self) -> &mut R {
    &mut self.memory
  }

  /// 原版 h:220 `Epoch epoch() const`。
  pub fn epoch(&self) -> Epoch {
    self.epoch
  }

  /// 原版 h:221 `size_t size() const { return memory_.size(); }`。
  pub fn size(&self) -> usize {
    self.memory.size()
  }

  /// cc:441 `std::move(result->virtual_memory())` 的搬出面 (包装体随即销毁)。
  pub fn into_virtual_memory(self) -> R {
    self.memory
  }
}

impl<R: Reservation> PooledEntry for PooledVirtualMemory<R> {
  fn epoch(&self) -> Epoch {
    PooledVirtualMemory::epoch(self)
  }
}

/// 原版 h:81-87 `struct MemoryPool::Config final`。
///
/// h:78-80 注释: 存旗标副本而非直读 v8_flags, 避免竞态 (尤其测试里的 FlagScope)。
/// 默认值逐项 = h:82-86 类内初始化。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Config {
  /// h:82 `bool single_threaded = false` — 心跳任务改挂 isolate 的 CTM。
  pub single_threaded: bool,
  /// h:83 `bool share_memory_on_teardown = true` — isolate 拆除时本地池转共享池而非直接释放。
  pub share_memory_on_teardown: bool,
  /// h:84 `bool trace_gc_nvp = false` — 心跳回收后打点 (cc:466-477)。
  pub trace_gc_nvp: bool,
  /// h:85 `size_t max_large_page_pool_size = 32` — 大页池上限 (单位 MB, cc:242)。
  pub max_large_page_pool_size: usize,
  /// h:86 `size_t timeout_in_sec = 8` — 释放心跳周期 (0 即关心跳, cc:519)。
  pub timeout_in_sec: usize,
}

impl Default for Config {
  fn default() -> Self {
    Self {
      single_threaded: false,
      share_memory_on_teardown: true,
      trace_gc_nvp: false,
      max_large_page_pool_size: 32,
      timeout_in_sec: 8,
    }
  }
}

/// 原版 h:156-159 `struct MemoryPool::PoolReleaseStats`。
/// Default 承接 h:157-158 类内初始化 `= 0 / = false`。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PoolReleaseStats {
  /// h:157 `size_t removed_entries = 0`。
  pub removed_entries: usize,
  /// h:158 `bool pool_emptied = false`。
  pub pool_emptied: bool,
}

/// 原版 h:228-233 `struct MemoryPool::ReleaseStats final` (三池汇总)。
/// Default 承接 h:229-232 类内初始化 `= 0 / = false`。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ReleaseStats {
  /// h:229 `size_t pages_removed = 0`。
  pub pages_removed: usize,
  /// h:230 `size_t large_pages_removed = 0`。
  pub large_pages_removed: usize,
  /// h:231 `size_t zone_reservations_removed = 0`。
  pub zone_reservations_removed: usize,
  /// h:232 `bool pool_emptied = false`。
  pub pool_emptied: bool,
}

impl ReleaseStats {
  /// 原版 cc:472-475 trace_gc_nvp 打点文案 (%zu ×3 逐位对应 + 结尾换行)。
  /// 打印通道见 [`ReleaseTaskHost::print`] (isolate 选点推迟, 台账 5)。
  #[must_use]
  pub fn message(&self) -> String {
    format!(
      "Memory pool: Removed pages: {}, removed large pages: {}, removed zone reservations: {}\n",
      self.pages_removed, self.large_pages_removed, self.zone_reservations_removed
    )
  }
}

/// 原版 h:161-190 `template <typename PoolEntry> class MemoryPool::PoolImpl final` + cc:97-234。
///
/// 本地池 (按 isolate 分桶) + 共享池的两级结构; 原版 h:187 `absl::flat_hash_map` →
/// `HashMap` (cc:129 `local_pools_.begin()` 的窃取序在两实现下同为任意序, 台账 9)。
pub struct PoolImpl<E> {
  /// h:186 `const MemoryPool::Config config_`。
  pub config: Config,
  /// h:187 `absl::flat_hash_map<Isolate*, std::vector<PoolEntry>> local_pools_`。
  pub local_pools: HashMap<*mut Isolate, Vec<E>>,
  /// h:188 `std::vector<PoolEntry> shared_pool_`。
  pub shared_pool: Vec<E>,
  /// h:189 `mutable base::Mutex mutex_` — 可变语义由 Mutex 内部承接。
  mutex: Mutex,
}

impl<E> PoolImpl<E> {
  /// 原版 h:164 `explicit PoolImpl(MemoryPool::Config config) : config_(config)`。
  pub fn new(config: Config) -> Self {
    Self {
      config,
      local_pools: HashMap::new(),
      shared_pool: Vec::new(),
      mutex: Mutex::new(),
    }
  }

  /// 原版 cc:97-103 `TearDown()`: 删 "nullptr-isolate" 桶 → DCHECK 本地池全空 → 清共享池。
  /// 实测原版此处不加锁 (cc:98-103 无 MutexGuard, 前置条件为心跳任务已全部取消 cc:372-373)。
  pub fn tear_down(&mut self) {
    self.local_pools.remove(&null_mut());
    debug_assert!(
      self.local_pools.is_empty(),
      "cc:101 DCHECK(local_pools_.empty())"
    );
    self.shared_pool.clear();
  }

  /// 原版 cc:105-110 `PutLocal(Isolate*, PoolEntry)`: 建桶并尾插。
  pub fn put_local(&mut self, isolate: *mut Isolate, entry: E) {
    let _guard = self.mutex.lock_guard();
    self.local_pools.entry(isolate).or_default().push(entry);
  }

  /// 原版 cc:112-144 `Get(Isolate*)`: 优先级 = 本 isolate 本地池 → 共享池尾 → 窃取任一本地池
  /// (h:100-103 注释)。
  pub fn get(&mut self, isolate: *mut Isolate) -> Option<E> {
    let _guard = self.mutex.lock_guard();
    if self.local_pools.contains_key(&isolate) {
      return Self::pop_local(&mut self.local_pools, isolate);
    }
    // cc:120-126 该 isolate 无本地桶: 共享池即将冲刷, 先取尾部。
    if let Some(entry) = self.shared_pool.pop() {
      return Some(entry);
    }
    // cc:128-129 否则从某个其他 isolate 的本地池窃取。
    match self.local_pools.keys().next().copied() {
      // cc:132-141
      Some(key) => Self::pop_local(&mut self.local_pools, key),
      // cc:143 return std::nullopt
      None => None,
    }
  }

  /// 原版 cc:132-141: 桶尾弹出, 桶空即整键移除 (cc:137-139)。
  /// 不变式: `PutLocal` 建的键恒非空 → cc:134 `DCHECK(!entries.empty())`。
  /// 自由函数形态承接: 只借 `local_pools` 字段, 与 `mutex` 字段锁槽借用互不冲突。
  fn pop_local(
    local_pools: &mut HashMap<*mut Isolate, Vec<E>>,
    isolate: *mut Isolate,
  ) -> Option<E> {
    let entries = local_pools.get_mut(&isolate)?;
    debug_assert!(!entries.is_empty(), "cc:134 DCHECK(!entries.empty())");
    let entry = entries.pop();
    if !entries.is_empty() {
      return entry;
    }
    local_pools.remove(&isolate);
    entry
  }

  /// 原版 cc:146-158 `MoveLocalToShared(Isolate*)`: 整桶搬入共享池 (cc:152-154
  /// make_move_iterator 全量搬移 → Vec::extend 按元素移动), 返回搬移后共享池是否非空。
  pub fn move_local_to_shared(&mut self, isolate: *mut Isolate) -> bool {
    let _guard = self.mutex.lock_guard();
    if let Some(entries) = self.local_pools.remove(&isolate) {
      debug_assert!(!entries.is_empty(), "cc:151 DCHECK(!it->second.empty())");
      self.shared_pool.extend(entries);
    }
    !self.shared_pool.is_empty()
  }

  /// 原版 cc:160-164 `ReleaseShared()`。
  pub fn release_shared(&mut self) {
    let _guard = self.mutex.lock_guard();
    self.shared_pool.clear();
  }

  /// 原版 cc:166-170 `ReleaseLocal()` (无参重载: 全部本地池)。
  pub fn release_local(&mut self) {
    let _guard = self.mutex.lock_guard();
    self.local_pools.clear();
  }

  /// 原版 cc:172-176 `ReleaseLocal(Isolate*)` (指定 isolate 重载)。
  pub fn release_local_of(&mut self, isolate: *mut Isolate) {
    let _guard = self.mutex.lock_guard();
    self.local_pools.remove(&isolate);
  }

  /// 原版 cc:178-187 `Size() const`: 所有本地池 + 共享池条目数。
  pub fn size(&self) -> usize {
    let _guard = self.mutex.lock_guard();
    self
      .local_pools
      .values()
      .map(|entries| entries.len())
      .sum::<usize>()
      + self.shared_pool.len()
  }

  /// 原版 cc:189-194 `LocalSize(Isolate*) const` (cc:193 无桶返 0)。
  pub fn local_size(&self, isolate: *mut Isolate) -> usize {
    let _guard = self.mutex.lock_guard();
    self
      .local_pools
      .get(&isolate)
      .map_or(0, |entries| entries.len())
  }

  /// 原版 cc:196-200 `SharedSize() const`。
  pub fn shared_size(&self) -> usize {
    let _guard = self.mutex.lock_guard();
    self.shared_pool.len()
  }

  /// 原版 cc:202-234 `ReleaseUpTo(Epoch)`: 凡 `entry.epoch() <= release_epoch` (cc:209) 者
  /// 搬出后在锁外释放 (cc:232 注释 "Entries will be freed automatically here")。
  pub fn release_up_to(&mut self, release_epoch: Epoch) -> PoolReleaseStats
  where
    E: PooledEntry,
  {
    let mut entries_to_free: Vec<E> = Vec::new();
    let mut pool_emptied = false;
    {
      let _guard = self.mutex.lock_guard();
      // cc:217-218 冲刷共享池 (近期拆除 isolate 转入的页)。
      let mut idx = 0;
      while idx < self.shared_pool.len() {
        if self.shared_pool[idx].epoch() <= release_epoch {
          entries_to_free.push(self.shared_pool.remove(idx));
        } else {
          idx += 1;
        }
      }
      // cc:219-227 冲刷各本地池; 原版只把非空池回填新 map 再 swap → retain 语义等价 (空键即删)。
      self.local_pools.retain(|_isolate, entries| {
        let mut idx = 0;
        while idx < entries.len() {
          if entries[idx].epoch() <= release_epoch {
            entries_to_free.push(entries.remove(idx));
          } else {
            idx += 1;
          }
        }
        !entries.is_empty()
      });
      if self.local_pools.is_empty() && self.shared_pool.is_empty() {
        pool_emptied = true;
      }
    }
    // cc:233 return {entries_to_free.size(), pool_emptied}; entries_to_free 出作用域即释放内存。
    PoolReleaseStats {
      removed_entries: entries_to_free.len(),
      pool_emptied,
    }
  }
}

/// 原版 h:166-169 `~PoolImpl()`: DCHECK 本地池与共享池均空 (拆除须经 TearDown, cc:371-377)。
impl<E> Drop for PoolImpl<E> {
  fn drop(&mut self) {
    debug_assert!(
      self.local_pools.is_empty(),
      "h:167 DCHECK(local_pools_.empty())"
    );
    debug_assert!(
      self.shared_pool.is_empty(),
      "h:168 DCHECK(shared_pool_.empty())"
    );
  }
}

/// 原版 h:192-212 `class MemoryPool::LargePagePoolImpl final` + cc:236-330。
/// 大页池为单一堆 (无 isolate 分桶), 取还按 best-fit。
pub struct LargePagePoolImpl<R: Reservation> {
  /// h:208 `const MemoryPool::Config config_`。
  pub config: Config,
  /// h:210 `std::vector<PooledPage> pages_`。
  pub pages: Vec<PooledPage<R>>,
  /// h:211 `size_t total_size_ = 0` — pages_ 尺寸的增量维护量 ( DCHECK 恒等 ComputeTotalSize)。
  total_size: usize,
  /// h:209 `base::Mutex mutex_`。
  mutex: Mutex,
}

impl<R: Reservation> LargePagePoolImpl<R> {
  /// 原版 h:194 `explicit LargePagePoolImpl(MemoryPool::Config config) : config_(config)`。
  pub fn new(config: Config) -> Self {
    Self {
      config,
      pages: Vec::new(),
      total_size: 0,
      mutex: Mutex::new(),
    }
  }

  /// 原版 cc:320-328 `ComputeTotalSize()`: 全量重算 (原版 cc:240/256/264/286/314 用它反证
  /// `total_size_` 增量维护的正确性)。不加锁 — 调用方持锁。
  pub fn compute_total_size(&self) -> usize {
    self.pages.iter().map(|entry| entry.size()).sum()
  }

  /// 原版 cc:236-258 `Add(std::vector<LargePage*>& pages, Epoch epoch)`。
  ///
  /// `pages` 为 `PooledPage::Create(LargePage*, epoch)` 的产物向量 (Create 族推迟, 台账 1);
  /// erase_if 语义守恒: 预算内 (`total_size + size <= max_total_size`, cc:246) 者入池并从
  /// 入参向量移除, 超预算者原地留下由调用方释放; 返回值 = 是否有条目入池 (cc:238/257)。
  pub fn add(&mut self, pages: &mut Vec<PooledPage<R>>) -> bool {
    let _guard = self.mutex.lock_guard();
    debug_assert_eq!(
      self.total_size,
      self.compute_total_size(),
      "cc:240 DCHECK_EQ(total_size_, ComputeTotalSize())"
    );
    let max_total_size = self.config.max_large_page_pool_size * MB;
    let mut added_to_pool = false;
    let mut idx = 0;
    while idx < pages.len() {
      let page_size = pages[idx].size();
      // cc:246-249 超预算 → 保留在入参向量 (return false 即 erase_if 不删)。
      if self.total_size + page_size > max_total_size {
        idx += 1;
        continue;
      }
      // cc:250-253 累加预算 + 入池 + 标记。
      self.total_size += page_size;
      let page = pages.remove(idx);
      self.pages.push(page);
      added_to_pool = true;
    }
    debug_assert_eq!(
      self.total_size,
      self.compute_total_size(),
      "cc:256 DCHECK_EQ(total_size_, ComputeTotalSize())"
    );
    added_to_pool
  }

  /// 原版 cc:260-288 `Remove(Isolate* isolate, size_t chunk_size)`。
  ///
  /// best-fit (cc:266-268 注释): 取 `size >= chunk_size` 中最小者, 同值取先者 (cc:274 严格
  /// 小于比较); 下一轮 full GC 自行裁剪偏大页。`_isolate` 原版形参未消费 (cc:261)。
  pub fn remove(&mut self, _isolate: *mut Isolate, chunk_size: usize) -> Option<PooledPageResult> {
    let _guard = self.mutex.lock_guard();
    debug_assert_eq!(
      self.total_size,
      self.compute_total_size(),
      "cc:264 DCHECK_EQ(total_size_, ComputeTotalSize())"
    );
    // cc:269-277 选页。
    let selected = self
      .pages
      .iter()
      .enumerate()
      .filter(|(_, page)| page.size() >= chunk_size)
      .min_by_key(|(_, page)| page.size())
      .map(|(idx, _)| idx);
    let Some(idx) = selected else {
      // cc:279-281 return std::nullopt
      return None;
    };
    // cc:283-287 搬出 → 减预算 → 删项 → ToResult。
    let mut result = self.pages.remove(idx);
    self.total_size -= result.size();
    debug_assert_eq!(
      self.total_size,
      self.compute_total_size(),
      "cc:286 DCHECK_EQ(total_size_, ComputeTotalSize())"
    );
    Some(result.to_result())
  }

  /// 原版 cc:290-294 `ReleaseAll()`。
  pub fn release_all(&mut self) {
    let _guard = self.mutex.lock_guard();
    self.pages.clear();
    self.total_size = 0;
  }

  /// 原版 cc:296-318 `ReleaseUpTo(Epoch)` (大页版: 同步减 `total_size_`, cc:304)。
  pub fn release_up_to(&mut self, release_epoch: Epoch) -> PoolReleaseStats {
    let mut entries_to_free: Vec<PooledPage<R>> = Vec::new();
    let mut pool_emptied = false;
    {
      let _guard = self.mutex.lock_guard();
      let mut idx = 0;
      while idx < self.pages.len() {
        if self.pages[idx].epoch() <= release_epoch {
          // cc:304-305 先减预算再搬出。
          self.total_size -= self.pages[idx].size();
          entries_to_free.push(self.pages.remove(idx));
        } else {
          idx += 1;
        }
      }
      if self.pages.is_empty() {
        pool_emptied = true;
      }
      debug_assert_eq!(
        self.total_size,
        self.compute_total_size(),
        "cc:314 DCHECK_EQ(total_size_, ComputeTotalSize())"
      );
    }
    // cc:317 return; entries_to_free 出作用域即释放 (cc:316 注释)。
    PoolReleaseStats {
      removed_entries: entries_to_free.len(),
      pool_emptied,
    }
  }

  /// 原版 cc:330 `TearDown()`: `CHECK(pages_.empty())` — 非 DCHECK, 目标配置展开侧保留
  /// 条件实体 (见 memory-pool.cc.js 展开台账), 违例走 v8 fatal 通道。
  pub fn tear_down(&self) {
    check_with_msg(
      self.pages.is_empty(),
      "MemoryPool::LargePagePoolImpl::TearDown pages_.empty",
    );
  }
}

/// 原版 h:196 `~LargePagePoolImpl() { DCHECK(pages_.empty()); }`。
impl<R: Reservation> Drop for LargePagePoolImpl<R> {
  fn drop(&mut self) {
    debug_assert!(self.pages.is_empty(), "h:196 DCHECK(pages_.empty())");
  }
}

/// 原版 h:235-236 延迟释放心跳的两个外部面 (cc:500-510 投递 + cc:466-477 打点) 的窄承接 trait。
///
/// 原版直接取 `Isolate::task_runner()` / `V8::GetCurrentPlatform()` / `IsolateGroup::current()`;
/// 三者均属他票 (Isolate/Platform/IsolateGroup 本仓实测无落地实体) → 由宿主实现, 见台账 4/5。
pub trait ReleaseTaskHost {
  /// 原版 cc:500-510: 单线程且有 isolate → `isolate->task_runner()->PostDelayedTask` (cc:502-504),
  /// 否则 `Platform::PostDelayedTaskOnWorkerThread(TaskPriority::kUserVisible, ...)` (cc:506-509,
  /// 注释: kUserVisible 避免取全局锁时与主线程优先级反转)。
  ///
  /// `pool_ctm` = 池自持 CTM (h:252): cc:451-453 的挂点选择在单线程分支应改用
  /// `isolate->cancelable_task_manager()` (Isolate 他票 → 宿主侧决定)。
  fn post_delayed_release_task(
    &mut self,
    task: &ReleasePooledChunksTask,
    pool_ctm: &CancelableTaskManager,
    delay: TimeDelta,
  );

  /// 原版 cc:469-476: `IsolateGroup::current()->FindAnotherIsolateLocked(nullptr, ...)` 借
  /// *某个* 活着的 isolate 调 `Isolate::PrintWithTimestamp` (cc:467-468 注释: isolate_ 字段可能
  /// 已销毁) → 选点归宿主, 本模块只提供文案 [`ReleaseStats::message`]。
  fn print(&mut self, msg: &str);
}

/// 原版 h:154 前向声明 + cc:445-489 `class MemoryPool::ReleasePooledChunksTask final : public CancelableTask`。
///
/// 差异申报: 原版基类 CancelableTask 的取消注册 (cc:451-453 CTM 挂点) 由宿主用 [`ReleaseTaskHost`]
/// 完成 (台账 4); `pool_` 指针字段 (cc:487) 在 Rust 侧由 [`run`](Self::run) 的借用参数承接,
/// 不落字段 (避免自引用生命周期)。
#[derive(Clone, Copy, Debug)]
pub struct ReleasePooledChunksTask {
  /// cc:485 `const MemoryPool::Config config_`。
  pub config: Config,
  /// cc:486 `Isolate* const isolate_` — 仅单线程配置用于投递 (cc:467-468 注释)。
  pub isolate: *mut Isolate,
  /// cc:488 `const Epoch release_epoch_`。
  pub release_epoch: Epoch,
}

impl ReleasePooledChunksTask {
  /// 原版 cc:447-457 ctor (config_/isolate_/pool_/release_epoch_ 四项 + 基类挂点选择)。
  pub fn new(config: Config, isolate: *mut Isolate, release_epoch: Epoch) -> Self {
    Self {
      config,
      isolate,
      release_epoch,
    }
  }

  /// 原版 cc:451 `config.single_threaded && isolate` — 决定任务挂 isolate CTM 还是池 CTM。
  #[must_use]
  pub fn attaches_to_isolate(&self) -> bool {
    self.config.single_threaded && !self.isolate.is_null()
  }

  /// 原版 cc:464-483 `RunInternal()`: 回收到期条目 → trace 打点 → 池未空则自续下一个心跳。
  pub fn run<R: Reservation, H: ReleaseTaskHost + ?Sized>(
    &self,
    pool: &mut MemoryPool<R>,
    host: &mut H,
  ) {
    // cc:465。
    let stats = pool.release_up_to(self.release_epoch);
    // cc:466-477。
    if self.config.trace_gc_nvp {
      host.print(&stats.message());
    }
    // cc:478-482: 仅当池未完全清空才续投, 延迟 = FromSeconds(timeout_in_sec)。
    if !stats.pool_emptied {
      pool.post_delayed_release_task(
        self.isolate,
        TimeDelta::from_seconds(self.config.timeout_in_sec as i64),
        host,
      );
    }
  }
}

/// 原版 h:254 `static_assert(std::atomic<base::TimeTicks>::is_always_lock_free)`。
/// Rust 无 `atomic<TimeTicks>` 形态, `posted_time_` 以 `AtomicI64` 承载 TimeTicks 的 µs 位值
/// (`to/from_internal_value`), 断言代入该原子类型: 目标可用 64 位原子 + 载体宽度 8。
/// 实测本工具链 std 未提供 `is_always_lock_free` 常量 (sysroot core/sync/atomic.rs grep 0 命中),
/// lock-free 面以 `target_has_atomic = "64"` cfg 承接 (差异登记见推迟台账 11)。
const _: () = assert!(cfg!(target_has_atomic = "64") && size_of::<AtomicI64>() == 8);

/// 原版 h:74-255 `class MemoryPool final` + cc:332-536 — 显式冲刷前一直缓存内存的池。
///
/// h:69-73 注释: 假定内存可自由复用并全局共享 (入池页可立刻被别的 Isolate 取走);
/// 当前用于页与 zone 预留两类。
///
/// 线程模型: 三池各自持锁 (h:189/209), 池级量为原子 (h:239/250/251); 原版 h:254
/// `static_assert(std::atomic<base::TimeTicks>::is_always_lock_free)` → Rust 以
/// `AtomicI64` 承载 TimeTicks 的内部 µs 位值 (`to/from_internal_value`), 断言代入该原子类型。
pub struct MemoryPool<R: Reservation> {
  /// h:238 `const Config config_`。
  pub config: Config,
  /// h:239 `std::atomic<bool> task_disabled_ = false`。
  task_disabled: AtomicBool,
  /// h:241 `PoolImpl<PooledPage> page_pool_`。
  page_pool: PoolImpl<PooledPage<R>>,
  /// h:242 `PoolImpl<PooledVirtualMemory> zone_pool_`。
  zone_pool: PoolImpl<PooledVirtualMemory<R>>,
  /// h:244 `LargePagePoolImpl large_pool_`。
  large_pool: LargePagePoolImpl<R>,
  /// h:250 `std::atomic<Epoch> current_epoch_{0}`。
  current_epoch: AtomicUsize,
  /// h:251 `std::atomic<base::TimeTicks> posted_time_` (µs 位值; 默认构造即 null TimeTicks = 0)。
  posted_time: AtomicI64,
  /// h:252 `std::unique_ptr<CancelableTaskManager> cancellable_task_manager_`。
  cancellable_task_manager: CancelableTaskManager,
}

impl<R: Reservation> MemoryPool<R> {
  // h:254 的 static_assert 代点在模块级 `const _` 断言 (本结构 posted_time 字段上方注释)。

  /// cc:517 `static constexpr base::TimeDelta kTimeSlack = base::TimeDelta::FromSeconds(4)`。
  const K_TIME_SLACK_SECONDS: i64 = 4;

  /// 原版 h:89 声明 + cc:337-342 ctor: config_ + 三池 (同 config 注入) + 池自持 CTM。
  pub fn new(config: Config) -> Self {
    Self {
      config,
      task_disabled: AtomicBool::new(false),
      page_pool: PoolImpl::new(config),
      zone_pool: PoolImpl::new(config),
      large_pool: LargePagePoolImpl::new(config),
      current_epoch: AtomicUsize::new(0),
      posted_time: AtomicI64::new(0),
      cancellable_task_manager: CancelableTaskManager::new(),
    }
  }

  /// 当前世代号 (cc:334/406/420/433 `current_epoch_.load(std::memory_order_relaxed)`)。
  #[must_use]
  pub fn current_epoch(&self) -> Epoch {
    self.current_epoch.load(Ordering::Relaxed)
  }

  /// `posted_time_` 的读取面 (h:251; µs 位值还原的 TimeTicks)。
  #[must_use]
  pub fn posted_time(&self) -> TimeTicks {
    TimeTicks::from_internal_value(self.posted_time.load(Ordering::Relaxed))
  }

  /// 池自持 CTM 的读取面 (h:252; 原版 cc:453 任务挂点)。
  #[must_use]
  pub fn cancellable_task_manager(&self) -> &CancelableTaskManager {
    &self.cancellable_task_manager
  }

  /// 原版 h:99 声明 + cc:401-408 `Add(Isolate*, MutablePage*)`。
  ///
  /// 差异申报 (台账 1): 原版入参为 `MutablePage*`, 经 cc:405 `PooledPage::Create(
  /// static_cast<NormalPage*>(page), epoch)` 转成池化条目 — 页面对象未落地 → 本处参数即该
  /// Create 的产物 (epoch 由 Create 侧载, cc:406)。
  pub fn add<H: ReleaseTaskHost + ?Sized>(
    &mut self,
    isolate: *mut Isolate,
    entry: PooledPage<R>,
    host: &mut H,
  ) {
    debug_assert!(!isolate.is_null(), "cc:402 DCHECK_NOT_NULL(isolate)");
    self.page_pool.put_local(isolate, entry);
    self.post_delayed_release_task_if_needed(isolate, host);
  }

  /// 原版 h:104 声明 + cc:410-417 `Remove(Isolate*)`: 取条目并搬出为 [`PooledPageResult`]。
  pub fn remove(&mut self, isolate: *mut Isolate) -> Option<PooledPageResult> {
    debug_assert!(!isolate.is_null(), "cc:411 DCHECK_NOT_NULL(isolate)");
    let mut result = self.page_pool.get(isolate)?;
    Some(result.to_result())
  }

  /// 原版 h:96 声明 + cc:332-335 `CreatePooledPage(NormalPage*)` — 依赖 Create 族 → 台账 1/2。
  ///
  /// 落地面等价表达: 以当前 epoch 组装条目 (调用方备好 reservation)。
  pub fn create_pooled_page(&mut self, reservation: R) -> PooledPage<R> {
    PooledPage::new(null_mut(), reservation, self.current_epoch())
  }

  /// 原版 h:106 声明 + cc:419-422 `AddLarge(Isolate*, std::vector<LargePage*>&)`。
  ///
  /// 差异申报 (台账 1): 入参为 Create(LargePage*) 的产物向量; epoch 由本处载入 (cc:420)。
  pub fn add_large<H: ReleaseTaskHost + ?Sized>(
    &mut self,
    isolate: *mut Isolate,
    pages: &mut Vec<PooledPage<R>>,
    host: &mut H,
  ) {
    self.large_pool.add(pages);
    self.post_delayed_release_task_if_needed(isolate, host);
  }

  /// 原版 h:109-110 声明 + cc:423-426 `RemoveLarge(Isolate*, size_t chunk_size)`。
  pub fn remove_large(
    &mut self,
    isolate: *mut Isolate,
    chunk_size: usize,
  ) -> Option<PooledPageResult> {
    self.large_pool.remove(isolate, chunk_size)
  }

  /// 原版 h:115 声明 + cc:428-435 `AddZoneReservation(Isolate*, VirtualMemory)`。
  /// isolate 允许为 nullptr (h:112-114 注释)。
  pub fn add_zone_reservation<H: ReleaseTaskHost + ?Sized>(
    &mut self,
    isolate: *mut Isolate,
    zone_reservation: R,
    host: &mut H,
  ) {
    let entry = PooledVirtualMemory::new(zone_reservation, self.current_epoch());
    self.zone_pool.put_local(isolate, entry);
    self.post_delayed_release_task_if_needed(isolate, host);
  }

  /// 原版 h:120 声明 + cc:437-443 `RemoveZoneReservation(Isolate*)` (取还细节见 Remove)。
  pub fn remove_zone_reservation(&mut self, isolate: *mut Isolate) -> Option<R> {
    let result = self.zone_pool.get(isolate);
    result.map(|entry| entry.into_virtual_memory())
  }

  /// 原版 h:126 声明 + cc:346-355 `ReleaseOnTearDown(Isolate*)`:
  /// 关共享 → 立即释放; 否则本地池转共享池 (给他 isolate 复用机会) + 大页池直接清空。
  pub fn release_on_teardown(&mut self, isolate: *mut Isolate) {
    if !self.config.share_memory_on_teardown {
      self.release_immediately(isolate);
      return;
    }
    self.page_pool.move_local_to_shared(isolate);
    self.zone_pool.move_local_to_shared(isolate);
    self.large_pool.release_all();
  }

  /// 原版 h:129 声明 + cc:357-361 `ReleaseImmediately(Isolate*)` (V8_EXPORT_PRIVATE)。
  /// 注: 大页无 isolate 分桶 → 原版此处亦为全量清空 (cc:360)。
  pub fn release_immediately(&mut self, isolate: *mut Isolate) {
    self.page_pool.release_local_of(isolate);
    self.zone_pool.release_local_of(isolate);
    self.large_pool.release_all();
  }

  /// 原版 h:132 声明 + cc:363-369 `ReleaseAllImmediately()`。
  pub fn release_all_immediately(&mut self) {
    self.page_pool.release_local();
    self.page_pool.release_shared();
    self.zone_pool.release_local();
    self.zone_pool.release_shared();
    self.large_pool.release_all();
  }

  /// 原版 h:135 声明 + cc:371-377 `TearDown()`: 先取消全部已投任务, 再三池拆除。
  pub fn tear_down(&mut self) {
    self.cancellable_task_manager.cancel_and_wait();
    self.page_pool.tear_down();
    self.zone_pool.tear_down();
    self.large_pool.tear_down();
  }

  /// 原版 h:246-248 声明 + cc:379-391 `ReleaseUpTo(Epoch)`: 三池汇总,
  /// `pool_emptied` = 三者皆空 (cc:389-390)。
  pub fn release_up_to(&mut self, release_epoch: Epoch) -> ReleaseStats {
    let page_pool_stats = self.page_pool.release_up_to(release_epoch);
    let large_pool_stats = self.large_pool.release_up_to(release_epoch);
    let zone_pool_stats = self.zone_pool.release_up_to(release_epoch);
    ReleaseStats {
      pages_removed: page_pool_stats.removed_entries,
      large_pages_removed: large_pool_stats.removed_entries,
      zone_reservations_removed: zone_pool_stats.removed_entries,
      pool_emptied: page_pool_stats.pool_emptied
        && large_pool_stats.pool_emptied
        && zone_pool_stats.pool_emptied,
    }
  }

  /// 原版 h:141 声明 + cc:393-395 `GetCount(Isolate*)`: 该 isolate 本地页池条目数。
  pub fn get_count(&self, isolate: *mut Isolate) -> usize {
    self.page_pool.local_size(isolate)
  }

  /// 原版 h:144 声明 + cc:397 `GetSharedCount()`: 共享页池条目数。
  pub fn get_shared_count(&self) -> usize {
    self.page_pool.shared_size()
  }

  /// 原版 h:147 声明 + cc:399 `GetTotalCount()`: 本地各池 + 共享池合计。
  pub fn get_total_count(&self) -> usize {
    self.page_pool.size()
  }

  /// 原版 h:235 声明 + cc:491-512 `PostDelayedReleaseTask(Isolate*, base::TimeDelta)`。
  ///
  /// cc:495-497 注释: 该方案下入池页在 [timeout, 2*timeout) 秒内被回收, 避免 GC 恰在任务前
  /// 跑完时过早释放。release_epoch 由 `fetch_add` 取旧值 (cc:498-499)。
  pub fn post_delayed_release_task<H: ReleaseTaskHost + ?Sized>(
    &mut self,
    isolate: *mut Isolate,
    delay: TimeDelta,
    host: &mut H,
  ) {
    debug_assert_ne!(
      self.config.timeout_in_sec, 0,
      "cc:493 DCHECK(timeout_in_sec != 0)"
    );
    debug_assert!(
      !self.task_disabled.load(Ordering::SeqCst),
      "cc:494 DCHECK(!task_disabled_.load())"
    );
    let release_epoch = self.current_epoch.fetch_add(1, Ordering::Relaxed);
    // cc:500-501 构造任务; cc:502-510 投递 (挂点选择见 task.attaches_to_isolate)。
    let task = ReleasePooledChunksTask::new(self.config, isolate, release_epoch);
    host.post_delayed_release_task(&task, &self.cancellable_task_manager, delay);
    // cc:511。
    self
      .posted_time
      .store(TimeTicks::now().to_internal_value(), Ordering::Relaxed);
  }

  /// 原版 h:236 声明 + cc:514-529 `PostDelayedReleaseTaskIfNeeded(Isolate*)`:
  /// 距上次投递超过 `timeout + kTimeSlack` 才补投 (cc:515-516 注释: 工作线程死亡时任务可能
  /// 未执行, 给若干秒余量)。
  pub fn post_delayed_release_task_if_needed<H: ReleaseTaskHost + ?Sized>(
    &mut self,
    isolate: *mut Isolate,
    host: &mut H,
  ) {
    let pool_timeout = self.config.timeout_in_sec;
    if pool_timeout == 0 || self.task_disabled.load(Ordering::SeqCst) {
      return;
    }
    // cc:521-522 delta = FromSeconds(pool_timeout) + kTimeSlack。
    let delta = TimeDelta::from_seconds(pool_timeout as i64)
      + TimeDelta::from_seconds(Self::K_TIME_SLACK_SECONDS);
    let now = TimeTicks::now();
    // cc:524-525 deadline = posted_time_ + delta。
    let deadline = self.posted_time() + delta;
    if now > deadline {
      // cc:527 `base::TimeDelta()` = 零延迟。
      self.post_delayed_release_task(isolate, TimeDelta::new(), host);
    }
  }

  /// 原版 h:150 声明 + cc:531-534 `CancelAndWaitForTaskToFinishForTesting()` (V8_EXPORT_PRIVATE)。
  pub fn cancel_and_wait_for_task_to_finish_for_testing(&mut self) {
    self.cancellable_task_manager.cancel_and_wait();
    self.task_disabled.store(true, Ordering::SeqCst);
  }

  /// 原版 h:151 声明 + cc:536 `ReenableTaskForTesting()` (V8_EXPORT_PRIVATE)。
  pub fn reenable_task_for_testing(&mut self) {
    self.task_disabled.store(false, Ordering::SeqCst);
  }
}

// 原版 cc:344 `MemoryPool::~MemoryPool() = default` → Rust 字段递归落释, 无自定义 Drop
// (条目释还顺序由各池的 DCHECK 承担, h:166-169/196)。

// ══════════════════════════════════════════════════════════════════════════
// 推迟台账 (真实耦合面; 双行号 + 原因; 台账锚见 tool/src/heap/memory-pool.*.js 落地臂)
// ══════════════════════════════════════════════════════════════════════════
// 1. PooledPage::Create ×2 — 原版 h:57-58 声明, cc:27-53 (NormalPage*) / cc:56-74 (LargePage*)。
//    实体消费 metadata->size()/is_large()/is_trusted()/is_executable()/ContainsAnySlots()/
//    reserved_memory()/ChunkAddress()/Chunk() 与 chunk->~MemoryChunk() + metadata->~NormalPage()
//    placement 析构 (cc:49-50/70-71), 另 cc:45 `!chunk->InReadOnlySpace()` DCHECK。原因:
//    NormalPage/LargePage/MutablePage/BasePage 本仓实测无落地实体 (仅 memory_chunk.rs:26-27
//    opaque BasePage 与 Metadata 族推迟台账 1-2)。承接位 = [`PooledPage::new`] (cc:20-24)。
// 2. MemoryPool::CreatePooledPage(NormalPage*) — 原版 h:96 声明, cc:332-335 实体 (仅转发 Create +
//    epoch 载入)。原因同 1; 已落等价面 [`MemoryPool::create_pooled_page`] (reservation 由调用方备)。
// 3. Add/AddLarge 的 `MutablePage*` / `std::vector<LargePage*>&` 入参 — 原版 h:99/h:106,
//    cc:401-408/cc:419-422: 转换在函数体内 (cc:405/cc:251)。原因同 1 → 参数改为 Create 产物,
//    erase_if 预算接纳语义与 PostDelayedReleaseTaskIfNeeded 触发点逐行守恒。
// 4. 心跳任务的真实投递与取消注册 — 原版 cc:502-504 `isolate->task_runner()->PostDelayedTask`、
//    cc:506-509 `V8::GetCurrentPlatform()->PostDelayedTaskOnWorkerThread(TaskPriority::
//    kUserVisible, ...)`、cc:449-453 CancelableTask 基类挂点 (单线程挂 isolate CTM)。原因:
//    Isolate 与 Platform 任务投递属他票 (v8-libplatform/v8-tasks 侧)。承接: [`ReleaseTaskHost`]
//    窄 trait + 池自持 [`v8_tasks::CancelableTaskManager`] (h:252/cc:342/373/532 已直用落地面)。
// 5. trace_gc_nvp 打点的 isolate 选点 — 原版 cc:466-477
//    `IsolateGroup::current()->FindAnotherIsolateLocked(nullptr, lambda)` +
//    `Isolate::PrintWithTimestamp`。原因: IsolateGroup 单例他票 (同 memory_chunk 推迟台账 2)。
//    文案面已落 [`ReleaseStats::message`] (cc:472-475), 打印通道由宿主 print 承接。
// 6. VirtualMemory 实体 — 原版 utils/allocation.h:193-306 (IsReserved h:236 / Reset h:239 /
//    region h:243 / size h:263 / free h:293; 实体 allocation.cc:216-300)。原因: PageAllocator
//    平台层, 本仓实测推迟 (v8-utils/src/allocation.rs:5、v8-zone/src/zone/accounting.rs:5 已登记)。
//    承接: [`Reservation`] 窄 trait, 5 面全部消费点已逐行落 (cc:77/82/88/92 + h:45/221)。
// 7. GarbageCollectionPrologue(Isolate*, GarbageCollector) — 原版 h:138 声明。实测 v8/src 与
//    v8/test 全树 grep 无 `MemoryPool::GarbageCollectionPrologue` 定义亦无调用点 (仅
//    new-spaces.h:223/390、heap.h:1928/1930 同名而异主的成员) → 原版为悬空声明, 不落地。
// 8. 移动/拷贝控制 — 原版 h:40-43 (PooledPage move default + copy delete)、h:92-93 (MemoryPool
//    copy delete): Rust 移动语义 + 不派生 Clone 承接, 无需实体。
// 9. 容器与友元 — h:187 `absl::flat_hash_map` → `std::HashMap` (cc:129 begin() 窃取序两实现同为
//    任意序); h:53 `friend class MemoryPool` 与 h:154 内嵌类私有面 → Rust 以 pub 字段承接
//    (同 memory_chunk 推迟台账 11 先例)。
// 10. h:15 前向声明 `class CancelableTaskManager;` 已直用 v8-tasks 落地面 (无需 opaque 承接);
//    h:16-20 其余前向声明 Isolate/LargePage/BasePage/MutablePage/NormalPage 中, 仅 Isolate 有
//    Rust 消费点 (map 键/形参) → 本模块落 opaque [`Isolate`], 页面对象四者由台账 1 覆盖。
// 11. h:254 `static_assert(std::atomic<base::TimeTicks>::is_always_lock_free)` — Rust 无
//    `atomic<TimeTicks>` 形态, posted_time 以 `AtomicI64` 承载 µs 位值 (h:251)。实测本工具链
//    std 无 `is_always_lock_free` 常量 (sysroot core/sync/atomic.rs grep 0 命中) → 断言代入为
//    `cfg!(target_has_atomic = "64")` + 载体宽度 8 (模块级 `const _`), lock-free 属性该目标由
//    64 位原子可用性佐证, 不留假断言面。
