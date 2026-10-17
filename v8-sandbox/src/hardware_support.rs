// 由 tool/src/sandbox/hardware-support.js 从 v8/src/sandbox/hardware-support.h + src/sandbox/hardware-support.cc + src/sandbox/code-sandboxing-mode.h 生成; 修订就地改, 复跑覆盖
// 硬件沙箱支持: PKU/pkey (linux x86_64 真实, 其余平台恒 inactive); 对数核对 8 条通过

//! SandboxHardwareSupport; 对应原版 hardware-support{.h,.cc} + code-sandboxing-mode.h。
//! 仅 linux x86_64 支持 PKU; 其他平台 TryActivate 恒 false, Enter/Exit 为 no-op
//! (与 v8-base-platform::MemoryProtectionKey 的平台矩阵一致)。
//! 偏差 (README 登记): EnableForCurrentThread 的备用信号栈保障
//! (EnsureAlternativeSignalStackIsAvailableForCurrentThread) 依赖未落地的信号子系统, 暂为 no-op;
//! DisallowSandboxAccess/AllowSandboxAccess 原版仅 DEBUG 构建 + 硬件启用时生效, 此处等价实现。

use std::sync::atomic::{AtomicI32, AtomicU32, Ordering};

use v8_base_platform::{
  K_DEFAULT_PROTECTION_KEY, K_NO_MEMORY_PROTECTION_KEY, MemoryPermission, MemoryProtectionKey,
  Permission,
};

use crate::Result;

// ---- CodeSandboxingMode (code-sandboxing-mode.h:22-36) ----
/// 代码沙箱模式: 沙箱内禁写沙箱外内存 (kSandboxed) / 无限制 (kUnsandboxed)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CodeSandboxingMode {
  Sandboxed = 0,
  Unsandboxed = 1,
}

// ---- pkey 全局 (hardware-support.cc:20-26) ----
static SANDBOX_PKEY: AtomicI32 = AtomicI32::new(K_NO_MEMORY_PROTECTION_KEY);
static OUT_OF_SANDBOX_PKEY: AtomicI32 = AtomicI32::new(K_NO_MEMORY_PROTECTION_KEY);
static EXTENSION_PKEY: AtomicI32 = AtomicI32::new(K_NO_MEMORY_PROTECTION_KEY);
/// 进入沙箱模式的 PKRU 掩码; 0 = 硬件支持未激活
static SANDBOXED_MODE_PKEY_MASK: AtomicU32 = AtomicU32::new(0);

/// 原版 SandboxHardwareSupport 静态集合
pub struct SandboxHardwareSupport;

impl SandboxHardwareSupport {
  /// 原版 TryActivateBeforeThreadCreation (hardware-support.cc:29-33):
  /// 线程创建前分配沙箱 pkey; 非 linux x86_64 恒 false
  pub fn try_activate_before_thread_creation() -> bool {
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    {
      if !MemoryProtectionKey::has_memory_protection_key_apis() {
        return false;
      }
      let Ok(key) = MemoryProtectionKey::allocate_key() else {
        return false;
      };
      SANDBOX_PKEY.store(key, Ordering::Relaxed);
      OUT_OF_SANDBOX_PKEY.store(K_DEFAULT_PROTECTION_KEY, Ordering::Relaxed);
      EXTENSION_PKEY.store(K_DEFAULT_PROTECTION_KEY, Ordering::Relaxed);
      SANDBOXED_MODE_PKEY_MASK.store(
        MemoryProtectionKey::compute_register_mask_for_permission_switch(
          key,
          Permission::DisableAccess,
        ),
        Ordering::Relaxed,
      );
      true
    }
    #[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
    {
      false
    }
  }

  /// 原版 IsActive (hardware-support.cc:36-38)
  pub fn is_active() -> bool {
    SANDBOX_PKEY.load(Ordering::Relaxed) != K_NO_MEMORY_PROTECTION_KEY
  }

  /// 原版 IsStrict (hardware-support.cc:41-47): 默认 pkey 作 out-of-sandbox key
  pub fn is_strict() -> bool {
    OUT_OF_SANDBOX_PKEY.load(Ordering::Relaxed) == K_DEFAULT_PROTECTION_KEY
  }

  /// 原版 EnableForCurrentThread (hardware-support.cc:50-82); altstack 保障偏差见模块注释
  pub fn enable_for_current_thread(_limit_address: Option<usize>) -> Result<()> {
    // 未激活: no-op 成功 (原版 IsSetup 前置)
    if !Self::is_active() {
      return Ok(());
    }
    // 激活态: 沙箱外内存以 out-of-sandbox key 全禁 (limit 寻址暂不启用, 登记后续梯队)
    let out_key = OUT_OF_SANDBOX_PKEY.load(Ordering::Relaxed);
    MemoryProtectionKey::set_key_permissions(out_key, Permission::DisableAccess);
    Ok(())
  }

  /// 原版 RegisterOutOfSandboxMemory (hardware-support.cc:84-90)
  ///
  /// # Safety
  ///
  /// addr+size 须落在合法映射内。
  pub unsafe fn register_out_of_sandbox_memory(
    addr: usize,
    size: usize,
    permissions: v8_base_platform::MemoryPermission,
  ) -> Result<()> {
    if !Self::is_active() {
      return Ok(());
    }
    let key = OUT_OF_SANDBOX_PKEY.load(Ordering::Relaxed);
    // SAFETY: 调用方保证区间合法。
    unsafe {
      MemoryProtectionKey::set_permissions_and_key(addr as *mut _, size, permissions, key)
    }?;
    Ok(())
  }

  /// 原版 RegisterUnsafeSandboxExtensionMemory (hardware-support.cc:92-98)
  ///
  /// # Safety
  ///
  /// addr+size 须落在合法映射内。
  pub unsafe fn register_unsafe_sandbox_extension_memory(addr: usize, size: usize) -> Result<()> {
    if !Self::is_active() {
      return Ok(());
    }
    let key = EXTENSION_PKEY.load(Ordering::Relaxed);
    // SAFETY: 调用方保证区间合法。
    unsafe {
      MemoryProtectionKey::set_permissions_and_key(
        addr as *mut _,
        size,
        MemoryPermission::ReadWrite,
        key,
      )
    }?;
    Ok(())
  }

  /// 原版 RegisterReadOnlyMemoryInsideSandbox (hardware-support.cc:101-110)
  ///
  /// # Safety
  ///
  /// addr+size 须落在合法映射内。
  pub unsafe fn register_read_only_memory_inside_sandbox(
    addr: usize,
    size: usize,
    current_permissions: v8_base_platform::MemoryPermission,
  ) -> Result<()> {
    if !Self::is_active() {
      return Ok(());
    }
    let key = SANDBOX_PKEY.load(Ordering::Relaxed);
    // SAFETY: 调用方保证区间合法。
    unsafe {
      MemoryProtectionKey::set_permissions_and_key(addr as *mut _, size, current_permissions, key)
    }?;
    Ok(())
  }

  /// 原版 EnterSandboxedExecutionModeForCurrentThread (hardware-support.cc:113-120):
  /// 沙箱内代码禁访沙箱外 (沙箱 key 全禁)
  pub fn enter_sandboxed_execution_mode_for_current_thread() -> Result<()> {
    if !Self::is_active() {
      return Ok(());
    }
    let sandbox_key = SANDBOX_PKEY.load(Ordering::Relaxed);
    MemoryProtectionKey::set_key_permissions(sandbox_key, Permission::DisableAccess);
    Ok(())
  }

  /// 原版 ExitSandboxedExecutionModeForCurrentThread (hardware-support.cc:123-130)
  pub fn exit_sandboxed_execution_mode_for_current_thread() -> Result<()> {
    if !Self::is_active() {
      return Ok(());
    }
    let sandbox_key = SANDBOX_PKEY.load(Ordering::Relaxed);
    MemoryProtectionKey::set_key_permissions(sandbox_key, Permission::NoRestrictions);
    Ok(())
  }

  /// 原版 CurrentSandboxingMode (hardware-support.cc:133-145)
  pub fn current_sandboxing_mode() -> CodeSandboxingMode {
    if !Self::is_active() {
      return CodeSandboxingMode::Unsandboxed;
    }
    let sandbox_key = SANDBOX_PKEY.load(Ordering::Relaxed);
    if MemoryProtectionKey::get_key_permission(sandbox_key) == Permission::DisableAccess {
      CodeSandboxingMode::Sandboxed
    } else {
      CodeSandboxingMode::Unsandboxed
    }
  }

  /// 原版 CurrentSandboxingModeIs (hardware-support.cc:148-152): 未启用时恒 true
  pub fn current_sandboxing_mode_is(expected: CodeSandboxingMode) -> bool {
    if !Self::is_active() {
      return true;
    }
    Self::current_sandboxing_mode() == expected
  }

  /// 原版 KernelSupportsSignalDeliveryInSandbox (hardware-support.cc:155-171):
  /// Linux 内核 >= 6.12; 非 Linux 平台不支持 (原版 UNREACHABLE → Err)
  pub fn kernel_supports_signal_delivery_in_sandbox() -> Result<bool> {
    #[cfg(target_os = "linux")]
    {
      Ok(kernel_version_at_least(6, 12))
    }
    #[cfg(not(target_os = "linux"))]
    {
      Err(crate::Error::Unsupported)
    }
  }

  /// 原版 sandboxed_mode_pkey_mask_address: 生成代码用掩码地址 (此处暴露值)
  pub fn sandboxed_mode_pkey_mask() -> u32 {
    SANDBOXED_MODE_PKEY_MASK.load(Ordering::Relaxed)
  }

  /// 原版 SandboxPkey()
  pub fn sandbox_pkey() -> i32 {
    SANDBOX_PKEY.load(Ordering::Relaxed)
  }
}

/// 解析 uname release, 判断内核版本 >= (major, minor)
#[cfg(target_os = "linux")]
fn kernel_version_at_least(major: u32, minor: u32) -> bool {
  use std::{mem::MaybeUninit, slice::from_raw_parts};
  // SAFETY: uname 写入 utsname 结构, 无其他副作用。
  let mut uts = MaybeUninit::<libc::utsname>::uninit();
  if unsafe { libc::uname(uts.as_mut_ptr()) } != 0 {
    return false;
  }
  // SAFETY: uname 成功后已初始化。
  let uts = unsafe { uts.assume_init() };
  let nul_pos = uts
    .release
    .iter()
    .position(|&c| c == 0)
    .unwrap_or(uts.release.len());
  // SAFETY: uts.release 在 0..nul_pos 均为有效 ASCII 字节
  let bytes = unsafe { from_raw_parts(uts.release.as_ptr().cast::<u8>(), nul_pos) };
  let mut parts = bytes.split(|&b| b == b'.');
  let parse = |part: Option<&[u8]>| -> u32 {
    let Some(part) = part else { return 0 };
    let mut num: u32 = 0;
    for &b in part {
      if b.is_ascii_digit() {
        num = num.saturating_mul(10).saturating_add((b - b'0') as u32);
      } else {
        break;
      }
    }
    num
  };
  let k_major = parts.next();
  let k_minor = parts.next();
  (parse(k_major), parse(k_minor)) >= (major, minor)
}

// ---- Enter/Exit + scope (hardware-support.h:179-212) ----
/// 原版 EnterSandbox: 进入沙箱执行模式 (禁写沙箱外)
pub fn enter_sandbox() -> Result<()> {
  SandboxHardwareSupport::enter_sandboxed_execution_mode_for_current_thread()
}

/// 原版 ExitSandbox
pub fn exit_sandbox() -> Result<()> {
  SandboxHardwareSupport::exit_sandboxed_execution_mode_for_current_thread()
}

/// 原版 EnterSandboxScope / ExitSandboxScope: RAII
pub struct EnterSandboxScope;
pub struct ExitSandboxScope;

impl EnterSandboxScope {
  /// 进入沙箱执行模式; Drop 时退出
  pub fn new() -> Result<Self> {
    enter_sandbox().map(|_| Self)
  }
}

impl Drop for EnterSandboxScope {
  fn drop(&mut self) {
    let _ = exit_sandbox();
  }
}

impl ExitSandboxScope {
  /// 退出沙箱执行模式; Drop 时重进
  pub fn new() -> Result<Self> {
    exit_sandbox().map(|_| Self)
  }
}

impl Drop for ExitSandboxScope {
  fn drop(&mut self) {
    let _ = enter_sandbox();
  }
}

// ---- DisallowSandboxAccess / AllowSandboxAccess (hardware-support.h:226-270, DEBUG 行为) ----
/// 原版 DisallowSandboxAccess: 存续期移除沙箱读写 (LIFO)
pub struct DisallowSandboxAccess {
  previous_permission: Option<Permission>,
}

impl DisallowSandboxAccess {
  /// 激活态时移除沙箱访问; 未激活 no-op (原版 DEBUG 前置)
  pub fn new(_reason: &str) -> Self {
    Self {
      previous_permission: set_sandbox_permissions_if_active(Permission::DisableAccess),
    }
  }
}

impl Drop for DisallowSandboxAccess {
  fn drop(&mut self) {
    restore_sandbox_permissions(self.previous_permission);
  }
}

/// 原版 AllowSandboxAccess: 对 DisallowSandboxAccess 的临时豁免
pub struct AllowSandboxAccess {
  previous_permission: Option<Permission>,
}

impl AllowSandboxAccess {
  /// 激活态时恢复沙箱读写; 未激活 no-op
  pub fn new(_reason: &str) -> Self {
    Self {
      previous_permission: set_sandbox_permissions_if_active(Permission::NoRestrictions),
    }
  }
}

impl Drop for AllowSandboxAccess {
  fn drop(&mut self) {
    restore_sandbox_permissions(self.previous_permission);
  }
}

/// 未激活返回 None (无副作用); 激活时改沙箱 key 权限并返回旧值
fn set_sandbox_permissions_if_active(permission: Permission) -> Option<Permission> {
  let key = SANDBOX_PKEY.load(Ordering::Relaxed);
  if key == K_NO_MEMORY_PROTECTION_KEY {
    return None;
  }
  let previous = MemoryProtectionKey::get_key_permission(key);
  MemoryProtectionKey::set_key_permissions(key, permission);
  Some(previous)
}

/// 恢复旧权限; None = 未激活时无操作
fn restore_sandbox_permissions(previous: Option<Permission>) {
  if let Some(permission) = previous {
    let key = SANDBOX_PKEY.load(Ordering::Relaxed);
    if key != K_NO_MEMORY_PROTECTION_KEY {
      MemoryProtectionKey::set_key_permissions(key, permission);
    }
  }
}
