//! tests/memory_allocator_unittest.rs — v8-heap-core/src/memory_allocator.rs 金标准用例 (HC-2b)。
//!
//! 原版测试勘定实测 (口径申报, 与票面所述一并核对):
//! - `v8/test/unittests/heap/heap-allocator-unittest.cc` (实测 53 行) 仅 1 例
//!   `TEST_F(HeapAllocatorTest, TryResizeLargeObject)` — 经 Isolate 取 **MainHeapAllocator**
//!   (`src/heap/main-allocator.{h,cc}`, HC-2c 未开) → 与 MemoryAllocator 基座无直接可迁面;
//!   票面 "该文件主要测 MainHeapAllocator" 与实况一致。
//! - 全树 grep 实测与 MemoryAllocator 直接相关的用例确实存在, 但全部经运行态或本模块
//!   推迟台账面: `spaces-unittest.cc:726 SpacesTest.MemoryAllocator` 与 `:1088
//!   SpacesTest.NoMemoryForNewPage` (AllocatePage + NormalPage owner/next_page 链,
//!   台账 1/3); `inner-pointer-resolution-unittest.cc:55/72/96` 与 `heap-unittest.cc:469`
//!   (`Free(kImmediately, page)`, 台账 7); `weak-references-unittest.cc:572`
//!   (`ReleasePooledChunksImmediately`, 本档 pool 查询用例直测落地面);
//!   构造面 `spaces-unittest.cc:57-95 TestMemoryAllocatorScope` (h:414-415 friend 实体) 依赖
//!   Isolate/Heap/CodePages → 不可直迁。
//! - 结论: 可迁面为零 → 按 memory_pool_unittest (16 例) 先例, 依原版 CHECK/DCHECK 断言语义
//!   与 h/cc 注释契约自造等价覆盖 19 例, 映射如下:
//!   h:43-66 枚举 ABI → allocation_and_free_mode_gold;
//!   cc:674-680 + h:69 → initialize_once_per_process_gold / 旗标双支;
//!   h:71-79 DCHECK_LT 界 → commit_page_size_getters_gold;
//!   cc:36-52 (RoundUp + DCHECK_NOT_NULL ×4) + h:334-409 字段初值 → ctor_*_gold;
//!   h:120-130 Available 三目 → size_and_available_saturation_gold;
//!   h:132-152 (两 executability 支 + AND 语义) → is_outside_allocated_space_matrix_gold;
//!   cc:781-815 CAS 循环 → update_allocated_space_limits_gold;
//!   h:163-205 (13 空间分派) → page_allocator_dispatch_matrix_gold;
//!   cc:739-763 (insert/erase DCHECK) + cc:715-720 → record_and_lookup_normal_page_gold;
//!   cc:721-734 (upper_bound 前驱 + DCHECK_IMPLIES) → lookup_large_page_upper_bound_gold;
//!   cc:76-92 池查询三目 + h:226-227 → pooled_chunks_*_gold;
//!   cc:54-74 (ReleaseOnTearDown/DCHECK + capacity 归零 + Free + 四位置空) → tear_down_gold;
//!   cc:173-188 + h:252-253 → compute_chunk_size_gold;
//!   cc:506-511 → is_page_pool_supported_for_large_space_matrix_gold;
//!   h:229-236/329-332 + cc:817-830 (#ifdef DEBUG 登记) → debug_executable_memory_*_gold;
//!   h:239-250/255 → memory_chunk_allocation_result_and_pool_accessor_gold;
//!   h:391-393 (unordered_set/std::set 容器承接, 台账 19 注) → chunk_containers_membership_gold;
//!   h:293-305/404-409 (延迟页三桶, 实体流台账 7/10) → delayed_page_vectors_buckets_gold;
//!   h:81 + cc:766-779 前置恒等式 (台账 21) → memory_chunk_address_roundtrip_gold。
//!
//! 页面对象/VirtualMemory/LOG/zapping 实体属他票 (本模块推迟台账 1-14) → 以
//! [`Reservation`] 测试替身 + [`ChunkContainsEnv`] 宿主注入承接 (memory_pool_unittest 同型)。
//!
//! 进程级静态 (cc:33-34 commit_page_size_) 的用例互不干扰依赖 nextest 每例独立进程。

use std::{
  collections::HashSet,
  ffi::c_void,
  ptr::{addr_of_mut, null_mut},
  sync::atomic::Ordering,
};

use aok::{OK, Void};
use log::info;
use v8_base::{
  address_region::AddressRegion, macros::round_up_address, page_allocator::os_commit_page_size,
};
use v8_base_platform::TimeDelta;
use v8_common::{
  Address, AllocationSpace, Executability, K_NULL_ADDRESS, K_PAGE_SIZE_BITS, K_REGULAR_PAGE_SIZE,
  KB,
};
use v8_flags::Flags;
use v8_heap_core::{
  memory_allocator::*,
  memory_chunk::{K_INSTRUCTION_STREAM_HEADER_SIZE_DERIVED, MemoryChunk, MemoryChunkLayout},
  memory_pool::{
    Config, Isolate, MemoryPool, PooledPage, ReleasePooledChunksTask, ReleaseTaskHost, Reservation,
  },
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

// ══════════════════════════════════════════════════════════════════════════
// 测试替身 (memory_pool_unittest.rs:38-113 同型, 本 bin 自立)
// ══════════════════════════════════════════════════════════════════════════

/// VirtualMemory 替身 (本模块推迟台账 3/6/9 面): 预留位型 (address,size) + reserved 开关。
#[derive(Clone, Copy, Debug)]
struct TestRes {
  address: Address,
  size: usize,
  reserved: bool,
}

impl TestRes {
  fn reserved(address: Address, size: usize) -> Self {
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

/// ReleaseTaskHost 替身 (池 add 面只需投递计数)。
#[derive(Default)]
struct TestHost {
  posts: usize,
}

impl ReleaseTaskHost for TestHost {
  fn post_delayed_release_task(
    &mut self,
    _task: &ReleasePooledChunksTask,
    _pool_ctm: &v8_tasks::CancelableTaskManager,
    _delay: TimeDelta,
  ) {
    self.posts += 1;
  }
  fn print(&mut self, _msg: &str) {}
}

/// 假 isolate (原版仅透传指针; h:334 isolate_)。
fn iso(tag: usize) -> *mut Isolate {
  (tag * 0x1000) as *mut Isolate
}

/// 假 v8::PageAllocator (h:339-355 四路 allocator, 从不解引用)。
fn pa(tag: usize) -> *mut PageAllocator {
  (tag * 0x2000) as *mut PageAllocator
}

/// 四个 allocator 位标 (ctor 参数注入顺序的观测锚)。
const DATA: usize = 1;
const RO: usize = 2;
const CODE: usize = 3;
const TRUSTED: usize = 4;

/// cc:36-39 ctor 的测试构造位 (isolate = iso(7))。
fn new_allocator(max_capacity: usize, pool: *mut MemoryPool<TestRes>) -> MemoryAllocator<TestRes> {
  MemoryAllocator::new(
    iso(7),
    pa(CODE),
    pa(TRUSTED),
    pool,
    max_capacity,
    pa(DATA),
    pa(RO),
  )
}

/// 页对齐基址 (MemoryChunk::alignment() = 1 << kPageSizeBits, h:393/chunk 落地)。
fn chunk_base(slot: usize) -> Address {
  slot * MemoryChunk::alignment()
}

/// cc:720/731 MetadataNoIsolateCheck()->Contains(addr) 的宿主替身:
/// 命中表按 (chunk 基址, 地址) 精确登记。
#[derive(Default)]
struct TestContains {
  hits: HashSet<(Address, Address)>,
}

impl ChunkContainsEnv for TestContains {
  fn metadata_contains(&self, chunk: *const MemoryChunk, addr: Address) -> bool {
    self.hits.contains(&(chunk as Address, addr))
  }
} // ══════════════════════════════════════════════════════════════════════════
// 枚举 ABI (h:43-66)
// ══════════════════════════════════════════════════════════════════════════

/// h:43-49 AllocationMode 与 h:51-66 FreeMode 判别值金标准 (enum class 默认 int)。
#[test]
fn allocation_and_free_mode_gold() -> Void {
  // h:44-48
  assert_eq!(0, AllocationMode::Regular as i32);
  assert_eq!(1, AllocationMode::TryDelayedAndPooled as i32);
  // h:52-65
  assert_eq!(0, FreeMode::Immediately as i32);
  assert_eq!(1, FreeMode::Pool as i32);
  assert_eq!(2, FreeMode::DelayThenRelease as i32);
  assert_eq!(3, FreeMode::DelayThenPool as i32);
  // h:57-61 注释 (延迟页期间保持 Isolate-local) 与 h:62-65 (延迟后入池) 为不同模式
  assert_ne!(FreeMode::DelayThenRelease, FreeMode::DelayThenPool);
  // h:46-48 注释: 先试延迟与池化 → 与常规路径互斥
  assert_ne!(AllocationMode::Regular, AllocationMode::TryDelayedAndPooled);
  info!("allocation_and_free_mode_gold ok");
  OK
}

// ══════════════════════════════════════════════════════════════════════════
// 进程级提交页大小 (h:68-79 + cc:33-34/674-680)
// ══════════════════════════════════════════════════════════════════════════

/// cc:674-680 双支: 旗标 >0 → `v8_os_page_size * KB` (cc:675-676);
/// 旗标 = 0 → `CommitPageSize()` (cc:677); cc:678 CHECK(IsPowerOfTwo) + cc:679 WhichPowerOfTwo。
#[test]
fn initialize_once_per_process_gold() -> Void {
  // cc:675-676 旗标支 (差异申报: v8_flags 全局 → &Flags 参数注入, free_list 先例)。
  let mut flags = Flags::default();
  flags.set_v8_os_page_size(4).unwrap();
  assert_eq!(4, flags.v8_os_page_size());
  MemoryAllocator::<TestRes>::initialize_once_per_process(&flags);
  assert_eq!(
    4 * KB,
    MemoryAllocator::<TestRes>::get_commit_page_size() as usize
  );
  assert_eq!(12, MemoryAllocator::<TestRes>::get_commit_page_size_bits());

  // cc:677 缺省支: 旗标 0 → 平台 CommitPageSize (posix 与 allocate 页大小一致)。
  let flags = Flags::default();
  assert_eq!(0, flags.v8_os_page_size());
  MemoryAllocator::<TestRes>::initialize_once_per_process(&flags);
  let expect = os_commit_page_size();
  assert!(expect.is_power_of_two(), "cc:678 CHECK(IsPowerOfTwo)");
  assert_eq!(
    expect as isize,
    MemoryAllocator::<TestRes>::get_commit_page_size()
  );
  assert_eq!(
    expect.trailing_zeros() as isize,
    MemoryAllocator::<TestRes>::get_commit_page_size_bits()
  );
  info!("initialize_once_per_process_gold ok");
  OK
}

/// h:71-74/76-79 `GetCommitPageSize{,Bits}()` DCHECK_LT(0, ...) 界 + 两者恒等式
/// (cc:679 `WhichPowerOfTwo(commit_page_size_)` = 1<<bits 复原)。
#[test]
fn commit_page_size_getters_gold() -> Void {
  let mut flags = Flags::default();
  // h:72/78 DCHECK_LT(0, …): 旗标为 1..16 KiB 的 2 的幂, 尺寸与位数逐点核对。
  for (kb, bits) in [(1usize, 10i32), (2, 11), (4, 12), (8, 13), (16, 14)] {
    flags.set_v8_os_page_size(kb as i32).unwrap();
    MemoryAllocator::<TestRes>::initialize_once_per_process(&flags);
    let size = MemoryAllocator::<TestRes>::get_commit_page_size();
    assert!(0 < size, "h:72 DCHECK_LT(0, commit_page_size_)");
    assert_eq!(kb * KB, size as usize);
    let got_bits = MemoryAllocator::<TestRes>::get_commit_page_size_bits();
    assert!(0 < got_bits, "h:78 DCHECK_LT(0, commit_page_size_bits_)");
    assert_eq!(bits as isize, got_bits);
    // 恒等式: size == 1 << bits
    assert_eq!(size, 1 << got_bits);
  }
  info!("commit_page_size_getters_gold ok");
  OK
}

// ══════════════════════════════════════════════════════════════════════════
// 构造 (cc:36-52) 与字段初值 (h:334-409)
// ══════════════════════════════════════════════════════════════════════════

/// cc:46 `capacity_ = RoundUp(capacity, NormalPage::kPageSize)` + cc:48-51 四个
/// DCHECK_NOT_NULL 的入参面 + h:360-377 边界初值 (低界 `-1ll`, 高界 kNullAddress) +
/// h:379-409 容器初值 (全空 / None)。
#[test]
fn ctor_capacity_roundup_and_initial_bookkeeping_gold() -> Void {
  let max_capacity = K_REGULAR_PAGE_SIZE + 100;
  let mut a = new_allocator(max_capacity, null_mut());

  // cc:46: 上取整到 NormalPage::kPageSize (= kRegularPageSize, spaces.h 别名链)。
  assert_eq!(2 * K_REGULAR_PAGE_SIZE, a.capacity);
  // cc:40-47 其余初始化列表项: size/size_executable 归零, 池指针透传。
  assert_eq!(0, a.size());
  assert_eq!(0, a.size_executable());
  assert!(a.pool.is_null());
  assert_eq!(iso(7) as usize, a.isolate as usize);

  // h:367/376 `static_cast<Address>(-1ll)` 低界; h:374/377 `{kNullAddress}` 高界。
  assert_eq!(
    !0usize,
    a.lowest_not_executable_ever_allocated
      .load(Ordering::SeqCst)
  );
  assert_eq!(
    K_NULL_ADDRESS,
    a.highest_not_executable_ever_allocated
      .load(Ordering::SeqCst)
  );
  assert_eq!(
    !0usize,
    a.lowest_executable_ever_allocated.load(Ordering::SeqCst)
  );
  assert_eq!(
    K_NULL_ADDRESS,
    a.highest_executable_ever_allocated.load(Ordering::SeqCst)
  );

  // h:379 optional<VirtualMemory> 空 + h:391-409 五容器空。
  assert!(a.reserved_chunk_at_virtual_memory_limit.is_none());
  assert!(a.normal_pages.is_empty());
  assert!(a.large_pages.is_empty());
  assert!(a.delayed_then_pooled_pages.is_empty());
  assert!(a.delayed_then_pooled_large_pages.is_empty());
  assert!(a.delayed_then_released_pages.is_empty());
  assert!(a.memory_pool().is_null());

  // h:163-181 访问器透传 (cc:41-45 取源推迟台账 17 → 参数注入的可观测面)。
  assert_eq!(pa(DATA) as usize, a.data_page_allocator() as usize);
  assert_eq!(pa(RO) as usize, a.read_only_page_allocator as usize);
  assert_eq!(pa(CODE) as usize, a.code_page_allocator() as usize);
  assert_eq!(pa(TRUSTED) as usize, a.trusted_page_allocator() as usize);

  // 已落地的 cc:48-51 DCHECK_NOT_NULL ×4 正例: 四个 allocator 均非空即通过;
  // 反例 (置空) 走 debug_assert panic → 不可在 OK 路径内构造, 记录于本档口径。
  info!("ctor_capacity_roundup_and_initial_bookkeeping_gold ok");
  OK
}

/// h:120-130 `Size()/SizeExecutable()/Available()` — h:129 三目
/// `capacity_ < size_ ? 0 : capacity_ - size_` 的饱和语义。
#[test]
fn size_and_available_saturation_gold() -> Void {
  let a = new_allocator(2 * K_REGULAR_PAGE_SIZE, null_mut());
  let capacity = a.capacity;
  assert_eq!(capacity, a.available());

  // h:121 Size 读出 + h:130 正常差值。
  a.size_.store(capacity / 4, Ordering::SeqCst);
  assert_eq!(capacity / 4, a.size());
  assert_eq!(capacity - capacity / 4, a.available());

  // h:129 三目前支 (capacity_ < size_) → 0, 不允许下溢。
  a.size_.store(capacity + 1, Ordering::SeqCst);
  assert_eq!(0, a.available());
  a.size_.store(usize::MAX, Ordering::SeqCst);
  assert_eq!(0, a.available());

  // h:124 SizeExecutable 独立记账 (h:362-363 原子位)。
  a.size_executable_.store(4096, Ordering::SeqCst);
  assert_eq!(4096, a.size_executable());
  info!("size_and_available_saturation_gold ok");
  OK
}

// ══════════════════════════════════════════════════════════════════════════
// 越界判定 (h:132-152) 与边界更新 (cc:781-815)
// ══════════════════════════════════════════════════════════════════════════

/// h:137-140 `IsOutsideAllocatedSpace(Address)` = 两个 executability 支都越界 (AND,
/// h:134-136 注释: 保守, 允许假阴性不允许假阳性) + h:141-152 switch 双支区间判据
/// (`addr < lowest || addr >= highest`)。
#[test]
fn is_outside_allocated_space_matrix_gold() -> Void {
  let mut a = new_allocator(0, null_mut());
  // h:365-377 初值 (低界 -1ll / 高界 kNullAddress) 下任意地址两支皆判越界。
  for addr in [0usize, 1, 0x1500, usize::MAX] {
    assert!(a.is_outside_allocated_space(addr));
  }

  // 非可执行区间 [0x1000, 0x2000)。
  a.update_allocated_space_limits(0x1000, 0x2000, Executability::NotExecutable);
  assert!(!a.is_outside_allocated_space_of(0x1000, Executability::NotExecutable));
  assert!(!a.is_outside_allocated_space_of(0x1500, Executability::NotExecutable));
  assert!(!a.is_outside_allocated_space_of(0x1FFF, Executability::NotExecutable));
  // h:150 `>=` 高界含端点 → 越界。
  assert!(a.is_outside_allocated_space_of(0x2000, Executability::NotExecutable));
  assert!(a.is_outside_allocated_space_of(0x0FFF, Executability::NotExecutable));

  // h:134-138 注释 (保守: 允许假阴性, 不允许假阳性) — 可执行支仍全越界, 但地址在非可执行
  // 区间内 → AND (h:139-140) 短路为 "在内", 即 h:136-137 允许的假阴性。
  assert!(a.is_outside_allocated_space_of(0x1500, Executability::Executable));
  assert!(!a.is_outside_allocated_space(0x1500));

  // 可执行区间 [0x1800, 0x4000) → 只有两支皆外才判越界。
  a.update_allocated_space_limits(0x1800, 0x4000, Executability::Executable);
  assert!(!a.is_outside_allocated_space(0x1800)); // 两支皆内 (交集)
  assert!(!a.is_outside_allocated_space(0x1FFF));
  assert!(!a.is_outside_allocated_space(0x1000)); // 非可执行内 + 可执行外 → 假阴性
  assert!(!a.is_outside_allocated_space(0x2000)); // 可执行内 + 非可执行外 → 假阴性
  assert!(a.is_outside_allocated_space(0x0FFF)); // 两支皆外
  assert!(a.is_outside_allocated_space(0x4000)); // 两支皆外 (高界含端点)
  info!("is_outside_allocated_space_matrix_gold ok");
  OK
}

/// cc:781-815: 四原子位各自的 CAS 循环保序 (cc:790/797/805/811 `low < ptr` /
/// `high > ptr`) — 低界只降、高界只升; executability 两支互不串写。
#[test]
fn update_allocated_space_limits_gold() -> Void {
  let mut a = new_allocator(0, null_mut());

  // cc:789-799 非可执行支: 连续收缩低界 + 扩张高界。
  a.update_allocated_space_limits(0x8000, 0x9000, Executability::NotExecutable);
  assert_eq!(
    0x8000,
    a.lowest_not_executable_ever_allocated
      .load(Ordering::SeqCst)
  );
  assert_eq!(
    0x9000,
    a.highest_not_executable_ever_allocated
      .load(Ordering::SeqCst)
  );
  a.update_allocated_space_limits(0x1000, 0x2000, Executability::NotExecutable);
  assert_eq!(
    0x1000,
    a.lowest_not_executable_ever_allocated
      .load(Ordering::SeqCst)
  );
  assert_eq!(
    0x9000,
    a.highest_not_executable_ever_allocated
      .load(Ordering::SeqCst)
  );
  // cc:797 `high > ptr` 不成立 → 高界不回退。
  a.update_allocated_space_limits(0x0800, 0x1000, Executability::NotExecutable);
  assert_eq!(
    0x0800,
    a.lowest_not_executable_ever_allocated
      .load(Ordering::SeqCst)
  );
  assert_eq!(
    0x9000,
    a.highest_not_executable_ever_allocated
      .load(Ordering::SeqCst)
  );
  // cc:790 `low < ptr` 不成立 (同值幂等) → 两次调用等价 (CAS 循环退出)。
  a.update_allocated_space_limits(0x0800, 0x9000, Executability::NotExecutable);
  assert_eq!(
    0x0800,
    a.lowest_not_executable_ever_allocated
      .load(Ordering::SeqCst)
  );
  assert_eq!(
    0x9000,
    a.highest_not_executable_ever_allocated
      .load(Ordering::SeqCst)
  );

  // cc:801-812 可执行支: 独立四原子位。
  a.update_allocated_space_limits(0x3000, 0x4000, Executability::Executable);
  assert_eq!(
    0x3000,
    a.lowest_executable_ever_allocated.load(Ordering::SeqCst)
  );
  assert_eq!(
    0x4000,
    a.highest_executable_ever_allocated.load(Ordering::SeqCst)
  );
  a.update_allocated_space_limits(0x2000, 0x8000, Executability::Executable);
  assert_eq!(
    0x2000,
    a.lowest_executable_ever_allocated.load(Ordering::SeqCst)
  );
  assert_eq!(
    0x8000,
    a.highest_executable_ever_allocated.load(Ordering::SeqCst)
  );
  // 两支互不串写。
  assert_eq!(
    0x0800,
    a.lowest_not_executable_ever_allocated
      .load(Ordering::SeqCst)
  );
  assert_eq!(
    0x9000,
    a.highest_not_executable_ever_allocated
      .load(Ordering::SeqCst)
  );
  info!("update_allocated_space_limits_gold ok");
  OK
}

// ══════════════════════════════════════════════════════════════════════════
// 分配器分派 (h:183-205)
// ══════════════════════════════════════════════════════════════════════════

/// h:184-205 `page_allocator(AllocationSpace)` 13 空间四支分派金标准
/// (switch 尾 UNREACHABLE 由 Rust 穷尽 match 承接 → 13 支全覆盖)。
#[test]
fn page_allocator_dispatch_matrix_gold() -> Void {
  let a = new_allocator(0, null_mut());

  // h:186-187 RO_SPACE → read_only_page_allocator_。
  assert_eq!(
    pa(RO) as usize,
    a.page_allocator(AllocationSpace::RoSpace) as usize
  );
  // h:189-191 CODE_SPACE / CODE_LO_SPACE → code_page_allocator_。
  assert_eq!(
    pa(CODE) as usize,
    a.page_allocator(AllocationSpace::CodeSpace) as usize
  );
  assert_eq!(
    pa(CODE) as usize,
    a.page_allocator(AllocationSpace::CodeLoSpace) as usize
  );
  // h:192-197 trusted 四空间 (含 shared 两支)。
  for space in [
    AllocationSpace::TrustedSpace,
    AllocationSpace::SharedTrustedSpace,
    AllocationSpace::TrustedLoSpace,
    AllocationSpace::SharedTrustedLoSpace,
  ] {
    assert_eq!(pa(TRUSTED) as usize, a.page_allocator(space) as usize);
  }
  // h:198-201 其余数据空间 → data_page_allocator_。
  for space in [
    AllocationSpace::NewSpace,
    AllocationSpace::NewLoSpace,
    AllocationSpace::OldSpace,
    AllocationSpace::LoSpace,
    AllocationSpace::SharedSpace,
    AllocationSpace::SharedLoSpace,
  ] {
    assert_eq!(pa(DATA) as usize, a.page_allocator(space) as usize);
  }
  // 13 支穷尽性自查 (与 h:184-205 空间清单同规模)。
  let covered = 1 + 2 + 4 + 6;
  assert_eq!(13, covered);
  info!("page_allocator_dispatch_matrix_gold ok");
  OK
}

// ══════════════════════════════════════════════════════════════════════════
// chunk 登记与地址反查 (cc:706-763)
// ══════════════════════════════════════════════════════════════════════════

/// cc:739-750/752-763 Record 族 (normal/large 分桶 + 重复插入/未登记 erase 的 DCHECK 位)
/// 与 cc:715-720 normal 命中支 (found 但 Contains 假 → cc:736 nullptr, 不入 large 支)。
#[test]
fn record_and_lookup_normal_page_gold() -> Void {
  let mut a = new_allocator(0, null_mut());
  let mut env = TestContains::default();
  let base = chunk_base(10);
  let chunk = MemoryChunk::from_address(base + 16);

  // cc:746-749 normal 登记 (cc:743-744 DCHECK(result.second) 正例)。
  a.record_memory_chunk_created(chunk, false);
  assert_eq!(1, a.normal_pages.len());
  assert!(a.large_pages.is_empty());

  // cc:715-720: Contains 真 → 返回该 chunk (cc:719-720 注释面: 免 isolate 校验)。
  env.hits.insert((base, base + 16));
  let found = a.lookup_chunk_containing_address(base + 16, &env);
  assert_eq!(chunk as usize, found as usize);
  // h:207-210 注释: 槽位地址未必合法 → 非登记内地址 (归位后不等于基址) 不命中。
  assert_eq!(
    0usize,
    a.lookup_chunk_containing_address(base - 16, &env) as usize
  );

  // Contains 假 (登记在集合但区间不含) → cc:733-736 nullptr, 且不走 large 支。
  a.record_memory_chunk_created(MemoryChunk::from_address(chunk_base(200)), true);
  assert!(a.lookup_chunk_containing_address(base + 32, &env).is_null());

  // cc:706-710 safepoint 版与加锁版等价 (cc:703 转发)。
  assert_eq!(
    a.lookup_chunk_containing_address(base + 16, &env) as usize,
    a.lookup_chunk_containing_address_in_safepoint(base + 16, &env) as usize
  );

  // cc:752-763 注销 (erase 计数 1u 正例) — 分桶各归各。
  a.record_memory_chunk_destroyed(chunk, false);
  assert!(a.normal_pages.is_empty());
  assert_eq!(1, a.large_pages.len());
  a.record_memory_chunk_destroyed(MemoryChunk::from_address(chunk_base(200)), true);
  assert!(a.large_pages.is_empty());
  assert!(a.lookup_chunk_containing_address(base + 16, &env).is_null());
  info!("record_and_lookup_normal_page_gold ok");
  OK
}

/// cc:721-734 large_pages_ `upper_bound` 前驱查找: 归位基址未登记时取"不大于它的最大元素"
/// (cc:723 注释: chunk 可能在某个大页内), cc:724-725 DCHECK_IMPLIES (upper_bound 基址 > addr)
/// 与 cc:727-728 DCHECK_NOT_NULL/DCHECK_LE 的可观测序。
#[test]
fn lookup_large_page_upper_bound_gold() -> Void {
  let mut a = new_allocator(0, null_mut());
  let mut env = TestContains::default();
  for slot in [100usize, 200, 300] {
    a.record_memory_chunk_created(MemoryChunk::from_address(chunk_base(slot)), true);
  }
  assert_eq!(3, a.large_pages.len());

  // 归位基址恰为登记元素 (addr = 200·alignment + 8) → 前驱即自身 (相等入集)。
  let addr = chunk_base(200) + 8;
  env.hits.insert((chunk_base(200), addr));
  let found = a.lookup_chunk_containing_address_in_safepoint(addr, &env);
  assert_eq!(chunk_base(200), found as usize);

  // 归位基址落在空隙 (250·alignment) → 前驱 = 200·alignment (cc:726 std::next(it, -1))。
  let gap = chunk_base(250) + 4;
  env.hits.insert((chunk_base(200), gap));
  let found = a.lookup_chunk_containing_address_in_safepoint(gap, &env);
  assert_eq!(chunk_base(200), found as usize);

  // 最小元素之下 (50·alignment) → upper_bound == begin() → cc:721-722 支不进入 → nullptr。
  assert!(
    a.lookup_chunk_containing_address_in_safepoint(chunk_base(50), &env)
      .is_null()
  );

  // 最大元素之上 (350·alignment): 前驱 = 300·alignment, Contains 未登记 → nullptr。
  assert!(
    a.lookup_chunk_containing_address_in_safepoint(chunk_base(350), &env)
      .is_null()
  );
  env.hits.insert((chunk_base(300), chunk_base(350)));
  let found = a.lookup_chunk_containing_address_in_safepoint(chunk_base(350), &env);
  assert_eq!(chunk_base(300), found as usize);

  // normal 优先 (cc:715 find 命中 → else-if 不入 large 支)。
  a.record_memory_chunk_created(MemoryChunk::from_address(chunk_base(200)), false);
  let found = a.lookup_chunk_containing_address_in_safepoint(addr, &env);
  assert_eq!(chunk_base(200), found as usize);
  // normal 命中支内 Contains 假 → cc:733-736 nullptr, 不回退 large 支 (空判据宿主)。
  let empty_env = TestContains::default();
  assert!(
    a.lookup_chunk_containing_address_in_safepoint(chunk_base(250) + 4, &empty_env)
      .is_null(),
    "normal 命中支内 Contains 假即返回 null, 不回退 large 支"
  );
  assert!(
    a.lookup_chunk_containing_address_in_safepoint(addr, &empty_env)
      .is_null()
  );
  // large 支同判据: 前驱存在但 Contains 假 → nullptr (cc:732-733 支内不返回)。
  a.record_memory_chunk_destroyed(MemoryChunk::from_address(chunk_base(200)), false);
  assert!(
    a.lookup_chunk_containing_address_in_safepoint(chunk_base(250) + 4, &empty_env)
      .is_null()
  );
  info!("lookup_large_page_upper_bound_gold ok");
  OK
}

// ══════════════════════════════════════════════════════════════════════════
// 池查询门面 (cc:76-92 + h:226-227/255)
// ══════════════════════════════════════════════════════════════════════════

/// cc:76-86 三个查询的 `pool ? … : 0` 三目两面 (无池 → 0; 弱引用单测 cc:572 的
/// ReleasePooledChunksImmediately 面即本处落地面直测) + h:255 memory_pool()。
#[test]
fn pooled_chunks_queries_gold() -> Void {
  let mut pool = MemoryPool::<TestRes>::new(Config::default());
  let pool_ptr = addr_of_mut!(pool);
  let mut a = new_allocator(0, pool_ptr);
  let mut host = TestHost::default();

  // 无池面 (cc:77/81/85 三目左支)。
  let mut bare = new_allocator(0, null_mut());
  assert_eq!(0, bare.get_pooled_chunks_count());
  assert_eq!(0, bare.get_shared_pooled_chunks_count());
  assert_eq!(0, bare.get_total_pooled_chunks_count());
  bare.release_pooled_chunks_immediately(); // cc:88-92 判空即返 (无副作用)
  assert!(bare.memory_pool().is_null());

  // 本 isolate 本地桶 (cc:77 GetCount = local_size)。
  a.release_pooled_chunks_immediately();
  assert_eq!(0, a.get_pooled_chunks_count());
  pool.add(
    iso(7),
    PooledPage::new(null_mut(), TestRes::reserved(chunk_base(1), 4096), 0),
    &mut host,
  );
  pool.add(
    iso(7),
    PooledPage::new(null_mut(), TestRes::reserved(chunk_base(2), 4096), 0),
    &mut host,
  );
  assert_eq!(2, a.get_pooled_chunks_count());
  assert_eq!(0, a.get_shared_pooled_chunks_count()); // cc:81
  assert_eq!(2, a.get_total_pooled_chunks_count()); // cc:85
  // 池投递合批 (cc:514-529): 首投后 `timeout + kTimeSlack` 内不补投 → 两次 add 仅 1 投
  // (投递面本体在 memory_pool_unittest 已金标准核对, 本处只锚 allocator→pool 的转接)。
  assert_eq!(1, host.posts);

  // 他 isolate 的本地桶不计入本 isolate (h:222-224 注释: Total 含之)。
  pool.add(
    iso(8),
    PooledPage::new(null_mut(), TestRes::reserved(chunk_base(3), 4096), 0),
    &mut host,
  );
  assert_eq!(2, a.get_pooled_chunks_count());
  assert_eq!(3, a.get_total_pooled_chunks_count());

  // cc:88-92 ReleaseImmediately: 本地桶清空, 他 isolate 与共享池不动。
  a.release_pooled_chunks_immediately();
  assert_eq!(0, a.get_pooled_chunks_count());
  assert_eq!(1, a.get_total_pooled_chunks_count());
  assert_eq!(pool_ptr as usize, a.memory_pool() as usize);

  // 池收口 (memory_pool_unittest:641-643 先例): 三池清空 + CTM 取消 (h:166-169/
  // cc:371-377 的 Drop DCHECK 前置)。
  pool.release_all_immediately(); // cc:363-369
  assert_eq!(0, a.get_total_pooled_chunks_count());
  pool.tear_down(); // cc:371-377
  info!("pooled_chunks_queries_gold ok");
  OK
}

/// cc:54-74 `TearDown()`: ReleaseOnTearDown + cc:57 DCHECK_EQ(GetCount, 0) 正例;
/// cc:61 DCHECK_EQ(size_, 0u) 正例; cc:64 capacity_ = 0; cc:66-68 末位保留 chunk 的 Free;
/// cc:70-73 四 allocator 置空 (cc:62-63 TODO 注释区的 size_executable 断言原版被注释 → 不复活)。
#[test]
fn tear_down_gold() -> Void {
  let mut pool = MemoryPool::<TestRes>::new(Config::default());
  let pool_ptr = addr_of_mut!(pool);
  let mut a = new_allocator(3 * K_REGULAR_PAGE_SIZE, pool_ptr);
  let mut host = TestHost::default();
  pool.add(
    iso(7),
    PooledPage::new(null_mut(), TestRes::reserved(chunk_base(9), 4096), 0),
    &mut host,
  );

  // cc:55-58 的 cc:64-68 前置面: 末位保留 chunk 存在 → Free 后 is_reserved 复位。
  a.reserved_chunk_at_virtual_memory_limit = Some(TestRes::reserved(chunk_base(77), 4096));
  // cc:62-63 注释 (TODO(gc)) 位: size_executable 非零不阻断拆除, 照录不复活的断言。
  a.size_executable_.store(4096, Ordering::SeqCst);

  a.tear_down();

  // 默认 Config share_memory_on_teardown → 本地桶转共享 (cc:346-355), GetCount 归零。
  assert_eq!(0, a.get_pooled_chunks_count());
  assert_eq!(1, a.get_shared_pooled_chunks_count());
  assert_eq!(1, a.get_total_pooled_chunks_count());
  // cc:61 断言面: size_ 保持 0 (spaces 先于本类拆除)。
  assert_eq!(0, a.size());
  // cc:64
  assert_eq!(0, a.capacity);
  assert_eq!(0, a.available());
  // cc:66-68 reserved_chunk_at_…->Free() (Option 本体保留, 所有权位归还)。
  let reserved = a.reserved_chunk_at_virtual_memory_limit.as_ref().unwrap();
  assert!(!reserved.is_reserved());
  // cc:70-73 四位置空。
  assert!(a.code_page_allocator().is_null());
  assert!(a.data_page_allocator().is_null());
  assert!(a.read_only_page_allocator.is_null());
  assert!(a.trusted_page_allocator().is_null());

  // 池收口 (共享池残留 1 条目): release_all_immediately + tear_down (cc:363-377)。
  pool.release_all_immediately();
  pool.tear_down();
  info!("tear_down_gold ok");
  OK
}

// ══════════════════════════════════════════════════════════════════════════
// 尺寸计算与池支持判据 (cc:173-188/506-511)
// ══════════════════════════════════════════════════════════════════════════

/// cc:173-188 `ComputeChunkSize(area_size, space)` = RoundUp(ObjectStartOffset +
/// area_size, GetCommitPageSize()) — 布局注释 (cc:175-183) 的区间恒等式 +
/// 码/数据空间布局分派 (h:252-253 声明)。
#[test]
fn compute_chunk_size_gold() -> Void {
  let mut flags = Flags::default();
  flags.set_v8_os_page_size(4).unwrap();
  MemoryAllocator::<TestRes>::initialize_once_per_process(&flags);
  let commit = 4 * KB;
  assert_eq!(
    commit as isize,
    MemoryAllocator::<TestRes>::get_commit_page_size()
  );

  for (space, is_code) in [
    (AllocationSpace::NewSpace, false),
    (AllocationSpace::OldSpace, false),
    (AllocationSpace::LoSpace, false),
    (AllocationSpace::RoSpace, false),
    (AllocationSpace::CodeSpace, true),
    (AllocationSpace::CodeLoSpace, true),
    (AllocationSpace::TrustedSpace, false),
  ] {
    let offset = MemoryChunkLayout::object_start_offset_in_memory_chunk(
      space,
      K_INSTRUCTION_STREAM_HEADER_SIZE_DERIVED,
    );
    // 布局分派核对 (memory_chunk.rs:505-513 is_any_code_space)。
    let expect_offset = if is_code {
      MemoryChunkLayout::object_start_offset_in_code_page(K_INSTRUCTION_STREAM_HEADER_SIZE_DERIVED)
    } else {
      MemoryChunkLayout::object_start_offset_in_data_page()
    };
    assert_eq!(expect_offset, offset);

    for area in [0usize, 1, 1000, commit - 1, commit, commit + 1] {
      let size = MemoryAllocator::<TestRes>::compute_chunk_size(area, space);
      // cc:185-187 RoundUp 面: 提交页对齐 + 覆盖 [offset, offset+area] 且不浪费整页。
      assert_eq!(0, size % commit, "cc:187 RoundUp(…, commit_page_size)");
      assert!(size >= offset + area);
      assert!(size - (offset + area) < commit);
      assert_eq!(round_up_address(offset + area, commit), size);
    }
  }
  info!("compute_chunk_size_gold ok");
  OK
}

/// cc:506-511 `IsPagePoolSupportedForLargeSpace` (匿名命名空间, cc:519 调用点):
/// identity ∈ {NEW_LO_SPACE, LO_SPACE} (h:100-103 AllocateLargePage 的池化白名单)。
#[test]
fn is_page_pool_supported_for_large_space_matrix_gold() -> Void {
  // cc:508 `space->identity()` 差异申报 (台账 18) → 入参取 AllocationSpace。
  assert!(
    MemoryAllocator::<TestRes>::is_page_pool_supported_for_large_space(AllocationSpace::NewLoSpace)
  );
  assert!(
    MemoryAllocator::<TestRes>::is_page_pool_supported_for_large_space(AllocationSpace::LoSpace)
  );
  for space in [
    AllocationSpace::NewSpace,
    AllocationSpace::OldSpace,
    AllocationSpace::SharedSpace,
    AllocationSpace::SharedLoSpace,
    AllocationSpace::CodeSpace,
    AllocationSpace::CodeLoSpace,
    AllocationSpace::TrustedSpace,
    AllocationSpace::TrustedLoSpace,
    AllocationSpace::SharedTrustedSpace,
    AllocationSpace::SharedTrustedLoSpace,
    AllocationSpace::RoSpace,
  ] {
    assert!(
      !MemoryAllocator::<TestRes>::is_page_pool_supported_for_large_space(space),
      "cc:509-510 白名单外: {space:?}"
    );
  }
  info!("is_page_pool_supported_for_large_space_matrix_gold ok");
  OK
}

// ══════════════════════════════════════════════════════════════════════════
// 目标配置 DEBUG 面承接 (h:229-236/329-332 + cc:817-830)
// ══════════════════════════════════════════════════════════════════════════

/// h:229-236 `IsMemoryChunkExecutable` + cc:817-830 登记/反登记 (原版 #ifdef DEBUG 区,
/// Rust `#[cfg(debug_assertions)]` 承接, allocation.rs DEBUG 账目先例)。
#[cfg(debug_assertions)]
#[test]
fn debug_executable_memory_registry_gold() -> Void {
  let mut a = new_allocator(0, null_mut());
  let p1 = chunk_base(1) as *mut MutablePage;
  let p2 = chunk_base(2) as *mut MutablePage;

  assert!(!a.is_memory_chunk_executable(p1));
  // cc:821 DCHECK_EQ(find, end) 正例 → 插入。
  a.register_executable_memory_chunk(p1);
  assert!(a.is_memory_chunk_executable(p1));
  assert!(!a.is_memory_chunk_executable(p2));
  assert_eq!(1, a.executable_memory.len());

  // cc:827 DCHECK_NE(find, end) 正例 → 删除。
  a.unregister_executable_memory_chunk(p1);
  assert!(!a.is_memory_chunk_executable(p1));
  assert!(a.executable_memory.is_empty());

  // 重复登记/未登记反登记走 debug_assert panic (cc:820 DCHECK(is_executable) 依赖
  // MutablePage 方法面 → 台账 14 注记跳过), 不在 OK 路径构造。
  info!("debug_executable_memory_registry_gold ok");
  OK
}

/// h:238-250 `MemoryChunkAllocationResult` 全数据面 + h:255 `memory_pool()` —
/// 结构体本身可落 (实体生产方推迟台账 2), 以位型搬运金标准承接。
#[test]
fn memory_chunk_allocation_result_and_pool_accessor_gold() -> Void {
  let mut pool = MemoryPool::<TestRes>::new(Config::default());
  let pool_ptr = addr_of_mut!(pool);
  let mut a = new_allocator(0, pool_ptr);

  // h:242-249 字段按声明序承载 (chunk/optional_metadata/size/area_start/area_end/reservation)。
  let result = MemoryChunkAllocationResult {
    chunk: chunk_base(3) as *mut c_void,
    optional_metadata: null_mut(),
    size: 8192,
    area_start: 64,
    area_end: 64 + 8000,
    reservation: TestRes::reserved(chunk_base(3), 8192),
  };
  assert_eq!(chunk_base(3), result.chunk as Address);
  assert!(result.optional_metadata.is_null());
  assert_eq!(8192, result.size);
  assert_eq!(64, result.area_start);
  assert_eq!(64 + 8000, result.area_end);
  assert_eq!(
    AddressRegion::new(chunk_base(3), 8192),
    result.reservation.region()
  );
  assert!(result.reservation.is_reserved());

  // cc:55/77/400 消费形态 `if (auto* pool = memory_pool())` 的判空源 (h:255)。
  assert!(!a.memory_pool().is_null());
  assert_eq!(pool_ptr as usize, a.memory_pool() as usize);
  a.pool = null_mut();
  assert!(a.memory_pool().is_null());
  assert_eq!(0, a.get_pooled_chunks_count());
  // 空池收口 (cc:371-377; CTM 须先取消, v8-tasks cancelable.rs:153 前置)。
  pool.tear_down();
  info!("memory_chunk_allocation_result_and_pool_accessor_gold ok");
  OK
}

// ══════════════════════════════════════════════════════════════════════════
// 承接面 (opaque 前向 / 容器分桶) 的最小可观测金标准
// ══════════════════════════════════════════════════════════════════════════

/// h:29-35 前向声明与 h:393 `std::set` 有序承接 (BTreeSet) 的结构面: 反查依赖升序序,
/// normal 走哈希集 — 两者成员判据一致而遍历序不同 (容器差异申报, 台账 19 注)。
#[test]
fn chunk_containers_membership_gold() -> Void {
  let mut a = new_allocator(0, null_mut());
  // 地址归位恒等式 (cc:711 FromAddress / h:305-306 kAlignment): 基址向下对齐。
  let base = chunk_base(42);
  assert_eq!(base, MemoryChunk::base_address(base + 7));
  assert_eq!(base, MemoryChunk::from_address(base + 7) as Address);

  for slot in [300usize, 100, 200] {
    a.record_memory_chunk_created(MemoryChunk::from_address(chunk_base(slot)), true);
  }
  // BTreeSet 升序 (cc:721 upper_bound 前驱查找的前提)。
  let ascending: Vec<Address> = a.large_pages.iter().map(|c| *c as Address).collect();
  assert_eq!(
    vec![chunk_base(100), chunk_base(200), chunk_base(300)],
    ascending
  );

  // HashSet 成员判据等价 (cc:715 find)。
  a.record_memory_chunk_created(MemoryChunk::from_address(base), false);
  assert!(
    a.normal_pages
      .contains(&MemoryChunk::from_address(base + 3))
  );
  assert!(
    !a.normal_pages
      .contains(&MemoryChunk::from_address(chunk_base(11)))
  );

  // 分桶规模即观测面 (cc:742/746 两支互不串写)。
  assert_eq!(3, a.large_pages.len());
  assert_eq!(1, a.normal_pages.len());
  info!("chunk_containers_membership_gold ok");
  OK
}

/// 延迟页三向量与 `FreeMode` 的对应承载面 (h:293-305 声明 + h:404/407/409 字段):
/// 实体流 (cc:366-430) 属推迟台账 7/10, 本处金标准锁定"三桶互不串写 + 顺序保留"。
#[test]
fn delayed_page_vectors_buckets_gold() -> Void {
  let mut a = new_allocator(0, null_mut());
  let p = |slot: usize| chunk_base(slot) as *mut NormalPage;
  let q = |slot: usize| chunk_base(slot) as *mut LargePage;
  let m = |slot: usize| chunk_base(slot) as *mut MutablePage;

  // h:404 delayed_then_pooled_pages_ (kDelayThenPool 常规页, 可立即复用 h:398-401 注释)。
  a.delayed_then_pooled_pages.push(p(1));
  a.delayed_then_pooled_pages.push(p(2));
  // h:407 delayed_then_pooled_large_pages_ (原版"不能复用"注释 h:405-406)。
  a.delayed_then_pooled_large_pages.push(q(3));
  // h:409 delayed_then_released_pages_ (kDelayThenRelease)。
  a.delayed_then_released_pages.push(m(4));

  assert_eq!(2, a.delayed_then_pooled_pages.len());
  assert_eq!(1, a.delayed_then_pooled_large_pages.len());
  assert_eq!(1, a.delayed_then_released_pages.len());
  // 入序保留 (cc:377-388 Free 的 push_back 序, ReleaseDelayedPages cc:412-428 依序处理)。
  assert_eq!(p(1) as usize, a.delayed_then_pooled_pages[0] as usize);
  assert_eq!(p(2) as usize, a.delayed_then_pooled_pages[1] as usize);
  info!("delayed_page_vectors_buckets_gold ok");
  OK
}

/// h:81 `DeleteMemoryChunk` 声明面的承接位实测: 本模块不落地实体 (台账 21),
/// 以"MemoryChunk 归位与 Metadata 替身无关"的纯地址运算金标准锚定不回归。
#[test]
fn memory_chunk_address_roundtrip_gold() -> Void {
  let align = MemoryChunk::alignment();
  assert_eq!(1 << K_PAGE_SIZE_BITS, align);
  assert_eq!(align - 1, MemoryChunk::alignment_mask());
  for addr in [0usize, 1, align - 1, align, align + 1, 123 * align + 7] {
    let base = MemoryChunk::base_address(addr);
    assert_eq!(0, base % align);
    assert!(base <= addr, "cc:717 DCHECK_LE(address(), addr) 恒等式");
    assert_eq!(base, MemoryChunk::from_address(addr) as Address);
  }
  // h:81 DeleteMemoryChunk(MutablePage*) 实体在 cc:766-779 (placement 析构 + free) →
  // 台账 21; 本面仅锚定"chunk 指针可由地址复原"的前置恒等式。
  let chunk = MemoryChunk::from_address(chunk_base(5));
  assert_eq!(chunk_base(5), chunk as Address);
  assert_eq!(K_REGULAR_PAGE_SIZE, align);
  info!("memory_chunk_address_roundtrip_gold ok");
  OK
}
