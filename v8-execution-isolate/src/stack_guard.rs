// 由 tool/src/execution/stack-guard.js 从 v8/src/execution/stack-guard.h, src/execution/stack-guard.cc 生成; 修订就地改, 复跑覆盖
// StackGuard 中断表 12 项与栈限防护

use core::sync::atomic::{AtomicU32, AtomicUsize, Ordering};

/// 中断处理效果等级 (对应原版 InterruptLevel)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum InterruptLevel {
  /// 不触发 GC (kNoGC)
  NoGC = 0,
  /// 允许读取堆，但不写入堆 (kNoHeapWrites)
  NoHeapWrites = 1,
  /// 任意副作用，包括 GC 和装载代码 (kAnyEffect)
  AnyEffect = 2,
}

impl InterruptLevel {
  /// 是否允许在该等级下执行 target_level 要求的操作
  #[inline]
  pub const fn allows(self, target_level: InterruptLevel) -> bool {
    (target_level as u8) <= (self as u8)
  }
}

/// 12 项中断原因标志枚举与位定义 (对应原版 InterruptFlag : uint32_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum InterruptType {
  /// id=0, level=kNoGC
  TerminateExecution = 0,
  /// id=1, level=kNoHeapWrites
  GC = 1,
  /// id=2, level=kAnyEffect
  InstallCode = 2,
  /// id=3, level=kAnyEffect
  InstallBaselineCode = 3,
  /// id=4, level=kNoHeapWrites
  ApiInterrupt = 4,
  /// id=5, level=kNoHeapWrites
  DeoptMarkedAllocationSites = 5,
  /// id=6, level=kNoGC
  GrowSharedMemory = 6,
  /// id=7, level=kAnyEffect
  LogWasmCode = 7,
  /// id=8, level=kNoHeapWrites
  WasmCodeGC = 8,
  /// id=9, level=kAnyEffect
  InstallMaglevCode = 9,
  /// id=10, level=kNoHeapWrites
  GlobalSafepoint = 10,
  /// id=11, level=kNoHeapWrites
  StartIncrementalMarking = 11,
}

impl InterruptType {
  /// 中断标志位掩码 (1 << id)
  #[inline]
  pub const fn mask(self) -> u32 {
    1 << (self as u8)
  }

  /// 对应中断等级
  #[inline]
  pub const fn level(self) -> InterruptLevel {
    match self {
      InterruptType::TerminateExecution => InterruptLevel::NoGC,
      InterruptType::GC => InterruptLevel::NoHeapWrites,
      InterruptType::InstallCode => InterruptLevel::AnyEffect,
      InterruptType::InstallBaselineCode => InterruptLevel::AnyEffect,
      InterruptType::ApiInterrupt => InterruptLevel::NoHeapWrites,
      InterruptType::DeoptMarkedAllocationSites => InterruptLevel::NoHeapWrites,
      InterruptType::GrowSharedMemory => InterruptLevel::NoGC,
      InterruptType::LogWasmCode => InterruptLevel::AnyEffect,
      InterruptType::WasmCodeGC => InterruptLevel::NoHeapWrites,
      InterruptType::InstallMaglevCode => InterruptLevel::AnyEffect,
      InterruptType::GlobalSafepoint => InterruptLevel::NoHeapWrites,
      InterruptType::StartIncrementalMarking => InterruptLevel::NoHeapWrites,
    }
  }

  /// 原版大写名称
  #[inline]
  pub const fn name(self) -> &'static str {
    match self {
      InterruptType::TerminateExecution => "TERMINATE_EXECUTION",
      InterruptType::GC => "GC_REQUEST",
      InterruptType::InstallCode => "INSTALL_CODE",
      InterruptType::InstallBaselineCode => "INSTALL_BASELINE_CODE",
      InterruptType::ApiInterrupt => "API_INTERRUPT",
      InterruptType::DeoptMarkedAllocationSites => "DEOPT_MARKED_ALLOCATION_SITES",
      InterruptType::GrowSharedMemory => "GROW_SHARED_MEMORY",
      InterruptType::LogWasmCode => "LOG_WASM_CODE",
      InterruptType::WasmCodeGC => "WASM_CODE_GC",
      InterruptType::InstallMaglevCode => "INSTALL_MAGLEV_CODE",
      InterruptType::GlobalSafepoint => "GLOBAL_SAFEPOINT",
      InterruptType::StartIncrementalMarking => "START_INCREMENTAL_MARKING",
    }
  }
}

/// 中断标志位常量集合
pub mod interrupt_flags {

  /// 原版 TERMINATE_EXECUTION = 1 << 0
  pub const TERMINATE_EXECUTION: u32 = 1 << 0;
  /// 原版 GC_REQUEST = 1 << 1
  pub const GC_REQUEST: u32 = 1 << 1;
  /// 原版 INSTALL_CODE = 1 << 2
  pub const INSTALL_CODE: u32 = 1 << 2;
  /// 原版 INSTALL_BASELINE_CODE = 1 << 3
  pub const INSTALL_BASELINE_CODE: u32 = 1 << 3;
  /// 原版 API_INTERRUPT = 1 << 4
  pub const API_INTERRUPT: u32 = 1 << 4;
  /// 原版 DEOPT_MARKED_ALLOCATION_SITES = 1 << 5
  pub const DEOPT_MARKED_ALLOCATION_SITES: u32 = 1 << 5;
  /// 原版 GROW_SHARED_MEMORY = 1 << 6
  pub const GROW_SHARED_MEMORY: u32 = 1 << 6;
  /// 原版 LOG_WASM_CODE = 1 << 7
  pub const LOG_WASM_CODE: u32 = 1 << 7;
  /// 原版 WASM_CODE_GC = 1 << 8
  pub const WASM_CODE_GC: u32 = 1 << 8;
  /// 原版 INSTALL_MAGLEV_CODE = 1 << 9
  pub const INSTALL_MAGLEV_CODE: u32 = 1 << 9;
  /// 原版 GLOBAL_SAFEPOINT = 1 << 10
  pub const GLOBAL_SAFEPOINT: u32 = 1 << 10;
  /// 原版 START_INCREMENTAL_MARKING = 1 << 11
  pub const START_INCREMENTAL_MARKING: u32 = 1 << 11;

  /// 所有 12 项中断组合掩码 (对应原版 ALL_INTERRUPTS)
  pub const ALL_INTERRUPTS: u32 = (1 << 12) - 1;

  /// 根据中断等级计算过滤掩码 (对应原版 InterruptLevelMask)
  #[inline]
  pub const fn interrupt_level_mask(level: super::InterruptLevel) -> u32 {
    let mut mask = 0u32;
    if (super::InterruptLevel::NoGC as u8) <= (level as u8) {
      mask |= TERMINATE_EXECUTION;
    }
    if (super::InterruptLevel::NoHeapWrites as u8) <= (level as u8) {
      mask |= GC_REQUEST;
    }
    if (super::InterruptLevel::AnyEffect as u8) <= (level as u8) {
      mask |= INSTALL_CODE;
    }
    if (super::InterruptLevel::AnyEffect as u8) <= (level as u8) {
      mask |= INSTALL_BASELINE_CODE;
    }
    if (super::InterruptLevel::NoHeapWrites as u8) <= (level as u8) {
      mask |= API_INTERRUPT;
    }
    if (super::InterruptLevel::NoHeapWrites as u8) <= (level as u8) {
      mask |= DEOPT_MARKED_ALLOCATION_SITES;
    }
    if (super::InterruptLevel::NoGC as u8) <= (level as u8) {
      mask |= GROW_SHARED_MEMORY;
    }
    if (super::InterruptLevel::AnyEffect as u8) <= (level as u8) {
      mask |= LOG_WASM_CODE;
    }
    if (super::InterruptLevel::NoHeapWrites as u8) <= (level as u8) {
      mask |= WASM_CODE_GC;
    }
    if (super::InterruptLevel::AnyEffect as u8) <= (level as u8) {
      mask |= INSTALL_MAGLEV_CODE;
    }
    if (super::InterruptLevel::NoHeapWrites as u8) <= (level as u8) {
      mask |= GLOBAL_SAFEPOINT;
    }
    if (super::InterruptLevel::NoHeapWrites as u8) <= (level as u8) {
      mask |= START_INCREMENTAL_MARKING;
    }
    mask
  }
}

/// 栈限与中断哨兵管理 (StackGuard)
#[derive(Debug)]
pub struct StackGuard {
  /// JS 栈限 (触发中断检查的当前限制)
  jslimit: AtomicUsize,
  /// 真实 JS 栈限 (底线)
  real_jslimit: AtomicUsize,
  /// C++ 栈限
  climit: AtomicUsize,
  /// 真实 C++ 栈限
  real_climit: AtomicUsize,
  /// 当前处于激活状态的中断原因位图
  interrupt_flags: AtomicU32,
}

impl Default for StackGuard {
  #[inline]
  fn default() -> Self {
    Self::new()
  }
}

impl StackGuard {
  /// 中断哨兵标记地址 (对应原版 kInterruptLimit = ~0)
  pub const INTERRUPT_LIMIT: usize = usize::MAX;

  /// 新建 StackGuard 实例
  pub const fn new() -> Self {
    Self {
      jslimit: AtomicUsize::new(Self::INTERRUPT_LIMIT),
      real_jslimit: AtomicUsize::new(0),
      climit: AtomicUsize::new(Self::INTERRUPT_LIMIT),
      real_climit: AtomicUsize::new(0),
      interrupt_flags: AtomicU32::new(0),
    }
  }

  /// 设置真实栈限制 (由运行时根据当前线程栈顶和栈大小初始化)
  pub fn set_stack_limit(&self, limit: usize) {
    self.real_jslimit.store(limit, Ordering::Release);
    self.real_climit.store(limit, Ordering::Release);
    // 若无中断待处理，同步更新当前限值
    if self.interrupt_flags.load(Ordering::Acquire) == 0 {
      self.jslimit.store(limit, Ordering::Release);
      self.climit.store(limit, Ordering::Release);
    }
  }

  /// 当前生效的 jslimit
  #[inline]
  pub fn jslimit(&self) -> usize {
    self.jslimit.load(Ordering::Acquire)
  }

  /// 真实底线 real_jslimit
  #[inline]
  pub fn real_jslimit(&self) -> usize {
    self.real_jslimit.load(Ordering::Acquire)
  }

  /// 当前生效的 climit
  #[inline]
  pub fn climit(&self) -> usize {
    self.climit.load(Ordering::Acquire)
  }

  /// 真实底线 real_climit
  #[inline]
  pub fn real_climit(&self) -> usize {
    self.real_climit.load(Ordering::Acquire)
  }

  /// 请求中断 (设置标志并下压栈限至 INTERRUPT_LIMIT 强制进入运行时拦截)
  pub fn request_interrupt(&self, flag: InterruptType) {
    self.interrupt_flags.fetch_or(flag.mask(), Ordering::SeqCst);
    self.jslimit.store(Self::INTERRUPT_LIMIT, Ordering::SeqCst);
    self.climit.store(Self::INTERRUPT_LIMIT, Ordering::SeqCst);
  }

  /// 清除特定中断
  pub fn clear_interrupt(&self, flag: InterruptType) {
    let old = self
      .interrupt_flags
      .fetch_and(!flag.mask(), Ordering::SeqCst);
    // 若所有中断已清空，还原为真实底线
    if (old & !flag.mask()) == 0 {
      let r_js = self.real_jslimit.load(Ordering::Acquire);
      let r_c = self.real_climit.load(Ordering::Acquire);
      self.jslimit.store(r_js, Ordering::Release);
      self.climit.store(r_c, Ordering::Release);
    }
  }

  /// 检查特定中断是否处于 pending 状态
  #[inline]
  pub fn check_interrupt(&self, flag: InterruptType) -> bool {
    (self.interrupt_flags.load(Ordering::Acquire) & flag.mask()) != 0
  }

  /// 检查是否存在任何中断请求
  #[inline]
  pub fn has_any_interrupt(&self) -> bool {
    self.interrupt_flags.load(Ordering::Acquire) != 0
  }

  /// 检查特定等级及以下是否有中断请求
  #[inline]
  pub fn has_interrupt_for_level(&self, level: InterruptLevel) -> bool {
    let mask = interrupt_flags::interrupt_level_mask(level);
    (self.interrupt_flags.load(Ordering::Acquire) & mask) != 0
  }

  /// 当前挂起的中断掩码
  #[inline]
  pub fn pending_interrupts(&self) -> u32 {
    self.interrupt_flags.load(Ordering::Acquire)
  }

  /// 检查给定的栈指针是否发生溢出 (低于 real_jslimit)
  #[inline]
  pub fn is_stack_overflow(&self, current_sp: usize) -> bool {
    let limit = self.real_jslimit.load(Ordering::Acquire);
    limit != 0 && current_sp < limit
  }

  /// 原版 CheckTerminateExecution
  #[inline]
  pub fn check_terminate_execution(&self) -> bool {
    self.check_interrupt(InterruptType::TerminateExecution)
  }

  /// 原版 RequestTerminateExecution
  #[inline]
  pub fn request_terminate_execution(&self) {
    self.request_interrupt(InterruptType::TerminateExecution);
  }

  /// 原版 ClearTerminateExecution
  #[inline]
  pub fn clear_terminate_execution(&self) {
    self.clear_interrupt(InterruptType::TerminateExecution);
  }

  /// 原版 CheckGC
  #[inline]
  pub fn check_gc(&self) -> bool {
    self.check_interrupt(InterruptType::GC)
  }

  /// 原版 RequestGC
  #[inline]
  pub fn request_gc(&self) {
    self.request_interrupt(InterruptType::GC);
  }

  /// 原版 ClearGC
  #[inline]
  pub fn clear_gc(&self) {
    self.clear_interrupt(InterruptType::GC);
  }

  /// 原版 CheckInstallCode
  #[inline]
  pub fn check_install_code(&self) -> bool {
    self.check_interrupt(InterruptType::InstallCode)
  }

  /// 原版 RequestInstallCode
  #[inline]
  pub fn request_install_code(&self) {
    self.request_interrupt(InterruptType::InstallCode);
  }

  /// 原版 ClearInstallCode
  #[inline]
  pub fn clear_install_code(&self) {
    self.clear_interrupt(InterruptType::InstallCode);
  }

  /// 原版 CheckInstallBaselineCode
  #[inline]
  pub fn check_install_baseline_code(&self) -> bool {
    self.check_interrupt(InterruptType::InstallBaselineCode)
  }

  /// 原版 RequestInstallBaselineCode
  #[inline]
  pub fn request_install_baseline_code(&self) {
    self.request_interrupt(InterruptType::InstallBaselineCode);
  }

  /// 原版 ClearInstallBaselineCode
  #[inline]
  pub fn clear_install_baseline_code(&self) {
    self.clear_interrupt(InterruptType::InstallBaselineCode);
  }

  /// 原版 CheckApiInterrupt
  #[inline]
  pub fn check_api_interrupt(&self) -> bool {
    self.check_interrupt(InterruptType::ApiInterrupt)
  }

  /// 原版 RequestApiInterrupt
  #[inline]
  pub fn request_api_interrupt(&self) {
    self.request_interrupt(InterruptType::ApiInterrupt);
  }

  /// 原版 ClearApiInterrupt
  #[inline]
  pub fn clear_api_interrupt(&self) {
    self.clear_interrupt(InterruptType::ApiInterrupt);
  }

  /// 原版 CheckDeoptMarkedAllocationSites
  #[inline]
  pub fn check_deopt_marked_allocation_sites(&self) -> bool {
    self.check_interrupt(InterruptType::DeoptMarkedAllocationSites)
  }

  /// 原版 RequestDeoptMarkedAllocationSites
  #[inline]
  pub fn request_deopt_marked_allocation_sites(&self) {
    self.request_interrupt(InterruptType::DeoptMarkedAllocationSites);
  }

  /// 原版 ClearDeoptMarkedAllocationSites
  #[inline]
  pub fn clear_deopt_marked_allocation_sites(&self) {
    self.clear_interrupt(InterruptType::DeoptMarkedAllocationSites);
  }

  /// 原版 CheckGrowSharedMemory
  #[inline]
  pub fn check_grow_shared_memory(&self) -> bool {
    self.check_interrupt(InterruptType::GrowSharedMemory)
  }

  /// 原版 RequestGrowSharedMemory
  #[inline]
  pub fn request_grow_shared_memory(&self) {
    self.request_interrupt(InterruptType::GrowSharedMemory);
  }

  /// 原版 ClearGrowSharedMemory
  #[inline]
  pub fn clear_grow_shared_memory(&self) {
    self.clear_interrupt(InterruptType::GrowSharedMemory);
  }

  /// 原版 CheckLogWasmCode
  #[inline]
  pub fn check_log_wasm_code(&self) -> bool {
    self.check_interrupt(InterruptType::LogWasmCode)
  }

  /// 原版 RequestLogWasmCode
  #[inline]
  pub fn request_log_wasm_code(&self) {
    self.request_interrupt(InterruptType::LogWasmCode);
  }

  /// 原版 ClearLogWasmCode
  #[inline]
  pub fn clear_log_wasm_code(&self) {
    self.clear_interrupt(InterruptType::LogWasmCode);
  }

  /// 原版 CheckWasmCodeGC
  #[inline]
  pub fn check_wasm_code_gc(&self) -> bool {
    self.check_interrupt(InterruptType::WasmCodeGC)
  }

  /// 原版 RequestWasmCodeGC
  #[inline]
  pub fn request_wasm_code_gc(&self) {
    self.request_interrupt(InterruptType::WasmCodeGC);
  }

  /// 原版 ClearWasmCodeGC
  #[inline]
  pub fn clear_wasm_code_gc(&self) {
    self.clear_interrupt(InterruptType::WasmCodeGC);
  }

  /// 原版 CheckInstallMaglevCode
  #[inline]
  pub fn check_install_maglev_code(&self) -> bool {
    self.check_interrupt(InterruptType::InstallMaglevCode)
  }

  /// 原版 RequestInstallMaglevCode
  #[inline]
  pub fn request_install_maglev_code(&self) {
    self.request_interrupt(InterruptType::InstallMaglevCode);
  }

  /// 原版 ClearInstallMaglevCode
  #[inline]
  pub fn clear_install_maglev_code(&self) {
    self.clear_interrupt(InterruptType::InstallMaglevCode);
  }

  /// 原版 CheckGlobalSafepoint
  #[inline]
  pub fn check_global_safepoint(&self) -> bool {
    self.check_interrupt(InterruptType::GlobalSafepoint)
  }

  /// 原版 RequestGlobalSafepoint
  #[inline]
  pub fn request_global_safepoint(&self) {
    self.request_interrupt(InterruptType::GlobalSafepoint);
  }

  /// 原版 ClearGlobalSafepoint
  #[inline]
  pub fn clear_global_safepoint(&self) {
    self.clear_interrupt(InterruptType::GlobalSafepoint);
  }

  /// 原版 CheckStartIncrementalMarking
  #[inline]
  pub fn check_start_incremental_marking(&self) -> bool {
    self.check_interrupt(InterruptType::StartIncrementalMarking)
  }

  /// 原版 RequestStartIncrementalMarking
  #[inline]
  pub fn request_start_incremental_marking(&self) {
    self.request_interrupt(InterruptType::StartIncrementalMarking);
  }

  /// 原版 ClearStartIncrementalMarking
  #[inline]
  pub fn clear_start_incremental_marking(&self) {
    self.clear_interrupt(InterruptType::StartIncrementalMarking);
  }
}

// ── 编译期静态断言 ──
const _: () = {
  assert!(interrupt_flags::ALL_INTERRUPTS == 0xFFF);
  assert!(interrupt_flags::interrupt_level_mask(InterruptLevel::NoGC) == 0x041);
  assert!(interrupt_flags::interrupt_level_mask(InterruptLevel::NoHeapWrites) == 0xD73);
  assert!(interrupt_flags::interrupt_level_mask(InterruptLevel::AnyEffect) == 0xFFF);
};
