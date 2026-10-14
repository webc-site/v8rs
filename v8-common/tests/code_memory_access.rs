//! 原版 v8/src/common/code-memory-access.{h,-inl.h,cc} 模块级补强校验
//! (上游 thread-isolation-unittest.cc 的 7 用例已 1:1 转写于 tests/thread_isolation.rs,
//! 本文件覆盖上游未及的拆分/查找/写通道/作用域路径)
//!
//! 断言规约清单 (先规约后断言, 强度守恒):
//! - jit_allocation_type_discriminants: 5 变体判别值 0..=4 (h:143-149)
//! - k_split_threshold_value: 0x40000 (cc:544)
//! - fill_page_sz: 对齐宏在 PKU 配置下 0x1000-回绕取整, 非 PKU 恒 0 (h:55-64)
//! - register_jit_allocations_splits_page: 总量达阈值走 SplitJitPage,
//!   head/mid/tail 三段边界可探测 (cc:534-553, cc:578-604)
//! - [round5] register_jit_allocations_below_threshold_no_split: 总量低于
//!   阈值走 LookupJitPage 不拆页, 切入既有分配报 RegionOverlap 非
//!   SplitFreeRange (cc:544-547 分支判别)
//! - [K6 回归锁] split_error_keeps_registry_consistent: 切口落在分配中间时
//!   Err(SplitFreeRange) 后, 原分配仍可经地址查询 — 原版 CHECK 即时终止进程
//!   无可观测残留; Rust 落 Result 继续运行, 不得静默丢注册项
//! - start_of_jit_allocation_at: 命中 Some / 间隙 None / 未注册 None
//!   (cc:622-630, CHECK→None+error 日志 登记)
//! - can_lookup_start_of_jit_allocation_at: 上界页命中 try_lock 真; 低于全部
//!   注册页假; 页内含不校验 (原版 cc:681-686 语义); 持页引用锁定期假
//!   (原版仅页锁不持全局锁 — 跨页差异见 K5)
//! - non_executable_write_channel: ForNonExecutableMemory 形态
//!   write_value/write_unaligned_value/copy_code/clear_bytes/write_header_slot
//!   对真实缓冲逐字节强校验 (h:404-492, -inl.h:147-278)
//! - writable_free_space: for_non_executable_memory 元数据 + clear_tagged 清零
//!   (h:499-529, cc:756-762)
//! - jump_table_pair_lookup: 双页两分配 lookup Ok + 元数据; 未注册 → Err;
//!   重叠 → Err (cc:716-721, -inl.h:115-143)
//! - scope_smoke: RwxMemoryWriteScope::is_supported / Nop / CFI / ForTesting /
//!   DiscardSealed 构造 (h:247-259, 615-650)
//! - [K5 回归锁] cross_page_registration_parallel: 持 WritableJitPage (页 A)
//!   期间, 他线程注册不相交页 B 须在原版语义下并行完成 — 原版 RegisterJitPage
//!   仅取全局 pages 锁、WritableJitPage 仅持 A 页锁 (h:294, cc:446);
//! - [回归 round2-F3] unregister_allocations_except_empty_range_no_panic:
//!   全部分配在 range_end 之上时空区间 no-op 不 panic (原版 keep_after ≥
//!   keep_before 恒序 cc:391-409; Rust BTreeMap range 起点大于终点无条件 panic)
//! - [round2-F4 锁行为] unregister_allocations_except_right_slice_rejected:
//!   区间内分配跨过 range_end → Err(SplitFreeRange) 且注册表零副作用
//!   (登记偏差: 原版静默弃注册)
//! - unregister_range_left_slice_rejected: 左侧切口落进分配中间 →
//!   Err(SplitFreeRange) 注册表不动 (登记偏差: 原版不检左切, cc:373-384)
//! - start_of_allocation_exact_bounds: 起始/中间/末字节命中, 越界与空隙未命中
//! - [回归 round2-F2] initialize_forced_pkey_non_pku_platform: 非 pku 平台
//!   force_memory_protection_keys 下提供分配器即初始化成功 (原版检查包
//!   #if V8_HAS_PKU_JIT_WRITE_PROTECT, cc:95-104, 非 pku 不评估)
//! - [回归 round2-F3] expand_then_register_overlap_rejected: expand 扩页后
//!   (契约内串行) 注册扩张区间 → Err(RegionOverlap) — 重叠检查必须观察到
//!   扩张后尺寸; expand 与 register 并发竞争由契约排除 (见 expand 文档)
//! - [round2] jump_table_lookup_contains_mismatch: 跳表已注册但类型/尺寸
//!   不符 → split 成功后 Contains 判假 → Err(JitAllocationMismatch)
//!   (原版 -inl.h:131-136 CHECK)
//! - [round3] registry_concurrent_stress: 多线程 register/unregister/bulk-split/
//!   except 交错 (固定 seed) + stable 页并发读 + 收尾注册表清空不变式
//! - [回归 round4-F8] RwxMemoryWriteScope !Send + !Sync 编译期锁
//!   (code-memory-access.h:99-100 不可移动 / h:72-89 权限仅作用构造线程)

use core::ptr::NonNull;
use std::{sync::mpsc, thread, thread::scope, time::Duration};

use aok::{OK, Void};
use fastrand::Rng;
use v8_common::{
  CFIMetadataWriteScope, DiscardSealedMemoryScope, Error, JitAllocationType, K_SPLIT_THRESHOLD,
  NopRwxMemoryWriteScope, RwxMemoryWriteScope, RwxMemoryWriteScopeForTesting,
  THREAD_ISOLATION_ALIGN_SZ, ThreadIsolatedAllocator, ThreadIsolation, WritableFreeSpace,
  WritableJitAllocation, WritableJitPage, set_force_memory_protection_keys,
  thread_isolation_fill_page_sz,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

const PAGE: usize = 0x1000;

// 写通道逐字节断言依赖小端目标 (x64/arm64 目标配置)
const _: () = assert!(cfg!(target_endian = "little"));

// [回归 round4-F8] RwxMemoryWriteScope 线程绑定 — 原版删除拷贝构造且
// 自声明析构抑制移动 (code-memory-access.h:99-100), 权限窗口仅作用于
// 构造线程 (h:72-89); Rust 零字段载体不得派生 Send/Sync (防跨线程 drop
// 使构造线程永久残留 SetWritable 权限)。编译期锁, 同 assert_scope F7 纪律
static_assertions::assert_not_impl_all!(v8_common::RwxMemoryWriteScope: Send, Sync);

fn init() -> Void {
  ThreadIsolation::initialize_for_tests(None)?;
  OK
}

/// JitAllocationType 判别值 (原版 enum class 顺序)
#[test]
fn code_memory_jit_allocation_type_discriminants() -> Void {
  assert_eq!(JitAllocationType::InstructionStream as u8, 0);
  assert_eq!(JitAllocationType::WasmCode as u8, 1);
  assert_eq!(JitAllocationType::WasmJumpTable as u8, 2);
  assert_eq!(JitAllocationType::WasmFarJumpTable as u8, 3);
  assert_eq!(JitAllocationType::WasmLazyCompileTable as u8, 4);
  OK
}

/// kSplitThreshold 常量 (code-memory-access.cc:544)
#[test]
fn code_memory_k_split_threshold_value() -> Void {
  assert_eq!(K_SPLIT_THRESHOLD, 0x40000);
  OK
}

/// THREAD_ISOLATION_FILL_PAGE_SZ 分支 (code-memory-access.h:55-64)
/// 注意: 原版以 V8_HAS_PKU_JIT_WRITE_PROTECT (含构建开关) 为门, Rust 仅按
/// linux-x64 target 判定 — 本用例覆盖当前 host 分支
#[test]
fn code_memory_fill_page_sz_matches_alignment_cfg() -> Void {
  if cfg!(all(target_os = "linux", target_arch = "x86_64")) {
    assert_eq!(THREAD_ISOLATION_ALIGN_SZ, 0x1000);
    assert_eq!(thread_isolation_fill_page_sz(0x1000), 0);
    assert_eq!(thread_isolation_fill_page_sz(0x123), 0x1000 - 0x123);
    assert_eq!(thread_isolation_fill_page_sz(0), 0);
  } else {
    assert_eq!(THREAD_ISOLATION_ALIGN_SZ, 0);
    for sz in [0usize, 1, 0x123, 0x1000, 0x10000] {
      assert_eq!(thread_isolation_fill_page_sz(sz), 0);
    }
  }
  OK
}

/// RegisterJitAllocations 达阈值拆页: head/mid/tail 边界 (cc:534-553)
#[test]
fn code_memory_register_jit_allocations_splits_page() -> Void {
  init()?;
  let base = 0x6000_0000usize;
  ThreadIsolation::register_jit_page(base, 0x80000)?;
  // 起点偏移 0x1000, 总量恰 0x40000 → SplitJitPages
  ThreadIsolation::register_jit_allocations(
    base + 0x1000,
    &[0x20000, 0x20000],
    JitAllocationType::WasmCode,
  )?;
  // mid 段内两条分配可查
  assert_eq!(
    ThreadIsolation::start_of_jit_allocation_at(base + 0x1008),
    Some(base + 0x1000)
  );
  assert_eq!(
    ThreadIsolation::start_of_jit_allocation_at(base + 0x21000),
    Some(base + 0x20000 + 0x1000)
  );
  // head 页 [base, base+0x1000): 页内精确注册成功
  assert_eq!(
    ThreadIsolation::register_jit_allocation(base, 0xF00, JitAllocationType::WasmCode, false)
      .map(|a| (a.address(), a.size())),
    Ok((base, 0xF00))
  );
  // 越界注册 [base+0xF00, +0x200): 原版 TryLookupJitPage 自动并入相邻 mid 页
  // (cc:196-242), 随后与 mid 首条分配 (base+0x1000) 区间重叠 → 原 CHECK 终止
  // → Rust Err(RegionOverlap)
  let over = ThreadIsolation::register_jit_allocation(
    base + 0xF00,
    0x200,
    JitAllocationType::WasmCode,
    false,
  );
  assert_eq!(
    over.err(),
    Some(Error::RegionOverlap {
      addr: base + 0xF00,
      size: 0x200,
    })
  );
  // tail 页自 base+0x41000 起: 页内可注册
  assert!(
    ThreadIsolation::register_jit_allocation(
      base + 0x41000,
      0x100,
      JitAllocationType::WasmCode,
      false
    )
    .is_ok()
  );
  // mid 起点 0x1000 处: head 与 mid 之间无空洞 (页边界恰在 0x1000)
  assert_eq!(
    ThreadIsolation::start_of_jit_allocation_at(base + 0x800),
    Some(base)
  );
  OK
}

/// RegisterJitAllocations 低于阈值走 LookupJitPage 不拆页 (cc:544-547 分支):
/// 正路径全量可查; 切口落既有分配中间时为 Err(RegionOverlap) 而非
/// SplitFreeRange — 拆页分支才会触发 K6 切口校验, 以此锁定分支选择
#[test]
fn code_memory_register_jit_allocations_below_threshold_no_split() -> Void {
  init()?;
  let base = 0x7D00_0000usize;
  ThreadIsolation::register_jit_page(base, 4 * PAGE)?;
  // 既有分配 [base+0x2000, base+0x2040)
  ThreadIsolation::register_jit_allocation(
    base + 0x2000,
    0x40,
    JitAllocationType::WasmCode,
    false,
  )?;
  // 正路径: 总量 0x200 < 0x40000 → lookup 分支, 两条衔接分配注册可查
  ThreadIsolation::register_jit_allocations(
    base + 0x100,
    &[0x100, 0x100],
    JitAllocationType::WasmCode,
  )?;
  assert_eq!(
    ThreadIsolation::start_of_jit_allocation_at(base + 0x180),
    Some(base + 0x100)
  );
  assert_eq!(
    ThreadIsolation::start_of_jit_allocation_at(base + 0x2FF),
    Some(base + 0x200)
  );
  // 分支判别: 起点 0x2010 切进既有分配 [0x2000, 0x2040) — 不拆页的
  // lookup 分支在分配重叠校验处 Err(RegionOverlap); 若误走拆页分支,
  // K6 切口校验会先行 Err(SplitFreeRange)
  let err = ThreadIsolation::register_jit_allocations(
    base + 0x2010,
    &[0x10, 0x10],
    JitAllocationType::WasmCode,
  );
  assert_eq!(
    err,
    Err(Error::RegionOverlap {
      addr: base + 0x2010,
      size: 0x10,
    })
  );
  OK
}

/// 切口落在分配中间 → Err 后注册表不得静默丢失原分配项 (K6 回归锁, 绿)。
/// 原版 SplitJitPageLocked 的 defense-in-depth CHECK 即时终止进程 (cc:594-601),
/// 无可观测残留; Rust 落 Result 后错误路径必须可观测一致 — 切口校验提前到
/// 任何注册表变更前 (K6), Err 后原分配 [0x2800 处) 仍可查。
#[test]
fn code_memory_split_error_keeps_registry_consistent() -> Void {
  init()?;
  let base = 0x6100_0000usize;
  ThreadIsolation::register_jit_page(base, 0x80000)?;
  // 分配 [base+0x2000, base+0x3000) — 注册对象须及时释放 (持注册表锁贯穿存活期)
  {
    let _alloc = ThreadIsolation::register_jit_allocation(
      base + 0x2000,
      0x1000,
      JitAllocationType::WasmCode,
      false,
    )?;
  }
  assert_eq!(
    ThreadIsolation::start_of_jit_allocation_at(base + 0x2800),
    Some(base + 0x2000)
  );
  // 总量达阈值的拆分请求, 切口 base+0x2500 落在分配中间
  let err = ThreadIsolation::register_jit_allocations(
    base + 0x2500,
    &[0x20000, 0x20000],
    JitAllocationType::WasmCode,
  );
  assert_eq!(
    err,
    Err(Error::SplitFreeRange {
      addr: base,
      size: 0x40000
    })
  );
  // Err 后原分配 [0x2800 处) 必须仍可查 (注册表一致性)
  assert_eq!(
    ThreadIsolation::start_of_jit_allocation_at(base + 0x2800),
    Some(base + 0x2000)
  );
  OK
}

/// StartOfJitAllocationAt 命中/间隙/未注册 (cc:622-630)
#[test]
fn code_memory_start_of_jit_allocation_at_paths() -> Void {
  init()?;
  let base = 0x6200_0000usize;
  ThreadIsolation::register_jit_page(base, PAGE)?;
  ThreadIsolation::register_jit_allocation(
    base + 0x10,
    0x20,
    JitAllocationType::InstructionStream,
    false,
  )?;
  assert_eq!(
    ThreadIsolation::start_of_jit_allocation_at(base + 0x18),
    Some(base + 0x10)
  );
  // 间隙 (分配外, 页内) → 原 CHECK → None + error 日志
  assert_eq!(
    ThreadIsolation::start_of_jit_allocation_at(base + 0x40),
    None
  );
  // 未注册页 → None
  assert_eq!(
    ThreadIsolation::start_of_jit_allocation_at(base + 0x9000),
    None
  );
  OK
}

/// CanLookupStartOfJitAllocationAt try_lock 语义 (cc:667-694)
#[test]
fn code_memory_can_lookup_start_of_jit_allocation_at() -> Void {
  init()?;
  let base = 0x6300_0000usize;
  ThreadIsolation::register_jit_page(base, PAGE)?;
  assert!(ThreadIsolation::can_lookup_start_of_jit_allocation_at(base));
  // 原版语义: 上界页命中即可, 无页内含校验 (cc:681-686) — 高于所有页仍真
  assert!(ThreadIsolation::can_lookup_start_of_jit_allocation_at(
    base + 0x8000
  ));
  // 低于全部注册页 → 假 (cc:679-681 begin 分支)
  assert!(!ThreadIsolation::can_lookup_start_of_jit_allocation_at(
    base - 0x1000
  ));
  // 持整页写通道 (Rust: 全局注册表锁贯穿) → try_lock 假
  let wp = WritableJitPage::new(base, PAGE)?;
  assert!(!ThreadIsolation::can_lookup_start_of_jit_allocation_at(
    base
  ));
  drop(wp);
  assert!(ThreadIsolation::can_lookup_start_of_jit_allocation_at(base));
  OK
}

/// ForNonExecutableMemory 写通道对真实缓冲的逐字节强校验
#[test]
fn code_memory_non_executable_write_channel() -> Void {
  init()?;
  let mut buf: Vec<u64> = vec![0; 8];
  let addr = buf.as_mut_ptr() as usize;
  let alloc =
    WritableJitAllocation::for_non_executable_memory(addr, 64, JitAllocationType::WasmCode);
  assert_eq!(alloc.address(), addr);
  assert_eq!(alloc.size(), 64);

  alloc.write_value(addr + 8, 0x1122_3344_5566_7788u64);
  assert_eq!(buf[1], 0x1122_3344_5566_7788);

  alloc.write_unaligned_value(addr + 16, 0xABBEu16);
  assert_eq!(buf[2] & 0xFFFF, 0xABBE);

  alloc.copy_code(24, &[1, 2, 3, 4]);
  assert_eq!(buf[3], u64::from_le_bytes([1, 2, 3, 4, 0, 0, 0, 0]));

  alloc.copy_data(28, &[5, 6]);
  assert_eq!(buf[3], u64::from_le_bytes([1, 2, 3, 4, 5, 6, 0, 0]));

  alloc.clear_bytes(24, 4);
  assert_eq!(buf[3], u64::from_le_bytes([0, 0, 0, 0, 5, 6, 0, 0]));

  alloc.write_header_slot::<_, 0>(0xDEAD_u32);
  assert_eq!(buf[0], 0xDEAD);

  // [round2-F4] offset 模板参: 写点 address + OFFSET (原版 -inl.h:147-161
  // WriteMaybeUnalignedValue(address_ + offset, value))
  alloc.write_header_slot::<_, 36>(0xBEEF_u16);
  assert_eq!(buf[4], 0x0000_BEEF_0000_0000);

  // [round2-F4] WriteValue RelaxedStoreTag 变体: 8/4/1 字节原子宽松写
  // (原版 -inl.h:249-258 std::atomic<T>::store(relaxed))
  alloc.write_value_relaxed(addr + 32, 0x99AA_BBCC_DDEE_0011u64);
  assert_eq!(buf[4], 0x99AA_BBCC_DDEE_0011);
  alloc.write_value_relaxed(addr + 40, 0x5566_7788u32);
  assert_eq!(buf[5] & u32::MAX as u64, 0x5566_7788);
  alloc.write_value_relaxed(addr + 44, 0x77u8);
  assert_eq!(buf[5] >> 32, 0x77);
  OK
}

/// WritableFreeSpace 非可执行形态 + ClearTagged 清零 (cc:756-762)
/// kTaggedSize = 4 (COMPRESS_POINTERS 目标配置)
#[test]
fn code_memory_writable_free_space() -> Void {
  init()?;
  let mut buf: Vec<u64> = vec![0xDEAD_BEEF_1122_3344; 4];
  let addr = buf.as_mut_ptr() as usize;
  let free = WritableFreeSpace::for_non_executable_memory(addr, 32);
  assert_eq!(free.address(), addr);
  assert_eq!(free.size(), 32);
  assert!(!free.executable());
  free.write_header_slot::<_, 0>(0x1234_u32);
  // 未对齐 u32 写仅覆盖低 4 字节 (高 32 位保留)
  assert_eq!(buf[0], 0xDEAD_BEEF_0000_1234);
  // 2 槽 × 4 字节 = 8 字节 (恰 buf[0] 全宽) 清零, buf[1] 不动
  free.clear_tagged::<0>(2);
  assert_eq!(buf[0], 0);
  assert_eq!(buf[1], 0xDEAD_BEEF_1122_3344);
  assert_eq!(buf[2], 0xDEAD_BEEF_1122_3344);
  // OFFSET=8 处 1 槽 → buf[1] 低半清零
  free.clear_tagged::<8>(1);
  assert_eq!(buf[1], 0xDEAD_BEEF_0000_0000);
  assert_eq!(buf[2], 0xDEAD_BEEF_1122_3344);
  OK
}

/// WritableJumpTablePair lookup 双页正路径与错误路径 (cc:716-721)
#[test]
fn code_memory_jump_table_pair_lookup() -> Void {
  init()?;
  let jt = 0x6400_1000usize;
  let far = 0x6500_1000usize;
  ThreadIsolation::register_jit_page(jt - 0x1000, 0x2000)?;
  ThreadIsolation::register_jit_page(far - 0x1000, 0x2000)?;
  ThreadIsolation::register_jit_allocation(jt, 0x40, JitAllocationType::WasmJumpTable, false)?;
  ThreadIsolation::register_jit_allocation(far, 0x40, JitAllocationType::WasmFarJumpTable, false)?;
  let pair = ThreadIsolation::lookup_jump_table_allocations(jt, 0x40, far, 0x40)?;
  assert_eq!(pair.jump_table().address(), jt);
  assert_eq!(pair.jump_table().size(), 0x40);
  assert_eq!(pair.far_jump_table().address(), far);
  assert_eq!(pair.far_jump_table().size(), 0x40);
  drop(pair);

  // 未注册分配 → Err(JitAllocationMismatch) (原 CHECK 终止 → Result, 登记)
  let jt2 = 0x6600_1000usize;
  let far2 = 0x6700_1000usize;
  ThreadIsolation::register_jit_page(jt2 - 0x1000, 0x2000)?;
  ThreadIsolation::register_jit_page(far2 - 0x1000, 0x2000)?;
  assert_eq!(
    ThreadIsolation::lookup_jump_table_allocations(jt2, 0x40, far2, 0x40).err(),
    Some(Error::JitAllocationMismatch {
      addr: jt2,
      size: 0x40
    })
  );
  // 重叠 (jt 尾 > far 头) → Err(RegionOverlap)
  let jt3 = 0x6800_1000usize;
  let far3 = 0x6800_1020usize;
  assert_eq!(
    ThreadIsolation::lookup_jump_table_allocations(jt3, 0x40, far3, 0x40).err(),
    Some(Error::RegionOverlap {
      addr: jt3,
      size: 0x40
    })
  );
  OK
}

/// 写作用域构造面 (h:247-259, 615-650)
#[test]
fn code_memory_scope_smoke() -> Void {
  init()?;
  let supported = RwxMemoryWriteScope::is_supported();
  if cfg!(all(target_os = "macos", target_arch = "aarch64")) {
    assert!(supported);
    assert!(!RwxMemoryWriteScope::is_pku_writable());
  } else if !supported {
    assert!(!RwxMemoryWriteScope::is_pku_writable());
  }
  let _s = RwxMemoryWriteScope::new("test");
  let _t = RwxMemoryWriteScopeForTesting::new("test");
  let _n = NopRwxMemoryWriteScope::new("test");
  let _c = CFIMetadataWriteScope::new("test");
  let _d = DiscardSealedMemoryScope::new("test");
  OK
}

/// K5 回归锁 (绿): 原版持 WritableJitPage(A) 只占 A 页锁 (h:294 page_lock_),
/// RegisterJitPage 只取全局 pages 锁 (cc:446) — 两操作并行。R3 页级锁落地
/// (注册表存 Arc<JitPage>, 页锁 per-page, 注册表锁仅查找/插入瞬间持有),
/// 不相交页注册须在 WritableJitPage(A) 存活期间完成。
#[test]
fn code_memory_cross_page_registration_parallel() -> Void {
  init()?;
  let a = 0x7000_0000usize;
  let b = 0x7100_0000usize;
  ThreadIsolation::register_jit_page(a, PAGE)?;

  let (held_tx, held_rx) = mpsc::channel::<()>();
  let (release_tx, release_rx) = mpsc::channel::<()>();
  let holder = thread::spawn(move || {
    let wp = WritableJitPage::new(a, PAGE).expect("writable page A");
    held_tx.send(()).unwrap();
    // 兜底: 2s 后无条件释放, 防测试失败路径悬挂
    let _ = release_rx.recv_timeout(Duration::from_secs(2));
    drop(wp);
  });
  held_rx.recv().unwrap();

  let (res_tx, res_rx) = mpsc::channel::<Result<(), Error>>();
  let registrant = thread::spawn(move || {
    let r = ThreadIsolation::register_jit_page(b, PAGE);
    let _ = res_tx.send(r);
  });

  let outcome = res_rx.recv_timeout(Duration::from_millis(250));

  // 无论断言成败先复位: 通知持有者释放, 避免他线程悬挂
  let _ = release_tx.send(());
  holder.join().unwrap();

  // 原版语义: 不相交页注册在 250ms 内完成且成功
  assert!(
    matches!(outcome, Ok(Ok(()))),
    "K5: 原版 WritableJitPage(页 A) 不阻塞他页 RegisterJitPage 并行注册, \
     Rust 全局注册表锁贯穿持有导致未完成 (outcome: {outcome:?})"
  );
  registrant.join().unwrap();
  OK
}

/// UnregisterAllocationsExcept: 确保 start 之前与 range_end 之后的分配不被误删,
/// 仅 [start, range_end) 内未在 keep 清单的分配被注销;
/// 且切口校验与清单不匹配错误不污染既有分配。
#[test]
fn code_memory_unregister_allocations_except_preserves_surrounding_and_filters() -> Void {
  init()?;
  let base = 0x7200_0000usize;
  ThreadIsolation::register_jit_page(base, 4 * PAGE)?;

  let a1 = base + 0x100; // before start
  let a2 = base + 0x1000; // in range, kept
  let a3 = base + 0x1200; // in range, dropped
  let a4 = base + 0x1400; // in range, kept
  let a5 = base + 0x2000; // after range

  ThreadIsolation::register_jit_allocation(a1, 0x40, JitAllocationType::WasmCode, false)?;
  ThreadIsolation::register_jit_allocation(a2, 0x40, JitAllocationType::WasmCode, false)?;
  ThreadIsolation::register_jit_allocation(a3, 0x40, JitAllocationType::WasmCode, false)?;
  ThreadIsolation::register_jit_allocation(a4, 0x40, JitAllocationType::WasmCode, false)?;
  ThreadIsolation::register_jit_allocation(a5, 0x40, JitAllocationType::WasmCode, false)?;

  {
    let mut page = WritableJitPage::new(base, 4 * PAGE)?;
    // 错误路径 1: keep 清单包含不存在的地址 → Err(KeepListMismatch), 注册表无副作用
    let err = page.unregister_allocations_except(base + 0x1000, 0x800, &[a2, base + 0x1300]);
    assert_eq!(err, Err(Error::KeepListMismatch));
    assert_eq!(page.page_ref().start_of_allocation_at(a3), Ok(a3));

    // 正确路径: 仅保留 a2 与 a4
    page.unregister_allocations_except(base + 0x1000, 0x800, &[a2, a4])?;
  }

  // 验证:
  // a1 在 start 之前: 必须完好保留 (原版/旧逻辑曾误删 pred)
  assert_eq!(ThreadIsolation::start_of_jit_allocation_at(a1), Some(a1));
  // a2: 在 keep 中, 保留
  assert_eq!(ThreadIsolation::start_of_jit_allocation_at(a2), Some(a2));
  // a3: 不在 keep 中, 已注销
  assert_eq!(ThreadIsolation::start_of_jit_allocation_at(a3), None);
  // a4: 在 keep 中, 保留
  assert_eq!(ThreadIsolation::start_of_jit_allocation_at(a4), Some(a4));
  // a5 在 range_end 之后: 必须完好保留
  assert_eq!(ThreadIsolation::start_of_jit_allocation_at(a5), Some(a5));

  ThreadIsolation::unregister_jit_page(base, 4 * PAGE)?;
  OK
}

/// UnregisterRange: 左侧切入分配中间必须被拒绝 (SplitFreeRange), 保证一致性
#[test]
fn code_memory_unregister_range_left_slice_rejected() -> Void {
  init()?;
  let base = 0x7300_0000usize;
  ThreadIsolation::register_jit_page(base, PAGE)?;

  let a = base + 0x100;
  ThreadIsolation::register_jit_allocation(a, 0x40, JitAllocationType::InstructionStream, false)?;

  {
    let mut page = WritableJitPage::new(base, PAGE)?;
    // 从分配内部 (base + 0x120) 开始释放 → 左侧切口落在 a 中间, 必须报错
    let err = page.free_range(base + 0x120, 0x40);
    assert_eq!(
      err.map(|_| ()),
      Err(Error::SplitFreeRange {
        addr: base + 0x120,
        size: 0x40
      })
    );
  }

  // 分配必须完好保留
  assert_eq!(
    ThreadIsolation::start_of_jit_allocation_at(a + 0x10),
    Some(a)
  );

  ThreadIsolation::unregister_jit_page(base, PAGE)?;
  OK
}

/// StartOfAllocationAt 边界点全覆盖:
/// 起始字节 / 内部 / 末字节命中; 末字节后 1 字节及空隙未命中
#[test]
fn code_memory_start_of_allocation_exact_bounds() -> Void {
  init()?;
  let base = 0x7400_0000usize;
  ThreadIsolation::register_jit_page(base, PAGE)?;

  let alloc_start = base + 0x200;
  let alloc_size = 0x50;
  ThreadIsolation::register_jit_allocation(
    alloc_start,
    alloc_size,
    JitAllocationType::InstructionStream,
    false,
  )?;

  // 起始字节
  assert_eq!(
    ThreadIsolation::start_of_jit_allocation_at(alloc_start),
    Some(alloc_start)
  );
  // 中间字节
  assert_eq!(
    ThreadIsolation::start_of_jit_allocation_at(alloc_start + 0x25),
    Some(alloc_start)
  );
  // 尾字节 (alloc_start + size - 1)
  assert_eq!(
    ThreadIsolation::start_of_jit_allocation_at(alloc_start + alloc_size - 1),
    Some(alloc_start)
  );
  // 越界 1 字节 (alloc_start + size)
  assert_eq!(
    ThreadIsolation::start_of_jit_allocation_at(alloc_start + alloc_size),
    None
  );
  // 分配前 1 字节
  assert_eq!(
    ThreadIsolation::start_of_jit_allocation_at(alloc_start - 1),
    None
  );

  ThreadIsolation::unregister_jit_page(base, PAGE)?;
  OK
}

/// [回归 round2-F3] UnregisterAllocationsExcept 空区间: 全部分配均在
/// range_end 之上时, keep_before (原版 lower_bound(start) 前移一条,
/// cc:395-398) 大于 range_end — 旧实现直接 `range(kb..range_end)` 迭代,
/// BTreeMap range 起点大于终点无条件 panic (release 亦然); 现实现必须按
/// 原版空迭代语义 no-op, 区间外分配完好保留
#[test]
fn code_memory_unregister_allocations_except_empty_range_no_panic() -> Void {
  init()?;
  let base = 0x7500_0000usize;
  ThreadIsolation::register_jit_page(base, PAGE)?;

  // 分配全部落在 [base+0x800, base+0x840) — 释放区间 [base, base+0x10)
  // 不含任何分配, keep_before = base+0x800 > range_end = base+0x10
  let a = base + 0x800;
  ThreadIsolation::register_jit_allocation(a, 0x40, JitAllocationType::WasmCode, false)?;

  {
    let mut page = WritableJitPage::new(base, PAGE)?;
    page.unregister_allocations_except(base, 0x10, &[])?;
  }

  // 空区间 no-op: 分配必须完好保留 (旧实现此处 panic / 原版 C++ 恒序不触发)
  assert_eq!(
    ThreadIsolation::start_of_jit_allocation_at(a + 0x10),
    Some(a)
  );

  ThreadIsolation::unregister_jit_page(base, PAGE)?;
  OK
}

/// [round2-F4 锁行为] UnregisterAllocationsExcept 右切: 区间内分配跨过
/// range_end 必须 Err(SplitFreeRange) 且注册表不动 (原版静默弃注册,
/// 登记偏差, 本用例锁定加严语义)
#[test]
fn code_memory_unregister_allocations_except_right_slice_rejected() -> Void {
  init()?;
  let base = 0x7600_0000usize;
  let page_size = 2 * PAGE;
  ThreadIsolation::register_jit_page(base, page_size)?;

  let straddle = base + 0x1080; // [0x1080, 0x10c0) 跨过 range_end 0x10a0
  let after = base + 0x1800;
  ThreadIsolation::register_jit_allocation(straddle, 0x40, JitAllocationType::WasmCode, false)?;
  ThreadIsolation::register_jit_allocation(after, 0x40, JitAllocationType::WasmCode, false)?;

  {
    let mut page = WritableJitPage::new(base, page_size)?;
    let err = page.unregister_allocations_except(base + 0x1000, 0xa0, &[]);
    assert_eq!(
      err,
      Err(Error::SplitFreeRange {
        addr: base + 0x1000,
        size: 0xa0
      })
    );
  }

  // 错误路径零副作用: 跨切分配与区间外分配均完好
  assert_eq!(
    ThreadIsolation::start_of_jit_allocation_at(straddle + 0x10),
    Some(straddle)
  );
  assert_eq!(
    ThreadIsolation::start_of_jit_allocation_at(after),
    Some(after)
  );

  ThreadIsolation::unregister_jit_page(base, page_size)?;
  OK
}

/// WritableJitAllocation copy_code 越界门禁校验
#[test]
#[should_panic(expected = "copy_code 越界")]
fn code_memory_copy_code_oob_panics() {
  let mut buf = [0u8; 16];
  let alloc = WritableJitAllocation::for_non_executable_memory(
    buf.as_mut_ptr() as usize,
    16,
    JitAllocationType::InstructionStream,
  );
  alloc.copy_code(10, &[0u8; 10]);
}

/// WritableJitAllocation clear_bytes 越界门禁校验
#[test]
#[should_panic(expected = "clear_bytes 越界")]
fn code_memory_clear_bytes_oob_panics() {
  let mut buf = [0u8; 16];
  let alloc = WritableJitAllocation::for_non_executable_memory(
    buf.as_mut_ptr() as usize,
    16,
    JitAllocationType::InstructionStream,
  );
  alloc.clear_bytes(10, 10);
}

/// WritableJitAllocation write_header_slot 越界门禁校验
#[test]
#[should_panic(expected = "write_header_slot 越界")]
fn code_memory_write_header_slot_oob_panics() {
  let mut buf = [0u8; 4];
  let alloc = WritableJitAllocation::for_non_executable_memory(
    buf.as_mut_ptr() as usize,
    4,
    JitAllocationType::InstructionStream,
  );
  alloc.write_header_slot::<_, 0>(0x1234_5678_9ABC_DEF0_u64);
}

/// WritableJitAllocation write_unaligned_value 地址下溢门禁校验
#[test]
#[should_panic(expected = "write_unaligned_value 地址下溢")]
fn code_memory_write_unaligned_underflow_panics() {
  let mut buf = [0u8; 16];
  let addr = buf.as_mut_ptr() as usize;
  let alloc = WritableJitAllocation::for_non_executable_memory(
    addr,
    16,
    JitAllocationType::InstructionStream,
  );
  alloc.write_unaligned_value(addr - 1, 0x12_u8);
}

/// WritableJitAllocation write_unaligned_value 越界门禁校验
#[test]
#[should_panic(expected = "write_unaligned_value 越界")]
fn code_memory_write_unaligned_oob_panics() {
  let mut buf = [0u8; 16];
  let addr = buf.as_mut_ptr() as usize;
  let alloc = WritableJitAllocation::for_non_executable_memory(
    addr,
    16,
    JitAllocationType::InstructionStream,
  );
  alloc.write_unaligned_value(addr + 10, 0x1234_5678_9ABC_DEF0_u64);
}

/// WritableJitAllocation write_value 地址下溢门禁校验
#[test]
#[should_panic(expected = "write_value 地址下溢")]
fn code_memory_write_value_underflow_panics() {
  let mut buf = [0u8; 16];
  let addr = buf.as_mut_ptr() as usize;
  let alloc = WritableJitAllocation::for_non_executable_memory(
    addr,
    16,
    JitAllocationType::InstructionStream,
  );
  alloc.write_value(addr - 1, 0x12_u8);
}

/// WritableJitAllocation write_value 越界门禁校验
#[test]
#[should_panic(expected = "write_value 越界")]
fn code_memory_write_value_oob_panics() {
  let mut buf = [0u8; 16];
  let addr = buf.as_mut_ptr() as usize;
  let alloc = WritableJitAllocation::for_non_executable_memory(
    addr,
    16,
    JitAllocationType::InstructionStream,
  );
  alloc.write_value(addr + 12, 0x1234_5678_u64);
}

/// WritableFreeSpace clear_tagged 越界门禁校验
#[test]
#[should_panic(expected = "clear_tagged 越界")]
fn code_memory_free_space_clear_tagged_oob_panics() {
  let mut buf = [0u8; 16];
  let free = WritableFreeSpace::for_non_executable_memory(buf.as_mut_ptr() as usize, 16);
  free.clear_tagged::<8>(3);
}

/// WritableFreeSpace write_header_slot 越界门禁校验
#[test]
#[should_panic(expected = "write_header_slot 越界")]
fn code_memory_free_space_write_header_slot_oob_panics() {
  let mut buf = [0u8; 4];
  let free = WritableFreeSpace::for_non_executable_memory(buf.as_mut_ptr() as usize, 4);
  free.write_header_slot::<_, 0>(0x1234_5678_9ABC_DEF0_u64);
}

/// WritableJitAllocation write_header_slot offset 越界门禁校验
/// (round2-F4: offset 模板参参与边界计算)
#[test]
#[should_panic(expected = "write_header_slot 越界")]
fn code_memory_write_header_slot_offset_oob_panics() {
  let mut buf = [0u8; 8];
  let alloc = WritableJitAllocation::for_non_executable_memory(
    buf.as_mut_ptr() as usize,
    8,
    JitAllocationType::InstructionStream,
  );
  // offset 6 + u32 → 越出 8 字节分配
  alloc.write_header_slot::<_, 6>(0x1234_5678_u32);
}

/// WritableJitAllocation write_value_relaxed 越界门禁校验 (round2-F4)
#[test]
#[should_panic(expected = "write_value_relaxed 越界")]
fn code_memory_write_value_relaxed_oob_panics() {
  let mut buf = [0u8; 16];
  let addr = buf.as_mut_ptr() as usize;
  let alloc = WritableJitAllocation::for_non_executable_memory(
    addr,
    16,
    JitAllocationType::InstructionStream,
  );
  alloc.write_value_relaxed(addr + 12, 0x1234_5678_u64);
}

/// 测试用无 pkey 分配器 (trait 契约: 非 pku 平台 pkey 恒 -1)
struct PkeylessAllocator;

impl ThreadIsolatedAllocator for PkeylessAllocator {
  fn allocate(&self, _size: usize) -> Option<NonNull<u8>> {
    None
  }
  fn free(&self, _ptr: NonNull<u8>) {}
  fn pkey(&self) -> i32 {
    -1
  }
}

static PKEYLESS: PkeylessAllocator = PkeylessAllocator;

/// [回归 round2-F2] 非 pku 平台 force_memory_protection_keys: 原版 enable
/// 清零检查包在 #if V8_HAS_PKU_JIT_WRITE_PROTECT (cc:95-104) — 非 pku 平台
/// 不评估, 提供分配器即初始化成功 (旧实现恒落 Err(ForcedPkeyUnsupported))。
/// pku 平台 (linux x64) 结果依赖运行时 pkey API 探测, 不在本用例断言
#[test]
fn code_memory_initialize_forced_pkey_non_pku_platform() -> Void {
  if !cfg!(all(target_os = "linux", target_arch = "x86_64")) {
    set_force_memory_protection_keys(true);
    let r = ThreadIsolation::initialize(Some(&PKEYLESS));
    set_force_memory_protection_keys(false);
    assert!(r.is_ok(), "非 pku 平台强制 pkey 应初始化成功: {r:?}");
  }
  OK
}

/// [回归 round2-F3] expand 扩页后, 契约内 (串行) 注册路径的重叠检查必须
/// 观察到扩张后尺寸 — 扩张区间内的注册须 Err(RegionOverlap)。
/// expand 与 register 的并发竞争窗口由调用方契约排除 (expand 只持页锁
/// 不持注册表锁, 见 JitPageRef::expand 文档), 本用例锁定串行契约拒绝路径
#[test]
fn code_memory_expand_then_register_overlap_rejected() -> Void {
  init()?;
  let base = 0x7700_0000usize;
  ThreadIsolation::register_jit_page(base, PAGE)?;
  // 持页引用扩页: [base, base+PAGE) → [base, base+2*PAGE) (cc:318-320)
  {
    let mut page = ThreadIsolation::lookup_jit_page(base, PAGE)?;
    page.expand(PAGE);
    assert_eq!(page.size(), 2 * PAGE);
  }
  // 扩张区间内注册 → 重叠检查读到扩张后尺寸, 必须拒绝
  assert_eq!(
    ThreadIsolation::register_jit_page(base + PAGE, PAGE),
    Err(Error::RegionOverlap {
      addr: base + PAGE,
      size: PAGE,
    })
  );
  // 按扩张后尺寸注销复位
  ThreadIsolation::unregister_jit_page(base, 2 * PAGE)?;
  OK
}

/// [round2] lookup_jump_table_allocations Contains 失败错误路径: 跳表地址
/// 已注册但类型不符 → split 成功后 Contains 判假 → Err(JitAllocationMismatch)
/// (原版 -inl.h:131-136 CHECK(Contains(...)))
#[test]
fn code_memory_jump_table_lookup_contains_mismatch() -> Void {
  init()?;
  let jt = 0x7800_1000usize;
  let far = 0x7900_1000usize;
  ThreadIsolation::register_jit_page(jt - 0x1000, 0x2000)?;
  ThreadIsolation::register_jit_page(far - 0x1000, 0x2000)?;
  // jt 注册为 WasmCode (类型不符); far 注册为 WasmFarJumpTable 尺寸 0x20
  // (lookup 以 0x40 查 → 尺寸不符)
  ThreadIsolation::register_jit_allocation(jt, 0x40, JitAllocationType::WasmCode, false)?;
  ThreadIsolation::register_jit_allocation(far, 0x20, JitAllocationType::WasmFarJumpTable, false)?;
  assert_eq!(
    ThreadIsolation::lookup_jump_table_allocations(jt, 0x40, far, 0x40).err(),
    Some(Error::JitAllocationMismatch {
      addr: jt,
      size: 0x40,
    })
  );
  OK
}

/// [round3] 注册表并发压力: 多线程 register/unregister/bulk-split/except 交错
/// (固定 seed, fastrand 播种)。
/// 布局: 24 个不相交 slot (各 0x80000), 原子占位保证同一 slot 同一时刻仅
/// 单线程操作 — 逻辑面无竞争, 压力集中于注册表锁 / 页锁 / 原子尺寸镜像的
/// 真实并发交错 (含 bulk ≥ kSplitThreshold 触发的 split + WritableJitPage
/// 全页查找回触发的 merge); 另设 stable 只读页全程在册, 各线程随机并发
/// 查询, 断言恒命中 / 间隙恒未命中。
/// 线程内零 panic: 全部校验走 error 通道, 占位标记必然释放。
/// 收尾不变式: 全部 slot 注销后注册表清空 (tear_down Ok)。
/// 运行契约同 thread_isolation.rs: 注册表为进程级静态, 仅 nextest 进程
/// 隔离下运行。
#[test]
fn code_memory_registry_concurrent_stress() -> Void {
  use core::sync::atomic::{AtomicU32, Ordering};

  use parking_lot::Mutex;

  const SEED: u64 = 0x5EED_2026;
  const THREADS: usize = 8;
  const SLOTS: usize = 24;
  const SLOT_SIZE: usize = 0x80000; // ≥ 2 × kSplitThreshold, bulk 必拆
  const ITERATIONS: usize = 64;
  const BASE: usize = 0x7C00_0000;
  const STABLE: usize = BASE + SLOTS * SLOT_SIZE;
  const STABLE_PAGE: usize = 0x4000;

  init()?;
  // stable 只读页: 两条分配 + 一处已知间隙, 注册后至收尾前不动
  ThreadIsolation::register_jit_page(STABLE, STABLE_PAGE)?;
  ThreadIsolation::register_jit_allocation(STABLE, 0x40, JitAllocationType::WasmCode, false)?;
  ThreadIsolation::register_jit_allocation(
    STABLE + 0x100,
    0x40,
    JitAllocationType::InstructionStream,
    false,
  )?;

  let slot_state: Vec<AtomicU32> = (0..SLOTS).map(|_| AtomicU32::new(0)).collect();
  let errors: Mutex<Vec<String>> = Mutex::new(Vec::new());

  scope(|s| {
    for t in 0..THREADS {
      let slot_state = &slot_state;
      let errors = &errors;
      s.spawn(move || {
        // 固定 seed + 线程号派生, 交错不定但不变式与交错无关
        let mut rng = Rng::with_seed(SEED ^ (t as u64).wrapping_mul(0x9E3_779B_97F4_A7C1));
        for _ in 0..ITERATIONS {
          // 并发读面: stable 分配恒命中 / 间隙恒未命中
          if rng.usize(..4) == 0 {
            let hit = ThreadIsolation::start_of_jit_allocation_at(STABLE + 0x8);
            if hit != Some(STABLE) {
              errors.lock().push(format!("t{t}: stable 命中错 {hit:?}"));
            }
            let hit2 = ThreadIsolation::start_of_jit_allocation_at(STABLE + 0x108);
            if hit2 != Some(STABLE + 0x100) {
              errors
                .lock()
                .push(format!("t{t}: stable+0x100 命中错 {hit2:?}"));
            }
            let gap = ThreadIsolation::start_of_jit_allocation_at(STABLE + 0x80);
            if gap.is_some() {
              errors.lock().push(format!("t{t}: stable 间隙错 {gap:?}"));
            }
            let _ = ThreadIsolation::can_lookup_start_of_jit_allocation_at(STABLE);
          }
          // 占 slot (CAS 轮转, 必有空闲)
          let slot = loop {
            let i = rng.usize(..SLOTS);
            if slot_state[i]
              .compare_exchange(0, 1, Ordering::SeqCst, Ordering::SeqCst)
              .is_ok()
            {
              break i;
            }
          };
          let base = BASE + slot * SLOT_SIZE;
          let run = (|| -> Result<(), Error> {
            ThreadIsolation::register_jit_page(base, SLOT_SIZE)?;
            for k in 1..=3usize {
              ThreadIsolation::register_jit_allocation(
                base + 0x100 * k,
                0x40,
                JitAllocationType::WasmCode,
                false,
              )?;
            }
            if ThreadIsolation::start_of_jit_allocation_at(base + 0x110) != Some(base + 0x100) {
              return Err(Error::JitAllocationMismatch {
                addr: base + 0x110,
                size: 0,
              });
            }
            // 1/3 概率 bulk 注册 (总量 0x40000 = kSplitThreshold → split)
            if rng.usize(..3) == 0 {
              ThreadIsolation::register_jit_allocations(
                base + 0x1000,
                &[0x20000, 0x20000],
                JitAllocationType::WasmCode,
              )?;
              if ThreadIsolation::start_of_jit_allocation_at(base + 0x1004) != Some(base + 0x1000) {
                return Err(Error::JitAllocationMismatch {
                  addr: base + 0x1004,
                  size: 0,
                });
              }
            }
            // 1/2 概率页写通道 free_range (全页查找触发 merge)
            if rng.bool() {
              let mut page = WritableJitPage::new(base, SLOT_SIZE)?;
              page.free_range(base + 0x100, 0x40)?;
            }
            // 1/2 概率 except 注销: 保留 [base+0x200, 0x300) 内的 keep 项
            if rng.bool() {
              let mut page = WritableJitPage::new(base, SLOT_SIZE)?;
              page.unregister_allocations_except(base + 0x200, 0x100, &[base + 0x200])?;
            }
            ThreadIsolation::unregister_jit_page(base, SLOT_SIZE)?;
            Ok(())
          })();
          if let Err(e) = run {
            errors.lock().push(format!("t{t} slot{slot}: {e}"));
          }
          slot_state[slot].store(0, Ordering::SeqCst);
        }
      });
    }
  });

  let errs = errors.into_inner();
  assert!(
    errs.is_empty(),
    "并发压力出现 {} 处不变式破坏: {errs:#?}",
    errs.len()
  );
  assert!(
    slot_state.iter().all(|s| s.load(Ordering::SeqCst) == 0),
    "占位标记未全部释放"
  );

  ThreadIsolation::unregister_jit_page(STABLE, STABLE_PAGE)?;
  // 收尾不变式: 注册表清空 (原版 TearDown CHECK(jit_pages_->empty()), cc:160)
  assert_eq!(ThreadIsolation::tear_down(), Ok(()));
  OK
}
