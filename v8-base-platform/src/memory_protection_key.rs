//! MemoryProtectionKey (PKR/PKU)：对应 V8 `memory-protection-key.{h,cc}`。
//!
//! 偏差：仅覆盖 `all(target_os = "linux", target_arch = "x86_64")`；
//! 其他平台所有 API 返回 `Unsupported` / 恒 `false`。原实现的 signal handler
//! 与栈保护 (`SetKeyForCurrentThreadsStack`, `SetDefaultPermissionsForAllKeysInSignalHandler`)
//! 依赖 v8-base 的 trap-handler/signal-handling 子系统——该模块尚未转写（readme 登记），保持不收。

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
use std::arch::asm;
use std::ffi::c_void;

use crate::{
  error::{Error, Result},
  os::MemoryPermission,
};

/// 与 V8 `kNoMemoryProtectionKey = -1` 对应。
pub const K_NO_MEMORY_PROTECTION_KEY: i32 = -1;
/// 与 V8 `kDefaultProtectionKey = 0` 对应。
pub const K_DEFAULT_PROTECTION_KEY: i32 = 0;

/// 与 V8 `MemoryProtectionKey::Permission` 对应。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Permission {
  NoRestrictions = 0,
  DisableAccess = 1,
  DisableWrite = 2,
}

/// PKRU 位图；参见 Intel SDM 13.1：每 2 位对应一个 pkey。
/// 仅 linux x64 的 `get_key_permission` 使用。
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
#[inline]
fn pkru_mask_for_key(key: i32) -> u32 {
  1u32 << ((key as u32) * 2)
}
/// 与 V8 `MemoryProtectionKey` 对应。
pub struct MemoryProtectionKey;

impl MemoryProtectionKey {
  #[inline]
  pub fn has_memory_protection_key_apis() -> bool {
    // 原版是弱符号存在性廉价检查（memory-protection-key.cc:46-52），明确
    // 「不测 CPU/内核支持」——那属于 TestKeyAllocation。Rust libc 静态绑定
    // glibc ≥ 2.9 后 pkey_* 符号恒存在，等价于「API 可用」。
    cfg!(all(target_os = "linux", target_arch = "x86_64"))
  }

  /// `SetPermissionsAndKey` 的平台实现（linux x64: `pkey_mprotect`）。
  #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
  unsafe fn set_permissions_and_key_impl(
    addr: *mut c_void,
    size: usize,
    access: MemoryPermission,
    key: i32,
  ) -> Result<()> {
    unsafe extern "C" {
      fn pkey_mprotect(
        addr: *mut c_void,
        len: usize,
        prot: libc::c_int,
        pkey: libc::c_int,
      ) -> libc::c_int;
    }

    let prot = match access {
      MemoryPermission::NoAccess | MemoryPermission::NoAccessWillJitLater => libc::PROT_NONE,
      MemoryPermission::Read => libc::PROT_READ,
      MemoryPermission::ReadWrite => libc::PROT_READ | libc::PROT_WRITE,
      MemoryPermission::ReadExecute => libc::PROT_READ | libc::PROT_EXEC,
      MemoryPermission::ReadWriteExecute => libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC,
    };
    // SAFETY: 由调用方保证范围。
    let r = unsafe { pkey_mprotect(addr, size, prot, key) };
    if r == 0 {
      Ok(())
    } else {
      Err(Error::MProtectFailed)
    }
  }

  /// 非 linux x64：MPK 不可用（原版 V8_HAS_PKU_SUPPORT 关闭时整类不编译）。
  #[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
  unsafe fn set_permissions_and_key_impl(
    addr: *mut c_void,
    size: usize,
    access: MemoryPermission,
    key: i32,
  ) -> Result<()> {
    let _ = (addr, size, access, key);
    Err(Error::Unsupported)
  }

  /// 对应 V8 `AllocateKey()`。
  pub fn allocate_key() -> Result<i32> {
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    {
      // SAFETY: pkey_alloc(flags=0, access_rights=Permission::NoRestrictions)。
      let r = unsafe {
        libc::syscall(
          libc::SYS_pkey_alloc,
          0u64,
          Permission::NoRestrictions as u64,
        )
      };
      if r < 0 {
        Err(Error::Unsupported)
      } else {
        Ok(r as i32)
      }
    }
    #[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
    {
      Err(Error::Unsupported)
    }
  }

  /// 对应 V8 `FreeKey(key)`。
  pub fn free_key(key: i32) -> Result<()> {
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    {
      // SAFETY: pkey_free；调用方保证 key 合法。
      let r = unsafe { libc::syscall(libc::SYS_pkey_free, key as u64) };
      if r == 0 {
        Ok(())
      } else {
        Err(Error::Unsupported)
      }
    }
    #[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
    {
      let _ = key;
      Err(Error::Unsupported)
    }
  }

  /// 对应 V8 `TestKeyAllocation()`：分配一个临时 key 并立即释放。
  pub fn test_key_allocation() -> bool {
    match Self::allocate_key() {
      Ok(k) => {
        let _ = Self::free_key(k);
        true
      }
      Err(_) => false,
    }
  }

  /// 对应 V8 `SetPermissionsAndKey(region, permissions, key)`；走 `pkey_mprotect`
  /// （memory-protection-key.h:105-106 / cc:99-115）：一次性设置页权限并关联 key。
  ///
  /// # Safety
  /// `addr`+`size` 必须落在合法映射内。
  pub unsafe fn set_permissions_and_key(
    addr: *mut c_void,
    size: usize,
    access: MemoryPermission,
    key: i32,
  ) -> Result<()> {
    // SAFETY: 由调用方保证。
    unsafe { Self::set_permissions_and_key_impl(addr, size, access, key) }
  }

  /// 对应 V8 `SetPermissionsForKey(key, permissions)`（memory-protection-key.h:110
  /// / cc:118-126）：改当前线程 PKRU 中该 key 的 2 个权限位（wrpkru 读改写），
  /// 与地址无关。key 必须是已分配的合法 key（原版 DCHECK_NE(kNoMemoryProtectionKey)）。
  pub fn set_key_permissions(key: i32, permission: Permission) {
    debug_assert!(
      key != K_NO_MEMORY_PROTECTION_KEY,
      "DCHECK_NE(kNoMemoryProtectionKey, key)"
    );
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    {
      // 每 key 占 PKRU 2 位：bit(2k)=DisableAccess, bit(2k+1)=DisableWrite。
      let shift = (key as u32) * 2;
      let mut pkru = read_pkru();
      pkru &= !(0b11 << shift);
      pkru |= (permission as u32) << shift;
      write_pkru(pkru);
    }
    #[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
    {
      let _ = (key, permission);
    }
  }

  /// 对应 V8 `GetKeyPermission(key)`：从当前 PKRU 读出 2 位。
  pub fn get_key_permission(key: i32) -> Permission {
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    {
      let pkru = read_pkru();
      let mask = pkru_mask_for_key(key);
      let disable_access = (pkru & mask) != 0;
      let disable_write = (pkru & (mask << 1)) != 0;
      if disable_access {
        Permission::DisableAccess
      } else if disable_write {
        Permission::DisableWrite
      } else {
        Permission::NoRestrictions
      }
    }
    #[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
    {
      let _ = key;
      Permission::NoRestrictions
    }
  }

  /// 对应 V8 `ComputeRegisterMaskForPermissionSwitch(key, permission)`
  /// （memory-protection-key.h:122-123，cc:142-153）：
  /// 返回 `permission << (key * kBitsPerKey)`——为该 key 设置该权限需要置位的
  /// PKRU 位。`permission == NoRestrictions` 是调用方错误（原版 DCHECK）。
  pub fn compute_register_mask_for_permission_switch(key: i32, permission: Permission) -> u32 {
    debug_assert!(
      permission != Permission::NoRestrictions,
      "DCHECK_NE(permissions, kNoRestrictions)"
    );
    debug_assert!(key >= 0, "key 必须是已分配的合法 key");
    const K_BITS_PER_KEY: u32 = 2;
    (permission as u32) << (key as u32 * K_BITS_PER_KEY)
  }
}

/// 通过内联汇编读取 PKRU；仅 x86_64 linux 使用。
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
#[inline]
fn read_pkru() -> u32 {
  // SAFETY: `rdpkru` 是纯寄存器读，不改变内存。
  unsafe {
    let mut a: u32;
    let mut d: u32;
    asm!("rdpkru", out("eax") a, out("edx") d, lateout("ecx") _, in("ecx") 0u32,
      options(nomem, nostack, preserves_flags));
    let _ = d;
    a
  }
}

/// 通过内联汇编写入 PKRU；仅 x86_64 linux 使用。
/// 不加 `nomem`：PKRU 变更影响后续内存访问的可访问性，编译器不得跨此重排。
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
#[inline]
fn write_pkru(pkru: u32) {
  // SAFETY: `wrpkru` 纯寄存器写；SDM 要求 ECX/EDX 必须为零。
  unsafe {
    asm!("wrpkru", in("eax") pkru, in("ecx") 0u32, in("edx") 0u32, options(nostack));
  }
}
