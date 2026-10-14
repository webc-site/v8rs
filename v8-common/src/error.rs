//! v8-common 统一错误层
//!
//! 原版 CHECK/DCHECK 失败 = 进程终止; 本 crate 禁 panic, 可失败路径一律落 Result。
//! 每个变体对应原版一处 CHECK 失败点 (登记见各模块注释)。

use core::result;

use thiserror::Error;

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
  /// 原版 LookupJitPageLocked 的 CHECK(jit_page.has_value())
  /// (code-memory-access.cc:171) — 未注册或跨页未合并
  #[error("jit page 不存在: addr={addr:#x} size={size:#x}")]
  JitPageNotFound { addr: usize, size: usize },
  /// 原版 CheckForRegionOverlap (code-memory-access.cc:255) —
  /// 注册区间与既有区间重叠 (CFI 安全检查)
  #[error("注册区间重叠: addr={addr:#x} size={size:#x}")]
  RegionOverlap { addr: usize, size: usize },
  /// 原版 RegisterAllocation 的页内越界 CHECK (code-memory-access.cc:336-341)
  #[error("分配越出 jit page: addr={addr:#x} size={size:#x} page={page:#x}")]
  AllocOutOfPage {
    addr: usize,
    size: usize,
    page: usize,
  },
  /// 原版 LookupAllocation / UnregisterAllocation 的条目校验失败
  /// (code-memory-access.cc:348-357, 367-371)
  #[error("jit 分配不存在或类型/尺寸不符: addr={addr:#x} size={size:#x}")]
  JitAllocationMismatch { addr: usize, size: usize },
  /// 原版 AllocationIsBehindRange 的 CHECK_LE (code-memory-access.cc:292) —
  /// 释放范围切进分配中间
  #[error("释放范围切进分配中间: addr={addr:#x} size={size:#x}")]
  SplitFreeRange { addr: usize, size: usize },
  /// 原版 UnregisterAllocationsExcept 的 CHECK_EQ(keep_iterator, keep.end())
  /// (code-memory-access.cc:410)
  #[error("保留清单与区间内分配不一致")]
  KeepListMismatch,
  /// 原版 CHECK_GE(addr + size, addr) — 地址空间回绕 (code-memory-access.cc:258)
  #[error("地址回绕: addr={addr:#x} size={size:#x}")]
  AddrOverflow { addr: usize, size: usize },
  /// 原版 CHECK_GT(end, addr) (code-memory-access.cc:200-201) /
  /// CHECK_GT(end_offset, offset) (cc:339) — 零尺寸区间拒绝 (CFI: data untrusted)
  #[error("零尺寸区间: addr={addr:#x} size={size:#x}")]
  ZeroSize { addr: usize, size: usize },
  /// 原版 InitBase 的 CHECK_EQ(base, GetPtrComprCageBaseAddress(base))
  /// (ptr-compr-inl.h:58, 176) — cage base 未按页对齐
  #[error("cage base 未对齐: {base:#x}")]
  CageBaseUnaligned { base: usize },
  /// 原版 CHECK_IMPLIES(v8_flags.force_memory_protection_keys, enable)
  /// (code-memory-access.cc:128) — 强制 pkey 但运行环境不支持
  #[error("强制 memory protection keys 但环境不支持")]
  ForcedPkeyUnsupported,
  /// 原版 TearDown 的状态校验 (code-memory-access.cc:149-164)
  #[error("TearDown 前置条件不满足")]
  TearDownState,
  /// 原版 SynchronizationPointSupport::Initialize 的 CHECK_NULL(instance_)
  /// (synchronization-point-support.cc:28)
  #[error("同步点支持已初始化")]
  SyncPointAlreadyInitialized,
  /// 平台/配置不支持该操作 (原版 UNREACHABLE 或空实现的显式化)
  #[error("当前平台不支持: {0}")]
  NotSupported(&'static str),
}

pub type Result<T> = result::Result<T, Error>;
