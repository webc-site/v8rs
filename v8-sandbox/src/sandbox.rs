// 由 tool/src/sandbox/sandbox.js 从 v8/src/sandbox/sandbox.h + src/sandbox/sandbox.cc 生成; 修订就地改, 复跑覆盖
// 沙箱本体: 地址空间预留/保护区域/常量池/单例; 对数核对 17 条通过

//! Sandbox; 对应原版 sandbox{.h,.cc}。
//! 结构: [前 guard 64GB][沙箱 1TB][后 guard 64GB+224GB] 连续 PROT_NONE 预留;
//! 预留失败回退部分预留沙箱 (减半至 8GB); 沙箱内分配 = 预留内 mprotect (模拟子空间)。
//! 偏差 (README 登记): VAS 子空间 → mmap 预留 + 首次适应; pkey 附加见 hardware_support。

use std::{
  collections::BTreeMap,
  ptr::null_mut,
  sync::{Mutex as StdMutex, MutexGuard, OnceLock},
};

use v8_base_platform::{AddressSpaceReservation, MemoryPermission, OS};

use crate::{
  Error, Result,
  internal::{
    Address, GB, K_ADDITIONAL_TRAILING_GUARD_REGION_SIZE, K_SANDBOX_ALIGNMENT,
    K_SANDBOX_GUARD_REGION_SIZE, K_SANDBOX_MINIMUM_RESERVATION_SIZE, K_SANDBOX_SIZE, KB,
  },
};

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
#[derive(Debug, Clone, Copy, Default)]
pub struct SandboxedPointerConstants {
  pub empty_backing_store_buffer: Address,
}

// ---- 沙箱内模拟子空间 ----
/// 预留内的页分配器 (EmulatedVirtualAddressSubspace 语义): 首次适应 + mprotect 提交
pub struct SandboxAddressSpace {
  /// 底层地址空间预留
  pub(crate) reservation: AddressSpaceReservation,
  /// 沙箱基址 (预留内偏移 guard 后)
  pub(crate) base: Address,
  /// 可分配区大小 (全预留 = 沙箱大小; 部分预留 = 实际预留大小)
  pub(crate) size: usize,
  /// 已分配区段 start→len (锁保护, &self 下的内部可变性)
  pub(crate) allocated: StdMutex<BTreeMap<Address, usize>>,
}

impl SandboxAddressSpace {
  /// 原版 VirtualAddressSpace::allocation_granularity (POSIX = 页大小)
  pub fn allocation_granularity(&self) -> usize {
    OS::allocate_page_size()
  }

  /// 原版 AllocatePages (kNoHint → 首次适应)
  pub fn allocate_pages(
    &self,
    hint: Address,
    length: usize,
    alignment: usize,
    access: MemoryPermission,
  ) -> Result<Address> {
    let mut allocated = self.lock_allocated();
    if length == 0 || !alignment.is_power_of_two() {
      return Err(Error::InvalidArgument);
    }
    let region = if hint == 0 {
      self.find_free_region(&allocated, length, alignment)?
    } else {
      if !Self::region_free(&allocated, hint, length, self.base, self.size) {
        return Err(Error::InvalidArgument);
      }
      hint
    };
    // SAFETY: [region, +length) 已确认在预留区内且按对齐取址。
    let committed = unsafe { self.reservation.allocate(region as *mut _, length, access) }
      .map_err(|_| Error::SandboxReservationFailed)?;
    if !committed {
      return Err(Error::SandboxReservationFailed);
    }
    allocated.insert(region, length);
    Ok(region)
  }

  /// 原版 AllocateGuardRegion: 占位不改权限 (保持 PROT_NONE)
  pub fn allocate_guard_region(&self, address: Address, size: usize) -> Result<bool> {
    let mut allocated = self.lock_allocated();
    let in_range = self.base.checked_add(self.size).is_some_and(|end| {
      address >= self.base && address.checked_add(size).is_some_and(|e| e <= end)
    });
    if !in_range || !Self::region_free(&allocated, address, size, self.base, self.size) {
      return Ok(false);
    }
    allocated.insert(address, size);
    Ok(true)
  }

  /// 原版 FreePages: 回收为 PROT_NONE + 丢弃数据
  pub fn free_pages(&self, address: Address, size: usize) -> Result<()> {
    let mut allocated = self.lock_allocated();
    if allocated.get(&address) != Some(&size) {
      return Err(Error::InvalidArgument);
    }
    // SAFETY: 区间来自本空间此前的分配记录。
    let freed = unsafe { self.reservation.free(address as *mut _, size) }
      .map_err(|_| Error::SandboxReservationFailed)?;
    if !freed {
      return Err(Error::InvalidArgument);
    }
    allocated.remove(&address);
    Ok(())
  }

  /// 原版 SetPagePermissions
  pub fn set_page_permissions(
    &self,
    address: Address,
    size: usize,
    access: MemoryPermission,
  ) -> Result<()> {
    // SAFETY: 调用方保证区间在本预留内 (原版同样前置约束)。
    let ok = unsafe {
      self
        .reservation
        .set_permissions(address as *mut _, size, access)
    }
    .map_err(|_| Error::SandboxReservationFailed)?;
    if !ok {
      return Err(Error::InvalidArgument);
    }
    Ok(())
  }

  /// 取分配表锁 (锁中毒 = 前持锁者 panic, 恢复数据继续用)
  fn lock_allocated(&self) -> MutexGuard<'_, BTreeMap<Address, usize>> {
    self
      .allocated
      .lock()
      .unwrap_or_else(|poisoned| poisoned.into_inner())
  }

  /// 首次适应找空区 (对齐向上取整)
  fn find_free_region(
    &self,
    allocated: &BTreeMap<Address, usize>,
    length: usize,
    alignment: usize,
  ) -> Result<Address> {
    let align_up = |v: usize| v.div_ceil(alignment).saturating_mul(alignment);
    let mut cursor = align_up(self.base);
    for (&addr, &len) in allocated {
      // cursor 与已分配区前的空隙可容纳
      if addr > cursor && cursor.checked_add(length).is_some_and(|end| end <= addr) {
        return Ok(cursor);
      }
      let block_end = addr.saturating_add(len);
      if cursor < block_end {
        cursor = align_up(block_end);
      }
    }
    match cursor.checked_add(length) {
      Some(end) if end <= self.base.saturating_add(self.size) => Ok(cursor),
      _ => Err(Error::SandboxReservationFailed),
    }
  }

  /// [address, +size) 与已分配区是否无交叠且在区内
  fn region_free(
    allocated: &BTreeMap<Address, usize>,
    address: Address,
    size: usize,
    base: Address,
    region_size: usize,
  ) -> bool {
    let Some(end) = address.checked_add(size) else {
      return false;
    };
    if end > base.saturating_add(region_size) || address < base {
      return false;
    }
    // BTreeMap 有序: 起点在 end 之前的所有区段检查交叠
    allocated
      .range(..end)
      .all(|(&start, &len)| start.saturating_add(len) <= address)
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
  /// 追加尾部 guard 的独立预留 (尽力而为, 失败不阻塞初始化)
  pub(crate) trailing_guard: Option<AddressSpaceReservation>,
}

/// 初始化配置 (对应原版 Platform 注入面: 仅取零段大小)
#[derive(Debug, Clone, Copy, Default)]
pub struct InitConfig {
  /// 平台零段大小 (原版 platform-&gt;GetZeroSegmentSize()); &gt;= 4GB+4KB 视为 Smi 范围不可访问
  pub zero_segment_size: usize,
}

/// 原版 DetermineAddressSpaceLimit (sandbox.cc:261-344):
/// 非 LA57 桌面平台进程可用 VA 宽度 47 位 (原版探测启发式的常量化)
pub fn determine_address_space_limit() -> Address {
  1usize << 47
}

impl Sandbox {
  pub fn new() -> Self {
    Self {
      base: 0,
      end: 0,
      size: 0,
      reservation_base: 0,
      reservation_size: 0,
      initialized: false,
      smi_address_range_reserved: false,
      constants: SandboxedPointerConstants::default(),
      address_space: None,
      trailing_guard: None,
    }
  }

  /// 原版 Initialize(platform, vas) (sandbox.cc:346-442): 全量流 (含回退)
  pub fn initialize(&mut self, config: InitConfig) -> Result<()> {
    if self.initialized {
      return Err(Error::AlreadyInitialized);
    }
    if self
      .initialize_with_size(config, K_SANDBOX_SIZE, true)
      .is_ok()
    {
      return Ok(());
    }
    // 回退: 部分预留 (逻辑大小不变, 预留减半至下限)
    if K_FALLBACK_TO_PARTIALLY_RESERVED_SANDBOX_ALLOWED {
      let mut reserved = K_SANDBOX_SIZE / 2;
      while reserved >= K_SANDBOX_MINIMUM_RESERVATION_SIZE {
        if self
          .initialize_as_partially_reserved_sandbox(config, K_SANDBOX_SIZE, reserved)
          .is_ok()
        {
          return Ok(());
        }
        reserved /= 2;
      }
    }
    Err(Error::SandboxReservationFailed)
  }

  /// 原版 私有 Initialize(vas, size, use_guard_regions) (sandbox.cc:444-515)
  pub fn initialize_with_size(
    &mut self,
    config: InitConfig,
    size: usize,
    use_guard_regions: bool,
  ) -> Result<()> {
    if self.initialized {
      return Err(Error::AlreadyInitialized);
    }
    let guard = if use_guard_regions {
      K_SANDBOX_GUARD_REGION_SIZE
    } else {
      0
    };
    // 主体预留 = 沙箱 + 前后 guard (trailing 为独立追加预留, 不计入)
    let total = size.checked_add(2 * guard).ok_or(Error::InvalidArgument)?;
    // SAFETY: 预留生命周期由 address_space.reservation 持有, tear_down 统一释放。
    let reservation = unsafe {
      AddressSpaceReservation::create(
        null_mut(),
        total,
        K_SANDBOX_ALIGNMENT,
        MemoryPermission::NoAccess,
      )
    }
    .map_err(|_| Error::SandboxReservationFailed)?;
    let reservation_base = reservation.base() as Address;
    let base = reservation_base + guard;
    self.base = base;
    self.size = size;
    self.reservation_base = reservation_base;
    self.reservation_size = total;
    self.address_space = Some(SandboxAddressSpace {
      reservation,
      base,
      size,
      allocated: StdMutex::new(BTreeMap::new()),
    });
    if use_guard_regions {
      // 追加尾部 guard: 独立预留尽力而为, 失败仅忽略 (原版 DCHECK 允许缺失)
      let hint = reservation_base.wrapping_add(total);
      self.trailing_guard = unsafe {
        AddressSpaceReservation::create(
          hint as *mut _,
          K_ADDITIONAL_TRAILING_GUARD_REGION_SIZE,
          OS::commit_page_size(),
          MemoryPermission::NoAccess,
        )
      }
      .ok();
    }
    self.finish_initialization(config)
  }

  /// 原版 InitializeAsPartiallyReservedSandbox (sandbox.cc:517-582):
  /// 逻辑大小 = size, 实际预留 = size_to_reserve (无 guard)
  pub fn initialize_as_partially_reserved_sandbox(
    &mut self,
    config: InitConfig,
    size: usize,
    size_to_reserve: usize,
  ) -> Result<()> {
    if self.initialized {
      return Err(Error::AlreadyInitialized);
    }
    if size_to_reserve == 0 || size_to_reserve > size {
      return Err(Error::InvalidArgument);
    }
    // SAFETY: 同 initialize_with_size。
    let reservation = unsafe {
      AddressSpaceReservation::create(
        null_mut(),
        size_to_reserve,
        K_SANDBOX_ALIGNMENT,
        MemoryPermission::NoAccess,
      )
    }
    .map_err(|_| Error::SandboxReservationFailed)?;
    let base = reservation.base() as Address;
    self.base = base;
    self.size = size;
    self.reservation_base = base;
    self.reservation_size = size_to_reserve;
    self.address_space = Some(SandboxAddressSpace {
      reservation,
      base,
      // 部分预留: 页分配仅限实际预留区
      size: size_to_reserve,
      allocated: StdMutex::new(BTreeMap::new()),
    });
    self.finish_initialization(config)
  }

  /// 原版 FinishInitialization (sandbox.cc:584-619): smi 范围检查 + 末页 guard + 常量
  fn finish_initialization(&mut self, config: InitConfig) -> Result<()> {
    if self.base == 0 || self.size == 0 {
      return Err(Error::NotInitialized);
    }
    self.end = self.base.wrapping_add(self.size);
    // 平台零段 >= 4GB+4KB → Smi 范围不可访问
    self.smi_address_range_reserved =
      config.zero_segment_size >= K_SMI_ADDRESS_RANGE + K_SMI_ADDRESS_RANGE_PADDING;
    // 空 backing store = 沙箱末字节 (末页 guard 语义: 误访问必撞 guard)
    self.constants = SandboxedPointerConstants {
      empty_backing_store_buffer: self.end.wrapping_sub(1),
    };
    self.initialized = true;
    Ok(())
  }

  /// 原版 TearDown (sandbox.cc:627-655): 释放预留 + 全字段复位
  pub fn tear_down(&mut self) -> Result<()> {
    if let Some(space) = self.address_space.take() {
      // SAFETY: 预留来自 initialize 且未被释放。
      unsafe { space.reservation.free_reservation() }
        .map_err(|_| Error::SandboxReservationFailed)?;
    }
    if let Some(trailing) = self.trailing_guard.take() {
      // SAFETY: 同上。
      unsafe { trailing.free_reservation() }.map_err(|_| Error::SandboxReservationFailed)?;
    }
    self.base = 0;
    self.end = 0;
    self.size = 0;
    self.reservation_base = 0;
    self.reservation_size = 0;
    self.initialized = false;
    self.smi_address_range_reserved = false;
    self.constants = SandboxedPointerConstants::default();
    Ok(())
  }

  pub const fn is_initialized(&self) -> bool {
    self.initialized
  }
  pub const fn is_partially_reserved(&self) -> bool {
    self.reservation_size < self.size
  }

  /// 原版 address_space(): 沙箱内页分配子空间
  pub fn address_space(&self) -> Result<&SandboxAddressSpace> {
    self.address_space.as_ref().ok_or(Error::NotInitialized)
  }

  /// 原版 Contains: [base, base+size)
  pub fn contains(&self, addr: Address) -> bool {
    addr >= self.base && addr < self.base.wrapping_add(self.size)
  }

  /// 原版 ReservationContains: 含 guard 的预留范围
  pub fn reservation_contains(&self, addr: Address) -> bool {
    addr >= self.reservation_base
      && addr < self.reservation_base.wrapping_add(self.reservation_size)
  }
}

impl Default for Sandbox {
  fn default() -> Self {
    Self::new()
  }
}

// ---- 进程级单例 (sandbox.cc:657-676 InitializeDefaultOncePerProcess / TearDownDefault) ----
static DEFAULT_SANDBOX: OnceLock<Sandbox> = OnceLock::new();

/// 原版 Sandbox::InitializeDefaultOncePerProcess
pub fn initialize_once(config: InitConfig) -> Result<&'static Sandbox> {
  if let Some(sandbox) = DEFAULT_SANDBOX.get() {
    return Ok(sandbox);
  }
  let mut sandbox = Sandbox::new();
  sandbox.initialize(config)?;
  // 竞争输家释放自己的预留防泄漏
  if let Err(mut loser) = DEFAULT_SANDBOX.set(sandbox) {
    let _ = loser.tear_down();
  }
  DEFAULT_SANDBOX.get().ok_or(Error::NotInitialized)
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
