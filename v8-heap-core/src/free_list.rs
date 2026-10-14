//! FreeList 族: 自由链表与类目 (FreeListCategory / FreeList / FreeListMany /
//! FreeListManyCached / FreeListManyCachedFastPathBase / FreeListManyCachedFastPath /
//! FreeListManyCachedFastPathForNewSpace / FreeListManyCachedOrigin)。
//! 对应原版 v8/src/heap/free-list.h (525 行) + free-list.cc (603 行) (HC-3 落地)。
//! 语义依赖面 free-list-inl.h (48 行, 非票面独立文件) 的 4 个定义 (inl:17-20/22-24/26-34/36-42)
//! 一并就地承接, 台账行号以 inl: 前缀标注。
//!
//! 目标配置 (与 tool/src/heap/free-list.{h,cc}.js clang -E 展开侧同源):
//!   V8_COMPRESS_POINTERS + V8_ENABLE_SANDBOX + V8_EXTERNAL_CODE_SPACE, DEBUG/TSAN 关。
//!
//! 类层级承接 (票面差异申报):
//! - C++ 单继承链 → Rust 组合链: FreeListMany { base } ⊂ FreeListManyCached { many } ⊂
//!   FreeListManyCachedFastPathBase { cached } ⊂ FreeListManyCachedFastPath { fast_base } /
//!   FreeListManyCachedFastPathForNewSpace { fast_base } ⊂ FreeListManyCachedOrigin { fast_path }。
//!   虚函数分派以 [`FreeList`] trait 承接 (本仓禁 dyn, 同 allocation_observer 先例):
//!   每层 impl 显式转接=原版继承/覆写逐槽对照 (覆写层 cc:262/267/272/287/301/334/375/441)。
//! - 原版 `FreeList` 基类字段+非虚实现 → [`FreeListBase`] 结构 (Rust 无同名类, 申报命名)。
//! - 原版 FreeListCategory* 挂载在 NormalPage 元数据 (新版 memory_chunk 无 free_list 槽位,
//!   HC-1 实测勘定): 类目承载=页元数据所有、本族经 [`FreeListEnv::page_free_list_category`]
//!   取裸指针挂链, 与原版 `page->free_list_category(type)->Free(...)` (cc:200/318) 同构。
//! - Tagged<FreeSpace> 节点成员函数 (objects/free-space 他票) 与 NormalPage/Isolate/Heap 运行态
//!   (heap 他票) → [`FreeListEnv`] 窄 trait 承接 (MetadataTableEnv 先例), 推迟双行号见文末台账。
//! - std::unique_ptr<FreeList> 工厂 (cc:139/143) → Rust 具名值返回 (无堆装箱多态, 申报)。
//! - v8_flags 全局 (h:447-449 构造读取, cc:467 DCHECK) → 构造/方法参数注入 &v8_flags::Flags (申报)。

use core::ptr::{from_mut, from_ref, null_mut};
use std::{
  io::stdout,
  ptr::eq,
  sync::atomic::{AtomicUsize, Ordering},
};

use v8_common::{Address, K_REGULAR_PAGE_SIZE, K_TAGGED_SIZE, KB, WritableFreeSpace};
use v8_flags::Flags;
use v8_objects_core::tagged::Tagged;

use crate::{allocation::AllocationOrigin, memory_chunk::MemoryChunk};

// ══════════════════════════════════════════════════════════════════════════
// 类型别名与常量 (原版 h:36-41)
// ══════════════════════════════════════════════════════════════════════════

/// 原版 h:36 `using FreeListCategoryType = int32_t`。
pub type FreeListCategoryType = i32;

/// 原版 h:38 `static constexpr FreeListCategoryType K_FIRST_CATEGORY = 0`。
pub const K_FIRST_CATEGORY: FreeListCategoryType = 0;

/// 原版 h:39 `static constexpr FreeListCategoryType kInvalidCategory = -1`。
pub const K_INVALID_CATEGORY: FreeListCategoryType = -1;

/// 原版 h:41 `enum FreeMode { kLinkCategory, kDoNotLinkCategory }` (非限定枚举, 隐式 0/1)。
/// 变体去 k 前缀 PascalCase 承接 (同 v8-common LanguageMode 先例), 值不变。
#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FreeMode {
  /// 原版 h:41 首项 kLinkCategory (值 0)
  LinkCategory = 0,
  /// 原版 h:41 次项 kDoNotLinkCategory (值 1)
  DoNotLinkCategory = 1,
}

// ══════════════════════════════════════════════════════════════════════════
// 前向声明承接 (原版 h:22-34; FreeSpace 实体经 h:13 include free-space.h)
// ══════════════════════════════════════════════════════════════════════════

/// 原版 h:22-24 namespace heap{ HeapTester / TestCodePageAllocatorScope } + h:26-34 前向声明
/// (AllocationObserver/FreeList/Isolate/LargeObjectSpace/LargePage/LinearAllocationArea/
/// NormalPage/PagedSpace/SemiSpace)。Rust 无友元; 仅签名用到的三个以 opaque 承接,
/// 其余 (LargeObjectSpace/LargePage/LinearAllocationArea/PagedSpace/SemiSpace/
/// AllocationObserver 及 heap tester ×2) 在本族 h/cc 签名中零使用, 台账登记不落 (h 脚本③锚)。
/// 原版 heap/normal-page.h NormalPage 实体 (他票) → 仅透传指针。
pub enum NormalPage {}
/// 原版 common/globals.h Isolate 前向声明 (h:29) → 仅透传指针。
pub enum Isolate {}
/// 原版 heap/heap.h Heap (cc:10 include heap.h) → 仅透传指针 (写屏障上下文令牌)。
pub enum Heap {}
/// 原版 objects/free-space.h FreeSpace 实体 (objects 他票, h:13 include) →
/// 本 crate 以空 enum 承接 Tagged<FreeSpace> 节点位型; 成员函数面走 [`FreeListEnv`]。
pub enum FreeSpace {}

// ══════════════════════════════════════════════════════════════════════════
// FreeListEnv — 本族运行态消费面窄 trait (objects/free-space + heap/normal-page 他票最小集;
// MetadataTableEnv 先例, 交票申报: 原版节点->Size()/next()/SetNext() 与页记账直改 env 法)
// ══════════════════════════════════════════════════════════════════════════

/// FreeList 族对 FreeSpace 节点 (objects 票) 与 NormalPage/Isolate (heap 票) 的窄接口。
/// 每法注释给出原版调用行号; 实现方 (上层 spaces 票) 在此落真实读写屏障与页元数据。
pub trait FreeListEnv {
  /// `node->Size()` / `node->size(kRelaxedLoad)` (cc:42/47/60/563; cc:56 循环读同源)。
  fn node_size(&self, node: Tagged<FreeSpace>) -> usize;

  /// `node->next()` (cc:46/57/65/75/81/88 间接/122/564/574)。
  fn node_next(&self, node: Tagged<FreeSpace>) -> Tagged<FreeSpace>;

  /// `node->SetNext(heap, writable_free_space, next)` (cc:75/77-81/99;
  /// 实体 objects/free-space-inl.h 他票, 写通道 guard 由 writable_free_space 承载)。
  fn node_set_next(
    &mut self,
    node: Tagged<FreeSpace>,
    heap: *const Heap,
    writable_free_space: &WritableFreeSpace,
    next: Tagged<FreeSpace>,
  );

  /// `MemoryChunk::FromHeapObject(node)->Metadata()->is_executable()` (cc:68-70;
  /// Metadata 族属 memory_chunk 推迟台账 1, 上层实现)。
  fn node_is_executable(&self, node: Tagged<FreeSpace>) -> bool;

  /// 可执行分支写通道 (cc:71-74 `WritableJitPage jit_page(addr, size);
  /// free_space = jit_page.FreeRange(addr, size)`; 实现侧走 v8_common::WritableJitPage,
  /// 原版注册表 CHECK 失败即致命, Rust 实现方以 check 语义收 Error)。
  fn free_range_executable(&mut self, address: Address, size: usize) -> WritableFreeSpace;

  /// `NormalPage::FromAddress(isolate, start)` (cc:188/306)。
  fn page_from_address(&self, isolate: *const Isolate, address: Address) -> *mut NormalPage;

  /// `NormalPage::FromHeapObject(node)` (inl:30; cc:250/365/427)。
  fn page_from_heap_object(&self, node: Tagged<FreeSpace>) -> *mut NormalPage;

  /// `page->DecreaseAllocatedBytes(bytes)` (cc:189/307)。
  fn page_decrease_allocated_bytes(&mut self, page: *mut NormalPage, bytes: usize);

  /// `page->IncreaseAllocatedBytes(bytes)` (cc:250/365/427)。
  fn page_increase_allocated_bytes(&mut self, page: *mut NormalPage, bytes: usize);

  /// `page->add_wasted_memory(bytes)` (cc:193/311/492)。
  fn page_add_wasted_memory(&mut self, page: *mut NormalPage, bytes: usize);

  /// `page->free_list_category(type)` (cc:200/318; 实体 normal-page.h:85 取元数据数组槽)。
  fn page_free_list_category(
    &mut self,
    page: *mut NormalPage,
    type_: FreeListCategoryType,
  ) -> *mut FreeListCategory;

  /// `page->ForAllFreeListCategories(callback)` (cc:487; 实体 normal-page-inl.h:38-42
  /// 遍历页元数据 categories_ 数组全部槽位, 含未挂链者)。
  fn page_for_all_free_list_categories<F: FnMut(*mut FreeListCategory)>(
    &mut self,
    page: *mut NormalPage,
    callback: F,
  );

  /// `isolate->heap()` (cc:200/318; 作为 SetNext 写屏障上下文令牌透传)。
  fn isolate_heap(&self, isolate: *const Isolate) -> *const Heap;
}

// ══════════════════════════════════════════════════════════════════════════
// FreeListCategory (原版 h:44-126; 实体 cc:22-129, cc:553-576; inl:17-24)
// ══════════════════════════════════════════════════════════════════════════

/// 自由链表类目: 维护一块自由内存块的链表 (原版 h:43-126)。
///
/// 承载 (票面差异申报): 原版类目对象住在 NormalPage 元数据内 (h:122-125 friend 面),
/// FreeList 只持裸指针挂双链; Rust 照此模型 — 实例由页元数据所有 (经
/// [`FreeListEnv::page_free_list_category`] 出槽), prev_/next_/categories_ 以裸指针承接。
#[derive(Debug)]
pub struct FreeListCategory {
  /// 类目类型 (原版 h:109-110 `FreeListCategoryType type_ = kInvalidCategory`)。
  type_: FreeListCategoryType,
  /// 本类目全部块可用字节 (原版 h:112-114 `uint32_t available_ = 0`)。
  available_: u32,
  /// 链顶 FreeSpace (原版 h:116-117 `Tagged<FreeSpace> top_`)。
  top_: Tagged<FreeSpace>,
  /// 双链前驱 (原版 h:119 `FreeListCategory* prev_ = nullptr`)。
  prev_: *mut FreeListCategory,
  /// 双链后继 (原版 h:120 `FreeListCategory* next_ = nullptr`)。
  next_: *mut FreeListCategory,
}

impl Default for FreeListCategory {
  /// 原版 h:110/114 字段初值 (type_=kInvalidCategory, available_=0; top_/prev_/next_ 默认空)。
  fn default() -> Self {
    Self {
      type_: K_INVALID_CATEGORY,
      available_: 0,
      top_: Tagged::default(),
      prev_: null_mut(),
      next_: null_mut(),
    }
  }
}

/// 原版 h:94-96 注释: debug 构建手工走链精确计链长到 kVeryLongFreeList 为止。
pub const K_VERY_LONG_FREE_LIST: i32 = 500;

impl FreeListCategory {
  /// 原版 h:46-51 `Initialize(FreeListCategoryType type)` — 置类型并清计数/链 (不动 top_)。
  pub fn initialize(&mut self, type_: FreeListCategoryType) {
    self.type_ = type_;
    self.available_ = 0;
    self.prev_ = null_mut();
    self.next_ = null_mut();
  }

  /// 原版 cc:22-28 `Unlink(FreeList* owner)` — 从自由链表摘出本类目。
  /// 已挂链且链顶非空时回收 owner 的 available 记账 (h:53 注释 Unlinks)。
  pub fn unlink<O: FreeList>(&mut self, owner: &mut O) {
    if self.is_linked(owner) && !self.top().is_null() {
      owner.decrease_available_bytes(self.available_ as usize);
    }
    self.set_prev(null_mut());
    self.set_next(null_mut());
  }

  /// 原版 cc:30-34 `Reset(FreeList* owner)` — 摘链 + 清 top_ + 清零 available_。
  pub fn reset<O: FreeList>(&mut self, owner: &mut O) {
    self.unlink(owner);
    self.set_top(Tagged::default());
    self.available_ = 0;
  }

  /// 原版 cc:126-129 `Relink(FreeList* owner)` — DCHECK(!is_linked) 后重新挂链 (h:60-61 注释)。
  pub fn relink<O: FreeList>(&mut self, owner: &mut O) {
    debug_assert!(!self.is_linked(owner)); // cc:127
    owner.add_category(self);
  }

  /// 原版 cc:93-110 `Free(const Heap*, const WritableFreeSpace&, FreeMode, FreeList*)`。
  /// 把 writable_free_space 处已形成的 FreeSpace 压入本类目链顶, 并按 mode 挂链。
  pub fn free<O: FreeList>(
    &mut self,
    env: &mut impl FreeListEnv,
    heap: *const Heap,
    writable_free_space: &WritableFreeSpace,
    mode: FreeMode,
    owner: &mut O,
  ) {
    // cc:96-97 `Cast<FreeSpace>(HeapObject::FromAddress(addr))` — Tagged 位型即强引用 tag=01;
    // SAFETY: 原版 h:150-151 前置约定 — 自由块起始字对齐且尺寸非零, writable 通道由调用方守。
    let free_space: Tagged<FreeSpace> =
      unsafe { Tagged::from_raw_address(writable_free_space.address()) };
    // cc:98 DCHECK_EQ(free_space->Size(), writable_free_space.Size())
    debug_assert!(env.node_size(free_space) == writable_free_space.size() as usize);
    env.node_set_next(free_space, heap, writable_free_space, self.top()); // cc:99
    self.set_top(free_space); // cc:100
    let size_in_bytes = writable_free_space.size() as usize; // cc:101
    // cc:102 `available_ += size_in_bytes` — uint32 隐式窄化照录
    self.available_ += size_in_bytes as u32;
    if mode == FreeMode::LinkCategory {
      // cc:103-109
      if self.is_linked(owner) {
        owner.increase_available_bytes(size_in_bytes); // cc:104-105
      } else {
        owner.add_category(self); // cc:107
      }
    }
  }

  /// 原版 cc:36-50 `PickNodeFromList(size_t minimum_size, size_t* node_size)` —
  /// 单次尝试取链顶不小于 minimum_size 的节点; 不足则置零返回 null (h:67-69 注释)。
  pub fn pick_node_from_list(
    &mut self,
    env: &mut impl FreeListEnv,
    minimum_size: usize,
    node_size: &mut usize,
  ) -> Tagged<FreeSpace> {
    let node = self.top(); // cc:38
    debug_assert!(!node.is_null()); // cc:39
    // cc:40-41 DCHECK(BasePage::FromHeapObject(Isolate::Current(), node)->CanAllocateOnChunk())
    //   — MutablePage 票面 (推迟台账 2), 目标配置展开侧剥除
    if env.node_size(node) < minimum_size {
      *node_size = 0; // cc:43
      return Tagged::default(); // cc:44 `return Tagged<FreeSpace>()`
    }
    self.set_top(env.node_next(node)); // cc:46
    *node_size = env.node_size(node); // cc:47
    self.update_counters_after_allocation(*node_size); // cc:48
    node
  }

  /// 原版 cc:52-91 `SearchForNodeInList(const Heap*, minimum_size, node_size)` —
  /// 走全链找首个够大节点, 从链中摘出并保持前驱链接 (可执行内存走 WritableJitPage 写通道)。
  pub fn search_for_node_in_list(
    &mut self,
    env: &mut impl FreeListEnv,
    heap: *const Heap,
    minimum_size: usize,
    node_size: &mut usize,
  ) -> Tagged<FreeSpace> {
    let mut prev_non_evac_node = Tagged::<FreeSpace>::default(); // cc:55
    let mut cur_node = self.top(); // cc:56
    while !cur_node.is_null() {
      // cc:56-57 for 递增位
      // cc:58-59 DCHECK CanAllocateOnChunk (推迟台账 2, 展开侧剥除)
      let size = env.node_size(cur_node); // cc:60 `size(kRelaxedLoad)`
      if size >= minimum_size {
        debug_assert!((self.available_ as usize) >= size); // cc:62 DCHECK_GE(available_, size)
        self.update_counters_after_allocation(size); // cc:63
        if cur_node == self.top() {
          self.set_top(env.node_next(cur_node)); // cc:64-66
        }
        if !prev_non_evac_node.is_null() {
          // cc:67-83 前驱改链: 按前驱块所在 chunk 元数据可执行性选写通道
          let next = env.node_next(cur_node); // cur_node->next() (cc:75/81)
          let prev_addr = prev_non_evac_node.address();
          let prev_size = env.node_size(prev_non_evac_node); // prev_non_evac_node->Size()
          let free_space: WritableFreeSpace = if env.node_is_executable(prev_non_evac_node) {
            // cc:68-74 WritableJitPage jit_page(...); FreeRange(...)
            env.free_range_executable(prev_addr, prev_size)
          } else {
            // cc:77-81 WritableFreeSpace::ForNonExecutableMemory(...)
            WritableFreeSpace::for_non_executable_memory(prev_addr, prev_size)
          };
          env.node_set_next(prev_non_evac_node, heap, &free_space, next); // cc:75/77-81
        }
        *node_size = size; // cc:84
        return cur_node; // cc:85
      }
      prev_non_evac_node = cur_node; // cc:88
      cur_node = env.node_next(cur_node); // cc:57 递增
    }
    Tagged::default() // cc:90
  }

  /// 原版 inl:17-20 `is_linked(FreeList* owner) const` (声明 h:78) —
  /// prev/next 任一在链, 或本类目正占 owner 的 categories_[type_] 槽。
  pub fn is_linked<O: FreeList>(&self, owner: &O) -> bool {
    self.is_linked_base(owner.base())
  }

  /// is_linked 的 base 侧实现 (供 FreeListBase 内部循环使用; inl:17-20 同式)。
  pub(crate) fn is_linked_base(&self, owner: &FreeListBase) -> bool {
    // 原版三指针比较 (inl:17-20), raw 指针等值走 ptr::eq
    !self.prev_.is_null()
      || !self.next_.is_null()
      || eq(
        owner.top(self.type_),
        from_ref(self) as *mut FreeListCategory,
      )
  }

  /// 原版 h:79 `bool is_empty() { return top().is_null(); }`。
  #[inline]
  pub fn is_empty(&self) -> bool {
    self.top().is_null()
  }

  /// 原版 h:80 `uint32_t available() const`。
  #[inline]
  pub fn available(&self) -> u32 {
    self.available_
  }

  /// 原版 cc:553-567 `SumFreeList()` — 走链累加块尺寸 (DEBUG 校验用)。
  /// cc:557-562 map 根 DCHECK (RootIndex/Isolate 耦合) 推迟台账 3, 展开侧剥除;
  /// cc:558-559 注释: 引导期 cur 与根的 map 都可能为空 → 不能走 cur->map()。
  pub fn sum_free_list(&self, env: &impl FreeListEnv) -> usize {
    let mut sum = 0usize; // cc:554
    let mut cur = self.top(); // cc:555
    while !cur.is_null() {
      sum += env.node_size(cur); // cc:563 `cur->size(kRelaxedLoad)`
      cur = env.node_next(cur); // cc:564
    }
    sum
  }

  /// 原版 cc:568-576 `FreeListLength()` — 走链计节点数。
  pub fn free_list_length(&self, env: &impl FreeListEnv) -> i32 {
    let mut length = 0i32; // cc:569
    let mut cur = self.top(); // cc:570
    while !cur.is_null() {
      length += 1; // cc:572
      cur = env.node_next(cur); // cc:573
    }
    length
  }

  /// 原版 h:85-91 `IterateNodesForTesting(Callback)` (模板回调) — 逐节点走链。
  /// 消费面锚: heap-unittest.cc:850-853 / heap-utils.cc:114-117。
  pub fn iterate_nodes_for_testing(
    &self,
    env: &impl FreeListEnv,
    mut callback: impl FnMut(Tagged<FreeSpace>),
  ) {
    let mut cur_node = self.top();
    while !cur_node.is_null() {
      callback(cur_node); // h:89
      cur_node = env.node_next(cur_node); // h:88 `cur_node = cur_node->next()`
    }
  }

  /// 原版 h:102 `Tagged<FreeSpace> top()` (private+friend, Rust pub 导出)。
  #[inline]
  pub fn top(&self) -> Tagged<FreeSpace> {
    self.top_
  }

  /// 原版 h:103 `void set_top(Tagged<FreeSpace> top)`。
  #[inline]
  pub fn set_top(&mut self, top: Tagged<FreeSpace>) {
    self.top_ = top;
  }

  /// 原版 h:104 `FreeListCategory* prev()`。
  #[inline]
  pub fn prev(&self) -> *mut FreeListCategory {
    self.prev_
  }

  /// 原版 h:105 `void set_prev(FreeListCategory* prev)`。
  #[inline]
  pub fn set_prev(&mut self, prev: *mut FreeListCategory) {
    self.prev_ = prev;
  }

  /// 原版 h:106 `FreeListCategory* next()`。
  #[inline]
  pub fn next(&self) -> *mut FreeListCategory {
    self.next_
  }

  /// 原版 h:107 `void set_next(FreeListCategory* next)`。
  #[inline]
  pub fn set_next(&mut self, next: *mut FreeListCategory) {
    self.next_ = next;
  }

  /// 原版 h:109-110 字段读出 (friend 消费面 cc:277/291/501/520 `category->type_`)。
  #[inline]
  pub fn type_(&self) -> FreeListCategoryType {
    self.type_
  }

  /// 原版 inl:22-24 `UpdateCountersAfterAllocation(size_t)` (声明 h:98-100) —
  /// 分配后仅更新本类目 available_ (owner 侧由调用点扣减, h:98 注释)。
  fn update_counters_after_allocation(&mut self, allocation_size: usize) {
    // cc 无越界检; uint32 隐式窄化照录 (inl:23 `available_ -= allocation_size`)
    self.available_ -= allocation_size as u32;
  }
}

// ══════════════════════════════════════════════════════════════════════════
// FreeList 基类字段 + 非虚实现 → FreeListBase (原版 h:128-290 类面; cc:134-204, 456-551, 578-599)
// ══════════════════════════════════════════════════════════════════════════

/// 原版 h:135 `class FreeList` 的字段与非虚实现承接 (申报命名: Rust trait [`FreeList`] 占同名)。
/// 构造 cc:134-137; 类目数组 cc:213 分配 / cc:217 释放 (Vec 回收, 无 Drop 面)。
#[derive(Debug)]
pub struct FreeListBase {
  /// 类目总数 (原版 h:273 `const int number_of_categories_ = 0`, cc:135 初始化列表)。
  number_of_categories_: i32,
  /// 末类目 (原版 h:274 `const FreeListCategoryType last_category_ = 0`, cc:136 = n-1)。
  last_category_: FreeListCategoryType,
  /// 最小块尺寸 (原版 h:275 `size_t min_block_size_ = 0`, cc:137)。
  min_block_size_: usize,
  /// 各类目链头数组 (原版 h:277 `FreeListCategory** categories_ = nullptr`;
  /// cc:213 `new FreeListCategory*[n]()` 零初始化)。
  categories_: Vec<*mut FreeListCategory>,
  /// 本链表可分配字节数 (原版 h:279-280 `size_t available_ = 0`)。
  available_: usize,
  /// 本链表不可用浪费字节 (原版 h:281-283 `std::atomic<size_t> wasted_bytes_ = 0`)。
  wasted_bytes_: AtomicUsize,
}

impl FreeListBase {
  /// 原版 cc:134-137 `FreeList::FreeList(int number_of_categories, size_t min_block_size)`。
  /// categories_ 此步仍空 (h:277 初值 nullptr), 由 FreeListMany ctor cc:213 分配。
  pub fn new(number_of_categories: i32, min_block_size: usize) -> Self {
    Self {
      number_of_categories_: number_of_categories,
      last_category_: number_of_categories - 1,
      min_block_size_: min_block_size,
      categories_: Vec::new(),
      available_: 0,
      wasted_bytes_: AtomicUsize::new(0),
    }
  }

  /// 原版 cc:213 `categories_ = new FreeListCategory*[number_of_categories_]()` —
  /// 零初始化数组分配 (仅 FreeListMany ctor 调用)。
  pub(crate) fn alloc_categories(&mut self) {
    self.categories_ = vec![null_mut(); self.number_of_categories_ as usize];
  }

  /// 原版 h:267-269 `FreeListCategory* top(FreeListCategoryType type) const` (protected)。
  #[inline]
  pub fn top(&self, type_: FreeListCategoryType) -> *mut FreeListCategory {
    self.categories_[type_ as usize]
  }

  // 原版 h:271 `inline NormalPage* GetPageForCategoryType(type)` 的实体 inl:26-34
  // 拆成 env 侧 (FreeList trait default get_page_for_category_type 承接, 见 trait)。

  /// 原版 h:171-174 `size_t Available()` — VerifyAvailable (h:247-249) 为 DEBUG DCHECK,
  /// 目标配置展开侧剥除 (调用点即返回 available_)。
  pub fn available(&self) -> usize {
    self.available_
  }

  /// 原版 h:177 `void IncreaseAvailableBytes(size_t bytes)`。
  #[inline]
  pub fn increase_available_bytes(&mut self, bytes: usize) {
    self.available_ += bytes;
  }

  /// 原版 h:178 `void DecreaseAvailableBytes(size_t bytes)`。
  #[inline]
  pub fn decrease_available_bytes(&mut self, bytes: usize) {
    self.available_ -= bytes;
  }

  /// 原版 h:180-182 `size_t wasted_bytes() const` (memory_order_relaxed)。
  pub fn wasted_bytes(&self) -> usize {
    self.wasted_bytes_.load(Ordering::Relaxed)
  }

  /// 原版 h:183-185 `void increase_wasted_bytes(size_t bytes)` (fetch_add relaxed)。
  pub fn increase_wasted_bytes(&mut self, bytes: usize) {
    self.wasted_bytes_.fetch_add(bytes, Ordering::Relaxed);
  }

  /// 原版 h:186-188 `void decrease_wasted_bytes(size_t bytes)` (fetch_sub relaxed)。
  pub fn decrease_wasted_bytes(&mut self, bytes: usize) {
    self.wasted_bytes_.fetch_sub(bytes, Ordering::Relaxed);
  }

  /// 原版 h:197 `int number_of_categories()`。
  #[inline]
  pub fn number_of_categories(&self) -> i32 {
    self.number_of_categories_
  }

  /// 原版 h:198 `FreeListCategoryType last_category()`。
  #[inline]
  pub fn last_category(&self) -> FreeListCategoryType {
    self.last_category_
  }

  /// 原版 h:200 `size_t min_block_size() const`。
  #[inline]
  pub fn min_block_size(&self) -> usize {
    self.min_block_size_
  }

  /// 原版 h:202-210 `ForAllFreeListCategories(type, Callback)` (模板回调, 链上逐个;
  /// 摘链安全: 先取 next 再回调, 同 cc 原版 current->next() 快照)。
  pub fn for_all_free_list_categories_of<F: FnMut(&mut FreeListCategory)>(
    &mut self,
    type_: FreeListCategoryType,
    mut callback: F,
  ) {
    let mut current = self.top(type_); // h:204
    while !current.is_null() {
      // SAFETY: categories_ 槽与 prev/next 均指向页元数据内存活类目 (原版裸指针别名模型);
      // 回调内允许 Unlink/Reset 本节点 (原版 cc:457-458/468-477 即如此), 链已先取 next 快照。
      let next = unsafe { (*current).next() }; // h:206
      callback(unsafe { &mut *current }); // h:207
      current = next; // h:208
    }
  }

  /// 原版 h:212-217 `ForAllFreeListCategories(Callback)` — 全类型遍历 (h:214 起于 K_FIRST_CATEGORY)。
  pub fn for_all_free_list_categories<F: FnMut(&mut FreeListCategory)>(&mut self, mut callback: F) {
    for i in K_FIRST_CATEGORY..self.number_of_categories() {
      // h:215 — static_cast<FreeListCategoryType>(i) (Rust i 已是 i32)
      self.for_all_free_list_categories_of(i, &mut callback);
    }
  }

  /// 原版 cc:500-517 `AddCategory(FreeListCategory*)` (虚, 基类默认实现)。
  /// 双链头插 + owner available 入账; 空类目直接 false。
  pub(crate) fn add_category_impl(&mut self, category: &mut FreeListCategory) -> bool {
    let type_ = category.type_(); // cc:501
    debug_assert!(type_ < self.number_of_categories_); // cc:502 DCHECK_LT
    let top = self.top(type_); // cc:503 `FreeListCategory* top = categories_[type]`
    if category.is_empty() {
      return false; // cc:505
    }
    debug_assert!(top != from_mut(category)); // cc:506 DCHECK_NE(top, category)
    // cc:508-513 双链头插 (Common double-linked list insertion)
    if !top.is_null() {
      // SAFETY: top 为活类目 (categories_ 槽语义), 与 category 不同实例 (cc:506 已检)。
      unsafe { (*top).set_prev(from_mut(category)) }; // cc:510
    }
    category.set_next(top); // cc:512
    self.categories_[type_ as usize] = from_mut(category); // cc:513
    self.increase_available_bytes(category.available() as usize); // cc:515
    true // cc:516
  }

  /// 原版 cc:519-540 `RemoveCategory(FreeListCategory*)` (虚, 基类默认实现)。
  /// 挂链时扣减 available, 双链摘除并清空 prev/next。
  pub(crate) fn remove_category_impl(&mut self, category: &mut FreeListCategory) {
    let type_ = category.type_(); // cc:520
    debug_assert!(type_ < self.number_of_categories_); // cc:521 DCHECK_LT
    let top = self.top(type_); // cc:522
    if category.is_linked_base(self) {
      self.decrease_available_bytes(category.available() as usize); // cc:524-525
    }
    // cc:528-539 双链摘除 (Common double-linked list removal)
    if top == from_mut(category) {
      self.categories_[type_ as usize] = category.next(); // cc:529-531
    }
    if !category.prev().is_null() {
      // SAFETY: prev 槽非空即活类目 (原版指针链不变量)。
      unsafe { (*category.prev()).set_next(category.next()) }; // cc:532-534
    }
    if !category.next().is_null() {
      // SAFETY: 同上。
      unsafe { (*category.next()).set_prev(category.prev()) }; // cc:535-537
    }
    category.set_next(null_mut()); // cc:538
    category.set_prev(null_mut()); // cc:539
  }

  /// 原版 cc:456-464 `Reset()` 基类实现 — 全类目 Reset(this) + 数组清空 + 双归零。
  /// (h:213 回调模板面在 Rust 循环内手拆链, 语义同 cc:457-458 lambda。)
  pub(crate) fn reset_impl(&mut self) {
    // cc:457-458 ForAllFreeListCategories(category->Reset(this))
    let cats = self.categories_.as_mut_ptr();
    for i in 0..self.number_of_categories_ as usize {
      let mut current = unsafe { *cats.add(i) };
      // SAFETY: 链上游走自页元数据活类目; next 快照先行 (原版 for 循环 current->next() 同构)。
      while !current.is_null() {
        let next = unsafe { (*current).next_ };
        let cat = unsafe { &mut *current };
        // Reset(owner) cc:30-34: Unlink cc:22-28 + set_top null + available_=0
        let linked = cat.is_linked_base(self);
        if linked && !cat.top().is_null() {
          self.available_ -= cat.available() as usize;
        }
        cat.prev_ = null_mut();
        cat.next_ = null_mut();
        cat.top_ = Tagged::default();
        cat.available_ = 0;
        current = next;
      }
    }
    // cc:459-461 `for (i = K_FIRST_CATEGORY; i < number_of_categories_; i++) categories_[i] = nullptr`
    for i in K_FIRST_CATEGORY as usize..self.number_of_categories_ as usize {
      unsafe { *cats.add(i) = null_mut() };
    }
    self.wasted_bytes_.store(0, Ordering::Relaxed); // cc:462 `wasted_bytes_ = 0`
    self.available_ = 0; // cc:463
  }

  /// 原版 cc:466-483 `ResetForNonBlackAllocatedPages()` 基类实现。
  /// cc:467 DCHECK(v8_flags.black_allocated_pages) — v8_flags 全局 → flags 参数注入 (申报)。
  pub(crate) fn reset_for_non_black_impl(&mut self, flags: &Flags) {
    debug_assert!(flags.black_allocated_pages()); // cc:467
    let cats = self.categories_.as_mut_ptr();
    for i in 0..self.number_of_categories_ as usize {
      let mut current = unsafe { *cats.add(i) };
      // SAFETY: 同 reset_impl; 原版 lambda cc:468-477 先取 top 反查 chunk (is_linked 后 Unlink)。
      while !current.is_null() {
        let next = unsafe { (*current).next_ };
        let cat = unsafe { &mut *current };
        if !cat.is_empty() {
          // cc:470-471 `auto* chunk = MemoryChunk::FromHeapObject(category->top());
          //   if (chunk->IsBlackAllocated())`
          // SAFETY: top 指向活 FreeSpace, 其页对齐块头在 base_address 处 (memory_chunk 契约)。
          let chunk = unsafe { &*MemoryChunk::from_heap_object(cat.top()) };
          if chunk.is_black_allocated() {
            // cc:472-473 category->Unlink(this); return; (跳 Reset, 类目保内容)
            let linked = cat.is_linked_base(self);
            if linked && !cat.top().is_null() {
              self.available_ -= cat.available() as usize;
            }
            cat.prev_ = null_mut();
            cat.next_ = null_mut();
            current = next;
            continue;
          }
        }
        // cc:476 category->Reset(this) (展开同 reset_impl 内联体)
        let linked = cat.is_linked_base(self);
        if linked && !cat.top().is_null() {
          self.available_ -= cat.available() as usize;
        }
        cat.prev_ = null_mut();
        cat.next_ = null_mut();
        cat.top_ = Tagged::default();
        cat.available_ = 0;
        current = next;
      }
    }
    // cc:478-480 数组清空
    for i in K_FIRST_CATEGORY as usize..self.number_of_categories_ as usize {
      unsafe { *cats.add(i) = null_mut() };
    }
    self.wasted_bytes_.store(0, Ordering::Relaxed); // cc:481
    self.available_ = 0; // cc:482
  }

  /// 原版 inl:36-42 `bool IsEmpty()` — 任一类目非空即 false。
  pub fn is_empty(&mut self) -> bool {
    let mut empty = true; // inl:37
    self.for_all_free_list_categories(|category| {
      if !category.is_empty() {
        empty = false; // inl:39
      }
    });
    empty
  }

  /// 原版 cc:578-589 `IsVeryLong()` (#ifdef DEBUG, 声明 h:244) —
  /// 走链累计节点数, 达 kVeryLongFreeList 提前 true。Rust 恒实现 (debug 侧与测试消费,
  /// release 调用点在目标配置展开剥除)。
  pub fn is_very_long(&mut self, env: &impl FreeListEnv) -> bool {
    let mut len = 0i32; // cc:580
    for i in K_FIRST_CATEGORY..self.number_of_categories_ {
      let mut it = FreeListCategoryIterator::new_base(self, i as FreeListCategoryType); // cc:581-582
      while it.has_next() {
        // cc:584 `len += it.Next()->FreeListLength()`
        let cat = it.next_ptr();
        len += unsafe { &*cat }.free_list_length(env);
        if len >= K_VERY_LONG_FREE_LIST {
          return true; // cc:585
        }
      }
    }
    false // cc:588
  }

  /// 原版 cc:591-599 `SumFreeLists()` (#ifdef DEBUG, 声明 h:243) —
  /// 全类目走链求和 (链超长时不可调用, cc:591-593 注释)。
  pub fn sum_free_lists(&mut self, env: &impl FreeListEnv) -> usize {
    let mut sum = 0usize; // cc:595
    self.for_all_free_list_categories(|category| {
      sum += category.sum_free_list(env); // cc:597
    });
    sum
  }

  /// 原版 h:247-249 `VerifyAvailable()` — DCHECK(IsVeryLong() || available_ == SumFreeLists())。
  pub fn verify_available(&mut self, env: &impl FreeListEnv) {
    debug_assert!(
      self.is_very_long(env) || self.available_ == self.sum_free_lists(env),
      "available_ 与走链求和须一致 (原版 h:248 DCHECK)"
    );
  }

  /// 原版 cc:542-551 `PrintCategories(FreeListCategoryType type)` —
  /// PrintF stdout 面 (同 v8-zone 先例: L0 不引 log 依赖, 直写标准输出, 写失败静默)。
  pub fn print_categories(&mut self, type_: FreeListCategoryType) {
    use std::io::Write;
    let mut it = FreeListCategoryIterator::new_base(self, type_); // cc:543
    let _ = write!(
      stdout(),
      "FreeList[{:p}, top={:p}, {}] ",
      // cc:544-545 static_cast<void*>(this) / categories_[type]
      from_ref(self),
      self.top(type_),
      type_
    );
    while it.has_next() {
      let current = it.next_ptr(); // cc:546-547
      let _ = write!(stdout(), "{current:?} -> "); // cc:548 %p (Rust 打印指针值)
    }
    let _ = writeln!(stdout(), "null"); // cc:550 `PrintF("null\n")`
  }
}

/// 原版 h:224-240 `FreeList::FreeListCategoryIterator final` (protected 内嵌类)。
pub struct FreeListCategoryIterator {
  /// h:239 `FreeListCategory* current_`
  current_: *mut FreeListCategory,
}

impl FreeListCategoryIterator {
  /// 原版 h:226-227 ctor — `current_(free_list->categories_[type])` (自 base 取槽)。
  pub fn new_base(free_list: &FreeListBase, type_: FreeListCategoryType) -> Self {
    Self {
      current_: free_list.top(type_),
    }
  }

  /// 原版 h:229 `bool HasNext() const`。
  #[inline]
  pub fn has_next(&self) -> bool {
    !self.current_.is_null()
  }

  /// 原版 h:231-236 `FreeListCategory* Next()` — DCHECK(HasNext()) + 快照前进。
  /// 命名避让 Iterator::next (clippy::should_implement_trait); 与 next_ref 成对。
  #[inline]
  pub fn next_ptr(&mut self) -> *mut FreeListCategory {
    debug_assert!(self.has_next()); // h:232
    let tmp = self.current_;
    // SAFETY: current_ 非空即活类目 (原版链不变量)。
    self.current_ = unsafe { (*tmp).next() }; // h:234
    tmp
  }

  /// 原版 h:231 Next() 的可引用形态 (本 crate 消费面 cc:170/547 的 &mut 化; 申报)。
  /// # Safety
  /// current_ 槽非空即活类目 — 与原版 ctor 前置同契约。
  #[inline]
  pub unsafe fn next_ref(&mut self) -> &mut FreeListCategory {
    // SAFETY: 由调用方维持原版链不变量 (HasNext 已 DCHECK)。
    unsafe { &mut *self.next_ptr() }
  }
}

// ══════════════════════════════════════════════════════════════════════════
// FreeList 虚面 trait (原版 h:135-290 虚函数/公有面; 分派承接面申报见模块头)
// ══════════════════════════════════════════════════════════════════════════

/// 原版 `class FreeList` 公有/保护虚面。本仓禁 dyn → 各具体类 impl 本 trait,
/// 覆写关系逐槽对照 cc 实体 (AddCategory/RemoveCategory/Free/Reset/
/// ResetForNonBlackAllocatedPages 可覆写; Allocate/GetPageForSize/
/// SelectFreeListCategoryType 纯虚)。
pub trait FreeList {
  /// 基类字段访问 (原版继承即字段内嵌; Rust 组合链逐层上转)。
  fn base(&self) -> &FreeListBase;
  /// 基类字段可变访问。
  fn base_mut(&mut self) -> &mut FreeListBase;

  // ── 纯虚面 (h:159-161/164-165/264-265) ──

  /// 原版 h:159-161 `virtual V8_WARN_UNUSED_RESULT Tagged<FreeSpace> Allocate(...)=0` —
  /// V8_WARN_UNUSED_RESULT → 返回 Tagged (null 即不可服务, h:155-158 注释)。
  #[must_use]
  fn allocate<E: FreeListEnv>(
    &mut self,
    env: &mut E,
    heap: *const Heap,
    size_in_bytes: usize,
    node_size: &mut usize,
    origin: AllocationOrigin,
  ) -> Tagged<FreeSpace>;

  /// 原版 h:163-165 `virtual NormalPage* GetPageForSize(size_t)=0`。
  fn get_page_for_size<E: FreeListEnv>(
    &mut self,
    env: &mut E,
    size_in_bytes: usize,
  ) -> *mut NormalPage;

  /// 原版 h:262-265 `virtual FreeListCategoryType SelectFreeListCategoryType(size_t)=0`。
  fn select_free_list_category_type(&self, size_in_bytes: usize) -> FreeListCategoryType;

  // ── 可覆写虚面 (默认 = FreeList 基类实现) ──

  /// 原版 h:146-153 注释 + cc:184-204 `virtual size_t Free(...)` 基类实现。
  /// 返回未入链的浪费字节 (块小于 min_block_size_ 时)。
  fn free<E: FreeListEnv>(
    &mut self,
    env: &mut E,
    isolate: *const Isolate,
    free_space: &WritableFreeSpace,
    mode: FreeMode,
  ) -> usize
  where
    Self: Sized,
  {
    let start = free_space.address(); // cc:186 `Address start = free_space.Address()`
    let size_in_bytes = free_space.size() as usize; // cc:187
    let page = env.page_from_address(isolate, start); // cc:188
    env.page_decrease_allocated_bytes(page, size_in_bytes); // cc:189

    // cc:191-195 块须够大才容得下链表簿记
    if size_in_bytes < self.base().min_block_size_ {
      env.page_add_wasted_memory(page, size_in_bytes); // cc:193
      return size_in_bytes; // cc:194
    }

    // cc:197-200 按量级选类目入链 (SelectFreeListCategoryType 虚分派 ✓)
    let type_ = self.select_free_list_category_type(size_in_bytes);
    let category = env.page_free_list_category(page, type_);
    let heap = env.isolate_heap(isolate); // cc:200 `isolate->heap()`
    // SAFETY: page 由 isolate 页表有效 (原版 FromAddress 契约); 类目槽为页元数据活实例。
    unsafe { &mut *category }.free(env, heap, free_space, mode, self); // cc:200
    // cc:201-202 DCHECK_EQ(page->AvailableInFreeList(),
    //   page->AvailableInFreeListFromAllocatedBytes()) — NormalPage 票面, 推迟台账 4
    0 // cc:203
  }

  /// 原版 h:167 + cc:456-464 `virtual void Reset()` 基类实现。
  fn reset(&mut self) {
    self.base_mut().reset_impl();
  }

  /// 原版 h:168 + cc:466-483 `virtual void ResetForNonBlackAllocatedPages()` 基类实现。
  fn reset_for_non_black_allocated_pages(&mut self, flags: &Flags) {
    self.base_mut().reset_for_non_black_impl(flags);
  }

  /// 原版 h:219 + cc:500-517 `virtual bool AddCategory(FreeListCategory*)` 基类实现。
  fn add_category(&mut self, category: &mut FreeListCategory) -> bool {
    self.base_mut().add_category_impl(category)
  }

  /// 原版 h:220 + cc:519-540 `virtual void RemoveCategory(FreeListCategory*)` 基类实现。
  fn remove_category(&mut self, category: &mut FreeListCategory) {
    self.base_mut().remove_category_impl(category)
  }

  // ── 非虚公有/保护面 (h/cc/inl 基类) ──

  /// 原版 cc:147-161 `TryFindNodeIn(type, minimum_size, node_size)` (h:251-255 注释:
  /// 首节点尝试, 空则摘类目) — RemoveCategory 为虚分派 (cc:158) → trait self。
  fn try_find_node_in<E: FreeListEnv>(
    &mut self,
    env: &mut E,
    type_: FreeListCategoryType,
    minimum_size: usize,
    node_size: &mut usize,
  ) -> Tagged<FreeSpace> {
    let category = self.base().top(type_); // cc:150
    if category.is_null() {
      return Tagged::default(); // cc:151
    }
    // SAFETY: categories_ 槽非空即页元数据活类目 (原版指针契约)。
    let node = unsafe { &mut *category }.pick_node_from_list(env, minimum_size, node_size); // cc:152
    if !node.is_null() {
      self.decrease_available_bytes(*node_size); // cc:153-154
      // cc:155 VerifyAvailable() — DEBUG DCHECK 目标配置剥除 (verify_available 单独可调)
    }
    if unsafe { &*category }.is_empty() {
      self.remove_category(unsafe { &mut *category }); // cc:157-158
    }
    node // cc:160
  }

  /// 原版 cc:163-182 `SearchForNodeInList(const Heap*, type, minimum_size, node_size)`
  /// (h:257-260 注释: 该 type 全链搜索) — 经 FreeListCategoryIterator (h:224-240)。
  fn search_for_node_in_list<E: FreeListEnv>(
    &mut self,
    env: &mut E,
    heap: *const Heap,
    type_: FreeListCategoryType,
    minimum_size: usize,
    node_size: &mut usize,
  ) -> Tagged<FreeSpace> {
    let mut it = FreeListCategoryIterator::new_base(self.base(), type_); // cc:167
    let mut node = Tagged::<FreeSpace>::default(); // cc:168
    while it.has_next() {
      let current = it.next_ptr(); // cc:170
      node = unsafe { &mut *current }.search_for_node_in_list(env, heap, minimum_size, node_size); // cc:171
      if !node.is_null() {
        self.decrease_available_bytes(*node_size); // cc:172-173
        // cc:174 VerifyAvailable() DEBUG 剥除
        if unsafe { &*current }.is_empty() {
          self.remove_category(unsafe { &mut *current }); // cc:175-176
        }
        return node; // cc:178
      }
    }
    node // cc:181
  }

  /// 原版 h:271 + inl:26-34 `GetPageForCategoryType(type)` — 类目顶反查所在页。
  fn get_page_for_category_type<E: FreeListEnv>(
    &self,
    env: &E,
    type_: FreeListCategoryType,
  ) -> *mut NormalPage {
    let category_top = self.base().top(type_); // inl:27
    if !category_top.is_null() {
      // SAFETY: 槽非空即活类目 (原版契约)。
      let top_node = unsafe { &*category_top }.top();
      debug_assert!(!top_node.is_null()); // inl:29 DCHECK(!category_top->top().is_null())
      env.page_from_heap_object(top_node) // inl:30 NormalPage::FromHeapObject
    } else {
      null_mut() // inl:32
    }
  }

  /// 原版 h:190 + inl:36-42 `bool IsEmpty()`。
  fn is_empty(&mut self) -> bool {
    self.base_mut().is_empty()
  }

  /// 原版 h:195 + cc:485-493 `EvictFreeListItems(NormalPage*)` —
  /// 页全部类目出链清账, 总量转页浪费内存 (RemoveCategory 虚分派 cc:489 → trait self)。
  fn evict_free_list_items<E: FreeListEnv>(&mut self, env: &mut E, page: *mut NormalPage)
  where
    Self: Sized,
  {
    let mut sum: usize = 0; // cc:486
    env.page_for_all_free_list_categories(page, |category| {
      // SAFETY: 页元数据数组槽活类目 (原版 lambda 形参即 FreeListCategory*)。
      let cat = unsafe { &mut *category };
      sum += cat.available() as usize; // cc:488
      self.remove_category(cat); // cc:489
      cat.reset(self); // cc:490
    });
    env.page_add_wasted_memory(page, sum); // cc:492
  }

  // 原版 h:193 + cc:495-498 `RepairLists(Heap*)` — 逐类目 RepairFreeList (h:192 注释:
  // boot 后修图)。RepairFreeList 实体 cc:112-124 依赖 ReadOnlyRoots/ObjectSlot (推迟台账 1),
  // 本 trait 不落地同名方法, 登记于文末台账。

  /// 原版 h:171-188 账目面转接。
  fn available(&self) -> usize {
    self.base().available()
  }
  /// h:177。
  fn increase_available_bytes(&mut self, bytes: usize) {
    self.base_mut().increase_available_bytes(bytes)
  }
  /// h:178。
  fn decrease_available_bytes(&mut self, bytes: usize) {
    self.base_mut().decrease_available_bytes(bytes)
  }
  /// h:180-182。
  fn wasted_bytes(&self) -> usize {
    self.base().wasted_bytes()
  }
  /// h:183-185。
  fn increase_wasted_bytes(&mut self, bytes: usize) {
    self.base_mut().increase_wasted_bytes(bytes)
  }
  /// h:186-188。
  fn decrease_wasted_bytes(&mut self, bytes: usize) {
    self.base_mut().decrease_wasted_bytes(bytes)
  }

  /// h:197。
  fn number_of_categories(&self) -> i32 {
    self.base().number_of_categories()
  }
  /// h:198。
  fn last_category(&self) -> FreeListCategoryType {
    self.base().last_category()
  }
  /// h:200。
  fn min_block_size(&self) -> usize {
    self.base().min_block_size()
  }
  /// h:202-210 (模板回调 type 限定版)。
  fn for_all_free_list_categories_of<F: FnMut(&mut FreeListCategory)>(
    &mut self,
    type_: FreeListCategoryType,
    callback: F,
  ) {
    self
      .base_mut()
      .for_all_free_list_categories_of(type_, callback)
  }
  /// h:212-217 (模板回调全量版)。
  fn for_all_free_list_categories<F: FnMut(&mut FreeListCategory)>(&mut self, callback: F) {
    self.base_mut().for_all_free_list_categories(callback)
  }
  /// h:221 + cc:542-551。
  fn print_categories(&mut self, type_: FreeListCategoryType) {
    self.base_mut().print_categories(type_)
  }
  /// h:247-249 + cc:578-599 族 (DEBUG 校验面, Rust 恒可调用)。
  fn verify_available<E: FreeListEnv>(&mut self, env: &mut E) {
    self.base_mut().verify_available(env)
  }
}

// ══════════════════════════════════════════════════════════════════════════
// 工厂 (原版 h:136-141 注释 + cc:139-145; std::unique_ptr → 具名值返回, 申报)
// ══════════════════════════════════════════════════════════════════════════

/// 原版 cc:139-141 `FreeList::CreateFreeList()` — 默认类目自由链表。
pub fn create_free_list() -> FreeListManyCachedOrigin {
  // cc:140 `return std::make_unique<FreeListManyCachedOrigin>()`
  FreeListManyCachedOrigin::new()
}

/// 原版 cc:143-145 `FreeList::CreateFreeListForNewSpace()` — 新空间自由链表
/// (h:139 注释 Creates a Freelist for new space)。
/// 申报: FastPathBase ctor 需 v8_flags (h:447-449), 原版读全局, Rust 参数注入。
pub fn create_free_list_for_new_space(flags: &Flags) -> FreeListManyCachedFastPathForNewSpace {
  // cc:144 `return std::make_unique<FreeListManyCachedFastPathForNewSpace>()`
  FreeListManyCachedFastPathForNewSpace::new(flags)
}

// ══════════════════════════════════════════════════════════════════════════
// FreeListMany (原版 h:292-350; cc:207-255)
// ══════════════════════════════════════════════════════════════════════════

/// 原版 h:310 `static constexpr size_t kMinBlockSize = 3 * kTaggedSize` (24)。
pub const K_MIN_BLOCK_SIZE: usize = 3 * K_TAGGED_SIZE;
/// 原版 h:312-314 `kMaxBlockSize = kRegularPageSize` (保守上界, 注释照录)。
pub const K_MAX_BLOCK_SIZE: usize = K_REGULAR_PAGE_SIZE;
/// 原版 h:315-317 `kPreciseCategoryMaxSize = 256` (常数时间可算类目的最大精确尺寸)。
pub const K_PRECISE_CATEGORY_MAX_SIZE: usize = 256;
/// 原版 h:319-327 类目边界 (perl 生成注释区照录于 skel) `kNumberOfCategories = 24`。
pub const K_NUMBER_OF_CATEGORIES: usize = 24;

/// 原版 h:328-330 `static constexpr unsigned int categories_min[kNumberOfCategories]`。
/// cc:209 类外 constexpr 重复定义无 Rust 对应 (台账注)。
#[rustfmt::skip]
pub const CATEGORIES_MIN: [i32; K_NUMBER_OF_CATEGORIES] = [
  24,  32,  48,  64,  80,  96,   112,  128,  144,  160,   176,   192,
  208, 224, 240, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536,
];

/// 原版 h:292-350 `class V8_EXPORT_PRIVATE FreeListMany : public FreeList`。
/// 24 类目 best-fit (首元素), h:292-297 类注释: 性能预期劣于 FreeListLegacy,
/// 内存占用更低 (碎片化更少)。
#[derive(Debug)]
pub struct FreeListMany {
  /// 基类 (h:273-283 字段; ctor cc:211-215)。
  pub(crate) base: FreeListBase,
}

impl Default for FreeListMany {
  fn default() -> Self {
    Self::new()
  }
}

impl FreeListMany {
  /// 原版 cc:211-215 `FreeListMany::FreeListMany(): FreeList(kNumberOfCategories, kMinBlockSize)` —
  /// h 注释 (cc:212): 初始化基类字段; categories_ 分配 (cc:213) + Reset() (cc:214)。
  /// dtor cc:217 `delete[] categories_` → Vec 随结构回收 (类目本身归页元数据, 不析构)。
  pub fn new() -> Self {
    let mut base = FreeListBase::new(K_NUMBER_OF_CATEGORIES as i32, K_MIN_BLOCK_SIZE); // cc:211
    base.alloc_categories(); // cc:213
    let mut many = Self { base };
    many.reset(); // cc:214
    many
  }
}

impl FreeList for FreeListMany {
  fn base(&self) -> &FreeListBase {
    &self.base
  }
  fn base_mut(&mut self) -> &mut FreeListBase {
    &mut self.base
  }

  /// 原版 h:332-346 `SelectFreeListCategoryType` (override) —
  /// <=256 常数式 `(size >> 4) - 1`, 其后线性找首个上界大于 size 的类目。
  fn select_free_list_category_type(&self, size_in_bytes: usize) -> FreeListCategoryType {
    if size_in_bytes <= K_PRECISE_CATEGORY_MAX_SIZE {
      // h:335-337
      if size_in_bytes < CATEGORIES_MIN[1] as usize {
        return 0; // h:336 `if (size_in_bytes < categories_min[1]) return 0`
      }
      return (size_in_bytes >> 4) as FreeListCategoryType - 1; // h:337
    }
    // h:339-344 线性尾搜
    let mut cat = (K_PRECISE_CATEGORY_MAX_SIZE >> 4) as FreeListCategoryType - 1;
    while cat < self.last_category() {
      if size_in_bytes < CATEGORIES_MIN[(cat + 1) as usize] as usize {
        return cat;
      }
      cat += 1;
    }
    self.last_category() // h:345
  }

  /// 原版 cc:219-231 `GetPageForSize(size_in_bytes)` (h:300 override) —
  /// 从最小可容类目 +1 起逐类目找页, 全无则回落最小类目页 (cc:226-229 注释:
  /// 可能返回装不下的页)。
  fn get_page_for_size<E: FreeListEnv>(
    &mut self,
    env: &mut E,
    size_in_bytes: usize,
  ) -> *mut NormalPage {
    let minimum_category = self.select_free_list_category_type(size_in_bytes); // cc:220-221
    let mut page = null_mut::<NormalPage>(); // cc:222
    let mut cat = minimum_category + 1;
    while page.is_null() && cat <= self.last_category() {
      // cc:223-225
      page = self.get_page_for_category_type(env, cat);
      cat += 1;
    }
    if page.is_null() {
      // cc:226-229 `Might return a page in which |size_in_bytes| will not fit`
      page = self.get_page_for_category_type(env, minimum_category);
    }
    page // cc:230
  }

  /// 原版 cc:233-255 `FreeListMany::Allocate(...)` (h:305-307 override) —
  /// [type, last) 逐类目首元素 best-fit; 末类目全链搜。
  fn allocate<E: FreeListEnv>(
    &mut self,
    env: &mut E,
    heap: *const Heap,
    size_in_bytes: usize,
    node_size: &mut usize,
    _origin: AllocationOrigin,
  ) -> Tagged<FreeSpace> {
    debug_assert!(K_MAX_BLOCK_SIZE >= size_in_bytes); // cc:236 DCHECK_GE(kMaxBlockSize, size)
    let mut node = Tagged::<FreeSpace>::default(); // cc:237
    let type_ = self.select_free_list_category_type(size_in_bytes); // cc:238
    let mut i = type_;
    while i < self.last_category() && node.is_null() {
      // cc:239 `for (int i = type; i < last_category_ && node.is_null(); i++)`
      node = self.try_find_node_in(env, i, size_in_bytes, node_size); // cc:240
      i += 1;
    }
    if node.is_null() {
      // cc:244-247 `Searching each element of the last category`
      node =
        self.search_for_node_in_list(env, heap, self.last_category(), size_in_bytes, node_size);
    }
    if !node.is_null() {
      // cc:249-251
      let page = env.page_from_heap_object(node);
      env.page_increase_allocated_bytes(page, *node_size);
    }
    self.verify_available(env); // cc:253 (DEBUG 面, release 空)
    node // cc:254
  }
}

// FRIEND_TEST(SpacesTest, FreeListManySelectFreeListCategoryType) (h:348) 与
// FRIEND_TEST(SpacesTest, FreeListManyGuaranteedAllocatable) (h:349) —
// 实测: 本仓 v8/test/unittests/heap/spaces-unittest.cc 仅落前者 (cc:485-534);
// 后者 FRIEND_TEST 引用的用例在本仓源中不存在 (grep 实测 0), Rust 以 pub 面承接 (推迟台账 12)。

// ══════════════════════════════════════════════════════════════════════════
// FreeListManyCached (原版 h:352-417; cc:257-370)
// ══════════════════════════════════════════════════════════════════════════

/// 原版 h:352-417 `class V8_EXPORT_PRIVATE FreeListManyCached : public FreeListMany` —
/// h:353-356 类注释: 用 next_nonempty_category 缓存标记空类目; best-fit 同 FreeListMany。
#[derive(Debug)]
pub struct FreeListManyCached {
  /// 继承链上一层。
  pub(crate) many: FreeListMany,
  /// 原版 h:403-406 注释: 缓存超配一格使末元素恒定义, 更新可用 cache[i+1]
  /// (i < kNumberOfCategories)。h:406 `int next_nonempty_category[kNumberOfCategories + 1]`。
  next_nonempty_category: [FreeListCategoryType; K_NUMBER_OF_CATEGORIES + 1],
}

impl Default for FreeListManyCached {
  fn default() -> Self {
    Self::new()
  }
}

impl FreeListManyCached {
  /// 原版 cc:260 `FreeListManyCached::FreeListManyCached() { ResetCache(); }`
  /// (隐式基类 FreeListMany ctor)。
  pub fn new() -> Self {
    let mut this = Self {
      many: FreeListMany::new(),
      next_nonempty_category: [0; K_NUMBER_OF_CATEGORIES + 1],
    };
    this.reset_cache(); // cc:260
    this
  }

  /// 原版 h:375-381 `UpdateCacheAfterAddition(cat)` — 加入类目后回写缓存 (h:375 注释)。
  pub(crate) fn update_cache_after_addition(&mut self, cat: FreeListCategoryType) {
    let mut i = cat;
    while i >= K_FIRST_CATEGORY && self.next_nonempty_category[i as usize] > cat {
      // h:377
      self.next_nonempty_category[i as usize] = cat; // h:379
      i -= 1;
    }
  }

  /// 原版 h:383-389 `UpdateCacheAfterRemoval(cat)` — 类目清空后回写缓存 (h:383 注释)。
  pub(crate) fn update_cache_after_removal(&mut self, cat: FreeListCategoryType) {
    let mut i = cat;
    while i >= K_FIRST_CATEGORY && self.next_nonempty_category[i as usize] == cat {
      // h:385
      self.next_nonempty_category[i as usize] = self.next_nonempty_category[(cat + 1) as usize]; // h:387
      i -= 1;
    }
  }

  /// 原版 h:409-416 `ResetCache()` (private) — 全槽置 kNumberOfCategories (含超配末格,
  /// h:413-414 注释; 循环体 h:411 + 末格 h:415 合并即全数组 fill)。
  pub(crate) fn reset_cache(&mut self) {
    self
      .next_nonempty_category
      .fill(K_NUMBER_OF_CATEGORIES as FreeListCategoryType);
  }

  /// 原版 h:391-401 `CheckCacheIntegrity()` (#ifdef DEBUG, h:391) —
  /// 缓存项指向非空类目或 last+1 哨兵; [i, cache[i]) 区间全空。
  pub(crate) fn check_cache_integrity(&self) {
    let last = self.many.base.last_category();
    for i in 0..=last {
      let cached = self.next_nonempty_category[i as usize];
      debug_assert!(
        cached == last + 1 || !self.many.base.top(cached).is_null(),
        "缓存槽须指非空类目或末哨兵 (原版 h:394-395 DCHECK)"
      );
      let mut j = i;
      while j < cached {
        debug_assert!(
          self.many.base.top(j).is_null(),
          "[i, cache[i]) 区间类目须全空 (原版 h:396-398 DCHECK)"
        );
        j += 1;
      }
    }
  }

  /// 缓存读出面 (h:406 数组; 子类 FastPath 系与本 crate allocate 实现消费)。
  #[inline]
  pub(crate) fn cache(&self, i: usize) -> FreeListCategoryType {
    self.next_nonempty_category[i]
  }

  /// 原版 cc:301-332 `FreeListManyCached::Free` 实体 (h:365-366 override) —
  /// 基类 cc:184-204 复制体 + 挂链时缓存回写 (cc:320-327)。
  fn free_impl<E: FreeListEnv>(
    &mut self,
    env: &mut E,
    isolate: *const Isolate,
    free_space: &WritableFreeSpace,
    mode: FreeMode,
  ) -> usize {
    let start = free_space.address(); // cc:304
    let size_in_bytes = free_space.size() as usize; // cc:305
    let page = env.page_from_address(isolate, start); // cc:306
    env.page_decrease_allocated_bytes(page, size_in_bytes); // cc:307

    if size_in_bytes < self.many.base.min_block_size_ {
      // cc:309-313
      env.page_add_wasted_memory(page, size_in_bytes); // cc:311
      return size_in_bytes; // cc:312
    }

    let type_ = self.select_free_list_category_type(size_in_bytes); // cc:315-317
    let category = env.page_free_list_category(page, type_);
    let heap = env.isolate_heap(isolate); // cc:318 `isolate->heap()`
    // SAFETY: 同 trait free 的 cc:200 处契约。
    unsafe { &mut *category }.free(env, heap, free_space, mode, self); // cc:318

    // cc:320-327 `Updating cache`
    if mode == FreeMode::LinkCategory {
      self.update_cache_after_addition(type_); // cc:322
      self.check_cache_integrity(); // cc:324-326 #ifdef DEBUG
    }

    // cc:329-330 DCHECK_EQ AvailableInFreeList 族 (推迟台账 4)
    0 // cc:331
  }

  /// 原版 cc:334-370 `FreeListManyCached::Allocate` (h:361-363 override) —
  /// 缓存跳跃版 best-fit。
  fn allocate_impl<E: FreeListEnv>(
    &mut self,
    env: &mut E,
    heap: *const Heap,
    size_in_bytes: usize,
    node_size: &mut usize,
    origin: AllocationOrigin,
  ) -> Tagged<FreeSpace> {
    let _ = origin; // cc:338 USE(origin)
    debug_assert!(K_MAX_BLOCK_SIZE >= size_in_bytes); // cc:339
    let mut node = Tagged::<FreeSpace>::default(); // cc:341
    let mut type_ = self.select_free_list_category_type(size_in_bytes); // cc:342
    type_ = self.next_nonempty_category[type_ as usize]; // cc:343
    while type_ < self.many.base.last_category() {
      // cc:344 `for (; type < last_category_; type = next_nonempty_category[type + 1])`
      node = self.try_find_node_in(env, type_, size_in_bytes, node_size); // cc:345
      if !node.is_null() {
        break; // cc:346
      }
      type_ = self.next_nonempty_category[(type_ + 1) as usize];
    }
    if node.is_null() {
      // cc:349-353
      type_ = self.many.base.last_category();
      node = self.search_for_node_in_list(env, heap, type_, size_in_bytes, node_size);
    }
    // cc:355-358 `Updating cache`
    if !node.is_null() && self.many.base.top(type_).is_null() {
      self.update_cache_after_removal(type_);
    }
    self.check_cache_integrity(); // cc:360-362 #ifdef DEBUG
    if !node.is_null() {
      // cc:364-366
      let page = env.page_from_heap_object(node);
      env.page_increase_allocated_bytes(page, *node_size);
    }
    self.verify_available(env); // cc:368
    node // cc:369
  }
}

impl FreeList for FreeListManyCached {
  fn base(&self) -> &FreeListBase {
    self.many.base()
  }
  fn base_mut(&mut self) -> &mut FreeListBase {
    self.many.base_mut()
  }

  // 非覆写虚槽逐层上转 (C++ 继承=同槽; 申报: Rust trait 无继承, 逐槽显式转接)。

  fn select_free_list_category_type(&self, size_in_bytes: usize) -> FreeListCategoryType {
    self.many.select_free_list_category_type(size_in_bytes)
  }
  fn get_page_for_size<E: FreeListEnv>(
    &mut self,
    env: &mut E,
    size_in_bytes: usize,
  ) -> *mut NormalPage {
    self.many.get_page_for_size(env, size_in_bytes)
  }

  /// 原版 cc:262-265 `Reset()` — ResetCache + FreeListMany::Reset() (h:368 override)。
  fn reset(&mut self) {
    self.reset_cache(); // cc:263
    FreeList::reset(&mut self.many); // cc:264 `FreeListMany::Reset()` → 无覆写 → 基类实现
  }

  /// 原版 cc:267-270 `ResetForNonBlackAllocatedPages()` (h:369 override)。
  fn reset_for_non_black_allocated_pages(&mut self, flags: &Flags) {
    self.reset_cache(); // cc:268
    FreeList::reset_for_non_black_allocated_pages(&mut self.many, flags); // cc:269
  }

  /// 原版 cc:272-285 `AddCategory` (h:371 override) — 基类挂链 + 缓存回写。
  fn add_category(&mut self, category: &mut FreeListCategory) -> bool {
    let was_added = FreeList::add_category(&mut self.many, category); // cc:273 `FreeList::AddCategory`
    if was_added {
      self.update_cache_after_addition(category.type_()); // cc:276-278 (h:277 读 type_)
    }
    self.check_cache_integrity(); // cc:280-282 #ifdef DEBUG
    was_added // cc:284
  }

  /// 原版 cc:287-299 `RemoveCategory` (h:372 override) — 基类摘链 + 类目全空时缓存回写。
  fn remove_category(&mut self, category: &mut FreeListCategory) {
    FreeList::remove_category(&mut self.many, category); // cc:288 `FreeList::RemoveCategory`
    let type_ = category.type_(); // cc:291 `int type = category->type_`
    if self.many.base.top(type_).is_null() {
      // cc:292-294
      self.update_cache_after_removal(type_);
    }
    self.check_cache_integrity(); // cc:296-298 #ifdef DEBUG
  }

  /// 原版 cc:301-332 `Free` (h:365-366 override)。
  fn free<E: FreeListEnv>(
    &mut self,
    env: &mut E,
    isolate: *const Isolate,
    free_space: &WritableFreeSpace,
    mode: FreeMode,
  ) -> usize {
    self.free_impl(env, isolate, free_space, mode)
  }

  /// 原版 cc:334-370 `Allocate` (h:361-363 override)。
  fn allocate<E: FreeListEnv>(
    &mut self,
    env: &mut E,
    heap: *const Heap,
    size_in_bytes: usize,
    node_size: &mut usize,
    origin: AllocationOrigin,
  ) -> Tagged<FreeSpace> {
    self.allocate_impl(env, heap, size_in_bytes, node_size, origin)
  }
}

// ══════════════════════════════════════════════════════════════════════════
// FreeListManyCachedFastPathBase (原版 h:419-491; cc:372-436)
// ══════════════════════════════════════════════════════════════════════════

/// 原版 h:441 `enum class SmallBlocksMode { kAllow, kProhibit }`。
/// 变体去 k 前缀 PascalCase 承接 (同 v8-common LanguageMode 先例), 值不变。
#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SmallBlocksMode {
  /// h:441 首项 kAllow (值 0)
  Allow = 0,
  /// h:441 次项 kProhibit (值 1)
  Prohibit = 1,
}

/// 原版 h:458-464 (protected constexpr)。h:458 注释: 第 18 类目内对象至少 2048 字节。
pub const K_FAST_PATH_FIRST_CATEGORY: FreeListCategoryType = 18;
/// 原版 h:460 `kFastPathStart = 2048`。
pub const K_FAST_PATH_START: usize = 2048;
/// 原版 h:461 `kTinyObjectMaxSize = 128`。
pub const K_TINY_OBJECT_MAX_SIZE: usize = 128;
/// 原版 h:462 `kFastPathOffset = kFastPathStart - kTinyObjectMaxSize` (1920 = 1.85k, h:420-431 类注释)。
pub const K_FAST_PATH_OFFSET: usize = K_FAST_PATH_START - K_TINY_OBJECT_MAX_SIZE;
/// 原版 h:463-464 注释: 第 15 类目内对象至少 256 字节。
pub const K_FAST_PATH_FALL_BACK_TINY: FreeListCategoryType = 15;

// 原版 h:466-468 static_assert ×2 (categories_min 表界对照) — Rust 编译期恒检。
const _: () =
  assert!(CATEGORIES_MIN[K_FAST_PATH_FIRST_CATEGORY as usize] == K_FAST_PATH_START as i32);
const _: () =
  assert!(CATEGORIES_MIN[K_FAST_PATH_FALL_BACK_TINY as usize] == K_TINY_OBJECT_MAX_SIZE as i32 * 2);

/// 原版 h:419-491 `class V8_EXPORT_PRIVATE FreeListManyCachedFastPathBase :
/// public FreeListManyCached` (h:419-437 类注释全译在 skel; 快路径超配 >=1.85k 字节)。
#[derive(Debug)]
pub struct FreeListManyCachedFastPathBase {
  /// 继承链上一层。
  pub(crate) cached: FreeListManyCached,
  /// 原版 h:485-486 `SmallBlocksMode small_blocks_mode_` (private)。
  small_blocks_mode_: SmallBlocksMode,
}

impl FreeListManyCachedFastPathBase {
  /// 原版 h:443-451 `explicit FreeListManyCachedFastPathBase(SmallBlocksMode)`。
  /// 申报: v8_flags 全局 (h:447-449) → flags 参数注入。
  pub fn new(small_blocks_mode: SmallBlocksMode, flags: &Flags) -> Self {
    let mut this = Self {
      cached: FreeListManyCached::new(), // 隐式基类 ctor
      small_blocks_mode_: small_blocks_mode,
    };
    // h:445-450 kProhibit 抬最小块: minor_ms 开且 min_lab_size>0 取其 KB 值, 否则 kFastPathStart
    if this.small_blocks_mode_ == SmallBlocksMode::Prohibit {
      this.cached.many.base.min_block_size_ =
        if flags.minor_ms() && flags.minor_ms_min_lab_size_kb() > 0 {
          flags.minor_ms_min_lab_size_kb() * KB
        } else {
          K_FAST_PATH_START
        };
    }
    this
  }

  /// 原版 h:470-483 `SelectFastAllocationFreeListCategoryType(size_in_bytes)` (protected)。
  pub fn select_fast_allocation_free_list_category_type(
    &self,
    size_in_bytes: usize,
  ) -> FreeListCategoryType {
    debug_assert!(size_in_bytes < K_MAX_BLOCK_SIZE); // h:472 DCHECK
    if size_in_bytes >= CATEGORIES_MIN[self.last_category() as usize] as usize {
      return self.last_category(); // h:474
    }
    let size = size_in_bytes + K_FAST_PATH_OFFSET; // h:476 `size_in_bytes += kFastPathOffset` (参数局部, 后续无再赋值)
    let mut cat = K_FAST_PATH_FIRST_CATEGORY;
    while cat < self.last_category() {
      // h:477-481
      if size <= CATEGORIES_MIN[cat as usize] as usize {
        return cat;
      }
      cat += 1;
    }
    self.last_category() // h:482
  }

  /// 原版 cc:375-436 `FreeListManyCachedFastPathBase::Allocate` (h:453-455 override)。
  fn allocate_impl<E: FreeListEnv>(
    &mut self,
    env: &mut E,
    heap: *const Heap,
    size_in_bytes: usize,
    node_size: &mut usize,
    origin: AllocationOrigin,
  ) -> Tagged<FreeSpace> {
    let _ = origin; // cc:378 USE(origin)
    debug_assert!(K_MAX_BLOCK_SIZE >= size_in_bytes); // cc:379
    let mut node = Tagged::<FreeSpace>::default(); // cc:380

    // cc:382-390 `Fast path part 1: searching the last categories`
    let mut first_category = self.select_fast_allocation_free_list_category_type(size_in_bytes); // cc:383-384
    let mut type_ = first_category; // cc:385
    type_ = self.cached.cache(type_ as usize); // cc:386 `type = next_nonempty_category[type]`
    while type_ <= self.cached.many.base.last_category() {
      node = self.try_find_node_in(env, type_, size_in_bytes, node_size); // cc:388
      if !node.is_null() {
        break; // cc:389
      }
      type_ = self.cached.cache((type_ + 1) as usize); // cc:387 递增位
    }

    // cc:392-406 `Fast path part 2: searching the medium categories for tiny objects`
    if self.small_blocks_mode_ == SmallBlocksMode::Allow {
      // cc:393
      if node.is_null() {
        // cc:394
        if size_in_bytes <= K_TINY_OBJECT_MAX_SIZE {
          // cc:395
          // W10HCR2: cc:396 DCHECK_EQ(kFastPathFirstCategory, first_category) 补回
          // (外部整改落 part2 共享型修复时该行一度丢失, 本席按转写基准复原)。
          // 原版 cc:397-402 复用共享变量 type 跨 part1/part2 阶段, 命中时尾部
          // categories_[type]/UpdateCacheAfterRemoval(type) (cc:425-426) 读到 part2 类目;
          // 先行独立局部 t 形致 type_ 滞留 part1 退出哨兵 24 → top(24) 越界, 金测试 =
          // tests/free_list_unittest.rs fast_path_allocate_tiny_fallback_part2_gold。
          debug_assert!(K_FAST_PATH_FIRST_CATEGORY == first_category); // cc:396 DCHECK_EQ
          type_ = self.cached.cache(K_FAST_PATH_FALL_BACK_TINY as usize); // cc:397
          while type_ < K_FAST_PATH_FIRST_CATEGORY {
            // cc:398-399
            node = self.try_find_node_in(env, type_, size_in_bytes, node_size); // cc:400
            if !node.is_null() {
              break; // cc:401
            }
            type_ = self.cached.cache((type_ + 1) as usize); // cc:397 递增位
          }
          first_category = K_FAST_PATH_FALL_BACK_TINY; // cc:403
        }
      }
    }

    // cc:408-413 `Searching the last category`
    if node.is_null() {
      type_ = self.cached.many.base.last_category(); // cc:411
      node = self.search_for_node_in_list(env, heap, type_, size_in_bytes, node_size); // cc:412
    }

    // cc:415-423 `Finally, search the most precise category`
    if node.is_null() {
      type_ = self.select_free_list_category_type(size_in_bytes); // cc:417
      type_ = self.cached.cache(type_ as usize); // cc:418
      while type_ < first_category {
        node = self.try_find_node_in(env, type_, size_in_bytes, node_size); // cc:420
        if !node.is_null() {
          break; // cc:421
        }
        type_ = self.cached.cache((type_ + 1) as usize); // cc:419 递增位
      }
    }

    if !node.is_null() {
      // cc:425-428
      if self.cached.many.base.top(type_).is_null() {
        self.cached.update_cache_after_removal(type_); // cc:426
      }
      let page = env.page_from_heap_object(node); // cc:427
      env.page_increase_allocated_bytes(page, *node_size);
    }

    self.cached.check_cache_integrity(); // cc:430-432 #ifdef DEBUG
    self.verify_available(env); // cc:434
    node // cc:435
  }
}

impl FreeList for FreeListManyCachedFastPathBase {
  fn base(&self) -> &FreeListBase {
    self.cached.base()
  }
  fn base_mut(&mut self) -> &mut FreeListBase {
    self.cached.base_mut()
  }

  // 逐槽转接 (继承面): 覆写链在 cached 层, 本层不再覆写。
  fn select_free_list_category_type(&self, size_in_bytes: usize) -> FreeListCategoryType {
    self.cached.select_free_list_category_type(size_in_bytes)
  }
  fn get_page_for_size<E: FreeListEnv>(
    &mut self,
    env: &mut E,
    size_in_bytes: usize,
  ) -> *mut NormalPage {
    self.cached.get_page_for_size(env, size_in_bytes)
  }
  fn reset(&mut self) {
    FreeList::reset(&mut self.cached)
  }
  fn reset_for_non_black_allocated_pages(&mut self, flags: &Flags) {
    FreeList::reset_for_non_black_allocated_pages(&mut self.cached, flags)
  }
  fn add_category(&mut self, category: &mut FreeListCategory) -> bool {
    FreeList::add_category(&mut self.cached, category)
  }
  fn remove_category(&mut self, category: &mut FreeListCategory) {
    FreeList::remove_category(&mut self.cached, category)
  }
  fn free<E: FreeListEnv>(
    &mut self,
    env: &mut E,
    isolate: *const Isolate,
    free_space: &WritableFreeSpace,
    mode: FreeMode,
  ) -> usize {
    FreeList::free(&mut self.cached, env, isolate, free_space, mode)
  }

  /// 原版 cc:375-436 `Allocate` (h:453-455 override)。
  fn allocate<E: FreeListEnv>(
    &mut self,
    env: &mut E,
    heap: *const Heap,
    size_in_bytes: usize,
    node_size: &mut usize,
    origin: AllocationOrigin,
  ) -> Tagged<FreeSpace> {
    self.allocate_impl(env, heap, size_in_bytes, node_size, origin)
  }
}

// FRIEND_TEST(SpacesTest, FreeListManyCachedFastPathSelectFastAllocationFreeListCategoryType)
// (h:488-490) → pub select 面承接 (实测该用例在 spaces-unittest.cc:539-583, 已直迁)。

// ══════════════════════════════════════════════════════════════════════════
// FreeListManyCachedFastPath / …ForNewSpace (原版 h:493-504)
// ══════════════════════════════════════════════════════════════════════════

/// 原版 h:493-497 `class FreeListManyCachedFastPath : public FreeListManyCachedFastPathBase` —
/// ctor h:495-496 SmallBlocksMode::Allow。
#[derive(Debug)]
pub struct FreeListManyCachedFastPath {
  /// 继承链上一层。
  pub(crate) fast_base: FreeListManyCachedFastPathBase,
}

impl Default for FreeListManyCachedFastPath {
  fn default() -> Self {
    Self::new()
  }
}

impl FreeListManyCachedFastPath {
  /// 原版 h:495-496 ctor (kAllow 路径不读 flags, h:445 分支前置短路; flags 仅 kProhibit 消费)。
  pub fn new() -> Self {
    Self {
      fast_base: FreeListManyCachedFastPathBase::new(
        SmallBlocksMode::Allow,
        // kAllow 分支不触及 flags (h:445-450 if kProhibit), 以默认 Flags 代入
        &Flags::default(),
      ),
    }
  }
}

/// 原版 h:499-504 `class FreeListManyCachedFastPathForNewSpace :
/// public FreeListManyCachedFastPathBase` — ctor h:502-503 SmallBlocksMode::Prohibit。
#[derive(Debug)]
pub struct FreeListManyCachedFastPathForNewSpace {
  /// 继承链上一层。
  pub(crate) fast_base: FreeListManyCachedFastPathBase,
}

impl FreeListManyCachedFastPathForNewSpace {
  /// 原版 h:502-503 ctor。申报: flags 参数注入 (kProhibit 分支读 minor_ms 系, h:447-449)。
  pub fn new(flags: &Flags) -> Self {
    Self {
      fast_base: FreeListManyCachedFastPathBase::new(SmallBlocksMode::Prohibit, flags),
    }
  }
}

/// 逐槽转接宏体重复 (两薄壳类同 C++ 空继承): 见 FreeList impl。
macro_rules! forward_free_list_impl {
  ($ty:ty, $field:ident) => {
    impl FreeList for $ty {
      fn base(&self) -> &FreeListBase {
        self.$field.base()
      }
      fn base_mut(&mut self) -> &mut FreeListBase {
        self.$field.base_mut()
      }
      fn select_free_list_category_type(&self, size_in_bytes: usize) -> FreeListCategoryType {
        self.$field.select_free_list_category_type(size_in_bytes)
      }
      fn get_page_for_size<E: FreeListEnv>(
        &mut self,
        env: &mut E,
        size_in_bytes: usize,
      ) -> *mut NormalPage {
        self.$field.get_page_for_size(env, size_in_bytes)
      }
      fn reset(&mut self) {
        self.$field.reset()
      }
      fn reset_for_non_black_allocated_pages(&mut self, flags: &Flags) {
        self.$field.reset_for_non_black_allocated_pages(flags)
      }
      fn add_category(&mut self, category: &mut FreeListCategory) -> bool {
        self.$field.add_category(category)
      }
      fn remove_category(&mut self, category: &mut FreeListCategory) {
        self.$field.remove_category(category)
      }
      fn free<E: FreeListEnv>(
        &mut self,
        env: &mut E,
        isolate: *const Isolate,
        free_space: &WritableFreeSpace,
        mode: FreeMode,
      ) -> usize {
        self.$field.free(env, isolate, free_space, mode)
      }
      fn allocate<E: FreeListEnv>(
        &mut self,
        env: &mut E,
        heap: *const Heap,
        size_in_bytes: usize,
        node_size: &mut usize,
        origin: AllocationOrigin,
      ) -> Tagged<FreeSpace> {
        self
          .$field
          .allocate(env, heap, size_in_bytes, node_size, origin)
      }
    }
  };
}

forward_free_list_impl!(FreeListManyCachedFastPath, fast_base);
forward_free_list_impl!(FreeListManyCachedFastPathForNewSpace, fast_base);

// ══════════════════════════════════════════════════════════════════════════
// FreeListManyCachedOrigin (原版 h:506-520; cc:438-451)
// ══════════════════════════════════════════════════════════════════════════

/// 原版 h:506-513 类注释: GC 中走 FreeListManyCached (并行、贵分配不敏感, 降碎片),
/// 其余走 FreeListManyCachedFastPath (代码/runtime 需快, 容一些碎片)。
/// h:514-520 `class V8_EXPORT_PRIVATE FreeListManyCachedOrigin :
/// public FreeListManyCachedFastPath`。
#[derive(Debug)]
pub struct FreeListManyCachedOrigin {
  /// 继承链上一层。
  pub(crate) fast_path: FreeListManyCachedFastPath,
}

impl Default for FreeListManyCachedOrigin {
  fn default() -> Self {
    Self::new()
  }
}

impl FreeListManyCachedOrigin {
  /// 隐式基类 ctor (cc:140 make_unique 消费面)。
  pub fn new() -> Self {
    Self {
      fast_path: FreeListManyCachedFastPath::new(),
    }
  }
}

impl FreeList for FreeListManyCachedOrigin {
  fn base(&self) -> &FreeListBase {
    self.fast_path.base()
  }
  fn base_mut(&mut self) -> &mut FreeListBase {
    self.fast_path.base_mut()
  }

  fn select_free_list_category_type(&self, size_in_bytes: usize) -> FreeListCategoryType {
    self.fast_path.select_free_list_category_type(size_in_bytes)
  }

  /// 原版 h:434-437 注释 (类注释续): 精确 GetPageForSize 继承自 FreeListMany
  /// (使快路径在 Scavenger 中不更快 — 有意为之, 供本类对 scavenger 保持精确)。
  fn get_page_for_size<E: FreeListEnv>(
    &mut self,
    env: &mut E,
    size_in_bytes: usize,
  ) -> *mut NormalPage {
    self.fast_path.get_page_for_size(env, size_in_bytes)
  }

  fn reset(&mut self) {
    self.fast_path.reset()
  }
  fn reset_for_non_black_allocated_pages(&mut self, flags: &Flags) {
    self.fast_path.reset_for_non_black_allocated_pages(flags)
  }
  fn add_category(&mut self, category: &mut FreeListCategory) -> bool {
    self.fast_path.add_category(category)
  }
  fn remove_category(&mut self, category: &mut FreeListCategory) {
    self.fast_path.remove_category(category)
  }
  fn free<E: FreeListEnv>(
    &mut self,
    env: &mut E,
    isolate: *const Isolate,
    free_space: &WritableFreeSpace,
    mode: FreeMode,
  ) -> usize {
    self.fast_path.free(env, isolate, free_space, mode)
  }

  /// 原版 cc:441-451 `FreeListManyCachedOrigin::Allocate` (h:517-519 override) —
  /// 按 origin 分派: kGC → ManyCached 精确策略; 否则 FastPath。
  fn allocate<E: FreeListEnv>(
    &mut self,
    env: &mut E,
    heap: *const Heap,
    size_in_bytes: usize,
    node_size: &mut usize,
    origin: AllocationOrigin,
  ) -> Tagged<FreeSpace> {
    if origin == AllocationOrigin::Gc {
      // cc:446 `return FreeListManyCached::Allocate(...)`
      FreeList::allocate(
        &mut self.fast_path.fast_base.cached,
        env,
        heap,
        size_in_bytes,
        node_size,
        origin,
      )
    } else {
      // cc:448-449 `return FreeListManyCachedFastPath::Allocate(...)`
      FreeList::allocate(
        &mut self.fast_path,
        env,
        heap,
        size_in_bytes,
        node_size,
        origin,
      )
    }
  }
}

// ══════════════════════════════════════════════════════════════════════════
// 推迟台账 (他票耦合面; 双行号 + 原因; 台账锚见 tool/src/heap/free-list.{h,cc}.js 落地臂)
// ══════════════════════════════════════════════════════════════════════════
// 1. FreeListCategory::RepairFreeList — 声明 h:58, 实体 cc:112-124; 调用面 cc:495-498
//    FreeList::RepairLists (声明 h:193)。原因: ReadOnlyRoots(heap).free_space_map() +
//    ObjectSlot::contains_map_value/store_map (heap/objects 票); 消费面即 boot 后修图,
//    本 trait 仅注释占位 (FreeList::repair_lists 注)。
// 2. PickNodeFromList/SearchForNodeInList 的 CanAllocateOnChunk DCHECK — cc:40-41/cc:58-59
//    (BasePage::FromHeapObject(Isolate::Current()/FromHeap(heap), node))。原因: MutablePage 票;
//    目标配置展开侧 DCHECK 剥除, Rust debug_assert 亦不激活。
// 3. SumFreeList 的 map 根 DCHECK — cc:557-562 (RootIndex::FreeSpaceMap + Isolate::FromHeap)。
//    原因: Isolate/roots 耦合; cc:558-559 "boot 期 map 可为空" 注释语义随 size 累加面保留。
// 4. Free/ManyCached::Free 的 DCHECK_EQ(page->AvailableInFreeList(),
//    AvailableInFreeListFromAllocatedBytes()) — cc:201-202/cc:329-330。原因: NormalPage 票
//    (normal-page.cc:62-66 / normal-page.h:78-83); Rust 注释占位。
// 5. 节点簿记实体 — Tagged<FreeSpace>::Size()/size(kRelaxedLoad)/next()/SetNext()
//    (cc:42/47/56-57/60/65/75/77-81/99/122 等) 与 map_slot (cc:116-120 域) —
//    objects/free-space{,-inl}.h 票; Rust 以 FreeListEnv node_* 四面承接 (窄接口非空实现)。
// 6. NormalPage 运行态 — FromAddress/FromHeapObject/Decrease|IncreaseAllocatedBytes/
//    add_wasted_memory/free_list_category/ForAllFreeListCategories (cc:188-193/200/250/306-311/
//    318/365/427/487-492) — heap/normal-page{-inl}.h 票; Rust FreeListEnv page_* 六面承接。
// 7. 可执行内存写通道 — cc:68-74 (MemoryChunk::Metadata()->is_executable() +
//    WritableJitPage::FreeRange): Metadata 族属 memory_chunk 推迟台账 1; v8_common::
//    WritableJitPage 已实体存在 (code_memory_access.rs:1605+), FreeRange 的 Result 收束与
//    注册表 CHECK 语义归实现方 (FreeListEnv::node_is_executable/free_range_executable)。
// 8. Isolate::Current/FromHeap + isolate->heap() — cc:41/58/200/318 (h:29 前向声明) —
//    Isolate 票; Rust opaque + FreeListEnv::isolate_heap 令牌透传。
// 9. heap 写屏障上下文 Heap — cc:52-91/93-110 形参 const Heap* (经 SetNext 入屏障) —
//    heap/heap.h 票; Rust opaque 令牌。
// 10. FreeList::RepairLists — 声明 h:193, 实体 cc:495-498 — 随推迟 1 (逐类目 RepairFreeList)。
// 11. semi-space 消费面 (h:34 SemiSpace 前向声明等 h:26-34 其余声明) — 本族签名零使用, 不落。
// 12. FRIEND_TEST ×3 (h:348-349/h:488-490) 与 friend class ×9 (h:122-125/h:285-289) 与
//     namespace heap tester 前向 (h:21-24) — Rust 无友元, pub 导出承接 (memory_chunk 先例 11);
//     GuaranteedAllocatable 用例 (h:349) 实测本仓 spaces-unittest.cc 不存在, 不自造。
// 13. 虚析构 h:144 `virtual ~FreeList() = default` / override dtor h:303 —
//     Rust 组合链随作用域回收, 无 vtable dtor 面。
// 14. cc:209 `constexpr unsigned int FreeListMany::categories_min[kNumberOfCategories]`
//     类外定义 (ODR 兼容) — Rust pub const 单定义处, 无重复声明面。
