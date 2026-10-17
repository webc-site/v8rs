// Sandbox 测试: 1:1 对应 v8/test/unittests/sandbox/sandbox-unittest.cc
// (初始化 / 指定尺寸 / 部分预留 / 边界谓词 / 页分配 / 常量池)

use aok::{OK, Void};
use v8_base_platform::{MemoryPermission, OS};
use v8_sandbox::{
  Address, InitConfig, K_SANDBOX_ADDRESS_SPACE_NAME, K_SANDBOX_GUARD_REGION_SIZE, K_SANDBOX_SIZE,
  Sandbox, determine_address_space_limit,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// InitializationWithSize (unittest: 8GB / 无 guard)
#[test]
fn initialization_with_size() -> Void {
  let mut sandbox = Sandbox::new();
  assert!(!sandbox.is_initialized());
  assert!(!sandbox.is_partially_reserved());
  assert_eq!(sandbox.size, 0);

  sandbox
    .initialize_with_size(InitConfig::default(), 8 * 1024 * 1024 * 1024, false)
    .expect("sandbox io");
  assert!(sandbox.is_initialized());
  assert!(!sandbox.is_partially_reserved());
  assert_eq!(sandbox.size, 8 * 1024 * 1024 * 1024);

  sandbox.tear_down().expect("sandbox io");
  assert!(!sandbox.is_initialized());
  assert_eq!(sandbox.size, 0);
  OK
}

/// PartiallyReservedSandbox (unittest: 全尺寸 1GB, 实际仅预留 2 页粒度)
#[test]
fn partially_reserved_sandbox() -> Void {
  let granularity = OS::commit_page_size();
  let size = 1 << 30; // 1GB
  let reserved_size = 2 * granularity;
  let mut sandbox = Sandbox::new();
  sandbox
    .initialize_as_partially_reserved_sandbox(InitConfig::default(), size, reserved_size)
    .expect("sandbox io");

  assert!(sandbox.is_initialized());
  assert!(sandbox.is_partially_reserved());
  assert_ne!(sandbox.base, 0);
  assert_eq!(sandbox.size, size);
  assert_eq!(sandbox.reservation_size, reserved_size);

  assert!(!sandbox.reservation_contains(sandbox.base - 1));
  assert!(sandbox.reservation_contains(sandbox.base));
  assert!(sandbox.reservation_contains(sandbox.base + reserved_size - 1));
  assert!(!sandbox.reservation_contains(sandbox.base + reserved_size));

  sandbox.tear_down().expect("sandbox io");
  assert!(!sandbox.is_initialized());
  OK
}

/// Contains (unittest 边界谓词, 全预留沙箱含 guard)
#[test]
fn contains_boundaries() -> Void {
  let mut sandbox = Sandbox::new();
  sandbox
    .initialize_with_size(InitConfig::default(), 1 << 30, true)
    .expect("sandbox io");
  if sandbox.is_partially_reserved() {
    // 无全量沙箱时跳过 (对应 unittest 的 fallback 分支; 静态允许回退)
    let _ = sandbox.tear_down();
    return OK;
  }

  let base = sandbox.base;
  let size = sandbox.size;
  assert!(sandbox.contains(base));
  assert!(sandbox.contains(base + size - 1));
  assert!(!sandbox.contains(base - 1));
  assert!(!sandbox.contains(base + size));

  // ReservationContains 计入 guard 区
  assert!(sandbox.reservation_contains(base));
  assert!(sandbox.reservation_contains(base + size - 1));
  assert!(sandbox.reservation_contains(base - 1));
  assert!(sandbox.reservation_contains(base.wrapping_sub(K_SANDBOX_GUARD_REGION_SIZE)));
  assert!(sandbox.reservation_contains(base + size));
  assert!(
    !sandbox.reservation_contains(
      base
        .wrapping_sub(K_SANDBOX_GUARD_REGION_SIZE)
        .wrapping_sub(1)
    )
  );

  sandbox.tear_down().expect("sandbox io");
  OK
}

/// PageAllocation (unittest: 沙箱子空间页分配/释放)
#[test]
fn page_allocation() -> Void {
  let mut sandbox = Sandbox::new();
  sandbox
    .initialize_with_size(InitConfig::default(), 1 << 30, false)
    .expect("sandbox io");
  let space = sandbox.address_space().expect("sandbox io");
  let granularity = space.allocation_granularity();

  let sizes_in_pages = [1usize, 1, 2, 3, 5, 8, 13, 21, 34];
  let mut allocations = Vec::new();
  for &pages in &sizes_in_pages {
    let length = granularity * pages;
    let ptr = space
      .allocate_pages(0, length, granularity, MemoryPermission::NoAccess)
      .expect("sandbox io");
    assert!(sandbox.contains(ptr), "分配必须落在沙箱内");
    allocations.push((ptr, length));
  }
  // 分配互不重叠
  let mut sorted: Vec<Address> = allocations.iter().map(|&(p, _)| p).collect();
  sorted.sort_unstable();
  sorted.dedup();
  assert_eq!(sorted.len(), allocations.len());

  for (ptr, length) in allocations {
    space.free_pages(ptr, length).expect("sandbox io");
  }
  sandbox.tear_down().expect("sandbox io");
  OK
}

/// 常量池: 空 backing store = 沙箱末字节 (末页 guard 内)
#[test]
fn constants_empty_backing_store() -> Void {
  let mut sandbox = Sandbox::new();
  sandbox
    .initialize_with_size(InitConfig::default(), 1 << 30, true)
    .expect("sandbox io");
  assert_eq!(
    sandbox.constants.empty_backing_store_buffer,
    sandbox.end - 1
  );
  assert!(sandbox.contains(sandbox.constants.empty_backing_store_buffer));
  sandbox.tear_down().expect("sandbox io");
  assert_eq!(
    sandbox.constants.empty_backing_store_buffer, 0,
    "teardown 复位常量"
  );
  OK
}

/// 默认初始化流 (sandbox.cc Initialize): 1TB + guard 或回退部分预留
#[test]
fn default_initialize_flow() -> Void {
  assert_eq!(K_SANDBOX_SIZE, 1 << 40);
  assert_eq!(K_SANDBOX_ADDRESS_SPACE_NAME, "v8-sandbox");
  let limit = determine_address_space_limit();
  assert!(limit.is_power_of_two());
  assert!(limit >= 1 << 36, "地址空间下限 64GB");
  assert!(K_SANDBOX_SIZE < limit, "沙箱必须小于进程地址空间");

  let mut sandbox = Sandbox::new();
  sandbox
    .initialize(InitConfig::default())
    .expect("sandbox io");
  assert!(sandbox.is_initialized());
  if !sandbox.is_partially_reserved() {
    assert_eq!(sandbox.size, K_SANDBOX_SIZE);
    // 全预留: 预留含前后 guard
    assert_eq!(
      sandbox.reservation_size,
      K_SANDBOX_SIZE + 2 * K_SANDBOX_GUARD_REGION_SIZE
    );
  } else {
    assert!(sandbox.reservation_size < K_SANDBOX_SIZE);
    assert!(sandbox.reservation_size >= v8_sandbox::K_SANDBOX_MINIMUM_RESERVATION_SIZE);
  }
  sandbox.tear_down().expect("sandbox io");
  OK
}

/// Smi 范围可访问性判定 (FinishInitialization 的平台零段检查)
#[test]
fn smi_address_range_flag() -> Void {
  let mut sandbox = Sandbox::new();
  sandbox
    .initialize_with_size(
      InitConfig {
        zero_segment_size: 0,
      },
      1 << 26,
      false,
    )
    .expect("sandbox io");
  assert!(!sandbox.smi_address_range_reserved);
  sandbox.tear_down().expect("sandbox io");

  let mut sandbox = Sandbox::new();
  sandbox
    .initialize_with_size(
      InitConfig {
        zero_segment_size: 4 * 1024 * 1024 * 1024 + 4 * 1024,
      },
      1 << 26,
      false,
    )
    .expect("sandbox io");
  assert!(sandbox.smi_address_range_reserved);
  sandbox.tear_down().expect("sandbox io");
  OK
}

/// 边界谓词 (sandbox.h OutsideSandbox / InsideSandbox): 未初始化时恒 outside
#[test]
fn boundary_predicates() -> Void {
  assert!(v8_sandbox::outside_sandbox(0x1234));
  assert!(!v8_sandbox::inside_sandbox(0x1234));
  OK
}
