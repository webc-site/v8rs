//! 大对象空间族 (LargeObjectSpace 及六具体空间 + 对象迭代器)。
//! 对应原版 v8/src/heap/large-spaces.h (204 行) + large-spaces.cc (443 行)。
//! (HC-4 空间实体族第一波 R5H3 落地; Space 基面见 crate::spaces)
//!
//! 目标配置 (与 tool/src/heap/large_spaces.js clang -E 展开侧同源):
//!   V8_COMPRESS_POINTERS + V8_ENABLE_SANDBOX + V8_EXTERNAL_CODE_SPACE, DEBUG/TSAN 关。
//!   `#ifdef VERIFY_HEAP` (h:87-89 / cc:280-311) 展开侧剥除 → 不落地 (台账 4);
//!   `#ifdef DEBUG` (h:91-93 / cc:313-321 Print、cc:174-180/209-213 断言区) 以
//!   `#[cfg(debug_assertions)]` 承接 (memory_allocator.rs DEBUG 面先例)。
//!
//! 票面差异申报 (以本仓源实态为准, 主控汇报):
//! 票面所述 "LargeObject/LargePage … block 链表、AllocatedBytes/CommittedBytes 核算" 为
//! 旧版 V8 叙事 (2018 前后 large-object-space); 本仓 large-spaces.{h,cc} 实态 (2020 版,
//! 204/443 行) 已是 **一页一对象** 的 LargePage 模型: 无 LargeObject 类、无 block 链表、
//! 核算面为 size_ (已分配字节)/objects_size_ (对象实体字节)/committed_ (BaseSpace) 三账
//! (h:109-111)。本票按实况 1:1 落地, 不复活旧版面; large block 合并拆分不在本文件族
//! (其在 free-list 族与旧版叙事, free_list.rs 已另行落地)。
//!
//! 承接形映射 (申报):
//! - C++ 页/堆/分配器耦合面 (LargePage 方法面 large-page.h/mutable-page.h/base-page.h、
//!   MemoryAllocator Free/PartialFreeMemory/ReleaseDelayedPages、Heap 观察者/填充与
//!   Isolate RemoveCodeMemoryChunk) 实体均属他票 → 以 [`LargePageOps`] /
//!   [`LargeSpaceRunEnv`] 双 env trait 注入承接 (free_list.rs FreeListEnv /
//!   memory_allocator.rs ChunkContainsEnv 先例), 真实接回条件见文末台账 7。
//! - `std::atomic<size_t> size_/objects_size_` (h:109/111) → AtomicUsize,
//!   原子序 SeqCst 承接 std::atomic 默认序 (全 crate 先例)。
//! - 派生关系 → 组合承接: OldLargeObjectSpace/NewLargeObjectSpace/CodeLargeObjectSpace/
//!   Shared/Trusted/SharedTrusted 以 `Deref/DerefMut → LargeObjectSpace` 暴露继承面
//!   (Rust 无继承; Space trait 实现槽归 Heap 整合票, 台账 8)。
//! - `heap::List` 消费位复用 spaces.rs [`PageList`](crate::spaces::PageList), 页指针
//!   reinterpret_cast (h:69-74) → 裸指针位型直转。
//! - `std::function<bool(Tagged<HeapObject>)>` (cc:387) → `impl FnMut` 形参。
//! - `v8_flags.large_page_pool` (cc:390) → `&Flags` 参数注入 (free_list.rs /
//!   memory_allocator.rs 先例)。
//! - h:106/h:125-184 AllocateRaw/AllocateLargePage 族依赖 MainAllocator 具体分配路径与
//!   GC 阶段状态机 (增量标记/黑分配) → 函数体 todo!() 登记台账 1-3。

use std::{
  ffi::c_void,
  ops::{Deref, DerefMut},
  ptr::{addr_of, null, null_mut},
  sync::atomic::{AtomicUsize, Ordering},
};

use v8_base::{address_region::AddressRegion, macros::round_up_address};
use v8_base_platform::RecursiveMutex;
use v8_common::{Address, AllocationHint, AllocationSpace, Executability, slow_dcheck};
use v8_flags::Flags;
use v8_objects_core::{heap_object::HeapObject, tagged::Tagged};

use crate::{
  allocation::AllocationResult,
  allocation_observer::{AllocationCounter, ObserverRef},
  main_allocator::LocalHeap,
  memory_allocator::{FreeMode, Heap, LargePage, MemoryAllocator, MutablePage},
  memory_chunk::{FROM_PAGE, MemoryChunk, TO_PAGE},
  memory_pool::Reservation,
  spaces::{LargePageIterator, NoFreeList, ObjectIterator, PageIteratorImpl, Space, SpaceBase},
};

// ══════════════════════════════════════════════════════════════════════════
// 页实体 / 运行时桥 env (承接形申报见模块头; 真实接回条件见台账 7)
// ══════════════════════════════════════════════════════════════════════════

/// 原版 large-page.h:27-34 + large-page-inl.h + base-page.h + mutable-page.h 页方法面的
/// 消费子集承接 (页实体属他票)。所有槽以 *mut LargePage 句柄寻址, 实体落地后由
/// LargePage 方法直调替换 (台账 7)。
pub trait LargePageOps {
  /// 原版 base-page.h `size_t size() const { return size_; }` (AddPage/RemovePage/
  /// Shrink cc:189/198/225-226 消费)。
  fn page_size(&self, page: *mut LargePage) -> usize;

  /// 原版 mutable-page.h `MemoryChunk* Chunk()` (cc:175/208/268 消费)。
  fn chunk_of(&self, page: *mut LargePage) -> *mut MemoryChunk;

  /// 原版 memory-chunk-inl.h `Address MemoryChunk::address() const` (cc:228 消费)。
  fn chunk_address(&self, chunk: *mut MemoryChunk) -> Address;

  /// 原版 memory-chunk-inl.h `size_t MemoryChunk::Offset(Address) const` (cc:216 消费)。
  fn chunk_offset(&self, chunk: *mut MemoryChunk, addr: Address) -> usize;

  /// 原版 base-page.h `Address area_start() const` (cc:222/292 消费)。
  fn area_start(&self, page: *mut LargePage) -> Address;

  /// 原版 base-page.h:52 注释面 `size_t area_size() const { return area_end_ - area_start_; }`
  /// (cc:156/220/239 消费)。
  fn area_size(&self, page: *mut LargePage) -> usize;

  /// 原版 base-page.h `void set_area_end(Address end)` (cc:234 消费)。
  fn set_area_end(&mut self, page: *mut LargePage, end: Address);

  /// 原版 large-page.h:27-29 `Tagged<HeapObject> GetObject() const` —
  /// `HeapObject::FromAddress(area_start())` (cc:43/127/183/210/211/291/357/396 消费;
  /// HCR1-F7 校正: 原注 "43?" 问号残留已亲验定谳, grep 实测 GetObject() 落点)。
  fn get_object(&self, page: *mut LargePage) -> Tagged<HeapObject>;

  /// 原版 objects/heap-object.h `int HeapObject::Size() const` (cc:183/210/401 消费;
  /// v8-objects-core heap_object.rs 无 Size 面 → 宿主桥, 台账 7)。
  fn object_size(&self, object: Tagged<HeapObject>) -> usize;

  /// 原版 objects/objects.h `IsFreeSpaceOrFiller(object)` (cc:45 消费; instance_type
  /// 判定 v8-objects-core instance_type_inl.rs:161 todo → 宿主桥)。
  fn is_free_space_or_filler(&self, object: Tagged<HeapObject>) -> bool;

  /// 原版 large-page.h:31-34 `LargePage* next_page()` — list_node_.next() 转型
  /// (spaces-inl.h:25 与 cc:285/379-380 消费)。
  fn next_page(&self, page: *mut LargePage) -> *mut LargePage;

  /// 原版 base-page.h `BaseSpace* owner()` + mutable-page.h set_owner (cc:194/202 消费)。
  /// owner 位型以 `*const c_void` 承接 (C++ BaseSpace*, 仅指针相等比较, cc:259)。
  fn set_page_owner(&mut self, page: *mut LargePage, owner: *const c_void);

  /// 原版 large-page.cc `ClearOutOfLiveRangeSlots(Address free_start)` (cc:221 消费;
  /// 实体桶对齐面 large-page.cc 他票, skel LargePageSkeleton::aligned_area_end_for_bucket_clear
  /// 注记留档)。
  fn clear_out_of_live_range_slots(&mut self, page: *mut LargePage, free_start: Address);

  /// 原版 mutable-page.h `SetFlagNonExecutable(MemoryChunk::MainThreadFlags)`
  /// (cc:359/381 消费; 可执行页写保护域目标配置剥除, 申报见台账 7)。
  fn set_flag_non_executable(&mut self, page: *mut LargePage, flag: usize);

  /// 原版 mutable-page.h `ClearFlagNonExecutable(MainThreadFlags)` (cc:182/382 消费)。
  fn clear_flag_non_executable(&mut self, page: *mut LargePage, flag: usize);

  /// 原版 mutable-page.h `bool is_executable() const` (cc:212 DCHECK 消费)。
  fn page_is_executable(&self, page: *mut LargePage) -> bool;

  /// 原版 mutable-page.h `ClearLiveness()` (cc:361 minor_ms 支消费)。
  fn clear_liveness(&mut self, page: *mut LargePage);
}

/// 原版 large-spaces.cc 消费 Heap/MemoryAllocator/Isolate 运行面的桥承接
/// (MemoryAllocator::Free/PartialFreeMemory/ReleaseDelayedPages 在 memory_allocator.rs
/// 推迟台账 7/9 同批解锁; Heap 观察者/填充面与 Isolate 链属 heap.h 他票, 台账 7)。
pub trait LargeSpaceRunEnv {
  /// 原版 cc:76 `heap()->IsAllocationObserverActive()`。
  fn is_allocation_observer_active(&self) -> bool;

  /// 原版 cc:80 `heap_->CreateFillerObjectAt(soon_object, static_cast<int>(size))`。
  fn create_filler_object_at(&mut self, soon_object: Address, size: i32);

  /// 原版 cc:69-70/399 `heap()->memory_allocator()->Free(free_mode, page)`。
  fn allocator_free(&mut self, mode: FreeMode, page: *mut LargePage);

  /// 原版 cc:227-229 `memory_allocator()->PartialFreeMemory(page, start, bytes,
  /// new_area_end)`。
  fn partial_free_memory(
    &mut self,
    page: *mut LargePage,
    start: Address,
    bytes_to_free: usize,
    new_area_end: Address,
  );

  /// 原版 cc:407 `heap()->memory_allocator()->ReleaseDelayedPages()`。
  fn release_delayed_pages(&mut self);

  /// 原版 cc:388 `heap()->incremental_marking()->IsMarking()` (DCHECK 位)。
  fn is_marking(&self) -> bool;

  /// 原版 cc:257-259 `BasePage::FromHeapObject(heap()->isolate(), object)->owner()` —
  /// Isolate 链 + BasePage 实体他票 → owner 位型 `*const c_void` 直出。
  fn page_owner_of_object(&self, object: Tagged<HeapObject>) -> *const c_void;

  /// 原版 cc:429 `heap()->isolate()->RemoveCodeMemoryChunk(page)`。
  fn remove_code_memory_chunk(&mut self, page: *mut LargePage);
}

/// 原版 MemoryAllocator::GetCommitPageSize() 静态面的泛参占位 (get_commit_page_size 为
/// MemoryAllocator<R> 关联静态函数, 调用不消费 R) — 不可建空 enum 的 Reservation 空实现
/// (方法体经空匹配永不可达, 无 panic 面; memory_allocator.rs:124 进程级静态同源)。
pub enum NoReservation {}

impl Reservation for NoReservation {
  fn is_reserved(&self) -> bool {
    match *self {}
  }
  fn size(&self) -> usize {
    match *self {}
  }
  fn region(&self) -> AddressRegion {
    match *self {}
  }
  fn free(&mut self) {
    match *self {}
  }
  fn reset(&mut self) {
    match *self {}
  }
}

/// 提交页大小读出 (cc:217/246-247 `MemoryAllocator::GetCommitPageSize()`)。
#[inline]
#[must_use]
fn commit_page_size() -> usize {
  MemoryAllocator::<NoReservation>::get_commit_page_size() as usize
}

// ══════════════════════════════════════════════════════════════════════════
// LargeObjectSpace (原版 h:29-123; h:30-31 注释: > kMaxRegularHeapObjectSize 的大对象
// 由本空间分配管理, GC 期间不移动)
// ══════════════════════════════════════════════════════════════════════════

/// 原版 h:33 `class LargeObjectSpace : public Space`。
/// h:109-119 字段面照录 (size_/page_count_/objects_size_/allocation_mutex_/
/// allocation_counter_)。h:38 析构 `~LargeObjectSpace() { TearDown(); }` → Rust 无隐式
/// Drop 契约 (tear_down 显式调用, 归 Heap 整合票接线, 台账 8)。
pub struct LargeObjectSpace {
  /// Space/BaseSpace 字段面 (h:69-70 构造, cc:53-54 `Space(heap, id, nullptr)` —
  /// free_list 恒空, NoFreeList 泛参占位)。
  pub base: SpaceBase<NoFreeList>,
  /// 原版 h:109 `std::atomic<size_t> size_` — 已分配字节 (注释 `// allocated bytes`)。
  pub size_: AtomicUsize,
  /// 原版 h:110 `int page_count_` — chunk 数 (注释 `// number of chunks`)。非原子照录。
  pub page_count: i32,
  /// 原版 h:111 `std::atomic<size_t> objects_size_` — 对象实体字节。
  pub objects_size_: AtomicUsize,
  /// 原版 h:112-116 注释 + `base::RecursiveMutex allocation_mutex_` —
  /// 重入必要性 (profiler tick 再入 FindPage) 照录。
  pub allocation_mutex: RecursiveMutex,
  /// 原版 h:119 `AllocationCounter allocation_counter_`。
  pub allocation_counter: AllocationCounter,
}

impl LargeObjectSpace {
  /// 原版 h:101 声明 + cc:53-54 protected ctor
  /// `LargeObjectSpace(Heap* heap, AllocationSpace id)
  ///   : Space(heap, id, nullptr), size_(0), page_count_(0), objects_size_(0)`。
  #[must_use]
  pub fn new(heap: *mut Heap, id: AllocationSpace) -> Self {
    Self {
      base: SpaceBase::new(heap, id, None),
      size_: AtomicUsize::new(0),
      page_count: 0,
      objects_size_: AtomicUsize::new(0),
      allocation_mutex: RecursiveMutex::new(),
      allocation_counter: AllocationCounter::default(),
    }
  }

  /// 原版 h:64 `bool IsEmpty() const { return first_page() == nullptr; }`。
  #[inline]
  #[must_use]
  pub fn is_empty(&self) -> bool {
    self.first_page().is_null()
  }

  /// 原版 h:69-71 `LargePage* first_page() override` —
  /// `reinterpret_cast<LargePage*>(memory_chunk_list_.front())`。
  #[inline]
  #[must_use]
  pub fn first_page(&self) -> *mut LargePage {
    self.base.memory_chunk_list.front() as *mut LargePage
  }

  /// 原版 h:72-74 const 重载 — 指针位型同值, 并槽承接 (spaces.rs first_page 四重载
  /// 并槽申报同法)。
  #[inline]
  #[must_use]
  pub fn first_page_const(&self) -> *const LargePage {
    self.first_page() as *const LargePage
  }

  /// 原版 h:52 `int PageCount() const { return page_count_; }`。
  #[inline]
  #[must_use]
  pub fn page_count(&self) -> i32 {
    self.page_count
  }

  /// 原版 h:98 `void set_objects_size(size_t)`。
  #[inline]
  pub fn set_objects_size(&mut self, objects_size: usize) {
    self.objects_size_.store(objects_size, Ordering::SeqCst);
  }

  /// 原版 h:76 `iterator begin()` — LargePageIterator(first_page()) (h:35 别名)。
  #[inline]
  #[must_use]
  pub fn begin(&self) -> LargePageIterator {
    LargePageIterator::new(self.first_page())
  }

  /// 原版 h:77 `iterator end()` — LargePageIterator(nullptr)。
  #[inline]
  #[must_use]
  pub fn end(&self) -> LargePageIterator {
    LargePageIterator::new(null_mut())
  }

  /// 原版 h:79-80 const_iterator 对 — *const 位型并槽 (同上申报)。
  #[inline]
  #[must_use]
  pub fn const_begin(&self) -> PageIteratorImpl<*const LargePage> {
    PageIteratorImpl::new(self.first_page_const())
  }

  /// 原版 h:80 const_iterator end。
  #[inline]
  #[must_use]
  pub fn const_end(&self) -> PageIteratorImpl<*const LargePage> {
    PageIteratorImpl::new(null())
  }

  // ── 生命周期与观察者 (cc:62-96) ──

  /// 原版 h:40-41 注释 (释放内部资源, 释空本空间对象) + cc:62-72 `TearDown()`。
  /// 循环体 1:1: 非空则取 first → 表内摘除 → allocator Free(kImmediately) (桥, 台账 7)。
  /// cc:65-67 `LOG(DeleteEvent("LargeObjectChunk", …))` v8-logging 不在本 crate 依赖 →
  /// 剥除不复活 (memory_allocator 台账 14 同型; 台账 6)。
  pub fn tear_down<E: LargePageOps + LargeSpaceRunEnv + ?Sized>(&mut self, env: &mut E) {
    while !self.base.memory_chunk_list.is_empty() {
      let page = self.first_page();
      // SAFETY-注: page 非空由循环条件 (list 非空 → front 非 null) 保证。
      debug_assert!(!page.is_null(), "cc:64 first_page() 须非空 (list 非空)");
      self.base.memory_chunk_list.remove(page as *mut MutablePage);
      env.allocator_free(FreeMode::Immediately, page);
    }
  }

  /// 原版 h:103 声明 + cc:74-88 `AdvanceAndInvokeAllocationObservers`。
  /// cc:79 注释 (确保对象有效) 随 create_filler 桥落; cc:86 注释 (大对象无 LAB 可即时
  /// 核算) 照录。object_size 形参: 原版 size_t; cc:80 CreateFillerObjectAt 的
  /// `static_cast<int>` 截断在桥实现侧承接 (大对象尺寸 < 4GB 由页上限保证, 见
  /// large-page.h:18 kMaxCodePageSize)。
  pub fn advance_and_invoke_allocation_observers<E: LargeSpaceRunEnv + ?Sized>(
    &mut self,
    env: &mut E,
    soon_object: Address,
    object_size: usize,
  ) {
    // cc:76 `if (!heap()->IsAllocationObserverActive()) return;`
    if !env.is_allocation_observer_active() {
      return;
    }
    // cc:78 `if (object_size >= allocation_counter_.NextBytes())`
    if object_size >= self.allocation_counter.next_bytes() {
      // cc:80 CreateFillerObjectAt(soon_object, static_cast<int>(object_size))
      env.create_filler_object_at(soon_object, object_size as i32);
      // cc:82-83 InvokeAllocationObservers(soon_object, object_size, object_size)
      self
        .allocation_counter
        .invoke_allocation_observers(soon_object, object_size, object_size);
    }
    // cc:87 AdvanceAllocationObservers(object_size) — 无条件推进 (注释 cc:86)。
    self
      .allocation_counter
      .advance_allocation_observers(object_size);
  }

  /// 原版 h:84 声明 + cc:90-92 `AddAllocationObserver`。
  #[inline]
  pub fn add_allocation_observer(&mut self, observer: ObserverRef) {
    self.allocation_counter.add_allocation_observer(observer);
  }

  /// 原版 h:85 声明 + cc:94-96 `RemoveAllocationObserver`。
  #[inline]
  pub fn remove_allocation_observer(&mut self, observer: ObserverRef) {
    self.allocation_counter.remove_allocation_observer(observer);
  }

  // ── 页台账 (cc:188-203; h:66-67 虚槽) ──

  /// 原版 h:66 声明 + cc:188-195 `AddPage(LargePage*, size_t object_size)`。
  /// cc:189 `size_ += static_cast<int>(page->size())`: int 截断安全域 — 页尺寸上界
  /// kMaxCodePageSize = 512MB (large-page.h:18) < INT_MAX → 直接以 usize 累加承接
  /// (差异申报注记, 不复活截断)。
  pub fn add_page<E: LargePageOps + ?Sized>(
    &mut self,
    env: &mut E,
    page: *mut LargePage,
    object_size: usize,
  ) {
    // cc:189-192 三账 + 页计数。
    self.size_.fetch_add(env.page_size(page), Ordering::SeqCst);
    let owner = addr_of!(*self) as *const c_void;
    self.base.account_committed(env.page_size(page));
    self.objects_size_.fetch_add(object_size, Ordering::SeqCst);
    self.page_count += 1;
    // cc:193 `memory_chunk_list_.PushBack(page)` — 基表 *mut MutablePage 位型直转
    // (C++ List<MutablePage> 上转隐式, Rust as 承接)。
    self
      .base
      .memory_chunk_list
      .push_back(page as *mut MutablePage);
    // cc:194 `page->set_owner(this)`。
    env.set_page_owner(page, owner);
  }

  /// 原版 h:67 声明 + cc:197-203 `RemovePage(LargePage*)`。
  /// 注: 原版不减 objects_size_ (右修剪后惰性重算, cc:404-405 注释), 不补账。
  pub fn remove_page<E: LargePageOps + ?Sized>(&mut self, env: &mut E, page: *mut LargePage) {
    self.size_.fetch_sub(env.page_size(page), Ordering::SeqCst);
    self.base.account_uncommitted(env.page_size(page));
    self.page_count -= 1;
    self.base.memory_chunk_list.remove(page as *mut MutablePage);
    // cc:202 `page->set_owner(nullptr)`。
    env.set_page_owner(page, null());
  }

  // ── 尺寸核算 (cc:205-254) ──

  /// 原版 h:54-55 声明 + cc:205-240 `ShrinkPageToObjectSize`。
  /// cc:209-213 DEBUG 断言区以 `#[cfg(debug_assertions)]` 承接 (object/大小/不可执行三面
  /// 判据经 env 桥); cc:215-217 used_committed_size = RoundUp(chunk->Offset(addr)+size,
  /// GetCommitPageSize) 直落。PartialFreeMemory 走桥 (memory_allocator 台账 9 同批解锁)。
  pub fn shrink_page_to_object_size<E: LargePageOps + LargeSpaceRunEnv + ?Sized>(
    &mut self,
    env: &mut E,
    page: *mut LargePage,
    object: Tagged<HeapObject>,
    object_size: usize,
  ) {
    let chunk = env.chunk_of(page);
    #[cfg(debug_assertions)]
    {
      // cc:210 DCHECK_EQ(object, page->GetObject())
      debug_assert_eq!(
        env.get_object(page),
        object,
        "cc:210 DCHECK_EQ(object, page->GetObject())"
      );
      // cc:211 DCHECK_EQ(object_size, page->GetObject()->Size())
      debug_assert_eq!(
        env.object_size(env.get_object(page)),
        object_size,
        "cc:211 DCHECK_EQ(object_size, Size())"
      );
      // cc:212 DCHECK(!page->is_executable())
      debug_assert!(
        !env.page_is_executable(page),
        "cc:212 DCHECK(!page->is_executable())"
      );
    }

    // cc:215-217 `::RoundUp(chunk->Offset(object.address()) + object_size,
    // MemoryAllocator::GetCommitPageSize())`。
    let used_committed_size = round_up_address(
      env.chunk_offset(chunk, object.address()) + object_size,
      commit_page_size(),
    );

    // cc:219-220 注释 + 判据: 对象自上次 GC 后缩小。
    if object_size < env.area_size(page) {
      // cc:221 `page->ClearOutOfLiveRangeSlots(object.address() + object_size)`。
      env.clear_out_of_live_range_slots(page, object.address() + object_size);
      // cc:222 `const Address new_area_end = page->area_start() + object_size`。
      let new_area_end = env.area_start(page) + object_size;

      // cc:224-225 注释: 缩小到可释放部分 OS 页。
      if used_committed_size < env.page_size(page) {
        let bytes_to_free = env.page_size(page) - used_committed_size;
        // cc:227-229 PartialFreeMemory(page, chunk->address() + used_committed_size,
        // bytes_to_free, new_area_end)。
        env.partial_free_memory(
          page,
          env.chunk_address(chunk) + used_committed_size,
          bytes_to_free,
          new_area_end,
        );
        // cc:230-231 双账回退。
        self.size_.fetch_sub(bytes_to_free, Ordering::SeqCst);
        self.base.account_uncommitted(bytes_to_free);
      } else {
        // cc:233 注释: 不能释 OS 页, 仅更新对象区。
        env.set_area_end(page, new_area_end);
      }
    }

    // cc:238-239 收尾双 DCHECK (used_committed_size 与 area 尺寸应收敛到页账)。
    debug_assert_eq!(used_committed_size, env.page_size(page), "cc:238");
    debug_assert_eq!(object_size, env.area_size(page), "cc:239");
  }

  /// 原版 h:96 声明 + cc:242-254 `UpdateAccountingAfterResizingObject(old, new)`。
  /// 纯字节核算: 提交页对偶差值入 size_/committed_, 实体差值入 objects_size_。
  /// cc:243 注释 (对象原地改大小后补账) 照录。
  pub fn update_accounting_after_resizing_object(&mut self, old_size: usize, new_size: usize) {
    // cc:244 DCHECK_GE(new_object_size, old_object_size)。
    debug_assert!(new_size >= old_size, "cc:244 DCHECK_GE(new, old)");
    let cps = commit_page_size();
    let old_size_committed = round_up_address(old_size, cps);
    let new_size_committed = round_up_address(new_size, cps);
    // cc:249 DCHECK_GE(new_size_committed, old_size_committed)。
    debug_assert!(new_size_committed >= old_size_committed, "cc:249");

    let committed_delta = new_size_committed - old_size_committed;
    // cc:251-253。
    self.size_.fetch_add(committed_delta, Ordering::SeqCst);
    self.base.account_committed(committed_delta);
    self
      .objects_size_
      .fetch_add(new_size - old_size, Ordering::SeqCst);
  }

  // ── 归属判定 (cc:256-272; h:57-61 注释面照录) ──

  /// 原版 h:57-58 注释 (O(1) 判对象是否在本空间) + cc:256-264 `Contains`。
  /// cc:257 `BasePage::FromHeapObject(heap()->isolate(), object)` 的 Isolate 链与
  /// BasePage 实体他票 → 经 [`LargeSpaceRunEnv::page_owner_of_object`] 桥 (台账 7)。
  /// cc:261 `SLOW_DCHECK(!owned || ContainsSlow(...))` → v8_common::slow_dcheck 承接
  /// (checks.rs:29-34 裁定: 目标配置无 ENABLE_SLOW_DCHECKS → 编译期空操作且条件惰性
  /// 不求值, 故不作 debug 断言复活)。
  #[must_use]
  pub fn contains<E: LargePageOps + LargeSpaceRunEnv + ?Sized>(
    &self,
    env: &E,
    object: Tagged<HeapObject>,
  ) -> bool {
    // cc:259 `bool owned = (chunk->owner() == this)`。
    let owned = env.page_owner_of_object(object) == addr_of!(*self) as *const c_void;
    // cc:261 SLOW_DCHECK — 闭包惰性 (本配置下零成本)。
    slow_dcheck(|| !owned || self.contains_slow(env, object.address()));
    owned
  }

  /// 原版 h:59-61 注释 (遍历全部对象判地址, 可能慢) + cc:266-272 `ContainsSlow`。
  /// cc:267 `MemoryChunk::FromAddress` 直用已落地面 (memory_chunk.rs:202);
  /// cc:268-269 range-for 迭代即 next_page 链推进 (h:76-80 begin/end), 与本表序等价。
  #[must_use]
  pub fn contains_slow<E: LargePageOps + ?Sized>(&self, env: &E, addr: Address) -> bool {
    let chunk = MemoryChunk::from_address(addr);
    let mut page = self.first_page();
    while !page.is_null() {
      if env.chunk_of(page) == chunk {
        return true;
      }
      page = env.next_page(page);
    }
    false
  }

  // ── 分配路径 (cc:98-164; 深水推迟, 台账 1-2) ──

  /// 原版 h:105-106 声明 + cc:143-164 `AllocateLargePage(object_size, executable, hint)`。
  /// 推迟 (台账 2): Heap::heap_expansion_mutex / IsOldGenerationExpansionAllowed 与
  /// MemoryAllocator::AllocateLargePage (memory_allocator.rs 推迟台账 1) 未落地。
  pub fn allocate_large_page<E: LargePageOps + ?Sized>(
    &mut self,
    _env: &mut E,
    _object_size: i32,
    _executable: Executability,
    _hint: AllocationHint,
  ) -> *mut LargePage {
    todo!(
      "v8: large-spaces.cc:143-164 — MemoryAllocator::AllocateLargePage/Heap 扩容判据未落地 (台账 2)"
    )
  }

  /// 原版 h:82 声明 (GetObjectIterator) + cc:274-278 — unique_ptr<ObjectIterator> 返回
  /// 具名值 (spaces.rs trait 槽申报, 台账 23 对应: 本空间固有方法落点)。
  /// heap 形参照录 (原版 cc:274-277 不消费, 仅 h:84 签名面)。
  pub fn get_object_iterator<'e, E: LargePageOps + ?Sized>(
    &'e self,
    _heap: *mut Heap,
    env: &'e E,
  ) -> LargeObjectSpaceObjectIterator<'e, E> {
    // cc:276-277 `new LargeObjectSpaceObjectIterator(this)` (cc:36-39 取 first_page)。
    LargeObjectSpaceObjectIterator::new(self, env)
  }

  // h:87-88 `#ifdef VERIFY_HEAP Verify(...)` cc:280-311 — 目标配置剥除不落地 (台账 4)。
  // h:91-93 `#ifdef DEBUG Print()` cc:313-321 — 覆写位归 Space trait impl 槽 (HCR1-F3 整改:
  // 原固有 print(&self) 与 trait 槽 print(&mut self) 签名不合, 未构成覆写 → 统一覆写形)。
  // h:122 `friend class LargeObjectSpaceObjectIterator` → Rust 无友元, 面 pub 承接
  // (memory_allocator 台账 16 先例)。
}

/// 原版 h:33-123 Space 虚面实现 (spaces.rs [`Space`] trait)。
/// 注: trait impl 方法位不加 `#[must_use]` (编译器已废弃该附着位)。
impl Space for LargeObjectSpace {
  type FreeListKind = NoFreeList;

  fn space_base(&self) -> &SpaceBase<NoFreeList> {
    &self.base
  }
  fn space_base_mut(&mut self) -> &mut SpaceBase<NoFreeList> {
    &mut self.base
  }

  /// 原版 h:44 声明 + cc:56-60 `size_t Available() const override` —
  /// 返回 0 (h:43 注释: 无法复用既有大对象内存)。
  #[inline]
  fn available(&self) -> usize {
    0
  }

  /// 原版 h:46 `size_t Size() const override { return size_; }`。
  #[inline]
  fn size(&self) -> usize {
    self.size_.load(Ordering::SeqCst)
  }

  /// 原版 h:47 `size_t SizeOfObjects() const override { return objects_size_; }`。
  #[inline]
  fn size_of_objects(&self) -> usize {
    self.objects_size_.load(Ordering::SeqCst)
  }

  /// 原版 h:91-93 声明 + cc:313-321 `void Print() override` — DEBUG-only 面, 覆写
  /// spaces.rs [`Space::print`] trait 槽 (h:113 `#ifdef DEBUG virtual void Print() = 0`
  /// 纯虚位)。推迟 (台账 5): 体消费 i::Print(obj, os) (utils/ostreams 实体他票)。
  /// HCR1-F3 整改: 原固有 `print(&self)` 与 trait 槽 `print(&mut self)` 签名不合,
  /// 未构成覆写 (走 trait 默认空体) → 归位为 trait 槽覆写形。
  #[cfg(debug_assertions)]
  fn print(&mut self) {
    todo!("v8: large-spaces.cc:313-321 — i::Print 承接他票 (台账 5)")
  }

  /// 原版 h:50 注释 (物理内存近似) + cc:166-171 `CommittedPhysicalMemory()` —
  /// cc:170 `return CommittedMemory()`; cc:167-169 注释 (lazy-commit 平台过量核算,
  /// 暂无精确手段) 照录。
  #[inline]
  fn committed_physical_memory(&self) -> usize {
    self.committed_memory()
  }
}

/// 原版 h:191-199 + cc:36-48 `class LargeObjectSpaceObjectIterator : public ObjectIterator`。
/// E = 页判据宿主 (GetObject/next_page/IsFreeSpaceOrFiller 桥, 模块头申报)。
pub struct LargeObjectSpaceObjectIterator<'e, E: LargePageOps + ?Sized> {
  /// 原版 h:198 `LargePage* current_`; cc:38 初始化为 space->first_page()。
  pub current: *mut LargePage,
  /// 推进/取物判据宿主 (承接 C++ 页实体直调位)。
  pub env: &'e E,
}

impl<'e, E: LargePageOps + ?Sized> LargeObjectSpaceObjectIterator<'e, E> {
  /// 原版 cc:36-39 ctor。
  #[must_use]
  pub fn new(space: &LargeObjectSpace, env: &'e E) -> Self {
    Self {
      current: space.first_page(),
      env,
    }
  }
}

impl<E: LargePageOps + ?Sized> ObjectIterator for LargeObjectSpaceObjectIterator<'_, E> {
  /// 原版 h:195 声明 + cc:41-48 `Tagged<HeapObject> Next() override` —
  /// 链上逐页取对象并推进 current_, 跳过 FreeSpace/Filler; 耗尽返回 null Tagged
  /// (cc:47 `return Tagged<HeapObject>()`)。
  fn next(&mut self) -> Tagged<HeapObject> {
    while !self.current.is_null() {
      let object = self.env.get_object(self.current);
      self.current = self.env.next_page(self.current);
      // cc:45 `if (!IsFreeSpaceOrFiller(object)) return object;`。
      if !self.env.is_free_space_or_filler(object) {
        return object;
      }
    }
    Tagged::NULL
  }
}

// ══════════════════════════════════════════════════════════════════════════
// OldLargeObjectSpace (原版 h:125-140 + cc:98-141/173-186/324-328)
// ══════════════════════════════════════════════════════════════════════════

/// 原版 h:125 `class OldLargeObjectSpace : public LargeObjectSpace` — 组合承接 +
/// Deref 暴露继承面 (模块头申报)。
pub struct OldLargeObjectSpace {
  /// 继承体 (Space 虚面与台账全在此)。
  pub space: LargeObjectSpace,
}

impl Deref for OldLargeObjectSpace {
  type Target = LargeObjectSpace;
  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.space
  }
}

impl DerefMut for OldLargeObjectSpace {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.space
  }
}

impl OldLargeObjectSpace {
  /// 原版 h:127 声明 + cc:324-325 `explicit OldLargeObjectSpace(Heap*)` —
  /// LargeObjectSpace(heap, LO_SPACE)。
  #[must_use]
  pub fn new(heap: *mut Heap) -> Self {
    Self {
      space: LargeObjectSpace::new(heap, AllocationSpace::LoSpace),
    }
  }

  /// 原版 h:134-135 声明 + cc:327-328 protected `OldLargeObjectSpace(Heap*, id)` —
  /// 四子空间 (CODE_LO/TRUSTED_LO/SHARED_LO/SHARED_TRUSTED_LO) 复用槽。
  #[must_use]
  pub fn new_with_id(heap: *mut Heap, id: AllocationSpace) -> Self {
    Self {
      space: LargeObjectSpace::new(heap, id),
    }
  }

  /// 原版 h:129-130 声明 + cc:98-102 `AllocateRaw(local_heap, object_size, hint)` —
  /// 对位 cc:101 单行转发 `return AllocateRaw(local_heap, object_size,
  /// NOT_EXECUTABLE, hint);` (域参数 NOT_EXECUTABLE 与原版逐字对齐, HCR1-F6 整改:
  /// 原 todo! 漏填形清除; 被转发四参体仍台账 1 推迟)。
  pub fn allocate_raw(
    &mut self,
    local_heap: *mut LocalHeap,
    object_size: i32,
    hint: AllocationHint,
  ) -> AllocationResult {
    // cc:101 `NOT_EXECUTABLE` → Executability::NotExecutable (globals.h:1838)。
    self.allocate_raw_executability(local_heap, object_size, Executability::NotExecutable, hint)
  }

  /// 原版 h:136-139 声明 + cc:104-141 protected 四参 `AllocateRaw(..., executable, ...)`。
  /// 推迟 (台账 1): ShouldExpandOldGenerationOnSlowAllocation / CanExpandOldGeneration /
  /// StartIncrementalMarkingIfAllocationLimitIsReached / black_allocation 黑标 (marking
  /// 状态机) / NotifyOldGenerationExpansion / v8_flags.sticky_mark_bits —
  /// Heap+增量标记全链他票。
  pub fn allocate_raw_executability(
    &mut self,
    _local_heap: *mut LocalHeap,
    _object_size_in_bytes: i32,
    _executable: Executability,
    _hint: AllocationHint,
  ) -> AllocationResult {
    todo!("v8: large-spaces.cc:104-141 — Heap/GC 阶段状态机依赖 (台账 1)")
  }

  /// 原版 h:132 声明 + cc:173-186 `PromoteNewLargeObject(LargePage*)`。
  /// 推迟 (台账 3): `page->owner_identity()/Chunk() flag 断言` 页实体面 +
  /// `static_cast<LargeObjectSpace*>(page->owner())->RemovePage` owner 反解引用 +
  /// `SetOldGenerationPageFlags(incremental_marking()->marking_mode())` GC 阶段状态机。
  /// cc:174-180 DEBUG 断言区随体登记 (不单独承接)。
  pub fn promote_new_large_object(&mut self, _page: *mut LargePage) {
    todo!("v8: large-spaces.cc:173-186 — 页 owner 反解 + marking_mode 未落地 (台账 3)")
  }
}

// ══════════════════════════════════════════════════════════════════════════
// NewLargeObjectSpace (原版 h:159-177 + cc:330-412)
// ══════════════════════════════════════════════════════════════════════════

/// 原版 h:159 `class NewLargeObjectSpace : public LargeObjectSpace`。
pub struct NewLargeObjectSpace {
  /// 继承体。
  pub space: LargeObjectSpace,
  /// 原版 h:176 + cc:331 初参 `size_t capacity_`。
  pub capacity_: usize,
}

impl Deref for NewLargeObjectSpace {
  type Target = LargeObjectSpace;
  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.space
  }
}

impl DerefMut for NewLargeObjectSpace {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.space
  }
}

impl NewLargeObjectSpace {
  /// 原版 h:161 声明 + cc:330-331 `NewLargeObjectSpace(Heap* heap, size_t capacity)
  ///   : LargeObjectSpace(heap, NEW_LO_SPACE), capacity_(capacity)`。
  #[must_use]
  pub fn new(heap: *mut Heap, capacity: usize) -> Self {
    Self {
      space: LargeObjectSpace::new(heap, AllocationSpace::NewLoSpace),
      capacity_: capacity,
    }
  }

  /// 原版 h:167 声明 + cc:371-376 `Available() override` — h:166 注释 (可用字节) +
  /// cc:372-373 注释 (右修剪可致 SizeOfObjects > capacity_) → 三目饱和减。
  #[must_use]
  pub fn available(&self) -> usize {
    // cc:374 `const size_t size_of_objects = SizeOfObjects()`。
    let size_of_objects = self.space.size_of_objects();
    // cc:375 `capacity_ > size_of_objects ? capacity_ - size_of_objects : 0` —
    // 三目饱和减的 Rust 等价式。
    self.capacity_.saturating_sub(size_of_objects)
  }

  /// 原版 h:169 声明 + cc:378-384 `Flip()` — 全页 FROM/TO 旗标翻转 (Scavenge 换半区)。
  pub fn flip<E: LargePageOps + ?Sized>(&mut self, env: &mut E) {
    // cc:379-380 for (page = first_page(); page != nullptr; page = page->next_page())。
    let mut page = self.space.first_page();
    while !page.is_null() {
      // cc:381 `page->SetFlagNonExecutable(MemoryChunk::FROM_PAGE)`。
      env.set_flag_non_executable(page, FROM_PAGE);
      // cc:382 `page->ClearFlagNonExecutable(MemoryChunk::TO_PAGE)`。
      env.clear_flag_non_executable(page, TO_PAGE);
      page = env.next_page(page);
    }
  }

  /// 原版 h:171 声明 + cc:386-408 `FreeDeadObjects(is_dead)` — 死页摘除释还, 存活对象
  /// 尺寸和惰性重算 objects_size_ (cc:404-405 注释: 右修剪不更新该账, GC 后统一补)。
  /// cc:388 DCHECK(!IsMarking) 以 env 桥入 debug 区 (GC 阶段状态机判据宿主提供);
  /// cc:390-392 v8_flags.large_page_pool → &Flags 参数注入 (台账 7 接回时由宿主传全局旗标)。
  pub fn free_dead_objects<E: LargePageOps + LargeSpaceRunEnv + ?Sized>(
    &mut self,
    env: &mut E,
    flags: &Flags,
    is_dead: &mut impl FnMut(Tagged<HeapObject>) -> bool,
  ) {
    #[cfg(debug_assertions)]
    {
      // cc:388 `DCHECK(!heap()->incremental_marking()->IsMarking())`。
      debug_assert!(!env.is_marking(), "cc:388 DCHECK(!IsMarking())");
    }
    // cc:389 `size_t surviving_object_size = 0`。
    let mut surviving_object_size: usize = 0;
    // cc:390-392 `const auto free_mode = v8_flags.large_page_pool ? kDelayThenPool :
    // kImmediately`。
    let free_mode = if flags.large_page_pool() {
      FreeMode::DelayThenPool
    } else {
      FreeMode::Immediately
    };
    // cc:393-403 `for (auto it = begin(); it != end();)` + it++ 先行后释 (链上摘页不
    // 破坏迭代) → current/next 双指针 1:1 承接。
    let mut page = self.space.first_page();
    while !page.is_null() {
      let next = env.next_page(page);
      let object = env.get_object(page);
      if is_dead(object) {
        // cc:398-399 RemovePage + Free(free_mode, page)。
        self.space.remove_page(env, page);
        env.allocator_free(free_mode, page);
      } else {
        // cc:401 `surviving_object_size += static_cast<size_t>(object->Size())`。
        surviving_object_size += env.object_size(object);
      }
      page = next;
    }
    // cc:406 `objects_size_ = surviving_object_size` (惰性重算)。
    self
      .space
      .objects_size_
      .store(surviving_object_size, Ordering::SeqCst);
    // cc:407 `heap()->memory_allocator()->ReleaseDelayedPages()` (桥)。
    env.release_delayed_pages();
  }

  /// 原版 h:173 声明 + cc:410-412 `SetCapacity(capacity)` —
  /// `capacity_ = std::max(capacity, SizeOfObjects())` (h:172? 注释无, 语义: 不小于现存量)。
  pub fn set_capacity(&mut self, capacity: usize) {
    self.capacity_ = capacity.max(self.space.size_of_objects());
  }

  /// 原版 h:163-164 声明 + cc:333-369 `AllocateRaw`。
  /// 推迟 (台账 1 同族): CanExpandOldGeneration / ShouldExpandYoungGenerationOnSlowAllocation /
  /// AllocateLargePage (台账 2) / TO_PAGE 旗标+ClearLiveness (可落但整体宿主未落) /
  /// chunk->InitializationMemoryFence (本面已落地 memory_chunk.rs:303, 随体接入)。
  pub fn allocate_raw(
    &mut self,
    _local_heap: *mut LocalHeap,
    _object_size: i32,
    _hint: AllocationHint,
  ) -> AllocationResult {
    todo!("v8: large-spaces.cc:333-369 — Heap 扩容判据 + AllocateLargePage 未落地 (台账 1/2)")
  }
}

// ══════════════════════════════════════════════════════════════════════════
// CodeLargeObjectSpace (原版 h:179-189 + cc:414-431)
// ══════════════════════════════════════════════════════════════════════════

/// 原版 h:179 `class CodeLargeObjectSpace : public OldLargeObjectSpace`。
pub struct CodeLargeObjectSpace {
  /// 继承体。
  pub old: OldLargeObjectSpace,
}

impl Deref for CodeLargeObjectSpace {
  type Target = OldLargeObjectSpace;
  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.old
  }
}

impl DerefMut for CodeLargeObjectSpace {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.old
  }
}

impl CodeLargeObjectSpace {
  /// 原版 h:181 声明 + cc:414-415 `explicit CodeLargeObjectSpace(Heap*)` —
  /// OldLargeObjectSpace(heap, CODE_LO_SPACE)。
  #[must_use]
  pub fn new(heap: *mut Heap) -> Self {
    Self {
      old: OldLargeObjectSpace::new_with_id(heap, AllocationSpace::CodeLoSpace),
    }
  }

  /// 原版 h:183-184 声明 + cc:417-422 `AllocateRaw` —
  /// 对位 cc:420-421 单行转发 `return OldLargeObjectSpace::AllocateRaw(local_heap,
  /// object_size, EXECUTABLE, hint);` (域参数 EXECUTABLE 与原版逐字对齐, HCR1-F6 整改;
  /// 被转发体 cc:104-141 仍台账 1 推迟)。
  pub fn allocate_raw(
    &mut self,
    local_heap: *mut LocalHeap,
    object_size: i32,
    hint: AllocationHint,
  ) -> AllocationResult {
    // cc:420 `EXECUTABLE` → Executability::Executable (globals.h:1838)。
    self
      .old
      .allocate_raw_executability(local_heap, object_size, Executability::Executable, hint)
  }

  /// 原版 h:187 声明 + cc:424-426 `AddPage override` — 纯转基类 (原样保留覆写槽,
  /// 可执行页登记面在实体侧 large-page/MutablePage, 本版本 cc 体无附加动作)。
  pub fn add_page<E: LargePageOps + ?Sized>(
    &mut self,
    env: &mut E,
    page: *mut LargePage,
    object_size: usize,
  ) {
    // cc:425 `OldLargeObjectSpace::AddPage(page, object_size)`。
    self.old.space.add_page(env, page, object_size);
  }

  /// 原版 h:188 声明 + cc:428-431 `RemovePage override` —
  /// cc:429 `heap()->isolate()->RemoveCodeMemoryChunk(page)` 先于基类摘账 (顺序照录)。
  /// Isolate 链经 env 桥 (台账 7); 基类面直调。
  pub fn remove_page<E: LargePageOps + LargeSpaceRunEnv + ?Sized>(
    &mut self,
    env: &mut E,
    page: *mut LargePage,
  ) {
    // cc:429。
    env.remove_code_memory_chunk(page);
    // cc:430 `OldLargeObjectSpace::RemovePage(page)`。
    self.old.space.remove_page(env, page);
  }
}

// ══════════════════════════════════════════════════════════════════════════
// Shared/Trusted 三支 (原版 h:142-157 + cc:433-440; 纯 id 差异化 ctor)
// ══════════════════════════════════════════════════════════════════════════

/// 原版 h:142-145 `class SharedLargeObjectSpace : public OldLargeObjectSpace` —
/// h:142 (共享大对象) 语义仅 ctor id 差。
pub struct SharedLargeObjectSpace {
  /// 继承体。
  pub old: OldLargeObjectSpace,
}

impl Deref for SharedLargeObjectSpace {
  type Target = OldLargeObjectSpace;
  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.old
  }
}

impl DerefMut for SharedLargeObjectSpace {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.old
  }
}

impl SharedLargeObjectSpace {
  /// 原版 cc:433-434 — OldLargeObjectSpace(heap, SHARED_LO_SPACE)。
  #[must_use]
  pub fn new(heap: *mut Heap) -> Self {
    Self {
      old: OldLargeObjectSpace::new_with_id(heap, AllocationSpace::SharedLoSpace),
    }
  }
}

/// 原版 h:147-151 注释 (Similar to the TrustedSpace, but for large objects.) +
/// `class TrustedLargeObjectSpace : public OldLargeObjectSpace`。
pub struct TrustedLargeObjectSpace {
  /// 继承体。
  pub old: OldLargeObjectSpace,
}

impl Deref for TrustedLargeObjectSpace {
  type Target = OldLargeObjectSpace;
  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.old
  }
}

impl DerefMut for TrustedLargeObjectSpace {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.old
  }
}

impl TrustedLargeObjectSpace {
  /// 原版 h:150 + cc:439-440 — OldLargeObjectSpace(heap, TRUSTED_LO_SPACE)。
  #[must_use]
  pub fn new(heap: *mut Heap) -> Self {
    Self {
      old: OldLargeObjectSpace::new_with_id(heap, AllocationSpace::TrustedLoSpace),
    }
  }
}

/// 原版 h:153-157 注释 (Similar to the TrustedLargeObjectSpace, but for shared objects.) +
/// `class SharedTrustedLargeObjectSpace : public OldLargeObjectSpace`。
pub struct SharedTrustedLargeObjectSpace {
  /// 继承体。
  pub old: OldLargeObjectSpace,
}

impl Deref for SharedTrustedLargeObjectSpace {
  type Target = OldLargeObjectSpace;
  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.old
  }
}

impl DerefMut for SharedTrustedLargeObjectSpace {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.old
  }
}

impl SharedTrustedLargeObjectSpace {
  /// 原版 h:156 + cc:436-437 — OldLargeObjectSpace(heap, SHARED_TRUSTED_LO_SPACE)。
  #[must_use]
  pub fn new(heap: *mut Heap) -> Self {
    Self {
      old: OldLargeObjectSpace::new_with_id(heap, AllocationSpace::SharedTrustedLoSpace),
    }
  }
}

// h:26-27 前向声明 `class Isolate; class LocalHeap;` — Isolate 经 env 桥不落空型
// (memory_allocator.rs:81-84 同法), LocalHeap 复用 main_allocator.rs:50 承接位 (use 段)。

// ══════════════════════════════════════════════════════════════════════════
// 不变式自查 (编译期)
// ══════════════════════════════════════════════════════════════════════════

// 六支 wrapper ctor (cc:324-328/414-415/433-440) 的 id 落点须与 AllocationSpace 枚举序
// 一致 (本文件 const 锚; 迭代序上界 LAST_MUTABLE_SPACE = TrustedLoSpace 的锚在
// spaces.rs 不变式区)。
const _: () = assert!(
  AllocationSpace::NewLoSpace as u8 + 1 == AllocationSpace::LoSpace as u8
    && AllocationSpace::LoSpace as u8 + 1 == AllocationSpace::CodeLoSpace as u8
    && AllocationSpace::CodeLoSpace as u8 + 1 == AllocationSpace::SharedLoSpace as u8
    && AllocationSpace::SharedLoSpace as u8 + 1 == AllocationSpace::SharedTrustedLoSpace as u8
    && AllocationSpace::SharedTrustedLoSpace as u8 + 1 == AllocationSpace::TrustedLoSpace as u8
);

// ══════════════════════════════════════════════════════════════════════════
// 推迟台账 (真实耦合面; 双行号 + 解锁前置; 形态对齐 memory_allocator.rs 文末台账)
// ══════════════════════════════════════════════════════════════════════════
// 1. 分配路径 AllocateRaw 族 — h:136-139 (Old 四参, cc:104-141) 与 h:163-164 (New,
//    cc:333-369) 函数体 todo!。HCR1-F6 整改: 两处纯转发漏填已清零 — Old 三参 (h:129-130,
//    cc:98-102) 与 Code 三参 (h:183-184, cc:417-422) 现为单行直调本文件
//    allocate_raw_executability, 域参数 NOT_EXECUTABLE/EXECUTABLE 与原版逐字对齐
//    (转发终抵 todo 体, 运行时行为不变, 但转发形已如实落地)。原因:
//    Heap::ShouldExpand{Old,Young}Generation* / CanExpandOldGeneration /
//    StartIncrementalMarkingIfAllocationLimitIsReached / GCFlagsForIncrementalMarking /
//    marking_state()->TryMarkAndAccountLiveBytes / incremental_marking()->black_allocation /
//    NotifyOldGenerationExpansion / v8_flags.sticky_mark_bits / minor_ms 支 ClearLiveness
//    组合 — Heap 本体与 GC 阶段状态机他票。cc:108-109 ALIGN_TO_ALLOCATION_ALIGNMENT 与
//    cc:134 InitializationMemoryFence 随体接入 (后者面已落 memory_chunk.rs:303)。
// 2. AllocateLargePage — h:105-106 声明 + cc:143-164 体 todo!。原因:
//    Heap::heap_expansion_mutex (cc:146) / IsOldGenerationExpansionAllowed (cc:148-149) /
//    MemoryAllocator::AllocateLargePage (memory_allocator.rs 推迟台账 1) 未落地;
//    cc:156 DCHECK_GE(area_size, object_size) 位随体接入。解锁前置: HC-2b 台账 1 填体 +
//    Heap 整合票。
// 3. PromoteNewLargeObject — h:132 声明 + cc:173-186 体 todo!。原因: page->owner()
//    反解为 LargeObjectSpace* 并调其 RemovePage (页实体面他票) +
//    SetOldGenerationPageFlags(incremental_marking()->marking_mode()) GC 阶段状态机;
//    cc:174-180 DEBUG 断言区 (owner_identity/is_large/IsFromPage/IsToPage) 随体承接。
// 4. Verify — h:87-88 声明 + cc:280-311 (`#ifdef VERIFY_HEAP`): 目标配置展开侧剥除 →
//    不落地 (memory_allocator 台账 20 声明面清单同法); SpaceVerificationVisitor/
//    DYNAMICALLY_SIZED_HEAP_OBJECT_LIST 实体亦属他票, 下波若开 VERIFY 配置再落。
// 5. Print — h:91-93 + cc:313-321 + 原版 h:113 (spaces.h DEBUG 纯虚): 覆写已归位为
//    impl Space 的 print(&mut self) trait 槽形 (HCR1-F3; spaces.rs 台账 12 双记), 体
//    todo! (DEBUG 配置位)。原因: i::Print(HeapObject, ostream&) (utils/ostreams.h 他票)。
// 6. LOG 事件 — cc:65-67 DeleteEvent("LargeObjectChunk", …)。原因: v8-logging crate
//    不在本 crate 依赖 (memory_allocator 台账 14 同型); TearDown 体已落, 仅剥该语句不复活。
// 7. 运行时桥槽 — [`LargePageOps`] 17 槽 (页实体 large-page.h/mutable-page.h/base-page.h
//    + HeapObject::Size 与 IsFreeSpaceOrFiller 属 v8-objects-core 待填面) 与
//    [`LargeSpaceRunEnv`] 8 槽 (MemoryAllocator::Free/PartialFreeMemory/ReleaseDelayedPages
//    = memory_allocator.rs 推迟台账 7/9; CreateFillerObjectAt/IsAllocationObserverActive/
//    IsMarking = Heap/增量标记他票; page_owner_of_object = BasePage::FromHeapObject(Isolate)
//    他票; RemoveCodeMemoryChunk = Isolate 链他票)。落地面函数体已 1:1, 实体落地后将桥
//    实现改为直调 (无签名变更面: env 即宿主适配层)。
// 8. 析构与 Space 实现槽 — h:38 `~LargeObjectSpace() { TearDown(); }` 与 h:54/66-67 虚覆写
//    的"以 Space* 消费"位 (heap.cc lo_space() 族) 属 Heap 整合票; 本票 wrapper 类
//    (Old/New/Code/Shared/Trusted/SharedTrusted) 未另实现 Space trait (Deref 继承面已
//    覆盖值语义, trait 对象槽待 Heap 票以 enum_dispatch 或单态收敛, 禁 dyn 先例)。
// 9. h:112-116 重入锁消费 — cc:146-147 (expansion_guard) 与 cc:158-161
//    (RecursiveMutexGuard + AddPage) 在台账 2 todo 体内; allocation_mutex 字段已落,
//    填体时锁位勿漏。
// 10. 票面"LargeObject/block 链表/AllocatedBytes/CommittedBytes"旧版叙事 — 本仓实态
//     无该面 (申报见模块头); large_object 页链即 PageList+next_page, AllocatedBytes 面
//     在 AllocationStats (allocation.rs 已落地)。
