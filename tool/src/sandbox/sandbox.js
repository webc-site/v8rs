#!/usr/bin/env bun
// tool/src/sandbox/sandbox.js — 沙箱本体骨架生成
// 组→文件映射 (本脚本管辖):
//   v8/src/sandbox/sandbox.h + sandbox.cc → v8-sandbox/src/sandbox.rs
// 规则:
//   - 常量族核对 (Smi 范围/保护区域/fallback/命名)
//   - 初始化流锚点核对 (limit/4, 部分预留减半回退, 末页 guard, 常量池)
//   - 结构/签名骨架直出, 语义修订阶段实现
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/sandbox/sandbox.h");
const cc = readV8("src/sandbox/sandbox.cc");

// ---- 锚点断言 ----
const checks = [];
function check(desc, ok) {
  if (!ok) throw new Error(`sandbox: 对数核对失败: ${desc}`);
  checks.push(desc);
}
check("kFallbackToPartiallyReservedSandboxAllowed = true", h.includes("kFallbackToPartiallyReservedSandboxAllowed = true"));
check("kSandboxAddressSpaceName = v8-sandbox", h.includes('kSandboxAddressSpaceName = "v8-sandbox"'));
check("kSmiAddressRange = 4UL * GB", h.includes("kSmiAddressRange = 4UL * GB"));
check("kSmiAddressRangePadding = 4 * KB", h.includes("kSmiAddressRangePadding = 4 * KB"));
check("is_partially_reserved = reservation_size < size", h.includes("reservation_size_ < size_"));
check("Contains 半开区间", h.includes("base::IsInHalfOpenRange(addr, base_, base_ + size_)"));
check("预留上限 = 地址空间 / 4", cc.includes("max_reservation_size = address_space_limit / 4"));
check("部分预留回退逐次减半到最小值", cc.includes("reservation_size /= 2"));
check("默认 VA 位 48", cc.includes("kDefaultVirtualAddressBits = 48"));
check("VA 上下对半分", cc.includes("hardware_virtual_address_bits -= 1"));
check("真预留含追加尾部 guard", cc.includes("kAdditionalTrailingGuardRegionSize"));
check("前后 guard 各 kSandboxGuardRegionSize", /reservation_size \+= 2 \* kSandboxGuardRegionSize/.test(cc));
check("沙箱内最大权限 RW (禁可执行)", cc.includes("const PagePermissions kSandboxMaxPermissions = PagePermissions::kReadWrite"));
check("末页 guard 防越界 SandboxedPointer", cc.includes("end_ - allocation_granularity"));
check("空 backing store 置于沙箱末尾", cc.includes("set_empty_backing_store_buffer(end_ - 1)"));
check("部分预留尝试上限 10 次", cc.includes("kMaxAttempts = 10"));
check("进程级默认沙箱 (InitializeDefaultOncePerProcess)", h.includes("InitializeDefaultOncePerProcess"));

const body = `${banner("src/sandbox/sandbox.js", "src/sandbox/sandbox.h + src/sandbox/sandbox.cc", "沙箱本体: 地址空间预留/保护区域/常量池/单例; 对数核对 " + checks.length + " 条通过")}
//! Sandbox; 对应原版 sandbox{.h,.cc}。
//! 结构: [前 guard 64GB][沙箱 1TB][后 guard 64GB+224GB] 连续 PROT_NONE 预留;
//! 预留失败回退部分预留沙箱 (减半至 8GB); 沙箱内分配 = 预留内 mprotect (模拟子空间)。
//! 偏差 (README 登记): VAS 子空间 → mmap 预留 + 首次适应; pkey 附加见 hardware_support。

use std::collections::BTreeMap;
use std::sync::{Mutex as StdMutex, OnceLock};

use v8_base_platform::{AddressSpaceReservation, MemoryPermission, OS};

use crate::internal::{
  Address, KB, GB, K_ADDITIONAL_TRAILING_GUARD_REGION_SIZE, K_SANDBOX_ALIGNMENT, K_SANDBOX_GUARD_REGION_SIZE,
  K_SANDBOX_MINIMUM_RESERVATION_SIZE, K_SANDBOX_SIZE,
};
use crate::{Error, Result};

/// 允许回退到部分预留沙箱 (sandbox.h:72)
pub const K_FALLBACK_TO_PARTIALLY_RESERVED_SANDBOX_ALLOWED: bool = true;
/// 预留名 (sandbox.h:75)
pub const K_SANDBOX_ADDRESS_SPACE_NAME: &str = "v8-sandbox";
/// Smi 混用防御范围 (sandbox.h:77)
pub const K_SMI_ADDRESS_RANGE: usize = 4 * GB;
/// in-object property 访问的额外余量 (sandbox.h:82)
pub const K_SMI_ADDRESS_RANGE_PADDING: usize = 4 * KB;
/// 崩溃过滤器启发式 VA 位数, 非 LA57 (sandbox.h:88-92)
pub const K_MAX_VIRTUAL_ADDRESS_BITS_FOR_CRASH_FILTER: u32 = 48;

// ---- 常量池 (sandbox.h:230-246 SandboxedPointerConstants) ----
/// 沙箱内常量对象 (空 backing store 置于末页, 误访问必撞 guard)
#[derive(Debug, Clone, Copy)]
pub struct SandboxedPointerConstants {
  pub empty_backing_store_buffer: Address,
}

impl Default for SandboxedPointerConstants {
  fn default() -> Self { Self { empty_backing_store_buffer: 0 } }
}

// ---- 沙箱内模拟子空间 ----
/// 预留内的页分配器 (EmulatedVirtualAddressSubspace 语义): 首次适应 + mprotect 提交
pub(crate) struct SandboxAddressSpace {
  pub(crate) reservation: AddressSpaceReservation,
  /// 沙箱基址 (预留内偏移 guard 后)
  pub(crate) base: Address,
  pub(crate) size: usize,
  /// 已分配区段 start→len
  pub(crate) allocated: BTreeMap<Address, usize>,
  pub(crate) lock: StdMutex<()>,
}

impl SandboxAddressSpace {
  /// 原版 VirtualAddressSpace::allocation_granularity (POSIX = 页大小)
  pub fn allocation_granularity(&self) -> usize {
    todo!("修订阶段实现")
  }
  /// 原版 AllocatePages (kNoHint → 首次适应)
  pub fn allocate_pages(&self, hint: Address, length: usize, alignment: usize, access: MemoryPermission) -> Result<Address> {
    todo!("修订阶段实现")
  }
  /// 原版 AllocateGuardRegion: 占位不改权限 (PROT_NONE)
  pub fn allocate_guard_region(&self, address: Address, size: usize) -> Result<bool> {
    todo!("修订阶段实现")
  }
  /// 原版 FreePages: 回收为 PROT_NONE + 丢弃数据
  pub fn free_pages(&self, address: Address, size: usize) -> Result<()> {
    todo!("修订阶段实现")
  }
  /// 原版 SetPagePermissions
  pub fn set_page_permissions(&self, address: Address, size: usize, access: MemoryPermission) -> Result<()> {
    todo!("修订阶段实现")
  }
}

// ---- Sandbox (sandbox.h:48-370) ----
/// V8 沙箱: 单实例或进程级单例
pub struct Sandbox {
  pub base: Address,
  pub end: Address,
  pub size: usize,
  pub reservation_base: Address,
  pub reservation_size: usize,
  pub initialized: bool,
  pub smi_address_range_reserved: bool,
  pub constants: SandboxedPointerConstants,
  pub(crate) address_space: Option<SandboxAddressSpace>,
}

/// 初始化配置 (对应原版 Platform 注入面: 仅取 zero segment 大小)
#[derive(Debug, Clone, Copy, Default)]
pub struct InitConfig {
  /// 平台零段大小 (原版 platform-&gt;GetZeroSegmentSize()); &gt;= 4GB+4KB 视为 Smi 范围不可访问
  pub zero_segment_size: usize,
}

/// 原版 DetermineAddressSpaceLimit (sandbox.cc:261-344)
pub fn determine_address_space_limit() -> Address {
  todo!("修订阶段实现")
}

impl Sandbox {
  pub fn new() -> Self {
    todo!("修订阶段实现")
  }

  /// 原版 Initialize(platform, vas) (sandbox.cc:346-442): 全量流 (含回退)
  pub fn initialize(&mut self, config: InitConfig) -> Result<()> {
    todo!("修订阶段实现")
  }

  /// 原版 私有 Initialize(vas, size, use_guard_regions) (sandbox.cc:444-515)
  pub fn initialize_with_size(&mut self, config: InitConfig, size: usize, use_guard_regions: bool) -> Result<()> {
    todo!("修订阶段实现")
  }

  /// 原版 InitializeAsPartiallyReservedSandbox (sandbox.cc:517-582)
  pub fn initialize_as_partially_reserved_sandbox(
    &mut self,
    config: InitConfig,
    size: usize,
    size_to_reserve: usize,
  ) -> Result<()> {
    todo!("修订阶段实现")
  }

  /// 原版 FinishInitialization (sandbox.cc:584-619): smi 范围检查 + 末页 guard + 常量
  fn finish_initialization(&mut self, config: InitConfig) -> Result<()> {
    todo!("修订阶段实现")
  }

  /// 原版 TearDown (sandbox.cc:627-655)
  pub fn tear_down(&mut self) -> Result<()> {
    todo!("修订阶段实现")
  }

  pub const fn is_initialized(&self) -> bool { self.initialized }
  pub const fn is_partially_reserved(&self) -> bool { self.reservation_size < self.size }

  /// 原版 Contains: [base, base+size)
  pub fn contains(&self, addr: Address) -> bool {
    addr >= self.base && addr < self.base.wrapping_add(self.size)
  }

  /// 原版 ReservationContains: 含 guard 的预留范围
  pub fn reservation_contains(&self, addr: Address) -> bool {
    addr >= self.reservation_base && addr < self.reservation_base.wrapping_add(self.reservation_size)
  }
}

impl Default for Sandbox {
  fn default() -> Self { Self::new() }
}

// ---- 进程级单例 (sandbox.cc:657-676 InitializeDefaultOncePerProcess / TearDownDefault) ----
static DEFAULT_SANDBOX: OnceLock<Sandbox> = OnceLock::new();

/// 原版 Sandbox::InitializeDefaultOncePerProcess
pub fn initialize_once(config: InitConfig) -> Result<&'static Sandbox> {
  todo!("修订阶段实现")
}

/// 原版 Sandbox::current() / GetDefault()
pub fn current() -> Option<&'static Sandbox> {
  DEFAULT_SANDBOX.get()
}

// ---- 沙箱边界谓词 (sandbox.h:378-405) ----
/// 原版 OutsideSandbox: 未初始化视为全在沙箱外
pub fn outside_sandbox(address: Address) -> bool {
  current().is_none_or(|s| !s.reservation_contains(address))
}

/// 原版 InsideSandbox
pub fn inside_sandbox(address: Address) -> bool {
  current().is_some_and(|s| s.contains(address))
}

/// 原版 EmptyBackingStoreBuffer
pub fn empty_backing_store_buffer() -> Address {
  current().map_or(0, |s| s.constants.empty_backing_store_buffer)
}
`;

const p = writeSrc("v8-sandbox", "sandbox.rs", body);
console.log(`sandbox.js: ${p} (对数核对 ${checks.length} 条通过)`);
