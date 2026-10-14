//! 堆内存分配器 (MemoryAllocator)。
//! 对应原版 v8/src/heap/memory-allocator.h (420 行) + memory-allocator.cc (832 行)。(HC-2b 落地)
//!
//! 目标配置 (与 tool/src/heap/memory-allocator.{h,cc}.js clang -E 展开侧同源):
//!   V8_COMPRESS_POINTERS + V8_ENABLE_SANDBOX + V8_EXTERNAL_CODE_SPACE, DEBUG/TSAN 关。
//!   展开侧实测: DCHECK/CHECK/V8_EXPORT_PRIVATE/V8_INLINE/V8_WARN_UNUSED_RESULT 记号 0 残留;
//!   cc:302-304/390-399/462-464/544-546/580-583 的 `#ifdef DEBUG` 消费区与 h:229-236/329-332/
//!   382-387 成员区在目标配置剥除 — 本模块以 `#[cfg(debug_assertions)]` 承接可执行块登记面
//!   (allocation.rs DEBUG 账目面先例)。
//!
//! 票面差异申报 (以本仓源实态为准, 主控汇报):
//! 票面所述 h 536 行/cc 716 行、h:117 `virtual MemoryRange AllocateMemoryRange`、
//! h:428-442 pool_ 成员 + kAllocatableSpaces/kAllSpaces 静态表、h:535 `friend class
//! MainAllocator` 均为旧版叙事; 本仓 memory-allocator 源实态为 h 420 行 / cc 832 行:
//! `class MemoryAllocator final` (h:41) 无 virtual, 无 AllocateMemoryRange 面, 无静态表;
//! friend 面为 heap::TestCodePageAllocatorScope / heap::TestMemoryAllocatorScope (h:414-415);
//! pool_ 成员实位 h:380。MainAllocator/MainHeapAllocator 在本源为上层独立类
//! (src/heap/main-allocator.{h,cc}, HC-2c), 与 MemoryAllocator 无 friend 关系。
//! 本票按实况 1:1 重映射落地。
//!
//! 消费的本仓已落地面 (只读):
//! - [`crate::memory_chunk::MemoryChunk`] (FromAddress/BaseAddress/alignment) ← cc:713 等;
//!   [`crate::memory_chunk::MemoryChunkLayout`] ← cc:186/240/626/645。
//! - [`crate::memory_pool::MemoryPool`] (GetCount/GetSharedCount/GetTotalCount/
//!   ReleaseImmediately/ReleaseOnTearDown) ← cc:54-92; [`crate::memory_pool::Reservation`]
//!   ← h:379 字段与 h:380 池对偶型承载 (memory_pool.rs 台账 6 同型)。
//! - [`crate::memory_pool::Isolate`] opaque 承接 (跨模块指针形参类型一致, h:33 先例)。
//! - [`v8_base::page_allocator::os_commit_page_size`] ← cc:677 `CommitPageSize()`。
//! - [`v8_flags::Flags`] ← cc:675-676 `v8_flags.v8_os_page_size` (free_list.rs 参数注入先例)。
//!
//! 未落地耦合的承接形: [`Reservation`]、[`ChunkContainsEnv`] (cc:720/731
//! MetadataNoIsolateCheck()->Contains)。页面/空间/Heap/VirtualMemory 构造/LOG/zapping 等
//! 耦合函数整体推迟, 文末推迟台账逐条双行号申报 (台账锚与 0 检臂见两转换脚本)。

use std::{
  collections::{BTreeSet, HashSet},
  ffi::c_void,
  ptr::{null, null_mut},
  sync::atomic::{AtomicUsize, Ordering},
};

use v8_base::{
  bits::which_power_of_two, logging::check_with_msg, macros::round_up_address,
  page_allocator::os_commit_page_size,
};
use v8_base_platform::Mutex;
use v8_common::{Address, AllocationSpace, Executability, K_NULL_ADDRESS, K_REGULAR_PAGE_SIZE, KB};
use v8_flags::Flags;

use crate::{
  memory_chunk::{K_INSTRUCTION_STREAM_HEADER_SIZE_DERIVED, MemoryChunk, MemoryChunkLayout},
  memory_pool::{Isolate, MemoryPool, Reservation},
};

// ══════════════════════════════════════════════════════════════════════════
// 前向声明承接 (原版 h:29-35 namespace heap 前向 + h:32-35 类前向)
// ══════════════════════════════════════════════════════════════════════════

/// 原版 h:33 `class Heap;` 的 opaque 承接 (实体 src/heap/heap.h 属他票)。
/// 本模块仅推迟台账内的 HandleAllocationFailure 等函数消费其方法面 → 指针位留台账。
pub enum Heap {}

/// 原版 include/v8-platform.h `v8::PageAllocator` 抽象类的 opaque 承接
/// (h:84-86 形参 + h:339-355 字段 + h:165-181 访问器)。
/// 差异申报: v8-base 的同名 `PageAllocator` trait 为 base:: 页分配器 (另一主, 不可混用);
/// v8:: 平台实体 (AllocatePages/FreePages 族) 属他票 (v8-utils allocation.rs:4-5 已登记) →
/// 本模块只透传指针并按 space 分派 (h:184-205)。
pub enum PageAllocator {}

/// 原版 h:21 mutable-page.h `MutablePage` 的 opaque 承接 (实体未落地, 他席面互斥)。
/// 本模块以指针形参/延迟页向量/DEBUG 登记表承接 (h:115/385/409), 从不解引用 →
/// 空 enum (memory_chunk.rs:27 BasePage 先例)。
pub enum MutablePage {}

/// 原版 spaces.h `NormalPage` 的 opaque 承接 (h:96/311/404 指针面)。
pub enum NormalPage {}

/// 原版 spaces.h `LargePage` 的 opaque 承接 (h:100/407 指针面)。
pub enum LargePage {}

// h:34 `class MemoryPool;` 由 crate::memory_pool 落地直用 (同型指针对偶);
// h:35 `class ReadOnlyPage;` 与 h:19-22 include 的 Space/BaseSpace/PagedSpace/
// LargeObjectSpace/ReadOnlySpace 仅推迟台账内签名消费 (h:100-116/158-161/259-312),
// Rust 无消费点 → 不落空型, 声明面入推迟台账。

// ══════════════════════════════════════════════════════════════════════════
// AllocationMode / FreeMode (原版 h:43-66)
// ══════════════════════════════════════════════════════════════════════════

/// 分配模式。原版 h:43-49 `enum class AllocationMode`。
/// C++ `enum class` 默认底层 int → `#[repr(i32)]`; 变体去 k 前缀 PascalCase
/// (同 allocation.rs AllocationOrigin 先例), 值不变 ABI 等价。
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AllocationMode {
  /// 常规分配路径, 不用池化/延迟页 (原版 h:44-45 kRegular, 值 0)。
  Regular = 0,
  /// 先试延迟页与池化页, 再分配新页 (原版 h:46-48 kTryDelayedAndPooled, 值 1)。
  TryDelayedAndPooled = 1,
}

/// 释放模式。原版 h:51-66 `enum class FreeMode` (h:54-65 注释逐条照录 doc)。
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FreeMode {
  /// 主线程立即释放 (原版 h:52-53 kImmediately, 值 0)。
  Immediately = 0,
  /// 允许入池; 池可能被禁用或配额已满而失败 (原版 h:54-56 kPool, 值 1)。
  Pool = 1,
  /// 延迟到 `ReleaseDelayedPages()` 释放; 期间页保持 Isolate-local,
  /// 供 pointers updating 等需要已释页暂存的调用方 (原版 h:57-61 kDelayThenRelease, 值 2)。
  DelayThenRelease = 2,
  /// 同 kDelayThenRelease, 但延迟后入池而非释放 (原版 h:62-65 kDelayThenPool, 值 3)。
  DelayThenPool = 3,
}

// ══════════════════════════════════════════════════════════════════════════
// 进程级提交页大小静态 (原版 h:68-79/411-412 + cc:33-34/674-680)
// ══════════════════════════════════════════════════════════════════════════

/// 原版 cc:33 `size_t MemoryAllocator::commit_page_size_ = 0;` (h:411 声明)。
/// 差异申报: 原版普通静态 size_t (InitializeOncePerProcess 单点写, 之后只读),
/// Rust 全局可变以 AtomicUsize 承接; 读写序 SeqCst (对齐 std::atomic 默认)。
static COMMIT_PAGE_SIZE: AtomicUsize = AtomicUsize::new(0);

/// 原版 cc:34 `size_t MemoryAllocator::commit_page_size_bits_ = 0;` (h:412 声明)。
static COMMIT_PAGE_SIZE_BITS: AtomicUsize = AtomicUsize::new(0);

// 原版 h:68 注释: Initialize page sizes field in V8::Initialize (进程级一次性) —
// 落点见 impl 的 [`MemoryAllocator::initialize_once_per_process`] (cc:674-680)。

// ══════════════════════════════════════════════════════════════════════════
// MemoryChunkAllocationResult (原版 h:239-250, private 嵌套 struct)
// ══════════════════════════════════════════════════════════════════════════

/// 原版 h:241-250 `struct MemoryChunkAllocationResult` — AllocateUninitializedChunk 的
/// 全数据面 (h:239-240 注释)。h:249 `VirtualMemory reservation` → [`Reservation`] trait
/// 承接 (memory_pool.rs 台账 6 同型)。原版 private + 友元消费 → Rust pub 承接
/// (memory_chunk 台账 11 先例)。实体生产方 (chunk-at 族) 见文末推迟台账 2。
#[derive(Debug)]
pub struct MemoryChunkAllocationResult<R: Reservation> {
  /// h:242 `void* chunk` — 未初始化的 MemoryChunk 头地址。
  pub chunk: *mut c_void,
  /// h:243-244 注释 + h:244 字段: 池化 chunk 复用时交回的 metadata 分配块。
  pub optional_metadata: *mut c_void,
  /// h:245 `size_t size`。
  pub size: usize,
  /// h:246 `size_t area_start`。
  pub area_start: usize,
  /// h:247 `size_t area_end`。
  pub area_end: usize,
  /// h:249 `VirtualMemory reservation`。
  pub reservation: R,
}

// ══════════════════════════════════════════════════════════════════════════
// ChunkContainsEnv (cc:720/731 MetadataNoIsolateCheck()->Contains 承接形)
// ══════════════════════════════════════════════════════════════════════════

/// 原版 cc:720/731 `chunk->MetadataNoIsolateCheck()->Contains(addr)` 的窄承接 trait。
/// MetadataNoIsolateCheck (memory-chunk-inl.h) 属他票 (memory_chunk.rs 推迟台账 1),
/// BasePage::Contains 实体未落地 → 由宿主提供该判据; 本模块 1:1 承担查找结构本身
/// (normal 集合命中 / large 有序集 upper_bound 前驱 + 边界 DCHECK)。
pub trait ChunkContainsEnv {
  /// cc:720/731 消费点: 该 chunk 的页 metadata 区间是否包含 addr。
  fn metadata_contains(&self, chunk: *const MemoryChunk, addr: Address) -> bool;
}

// ══════════════════════════════════════════════════════════════════════════
// MemoryAllocator (原版 h:37-416 `class MemoryAllocator final`)
// ══════════════════════════════════════════════════════════════════════════

/// 原版 h:37-40 注释: 为不同种类的页提供分配/释还基础设施, 位于上层 V8 spaces 与
/// OS 或 memory pool 之间; Isolate-local 设计 — 页永不跨 Isolate 复用。
///
/// 泛参 `R: Reservation` 对偶 h:379 `std::optional<VirtualMemory>` 字段与 h:380
/// `MemoryPool*` 的池类型 [`MemoryPool<R>`]。h:88-89 copy ctor/assign = delete →
/// Rust 不派生 Clone/Copy (台账 16)。h:41 `final` 无派生面 (本版本非抽象基类, 票面差异申报)。
#[derive(Debug)]
pub struct MemoryAllocator<R: Reservation> {
  /// 原版 h:334 `Isolate* isolate_`。
  pub isolate: *mut Isolate,
  /// 原版 h:336-339 注释 + 字段: 数据页分配器 (平台提供或 BoundedPageAllocator)。
  /// 原版取源 `isolate->page_allocator()` (cc:41) 推迟台账 17 → ctor 参数注入。
  pub data_page_allocator: *mut PageAllocator,
  /// 原版 h:341-342 `read_only_page_allocator_` — 取源
  /// `IsolateGroup::current()->read_only_page_allocator()` (cc:42-43) 推迟台账 17。
  pub read_only_page_allocator: *mut PageAllocator,
  /// 原版 h:344-349 注释 + 字段: 代码页分配器。
  pub code_page_allocator: *mut PageAllocator,
  /// 原版 h:351-355 注释 + 字段: trusted 页分配器 (沙箱启用时在沙箱外分配)。
  pub trusted_page_allocator: *mut PageAllocator,
  /// 原版 h:357-358 `size_t capacity_` — 最大可用字节数 (ctor 处 RoundUp, cc:46)。
  pub capacity: usize,
  /// 原版 h:360-361 `std::atomic<size_t> size_ = 0` — 已分配空间字节数。
  pub size_: AtomicUsize,
  /// 原版 h:362-363 `std::atomic<size_t> size_executable_ = 0` — 可执行部分字节数。
  pub size_executable_: AtomicUsize,
  /// 原版 h:365-373 注释 + 字段: [lowest, highest) 快速越界判定区间, 可执行/非可执行
  /// 分离记账 (低界初值 `static_cast<Address>(-1ll)`, 高界初值 kNullAddress)。
  pub lowest_not_executable_ever_allocated: AtomicUsize,
  /// 原版 h:374 `highest_not_executable_ever_allocated_{kNullAddress}`。
  pub highest_not_executable_ever_allocated: AtomicUsize,
  /// 原版 h:375-376 `lowest_executable_ever_allocated_{-1ll}`。
  pub lowest_executable_ever_allocated: AtomicUsize,
  /// 原版 h:377 `highest_executable_ever_allocated_{kNullAddress}`。
  pub highest_executable_ever_allocated: AtomicUsize,
  /// 原版 h:379 `std::optional<VirtualMemory> reserved_chunk_at_virtual_memory_limit_` —
  /// 地址空间末位 chunk 占位 (cc:137-139 注释: 防 top/limit 溢出比较)。
  pub reserved_chunk_at_virtual_memory_limit: Option<R>,
  /// 原版 h:380 `MemoryPool* pool_` — 可空 (无池配置; cc:55/77 等判空消费)。
  pub pool: *mut MemoryPool<R>,

  /// 原版 h:382-387 `#ifdef DEBUG` 区: 已分配可执行块登记表 (仅 DCHECK 用) + 专用互斥。
  /// 目标配置展开侧整体剥除 (台账锚: 展开侧 executable_memory_ 0 残留);
  /// Rust 以 `#[cfg(debug_assertions)]` 承接 (allocation.rs DEBUG 账目先例)。
  #[cfg(debug_assertions)]
  pub executable_memory: HashSet<*mut MutablePage>,
  /// 原版 h:386 `mutable base::Mutex executable_memory_mutex_`。
  #[cfg(debug_assertions)]
  pub executable_memory_mutex: Mutex,

  /// 原版 h:389-392 注释 + 字段: 保守栈扫描用的 normal 页登记。
  /// `unordered_set<const MemoryChunk*, base::hash<...>>` → HashSet (容器差异申报:
  /// 原版 base::hash 特化为地址截断哈希, Rust 默认指针哈希, 集合成员判据等价 —
  /// memory_pool 台账 9 先例; chunk_hash 面见 memory_chunk.rs 落地)。
  pub normal_pages: HashSet<*mut MemoryChunk>,
  /// 原版 h:393 `std::set<const MemoryChunk*> large_pages_` → BTreeSet
  /// (有序性必须承接: cc:721 upper_bound 前驱查找)。
  pub large_pages: BTreeSet<*mut MemoryChunk>,

  /// 原版 h:395 `mutable base::Mutex chunks_mutex_`。
  pub chunks_mutex: Mutex,

  /// 原版 h:397-404 注释 + 字段: 延迟后入池的常规页 (可被立即复用)。
  pub delayed_then_pooled_pages: Vec<*mut NormalPage>,
  /// 原版 h:405-407 字段 + 注释: 延迟后入池的大页 (不可复用, 原版未实现)。
  pub delayed_then_pooled_large_pages: Vec<*mut LargePage>,
  /// 原版 h:408-409 字段 + 注释: 延迟后释放的页 (无复用)。
  pub delayed_then_released_pages: Vec<*mut MutablePage>,
}

/// 原版 h:373/376 `static_cast<Address>(-1ll)` 低界初值 (usize::MAX)。
const ADDRESS_LOW_BOUND: Address = !0usize;

/// 原版 cc:46 `RoundUp(capacity, NormalPage::kPageSize)` 的 NormalPage::kPageSize 承接:
/// spaces.h 别名链 NormalPage::kPageSize = MemoryChunk::kPageSize = kRegularPageSize。
const K_NORMAL_PAGE_SIZE: usize = K_REGULAR_PAGE_SIZE;

impl<R: Reservation> MemoryAllocator<R> {
  // ── 进程级初始化 (h:68-79 + cc:674-680) ──

  /// 原版 h:69 声明 + cc:674-680 `InitializeOncePerProcess()`:
  /// `v8_flags.v8_os_page_size > 0` 时取 `旗标 * KB` (cc:675-676), 否则 `CommitPageSize()`
  /// (cc:677, v8_base::page_allocator::os_commit_page_size 承接)。
  /// cc:678 `CHECK(base::bits::IsPowerOfTwo(...))` 非 DCHECK → check_with_msg (v8 fatal)。
  /// cc:679 `WhichPowerOfTwo` → v8_base::bits::which_power_of_two。
  /// 差异申报: cc:675 `v8_flags` 全局 → `&Flags` 参数注入 (free_list.rs:24 先例)。
  pub fn initialize_once_per_process(flags: &Flags) {
    let commit_page_size = if flags.v8_os_page_size() > 0 {
      (flags.v8_os_page_size() as usize) * KB
    } else {
      os_commit_page_size()
    };
    check_with_msg(
      commit_page_size.is_power_of_two(),
      "cc:678 CHECK(base::bits::IsPowerOfTwo(commit_page_size_))",
    );
    COMMIT_PAGE_SIZE.store(commit_page_size, Ordering::SeqCst);
    COMMIT_PAGE_SIZE_BITS.store(
      which_power_of_two(commit_page_size as u64) as usize,
      Ordering::SeqCst,
    );
  }

  /// 原版 h:71-74 `V8_INLINE static intptr_t GetCommitPageSize()` — DCHECK_LT(0, ...)。
  #[inline]
  #[must_use]
  pub fn get_commit_page_size() -> isize {
    let size = COMMIT_PAGE_SIZE.load(Ordering::SeqCst) as isize;
    debug_assert!(0 < size, "h:72 DCHECK_LT(0, commit_page_size_)");
    size
  }

  /// 原版 h:76-79 `V8_INLINE static intptr_t GetCommitPageSizeBits()` — DCHECK_LT(0, ...)。
  #[inline]
  #[must_use]
  pub fn get_commit_page_size_bits() -> isize {
    let bits = COMMIT_PAGE_SIZE_BITS.load(Ordering::SeqCst) as isize;
    debug_assert!(0 < bits, "h:78 DCHECK_LT(0, commit_page_size_bits_)");
    bits
  }

  // ── 构造/拆除 (h:83-91 + cc:36-74) ──

  /// 原版 h:83-86 声明 + cc:36-52 ctor。
  ///
  /// 差异申报 (台账 17): cc:41 `isolate->page_allocator()` 与 cc:42-43
  /// `IsolateGroup::current()->read_only_page_allocator()` 属他票运行态 →
  /// data/read_only 两个 allocator 改为参数注入, 初始化列表顺序照录 cc:40-47。
  /// cc:46 `capacity_ = RoundUp(capacity, NormalPage::kPageSize)` →
  /// [`round_up_address`] + [`K_NORMAL_PAGE_SIZE`]。
  /// cc:48-51 `DCHECK_NOT_NULL ×4` (data/read_only/code/trusted) → debug_assert。
  #[must_use]
  pub fn new(
    isolate: *mut Isolate,
    code_page_allocator: *mut PageAllocator,
    trusted_page_allocator: *mut PageAllocator,
    page_pool: *mut MemoryPool<R>,
    max_capacity: usize,
    data_page_allocator: *mut PageAllocator,
    read_only_page_allocator: *mut PageAllocator,
  ) -> Self {
    debug_assert!(
      !data_page_allocator.is_null(),
      "cc:48 DCHECK_NOT_NULL(data_page_allocator_)"
    );
    debug_assert!(
      !read_only_page_allocator.is_null(),
      "cc:49 DCHECK_NOT_NULL(read_only_page_allocator_)"
    );
    debug_assert!(
      !code_page_allocator.is_null(),
      "cc:50 DCHECK_NOT_NULL(code_page_allocator_)"
    );
    debug_assert!(
      !trusted_page_allocator.is_null(),
      "cc:51 DCHECK_NOT_NULL(trusted_page_allocator_)"
    );
    Self {
      isolate,
      data_page_allocator,
      read_only_page_allocator,
      code_page_allocator,
      trusted_page_allocator,
      capacity: round_up_address(max_capacity, K_NORMAL_PAGE_SIZE),
      size_: AtomicUsize::new(0),
      size_executable_: AtomicUsize::new(0),
      lowest_not_executable_ever_allocated: AtomicUsize::new(ADDRESS_LOW_BOUND),
      highest_not_executable_ever_allocated: AtomicUsize::new(K_NULL_ADDRESS),
      lowest_executable_ever_allocated: AtomicUsize::new(ADDRESS_LOW_BOUND),
      highest_executable_ever_allocated: AtomicUsize::new(K_NULL_ADDRESS),
      reserved_chunk_at_virtual_memory_limit: None,
      pool: page_pool,
      #[cfg(debug_assertions)]
      executable_memory: HashSet::new(),
      #[cfg(debug_assertions)]
      executable_memory_mutex: Mutex::new(),
      normal_pages: HashSet::new(),
      large_pages: BTreeSet::new(),
      chunks_mutex: Mutex::new(),
      delayed_then_pooled_pages: Vec::new(),
      delayed_then_pooled_large_pages: Vec::new(),
      delayed_then_released_pages: Vec::new(),
    }
  }

  /// 原版 h:91 声明 + cc:54-74 `TearDown()`。
  ///
  /// cc:55-58: 有池 → `pool->ReleaseOnTearDown(isolate_)` + DCHECK_EQ(GetCount, 0);
  /// cc:61 `DCHECK_EQ(size_, 0u)` (spaces 须先于本类拆除, cc:60 注释) → debug_assert;
  /// cc:62-63 TODO(gc) 注释区 (size_executable 断言原版被注释掉) 照录不复活;
  /// cc:64 capacity_ = 0; cc:66-68 末位保留 chunk 的 Free (Reservation::free);
  /// cc:70-73 四个 allocator 指针置空。
  pub fn tear_down(&mut self) {
    // SAFETY: pool_ 为 ctor 注入的稳定指针或 null; cc:55 `if (auto* pool = memory_pool())`
    // 判空后独占使用 (拆除期无并发, h:39 Isolate-local 注释)。
    if let Some(pool) = unsafe { self.pool.as_mut() } {
      pool.release_on_teardown(self.isolate);
      debug_assert_eq!(
        pool.get_count(self.isolate),
        0,
        "cc:57 DCHECK_EQ(pool->GetCount, 0)"
      );
    }
    debug_assert_eq!(self.size(), 0, "cc:61 DCHECK_EQ(size_, 0u)");
    self.capacity = 0;
    if let Some(reserved) = self.reserved_chunk_at_virtual_memory_limit.as_mut() {
      reserved.free();
    }
    self.code_page_allocator = null_mut();
    self.data_page_allocator = null_mut();
    self.read_only_page_allocator = null_mut();
    self.trusted_page_allocator = null_mut();
  }

  // ── 池查询门面 (h:216-227 + cc:76-92) ──

  /// 本 isolate 的池化 chunk 数; 无池 → 0 (cc:77 三目)。
  #[must_use]
  pub fn get_pooled_chunks_count(&self) -> usize {
    // SAFETY: 同 tear_down; GetCount 为 &self 只读计数 (memory_pool.rs cc:393-395)。
    match unsafe { self.pool.as_ref() } {
      Some(pool) => pool.get_count(self.isolate),
      None => 0,
    }
  }

  /// 共享池化 chunk 数; 无池 → 0 (cc:81 三目)。
  #[must_use]
  pub fn get_shared_pooled_chunks_count(&self) -> usize {
    match unsafe { self.pool.as_ref() } {
      Some(pool) => pool.get_shared_count(),
      None => 0,
    }
  }

  /// 全部池化 chunk 数 (含他 isolate 缓存, h:222-224 注释); 无池 → 0 (cc:85 三目)。
  #[must_use]
  pub fn get_total_pooled_chunks_count(&self) -> usize {
    match unsafe { self.pool.as_ref() } {
      Some(pool) => pool.get_total_count(),
      None => 0,
    }
  }

  /// 立即释放本 isolate 全部池化 chunk (h:226-227 注释 + cc:88-92)。
  pub fn release_pooled_chunks_immediately(&mut self) {
    // SAFETY: 同 tear_down; ReleaseImmediately 需 &mut (memory_pool.rs cc:357-361)。
    if let Some(pool) = unsafe { self.pool.as_mut() } {
      pool.release_immediately(self.isolate);
    }
  }

  /// 原版 h:255 `MemoryPool* memory_pool()` (private 访问器, 友元面 pub 承接) —
  /// cc:55/77/400 等消费形态 `if (auto* pool = memory_pool())` 的判空源。
  #[must_use]
  pub fn memory_pool(&mut self) -> *mut MemoryPool<R> {
    self.pool
  }

  // ── 容量/边界查询 (h:120-152) ──

  /// 原版 h:120-121 `size_t Size() const` — 已分配空间字节数。
  #[inline]
  #[must_use]
  pub fn size(&self) -> usize {
    self.size_.load(Ordering::SeqCst)
  }

  /// 原版 h:123-124 `size_t SizeExecutable() const` — 可执行部分字节数。
  #[inline]
  #[must_use]
  pub fn size_executable(&self) -> usize {
    self.size_executable_.load(Ordering::SeqCst)
  }

  /// 原版 h:126-130 `size_t Available() const` — `capacity_ < size ? 0 : capacity_ - size`
  /// (三目即饱和减法等价形)。
  #[must_use]
  pub fn available(&self) -> usize {
    // h:129 `capacity_ < size_ ? 0 : capacity_ - size_` → saturating_sub。
    self.capacity.saturating_sub(self.size())
  }

  /// 原版 h:132-140 `V8_INLINE bool IsOutsideAllocatedSpace(Address) const` —
  /// 两个 executability 分支均越界才算越界 (h:134-136 注释: 保守, 允许假阴性不允许假阳性)。
  #[inline]
  #[must_use]
  pub fn is_outside_allocated_space(&self, address: Address) -> bool {
    self.is_outside_allocated_space_of(address, Executability::NotExecutable)
      && self.is_outside_allocated_space_of(address, Executability::Executable)
  }

  /// 原版 h:141-152 `IsOutsideAllocatedSpace(Address, Executability)` — switch 双支 +
  /// UNREACHABLE(); Rust 穷尽 match, UNREACHABLE 尾支展开侧空位不复活 (台账注)。
  #[inline]
  #[must_use]
  pub fn is_outside_allocated_space_of(&self, address: Address, executable: Executability) -> bool {
    match executable {
      Executability::NotExecutable => {
        address
          < self
            .lowest_not_executable_ever_allocated
            .load(Ordering::SeqCst)
          || address
            >= self
              .highest_not_executable_ever_allocated
              .load(Ordering::SeqCst)
      }
      Executability::Executable => {
        address < self.lowest_executable_ever_allocated.load(Ordering::SeqCst)
          || address
            >= self
              .highest_executable_ever_allocated
              .load(Ordering::SeqCst)
      }
    }
  }

  // ── 分配器访问器与 space 分派 (h:163-205) ──

  /// 原版 h:163-167 `data_page_allocator()` (注释: Guaranteed to be a valid pointer)。
  #[inline]
  #[must_use]
  pub fn data_page_allocator(&self) -> *mut PageAllocator {
    self.data_page_allocator
  }

  /// 原版 h:169-173 `code_page_allocator()`。
  #[inline]
  #[must_use]
  pub fn code_page_allocator(&self) -> *mut PageAllocator {
    self.code_page_allocator
  }

  /// 原版 h:175-181 `trusted_page_allocator()` (沙箱注释照录)。
  #[inline]
  #[must_use]
  pub fn trusted_page_allocator(&self) -> *mut PageAllocator {
    self.trusted_page_allocator
  }

  /// 原版 h:183-205 `page_allocator(AllocationSpace)` — 按空间分派 13 空间四支;
  /// switch 尾 UNREACHABLE() 由 Rust 穷尽 match 承接 (同 is_outside 注)。
  #[must_use]
  pub fn page_allocator(&self, space: AllocationSpace) -> *mut PageAllocator {
    match space {
      AllocationSpace::RoSpace => self.read_only_page_allocator,
      AllocationSpace::CodeSpace | AllocationSpace::CodeLoSpace => self.code_page_allocator,
      AllocationSpace::TrustedSpace
      | AllocationSpace::SharedTrustedSpace
      | AllocationSpace::TrustedLoSpace
      | AllocationSpace::SharedTrustedLoSpace => self.trusted_page_allocator,
      AllocationSpace::NewSpace
      | AllocationSpace::NewLoSpace
      | AllocationSpace::OldSpace
      | AllocationSpace::LoSpace
      | AllocationSpace::SharedSpace
      | AllocationSpace::SharedLoSpace => self.data_page_allocator,
    }
  }

  // ── 页尺寸计算 (h:252-253 + cc:173-188) ──

  /// 原版 h:252-253 声明 + cc:173-188 `ComputeChunkSize(area_size, space)` (private static)。
  /// cc:185-187 `::RoundUp(ObjectStartOffsetInMemoryChunk(space) + area_size,
  /// GetCommitPageSize())`; 布局面直用 memory_chunk.rs 落地面。
  /// 差异申报 (台账 10): ObjectStartOffsetInMemoryChunk 的 InstructionStream::kHeaderSize
  /// 实体属他票 → memory_chunk.rs 推导常量 K_INSTRUCTION_STREAM_HEADER_SIZE_DERIVED 代入。
  /// cc:175-183 布局 ASCII 注释照录:
  /// ```text
  /// +----------------------------+<- base aligned at MemoryChunk::kAlignment
  /// |          Header            |
  /// +----------------------------+<- area_start_ (base + area_start_)
  /// |           Area             |
  /// +----------------------------+<- area_end_ (area_start + area_size)
  /// |  Committed but not used    |
  /// +----------------------------+<- base + chunk_size
  /// ```
  #[must_use]
  pub fn compute_chunk_size(area_size: usize, space: AllocationSpace) -> usize {
    round_up_address(
      MemoryChunkLayout::object_start_offset_in_memory_chunk(
        space,
        K_INSTRUCTION_STREAM_HEADER_SIZE_DERIVED,
      ) + area_size,
      Self::get_commit_page_size() as usize,
    )
  }

  /// 原版 cc:506-511 匿名命名空间 `IsPagePoolSupportedForLargeSpace(LargeObjectSpace*)`:
  /// identity ∈ {NEW_LO_SPACE, LO_SPACE} 判据 (cc:519 调用点)。
  /// 差异申报 (台账 18): 入参为 cc:508 `space->identity()` 输出 (LargeObjectSpace 实体未落地)。
  #[must_use]
  pub fn is_page_pool_supported_for_large_space(space: AllocationSpace) -> bool {
    matches!(
      space,
      AllocationSpace::NewLoSpace | AllocationSpace::LoSpace
    )
  }

  // ── 边界更新 (cc:314-315 声明 + cc:781-815 实体) ──

  /// 原版 h:314-315 声明 + cc:781-815 `UpdateAllocatedSpaceLimits(low, high, executable)`。
  ///
  /// cc:783-785 注释 (atomic 原语默认序不保语义, 循环确保仅在值未变时更新) 照录;
  /// CAS 循环逐支 1:1: `load(relaxed)` + `while (界判 && !compare_exchange_weak(ptr, v,
  /// acq_rel))` — Rust 显式失败序取 Acquire (C++ 两参重载的缺省失败序对 acq_rel 成功序
  /// 即映射为 acquire; 本仓 external_entity_table.rs:319 同型先例)。
  pub fn update_allocated_space_limits(
    &mut self,
    low: Address,
    high: Address,
    executable: Executability,
  ) {
    match executable {
      Executability::NotExecutable => {
        self
          .lowest_not_executable_ever_allocated
          .fetch_min(low, Ordering::AcqRel);
        self
          .highest_not_executable_ever_allocated
          .fetch_max(high, Ordering::AcqRel);
      }
      Executability::Executable => {
        self
          .lowest_executable_ever_allocated
          .fetch_min(low, Ordering::AcqRel);
        self
          .highest_executable_ever_allocated
          .fetch_max(high, Ordering::AcqRel);
      }
    }
  }

  // ── chunk 登记 (h:317-319 + cc:739-763) ──

  /// 原版 h:317-319 注释 + cc:739-750 `RecordMemoryChunkCreated(const BasePage*)`。
  /// 大页/常规页分支登记入两集合 (cc:742/746), USE + DCHECK(result.second) 重复插入
  /// 判据 (cc:743-744) → debug_assert。
  /// 差异申报 (台账 19): 入参为 cc:741-742 `metadata->is_large()` / `metadata->Chunk()`
  /// 输出 (BasePage 方法面未落地, memory_chunk 推迟台账 1)。
  pub fn record_memory_chunk_created(&mut self, chunk: *mut MemoryChunk, is_large: bool) {
    let _guard = self.chunks_mutex.lock_guard();
    let inserted = if is_large {
      self.large_pages.insert(chunk)
    } else {
      self.normal_pages.insert(chunk)
    };
    debug_assert!(inserted, "cc:744 DCHECK(result.second) — 重复登记");
  }

  /// 原版 cc:752-763 `RecordMemoryChunkDestroyed(const BasePage*)` —
  /// erase 计数 DCHECK_EQ(1u) (cc:757/761)。差异申报同 record_memory_chunk_created (台账 19)。
  pub fn record_memory_chunk_destroyed(&mut self, chunk: *mut MemoryChunk, is_large: bool) {
    let _guard = self.chunks_mutex.lock_guard();
    let removed = if is_large {
      self.large_pages.remove(&chunk)
    } else {
      self.normal_pages.remove(&chunk)
    };
    // std 集合 erase 面返回 bool; 原版 cc:757/761 以 erase 计数断言 1u → 差异申报 (台账 19)。
    debug_assert!(removed, "cc:757/761 DCHECK_EQ(1u, size) — 未登记的 chunk");
  }

  // ── 地址反查 (h:207-214 + cc:700-737) ──

  /// 原版 h:207-210 注释 + cc:700-704 `LookupChunkContainingAddress(Address)` —
  /// 取 chunks_mutex_ 后转 safepoint 版 (cc:703)。Contains 判据宿主注入 (台账 12)。
  #[must_use]
  pub fn lookup_chunk_containing_address<C: ChunkContainsEnv + ?Sized>(
    &self,
    addr: Address,
    env: &C,
  ) -> *const MemoryChunk {
    let _guard = self.chunks_mutex.lock_guard();
    self.lookup_chunk_containing_address_in_safepoint(addr, env)
  }

  /// 原版 h:211-214 注释 (全线程 parked/safepoint 时可免锁) + cc:706-737
  /// `LookupChunkContainingAddressInSafepoint(Address)`。
  ///
  /// 1:1 结构 (cc:711 `MemoryChunk::FromAddress(addr)` 归位 → normal 集合命中 cc:715-720
  /// → large 有序集 upper_bound 前驱 cc:721-734, `std::next(it, -1)` = 不大于归位基址的
  /// 最大元素): 命中且宿主 Contains 为真返回该 chunk, 否则 nullptr (cc:736)。
  /// cc:717 DCHECK_LE 以归位恒等式承接 (base_address(addr) ≤ addr);
  /// cc:724 DCHECK_IMPLIES 与 cc:727-728 断言依赖 chunk->address() 读出 → 以指针值等价
  /// (集合元素即页对齐 chunk 基址);
  /// cc:719-720/729-730 注释 (shared heap isolate 槽位可指客户 isolate, 跳过 isolate 校验)
  /// → MetadataNoIsolateCheck 选点即为此, Contains 面入 [`ChunkContainsEnv`] (台账 12)。
  #[must_use]
  pub fn lookup_chunk_containing_address_in_safepoint<C: ChunkContainsEnv + ?Sized>(
    &self,
    addr: Address,
    env: &C,
  ) -> *const MemoryChunk {
    // cc:711: 任意地址归位 (h:208-210 注释: 槽位地址未必对应合法对象)。
    let chunk = MemoryChunk::from_address(addr);
    debug_assert!(
      MemoryChunk::base_address(addr) <= addr,
      "cc:717 DCHECK_LE(address(), addr)"
    );
    // cc:715-720 normal_pages_ 命中支。
    if let Some(&found) = self.normal_pages.get(&chunk) {
      if env.metadata_contains(found as *const MemoryChunk, addr) {
        return found as *const MemoryChunk;
      }
    // cc:721-734 large_pages_ upper_bound 支 (`upper_bound != begin()` → 存在 ≤ 归位基址者)。
    } else {
      let mut prev: Option<*mut MemoryChunk> = None;
      for &candidate in &self.large_pages {
        // BTreeSet 升序迭代: 首个严格大于归位基址者即 upper_bound, 其前一元素即前驱。
        if candidate as usize > chunk as usize {
          // cc:724-725 DCHECK_IMPLIES(upper_bound != end, addr < (*upper_bound)->address()):
          // 页对齐集合下 upper_bound 基址恒 > addr。
          debug_assert!(
            addr < candidate as usize,
            "cc:725 DCHECK_IMPLIES addr < upper_bound"
          );
          break;
        }
        prev = Some(candidate);
      }
      if let Some(candidate) = prev {
        debug_assert!(
          !candidate.is_null(),
          "cc:727 DCHECK_NOT_NULL(large_page_chunk)"
        );
        debug_assert!(
          candidate as usize <= addr,
          "cc:728 DCHECK_LE(address(), addr)"
        );
        if env.metadata_contains(candidate as *const MemoryChunk, addr) {
          return candidate as *const MemoryChunk;
        }
      }
    }
    // cc:735-736: 任一页都未命中。
    null()
  }

  // ── DEBUG 可执行块登记 (h:229-236/329-332 + cc:817-830) ──

  /// 原版 h:330 声明 + cc:818-823 `RegisterExecutableMemoryChunk(MutablePage*)` (#ifdef DEBUG)。
  /// cc:820 `DCHECK(chunk->is_executable())` 依赖 MutablePage 方法面 → 台账 14 注记跳过;
  /// cc:821 find==end 判据 → debug_assert (重复登记即非可执行登记流错误)。
  #[cfg(debug_assertions)]
  pub fn register_executable_memory_chunk(&mut self, chunk: *mut MutablePage) {
    let _guard = self.executable_memory_mutex.lock_guard();
    debug_assert!(
      !self.executable_memory.contains(&chunk),
      "cc:821 DCHECK_EQ(find, end) — 重复登记可执行块"
    );
    self.executable_memory.insert(chunk);
  }

  /// 原版 h:331 声明 + cc:825-829 `UnregisterExecutableMemoryChunk(MutablePage*)` (#ifdef DEBUG)。
  /// cc:827 DCHECK_NE(find, end) → debug_assert; 调用点 cc:302-304 (UnregisterMemoryChunk)
  /// 随该函数推迟 (台账 6)。
  #[cfg(debug_assertions)]
  pub fn unregister_executable_memory_chunk(&mut self, chunk: *mut MutablePage) {
    let _guard = self.executable_memory_mutex.lock_guard();
    debug_assert!(
      self.executable_memory.contains(&chunk),
      "cc:827 DCHECK_NE(find, end)"
    );
    self.executable_memory.remove(&chunk);
  }

  /// 原版 h:229-236 `bool IsMemoryChunkExecutable(MutablePage*) const` (#ifdef DEBUG,
  /// h:230-231 注释: 检查已分配 chunk 是否用于可执行内存)。
  #[cfg(debug_assertions)]
  #[must_use]
  pub fn is_memory_chunk_executable(&self, chunk: *mut MutablePage) -> bool {
    let _guard = self.executable_memory_mutex.lock_guard();
    self.executable_memory.contains(&chunk)
  }
}

// h:414-415 `friend class heap::TestCodePageAllocatorScope / TestMemoryAllocatorScope` —
// Rust 无友元; 上述 pub 面即测试钩子的承接形 (台账 16; 两 Scope 实体属他席测试设施)。
// h:29-30 `namespace heap { class TestMemoryAllocatorScope; }` 前向声明同入该台账。

// ══════════════════════════════════════════════════════════════════════════
// 推迟台账 (真实耦合面; 双行号 + 原因; 台账锚与 0 检臂见
// tool/src/heap/memory-allocator.h.js / memory-allocator.cc.js)
// ══════════════════════════════════════════════════════════════════════════
// 1. 分配门面三件套 — h:96-98 `AllocatePage(AllocationMode, Space*, Executability)`、
//    h:100-103 `AllocateLargePage(LargeObjectSpace*, size_t, Executability, AllocationHint)`、
//    h:108-109 `AllocateReadOnlyPage(ReadOnlySpace*, Address)`; 实体 cc:432-473 /
//    cc:513-553 / cc:475-498。原因: placement new NormalPage/LargePage/ReadOnlyPage/
//    MemoryChunk 头 + malloc 元数据块 + space->InitializePage + RwxMemoryWriteScope
//    构造路径 + SandboxHardwareSupport (cc:492-495, v8-sandbox 不在本 crate 依赖) —
//    页/空间实体属他票。
// 2. 未初始化 chunk 分配 — h:259-265 inline `AllocateUninitializedChunk` 转发 +
//    h:266-270 声明 + cc:190-248 `AllocateUninitializedChunkAt`。原因: 依赖台账 3 的
//    VirtualMemory 构造面 + cc:224-235 zapping (互斥他席面, 台账 13) + cc:237-238 LOG +
//    cc:196-205 hint 支 (V8_COMPRESS_POINTERS 开时展开剥除, 本落地面即目标配置, 申报注)。
// 3. 对齐预留 — h:274-278 声明 + cc:113-171 `AllocateAlignedMemory`。原因:
//    VirtualMemory(PageAllocator, chunk_size, AllocationHint 带权限, alignment, permissions)
//    构造属平台层 (v8-utils allocation.rs:4-5 已登记推迟; reservation 族未落地),
//    另消费 cc:134/149 HandleAllocationFailure (台账 4)、cc:141-143 CHECK 末位 chunk
//    占位、cc:163-164 ThreadIsolation::RegisterJitPage (v8-sandbox, 台账 11)、
//    cc:167 UpdateAllocatedSpaceLimits (本面已落地, 待接入)。
// 4. 分配失败处理 — cc:99-111 匿名命名空间 `HandleAllocationFailure(Heap*, Executability)`。
//    原因: Heap::deserialization_complete()/FatalProcessOutOfMemory() 他票 (Heap 仅留
//    前向声明承接位)。
// 5. 区域释还 — h:287-288 声明 + cc:94-97 `FreeMemoryRegion(PageAllocator*, addr, size)`。
//    原因: 全局 `FreePages(v8::PageAllocator*, ...)` (utils/allocation.cc) 平台面未落地。
// 6. 注销族 — h:321-327 声明 + cc:282-288 `UnregisterSharedMemoryChunk`、cc:290-313
//    `UnregisterMemoryChunk` (含 cc:291 base_page->Chunk()、cc:292/298/300-301 size 族
//    DCHECK、cc:302-304 DEBUG 反登记、cc:306 ThreadIsolation::UnregisterJitPage、
//    cc:310-312 TryCast<MutablePage> + set_is_unregistered)、cc:315-317
//    `UnregisterMutableMemoryChunk`、cc:319-322 `UnregisterReadOnlyPage`。原因:
//    BasePage/MutablePage 方法面 (reserved_memory/size/is_executable 等) 实体未落地。
// 7. 释放流程 — h:114-118 声明 + cc:345-356 `PreFreeMemory` (Heap::RememberUnmappedPage +
//    ReleaseAllAllocatedMemory + LOG)、cc:358-364 `PerformFreeMemory`、cc:366-407 `Free`
//    (FreeMode 四支分发 + 延迟页向量类型化 push + pool->Add 需 PooledPage::Create,
//    memory_pool 台账 1)、cc:409-430 `ReleaseDelayedPages`。原因: 页面方法面 + 池 Create 族。
// 8. 只读页释还 — h:116 声明 + cc:324-343 `FreeReadOnlyPage`。原因: ReadOnlyPage 方法面 +
//    `delete chunk` 实体释构 + LOG (cc:326)。
// 9. 部分释还与扩容 — h:154-159 声明 + cc:250-280 `PartialFreeMemory` (DiscardSealedMemoryScope
//    已落地但 VirtualMemory::DiscardSystemPages/Release/SetPermissions 面不在 Reservation
//    5 面内 + jit 写保护旗标 cc:263-265) + h:105-106 声明 + cc:555-592 `ResizeLargePage`
//    (page->reservation_ 私有字段直读 + VirtualMemory::Resize)。原因: VirtualMemory 实体面。
// 10. 池取还内部面 — h:299-305 声明 + cc:594-636 `AllocateUninitializedPageFromDelayedOrPool`
//     (CreatePooledPage(NormalPage*) 即 memory_pool 台账 2 + cc:617 VirtualMemory  adopting
//     ctor + zapping) 与 cc:638-672 `TryAllocateUninitializedLargePageFromPool` (同型,
//     cc:657)。池门面 remove/remove_large 已落地 (memory_pool cc:410-426), 本处差
//     reservation 构造与 chunk_size=NormalPage::kPageSize 的页面对偶。
// 11. 可执行块权限 — h:280-284 声明 + cc:682-698 `SetPermissionsOnExecutableMemoryChunk`。
//     原因: VirtualMemory RecommitPages/SetPermissions + Isolate::RequiresCodeRange +
//     MutablePage::GetCodeModificationPermission 实体面; ThreadIsolation 族
//     (cc:163-165/306, sandbox/hardware-support.h) 与 SandboxHardwareSupport
//     (cc:492-495) 属 v8-sandbox crate, 不在本 crate 依赖 (台账 1/3 内逐点引用)。
// 12. 共享页重映射 — h:111-112 声明 + cc:500-504 `RemapSharedPage`。原因:
//     v8::PageAllocator::SharedMemory/SharedMemoryMapping::RemapTo 未迁移
//     (v8-base page_allocator.rs:6-7 已登记 SharedMemory 族); Contains 面已由
//     [`ChunkContainsEnv`] 承接至 lookup 族 (cc:720/731)。
// 13. zapping 消费 — cc:224-235/618-620/658-660 `heap::ShouldZapGarbage()/ZapBlock` +
//     cc:226-228 CodePageMemoryModificationScopeForDebugging。原因: zapping.rs 属互斥他席
//     面 (票面 L1 清单), 本席不读写; kZapValue 已落地 (v8_common::K_ZAP_VALUE) 但随函数体
//     一并待接。
// 14. LOG 事件 — cc:237-238 NewEvent / cc:326/348 DeleteEvent。原因: v8-logging crate
//     不在本 crate 依赖; 随宿主函数 (台账 2/7/8) 一并接入。
// 15. InitializePagesInChunk — h:307-312 声明 (mark-compact 重建 from-space 页头用途,
//     h:308-310 注释)。实测 v8/src + v8/test 全树 grep 仅此声明 1 处、无定义无调用 →
//     原版悬空声明, 不落地 (memory_pool 台账 7 GarbageCollectionPrologue 同型)。
// 16. 拷贝控制与 friend — h:88-89 copy ctor/assign = delete → Rust 不派生 Clone/Copy;
//     h:414-415 friend ×2 与 h:29-30 前向声明 → Rust 无友元, 访问面以 pub 承接
//     (memory_chunk 台账 11 先例)。
// 17. ctor 取源 — cc:41 `isolate->page_allocator()` 与 cc:42-43
//     `IsolateGroup::current()->read_only_page_allocator()`。原因: Isolate 方法面与
//     IsolateGroup 单例他票 → 对应 allocator 改参数注入 (本文件 [`MemoryAllocator::new`]),
//     初始化列表其余项与 cc:40-47 逐项 1:1。
// 18. LargeObjectSpace 入参形 — cc:506-511 函数体保留 1:1, 形参取 cc:508
//     `space->identity()` 输出 (空间实体未落地; 见本文件落地函数 doc)。
// 19. BasePage 入参形 — cc:739-763 Record 族的 `metadata->is_large()/Chunk()` 与
//     cc:706-737 lookup 的 chunk->address() 读出, 以 (chunk, is_large) 参数/指针值等价承接
//     (BasePage 方法面推迟, memory_chunk 台账 1 同源)。
// 20. h 声明面未复写清单 — h:41 `class MemoryAllocator final` 的访问控制 (private 段
//     h:238-409 字段/方法在 Rust 全 pub 承接, 台账 16 同因); h:259/266 V8_WARN_UNUSED_RESULT
//     与 h:71/76/137/141 V8_INLINE (展开侧记号剥除, Rust #[inline]/must_use 位随函数落点);
//     h:379/380/385-386 等字段声明均已落 struct, 无遗漏。
// 21. 实体析构释放 — h:81 声明 (public static) + cc:766-779 `DeleteMemoryChunk(MutablePage*)`
//     (调用点 cc:363 PerformFreeMemory, 该宿主已入台账 7)。原因: cc:767 metadata->
//     reserved_memory()->IsReserved() DCHECK + cc:771 DiscardSealedMemoryScope 写权限域 +
//     cc:772-776 `is_large()` 分派 LargePage/NormalPage placement 显式析构 + cc:778
//     free(metadata) — 页面实体析构面属他票, 与台账 1 的 malloc 元数据块分配对偶,
//     随页/空间实体同批接入。
// 22. h:327 `void RegisterReadOnlyMemory(ReadOnlyPage* page)` — 全树实测 (v8/src + v8/test)
//     仅此声明 1 处、无定义无调用 (cc:493 命中的
//     SandboxHardwareSupport::RegisterReadOnlyMemoryInsideSandbox 为另一符号) →
//     原版悬空声明, 不落地 (台账 15 InitializePagesInChunk 同型)。
