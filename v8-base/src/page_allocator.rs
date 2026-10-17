//! v8/src/base/page-allocator.h/cc 的转写 — OS 页分配器
//! 原版继承 ::v8::PageAllocator (include/v8-platform.h) 接口 → 本模块
//! `PageAllocator` trait; 具体实现委托 platform-posix.cc 的 OS:: 原语,
//! 此处以 libc (mmap/mprotect/madvise/munmap) 直接落地 (v8-base 不依赖
//! v8-base-platform, 差异登记于 lib.rs)
//! 差异登记: SharedMemory 族 (AllocateSharedPages/RemapShared, 原版仅 Linux +
//! INTERNAL ONLY) 与 ReserveForSharedMemoryMapping 未迁移, 上层需要时补建

use core::ptr;
#[cfg(unix)]
use std::io::Error;
use std::{
  ffi::c_void,
  sync::{Mutex, OnceLock},
};

use fastrand::Rng;
#[cfg(unix)]
use libc::{
  MAP_ANONYMOUS, MAP_FAILED, MAP_FIXED, MAP_NORESERVE, MAP_PRIVATE, PROT_EXEC, PROT_NONE,
  PROT_READ, PROT_WRITE, c_int, mmap, mprotect, munmap,
};

#[cfg(windows)]
mod win {
  use core::ffi::c_void;
  pub const MEM_COMMIT: u32 = 0x0000_1000;
  pub const MEM_RESERVE: u32 = 0x0000_2000;
  pub const MEM_DECOMMIT: u32 = 0x0000_4000;
  pub const MEM_RELEASE: u32 = 0x0000_8000;
  pub const PAGE_NOACCESS: u32 = 0x01;
  pub const PAGE_READONLY: u32 = 0x02;
  pub const PAGE_READWRITE: u32 = 0x04;
  pub const PAGE_EXECUTE_READ: u32 = 0x20;
  pub const PAGE_EXECUTE_READWRITE: u32 = 0x40;

  unsafe extern "system" {
    pub fn VirtualAlloc(
      lpAddress: *mut c_void,
      dwSize: usize,
      flAllocationType: u32,
      flProtect: u32,
    ) -> *mut c_void;
    pub fn VirtualFree(lpAddress: *mut c_void, dwSize: usize, dwFreeType: u32) -> i32;
    pub fn VirtualProtect(
      lpAddress: *mut c_void,
      dwSize: usize,
      flNewProtect: u32,
      lpflOldProtect: *mut u32,
    ) -> i32;
  }
}

/// 内存页权限 (原版 v8::PageAllocator::Permission, 枚举值与 OS::MemoryPermission 对齐)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Permission {
  /// 不可访问
  NoAccess = 0,
  /// 只读
  Read = 1,
  /// 读写
  ReadWrite = 2,
  /// 读写执行
  ReadWriteExecute = 3,
  /// 只读执行
  ReadExecute = 4,
  /// 预留未来需要 ReadWriteExecute 的内存 (Apple Silicon 上设置 MAP_JIT)
  NoAccessWillJitLater = 5,
}

/// 页分配器接口 (原版 include/v8-platform.h 的 v8::PageAllocator 纯虚集)
/// 地址统一以 usize (base::Address) 表达, 分配失败以 Option/bool 承接
pub trait PageAllocator {
  /// AllocatePages/FreePages 的页粒度 (地址与长度须为其倍数)
  fn allocate_page_size(&self) -> usize;

  /// SetPermissions/ReleasePages 的页粒度
  fn commit_page_size(&self) -> usize;

  /// 设置随机种子, 使 get_random_mmap_addr 产生可复现序列
  fn set_random_mmap_seed(&self, seed: i64);

  /// ASLR 下适合做 mmap hint 的随机地址 (按 allocate_page_size 对齐)
  fn get_random_mmap_addr(&self) -> usize;

  /// 以指定对齐与权限分配内存 (原版 void* 返回 → Option)
  fn allocate_pages(
    &self,
    hint: usize,
    length: usize,
    alignment: usize,
    permissions: Permission,
  ) -> Option<usize>;

  /// 释放 allocate_pages 分配的内存
  fn free_pages(&self, address: usize, length: usize) -> bool;

  /// 释放 allocate_pages 分配内存的尾部 [address+new_length, address+length)
  fn release_pages(&self, address: usize, length: usize, new_length: usize) -> bool;

  /// 设置已分配区间的权限
  fn set_permissions(&self, address: usize, length: usize, permissions: Permission) -> bool;

  /// 重新提交已丢弃的页 (须以原权限)
  fn recommit_pages(&self, address: usize, length: usize, permissions: Permission) -> bool;

  /// 向 OS 提示区间内页面不再需要 (下次访问透明恢复)
  fn discard_system_pages(&self, address: usize, size: usize) -> bool;

  /// 归档区间: OS 回收物理页并标记不可访问, 内容保证再次访问时为零
  fn decommit_pages(&self, address: usize, size: usize) -> bool;

  /// 封锁区间 (禁止改权限/解除映射), 平台不支持时返回 false
  fn seal_pages(&self, address: usize, length: usize) -> bool;

  /// 是否支持共享页分配 (默认不支持, 原版仅 Linux)
  fn can_allocate_shared_pages(&self) -> bool {
    false
  }
}

// ---------------------------------------------------------------------------
// OS 原语 (原版 platform-posix.cc 的 base::/OS:: 直调, 本模块私有化)
// ---------------------------------------------------------------------------

/// macOS arm64 固定 16K 页 (原版 kAppleArmPageSize; 其余走 sysconf)
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
const APPLE_ARM_PAGE_SIZE: usize = 1 << 14;

/// 进程级随机源 (原版 GetPlatformRandomNumberGenerator + rng_mutex;
/// 首次未播种时用系统熵, fastrand 承接 RandomNumberGenerator)
static RNG: Mutex<Option<fastrand::Rng>> = Mutex::new(None);

/// OS::AllocatePageSize
pub fn os_allocate_page_size() -> usize {
  #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
  {
    // 跨构建快照生成时 sysconf 返回宿主页大小, 目标页大小固定 16K (原版注释)
    APPLE_ARM_PAGE_SIZE
  }
  #[cfg(windows)]
  {
    4096
  }
  #[cfg(all(not(windows), not(all(target_os = "macos", target_arch = "aarch64"))))]
  {
    // SAFETY: sysconf 只读内核参数
    unsafe { libc::sysconf(libc::_SC_PAGESIZE) as usize }
  }
}

/// OS::CommitPageSize — posix 上与分配页大小一致
pub fn os_commit_page_size() -> usize {
  os_allocate_page_size()
}

/// Permission → mprotect prot (原版 GetProtectionFromMemoryPermission)
#[cfg(unix)]
const fn prot_of(access: Permission) -> c_int {
  match access {
    Permission::NoAccess | Permission::NoAccessWillJitLater => PROT_NONE,
    Permission::Read => PROT_READ,
    Permission::ReadWrite => PROT_READ | PROT_WRITE,
    Permission::ReadWriteExecute => PROT_READ | PROT_WRITE | PROT_EXEC,
    Permission::ReadExecute => PROT_READ | PROT_EXEC,
  }
}

#[cfg(windows)]
const fn prot_of(access: Permission) -> u32 {
  match access {
    Permission::NoAccess | Permission::NoAccessWillJitLater => win::PAGE_NOACCESS,
    Permission::Read => win::PAGE_READONLY,
    Permission::ReadWrite => win::PAGE_READWRITE,
    Permission::ReadWriteExecute => win::PAGE_EXECUTE_READWRITE,
    Permission::ReadExecute => win::PAGE_EXECUTE_READ,
  }
}

/// base::GetFlagsForMemoryPermission (私有匿名映射路径)
#[cfg(unix)]
fn flags_of(access: Permission) -> c_int {
  let mut flags = MAP_PRIVATE | MAP_ANONYMOUS;
  if matches!(
    access,
    Permission::NoAccess | Permission::NoAccessWillJitLater
  ) {
    flags |= MAP_NORESERVE;
  }
  #[cfg(target_os = "macos")]
  {
    // 强化运行时/Apple Silicon 上取得可写可执行页需要 MAP_JIT (原版注释)
    if matches!(
      access,
      Permission::NoAccessWillJitLater | Permission::ReadWriteExecute
    ) {
      flags |= libc::MAP_JIT;
    }
  }
  flags
}

/// base::Allocate — 裸 mmap (匿名私有)
fn os_mmap(hint: usize, size: usize, access: Permission) -> Option<usize> {
  #[cfg(unix)]
  {
    #[cfg(target_os = "macos")]
    let fd: c_int = 255 << 24; // VM_MAKE_TAG(255): 内存分析工具识别 V8 区域
    #[cfg(not(target_os = "macos"))]
    let fd: c_int = -1;
    // SAFETY: hint 仅作提示 (无 MAP_FIXED); size 为页倍数由调用方保证
    let ret = unsafe {
      mmap(
        hint as *mut c_void,
        size,
        prot_of(access),
        flags_of(access),
        fd,
        0,
      )
    };
    if ptr::eq(ret, MAP_FAILED) {
      None
    } else {
      Some(ret as usize)
    }
  }
  #[cfg(windows)]
  {
    let alloc_type = if access == Permission::NoAccess {
      win::MEM_RESERVE
    } else {
      win::MEM_COMMIT | win::MEM_RESERVE
    };
    let mut ret =
      unsafe { win::VirtualAlloc(hint as *mut c_void, size, alloc_type, prot_of(access)) };
    if ret.is_null() && hint != 0 {
      ret = unsafe { win::VirtualAlloc(ptr::null_mut(), size, alloc_type, prot_of(access)) };
    }
    if ret.is_null() {
      None
    } else {
      Some(ret as usize)
    }
  }
  #[cfg(not(any(unix, windows)))]
  {
    let _ = (hint, size, access);
    None
  }
}

/// OS::Free / OS::Release — munmap
pub fn os_munmap(address: usize, size: usize) -> bool {
  debug_assert_eq!(address % os_allocate_page_size(), 0);
  debug_assert_eq!(size % os_allocate_page_size(), 0);
  #[cfg(unix)]
  {
    // SAFETY: 调用方契约保证 [address, address+size) 为本分配器映射的区间
    let ret = unsafe { munmap(address as *mut c_void, size) };
    if ret != 0 {
      log::warn!(
        "munmap({address:#x}, {size:#x}) 失败: {}",
        Error::last_os_error()
      );
    }
    ret == 0
  }
  #[cfg(windows)]
  {
    let _ = size;
    let ret = unsafe { win::VirtualFree(address as *mut c_void, 0, win::MEM_RELEASE) };
    ret != 0
  }
  #[cfg(not(any(unix, windows)))]
  {
    let _ = (address, size);
    false
  }
}

/// OS::SetPermissions — mprotect + darwin MADV_FREE 兜底
pub fn os_set_permissions(address: usize, size: usize, access: Permission) -> bool {
  debug_assert_eq!(address % os_commit_page_size(), 0);
  debug_assert_eq!(size % os_commit_page_size(), 0);
  #[cfg(target_os = "macos")]
  let mut ret = unsafe { mprotect(address as *mut c_void, size, prot_of(access)) };
  #[cfg(all(unix, not(target_os = "macos")))]
  let ret = unsafe { mprotect(address as *mut c_void, size, prot_of(access)) };
  #[cfg(windows)]
  let ret = {
    let mut old_prot = 0u32;
    let prot = prot_of(access);
    if unsafe { win::VirtualProtect(address as *mut c_void, size, prot, &mut old_prot) } != 0 {
      0
    } else if access != Permission::NoAccess {
      // 原版 V8 platform-win32.cc: 预留但尚未提交的内存区, 触发 MEM_COMMIT
      let commit_ptr =
        unsafe { win::VirtualAlloc(address as *mut c_void, size, win::MEM_COMMIT, prot) };
      if !commit_ptr.is_null() { 0 } else { -1 }
    } else {
      -1
    }
  };
  #[cfg(not(any(unix, windows)))]
  let ret = {
    let _ = (address, size, access);
    -1
  };
  #[cfg(target_os = "macos")]
  {
    // macOS 11.2 (Apple Silicon) 拒绝 rwx→none 切换, 改用 madvise (原版同款)
    if ret != 0 && access == Permission::NoAccess {
      // SAFETY: 同上
      ret = unsafe { libc::madvise(address as *mut c_void, size, libc::MADV_FREE_REUSABLE) };
      return ret == 0;
    }
  }
  if ret == 0 && access == Permission::NoAccess {
    // 顺带丢弃物理页 (advisory, 忽略错误, 原版 USE(...))
    os_discard_system_pages(address, size);
  }
  #[cfg(target_os = "macos")]
  {
    // 计账需要: 脱离 NoAccess 后 MADV_FREE_REUSE (原版 TODO 同款)
    if access != Permission::NoAccess {
      // SAFETY: 同上
      unsafe { libc::madvise(address as *mut c_void, size, libc::MADV_FREE_REUSE) };
    }
  }
  ret == 0
}

/// OS::RecommitPages — darwin 上 MADV_FREE_REUSE (重试 EAGAIN)
pub fn os_recommit_pages(address: usize, size: usize, _access: Permission) -> bool {
  debug_assert_eq!(address % os_commit_page_size(), 0);
  debug_assert_eq!(size % os_commit_page_size(), 0);
  #[cfg(target_os = "macos")]
  {
    // SAFETY: 同上
    let mut ret = unsafe { libc::madvise(address as *mut c_void, size, libc::MADV_FREE_REUSE) };
    while ret == -1 && Error::last_os_error().raw_os_error() == Some(libc::EAGAIN) {
      // SAFETY: 同上
      ret = unsafe { libc::madvise(address as *mut c_void, size, libc::MADV_FREE_REUSE) };
    }
  }
  #[cfg(not(target_os = "macos"))]
  {
    let _ = (address, size);
  }
  true
}

/// OS::DiscardSystemPages — 丢弃物理页 (内容变为未定义/零)
pub fn os_discard_system_pages(address: usize, size: usize) -> bool {
  debug_assert_eq!(address % os_commit_page_size(), 0);
  debug_assert_eq!(size % os_commit_page_size(), 0);
  #[cfg(target_os = "macos")]
  {
    // MADV_FREE_REUSABLE 带 reusable 位, 供活动监视器/内存分析正确计账;
    // 失败时回退 MADV_DONTNEED (原版同款)
    // SAFETY: 调用方契约保证区间有效
    let mut ret = unsafe { libc::madvise(address as *mut c_void, size, libc::MADV_FREE_REUSABLE) };
    while ret != 0 && Error::last_os_error().raw_os_error() == Some(libc::EAGAIN) {
      // SAFETY: 同上
      ret = unsafe { libc::madvise(address as *mut c_void, size, libc::MADV_FREE_REUSABLE) };
    }
    if ret != 0 {
      // SAFETY: 同上
      ret = unsafe { libc::madvise(address as *mut c_void, size, libc::MADV_DONTNEED) };
    }
    ret == 0
  }
  #[cfg(all(unix, not(target_os = "macos")))]
  {
    // SAFETY: 同上
    let ret = unsafe { libc::madvise(address as *mut c_void, size, libc::MADV_DONTNEED) };
    ret == 0
  }
  #[cfg(windows)]
  {
    let _ = (address, size);
    true
  }
}

/// OS::DecommitPages — MAP_FIXED 重映射匿名 PROT_NONE (再次访问保证为零)
pub fn os_decommit_pages(address: usize, size: usize) -> bool {
  debug_assert_eq!(address % os_commit_page_size(), 0);
  debug_assert_eq!(size % os_commit_page_size(), 0);
  #[cfg(unix)]
  {
    // SAFETY: MAP_FIXED 覆盖 [address, address+size) 的既有映射, 调用方契约保证区间有效
    let ret = unsafe {
      mmap(
        address as *mut c_void,
        size,
        PROT_NONE,
        MAP_FIXED | MAP_ANONYMOUS | MAP_PRIVATE,
        -1,
        0,
      )
    };
    if ptr::eq(ret, MAP_FAILED) {
      // VMA 数超限等场景失败 (原版 CHECK_EQ(ENOMEM) → Rust 记日志返回 false)
      log::warn!(
        "decommit mmap({address:#x}, {size:#x}) 失败: {}",
        Error::last_os_error()
      );
      return false;
    }
    debug_assert_eq!(ret as usize, address);
    true
  }
  #[cfg(windows)]
  {
    let ret = unsafe { win::VirtualFree(address as *mut c_void, size, win::MEM_DECOMMIT) };
    ret != 0
  }
  #[cfg(not(any(unix, windows)))]
  {
    let _ = (address, size);
    false
  }
}

/// OS::SealPages — 结合系统平台（如 Linux mseal、POSIX mprotect PROT_READ 锁读或 Windows PAGE_READONLY）提供真实内存页封锁判定
pub fn os_seal_pages(address: usize, size: usize) -> bool {
  let page_size = os_commit_page_size();
  debug_assert_eq!(address % page_size, 0);
  debug_assert_eq!(size % page_size, 0);
  if address == 0 || size == 0 {
    return false;
  }

  #[cfg(target_os = "linux")]
  {
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
    const SYS_MSEAL: libc::c_long = 462;
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    const SYS_MSEAL: libc::c_long = 0;

    if SYS_MSEAL != 0 {
      let ret = unsafe { libc::syscall(SYS_MSEAL, address, size, 0) };
      if ret == 0 {
        return true;
      }
    }
    let ret = unsafe { mprotect(address as *mut c_void, size, PROT_READ) };
    ret == 0
  }

  #[cfg(all(unix, not(target_os = "linux")))]
  {
    let ret = unsafe { mprotect(address as *mut c_void, size, PROT_READ) };
    ret == 0
  }

  #[cfg(windows)]
  {
    let mut old_prot = 0u32;
    let ret = unsafe {
      win::VirtualProtect(
        address as *mut c_void,
        size,
        win::PAGE_READONLY,
        &mut old_prot,
      )
    };
    ret != 0
  }

  #[cfg(not(any(unix, windows)))]
  {
    let _ = (address, size);
    false
  }
}

/// OS::SetMemoryRegionName — 匿名内存区命名 (仅 Linux prctl 生效, 原版同款)
pub fn os_set_memory_region_name(address: usize, size: usize, name: &str) -> bool {
  #[cfg(target_os = "linux")]
  {
    // 内核 uapi: PR_SET_VMA = 0x53564d41 ("VMA"), PR_SET_VMA_ANON_NAME = 0
    const PR_SET_VMA: libc::c_int = 0x5356_4d41;
    const PR_SET_VMA_ANON_NAME: libc::c_ulong = 0;
    // 原版要求名称仅含字母数字与 '-', 调用方已校验; NUL 结尾追加
    let buf = format!("{name}\0");
    // SAFETY: prctl 只读用户参数; 名称指针指向本栈合法 C 字符串
    let ret = unsafe {
      libc::prctl(
        PR_SET_VMA,
        PR_SET_VMA_ANON_NAME,
        address,
        size,
        buf.as_ptr(),
      )
    };
    ret == 0
  }
  #[cfg(not(target_os = "linux"))]
  {
    let _ = (address, size, name);
    false
  }
}

/// OS::GetRandomMmapAddr — 随机 mmap hint 地址
pub fn os_get_random_mmap_addr() -> usize {
  // SAFETY: 仅持锁读改进程级 RNG
  let raw = {
    let mut guard = RNG.lock().unwrap_or_else(|e| e.into_inner());
    let rng = guard.get_or_insert_with(Rng::new);
    rng.u64(0..=u64::MAX)
  };
  let mut raw = raw as usize;
  #[cfg(target_arch = "aarch64")]
  {
    let page_size = os_allocate_page_size();
    // 保持页对齐 (AArch64 支持 4K/16K/64K 配置)
    raw &= !(page_size - 1);
  }
  // x64 与 arm64(macOS) 均截断到 46 位, 给内核满足放置请求的机会 (原版同款)
  #[cfg(any(
    target_arch = "x86_64",
    all(target_arch = "aarch64", target_os = "macos")
  ))]
  {
    raw &= 0x3FFF_FFFF_F000;
  }
  #[cfg(all(target_arch = "aarch64", target_os = "linux"))]
  {
    // Linux arm64 默认 39 位虚拟空间, 截断到 38 位
    raw &= 0x3F_FFFF_F000;
  }
  raw
}

/// OS::SetRandomMmapSeed
pub fn os_set_random_mmap_seed(seed: i64) {
  if seed != 0 {
    // 原版: seed 为 0 时保持现 generator
    // SAFETY: 仅持锁替换 RNG
    let mut guard = RNG.lock().unwrap_or_else(|e| e.into_inner());
    *guard = Some(Rng::with_seed(seed as u64));
  }
}

// ---------------------------------------------------------------------------
// 具体实现 (原版 v8::base::PageAllocator)
// ---------------------------------------------------------------------------

/// 默认页分配器: 直接走 OS mmap 族 (原版 base::PageAllocator 的 Rust 承接)
#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultPageAllocator;

impl PageAllocator for DefaultPageAllocator {
  fn allocate_page_size(&self) -> usize {
    os_allocate_page_size()
  }

  fn commit_page_size(&self) -> usize {
    os_commit_page_size()
  }

  fn set_random_mmap_seed(&self, seed: i64) {
    os_set_random_mmap_seed(seed);
  }

  fn get_random_mmap_addr(&self) -> usize {
    os_get_random_mmap_addr()
  }

  fn allocate_pages(
    &self,
    hint: usize,
    length: usize,
    alignment: usize,
    permissions: Permission,
  ) -> Option<usize> {
    let page_size = os_allocate_page_size();
    debug_assert_eq!(length % page_size, 0);
    debug_assert_eq!(alignment % page_size, 0);
    // 非 Apple Silicon JIT 写保护平台: WillJitLater 映射为普通 NoAccess
    // (避免下游处理两种枚举值, 原版同款)
    let permissions = if !cfg!(any(all(target_arch = "aarch64", target_os = "macos"),))
      && permissions == Permission::NoAccessWillJitLater
    {
      Permission::NoAccess
    } else {
      permissions
    };
    #[cfg(windows)]
    {
      // Windows VirtualAlloc 默认按系统分配粒度 (64KB) 对齐。
      // 若对齐需求 <= page_size，直分路径零切分。
      if alignment <= page_size {
        return os_mmap(hint, length, permissions);
      }

      // Windows 无法进行局部 munmap，大对齐走 V8 原版 platform-win32.cc“预留-释放-精准重占”策略
      let hint = hint & !(alignment - 1);
      if let Some(base) = os_mmap(hint, length, permissions) {
        if base % alignment == 0 {
          return Some(base);
        }
        os_munmap(base, length);
      }

      let padded_size = (length + (alignment - page_size)).div_ceil(page_size) * page_size;
      for _ in 0..3 {
        let base = os_mmap(0, padded_size, permissions)?;
        let aligned_base = base.div_ceil(alignment) * alignment;
        os_munmap(base, padded_size);

        // 尝试在探测出的对齐基址上精确申请
        if let Some(realloc) = os_mmap(aligned_base, length, permissions) {
          if realloc == aligned_base {
            return Some(aligned_base);
          }
          os_munmap(realloc, length);
        }
      }
      return None;
    }
    #[cfg(not(windows))]
    {
      // OS::Allocate: hint 先向下对齐, 申请量加上最大错位以保证对齐基址
      let hint = hint & !(alignment - 1);
      let mut request_size = (length + (alignment - page_size)).div_ceil(page_size) * page_size;
      let base = os_mmap(hint, request_size, permissions)?;

      // 释放对齐基址之前的多余映射
      let aligned_base = base.div_ceil(alignment) * alignment;
      if aligned_base != base {
        let prefix_size = aligned_base - base;
        os_munmap(base, prefix_size);
        request_size -= prefix_size;
      }
      // 释放未对齐尾部的多余映射
      if length != request_size {
        let suffix_size = request_size - length;
        os_munmap(aligned_base + length, suffix_size);
      }
      Some(aligned_base)
    }
  }

  fn free_pages(&self, address: usize, length: usize) -> bool {
    os_munmap(address, length)
  }

  fn release_pages(&self, address: usize, length: usize, new_length: usize) -> bool {
    debug_assert!(new_length < length);
    os_munmap(address + new_length, length - new_length)
  }

  fn set_permissions(&self, address: usize, length: usize, permissions: Permission) -> bool {
    os_set_permissions(address, length, permissions)
  }

  fn recommit_pages(&self, address: usize, length: usize, permissions: Permission) -> bool {
    os_recommit_pages(address, length, permissions)
  }

  fn discard_system_pages(&self, address: usize, size: usize) -> bool {
    os_discard_system_pages(address, size)
  }

  fn decommit_pages(&self, address: usize, size: usize) -> bool {
    os_decommit_pages(address, size)
  }

  fn seal_pages(&self, address: usize, length: usize) -> bool {
    os_seal_pages(address, length)
  }

  fn can_allocate_shared_pages(&self) -> bool {
    // 原版: 仅 Linux 支持共享页 (memfd 路径), 本转写未迁移 SharedMemory 族
    cfg!(target_os = "linux")
  }
}

/// 进程级默认分配器单例 (原版各处使用全局 PageAllocator 的承接;
/// ZST 拷贝零开销, OnceLock 仅提供稳定全局地址)
pub fn global_page_allocator() -> &'static DefaultPageAllocator {
  static GLOBAL: OnceLock<DefaultPageAllocator> = OnceLock::new();
  GLOBAL.get_or_init(DefaultPageAllocator::default)
}
