//! 空间公共件 (Space 基类与空间/页/对象迭代器族)。
//! 对应原版 v8/src/heap/spaces.h (235 行) + spaces.cc (67 行) + spaces-inl.h (113 行)。
//! (HC-4 空间实体族第一波 R5H3 落地)
//!
//! 目标配置 (与 tool/src/heap/spaces.js clang -E 展开侧同源):
//!   V8_COMPRESS_POINTERS + V8_ENABLE_SANDBOX + V8_EXTERNAL_CODE_SPACE, DEBUG/TSAN 关。
//!   `#ifdef DEBUG` 消费面 (h:112-114 Print 纯虚、h:51-55 DCHECK 宏) 以
//!   `#[cfg(debug_assertions)]` 承接 (memory_allocator.rs/allocation.rs DEBUG 先例);
//!   `#ifdef VERIFY_HEAP` 面 (base-space.h:44-47 Verify) 目标配置剥除 → 不落地 (台账 20)。
//!
//! 票面差异申报 (以本仓源实态为准, 主控汇报):
//! 票面所述 Space 面 "Capacity/Waste/RemovePage/AddPage/ReleasePage/Free/IsInlineable" 与
//! 字段面 "external_backing_store_bytes_/allocated_bytes_" 均为旧版 V8 叙事; 本仓 spaces.h
//! 实态 (235 行) 的 Space 抽象面为: SizeOfObjects/Available/GetObjectIterator/first_page/
//! last_page/memory_chunk_list/InitializePage/NotifyBlackArea{Created,Destroyed}/free_list/
//! FirstPageAddress/Print(DEBUG) + BaseSpace 继承面 (heap/identity/CommittedMemory/
//! CommittedPhysicalMemory/Size/Account{,Un}committed)。Capacity/Waste 族在本版本位于
//! PagedSpace/AllocationStats (paged-spaces.h/allocation-stats.h, 他票) → 本票按实况 1:1 落地,
//! 不复活旧版面。
//!
//! 承接形映射 (申报):
//! - C++ 虚函数基类 Space (h:67) → trait [`Space`] + 字段结构 [`SpaceBase`] (free_list.rs
//!   trait FreeList + FreeListBase 先例; h:69-70 ctor 即 SpaceBase::new)。
//! - `std::unique_ptr<FreeList>` (h:119) → 泛参 `Option<F> where F: FreeList` —
//!   FreeList trait 含泛型方法不可 dyn 化 (本仓禁 dyn), 具体自由链表类型由持链空间
//!   (PagedSpace 族, HC-4 下波) 单态化; 无链空间 (LargeObjectSpace, cc:53-54 传 nullptr)
//!   以 [`NoFreeList`] 空占位实例承载 (memory_allocator.rs `MemoryAllocator<R: Reservation>`
//!   泛参先例同型)。
//! - `heap::List<MutablePage>` (h:118; list.h 实体他票) → [`PageList`] 承接 front/back/
//!   PushBack/Remove/Empty 可观测面 (spaces.h/cc 消费子集; 侵入式链的节点推进面经
//!   PageIteratorImpl 闭包注入, 对齐 spaces-inl.h:24-27 `p_ = p_->next_page()`)。
//! - `reinterpret_cast<LargePage*>` (large-spaces.h:69-74) → 指针位型直转 (opaque 页型
//!   从不解引用, memory_allocator.rs 空 enum 先例)。
//! - `Tagged<HeapObject>`/`Heap`/`MutablePage`/`NormalPage`/`LargePage` 复用已落地承接型
//!   (v8-objects-core tagged / crate::memory_allocator), 不另造平行世界。
//! - `SpaceWithLinearArea::CreateAllocatorPolicy` (h:201) → trait 方法, 返回
//!   `Box<dyn AllocatorPolicy>` (main_allocator.rs h:353 同型 unique_ptr 承接先例)。
//! - 深水推迟: SpaceIterator/MemoryChunkIterator 依赖 Heap::space 与页实体 list_node 面
//!   (Isolate 全链/页实体他票) → 结构+字段落, 方法体 todo!() 并登记文末台账。

use std::{
  mem::size_of,
  ptr::null_mut,
  sync::atomic::{AtomicIsize, AtomicUsize, Ordering},
};

use v8_common::{
  Address, AllocationSpace, FIRST_MUTABLE_SPACE, K_SYSTEM_POINTER_SIZE, LAST_MUTABLE_SPACE,
};
use v8_objects_core::{heap_object::HeapObject, tagged::Tagged};
// h:51-55 两 DCHECK 宏的消费面仅在 DEBUG 配置位 (allocation.rs:18 同法)。
#[cfg(debug_assertions)]
use {
  crate::memory_chunk::{K_INSTRUCTION_STREAM_HEADER_SIZE_DERIVED, MemoryChunkLayout},
  v8_common::K_MAX_REGULAR_HEAP_OBJECT_SIZE,
};

use crate::{
  allocation::AllocationOrigin,
  free_list,
  free_list::{
    FreeList, FreeListBase, FreeListCategoryType, FreeListEnv, FreeSpace, Heap as FreeListHeap,
  },
  main_allocator::{AllocatorPolicy, MainAllocator},
  memory_allocator::{Heap, LargePage, MutablePage, NormalPage},
};

// ══════════════════════════════════════════════════════════════════════════
// 调试断言宏 (原版 spaces.h:49-55)
// ══════════════════════════════════════════════════════════════════════════

/// 原版 h:51-52 `DCHECK_VALID_REGULAR_OBJECT_SIZE(size)` —
/// `DCHECK((0 < size) && (size <= kMaxRegularHeapObjectSize))`。
/// 目标配置展开侧记号剥除, 断言体以 `#[cfg(debug_assertions)]` 承接 (DEBUG 面先例)。
#[cfg(debug_assertions)]
#[inline]
pub fn debug_assert_valid_regular_object_size(size: usize) {
  debug_assert!(
    (0 < size) && (size <= K_MAX_REGULAR_HEAP_OBJECT_SIZE),
    "h:51-52 DCHECK_VALID_REGULAR_OBJECT_SIZE({size})"
  );
}

/// 原版 h:54-55 `DCHECK_VALID_REGULAR_CODEOBJECT_SIZE(size)` —
/// `DCHECK((0 < size) && (size <= MemoryChunkLayout::MaxRegularCodeObjectSize()))`。
/// 差异申报 (台账 21): MaxRegularCodeObjectSize 的 InstructionStream::kHeaderSize 实体属
/// 他票 → memory_chunk.rs 推导常量参数注入 (memory_allocator.rs:554-556 先例)。
#[cfg(debug_assertions)]
#[inline]
pub fn debug_assert_valid_regular_codeobject_size(size: usize) {
  debug_assert!(
    (0 < size)
      && (size
        <= MemoryChunkLayout::max_regular_code_object_size(K_INSTRUCTION_STREAM_HEADER_SIZE_DERIVED)
          as usize),
    "h:54-55 DCHECK_VALID_REGULAR_CODEOBJECT_SIZE({size})"
  );
}

// ══════════════════════════════════════════════════════════════════════════
// ForAll 枚举遍历模板 (原版 h:57-62)
// ══════════════════════════════════════════════════════════════════════════

/// 原版 h:57-58 `template <typename Enum, typename Callback>` 的 Enum 侧约束承接:
/// kNumValues 终哨 + 整值还原 (strum 型枚举可就地 impl)。
pub trait ForAllEnum: Copy {
  /// 原版 h:59 `static_cast<int>(Enum::kNumValues)` 的枚举侧常量。
  const K_NUM_VALUES: i32;
  /// 原版 h:60 `static_cast<Enum>(i)`。
  fn from_index(i: i32) -> Self;
}

/// 原版 h:58-62 `ForAll<Enum>(callback)` — 0..kNumValues 逐值回调 (值, 序号)。
#[inline]
pub fn for_all<E: ForAllEnum>(mut callback: impl FnMut(E, i32)) {
  for i in 0..E::K_NUM_VALUES {
    callback(E::from_index(i), i);
  }
}

// ══════════════════════════════════════════════════════════════════════════
// PageList (h:118 heap::List<MutablePage> 承接形)
// ══════════════════════════════════════════════════════════════════════════

/// 原版 list.h `List<MutablePage>` 的消费面承接 (list.h 实体属他票, 本票 spaces.h/.cc
/// 仅消费 front/back/PushBack/Remove/Empty 五面 → 序语义等价承接: 尾插 + 按指针值移除;
/// 侵入式节点前向链在页实体票接入, 迭代推进面经 PageIteratorImpl 闭包/env 注入)。
pub struct PageList<P> {
  /// 页指针序列 (原版链表序 = 入列序)。
  pages: Vec<*mut P>,
}

/// 手写 Default: derive 会误加 `P: Default` 界 (MutablePage 等空 enum 承接型不实现 Default)。
impl<P> Default for PageList<P> {
  fn default() -> Self {
    Self { pages: Vec::new() }
  }
}

impl<P> PageList<P> {
  /// 原版 list.h:86 `T* front()` (front_ 初值 nullptr, list.h:130) — 空表返回 nullptr;
  /// spaces.h:86 `first_page()` 消费 (HCR1-F7 锚点校正: 原注 list.h:52 系 Remove 行位)。
  #[must_use]
  pub fn front(&self) -> *mut P {
    self.pages.first().copied().unwrap_or(null_mut())
  }

  /// 原版 h:87 `List<T>::back()` — 空表返回 nullptr。
  #[must_use]
  pub fn back(&self) -> *mut P {
    self.pages.last().copied().unwrap_or(null_mut())
  }

  /// 原版 large-spaces.cc:193 `memory_chunk_list_.PushBack(page)` (AddPage cc:188-195
  /// 落点; 定义 list.h:30-39) — HCR1-F7 锚点校正: 原注 h:193 误挂 spaces.h 前缀。
  /// list.h:31-32 前置 DCHECK (页未在他表) 序列表不可判 → 文末台账 13。
  pub fn push_back(&mut self, page: *mut P) {
    self.pages.push(page);
  }

  /// 原版 list.h `List<T>::Remove(page)` — 页须在表中 (实体 DCHECK 待接, 本处以
  /// 移除命中承接同断言)。
  pub fn remove(&mut self, page: *mut P) {
    let idx = self.pages.iter().position(|p| *p == page);
    debug_assert!(idx.is_some(), "list.h:Remove 未命中页 (原版 DCHECK)");
    if let Some(i) = idx {
      self.pages.remove(i);
    }
  }

  /// 原版 list.h:80 `Empty()` 定义 — large-spaces.cc:63 `TearDown` 循环条件消费
  /// (HCR1-F7 锚点校正: 原注 h:63 误挂 spaces.h 前缀)。
  #[must_use]
  pub fn is_empty(&self) -> bool {
    self.pages.is_empty()
  }

  /// 入列序遍历 (begin()/end() 的表侧等价面, 供具体空间实现迭代)。
  pub fn iter(&self) -> impl Iterator<Item = *mut P> + '_ {
    self.pages.iter().copied()
  }

  /// 表长 (承接可观测面; 原版无此 API, 仅供 Rust 侧断言入列序完整)。
  #[must_use]
  pub fn len(&self) -> usize {
    self.pages.len()
  }
}

// ══════════════════════════════════════════════════════════════════════════
// Space 基类 (原版 h:64-120 + base-space.h:20-67 继承面)
// ══════════════════════════════════════════════════════════════════════════

/// 原版 h:107-110 `FirstPageAddress()` 消费的 `MutablePage::ChunkAddress()` 判据承接
/// (base-page.h 实体他票 → 宿主注入, ChunkContainsEnv/FreeListEnv 先例)。
pub trait PageChunkAddressEnv {
  /// 原版 base-page.h `address_t ChunkAddress() const { return address_; }`。
  fn chunk_address(&self, page: *mut MutablePage) -> Address;
}

/// 原版 h:69-70 `Space(Heap*, AllocationSpace, std::unique_ptr<FreeList>)` +
/// BaseSpace 字段 (base-space.h:64-69) + h:116-119 protected 字段的承载结构。
/// 泛参 F = 具体自由链表类型 (模块头申报; 无链空间用 [`NoFreeList`])。
/// Debug 不派生: 承接位含裸指针链与原子字段, 无调试输出诉求 (下波按需手工 impl)。
pub struct SpaceBase<F: FreeList> {
  /// 原版 base-space.h:65 `Heap* heap_`。
  pub heap: *mut Heap,
  /// 原版 base-space.h:66 `AllocationSpace id_`。
  pub id: AllocationSpace,
  /// 原版 base-space.h:68-69 注释 + `std::atomic<size_t> committed_{0}` —
  /// 空间已提交内存 (含页头, base-space.h:34-35 注释)。原子序 SeqCst 承接
  /// std::atomic 默认序 (全 crate 先例)。
  pub committed: AtomicUsize,
  /// 原版 h:117-118 注释 + `heap::List<MutablePage> memory_chunk_list_`。
  pub memory_chunk_list: PageList<MutablePage>,
  /// 原版 h:119 `std::unique_ptr<FreeList> free_list_` — 可空 (cc:53-54 LargeObjectSpace
  /// 构造传 nullptr)。
  pub free_list: Option<F>,
}

impl<F: FreeList> SpaceBase<F> {
  /// 原版 h:69-70 ctor: `BaseSpace(heap, id)` (base-space.h:50) +
  /// `free_list_(std::move(free_list))`。committed_ 初值 0 (base-space.h:69)。
  #[must_use]
  pub fn new(heap: *mut Heap, id: AllocationSpace, free_list: Option<F>) -> Self {
    Self {
      heap,
      id,
      committed: AtomicUsize::new(0),
      memory_chunk_list: PageList::default(),
      free_list,
    }
  }

  /// 原版 base-space.h:27-30 `Heap* heap() const` — DCHECK_NOT_NULL(heap_) (h:28)。
  #[inline]
  #[must_use]
  pub fn heap(&self) -> *mut Heap {
    debug_assert!(
      !self.heap.is_null(),
      "base-space.h:28 DCHECK_NOT_NULL(heap_)"
    );
    self.heap
  }

  /// 原版 base-space.h:32 `AllocationSpace identity() const`。
  #[inline]
  #[must_use]
  pub fn identity(&self) -> AllocationSpace {
    self.id
  }

  /// 原版 base-space.h:34-35 注释 + h:36 `virtual size_t CommittedMemory() const` —
  /// `return committed_` (可覆写虚面, 默认读字段)。
  #[inline]
  #[must_use]
  pub fn committed_memory(&self) -> usize {
    self.committed.load(Ordering::SeqCst)
  }

  /// 原版 base-space.h:54-57 `void AccountCommitted(size_t bytes)` —
  /// h:55 DCHECK_GE(committed_ + bytes, committed_) (无回绕)。C++ 加法回绕的等价判据
  /// 即 `committed_ + bytes` 不溢出 → checked_add 承接 (HCR1-F2 整改: 原
  /// saturating_add >= committed 形恒真, 不具观测力)。
  /// 差异申报 (台账 22): C++ protected 成员 → Rust pub 承接 (memory_allocator 台账 16 先例)。
  pub fn account_committed(&mut self, bytes: usize) {
    let committed = self.committed_memory();
    debug_assert!(
      committed.checked_add(bytes).is_some(),
      "base-space.h:55 DCHECK_GE(committed_ + bytes, committed_) (回绕即违例)"
    );
    self.committed.store(committed + bytes, Ordering::SeqCst);
  }

  /// 原版 base-space.h:59-62 `void AccountUncommitted(size_t bytes)` —
  /// h:60 DCHECK_GE(committed_, committed_ - bytes) (无下溢) → checked_sub 同形承接 (F2)。
  pub fn account_uncommitted(&mut self, bytes: usize) {
    let committed = self.committed_memory();
    debug_assert!(
      committed.checked_sub(bytes).is_some(),
      "base-space.h:60 DCHECK_GE(committed_, committed_ - bytes) (下溢即违例)"
    );
    self.committed.store(committed - bytes, Ordering::SeqCst);
  }

  /// 原版 h:105 `FreeList* free_list()` — unique_ptr 取址; 空链返回 null 承接。
  #[inline]
  #[must_use]
  pub fn free_list(&mut self) -> Option<&mut F> {
    self.free_list.as_mut()
  }

  /// 原版 h:107-110 `Address FirstPageAddress() const` —
  /// DCHECK_NOT_NULL(first_page()) + first_page()->ChunkAddress() (判据宿主注入)。
  #[must_use]
  pub fn first_page_address<E: PageChunkAddressEnv + ?Sized>(&self, env: &E) -> Address {
    let page = self.memory_chunk_list.front();
    debug_assert!(!page.is_null(), "h:108 DCHECK_NOT_NULL(first_page())");
    env.chunk_address(page)
  }
}

/// 原版 h:64-67 注释: Space 是除启动后封存空间 (ReadOnlySpace) 外一切分配空间的抽象父类。
/// 虚面 trait (h:77-114 + base-space.h:27-62 继承面); 覆写关系由具体空间 impl 逐槽对照
/// (free_list.rs trait FreeList 先例)。h:74-75 copy ctor/assign = delete → 承接类型不派生
/// Clone/Copy (memory_allocator 台账 16 同型; base-space.h:24-25 同禁)。
pub trait Space {
  /// 自由链表泛参 (模块头申报: unique_ptr<FreeList> 位型由具体空间单态化)。
  type FreeListKind: FreeList;

  /// 基类字段面读出 (h:116-119 + base-space.h:64-69; Rust 组合链逐层上转)。
  fn space_base(&self) -> &SpaceBase<Self::FreeListKind>;

  /// 基类字段面可变读出。
  fn space_base_mut(&mut self) -> &mut SpaceBase<Self::FreeListKind>;

  /// 原版 h:77-79 注释 + `virtual size_t SizeOfObjects() const { return Size(); }` —
  /// 对象尺寸和, 可与分配尺寸不同 (OldLargeObjectSpace 覆写)。
  #[inline]
  #[must_use]
  fn size_of_objects(&self) -> usize {
    self.size()
  }

  /// 原版 h:81-82 `virtual size_t Available() const = 0` — 不扩容可用的字节数 (纯虚)。
  #[must_use]
  fn available(&self) -> usize;

  /// 原版 base-space.h:38-39 注释 + `virtual size_t CommittedPhysicalMemory() const = 0`
  /// — 物理内存近似提交量 (纯虚)。
  #[must_use]
  fn committed_physical_memory(&self) -> usize;

  /// 原版 base-space.h:41-42 `virtual size_t Size() const = 0` — 已分配尺寸 (纯虚)。
  #[must_use]
  fn size(&self) -> usize;

  /// 原版 base-space.h:34-36 注释 + `virtual size_t CommittedMemory() const
  /// { return committed_; }` — 总提交内存 (含页头), 基类默认读字段。
  #[inline]
  #[must_use]
  fn committed_memory(&self) -> usize {
    self.space_base().committed_memory()
  }

  // 原版 h:84 `virtual std::unique_ptr<ObjectIterator> GetObjectIterator(Heap*) = 0`。
  // 差异申报 (台账 23): 具体空间迭代器经页实体 env 注入而类型各异 (LO 走 LargePageOps),
  // 统一签名不可表达 → 本 trait 不复写该纯虚槽, 由各具体空间以固有方法承接
  // (large_spaces.rs `get_object_iterator` 落点 cc:274-278)。h:84 声明位照录于此,
  // 不入 vtable 面。

  /// 原版 h:86-94 `first_page()/last_page()` 四重载 (mutable/const × first/last) —
  /// Rust 裸指针不携带指点可变性 → 双 const 重载并入单槽 (申报, 值语义同源 list front/back)。
  #[inline]
  #[must_use]
  fn first_page(&self) -> *mut MutablePage {
    self.space_base().memory_chunk_list.front()
  }

  /// 原版 h:87/h:92-94 `last_page()`。
  #[inline]
  #[must_use]
  fn last_page(&self) -> *mut MutablePage {
    self.space_base().memory_chunk_list.back()
  }

  /// 原版 h:96-98 `virtual heap::List<MutablePage>& memory_chunk_list()`。
  #[inline]
  fn memory_chunk_list(&mut self) -> &mut PageList<MutablePage> {
    &mut self.space_base_mut().memory_chunk_list
  }

  /// 原版 h:100 `virtual NormalPage* InitializePage(MutablePage* chunk) { UNREACHABLE(); }`。
  /// 差异申报: 原版 UNREACHABLE 目标配置 release 下为不可达路径 → 本席禁 panic 面
  /// (memory_chunk.rs:567 先例), 基类默认以空指针承接 (调用即调用方违例, PagedSpace 族
  /// 覆写落地, 实体他票)。
  #[inline]
  #[must_use]
  fn initialize_page(&mut self, _chunk: *mut MutablePage) -> *mut NormalPage {
    null_mut()
  }

  /// 原版 h:102 `virtual void NotifyBlackAreaCreated(size_t size) {}` — 基类空默认。
  #[inline]
  fn notify_black_area_created(&mut self, _size: usize) {}

  /// 原版 h:103 `virtual void NotifyBlackAreaDestroyed(size_t size) {}` — 基类空默认。
  #[inline]
  fn notify_black_area_destroyed(&mut self, _size: usize) {}

  /// 原版 h:112-114 `#ifdef DEBUG virtual void Print() = 0;` (h:113 纯虚) — DEBUG 纯虚面,
  /// 目标配置展开侧剥除 → Rust 以 `#[cfg(debug_assertions)]` 承接可选覆写位
  /// (默认不打印, 具体空间覆写; LargeObjectSpace 覆写落点 = large_spaces.rs impl Space
  /// 槽, HCR1-F3 归位, 与本台账 12 双记)。
  #[cfg(debug_assertions)]
  fn print(&mut self) {}
}

/// 原版 h:122 `static_assert(sizeof(std::atomic<intptr_t>) == kSystemPointerSize)`。
const _: () = assert!(size_of::<AtomicIsize>() == K_SYSTEM_POINTER_SIZE);

// ══════════════════════════════════════════════════════════════════════════
// ObjectIterator (原版 h:124-136)
// ══════════════════════════════════════════════════════════════════════════

/// 原版 h:128-136 `class ObjectIterator : public Malloced` — 堆对象迭代器接口。
/// h:130-133 注释 (dtor 不可 = default 的 C++20 constexpr 细节) 为语言侧问题, Rust Drop
/// 无此约束, 不承接; 虚析构面由 Box 持有者释放承接。
pub trait ObjectIterator {
  /// 原版 h:135 `virtual Tagged<HeapObject> Next() = 0` — 耗尽返回 null Tagged
  /// (h:135 纯虚 + large-spaces.cc:41-48 消费形态)。
  /// 命名申报: 与 clippy::should_implement_trait 无涉 (返回型非 Option, 无 Iterator 对应体)。
  fn next(&mut self) -> Tagged<HeapObject>;
}

// ══════════════════════════════════════════════════════════════════════════
// PageIteratorImpl / PageRange (原版 h:138-192 + spaces-inl.h:23-38)
// ══════════════════════════════════════════════════════════════════════════

/// 原版 h:138-158 `template <class PageType> class PageIteratorImpl` —
/// 前向页迭代器 (base::iterator<forward_iterator_tag, PageType> 薄壳不承接, Rust 侧
/// 以固有方法表达 operator* / ++ / == / != 四件套; 泛参 P = 页裸指针类型, 对齐 p_ 字段)。
///
/// operator++/++(int) 的 `p_->next_page()` 推进判据 (spaces-inl.h:24-27/30-33) 属页实体面
/// (mutable-page.h 他票) → 以闭包注入承接 (本文件模块头申报)。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PageIteratorImpl<P> {
  /// 原版 h:157 `PageType* p_`。
  pub p: P,
}

impl<P: Copy> PageIteratorImpl<P> {
  /// 原版 h:142 `explicit PageIteratorImpl(PageType* p)`。
  #[inline]
  #[must_use]
  pub fn new(p: P) -> Self {
    Self { p }
  }

  /// 原版 h:146 `PageType* operator*()`。
  #[inline]
  #[must_use]
  pub fn deref(&self) -> P {
    self.p
  }

  /// 原版 spaces-inl.h:23-27 `operator++()` — `p_ = p_->next_page(); return *this`。
  /// next_page 判据 (mutable-page.h 他票) 以闭包注入。
  #[inline]
  pub fn advance_with(&mut self, next_page: impl Fn(P) -> P) -> &mut Self {
    self.p = next_page(self.p);
    self
  }

  /// 原版 spaces-inl.h:29-34 `operator++(int)` — 快照自身 → 前置 ++ → 返回旧值。
  #[inline]
  #[must_use]
  pub fn post_advance_with(&mut self, next_page: impl Fn(P) -> P) -> Self {
    let tmp = *self;
    self.advance_with(next_page);
    tmp
  }
}

/// 相等判据独立 impl 块 — 仅 `equals`/`not_equals` 需要 P: PartialEq,
/// 推进面 (闭包注入) 与解引用面不受多余约束牵连。
impl<P: PartialEq> PageIteratorImpl<P> {
  /// 原版 h:147-149 `bool operator==(const PageIteratorImpl&)` — p_ 指针值比较。
  #[inline]
  #[must_use]
  pub fn equals(&self, rhs: &Self) -> bool {
    self.p == rhs.p
  }

  /// 原版 h:150-152 `bool operator!=`。
  #[inline]
  #[must_use]
  pub fn not_equals(&self, rhs: &Self) -> bool {
    !self.equals(rhs)
  }
}

/// 原版 h:160 `using PageIterator = PageIteratorImpl<NormalPage>`。
pub type PageIterator = PageIteratorImpl<*mut NormalPage>;
/// 原版 h:161 `using ConstPageIterator = PageIteratorImpl<const NormalPage>`。
pub type ConstPageIterator = PageIteratorImpl<*const NormalPage>;
/// 原版 h:162 `using LargePageIterator = PageIteratorImpl<LargePage>`。
pub type LargePageIterator = PageIteratorImpl<*mut LargePage>;
/// 原版 h:163 `using ConstLargePageIterator = PageIteratorImpl<const LargePage>`。
pub type ConstLargePageIterator = PageIteratorImpl<*const LargePage>;

/// 原版 h:165-177 `class PageRange` — [begin_, end_) 页区间 (iterator = PageIterator)。
#[derive(Clone, Copy, Debug)]
pub struct PageRange {
  /// h:175 `NormalPage* begin_`。
  pub begin_: *mut NormalPage,
  /// h:176 `NormalPage* end_`。
  pub end_: *mut NormalPage,
}

impl PageRange {
  /// 原版 h:168 双参 ctor `PageRange(begin, end)`。
  #[inline]
  #[must_use]
  pub fn new(begin: *mut NormalPage, end: *mut NormalPage) -> Self {
    Self {
      begin_: begin,
      end_: end,
    }
  }

  /// 原版 h:169 + spaces-inl.h:36 `explicit PageRange(NormalPage* page)` —
  /// `PageRange(page, page->next_page())` (next_page 判据闭包注入, 申报见模块头)。
  #[inline]
  #[must_use]
  pub fn from_page(
    page: *mut NormalPage,
    next_page: impl Fn(*mut NormalPage) -> *mut NormalPage,
  ) -> Self {
    Self::new(page, next_page(page))
  }

  /// 原版 h:171 `iterator begin()`。
  #[inline]
  #[must_use]
  pub fn begin(&self) -> PageIterator {
    PageIterator::new(self.begin_)
  }

  /// 原版 h:172 `iterator end()`。
  #[inline]
  #[must_use]
  pub fn end(&self) -> PageIterator {
    PageIterator::new(self.end_)
  }
}

/// 原版 h:179-192 `class ConstPageRange` — const 页区间 (与 PageRange 仅指针可变性之差,
/// Rust 裸指针承接位型, 差异申报同 first_page 重载并槽注)。
#[derive(Clone, Copy, Debug)]
pub struct ConstPageRange {
  /// h:190 `const NormalPage* begin_`。
  pub begin_: *const NormalPage,
  /// h:191 `const NormalPage* end_`。
  pub end_: *const NormalPage,
}

impl ConstPageRange {
  /// 原版 h:182-183 双参 ctor。
  #[inline]
  #[must_use]
  pub fn new(begin: *const NormalPage, end: *const NormalPage) -> Self {
    Self {
      begin_: begin,
      end_: end,
    }
  }

  /// 原版 h:184 + spaces-inl.h:37-38 单页便捷 ctor。
  #[inline]
  #[must_use]
  pub fn from_page(
    page: *const NormalPage,
    next_page: impl Fn(*const NormalPage) -> *const NormalPage,
  ) -> Self {
    Self::new(page, next_page(page))
  }

  /// 原版 h:186 `iterator begin()`。
  #[inline]
  #[must_use]
  pub fn begin(&self) -> ConstPageIterator {
    ConstPageIterator::new(self.begin_)
  }

  /// 原版 h:187 `iterator end()`。
  #[inline]
  #[must_use]
  pub fn end(&self) -> ConstPageIterator {
    ConstPageIterator::new(self.end_)
  }
}

// ══════════════════════════════════════════════════════════════════════════
// SpaceWithLinearArea (原版 h:194-204 + spaces.cc:39-41)
// ══════════════════════════════════════════════════════════════════════════

/// 原版 h:194-204 `class SpaceWithLinearArea : public Space` — 带线性分配区 (LAB) 的空间基类。
/// h:196-199 注释: 复用既有 allocator, 不自建 MainAllocator 实例。
///
/// 落地形: trait 扩展位 (Space 之抽象派生); cc:39-41 构造透传面即
/// `SpaceBase::new(heap, id, free_list)` (不另立 ctor); h:203 `friend class MainAllocator`
/// → Rust 无友元, 面以 pub 承接 (memory_allocator 台账 16 先例)。
pub trait SpaceWithLinearArea: Space {
  /// 原版 h:198-201 `virtual AllocatorPolicy* CreateAllocatorPolicy(MainAllocator*) = 0` —
  /// unique_ptr 槽 (main_allocator.h:353) 的 `Box<dyn AllocatorPolicy>` 承接先例复用。
  fn create_allocator_policy(
    &mut self,
    allocator: &mut MainAllocator,
  ) -> Box<dyn AllocatorPolicy + '_>;
}

// 原版 h:203 注释面 (supports_allocation_info 等) 属 LAB 具体空间 (skel 注:
// allocation_info 面) → semi-space/paged-new-space 实体他票, 见台账。

// ══════════════════════════════════════════════════════════════════════════
// SpaceIterator / MemoryChunkIterator (原版 h:206-230 + cc:43-64 + inl:92-108)
// ══════════════════════════════════════════════════════════════════════════

/// 原版 h:206-217 `class SpaceIterator : public Malloced` — 可变空间序扫迭代器。
/// 依赖 Heap::space(int) (heap.h 实体他票, Isolate 全链) → 方法体推迟, 见台账 1。
#[derive(Debug)]
pub struct SpaceIterator {
  /// 原版 h:215 `Heap* heap_`。
  pub heap: *mut Heap,
  /// 原版 h:216 `int current_space_` — from enum AllocationSpace。
  pub current_space: i32,
}

impl SpaceIterator {
  /// 原版 h:208 声明 + cc:43-44 `SpaceIterator(Heap* heap)
  ///   : heap_(heap), current_space_(FIRST_MUTABLE_SPACE) {}`。
  #[must_use]
  pub fn new(heap: *mut Heap) -> Self {
    Self {
      heap,
      current_space: FIRST_MUTABLE_SPACE as i32,
    }
  }

  /// 原版 h:211 声明 + cc:48-57 `bool HasNext()` — while 序扫跳空空间。
  /// 推迟 (台账 1): 循环体消费 `heap_->space(current_space_)` (heap.h 他票)。
  pub fn has_next(&mut self) -> bool {
    todo!("v8: spaces.cc:48-57 — 依赖 Heap::space(int) (台账 1)")
  }

  /// 原版 h:212 声明 + cc:59-64 `Space* Next()` — DCHECK_LE(current, LAST) + 取后自增。
  /// 推迟 (台账 1): 返回 Space* 异构面同上。
  pub fn next_space(&mut self) -> *mut dyn SpaceMarker {
    todo!("v8: spaces.cc:59-64 — 依赖 Heap::space(int) (台账 1)")
  }
}

/// 原版 h:206/h:220-230 处 `Space*`/`MutablePage*` 异构槽的 trait 标记承接:
/// Space trait 含关联类型不可 dyn 化 → 本空标记 trait 仅作 Next() 返回位型的落点占位,
/// 具体空间实现 Space 时自动实现 (sealed 由 Space 派生约束保证; 台账 1 落地后移除)。
///
/// 差异申报: 本 trait 不复写任何虚函数, 仅承接 C++ 基类指针位型 (memory_pool.rs
/// PooledPage 位型承接同型思路)。
pub trait SpaceMarker {}

impl<S: Space> SpaceMarker for S {}

/// 原版 h:219-230 `class MemoryChunkIterator` — 跨空间全堆 chunk 迭代 (经 SpaceIterator)。
/// 依赖 SpaceIterator (台账 1) 与 MutablePage::list_node().next() (mutable-page.h 他票) →
/// 方法体推迟, 见台账 2。
#[derive(Debug)]
pub struct MemoryChunkIterator {
  /// 原版 h:228 `SpaceIterator space_iterator_` — h:222 ctor 以 heap 构造。
  pub space_iterator: SpaceIterator,
  /// 原版 h:229 `MutablePage* current_chunk_ = nullptr`。
  pub current_chunk: *mut MutablePage,
}

impl MemoryChunkIterator {
  /// 原版 h:222 `explicit MemoryChunkIterator(Heap* heap) : space_iterator_(heap) {}`。
  #[must_use]
  pub fn new(heap: *mut Heap) -> Self {
    Self {
      space_iterator: SpaceIterator::new(heap),
      current_chunk: null_mut(),
    }
  }

  /// 原版 h:224 声明 + spaces-inl.h:92-102 `bool HasNext()` —
  /// current 命中即真; 否则序扫空间取 first_page。
  /// 推迟 (台账 2): 依赖 SpaceIterator::HasNext/Next (台账 1) 与 Space::first_page 异构面。
  pub fn has_next(&mut self) -> bool {
    todo!("v8: spaces-inl.h:92-102 — 依赖 SpaceIterator/Heap::space (台账 1/2)")
  }

  /// 原版 h:225 声明 + spaces-inl.h:104-108 `MutablePage* Next()` —
  /// 取 current 后 `chunk->list_node().next()` 推进。
  /// 推迟 (台账 2): list_node 面属 mutable-page.h 实体他票。
  pub fn next_chunk(&mut self) -> *mut MutablePage {
    todo!("v8: spaces-inl.h:104-108 — 依赖 MutablePage::list_node().next() (台账 2)")
  }
}

// h:206 区前向声明清单 (spaces.h:39-47: AllocationObserver/FreeList/Heap/Isolate/
// LargeObjectSpace/LargePage/ObjectIterator/PagedSpaceBase/SemiSpace) —
// AllocationObserver/FreeList/Heap/LargePage/NormalPage/MutablePage/ObjectIterator 均已
// 复用 crate 内承接型 (本文件 use 段); PagedSpaceBase/SemiSpace 属 HC-4 下波空间实体票,
// 本票无 Rust 消费点 → 不落空型 (memory_allocator.rs:81-84 同法)。

// spaces-inl.h:40-90 `OldGenerationMemoryChunkIterator::next()` — 该类的**声明**在
// paged-spaces.h/他宿主 (spaces-inl.h 仅实现体, inl:40-41 ctor 同), 不属本票文件族
// 声明面 → 整体不落地, 登记台账 3 (inl:40-90 六态序换状态机供下波参照)。

// ══════════════════════════════════════════════════════════════════════════
// NoFreeList (unique_ptr<FreeList> = nullptr 的空占位承接)
// ══════════════════════════════════════════════════════════════════════════

/// 无自由链表空间的泛参占位 (原版 LargeObjectSpace 构造 `Space(heap, id, nullptr)`
/// cc:53-54)。链面从不被消费 (Option::None), 三个纯虚槽保留 todo! 以满足 trait 完备。
#[derive(Debug)]
pub struct NoFreeList {
  /// 基类字段槽 (free_list.rs FreeListBase 复用, 零类目零块宽退化形)。
  base: FreeListBase,
}

impl Default for NoFreeList {
  /// 承接位退化形: 无类目无最小块 (入参 0/0 为本仓占位侧选择 — 原版从不构造该链,
  /// cc:53-54 直传 nullptr; 签名位型对位 free-list.cc:134-137 `FreeList(
  /// number_of_categories, min_block_size)`。HCR1-F7 锚点校正: 原注裸 "cc:" 前缀在
  /// 本文件语境指向 spaces.cc 失实)。
  fn default() -> Self {
    Self {
      base: FreeListBase::new(0, 0),
    }
  }
}

impl FreeList for NoFreeList {
  fn base(&self) -> &FreeListBase {
    &self.base
  }
  fn base_mut(&mut self) -> &mut FreeListBase {
    &mut self.base
  }

  fn allocate<E: FreeListEnv>(
    &mut self,
    _env: &mut E,
    _heap: *const FreeListHeap,
    _size_in_bytes: usize,
    _node_size: &mut usize,
    _origin: AllocationOrigin,
  ) -> Tagged<FreeSpace> {
    todo!("v8: free-list.h:159-161 — NoFreeList 占位不可服务分配 (cc:53-54 nullptr 语义)")
  }

  fn get_page_for_size<E: FreeListEnv>(
    &mut self,
    _env: &mut E,
    _size_in_bytes: usize,
  ) -> *mut free_list::NormalPage {
    todo!("v8: free-list.h:163-165 — NoFreeList 占位 (同上)")
  }

  fn select_free_list_category_type(&self, _size_in_bytes: usize) -> FreeListCategoryType {
    todo!("v8: free-list.h:262-265 — NoFreeList 占位 (同上)")
  }
}

// ══════════════════════════════════════════════════════════════════════════
// 不变式自查 (编译期; skel 提取不变式承接)
// ══════════════════════════════════════════════════════════════════════════

// SpaceIterator 从 FIRST_MUTABLE_SPACE 起扫 (cc:44 构造) — 值面编译期锚:
const _: () = assert!(FIRST_MUTABLE_SPACE as i32 == AllocationSpace::NewSpace as i32);
// 序扫终点 = LAST_MUTABLE_SPACE (cc:49 循环界, h:216 注释 "from enum AllocationSpace")。
const _: () = assert!(LAST_MUTABLE_SPACE as i32 == AllocationSpace::TrustedLoSpace as i32);

// ══════════════════════════════════════════════════════════════════════════
// 推迟台账 (真实耦合面; 双行号 + 解锁前置; 形态对齐 memory_allocator.rs 文末台账)
// ══════════════════════════════════════════════════════════════════════════
// 1. SpaceIterator 方法面 — h:206-217 结构/字段已落 (本文件), cc:48-57 `HasNext()` 与
//    cc:59-64 `Next()` 体 todo!。原因: 消费 `heap_->space(current_space_)`
//    (src/heap/heap.h Heap 类本体属他票; 本 crate Heap 为 memory_allocator.rs:61 空 enum
//    承接位)。解锁前置: Heap::space(int) 落地。cc:60 DCHECK_LE 位随体接入 (F7 校正:
//    DCHECK_LE 实测在 spaces.cc:60, 原注 59 为函数入口行)。
// 2. MemoryChunkIterator 方法面 — h:219-230 结构/inl:222 ctor 已落, inl:92-102 `HasNext()`
//    与 inl:104-108 `Next()` 体 todo!。原因: 依赖台账 1 + `MutablePage::list_node().next()`
//    (mutable-page.h 实体他票) + Space::first_page 异构返回 (需 dyn 或 enum 分发,
//    Space trait 含关联类型不可 dyn 化)。解锁前置: 页实体票 + Heap 票 (下波以
//    enum_dispatch 或具体空间收敛形接入)。
// 3. OldGenerationMemoryChunkIterator — inl:40-90 六态序换状态机 (kOldSpace→…→kFinished,
//    std::variant<PageIterator, LargePageIterator>)。该类的声明宿主不在本票文件族
//    (spaces-inl.h 仅实现体; 声明在 paged-spaces.h 他票) → 本票不落, 状态机注记留此供
//    下波照抄。HCR1-F4 处置: 原先行落地的 spaces_enums.rs 系未挂 lib.rs 的孤儿文件且
//    全仓零 Rust 消费者 (与 memory_allocator.rs:105 FreeMode 重复型), 已删除 → 该枚举面
//    未落地, 下波随 paged-spaces 票在其宿主模块自声明 (原禁碰互斥解除; readme 台账同记)。
// 4. SpaceWithLinearArea::CreateAllocatorPolicy — h:201 纯虚槽 trait 声明已落, 具体覆写
//    落点 (SemiSpace/NewSpace/LO 无 LAB) 属 PagedSpace/NewSpaces 实体票 (HC-4 下波);
//    main_allocator.rs 策略族 83 todo 未填 → 本票不产出策略实例。解锁前置: HC-2c 填体票。
// 5. Space::InitializePage 基类默认 — h:100 原版 UNREACHABLE 以空指针承接 (本文件落点),
//    PagedSpace 族覆写实体 (normal-page 页头初始化) 属他票。
// 6. PageList 的侵入式节点推进 — 原版 list.h ListNode 双链在页元数据内, front/back/
//    push/remove 已承接为序列表; `page->next_page()` (spaces-inl.h:24-27) 与
//    `chunk->list_node().next()` (inl:106) 判据以闭包/env 注入, 页实体票落地后改为直调。
//    双存储同步约束 (HCR1-F10 补记): 页实体落地后序表 Vec 与页内 list_node 链为两份
//    存储, push/remove 两侧必须同点同步更新 (大空间即 AddPage/RemovePage 落点,
//    large-spaces.cc:193/201), 任一侧漏动即序分叉 — 接入时以单一事实源收敛或加序比对。
// 7. h:39-47 前向声明的 PagedSpaceBase/SemiSpace — 本票无消费点, 不落空型
//    (memory_allocator.rs:81-84 同法); HC-4 下波落地时于其模块自声明。
// 8. skel 字段面注 `external_backing_store_bytes_/allocated_bytes_` (spaces.rs.skel:7-8) —
//    本仓 spaces.h 实态无此字段 (位于 AllocationStats, allocation.rs:174 已落地) →
//    不承接, 票面旧版叙事申报见模块头。
// 9. h:122 static_assert — 已落编译期 const (本文件尾部 const _ 区)。
// 10. base-space.h:16 `class Heap;` 前向 + h:20 `Malloced` 基类 (operator new/delete 定制) —
//     Malloced 仅禁拷贝/堆分配标记, Rust 所有权模型天然承接 (不派生 Clone 即 h:23-24
//     = delete), 不落空型。
// 11. spaces-inl.h:14-18 include 的 new-spaces/paged-spaces/main-allocator-inl — 实现体
//     消费面均随台账 1/2/3 与下波空间实体票接入。
// 12. Print 虚面 (h:112-114) — trait 槽已落 (#[cfg(debug_assertions)] 默认空体);
//     LargeObjectSpace 覆写消费 i::Print(obj, os) (ostreams 他票) → large_spaces.rs
//     台账 5 双记 (HCR1-F3 归位: 覆写已改为 impl Space 的 print(&mut self) 槽形)。
// 13. PageList::push_back 缺前置 DCHECK — list.h:31-32 `DCHECK(!element->list_node()
//     .next() && !element->list_node().prev())` (入列前页未在任何表) 不可在序表承接形上
//     判 (list_node 属页实体他票, 与台账 6 同源解锁); 页实体落地后随直调改造补入。
// 20. base-space.h:44-47 `#ifdef VERIFY_HEAP virtual void Verify(...) = 0` — 目标配置
//     (DEBUG/TSAN 关, VERIFY_HEAP 随 debug 配置) 展开侧剥除 → 不落地
//     (memory_allocator.rs 台账 20 声明面清单同法; large_spaces.rs 台账 4 双记 LO 覆写)。
// 21. h:54-55 宏的 MaxRegularCodeObjectSize 消费 InstructionStream::kHeaderSize 实体他票 →
//     K_INSTRUCTION_STREAM_HEADER_SIZE_DERIVED 推导常量代入 (memory_allocator.rs:539-540
//     台账 10 同型)。
// 22. base-space.h:54-62 Account{,Un}committed 的 DCHECK_GE 无回绕/无下溢断言已以
//     debug_assert 落 (本文件 SpaceBase 落点); protected 访问级 → pub 承接
//     (memory_allocator 台账 16 先例)。
// 23. h:84 GetObjectIterator 纯虚槽 — trait 不复写 (原因见 trait 内 doc 注), LO 固有方法
//     落点见 large_spaces.rs cc:274-278 槽; 该申报与台账 2 的 dyn 限制同因。
