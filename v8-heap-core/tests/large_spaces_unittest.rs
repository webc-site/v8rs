//! tests/large_spaces_unittest.rs — v8-heap-core/src/large_spaces.rs 金标准用例 (HCR1-F1, R5H4 波)。
//!
//! 原版测试勘定实测: 直接相关用例在 v8/test/unittests/heap/spaces-unittest.cc
//! (:900 TEST_F(SpacesTestWithSmallHeap, OldLargeObjectSpace) 等), 依赖真 Isolate/Heap
//! 运行态不可直迁; :150-197 的 Observer 步长节奏 (TestAllocationObserver, 经
//! :585/:602 TEST_F 消费) 属纯 AllocationCounter 编排 → 本档以 LargeObjectSpace::
//! AdvanceAndInvokeAllocationObservers (large-spaces.cc:74-88) 为宿主 1:1 复刻其计数
//! 算例 (0/1/2/3/19/20/1)。其余面按 HCR1 判「现行 mock 即可落」口径, 对 large_spaces.rs
//! 已落地函数体逐分支复刻, 逐断言注释指原版 file:line 算例, 禁零断言冒烟:
//!   add/remove_page 三账对偶 → cc:188-203; update_accounting 舍入差 → cc:242-254;
//!   shrink_page_to_object_size 三分支+尾 DCHECK → cc:205-240; contains/contains_slow →
//!   cc:256-272; free_dead_objects 存活重算+池旗标双支+摘页安全序 → cc:386-408;
//!   flip 调用序 → cc:378-384; available 饱和 → cc:371-376; set_capacity max → cc:410-412;
//!   对象迭代器跳 Filler → cc:36-48; tear_down 清空循环 → cc:62-72;
//!   advance_and_invoke 节奏 → cc:74-88; Code::RemovePage 顺序 → cc:428-431;
//!   F6 转发终抵 todo 体 → cc:98-102/417-422 转发落点。
//! 页实体/Heap/MemoryAllocator 运行面以 LargePageOps + LargeSpaceRunEnv 双 env trait
//! 替身承接 (src 模块头申报的注入桥), 序观测 = 调用日志逐位核对。
//!
//! 提交页大小定谳: v8_os_page_size=4 → cps=4096 (memory_allocator.cc:675-676 旗标支;
//! memory_allocator_unittest.rs:207-224 同法先例)。nextest 每例独立进程, 进程级静态无串扰。

use std::{
  collections::HashMap,
  ffi::c_void,
  ptr::{addr_of, null, null_mut},
};

use aok::{OK, Void};
use log::info;
use v8_common::{Address, AllocationHint, AllocationSpace};
use v8_flags::Flags;
use v8_heap_core::{
  allocation_observer::{AllocationObserver, ObserverRef},
  large_spaces::*,
  memory_allocator::{FreeMode, Heap, LargePage, MemoryAllocator},
  memory_chunk::{FROM_PAGE, MemoryChunk, TO_PAGE},
  spaces::Space,
};
use v8_objects_core::{heap_object::HeapObject, tagged::Tagged};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 进程级提交页大小定谳 (cc:674-680): 旗标 v8_os_page_size=4 → commit_page_size 4096,
/// 使 shrink/update_accounting 的 RoundUp 算例 (cc:217/246-247) 平台无关。
#[ctor::ctor(unsafe)]
fn _cps_init() {
  let mut flags = Flags::default();
  flags.set_v8_os_page_size(4).unwrap();
  MemoryAllocator::<NoReservation>::initialize_once_per_process(&flags);
}

// ══════════════════════════════════════════════════════════════════════════
// 常量与替身
// ══════════════════════════════════════════════════════════════════════════

/// cps = GetCommitPageSize() (cc:675-676 旗标支)。
const CPS: usize = 4096;
/// chunk 域: MemoryChunk::alignment() = 1 << K_PAGE_SIZE_BITS = 1<<18 (memory_chunk.rs:160)
/// 对齐 → MemoryChunk::from_address 掩码归位可用 (cc:267 消费)。
const C1: Address = 0x2000_0000;
const C2: Address = 0x2004_0000;
const C3: Address = 0x2008_0000;
const C4: Address = 0x200c_0000;
/// 页头占用 (chunk->Offset(object) 的偏移域, cc:216 消费; 取 512 使 RoundUp 分支可判)。
const HEAD: usize = 512;

fn page(tag: usize) -> *mut LargePage {
  tag as *mut LargePage
}

fn heap_fake() -> *mut Heap {
  0x9000 as *mut Heap
}

/// 堆对象位型: ptr = addr|1 (强引用 tag, tagged.rs:31-33); address() 清 tag。
fn obj(addr: Address) -> Tagged<HeapObject> {
  Tagged::new(addr | 1)
}

/// 页元数据替身 (large-page.h/base-page.h/mutable-page.h 实体他票 → LargePageOps 槽料)。
#[derive(Clone, Copy, Debug)]
struct PageMeta {
  /// base-page.h `size_` (cc:189/198/225 消费)。
  size: usize,
  /// base-page.h `area_start_` (cc:222/292)。
  area_start: Address,
  /// base-page.h `area_end_` (area_size = end - start, cc:156/220/239)。
  area_end: Address,
  /// mutable-page.h `Chunk()` (cc:175/208/268)。
  chunk: *mut MemoryChunk,
  /// large-page.h GetObject 的堆址 (未 tag)。
  object: Address,
  /// HeapObject::Size() 宿主桥料 (cc:183/210/401)。
  obj_size: usize,
  /// base-page.h `owner_` (cc:194/202/259)。
  owner: *const c_void,
  /// large-page.h next_page (list_node 链, inl:25/cc:393-403)。
  next: *mut LargePage,
  /// mutable-page.h is_executable (cc:212)。
  executable: bool,
  /// instance type 判料: IsFreeSpaceOrFiller (cc:45)。
  filler: bool,
}

impl PageMeta {
  /// 整页装载形: area = [chunk+HEAD, chunk+size)。
  fn new(chunk_base: Address, size: usize, obj_size: usize) -> Self {
    Self {
      size,
      area_start: chunk_base + HEAD,
      area_end: chunk_base + size,
      chunk: chunk_base as *mut MemoryChunk,
      object: chunk_base + HEAD,
      obj_size,
      owner: null(),
      next: null_mut(),
      executable: false,
      filler: false,
    }
  }
}

/// 调用日志 (观测面逐位核对: 序语义即原版语句序)。
#[derive(Debug, PartialEq, Eq)]
enum Call {
  /// large-page.h ClearOutOfLiveRangeSlots (cc:221)。
  ClearSlots(*mut LargePage, Address),
  /// MemoryAllocator::PartialFreeMemory (cc:227-229)。
  PartialFree(*mut LargePage, Address, usize, Address),
  /// base-page.h set_area_end (cc:234)。
  SetAreaEnd(*mut LargePage, Address),
  /// mutable-page.h set_owner (cc:194/202)。
  SetOwner(*mut LargePage, *const c_void),
  /// MemoryAllocator::Free (cc:69-70/399-400)。
  AllocatorFree(FreeMode, *mut LargePage),
  /// mutable-page.h SetFlagNonExecutable (cc:381/359)。
  SetFlag(*mut LargePage, usize),
  /// mutable-page.h ClearFlagNonExecutable (cc:382)。
  ClearFlag(*mut LargePage, usize),
  /// mutable-page.h ClearLiveness (cc:361)。
  ClearLiveness(*mut LargePage),
  /// Heap::CreateFillerObjectAt (cc:80)。
  Filler(Address, i32),
  /// Isolate::RemoveCodeMemoryChunk (cc:429)。
  RemoveCodeChunk(*mut LargePage),
  /// MemoryAllocator::ReleaseDelayedPages (cc:407)。
  ReleaseDelayed,
}

/// 双 env trait (LargePageOps 17 槽 + LargeSpaceRunEnv 8 槽) 单一宿主。
struct TestEnv {
  pages: HashMap<usize, PageMeta>,
  calls: Vec<Call>,
  /// cc:76 IsAllocationObserverActive 旗标。
  observer_active: bool,
  /// cc:388 incremental_marking()->IsMarking() 旗标。
  marking: bool,
}

impl TestEnv {
  fn new() -> Self {
    Self {
      pages: HashMap::new(),
      calls: Vec::new(),
      observer_active: true,
      marking: false,
    }
  }
  fn put(&mut self, p: *mut LargePage, m: PageMeta) {
    self.pages.insert(p as usize, m);
  }
  fn meta(&self, p: *mut LargePage) -> PageMeta {
    self.pages[&(p as usize)]
  }
  fn meta_mut(&mut self, p: *mut LargePage) -> &mut PageMeta {
    self.pages.get_mut(&(p as usize)).unwrap()
  }
  fn set_next(&mut self, p: *mut LargePage, next: *mut LargePage) {
    self.meta_mut(p).next = next;
  }
  fn by_object(&self, object: Tagged<HeapObject>) -> Option<PageMeta> {
    self
      .pages
      .values()
      .copied()
      .find(|m| m.object == object.address())
  }
  fn covering(&self, address: Address) -> Option<PageMeta> {
    self
      .pages
      .values()
      .copied()
      .find(|m| address >= m.area_start && address < m.area_end)
  }
  fn filler_calls(&self) -> usize {
    self
      .calls
      .iter()
      .filter(|c| matches!(*c, Call::Filler(..)))
      .count()
  }
}

impl LargePageOps for TestEnv {
  fn page_size(&self, page: *mut LargePage) -> usize {
    self.meta(page).size
  }
  fn chunk_of(&self, page: *mut LargePage) -> *mut MemoryChunk {
    self.meta(page).chunk
  }
  fn chunk_address(&self, chunk: *mut MemoryChunk) -> Address {
    chunk as usize
  }
  fn chunk_offset(&self, chunk: *mut MemoryChunk, addr: Address) -> usize {
    addr - chunk as usize
  }
  fn area_start(&self, page: *mut LargePage) -> Address {
    self.meta(page).area_start
  }
  fn area_size(&self, page: *mut LargePage) -> usize {
    self.meta(page).area_end - self.meta(page).area_start
  }
  fn set_area_end(&mut self, page: *mut LargePage, end: Address) {
    self.meta_mut(page).area_end = end;
    self.calls.push(Call::SetAreaEnd(page, end));
  }
  fn get_object(&self, page: *mut LargePage) -> Tagged<HeapObject> {
    obj(self.meta(page).object)
  }
  fn object_size(&self, object: Tagged<HeapObject>) -> usize {
    self.by_object(object).map_or(0, |m| m.obj_size)
  }
  fn is_free_space_or_filler(&self, object: Tagged<HeapObject>) -> bool {
    self.by_object(object).is_some_and(|m| m.filler)
  }
  fn next_page(&self, page: *mut LargePage) -> *mut LargePage {
    self.meta(page).next
  }
  fn set_page_owner(&mut self, page: *mut LargePage, owner: *const c_void) {
    self.meta_mut(page).owner = owner;
    self.calls.push(Call::SetOwner(page, owner));
  }
  fn clear_out_of_live_range_slots(&mut self, page: *mut LargePage, free_start: Address) {
    self.calls.push(Call::ClearSlots(page, free_start));
  }
  fn set_flag_non_executable(&mut self, page: *mut LargePage, flag: usize) {
    self.calls.push(Call::SetFlag(page, flag));
  }
  fn clear_flag_non_executable(&mut self, page: *mut LargePage, flag: usize) {
    self.calls.push(Call::ClearFlag(page, flag));
  }
  fn page_is_executable(&self, page: *mut LargePage) -> bool {
    self.meta(page).executable
  }
  fn clear_liveness(&mut self, page: *mut LargePage) {
    self.calls.push(Call::ClearLiveness(page));
  }
}

impl LargeSpaceRunEnv for TestEnv {
  fn is_allocation_observer_active(&self) -> bool {
    self.observer_active
  }
  fn create_filler_object_at(&mut self, soon_object: Address, size: i32) {
    self.calls.push(Call::Filler(soon_object, size));
  }
  fn allocator_free(&mut self, mode: FreeMode, page: *mut LargePage) {
    self.calls.push(Call::AllocatorFree(mode, page));
  }
  fn partial_free_memory(
    &mut self,
    page: *mut LargePage,
    start: Address,
    bytes_to_free: usize,
    new_area_end: Address,
  ) {
    // 实体侧效应建模 (memory_allocator.cc PartialFree): 释 OS 页 → 页缩 + area 收
    // (cc:238 DCHECK_EQ(used_committed_size, page->size()) 成立的前提)。
    let m = self.meta_mut(page);
    m.size -= bytes_to_free;
    m.area_end = new_area_end;
    self
      .calls
      .push(Call::PartialFree(page, start, bytes_to_free, new_area_end));
  }
  fn release_delayed_pages(&mut self) {
    self.calls.push(Call::ReleaseDelayed);
  }
  fn is_marking(&self) -> bool {
    self.marking
  }
  fn page_owner_of_object(&self, object: Tagged<HeapObject>) -> *const c_void {
    self.covering(object.address()).map_or(null(), |m| m.owner)
  }
  fn remove_code_memory_chunk(&mut self, page: *mut LargePage) {
    self.calls.push(Call::RemoveCodeChunk(page));
  }
}

// ══════════════════════════════════════════════════════════════════════════
// 构造与 Space 槽 (cc:53-60/166-171 + h:44-64)
// ══════════════════════════════════════════════════════════════════════════

/// cc:53-54 `Space(heap, id, nullptr)` 初态全账 + LO 覆写槽: Available=0 (cc:56-60)、
/// Size/SizeOfObjects 读原子账 (h:46-47)、CommittedPhysicalMemory=CommittedMemory
/// (cc:166-171)、PageCount/IsEmpty/first/last/begin==end (h:52-77)。
#[test]
fn large_space_ctor_and_space_slots_gold() -> Void {
  let mut space = LargeObjectSpace::new(heap_fake(), AllocationSpace::LoSpace);
  // base-space.h:32 + cc:53-54 ctor 透传。
  assert_eq!(AllocationSpace::LoSpace, space.base.identity());
  assert!(space.base.heap() == heap_fake()); // base-space.h:27-30
  // h:105 `FreeList* free_list()` (非 const 重载) + cc:53-54 nullptr: 无链空间恒 None。
  assert!(space.base.free_list().is_none());
  // h:109/110/111 三账 + page_count 初值 0 (cc:53-54 初始化列表)。
  assert_eq!(0, Space::size(&space));
  assert_eq!(0, space.size_of_objects());
  assert_eq!(0, space.committed_memory());
  assert_eq!(0, space.page_count()); // h:52
  // cc:56-60 LO Available 恒 0 (h:43 注释: 无法复用既有大对象内存)。
  assert_eq!(0, space.available());
  // cc:166-171 覆写 = CommittedMemory()。
  assert_eq!(space.committed_memory(), space.committed_physical_memory());
  // h:64 IsEmpty = first_page()==nullptr; h:69-74/86-94 空表哨兵。
  assert!(space.is_empty());
  assert!(space.first_page().is_null());
  assert!(space.first_page_const().is_null());
  assert!(Space::last_page(&space).is_null());
  // h:76-77 begin==end (LargePageIterator(nullptr) 对)。
  assert!(space.begin().equals(&space.end()));
  assert!(space.const_begin().equals(&space.const_end()));
  info!("large_space_ctor_and_space_slots_gold ok");
  OK
}

// ══════════════════════════════════════════════════════════════════════════
// AddPage/RemovePage 三账对偶 (cc:188-203)
// ══════════════════════════════════════════════════════════════════════════

/// cc:189-195 AddPage: size_/committed 加 page->size()、objects_size_ 加 object_size、
/// page_count+1、PushBack 尾插 (cc:193/list.h:30-39)、set_owner(this) (cc:194);
/// cc:197-203 RemovePage: 对偶减账且不减 objects_size_ (cc:196 注: 右修剪后惰性重算,
/// cc:404-405)、表内摘除、owner 置空 (cc:202)。
#[test]
fn large_page_add_remove_bookkeeping_gold() -> Void {
  let mut space = LargeObjectSpace::new(heap_fake(), AllocationSpace::LoSpace);
  let mut env = TestEnv::new();
  let (p1, p2) = (page(0x100), page(0x200));
  env.put(p1, PageMeta::new(C1, 2 * CPS, 7680)); // 8192 页
  env.put(p2, PageMeta::new(C2, CPS, 3000)); // 4096 页
  env.calls.clear();

  // cc:188-195 第一页入账。
  space.add_page(&mut env, p1, 7680);
  let owner = addr_of!(space) as *const c_void; // cc:194 `page->set_owner(this)`
  assert_eq!(2 * CPS, Space::size(&space)); // cc:189 size_ += page->size()
  assert_eq!(2 * CPS, space.committed_memory()); // cc:191 AccountCommitted
  assert_eq!(7680, space.size_of_objects()); // cc:192 objects_size_ += object_size
  assert_eq!(1, space.page_count()); // cc:190
  assert_eq!(p1, space.first_page()); // cc:193 PushBack 尾插 (h:69-71 front 位型直转)
  assert_eq!(owner, env.meta(p1).owner);
  assert_eq!(&[Call::SetOwner(p1, owner)], env.calls.as_slice());

  // 尾插第二页: back 换手, front 保持 (list.h:30-39 序)。
  space.add_page(&mut env, p2, 3000);
  assert_eq!(3 * CPS, Space::size(&space));
  assert_eq!(3 * CPS, space.committed_memory());
  assert_eq!(10680, space.size_of_objects()); // cc:192 两页对象和
  assert_eq!(2, space.page_count());
  assert_eq!(p1, space.first_page()); // h:69-71
  assert_eq!(p2, space.last_page() as *mut LargePage); // h:87 (trait 槽 MutablePage 位型回转)
  assert_eq!(2, space.base.memory_chunk_list.len()); // 表序 = 入列序

  // cc:197-203 RemovePage(首页): 三账回退两账 — objects_size_ 不动 (cc:196 注)。
  env.calls.clear();
  space.remove_page(&mut env, p1);
  assert_eq!(CPS, Space::size(&space));
  assert_eq!(CPS, space.committed_memory());
  assert_eq!(10680, space.size_of_objects()); // 不减 (惰性重算归 cc:406)
  assert_eq!(1, space.page_count());
  assert_eq!(p2, space.first_page()); // 表首换手
  assert_eq!(null(), env.meta(p1).owner); // cc:202 set_owner(nullptr)
  assert_eq!(&[Call::SetOwner(p1, null())], env.calls.as_slice());

  // 对偶收净: 全账回零 (与 add 逐步互逆)。
  space.remove_page(&mut env, p2);
  assert_eq!(0, Space::size(&space));
  assert_eq!(0, space.committed_memory());
  assert_eq!(10680, space.size_of_objects()); // 依旧不减 — RemovePage 域无该账
  assert_eq!(0, space.page_count());
  assert!(space.is_empty());
  assert!(space.begin().equals(&space.end()));
  info!("large_page_add_remove_bookkeeping_gold ok");
  OK
}

// ══════════════════════════════════════════════════════════════════════════
// UpdateAccountingAfterResizingObject (cc:242-254)
// ══════════════════════════════════════════════════════════════════════════

/// cc:246-247 `RoundUp(size, GetCommitPageSize())` 舍入差算例 (cps=4096):
/// (1000→5000): 8192-4096=4096 入 size_/committed_, 实体差 4000 入 objects_size_;
/// (5000→8193): 12288-8192=4096 续入, objects +3193;
/// (8194→9000): 双舍入同桶 12288 → 提交差 0, 仅 objects +806 (cc:248 注: 跨 commit 页
/// 才有 size_ 变动的对偶表述)。
#[test]
fn update_accounting_rounding_delta_gold() -> Void {
  let mut space = LargeObjectSpace::new(heap_fake(), AllocationSpace::LoSpace);

  // cc:251-253 第一笔: RoundUp(1000,4096)=4096, RoundUp(5000,4096)=8192 → 差 4096。
  space.update_accounting_after_resizing_object(1000, 5000);
  assert_eq!(CPS, Space::size(&space));
  assert_eq!(CPS, space.committed_memory());
  assert_eq!(4000, space.size_of_objects());

  // 第二笔: RoundUp(5000)=8192, RoundUp(8193)=12288 → 差 4096; objects +3193。
  space.update_accounting_after_resizing_object(5000, 8193);
  assert_eq!(2 * CPS, Space::size(&space));
  assert_eq!(2 * CPS, space.committed_memory());
  assert_eq!(7193, space.size_of_objects());

  // 第三笔: 同舍入桶 (12288, 12288) → 提交差 0 (cc:249 DCHECK 等号位), 仅实体 +806。
  space.update_accounting_after_resizing_object(8194, 9000);
  assert_eq!(2 * CPS, Space::size(&space));
  assert_eq!(2 * CPS, space.committed_memory());
  assert_eq!(7999, space.size_of_objects());
  info!("update_accounting_rounding_delta_gold ok");
  OK
}

/// cc:244 `DCHECK_GE(new_object_size, old_object_size)` — 缩容入账违例。
#[test]
#[should_panic(expected = "cc:244 DCHECK_GE")]
fn update_accounting_decreasing_panics() {
  let mut space = LargeObjectSpace::new(heap_fake(), AllocationSpace::LoSpace);
  space.update_accounting_after_resizing_object(5000, 4000);
}

// ══════════════════════════════════════════════════════════════════════════
// ShrinkPageToObjectSize 三分支 (cc:205-240)
// ══════════════════════════════════════════════════════════════════════════

/// 三分支算例 (cps=4096, 页头偏移 HEAD=512 → used = RoundUp(512+size, 4096)):
/// A 不缩支 (cc:219 条件假, size==area): area_size 已等对象尺寸 → 零调用零账变;
/// B partial_free 支 (cc:224-231): 8192 页 size=3000 → used=4096 < 8192,
///   PartialFreeMemory(page, chunk+4096, 4096, area_start+3000) + size_/committed 各减 4096
///   (cc:230-231 双账回退; objects_size_ 不动 — 惰性重算 cc:404-405 注);
/// C set_area_end 支 (cc:232-234): 4096 页 size=1500 → used=4096 == 页尺寸, 无 OS 页可释
///   → 仅收 area_end。
/// 尾 DCHECK cc:238-239 由替身效应回写保证 (used==页尺寸 / size==area_size)。
#[test]
fn shrink_page_to_object_size_three_branches_gold() -> Void {
  // ── A: 无缩支 ──
  let mut space_a = LargeObjectSpace::new(heap_fake(), AllocationSpace::LoSpace);
  let mut env_a = TestEnv::new();
  let pa = page(0x100);
  // area = 4096-512 = 3584 == 对象尺寸 (cc:219 判据假路径)。
  env_a.put(pa, PageMeta::new(C1, CPS, 3584));
  space_a.add_page(&mut env_a, pa, 3584);
  env_a.calls.clear();
  // cc:210-212 DEBUG 三面判据入料: object=GetObject / Size 等 / 不可执行。
  space_a.shrink_page_to_object_size(&mut env_a, pa, obj(C1 + HEAD), 3584);
  assert!(env_a.calls.is_empty()); // cc:219 判据假 → 全跳过
  assert_eq!(CPS, Space::size(&space_a)); // 账不变
  assert_eq!(CPS, space_a.committed_memory());
  assert_eq!(3584, space_a.size_of_objects());
  // cc:238-239 尾恒等: used = RoundUp(512+3584) = 4096 == 页尺寸; 3584 == area_size。
  assert_eq!(CPS, env_a.meta(pa).size);
  assert_eq!(3584, env_a.area_size(pa));

  // ── B: PartialFreeMemory 支 ──
  let mut space_b = LargeObjectSpace::new(heap_fake(), AllocationSpace::LoSpace);
  let mut env_b = TestEnv::new();
  let pb = page(0x100);
  let mb = PageMeta::new(C1, 2 * CPS, 3000); // 8192 页; 对象原地缩至 3000
  env_b.put(pb, mb);
  space_b.add_page(&mut env_b, pb, 2 * CPS - HEAD); // 原尺寸 7680 入账
  env_b.calls.clear();
  let area_start = C1 + HEAD; // cc:222 读出侧同料
  space_b.shrink_page_to_object_size(&mut env_b, pb, obj(area_start), 3000);
  // cc:215-217: used = RoundUp(512+3000, 4096) = 4096 < 页 8192 → cc:224-231 支。
  assert_eq!(
    &[
      Call::ClearSlots(pb, area_start + 3000), // cc:221 free_start = obj + size
      Call::PartialFree(pb, C1 + 4096, 4096, area_start + 3000), // cc:227-229
    ],
    env_b.calls.as_slice()
  );
  assert_eq!(CPS, Space::size(&space_b)); // cc:230 size_ -= 4096
  assert_eq!(CPS, space_b.committed_memory()); // cc:231 AccountUncommitted
  assert_eq!(2 * CPS - HEAD, space_b.size_of_objects()); // objects_size_ 惰性不动 (cc:404-405)
  assert_eq!(1, space_b.page_count());
  // cc:238-239 尾恒等 (替身效应回写后): 页缩至 4096, area 3000 == 对象尺寸。
  assert_eq!(CPS, env_b.meta(pb).size);
  assert_eq!(3000, env_b.area_size(pb));

  // ── C: set_area_end 支 ──
  let mut space_c = LargeObjectSpace::new(heap_fake(), AllocationSpace::LoSpace);
  let mut env_c = TestEnv::new();
  let pc = page(0x100);
  env_c.put(pc, PageMeta::new(C1, CPS, 1500)); // 4096 页 (used==页尺寸)
  space_c.add_page(&mut env_c, pc, CPS - HEAD); // 原 3584
  env_c.calls.clear();
  space_c.shrink_page_to_object_size(&mut env_c, pc, obj(C1 + HEAD), 1500);
  // cc:215-217: used = RoundUp(512+1500) = 4096 == 页尺寸 → cc:232-234 仅收 area。
  assert_eq!(
    &[
      Call::ClearSlots(pc, C1 + HEAD + 1500), // cc:221
      Call::SetAreaEnd(pc, C1 + HEAD + 1500), // cc:234 new_area_end
    ],
    env_c.calls.as_slice()
  );
  assert_eq!(CPS, Space::size(&space_c)); // 双账不动 (无 OS 页释放)
  assert_eq!(CPS, space_c.committed_memory());
  assert_eq!(3584, space_c.size_of_objects());
  assert_eq!(CPS, env_c.meta(pc).size);
  assert_eq!(1500, env_c.area_size(pc)); // cc:239 收敛
  info!("shrink_page_to_object_size_three_branches_gold ok");
  OK
}

// ══════════════════════════════════════════════════════════════════════════
// Contains/ContainsSlow (cc:256-272)
// ══════════════════════════════════════════════════════════════════════════

/// cc:257-263 Contains: owner 恒等判据 (O(1)); cc:266-272 ContainsSlow: FromAddress
/// 掩码归位 (memory_chunk.rs:202 alignment=1<<18) 后沿 next_page 链比 chunk。
#[test]
fn contains_and_contains_slow_gold() -> Void {
  let mut s1 = LargeObjectSpace::new(heap_fake(), AllocationSpace::LoSpace);
  let mut s2 = LargeObjectSpace::new(heap_fake(), AllocationSpace::CodeLoSpace);
  let mut env = TestEnv::new();
  let (p1, p2) = (page(0x100), page(0x200));
  env.put(p1, PageMeta::new(C1, CPS, 3000));
  env.put(p2, PageMeta::new(C2, CPS, 3000));
  s1.add_page(&mut env, p1, 3000); // owner = &s1 (cc:194)
  s2.add_page(&mut env, p2, 3000); // owner = &s2

  // cc:259 owned = (chunk->owner() == this): 命中/异空间/无覆盖三态。
  assert!(s1.contains(&env, obj(C1 + 600))); // s1 页区内对象
  assert!(!s1.contains(&env, obj(C2 + 600))); // owner==&s2 → false
  assert!(!s1.contains(&env, obj(0x3000_0100))); // 无页覆盖 → owner null
  // cc:261 SLOW_DCHECK: 目标配置无 ENABLE_SLOW_DCHECKS → checks.rs:34 空操作 (编译期剥除,
  // 不作运行断言, 申报见 large_spaces.rs Contains 槽 doc)。

  // cc:267 FromAddress: C1+100 → 掩码归位 C1 == chunk_of(p1) (1<<18 界内任意址)。
  assert!(s1.contains_slow(&env, C1 + 100));
  assert!(s1.contains_slow(&env, C1)); // 界首自身
  assert!(!s1.contains_slow(&env, C2 + 100)); // 归位 C2 — s1 链上无 (链尽 null → false)

  // 摘页后链空: chunk 仍合法但不在表 → ContainsSlow false (cc:266 迭代面)。
  s1.remove_page(&mut env, p1);
  assert!(!s1.contains_slow(&env, C1 + 100));
  assert!(!s1.contains(&env, obj(C1 + 600))); // owner 已置 null (cc:202)
  info!("contains_and_contains_slow_gold ok");
  OK
}

// ══════════════════════════════════════════════════════════════════════════
// Flip (cc:378-384) + Available 饱和 (cc:371-376) + SetCapacity (cc:410-412)
// ══════════════════════════════════════════════════════════════════════════

/// cc:378-384: 链上逐页 SetFlagNonExecutable(FROM_PAGE) → ClearFlagNonExecutable(TO_PAGE)
/// (memory_chunk.rs:77/79 常量 1<<3/1<<4), 调用序逐位核对; h:161/331 NewLoSpace id 锚。
#[test]
fn flip_call_order_gold() -> Void {
  let mut nlo = NewLargeObjectSpace::new(heap_fake(), 4096);
  // cc:330-331 ctor: LargeObjectSpace(heap, NEW_LO_SPACE)。
  assert_eq!(AllocationSpace::NewLoSpace, nlo.space.base.identity());
  let mut env = TestEnv::new();
  let (p1, p2, p3) = (page(0x100), page(0x200), page(0x300));
  env.put(p1, PageMeta::new(C1, CPS, 10));
  env.put(p2, PageMeta::new(C2, CPS, 10));
  env.put(p3, PageMeta::new(C3, CPS, 10));
  env.set_next(p1, p2);
  env.set_next(p2, p3);
  nlo.add_page(&mut env, p1, 10); // cc:381 前入账 (Deref 继承面)
  nlo.add_page(&mut env, p2, 10);
  nlo.add_page(&mut env, p3, 10);
  env.calls.clear();

  nlo.flip(&mut env);
  // cc:379-383 序: 每页 Set(FROM) 先于 Clear(TO), 链尽止。
  assert_eq!(
    &[
      Call::SetFlag(p1, FROM_PAGE),
      Call::ClearFlag(p1, TO_PAGE),
      Call::SetFlag(p2, FROM_PAGE),
      Call::ClearFlag(p2, TO_PAGE),
      Call::SetFlag(p3, FROM_PAGE),
      Call::ClearFlag(p3, TO_PAGE),
    ],
    env.calls.as_slice()
  );
  info!("flip_call_order_gold ok");
  OK
}

/// cc:371-376 Available 三目饱和减 (h:166 注: 右修剪可致 SizeOfObjects > capacity_ → 0)
/// + cc:410-412 SetCapacity = max(capacity, SizeOfObjects) (h:171)。
#[test]
fn available_saturating_and_set_capacity_max_gold() -> Void {
  let mut nlo = NewLargeObjectSpace::new(heap_fake(), 1000);
  // cc:375 capacity_ - 0 = 1000; 基类覆写对照: LO Available 恒 0 (cc:56-60)。
  assert_eq!(1000, nlo.available());
  assert_eq!(0, Space::available(&nlo.space));

  // 右修剪超额场景 (cc:372-373 注释): objects 1500 > capacity 1000 → 饱和 0。
  nlo.set_objects_size(1500); // h:98
  assert_eq!(0, nlo.available());
  // cc:412 std::max: 下调请求被现存量顶住。
  nlo.set_capacity(1200);
  assert_eq!(1500, nlo.capacity_); // h:176 字段直读
  assert_eq!(0, nlo.available());
  // 上调生效并恢复差额。
  nlo.set_capacity(9000);
  assert_eq!(9000, nlo.capacity_);
  assert_eq!(7500, nlo.available()); // cc:375 9000 - 1500
  info!("available_saturating_and_set_capacity_max_gold ok");
  OK
}

// ══════════════════════════════════════════════════════════════════════════
// FreeDeadObjects (cc:386-408)
// ══════════════════════════════════════════════════════════════════════════

/// cc:393-403 摘页安全序: next 先行取再判死 (it++ 语义承接, 摘除不破坏迭代);
/// 死页 RemovePage(cc:398, 内含 set_owner(null) cc:202) → Free(free_mode)(cc:399);
/// cc:406 存活尺寸和惰性重算 objects_size_ (cc:404-405 注); cc:407 ReleaseDelayedPages;
/// cc:390-392 池旗标双支: large_page_pool=false → kImmediately / true → kDelayThenPool。
#[test]
fn free_dead_objects_survivor_recount_gold() -> Void {
  let mut nlo = NewLargeObjectSpace::new(heap_fake(), 0);
  let mut env = TestEnv::new();
  let (p1, p2, p3) = (page(0x100), page(0x200), page(0x300));
  env.put(p1, PageMeta::new(C1, CPS, 3000)); // 死
  env.put(p2, PageMeta::new(C2, CPS, 1000)); // 活
  env.put(p3, PageMeta::new(C3, CPS, 2000)); // 死
  env.set_next(p1, p2);
  env.set_next(p2, p3);
  nlo.add_page(&mut env, p1, 3000);
  nlo.add_page(&mut env, p2, 1000);
  nlo.add_page(&mut env, p3, 2000);
  assert_eq!(6000, nlo.size_of_objects());
  assert_eq!(3 * CPS, Space::size(&nlo.space));
  env.calls.clear();

  let dead = |o: Tagged<HeapObject>| o.address() == C1 + HEAD || o.address() == C3 + HEAD;
  // cc:390-392 池旗标双支 (本仓实态: large_page_pool 缺省 true, v8-flags defs.rs:4614
  // 实测) → 缺省 Flags 走 kDelayThenPool 支。
  let flags = Flags::default();
  nlo.free_dead_objects(&mut env, &flags, &mut |o| dead(o));

  // 序: 死页 p1 → [SetOwner(null), Free(旗标支模式)] → 活页 p2 零调用 → 死页 p3 →
  // 收尾 ReleaseDelayedPages (cc:407 无条件)。
  assert_eq!(
    &[
      Call::SetOwner(p1, null()),
      Call::AllocatorFree(FreeMode::DelayThenPool, p1), // cc:391 kDelayThenPool
      Call::SetOwner(p3, null()),
      Call::AllocatorFree(FreeMode::DelayThenPool, p3),
      Call::ReleaseDelayed,
    ],
    env.calls.as_slice()
  );
  // cc:398 摘页账对偶: 死页尺寸/提交/计数回退; 存活重算 objects_ = 1000 (cc:406)。
  assert_eq!(CPS, Space::size(&nlo.space));
  assert_eq!(CPS, nlo.committed_memory());
  assert_eq!(1, nlo.page_count());
  assert_eq!(1000, nlo.size_of_objects()); // 6000 → 1000 惰性重算
  assert_eq!(p2, nlo.first_page()); // 表内仅剩活页
  assert!(env.meta(p1).owner.is_null() && env.meta(p3).owner.is_null());

  // 池旗标假支 (cc:392): large_page_pool=false → kImmediately。
  let mut flags2 = Flags::default();
  flags2.set_large_page_pool(false).unwrap();
  let mut nlo2 = NewLargeObjectSpace::new(heap_fake(), 0);
  let mut env2 = TestEnv::new();
  env2.put(p1, PageMeta::new(C1, CPS, 3000));
  env2.set_next(p1, null_mut());
  nlo2.add_page(&mut env2, p1, 3000);
  env2.calls.clear();
  nlo2.free_dead_objects(&mut env2, &flags2, &mut |o| dead(o));
  assert_eq!(
    &[
      Call::SetOwner(p1, null()),
      Call::AllocatorFree(FreeMode::Immediately, p1), // cc:392 kImmediately
      Call::ReleaseDelayed,
    ],
    env2.calls.as_slice()
  );
  assert_eq!(0, nlo2.size_of_objects()); // 全死 → 重算 0 (cc:406)
  assert!(nlo2.is_empty());
  info!("free_dead_objects_survivor_recount_gold ok");
  OK
}

/// cc:388 `DCHECK(!heap()->incremental_marking()->IsMarking())` — GC 标记期清理违例
/// (判据经 LargeSpaceRunEnv::is_marking 桥)。
#[test]
#[should_panic(expected = "cc:388 DCHECK(!IsMarking())")]
fn free_dead_objects_while_marking_panics() {
  let mut nlo = NewLargeObjectSpace::new(heap_fake(), 0);
  let mut env = TestEnv::new();
  env.marking = true;
  let flags = Flags::default();
  nlo.free_dead_objects(&mut env, &flags, &mut |_| false);
}

// ══════════════════════════════════════════════════════════════════════════
// TearDown (cc:62-72) + 对象迭代器 (cc:36-48)
// ══════════════════════════════════════════════════════════════════════════

/// cc:62-72 清空循环: list 非空 → 取 first (front) → 表内摘除 → Free(kImmediately)
/// (cc:69-70 桥); 与 RemovePage 的对偶差异: TearDown 不减 size_/committed/page_count_
/// (页账释还在 allocator Free 实体侧, 台账 6/7), objects_ 同不动。
#[test]
fn tear_down_removes_without_rebookkeeping_gold() -> Void {
  let mut space = LargeObjectSpace::new(heap_fake(), AllocationSpace::LoSpace);
  let mut env = TestEnv::new();
  let (p1, p2) = (page(0x100), page(0x200));
  env.put(p1, PageMeta::new(C1, CPS, 3000));
  env.put(p2, PageMeta::new(C2, 2 * CPS, 6000));
  space.add_page(&mut env, p1, 3000);
  space.add_page(&mut env, p2, 6000);
  env.calls.clear();

  space.tear_down(&mut env);
  // 序: front 先释 (p1 → p2, cc:64 first_page + h:80 Empty 循环界)。
  assert_eq!(
    &[
      Call::AllocatorFree(FreeMode::Immediately, p1), // cc:69-70 kImmediately
      Call::AllocatorFree(FreeMode::Immediately, p2),
    ],
    env.calls.as_slice()
  );
  // 表清空 (list.remove 每轮) + owner 不清 (TearDown 域无 set_owner 语句, 对照 cc:202)。
  assert!(space.is_empty());
  assert!(space.base.memory_chunk_list.is_empty());
  assert!(!env.meta(p1).owner.is_null());
  // 三账不逆 (RemovePage 对偶差异本例即金断言)。
  assert_eq!(3 * CPS, Space::size(&space));
  assert_eq!(3 * CPS, space.committed_memory());
  assert_eq!(9000, space.size_of_objects());
  assert_eq!(2, space.page_count());
  info!("tear_down_removes_without_rebookkeeping_gold ok");
  OK
}

/// cc:36-39 ctor current = space->first_page(); cc:41-48 Next: 逐页取 GetObject 并先行
/// 推进 (cc:43-44), 跳 FreeSpace/Filler (cc:45), 耗尽回复 Tagged::NULL (cc:47)。
#[test]
fn object_iterator_skips_filler_gold() -> Void {
  use v8_heap_core::spaces::ObjectIterator;

  let mut space = LargeObjectSpace::new(heap_fake(), AllocationSpace::LoSpace);
  let mut env = TestEnv::new();
  let (p1, p2, p3, p4) = (page(0x100), page(0x200), page(0x300), page(0x400));
  let mut m1 = PageMeta::new(C1, CPS, 10);
  m1.filler = true; // cc:45 IsFreeSpaceOrFiller 命中料
  let mut m3 = PageMeta::new(C3, CPS, 10);
  m3.filler = true;
  env.put(p1, m1);
  env.put(p2, PageMeta::new(C2, CPS, 20));
  env.put(p3, m3);
  env.put(p4, PageMeta::new(C4, CPS, 30));
  env.set_next(p1, p2);
  env.set_next(p2, p3);
  env.set_next(p3, p4);
  space.add_page(&mut env, p1, 10); // 入列仅为 first_page 起点 (cc:38)
  space.add_page(&mut env, p2, 20);
  space.add_page(&mut env, p3, 10);
  space.add_page(&mut env, p4, 30);

  let mut it = space.get_object_iterator(heap_fake(), &env);
  assert_eq!(obj(C2 + HEAD), it.next()); // p1 filler 跳 (cc:44-45), p2 命中
  assert_eq!(obj(C4 + HEAD), it.next()); // p3 filler 跳, p4 命中
  assert!(it.next().is_null()); // cc:47 耗尽 null Tagged
  assert!(it.next().is_null()); // 耗尽态幂等 (current_ 已 null, while 不入)
  info!("object_iterator_skips_filler_gold ok");
  OK
}

// ══════════════════════════════════════════════════════════════════════════
// AdvanceAndInvokeAllocationObservers 节奏 (cc:74-88)
// ══════════════════════════════════════════════════════════════════════════

/// spaces-unittest.cc:150-156 Observer 替身 (128 步长计数型)。
struct TestObserver {
  step_size: isize,
  count: usize,
}

impl AllocationObserver for TestObserver {
  /// spaces-unittest.cc:154 `Step(...) { count_++; }`。
  fn step(&mut self, _bytes_allocated: i32, _soon_object: Address, _size: usize) {
    self.count += 1;
  }
  fn step_size(&self) -> isize {
    self.step_size
  }
}

/// 复刻 spaces-unittest.cc:160-197 (TestAllocationObserver 经 :585/:602 TEST_F 消费) 的
/// step-128 节奏算例, 宿主换 LargeObjectSpace::AdvanceAndInvoke (cc:74-88):
/// 64→0; +64 (恰满 128)→1; +136→2; +1024 (大对象单次)→3; 64×32 共 2048→19
/// (cc:186-190 期望 15+4=19 同数: 每满 128 触发 1 次); 加 96 步长第二观察者后 +2048
/// → 20/1 (cc:193-197)。cc:80 CreateFillerObjectAt 与 invoke 一一对应 (filler 计数=20)。
#[test]
fn advance_and_invoke_observer_cadence_gold() -> Void {
  let mut space = LargeObjectSpace::new(heap_fake(), AllocationSpace::LoSpace);
  let mut env = TestEnv::new();
  let soon = C1 + HEAD; // cc:88 DCHECK(soon_object) 非零位型

  let mut ob1 = TestObserver {
    step_size: 128,
    count: 0,
  };
  // cc:90-92 AddAllocationObserver → counter 排程 next=128。
  space.add_allocation_observer(ObserverRef::new(&mut ob1));
  assert_eq!(128, space.allocation_counter.next_bytes()); // h:75-78

  // spaces-unittest.cc:169-171: 64 < 128 → 不通知 (cc:78 判据假, 仅 cc:87 Advance)。
  space.advance_and_invoke_allocation_observers(&mut env, soon, 64);
  assert_eq!(0, ob1.count);
  // :174-176: 累计恰满 128 → 通知 1 次 (cc:78 `>=` 等号位)。
  space.advance_and_invoke_allocation_observers(&mut env, soon, 64);
  assert_eq!(1, ob1.count);
  // :179-180: 再 128+ → 第 2 次。
  space.advance_and_invoke_allocation_observers(&mut env, soon, 136);
  assert_eq!(2, ob1.count);
  // :183-184: 大对象 1024 仅 1 次通知 (单次 invoke)。
  space.advance_and_invoke_allocation_observers(&mut env, soon, 1024);
  assert_eq!(3, ob1.count);
  // :187-191: 2048 字节小对象流 = 64×32 → +16 次 (每 128 触发 1) → 19。
  for _ in 0..64 {
    space.advance_and_invoke_allocation_observers(&mut env, soon, 32);
  }
  assert_eq!(19, ob1.count);
  // cc:79-80: filler 与 invoke 分支一一对应 (advance-only 调用不产 filler)。
  assert_eq!(19, env.filler_calls());

  // :193-197: 第二观察者 (96 步长) 后 2048 → ob1=20 / ob2=1。
  let mut ob2 = TestObserver {
    step_size: 96,
    count: 0,
  };
  space.add_allocation_observer(ObserverRef::new(&mut ob2));
  space.advance_and_invoke_allocation_observers(&mut env, soon, 2048);
  assert_eq!(20, ob1.count);
  assert_eq!(1, ob2.count);
  assert_eq!(20, env.filler_calls());
  // cc:157-158: next = cur + min(剩余步长) = 观察者 96 域。
  assert_eq!(96, space.allocation_counter.next_bytes());

  // cc:94-96 RemoveAllocationObserver: 摘除后重算 (空集 SIZE_MAX, h:75-78)。
  space.remove_allocation_observer(ObserverRef::new(&mut ob1));
  space.remove_allocation_observer(ObserverRef::new(&mut ob2));
  assert_eq!(usize::MAX, space.allocation_counter.next_bytes());
  info!("advance_and_invoke_observer_cadence_gold ok");
  OK
}

/// cc:76 `if (!heap()->IsAllocationObserverActive()) return;` — 早退且 **不推进**
/// counter (Advance 在该判据之后, 与 LAB 侧 PauseAllocationObserversScope 语义同源):
/// next_bytes 冻结、零 filler、零 Step。
#[test]
fn advance_and_invoke_inactive_gate_gold() -> Void {
  let mut space = LargeObjectSpace::new(heap_fake(), AllocationSpace::LoSpace);
  let mut env = TestEnv::new();
  env.observer_active = false; // cc:76 桥旗标
  let mut ob = TestObserver {
    step_size: 128,
    count: 0,
  };
  space.add_allocation_observer(ObserverRef::new(&mut ob));
  assert_eq!(128, space.allocation_counter.next_bytes());

  space.advance_and_invoke_allocation_observers(&mut env, C1 + HEAD, 64);
  assert_eq!(0, ob.count); // 未回调
  assert_eq!(128, space.allocation_counter.next_bytes()); // 未推进 (cc:76 早退含 cc:87)
  assert!(env.calls.is_empty()); // 零 filler
  info!("advance_and_invoke_inactive_gate_gold ok");
  OK
}

// ══════════════════════════════════════════════════════════════════════════
// Code 覆写槽 (cc:424-431) + 六支 id (cc:324-328/414-415/433-440) + F6 转发 (cc:98-102/417-422)
// ══════════════════════════════════════════════════════════════════════════

/// cc:428-431 CodeLargeObjectSpace::RemovePage: cc:429 `heap()->isolate()->
/// RemoveCodeMemoryChunk(page)` **先于** 基类摘账 (顺序照录); cc:424-426 AddPage 纯转发。
#[test]
fn code_space_remove_page_order_gold() -> Void {
  let mut code = CodeLargeObjectSpace::new(heap_fake());
  assert_eq!(AllocationSpace::CodeLoSpace, code.old.space.base.identity()); // cc:414-415
  let mut env = TestEnv::new();
  let p1 = page(0x100);
  env.put(p1, PageMeta::new(C1, CPS, 3000));
  // cc:425 转发基类入账。
  code.add_page(&mut env, p1, 3000);
  assert_eq!(CPS, code.size());
  assert_eq!(1, code.page_count());
  env.calls.clear();

  code.remove_page(&mut env, p1);
  // cc:429 → cc:430: RemoveCodeChunk 先于基类 SetOwner(null) (序违例即回归)。
  assert_eq!(
    &[Call::RemoveCodeChunk(p1), Call::SetOwner(p1, null())],
    env.calls.as_slice()
  );
  assert_eq!(0, code.size());
  assert_eq!(0, code.committed_memory());
  assert_eq!(0, code.page_count());
  assert!(code.is_empty());
  info!("code_space_remove_page_order_gold ok");
  OK
}

/// 六支 wrapper ctor 的 id 落点 (cc:324-328/414-415/433-440; large_spaces.rs 编译期
/// 枚举序锚同源) — 值面逐支核对。
#[test]
fn six_wrapper_ctors_ids_gold() -> Void {
  assert_eq!(
    AllocationSpace::LoSpace,
    OldLargeObjectSpace::new(heap_fake()).space.base.identity()
  ); // cc:324-325
  assert_eq!(
    AllocationSpace::CodeLoSpace,
    CodeLargeObjectSpace::new(heap_fake())
      .old
      .space
      .base
      .identity()
  ); // cc:414-415
  assert_eq!(
    AllocationSpace::SharedLoSpace,
    SharedLargeObjectSpace::new(heap_fake())
      .old
      .space
      .base
      .identity()
  ); // cc:433-434
  assert_eq!(
    AllocationSpace::SharedTrustedLoSpace,
    SharedTrustedLargeObjectSpace::new(heap_fake())
      .old
      .space
      .base
      .identity()
  ); // cc:436-437
  assert_eq!(
    AllocationSpace::TrustedLoSpace,
    TrustedLargeObjectSpace::new(heap_fake())
      .old
      .space
      .base
      .identity()
  ); // cc:439-440
  assert_eq!(
    AllocationSpace::NewLoSpace,
    NewLargeObjectSpace::new(heap_fake(), 0)
      .space
      .base
      .identity()
  ); // cc:330-331
  info!("six_wrapper_ctors_ids_gold ok");
  OK
}

/// HCR1-F6 落地后的转发形: Old 三参 (cc:98-102) / Code 三参 (cc:417-422) 均为单行直调
/// 四参体 — 运行时终抵台账 1 的 todo 体 (large-spaces.cc:104-141), 断言 panic 文案锚定
/// 转发终点即证明 3 参槽自身不再持 todo (域参数 NOT_EXECUTABLE/EXECUTABLE 的逐字对齐为
/// 源面静态事实, 见 large_spaces.rs:716/926 落点)。
#[test]
#[should_panic(expected = "large-spaces.cc:104-141")]
fn old_allocate_raw_forwards_to_executability_body() {
  let mut old = OldLargeObjectSpace::new(heap_fake());
  let _ = old.allocate_raw(null_mut(), CPS as i32, AllocationHint::default());
}

/// 同上 Code 支 (cc:417-422 → 同一四参体)。
#[test]
#[should_panic(expected = "large-spaces.cc:104-141")]
fn code_allocate_raw_forwards_to_executability_body() {
  let mut code = CodeLargeObjectSpace::new(heap_fake());
  let _ = code.allocate_raw(null_mut(), CPS as i32, AllocationHint::default());
}

/// New 支 allocate_raw (cc:333-369) 仍持自身 todo (台账 1/2: Heap 扩容判据 +
/// AllocateLargePage 未落地) — 文案锚与 Old/Code 转发终点区分。
#[test]
#[should_panic(expected = "large-spaces.cc:333-369")]
fn new_allocate_raw_still_todo_anchor() {
  let mut nlo = NewLargeObjectSpace::new(heap_fake(), 0);
  let _ = nlo.allocate_raw(null_mut(), CPS as i32, AllocationHint::default());
}
