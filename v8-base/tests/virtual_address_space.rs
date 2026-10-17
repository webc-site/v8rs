//! 原版 v8/test/unittests/base/virtual-address-space-unittest.cc 1:1 迁移
//!
//! 规约 (每断言一行, cc:20-133 helper + cc:134-321 五个 TEST):
//! - TestPagePermissionSubsets: 25 条 IsSubset 真值表 (cc:141-169)
//! - TestRandomPageAddressGeneration: 播种后 10 次随机地址 ∈ [base, base+size) (cc:21-26)
//! - TestBasicPageAllocation: 21 档 KB 尺寸 (跳过未对齐), 分配非空/界内/可写,
//!   收集数 ≥ 11 (≥64KB 档全得), 回读后逐个释放 (cc:29-64)
//! - TestPageAllocationAlignment: 1..64 倍粒度对齐, 分配非空/对齐/界内/释放 (cc:66-84)
//! - TestParentSpaceCannotAllocateInChildSpace: 父空间以子空间随机地址为 hint
//!   分配 10 次, 结果必在子空间区间之外 (cc:86-104)
//! - TestSubspace: 32MB 子空间 + 16MB 子子空间, base≠0/size/max_perms 断言 + 全 helper
//! - TestEmulatedSubspace: 1MB 背书映射 + 32MB 模拟空间, base/size/max_perms + 全 helper
//!   (模拟子空间不保证父空间不落入, 故无 ParentCannotAllocate 断言, cc:264-265)
//!
//! 跳过项登记 (术语表规则):
//! - TestSharedPageAllocation (cc:106-132): SharedMemory 族 (AllocateSharedPages/
//!   FreeSharedPages) 未迁移, 属 v8-base-platform 职责 (src/virtual_address_space.rs 头部登记);
//!   原版在 handle 不可用时同样提前 return
//! - TestSubspaceNaming (cc:269-321): 原版 #ifdef V8_OS_LINUX (SignalSafeMapsParser
//!   仅 Linux); 本仓目标平台 darwin, prctl 命名路径不可用

use aok::{OK, Void};
use v8_base::{
  emulated_virtual_address_subspace::EmulatedVirtualAddressSubspace,
  virtual_address_space::{NO_HINT, PagePermissions, RootVirtualAddressSpace, VirtualAddressSpace},
};

/// 原版 GTEST_FLAG_GET(random_seed) 的确定性替身 (todo 7.4: 测试禁随机)
const SEED: i64 = 0xC0FF_EE01;

const KB: usize = 1024;
const MB: usize = KB * 1024;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// cc:20-27 TestRandomPageAddressGeneration
fn test_random_page_address_generation(space: &dyn VirtualAddressSpace) {
  space.set_random_seed(SEED);
  for _ in 0..10 {
    let addr = space.random_page_address();
    assert!(addr >= space.base(), "addr {addr:#x} >= base");
    assert!(
      addr < space.base() + space.size(),
      "addr {addr:#x} < base+size"
    );
  }
}

/// cc:29-64 TestBasicPageAllocation
fn test_basic_page_allocation(space: &dyn VirtualAddressSpace) {
  // KB 计的分配尺寸 (原版 allocation_sizes)
  const SIZES_KB: [usize; 21] = [
    4, 8, 12, 16, 32, 64, 128, 256, 512, 768, 1024, 768, 512, 256, 128, 64, 32, 16, 12, 8, 4,
  ];

  let mut allocations: Vec<(usize, usize)> = Vec::new();
  let alignment = space.allocation_granularity();
  for &kb in SIZES_KB.iter() {
    let size = kb * KB;
    if !size.is_multiple_of(space.allocation_granularity()) {
      continue;
    }
    let allocation = space
      .allocate_pages(NO_HINT, size, alignment, PagePermissions::READ_WRITE)
      .unwrap_or_else(|| panic!("AllocatePages({size:#x}) 失败"));
    assert!(allocation >= space.base());
    assert!(allocation < space.base() + space.size());

    allocations.push((allocation, size));

    // 内存必须可写 (原版 *reinterpret_cast<size_t*>(allocation) = size)
    // SAFETY: allocate_pages 以 READ_WRITE 提交了 [allocation, allocation+size),
    // size ≥ 4KB ≥ size_of::<usize>, 写入在映射区内
    unsafe {
      (allocation as *mut usize).write(size);
    }
  }

  // ≥64KB 档应全部取得 (Windows 64KB / macOS 16KB 粒度下的下界)
  assert!(
    allocations.len() >= 11,
    "仅 {} 次分配成功",
    allocations.len()
  );

  for (allocation, size) in allocations {
    // 回读 (原版 ... and readable)
    // SAFETY: 同上, 映射区内读回刚写入的 usize
    let read_back = unsafe { (allocation as *const usize).read() };
    assert_eq!(read_back, size, "回读失败: {allocation:#x}");
    space.free_pages(allocation, size);
  }
}

/// cc:66-84 TestPageAllocationAlignment
fn test_page_allocation_alignment(space: &dyn VirtualAddressSpace) {
  let size = space.allocation_granularity();
  for mult in [1usize, 2, 4, 8, 16, 32, 64] {
    let alignment = mult * space.allocation_granularity();
    let allocation = space
      .allocate_pages(NO_HINT, size, alignment, PagePermissions::READ_WRITE)
      .unwrap_or_else(|| panic!("AllocatePages(alignment={alignment:#x}) 失败"));
    assert_eq!(allocation % alignment, 0, "未按 {alignment:#x} 对齐");
    assert!(allocation >= space.base());
    assert!(allocation < space.base() + space.size());
    space.free_pages(allocation, size);
  }
}

/// cc:86-104 TestParentSpaceCannotAllocateInChildSpace
fn test_parent_space_cannot_allocate_in_child_space(
  parent: &dyn VirtualAddressSpace,
  child: &dyn VirtualAddressSpace,
) {
  child.set_random_seed(SEED);

  let chunksize = parent.allocation_granularity();
  let alignment = chunksize;
  let start = child.base();
  let end = start + child.size();

  for _ in 0..10 {
    let hint = child.random_page_address();
    let allocation = parent
      .allocate_pages(hint, chunksize, alignment, PagePermissions::NO_ACCESS)
      .unwrap_or_else(|| panic!("父空间 AllocatePages(hint={hint:#x}) 失败"));
    assert!(
      allocation < start || allocation >= end,
      "父空间分配 {allocation:#x} 落入子空间 [{start:#x},{end:#x})"
    );
    parent.free_pages(allocation, chunksize);
  }
}

/// VirtualAddressSpaceTest.TestPagePermissionSubsets (cc:134-170)
#[test]
fn test_page_permission_subsets() -> Void {
  use PagePermissions as PP;
  let is_subset = |a: PP, b: PP| a.is_subset_of(b);

  assert!(is_subset(PP::NO_ACCESS, PP::NO_ACCESS));
  assert!(!is_subset(PP::READ, PP::NO_ACCESS));
  assert!(!is_subset(PP::READ_WRITE, PP::NO_ACCESS));
  assert!(!is_subset(PP::READ_WRITE_EXECUTE, PP::NO_ACCESS));
  assert!(!is_subset(PP::READ_EXECUTE, PP::NO_ACCESS));

  assert!(is_subset(PP::NO_ACCESS, PP::READ));
  assert!(is_subset(PP::READ, PP::READ));
  assert!(!is_subset(PP::READ_WRITE, PP::READ));
  assert!(!is_subset(PP::READ_WRITE_EXECUTE, PP::READ));
  assert!(!is_subset(PP::READ_EXECUTE, PP::READ));

  assert!(is_subset(PP::NO_ACCESS, PP::READ_WRITE));
  assert!(is_subset(PP::READ, PP::READ_WRITE));
  assert!(is_subset(PP::READ_WRITE, PP::READ_WRITE));
  assert!(!is_subset(PP::READ_WRITE_EXECUTE, PP::READ_WRITE));
  assert!(!is_subset(PP::READ_EXECUTE, PP::READ_WRITE));

  assert!(is_subset(PP::NO_ACCESS, PP::READ_WRITE_EXECUTE));
  assert!(is_subset(PP::READ, PP::READ_WRITE_EXECUTE));
  assert!(is_subset(PP::READ_WRITE, PP::READ_WRITE_EXECUTE));
  assert!(is_subset(PP::READ_WRITE_EXECUTE, PP::READ_WRITE_EXECUTE));
  assert!(is_subset(PP::READ_EXECUTE, PP::READ_WRITE_EXECUTE));

  assert!(is_subset(PP::NO_ACCESS, PP::READ_EXECUTE));
  assert!(is_subset(PP::READ, PP::READ_EXECUTE));
  assert!(!is_subset(PP::READ_WRITE, PP::READ_EXECUTE));
  assert!(!is_subset(PP::READ_WRITE_EXECUTE, PP::READ_EXECUTE));
  assert!(is_subset(PP::READ_EXECUTE, PP::READ_EXECUTE));
  OK
}

/// VirtualAddressSpaceTest.TestRootSpace (cc:172-179)
#[test]
fn test_root_space() -> Void {
  let rootspace = RootVirtualAddressSpace::new();

  test_random_page_address_generation(&rootspace);
  test_basic_page_allocation(&rootspace);
  test_page_allocation_alignment(&rootspace);
  // TestSharedPageAllocation 跳过: SharedMemory 族未迁移 (module doc 登记)
  OK
}

/// VirtualAddressSpaceTest.TestSubspace (cc:181-219)
#[test]
fn test_subspace() -> Void {
  const SUBSPACE_SIZE: usize = 32 * MB;
  const SUB_SUBSPACE_SIZE: usize = 16 * MB;

  let rootspace = RootVirtualAddressSpace::new();
  if !rootspace.can_allocate_subspaces() {
    return OK;
  }
  let subspace_alignment = rootspace.allocation_granularity();
  let subspace = rootspace
    .allocate_subspace(
      NO_HINT,
      SUBSPACE_SIZE,
      subspace_alignment,
      PagePermissions::READ_WRITE,
    )
    .expect("AllocateSubspace(32MB) 失败");
  assert_ne!(subspace.base(), 0);
  assert_eq!(subspace.size(), SUBSPACE_SIZE);
  assert_eq!(subspace.max_page_permissions(), PagePermissions::READ_WRITE);

  test_random_page_address_generation(subspace.as_ref());
  test_basic_page_allocation(subspace.as_ref());
  test_page_allocation_alignment(subspace.as_ref());
  test_parent_space_cannot_allocate_in_child_space(&rootspace, subspace.as_ref());
  // TestSharedPageAllocation 跳过: 见 module doc

  // 子子空间 (cc:203-218)
  if !subspace.can_allocate_subspaces() {
    return OK;
  }
  let subsubspace_alignment = subspace.allocation_granularity();
  let subsubspace = subspace
    .allocate_subspace(
      NO_HINT,
      SUB_SUBSPACE_SIZE,
      subsubspace_alignment,
      PagePermissions::READ_WRITE,
    )
    .expect("AllocateSubspace(16MB 子子空间) 失败");
  assert_ne!(subsubspace.base(), 0);
  assert_eq!(subsubspace.size(), SUB_SUBSPACE_SIZE);
  assert_eq!(
    subsubspace.max_page_permissions(),
    PagePermissions::READ_WRITE
  );

  test_random_page_address_generation(subsubspace.as_ref());
  test_basic_page_allocation(subsubspace.as_ref());
  test_page_allocation_alignment(subsubspace.as_ref());
  test_parent_space_cannot_allocate_in_child_space(subspace.as_ref(), subsubspace.as_ref());
  OK
}

/// VirtualAddressSpaceTest.TestEmulatedSubspace (cc:221-267)
#[test]
fn test_emulated_subspace() -> Void {
  const SUBSPACE_SIZE: usize = 32 * MB;
  // 让页分配测试既落在背书映射区也落在未映射区 (原版注释)
  const SUBSPACE_MAPPED_SIZE: usize = MB;

  let rootspace = RootVirtualAddressSpace::new();

  let subspace_alignment = rootspace.allocation_granularity();
  assert_eq!(SUBSPACE_MAPPED_SIZE % rootspace.allocation_granularity(), 0);
  let mut reservation: Option<usize> = None;
  for _ in 0..10 {
    // 先按整尺寸在随机地址预留再释放, 确保最终预留后方有足够空闲 (原版注释)
    let hint = rootspace.random_page_address();
    let full = rootspace
      .allocate_pages(
        hint,
        SUBSPACE_SIZE,
        rootspace.allocation_granularity(),
        PagePermissions::NO_ACCESS,
      )
      .expect("整尺寸预留失败");
    let hint = full;
    rootspace.free_pages(full, SUBSPACE_SIZE);
    let mapped = rootspace
      .allocate_pages(
        hint,
        SUBSPACE_MAPPED_SIZE,
        subspace_alignment,
        PagePermissions::NO_ACCESS,
      )
      .expect("背书映射预留失败");
    if mapped == hint {
      reservation = Some(mapped);
      break;
    }
    rootspace.free_pages(mapped, SUBSPACE_MAPPED_SIZE);
    reservation = None;
  }
  let reservation = reservation.expect("10 次尝试均未在 hint 处获得背书映射");

  // SAFETY: rootspace 为栈上实例, 存活期覆盖 subspace 整个生命周期 (原版同款构造)
  let subspace = unsafe {
    EmulatedVirtualAddressSubspace::new(
      &rootspace,
      reservation,
      SUBSPACE_MAPPED_SIZE,
      SUBSPACE_SIZE,
    )
  }
  .expect("EmulatedVirtualAddressSubspace 构造失败");
  assert_eq!(reservation, subspace.base());
  assert_eq!(SUBSPACE_SIZE, subspace.size());
  assert_eq!(
    rootspace.max_page_permissions(),
    subspace.max_page_permissions()
  );

  test_random_page_address_generation(&subspace);
  test_basic_page_allocation(&subspace);
  test_page_allocation_alignment(&subspace);
  // 模拟子空间不保证父空间无法在其内分配, 无 ParentCannotAllocate 断言 (cc:264-265)
  OK
}
