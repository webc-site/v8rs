//! 原版 v8/test/unittests/common/thread-isolation-unittest.cc 1:1 转写
//!
//! 运行提示: JitPage 注册表 (JIT_PAGES / INITIALIZED / PKEY / ALLOCATOR) 为
//! 进程级静态全局, 仅限 nextest 进程隔离运行 (根 ./test.sh, 每用例独立进程);
//! 裸 cargo test 同进程并行会竞争注册表, 断言互相踩踏。
//!
//! 断言规约清单 (先规约后断言, 强度守恒):
//! - ReuseJitPage: 相邻页注册/注销/重注册/大区间再注册全链成功
//! - CatchJitPageOverlap: 重叠注册必须失败 (原 EXPECT_DEATH → Err(RegionOverlap))
//! - JitAllocation: 页末/跨页分配注册, 注销页后跨页分配随之释放, 内存可复用
//! - CatchOOBJitAllocation: 页外分配注册必须失败 (原 EXPECT_DEATH → Err)
//! - MergeJitPages: 相邻页双向合并, 大分配覆盖合并区, 中间页注销后再合并
//! - FreeRange: 空释放不删项 / 全量释放清空页 / 重复释放不崩
//! - InvalidFreeRange: 部分释放必须失败 (原 EXPECT_DEATH → Err(SplitFreeRange))
//!
//! EXPECT_DEATH_IF_SUPPORTED 语义: 原版 CHECK 失败即进程终止; 本 crate 禁
//! panic, 对应 API 落 Result — 断言等价改写为 assert!(result.is_err()),
//! 强度守恒 (登记见 README 映射表)。

use aok::{OK, Void};
use v8_common::{Error, ThreadIsolation, WritableJitPage};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

const BASE: usize = 0x4100_0000;
const PAGE: usize = 0x1000;

/// 原版 InitializeThreadIsolation (unittest:11-16) — 无 pkey 分配器,
/// force_memory_protection_keys = false
fn init() -> Void {
  ThreadIsolation::initialize_for_tests(None)?;
  OK
}

/// TEST(ThreadIsolation, ReuseJitPage)
#[test]
fn thread_isolation_reuse_jit_page() -> Void {
  init()?;

  let address1 = BASE;
  let address2 = address1 + PAGE;
  ThreadIsolation::register_jit_page(address1, PAGE)?;
  ThreadIsolation::register_jit_page(address2, PAGE)?;

  ThreadIsolation::unregister_jit_page(address1, PAGE)?;
  ThreadIsolation::register_jit_page(address1, PAGE)?;
  ThreadIsolation::unregister_jit_page(address1, PAGE)?;
  ThreadIsolation::unregister_jit_page(address2, PAGE)?;

  ThreadIsolation::register_jit_page(address1, 2 * PAGE)?;
  ThreadIsolation::unregister_jit_page(address1, 2 * PAGE)?;
  OK
}

/// TEST(ThreadIsolation, CatchJitPageOverlap)
#[test]
fn thread_isolation_catch_jit_page_overlap() -> Void {
  init()?;

  let address1 = BASE;
  ThreadIsolation::register_jit_page(address1, PAGE)?;
  // 原版 EXPECT_DEATH_IF_SUPPORTED({ RegisterJitPage(address1 + size - 1, 1); }, "")
  let overlap = ThreadIsolation::register_jit_page(address1 + PAGE - 1, 1);
  assert_eq!(
    overlap,
    Err(Error::RegionOverlap {
      addr: address1 + PAGE - 1,
      size: 1
    })
  );
  ThreadIsolation::unregister_jit_page(address1, PAGE)?;
  OK
}

/// TEST(ThreadIsolation, JitAllocation)
#[test]
fn thread_isolation_jit_allocation() -> Void {
  init()?;

  let address1 = BASE;
  let address2 = address1 + PAGE;
  ThreadIsolation::register_jit_page(address1, PAGE)?;
  ThreadIsolation::register_jit_page(address2, PAGE)?;

  ThreadIsolation::register_jit_allocation_for_testing(address2 + PAGE - 1, 1)?;
  ThreadIsolation::register_jit_allocation_for_testing(address1, 1)?;
  // 跨页分配
  ThreadIsolation::register_jit_allocation_for_testing(address2 - 1, 2)?;

  ThreadIsolation::unregister_jit_page(address1, PAGE)?;
  // 跨页分配已随页释放, 内存可复用
  ThreadIsolation::register_jit_allocation_for_testing(address2, 1)?;
  ThreadIsolation::unregister_jit_page(address2, PAGE)?;
  OK
}

/// TEST(ThreadIsolation, CatchOOBJitAllocation)
#[test]
fn thread_isolation_catch_oob_jit_allocation() -> Void {
  init()?;

  let address1 = BASE;
  ThreadIsolation::register_jit_page(address1, PAGE)?;
  // 原版 EXPECT_DEATH_IF_SUPPORTED({ RegisterJitAllocationForTesting(address1 + size, 1); }, "")
  // 精确错误: 页覆盖 [address1, address1+PAGE) 不含 address1+PAGE → JitPageNotFound
  let oob = ThreadIsolation::register_jit_allocation_for_testing(address1 + PAGE, 1);
  assert_eq!(
    oob,
    Err(Error::JitPageNotFound {
      addr: address1 + PAGE,
      size: 1
    })
  );
  ThreadIsolation::unregister_jit_page(address1, PAGE)?;
  OK
}

/// TEST(ThreadIsolation, MergeJitPages)
#[test]
fn thread_isolation_merge_jit_pages() -> Void {
  init()?;

  let address1 = BASE;
  let address2 = address1 + PAGE;
  let address3 = address2 + PAGE;

  ThreadIsolation::register_jit_page(address2, PAGE)?;
  ThreadIsolation::register_jit_page(address1, PAGE)?;
  ThreadIsolation::register_jit_page(address3, PAGE)?;

  ThreadIsolation::register_jit_allocation_for_testing(address1, 3 * PAGE)?;
  ThreadIsolation::unregister_jit_allocation_for_testing(address1, 3 * PAGE)?;

  // 双向合并: 注销中间页后重注册
  ThreadIsolation::unregister_jit_page(address2, PAGE)?;
  ThreadIsolation::register_jit_page(address2, PAGE)?;

  ThreadIsolation::register_jit_allocation_for_testing(address1, 3 * PAGE)?;
  ThreadIsolation::unregister_jit_allocation_for_testing(address1, 3 * PAGE)?;

  ThreadIsolation::unregister_jit_page(address2, PAGE)?;
  ThreadIsolation::unregister_jit_page(address1, PAGE)?;
  ThreadIsolation::unregister_jit_page(address3, PAGE)?;
  OK
}

/// TEST(ThreadIsolation, FreeRange)
#[test]
fn thread_isolation_free_range() -> Void {
  init()?;

  let address1 = BASE;
  let address2 = address1 + PAGE;
  let address3 = address2 + PAGE;
  ThreadIsolation::register_jit_page(address1, PAGE)?;
  ThreadIsolation::register_jit_page(address2, PAGE)?;
  ThreadIsolation::register_jit_page(address3, PAGE)?;

  ThreadIsolation::register_jit_allocation_for_testing(address2 - 1, 1)?;
  ThreadIsolation::register_jit_allocation_for_testing(address2, 1)?;
  ThreadIsolation::register_jit_allocation_for_testing(address2 + 1, PAGE - 2)?;
  ThreadIsolation::register_jit_allocation_for_testing(address3 - 1, 1)?;
  ThreadIsolation::register_jit_allocation_for_testing(address3, 1)?;

  {
    let mut jit_page = WritableJitPage::new(address2, PAGE)?;
    assert!(!jit_page.empty());
    jit_page.free_range(address2, 0)?;
    assert!(!jit_page.empty());
    jit_page.free_range(address2, PAGE)?;
    assert!(jit_page.empty());
    // 重复释放已空闲区间不崩
    jit_page.free_range(address2, PAGE)?;
  }
  {
    let mut jit_page = WritableJitPage::new(address1, PAGE)?;
    assert!(!jit_page.empty());
    jit_page.free_range(address1, PAGE)?;
    assert!(jit_page.empty());
  }
  {
    let mut jit_page = WritableJitPage::new(address3, PAGE)?;
    assert!(!jit_page.empty());
    jit_page.free_range(address3, PAGE)?;
    assert!(jit_page.empty());
  }

  ThreadIsolation::unregister_jit_page(address1, PAGE)?;
  ThreadIsolation::unregister_jit_page(address2, PAGE)?;
  ThreadIsolation::unregister_jit_page(address3, PAGE)?;
  OK
}

/// TEST(ThreadIsolation, InvalidFreeRange)
#[test]
fn thread_isolation_invalid_free_range() -> Void {
  init()?;

  let address1 = BASE;
  ThreadIsolation::register_jit_page(address1, PAGE)?;

  ThreadIsolation::register_jit_allocation_for_testing(address1, 2)?;

  {
    let mut jit_page = WritableJitPage::new(address1, PAGE)?;
    assert!(!jit_page.empty());
    // 原版 EXPECT_DEATH_IF_SUPPORTED({ jit_page.FreeRange(address1, 1); }, "") —
    // 部分释放切进分配中间必须失败; 精确错误: 分配 [address1, +2) 跨 range_end
    let partial = jit_page.free_range(address1, 1);
    assert_eq!(
      partial.map(|_| ()),
      Err(Error::SplitFreeRange {
        addr: address1,
        size: 1
      })
    );
    jit_page.free_range(address1, 2)?;
  }

  ThreadIsolation::unregister_jit_page(address1, PAGE)?;
  OK
}
