//! 原版 src/zone/zone.h/cc 转写
//!
//! Zone 支持小内存块的极快分配; 不能单独释放某个块, 只能整体回收。
//! 内在非线程安全 (原版注释), 生命周期用 &'z 引用参数化替代裸 ZonePointer。
//!
//! D4 (soundness): 回滚类入口 (Reset/Restore/ZoneScope Drop) 统一收紧为
//! &mut self —— 借出中的 &'z mut 分配引用与段回卷复用在类型层面互斥,
//! 同一地址二次交出 &mut 的别名形态不再可表达。

use core::marker::PhantomData;
#[cfg(feature = "precise-zone-stats")]
use std::cell::{Ref, RefCell};
use std::{
  cell::Cell,
  fmt,
  mem::{self, MaybeUninit},
  ptr::{self, NonNull},
  slice,
  sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
  },
};

#[cfg(feature = "precise-zone-stats")]
use crate::zone::{accounting::zone_stats_enabled, type_stats::TypeStats};
use crate::{
  error::{Error, Result},
  internal::bits::{ALIGNMENT, KB, align_down, align_up},
  kernel::chunk::Chunk,
  zone::{
    accounting::AccountingAllocator,
    segment::{Segment, ZAP_DEAD_BYTE},
  },
};

/// 原版 kMinimumSegmentSize
const MIN_SEGMENT_SIZE: usize = 8 * KB;
/// 原版 kMaximumSegmentSize
const MAX_SEGMENT_SIZE: usize = 32 * KB;
/// 原版 kSegmentOverhead = sizeof(Segment) + kAlignmentInBytes
const SEGMENT_OVERHEAD: usize = mem::size_of::<Segment>() + ALIGNMENT;
/// 原版 new_size > INT_MAX 上限
const MAX_NEW_SIZE: usize = i32::MAX as usize;

/// arena 主体: chunk 头 + bump 指针 + 段链
pub struct Zone<'a> {
  /// 当前可分配区间 [position, limit)
  chunk: Cell<Chunk>,
  allocator: &'a AccountingAllocator,
  /// LIFO 段链头 (最新版)
  segment_head: Cell<*mut Segment>,
  name: &'a str,
  /// 原版 sealed_ (仅 ASan 路径读取, 此处保留语义)
  sealed: Cell<bool>,
  /// 当前头段的基准分配位置 (allocation_size 的 extra 分量基准)
  position_base: Cell<usize>,
  /// 已提交段的使用字节数 (头段余量由 position 推出)
  allocation_size: Arc<AtomicUsize>,
  /// OS 侧已分配的段总字节数
  segment_bytes_allocated: Arc<AtomicUsize>,
  /// V8_ENABLE_PRECISE_ZONE_STATS: 含释放的分配总量
  allocation_size_for_tracing: Arc<AtomicUsize>,
  /// V8_ENABLE_PRECISE_ZONE_STATS: Delete 归还总量
  freed_size_for_tracing: Arc<AtomicUsize>,
  /// V8_ENABLE_PRECISE_ZONE_STATS: 逐 TypeTag 分配账 (type-stats.h)
  #[cfg(feature = "precise-zone-stats")]
  type_stats: RefCell<TypeStats>,
}

/// arena 语义: 经 &'z self 交出 &'z mut 槽位是 Zone bump 分配器的核心设计
/// (对应原版 Zone::Allocate 返回裸内存), 不构成可变性违背;
/// 回滚类入口 (reset/restore/ZoneScope) 已收紧 &mut, 'z 借出与回滚互斥
impl<'a> Zone<'a> {
  /// 原版 Zone(AccountingAllocator*, name)
  pub fn new(allocator: &'a AccountingAllocator, name: &'a str) -> Zone<'a> {
    let zone = Zone {
      chunk: Cell::new(Chunk::EMPTY),
      allocator,
      segment_head: Cell::new(ptr::null_mut()),
      name,
      sealed: Cell::new(false),
      position_base: Cell::new(0),
      allocation_size: Arc::new(AtomicUsize::new(0)),
      segment_bytes_allocated: Arc::new(AtomicUsize::new(0)),
      allocation_size_for_tracing: Arc::new(AtomicUsize::new(0)),
      freed_size_for_tracing: Arc::new(AtomicUsize::new(0)),
      #[cfg(feature = "precise-zone-stats")]
      type_stats: RefCell::new(TypeStats::default()),
    };
    allocator.trace_zone_creation(&zone);
    zone
  }

  /// 原版 name()
  pub fn name(&self) -> &'a str {
    self.name
  }

  /// 原版 allocator()
  pub fn allocator(&self) -> &'a AccountingAllocator {
    self.allocator
  }

  /// 原版 segment_bytes_allocated()
  pub fn segment_bytes_allocated(&self) -> usize {
    self.segment_bytes_allocated.load(Ordering::Relaxed)
  }

  /// 原版 allocation_size(): 精确用量, 仅属主线程可读
  pub fn allocation_size(&self) -> usize {
    let head = self.segment_head.get();
    let extra = if head.is_null() {
      0
    } else {
      self
        .chunk
        .get()
        .position_addr()
        .saturating_sub(self.position_base.get())
    };
    self.allocation_size.load(Ordering::Relaxed) + extra
  }

  /// 原版 allocation_size_for_tracing()
  pub fn allocation_size_for_tracing(&self) -> usize {
    if cfg!(feature = "precise-zone-stats") {
      self.allocation_size_for_tracing.load(Ordering::Relaxed)
    } else {
      self.allocation_size.load(Ordering::Relaxed)
    }
  }

  /// 原版 freed_size_for_tracing()
  pub fn freed_size_for_tracing(&self) -> usize {
    if cfg!(feature = "precise-zone-stats") {
      self.freed_size_for_tracing.load(Ordering::Relaxed)
    } else {
      0
    }
  }

  /// 原版 type_stats() (仅 V8_ENABLE_PRECISE_ZONE_STATS 构建存在)
  #[cfg(feature = "precise-zone-stats")]
  pub fn type_stats(&self) -> Ref<'_, TypeStats> {
    self.type_stats.borrow()
  }

  /// 原版 Seal()
  pub fn seal(&self) {
    self.sealed.set(true);
  }

  /// 原版 sealed_ 的读取口 (ASan 路径语义保留)
  pub fn is_sealed(&self) -> bool {
    self.sealed.get()
  }

  /// 原版 Allocate<TypeTag>(size) 公共路径: 对齐 + 精确统计记账 + bump。
  /// instance_size 对应 sizeof(TypeTagForSizeof); void/T[] 不完整类型
  /// 取 sizeof(char)=1 (type-stats.h kIsIncomplete 分支)
  fn allocate_tagged<Tag: ?Sized>(
    &self,
    size: usize,
    instance_size: usize,
    _tag: PhantomData<Tag>,
  ) -> Result<*mut u8> {
    // kIsIncomplete 口径: sizeof(TypeTagForSizeof) 至少为 sizeof(char)=1
    debug_assert!(instance_size >= 1);
    if size > MAX_NEW_SIZE {
      return Err(Error::OutOfMemory { requested: size });
    }
    let size = align_up(size, ALIGNMENT);
    if cfg!(feature = "precise-zone-stats") {
      self
        .allocation_size_for_tracing
        .fetch_add(size, Ordering::Relaxed);
    }
    #[cfg(feature = "precise-zone-stats")]
    if zone_stats_enabled() {
      self
        .type_stats
        .borrow_mut()
        .add_allocated::<Tag>(size, instance_size);
    }
    if size == 0 {
      // 原版 zone.h:66-69: 0 不大于余量, 走 AllocateUnchecked 返回
      // 段内 position_ 真址; 仅空 zone 兜底 dangling
      let head = self.segment_head.get();
      if !head.is_null() {
        return Ok(self.chunk.get().position);
      }
      return Ok(ptr::dangling_mut::<u64>().cast::<u8>());
    }
    // 原版 V8_UNLIKELY(size > limit_ - position_) 慢路径判断
    let mut chunk = self.chunk.get();
    if let Some(addr) = chunk.try_bump(size) {
      self.chunk.set(chunk);
      Ok(addr)
    } else {
      self.expand_and_allocate(size)
    }
  }

  /// 原版 Allocate<void>(size): 未指定 TypeTag 的字节分配
  pub(crate) fn allocate_bytes(&self, size: usize) -> Result<*mut u8> {
    self.allocate_tagged::<()>(size, 1, PhantomData)
  }

  /// 原版 Allocate<TypeTag>: 公开安全形态, 返回对齐字节块
  pub fn allocate_bytes_ref(&self, size: usize) -> Result<&mut [MaybeUninit<u8>]> {
    let ptr = self.allocate_bytes(size)?;
    // SAFETY: allocate_bytes 返回段内 >= size 的合法可写区间
    Ok(unsafe { slice::from_raw_parts_mut(ptr.cast::<MaybeUninit<u8>>(), size) })
  }

  /// 原版 New<T>(args...): 分配并构造 (Rust 以 move 值代替 Args...)
  pub fn make<T>(&self, value: T) -> Result<&mut T> {
    let align = mem::align_of::<T>();
    if align > ALIGNMENT {
      return Err(Error::UnsupportedAlignment { alignment: align });
    }
    if mem::size_of::<T>() == 0 {
      // ZST 不占段空间, mem::forget 防止参数 value 提前析构导致 UAF
      mem::forget(value);
      return Ok(unsafe { &mut *ptr::dangling_mut::<T>() });
    }
    // 原版 New<T> → Allocate<T>(sizeof(T)), TypeTag = T
    let ptr = self.allocate_tagged::<T>(mem::size_of::<T>(), mem::size_of::<T>(), PhantomData)?;
    let typed = ptr.cast::<T>();
    // SAFETY: 区间来自本 zone, 未被他用; 'z 借出保证 arena 生命周期
    unsafe { ptr::write(typed, value) };
    // SAFETY: 同上, 且返回引用不与 self 的其他借用重叠
    Ok(unsafe { &mut *typed })
  }

  /// 原版 AllocateArray<T>: length 个未初始化 T
  pub fn allocate_array<T>(&self, length: usize) -> Result<&mut [MaybeUninit<T>]> {
    let elem = mem::size_of::<T>();
    let align = mem::align_of::<T>();
    if align > ALIGNMENT {
      return Err(Error::UnsupportedAlignment { alignment: align });
    }
    if elem == 0 {
      // ZST 数组不占段空间 (Rust 特有形态, 原版 sizeof(T)==0 不存在)
      let ptr = ptr::dangling_mut::<MaybeUninit<T>>();
      return Ok(unsafe { slice::from_raw_parts_mut(ptr, length) });
    }
    // 原版 CHECK_LT(length, size_max / sizeof(T)) 溢出防护;
    // TypeTag = T[] (不完整类型, instance_size 取 sizeof(char)=1);
    // length == 0 与原版一致仍走 Allocate(0) 记账并返回段内 position
    let size = length.checked_mul(elem).ok_or(Error::SizeOverflow)?;
    let ptr = self.allocate_tagged::<[T]>(size, 1, PhantomData)?;
    let typed = ptr.cast::<MaybeUninit<T>>();
    // SAFETY: 区间由本 zone 分配, 长度匹配; MaybeUninit 无需初始化
    Ok(unsafe { slice::from_raw_parts_mut(typed, length) })
  }

  /// 原版 NewVector<T>(length): 以 default 值构造数组
  pub fn make_array_default<T: Default>(&self, length: usize) -> Result<&mut [T]> {
    let storage = self.allocate_array::<T>(length)?;
    for slot in storage.iter_mut() {
      slot.write(T::default());
    }
    let (ptr, len) = (storage.as_mut_ptr().cast::<T>(), length);
    // SAFETY: 全部 slot 已写入 T
    Ok(unsafe { slice::from_raw_parts_mut(ptr, len) })
  }

  /// 原版 NewVector<T>(length, value): 以 value 填充数组
  pub fn make_array_with<'z, T: Clone>(&'z self, length: usize, value: &T) -> Result<&'z mut [T]> {
    let storage = self.allocate_array::<T>(length)?;
    for slot in storage.iter_mut() {
      slot.write(value.clone());
    }
    let (ptr, len) = (storage.as_mut_ptr().cast::<T>(), length);
    // SAFETY: 全部 slot 已写入 T
    Ok(unsafe { slice::from_raw_parts_mut(ptr, len) })
  }

  /// 原版 CloneVector(base::Vector<T>): 复制构造数组
  pub fn clone_vector<'z, T: Clone>(&'z self, src: &[T]) -> Result<&'z mut [T]> {
    let storage = self.allocate_array::<T>(src.len())?;
    for (slot, item) in storage.iter_mut().zip(src.iter()) {
      slot.write(item.clone());
    }
    let (ptr, len) = (storage.as_mut_ptr().cast::<T>(), src.len());
    // SAFETY: 全部 slot 已写入 T
    Ok(unsafe { slice::from_raw_parts_mut(ptr, len) })
  }

  /// 原版 Delete<TypeTag>/DeleteArray<T>: 归还仅做记账 (release 构建不回收内存)
  pub(crate) fn delete<T>(&self, pointer: *mut T, count: usize) {
    debug_assert!(!pointer.is_null());
    debug_assert_ne!(count, 0);
    let Some(raw_size) = count.checked_mul(mem::size_of::<T>()) else {
      return;
    };
    if raw_size > MAX_NEW_SIZE {
      return;
    }
    let size = align_up(raw_size, ALIGNMENT);
    if cfg!(feature = "precise-zone-stats") {
      self
        .freed_size_for_tracing
        .fetch_add(size, Ordering::Relaxed);
    }
    // 原版 DeleteArray<T> 的 TypeTag = T[]
    #[cfg(feature = "precise-zone-stats")]
    if zone_stats_enabled() {
      self.type_stats.borrow_mut().add_deallocated::<[T]>(size);
    }
    // 原版 DEBUG memset zap (zone.h:90-93)。Miri Stacked Borrows 下 poison 写
    // 会使同段存活派生 tag 出栈 (C++ DEBUG memset 无借用追踪对应物, 语义为
    // 纯诊断填充), miri 构建跳过; freed 记账不受影响
    if cfg!(all(debug_assertions, not(miri))) {
      // SAFETY: 调用方保证区间属于本 zone 且不再读取 (原版 DEBUG memset zap)
      unsafe { ptr::write_bytes(pointer.cast::<u8>(), ZAP_DEAD_BYTE, size) };
    }
  }

  /// 原版 Reset(): 保留最后一段并复用。
  /// D4 修复: 收紧 &mut self —— 段回卷复用与借出中的 &'z mut 引用互斥;
  /// position 回卷到 RoundUp(keep->start()) 恢复原版语义 (zone.cc:89),
  /// 不再为避免别名而保留高水位
  pub fn reset(&mut self) {
    let keep = self.segment_head.get();
    if keep.is_null() {
      return;
    }
    let keep_total = unsafe { (*keep).total_size() };
    // SAFETY: keep 为本 zone 持有的有效段, 摘下 keep
    let remaining_head = unsafe {
      let next = (*keep).next();
      (*keep).set_next(ptr::null_mut());
      next
    };
    self.segment_head.set(remaining_head);
    self.chunk.set(Chunk::EMPTY);
    self.position_base.set(0);
    self.delete_all();
    self.allocator.trace_zone_creation(self);
    // SAFETY: keep 摘链后仍为本 zone 持有的有效段
    unsafe {
      // 原版: keep->ZapContents() 调试填充后复用 (zone.cc:86)
      Segment::zap_contents(keep);
      let start = (*keep).aligned_start();
      self.segment_head.set(keep);
      self
        .segment_bytes_allocated
        .store(keep_total, Ordering::Relaxed);
      self.chunk.set(Chunk::from_segment(keep));
      self.position_base.set(start);
    }
    debug_assert_eq!(self.allocation_size(), 0);
    debug_assert_eq!(self.segment_bytes_allocated(), keep_total);
  }

  /// 原版 Snapshot()
  pub fn snapshot(&self) -> ZoneSnapshot {
    let chunk = self.chunk.get();
    ZoneSnapshot {
      zone_address: ptr::from_ref(self) as usize,
      position_base: self.position_base.get(),
      allocation_size_for_tracing: self.allocation_size_for_tracing.load(Ordering::Relaxed),
      freed_size_for_tracing: self.freed_size_for_tracing.load(Ordering::Relaxed),
      allocation_size: self.allocation_size.load(Ordering::Relaxed),
      segment_bytes_allocated: self.segment_bytes_allocated.load(Ordering::Relaxed),
      position: chunk.position_addr(),
      limit: chunk.limit as usize,
      segment_head: self.segment_head.get(),
    }
  }

  /// 原版 Contains(ptr) (DEBUG)
  pub fn contains(&self, ptr: *const ()) -> bool {
    let address = ptr as usize;
    let mut segment = self.segment_head.get();
    while !segment.is_null() {
      // SAFETY: 段链由本 zone 持有
      let seg = unsafe { &*segment };
      if seg.contains_address(address) {
        return true;
      }
      segment = seg.next();
    }
    false
  }

  /// 原版 Zone::DeleteAll (私有; Drop/Reset 调用)
  pub(crate) fn delete_all(&self) {
    let mut current = self.segment_head.get();
    if !current.is_null() {
      self
        .allocation_size
        .store(self.allocation_size(), Ordering::Relaxed);
      self.segment_head.set(ptr::null_mut());
    }
    self.allocator.trace_zone_destruction(self);
    while !current.is_null() {
      // SAFETY: 段链由本 zone 持有, 逐段归还分配器
      let next = unsafe { (*current).next() };
      let total = unsafe { (*current).total_size() };
      self
        .segment_bytes_allocated
        .fetch_sub(total, Ordering::Relaxed);
      self.release_segment(current);
      current = next;
    }
    self.chunk.set(Chunk::EMPTY);
    self.position_base.set(0);
    self.allocation_size.store(0, Ordering::Relaxed);
    if cfg!(feature = "precise-zone-stats") {
      self.allocation_size_for_tracing.store(0, Ordering::Relaxed);
    }
  }

  /// 原版 ReleaseSegment: 无簿记直接归还 (ASan unpoison 略)
  fn release_segment(&self, segment: *mut Segment) {
    if let Some(non_null) = NonNull::new(segment) {
      // SAFETY: 段来自本 zone 的链, 仅归还一次
      unsafe { self.allocator.return_segment(non_null) };
    }
  }

  /// 原版 Expand(size): 高水位策略扩段
  #[inline(never)]
  fn expand(&self, size: usize) -> Result<()> {
    debug_assert_eq!(size, align_down(size, ALIGNMENT));
    debug_assert!(self.chunk.get().remaining() < size);
    let head = self.segment_head.get();
    let old_size = if head.is_null() {
      0
    } else {
      unsafe { (*head).total_size() }
    };
    // 原版高水位: new = size + 2*old, 夹在 [8KB, max(32KB, 请求+overhead)]
    let new_size_no_overhead = size.wrapping_add(old_size.wrapping_shl(1));
    let mut new_size = SEGMENT_OVERHEAD.wrapping_add(new_size_no_overhead);
    let min_new_size = SEGMENT_OVERHEAD
      .checked_add(size)
      .ok_or(Error::OutOfMemory { requested: size })?;
    // 原版溢出即 FatalProcessOutOfMemory
    if new_size_no_overhead < size
      || new_size.wrapping_sub(SEGMENT_OVERHEAD) != new_size_no_overhead
    {
      return Err(Error::OutOfMemory { requested: size });
    }
    if new_size < MIN_SEGMENT_SIZE {
      new_size = MIN_SEGMENT_SIZE;
    } else if new_size >= MAX_SEGMENT_SIZE {
      // D1 修复: 原版 zone.cc:168-174 该分支为
      // new_size = std::max({min_new_size, kMaximumSegmentSize}),
      // 丢弃超出 32K 的高水位值 (20K 请求得 32768 段而非 36896+overhead)
      new_size = min_new_size.max(MAX_SEGMENT_SIZE);
    }
    if new_size > MAX_NEW_SIZE {
      return Err(Error::OutOfMemory {
        requested: new_size,
      });
    }
    let segment = self
      .allocator
      .allocate_segment(new_size)
      .ok_or(Error::OutOfMemory {
        requested: new_size,
      })?;
    // SAFETY: 段刚由分配器构造, 头字段有效
    unsafe {
      let seg = segment.as_ref();
      self
        .segment_bytes_allocated
        .fetch_add(seg.total_size(), Ordering::Relaxed);
      seg.set_zone(self);
      seg.set_next(head);
      // 提交头段用量, 保证 tracing 看到含头段的值
      self
        .allocation_size
        .store(self.allocation_size(), Ordering::Relaxed);
      self.segment_head.set(segment.as_ptr());
      self.position_base.set(seg.aligned_start());
      self.allocator.trace_allocate_segment(seg);
      self.chunk.set(Chunk::from_segment(segment.as_ptr()));
    }
    Ok(())
  }

  /// 原版 ExpandAndAllocate(size) (慢路径)
  #[inline(never)]
  fn expand_and_allocate(&self, size: usize) -> Result<*mut u8> {
    self.expand(size)?;
    let mut chunk = self.chunk.get();
    let addr = chunk
      .try_bump(size)
      .ok_or(Error::OutOfMemory { requested: size })?;
    self.chunk.set(chunk);
    Ok(addr)
  }

  /// tracing 视图: 与活跃区统计共享的计数句柄
  pub(crate) fn allocation_size_view(&self) -> &Arc<AtomicUsize> {
    &self.allocation_size
  }

  pub(crate) fn segment_bytes_view(&self) -> &Arc<AtomicUsize> {
    &self.segment_bytes_allocated
  }

  pub(crate) fn freed_size_view(&self) -> &Arc<AtomicUsize> {
    &self.freed_size_for_tracing
  }
}

impl Drop for Zone<'_> {
  /// 原版 ~Zone(): DeleteAll 并断言段已清零
  fn drop(&mut self) {
    self.delete_all();
    debug_assert_eq!(self.segment_bytes_allocated(), 0);
  }
}

// SAFETY: Zone 独占拥有其段链与 chunk 状态; 段内存由全局分配器分配, 可在任意线程安全释放;
// 借用的 AccountingAllocator 为 Sync (基于原子量与 Mutex).
// Zone 包含 Cell, 因而保持 !Sync, 禁止跨线程并发共享 (符合原版
// "The implementation is inherently not thread safe. Do not use from multi-threaded code").
unsafe impl<'a> Send for Zone<'a> {}

/// 原版 ZoneSnapshot: 记录 zone 分配状态, 可回滚
#[derive(Clone, Copy)]
pub struct ZoneSnapshot {
  zone_address: usize,
  position_base: usize,
  allocation_size_for_tracing: usize,
  freed_size_for_tracing: usize,
  allocation_size: usize,
  segment_bytes_allocated: usize,
  position: usize,
  limit: usize,
  segment_head: *mut Segment,
}

// SAFETY: ZoneSnapshot 仅记录段指针与用量标量快照, 不包含共享可变状态
unsafe impl Send for ZoneSnapshot {}

impl ZoneSnapshot {
  /// 原版 ZoneSnapshot::Restore
  pub fn restore(&self, zone: &mut Zone<'_>) -> Result<()> {
    if self.zone_address != ptr::from_ref(&*zone) as usize {
      return Err(Error::WrongZone);
    }
    let mut check = zone.segment_head.get();
    let mut reachable = false;
    while !check.is_null() {
      if check == self.segment_head {
        reachable = true;
        break;
      }
      // SAFETY: 段链由目标 zone 持有
      check = unsafe { (*check).next() };
    }
    if !reachable && !self.segment_head.is_null() {
      return Err(Error::WrongZone);
    }

    let mut current = zone.segment_head.get();
    while current != self.segment_head {
      debug_assert!(!current.is_null());
      // SAFETY: 预检已保证 snapshot.segment_head 可达, 链中段均有效
      let next = unsafe { (*current).next() };
      let total = unsafe { (*current).total_size() };
      zone
        .segment_bytes_allocated
        .fetch_sub(total, Ordering::Relaxed);
      zone.release_segment(current);
      current = next;
    }
    zone
      .allocation_size
      .store(self.allocation_size, Ordering::Relaxed);
    zone
      .segment_bytes_allocated
      .store(self.segment_bytes_allocated, Ordering::Relaxed);
    zone.position_base.set(self.position_base);
    zone.segment_head.set(self.segment_head);
    if self.segment_head.is_null() {
      zone.chunk.set(Chunk::EMPTY);
    } else {
      // SAFETY: 预检已保证 snapshot.segment_head 在本 zone 段链可达且头有效
      let mut chunk = unsafe { Chunk::from_segment(self.segment_head) };
      // 原版 Restore 直接赋值绝对地址 position_ (zone.cc:243); 此处按快照偏移
      // 经段 raw 指针链重派生, 保留 provenance (SB 兼容), 地址语义不变
      // SAFETY: 快照 position 落在本段 [aligned_start, end] 区间内
      unsafe {
        chunk.position = chunk.position.add(self.position - self.position_base);
      }
      debug_assert_eq!(chunk.limit as usize, self.limit);
      zone.chunk.set(chunk);
    }
    if cfg!(feature = "precise-zone-stats") {
      zone
        .allocation_size_for_tracing
        .store(self.allocation_size_for_tracing, Ordering::Relaxed);
      zone
        .freed_size_for_tracing
        .store(self.freed_size_for_tracing, Ordering::Relaxed);
    }
    Ok(())
  }
}

/// 原版 ZoneScope: 作用域内 zone 内存的有效性边界, 析构时回滚
pub struct ZoneScope<'z, 'a> {
  zone: &'z mut Zone<'a>,
  snapshot: ZoneSnapshot,
}

impl<'z, 'a> ZoneScope<'z, 'a> {
  /// 原版 ZoneScope(Zone*)
  pub fn new(zone: &'z mut Zone<'a>) -> ZoneScope<'z, 'a> {
    let snapshot = zone.snapshot();
    ZoneScope { zone, snapshot }
  }

  /// 原版 zone_; 经受约束的可变再借用分发分配入口
  pub fn zone(&mut self) -> &mut Zone<'a> {
    &mut *self.zone
  }
}

impl Drop for ZoneScope<'_, '_> {
  fn drop(&mut self) {
    // 原版 snapshot_.Restore(zone_); 同 zone 回滚不可失败
    let result = self.snapshot.restore(&mut *self.zone);
    debug_assert!(result.is_ok());
  }
}

/// 原版 ZoneAllocationPolicy: 泛型数据结构自分配的策略句柄
#[derive(Clone, Copy)]
pub struct ZoneAllocationPolicy<'z, 'a> {
  zone: &'z Zone<'a>,
}

impl<'z, 'a> ZoneAllocationPolicy<'z, 'a> {
  /// 原版 explicit ZoneAllocationPolicy(Zone*)
  pub fn new(zone: &'z Zone<'a>) -> ZoneAllocationPolicy<'z, 'a> {
    ZoneAllocationPolicy { zone }
  }

  /// 原版 zone()
  pub fn zone(&self) -> &'z Zone<'a> {
    self.zone
  }

  /// 原版 AllocateArray<T, TypeTag>
  pub fn allocate_array<T>(&self, length: usize) -> Result<&'z mut [MaybeUninit<T>]> {
    self.zone.allocate_array::<T>(length)
  }

  /// 原版 DeleteArray<T, TypeTag>
  pub fn delete_array<T>(&self, pointer: *mut T, length: usize) {
    if length != 0 {
      self.zone.delete(pointer, length);
    }
  }
}

impl fmt::Debug for ZoneAllocationPolicy<'_, '_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("ZoneAllocationPolicy")
      .field("zone", &self.zone.name())
      .finish()
  }
}

impl PartialEq for ZoneAllocationPolicy<'_, '_> {
  fn eq(&self, other: &Self) -> bool {
    ptr::eq(self.zone, other.zone)
  }
}

impl Eq for ZoneAllocationPolicy<'_, '_> {}
