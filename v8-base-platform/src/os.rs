//! OS：对应 `src/base/platform/platform.h` 的 `class OS` 与 `platform.cc / platform-posix.cc`。
//!
//! 偏差：
//! - `Initialize` / `GetGCFakeMMapFile` / `SetRandomMmapSeed` / `GetRandomMmapAddr`
//!   已回收：随机源与掩码规则复用 `v8-base::page_allocator`（进程级单例 RNG，
//!   与原版共用 GetPlatformRandomNumberGenerator 的方向一致）。
//! - `GetSharedLibraryAddresses` / `SharedMemoryHandle` 族 / `TimezoneCache` 族 /
//!   stdio 族等未实现项见 readme「推迟 API 登记」表（round1 补全）。
//! - Windows / AIX / BSD 分支本轮不覆盖。

#[cfg(unix)]
use core::mem::zeroed;
use std::{ffi::c_void, io, ptr::null_mut, thread, time::Duration};

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

use parking_lot::Mutex;
use v8_base::page_allocator::{os_get_random_mmap_addr, os_set_random_mmap_seed};

use crate::{
  error::{Error, Result},
  time::{Time, TimeDelta},
};

/// 与 V8 `OS::MemoryPermission` 对应。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryPermission {
  NoAccess,
  Read,
  ReadWrite,
  ReadWriteExecute,
  ReadExecute,
  NoAccessWillJitLater,
}

/// 与 V8 `OS::MemoryRange` 对应。
#[derive(Debug, Clone, Copy, Default)]
pub struct MemoryRange {
  pub start: usize,
  pub end: usize,
}

/// 进程级 fake mmap 名；原 `g_gc_fake_mmap`（platform-posix.cc:113）。
static GC_FAKE_MMAP: parking_lot::Mutex<Option<&'static str>> = Mutex::new(None);

/// 系统调用聚合；对应 `class OS` 静态方法集合。
pub struct OS;

#[inline]
fn align_up(v: usize, a: usize) -> usize {
  (v + a - 1) & !(a - 1)
}

impl OS {
  /// 系统页大小；对应 V8 `OS::CommitPageSize`。
  /// darwin arm64 = 16384；x86_64 = 4096；linux = 4096；由 `sysconf(_SC_PAGESIZE)` 获取。
  #[inline]
  pub fn commit_page_size() -> usize {
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    {
      // SAFETY: `sysconf` 无副作用；常量查询。
      let p = unsafe { libc::sysconf(libc::_SC_PAGESIZE) };
      if p <= 0 { 4096 } else { p as usize }
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
      4096
    }
  }

  /// V8 `OS::AllocatePageSize`。POSIX 与 commit 页一致；Windows 才有 granularity 差异。
  #[inline]
  pub fn allocate_page_size() -> usize {
    Self::commit_page_size()
  }

  #[inline]
  pub fn has_lazy_commits() -> bool {
    // 与 V8 POSIX 分支一致。
    true
  }

  /// 对应 V8 `OS::Sleep(TimeDelta)`；interval 视为非负。
  pub fn sleep(interval: TimeDelta) {
    let us = interval.in_microseconds();
    if us <= 0 {
      return;
    }
    let dur = Duration::from_micros(us as u64);
    thread::sleep(dur);
  }

  /// 对应 V8 `OS::Abort()`；直接调用 libc `abort`（不产生 Rust panic）。
  pub fn abort() -> ! {
    #[cfg(unix)]
    unsafe {
      libc::abort();
    }
    #[cfg(not(unix))]
    {
      std::process::abort();
    }
  }

  /// 对应 V8 `OS::DebugBreak()`；POSIX 下 raise(SIGTRAP)，Windows 下 DebugBreak()。
  pub fn debug_break() {
    #[cfg(unix)]
    unsafe {
      libc::raise(libc::SIGTRAP);
    }
    #[cfg(windows)]
    unsafe {
      unsafe extern "system" {
        fn DebugBreak();
      }
      DebugBreak();
    }
    #[cfg(not(any(unix, windows)))]
    {
      std::process::abort();
    }
  }

  /// 对应 V8 `OS::GetLastError()`；返回原始 errno。
  pub fn get_last_error() -> i32 {
    io::Error::last_os_error().raw_os_error().unwrap_or(0)
  }

  /// 对应 V8 `OS::TimeCurrentMillis()`（platform-posix.cc:984-986）：
  /// `return Time::Now().ToJsTime();`——含 null/max 哨兵分支。
  pub fn time_current_millis() -> f64 {
    Time::now().to_js_time()
  }

  #[inline]
  pub fn directory_separator() -> char {
    if cfg!(target_os = "windows") {
      '\\'
    } else {
      '/'
    }
  }

  /// 对应 V8 `OS::Initialize(gc_fake_mmap)`（platform-posix.cc:276-278）：
  /// 仅记录 GC 假 mmap 文件名，供日志/覆盖率工具读取。
  pub fn initialize(gc_fake_mmap: Option<&'static str>) {
    *GC_FAKE_MMAP.lock() = gc_fake_mmap;
  }

  /// 对应 V8 `OS::GetGCFakeMMapFile()`（platform-posix.cc:785-787）。
  pub fn get_gc_fake_mmap_file() -> Option<&'static str> {
    *GC_FAKE_MMAP.lock()
  }

  /// 对应 V8 `OS::SetRandomMmapSeed(seed)`（platform-posix.cc:355-360）。
  /// 进程级随机源已回收 → `v8-base::page_allocator`（与原版共用同一
  /// GetPlatformRandomNumberGenerator 的进程级单例语义一致）；seed 为 0 时保持现 generator。
  pub fn set_random_mmap_seed(seed: i64) {
    os_set_random_mmap_seed(seed);
  }

  /// 对应 V8 `OS::GetRandomMmapAddr()`（platform-posix.cc:363-395）；
  /// 随机 hint 地址的截断/对齐规则（46 位掩码、arm64 页对齐）由 v8-base 承接。
  pub fn get_random_mmap_addr() -> *mut c_void {
    os_get_random_mmap_addr() as *mut c_void
  }

  #[inline]
  pub fn is_directory_separator(ch: char) -> bool {
    ch == '/' || ch == '\\'
  }

  /// 对应 V8 `OS::GetCurrentProcessId()`。
  pub fn get_current_process_id() -> i32 {
    // SAFETY: `getpid` 是 POSIX 无副作用调用。
    unsafe { libc::getpid() }
  }

  /// 对应 V8 `OS::GetCurrentThreadId()`。
  pub fn get_current_thread_id() -> i32 {
    #[cfg(target_os = "macos")]
    {
      unsafe extern "C" {
        fn pthread_mach_thread_np(id: libc::pthread_t) -> u32;
      }
      // SAFETY: 只读；mach thread id 在进程内唯一。
      let m = unsafe { pthread_mach_thread_np(libc::pthread_self()) };
      m as i32
    }
    #[cfg(target_os = "linux")]
    {
      // SAFETY: `gettid` 系统调用无副作用。
      unsafe { libc::syscall(libc::SYS_gettid) as i32 }
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
      0
    }
  }

  /// 对应 V8 `OS::ActivationFrameAlignment()`；aarch64/x64 均为 16。
  pub fn activation_frame_alignment() -> i32 {
    if cfg!(any(target_arch = "x86_64", target_arch = "aarch64")) {
      16
    } else {
      0
    }
  }

  /// 对应 V8 `OS::GetPeakMemoryUsageKb`。macOS 上 `ru_maxrss` 为字节，Linux 为 KB。
  /// POSIX 分支：`getrusage(RUSAGE_SELF)`。
  pub fn get_peak_memory_usage_kb() -> i32 {
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    {
      let mut ru: libc::rusage = unsafe { zeroed() };
      // SAFETY: `getrusage` 是 POSIX 调用；栈局部结构。
      let rc = unsafe { libc::getrusage(libc::RUSAGE_SELF, &mut ru) };
      if rc != 0 {
        // 原版失败返回 -1（platform-posix.cc:971）。
        return -1;
      }
      #[cfg(target_os = "macos")]
      {
        // darwin: `ru_maxrss` 单位为字节。
        (ru.ru_maxrss / 1024) as i32
      }
      #[cfg(target_os = "linux")]
      {
        // linux: KB。
        ru.ru_maxrss as i32
      }
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
      0
    }
  }

  /// 对应 V8 `OS::GetUserTime(secs, usecs)`（platform-posix.cc:952-960）：
  /// `getrusage(RUSAGE_SELF)` 的 `ru_utime`（用户 CPU 时间，非墙钟）。
  pub fn get_user_time() -> (u32, u32) {
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    {
      let mut ru: libc::rusage = unsafe { zeroed() };
      // SAFETY: `getrusage` 是 POSIX 调用；栈局部结构。
      if unsafe { libc::getrusage(libc::RUSAGE_SELF, &mut ru) } != 0 {
        return (0, 0);
      }
      (ru.ru_utime.tv_sec as u32, ru.ru_utime.tv_usec as u32)
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
      (0, 0)
    }
  }

  // ---------------------------------------------------------------------------
  // 内存管理（POSIX）

  /// 对应 V8 `OS::Allocate(hint, size, alignment, access)`。
  ///
  /// # Safety
  /// 返回值指向 mmap 分配的裸内存；调用方负责 `Free` 且大小/对齐须匹配。
  pub unsafe fn allocate(
    hint: *mut c_void,
    size: usize,
    alignment: usize,
    access: MemoryPermission,
  ) -> Result<*mut c_void> {
    let page_size = Self::commit_page_size();
    debug_assert_eq!(0, size % page_size, "DCHECK_EQ(0, size % page_size)");
    let alignment = if alignment == 0 { page_size } else { alignment };
    if !alignment.is_power_of_two() || alignment < page_size {
      return Err(Error::InvalidArgument);
    }
    // 原版 `hint = AlignedAddress(hint, alignment)`（platform-posix.cc:471）：
    // hint 非空时向下对齐到 alignment。
    let hint = if hint.is_null() {
      hint
    } else {
      ((hint as usize) & !(alignment - 1)) as *mut c_void
    };
    #[cfg(unix)]
    {
      // 与 V8 一致：先请求 `size + (alignment - page_size)` 向上取整到页，之后再修剪两端。
      let request_size = align_up(size + (alignment - page_size), page_size);
      let prot = protection_from_permission(access);
      let flags = mmap_flags_for_access(access);
      let fd = mmap_fd_for_access(access);
      // SAFETY: 由调用方保证 hint 有效或为 null；mmap 返回值以 Result 传播。
      let base = unsafe { libc::mmap(hint, request_size, prot, flags, fd, 0) };
      if base == libc::MAP_FAILED {
        return Err(Error::MmapFailed);
      }
      let aligned = align_up(base as usize, alignment);
      let prefix = aligned - base as usize;
      let suffix = request_size - prefix - size;
      // SAFETY: 已知 base + prefix + size <= base + request_size，均在同一映射内。
      unsafe {
        if prefix > 0 {
          libc::munmap(base, prefix);
        }
        if suffix > 0 {
          libc::munmap((aligned + size) as *mut c_void, suffix);
        }
      }
      Ok(aligned as *mut c_void)
    }
    #[cfg(windows)]
    {
      let page_size = Self::allocate_page_size();
      let size = align_up(size, page_size);
      let prot = protection_from_permission(access) as u32;
      let alloc_type = if matches!(
        access,
        MemoryPermission::NoAccess | MemoryPermission::NoAccessWillJitLater
      ) {
        win::MEM_RESERVE
      } else {
        win::MEM_COMMIT | win::MEM_RESERVE
      };

      if alignment <= page_size {
        let mut base = unsafe { win::VirtualAlloc(hint, size, alloc_type, prot) };
        if base.is_null() && !hint.is_null() {
          base = unsafe { win::VirtualAlloc(null_mut(), size, alloc_type, prot) };
        }
        return if base.is_null() {
          Err(Error::MmapFailed)
        } else {
          Ok(base)
        };
      }

      // Windows 无法进行局部 VirtualFree，大对齐走 V8 原版 platform-win32.cc“预留-释放-精准重占”策略
      let aligned_hint = if !hint.is_null() {
        (hint as usize & !(alignment - 1)) as *mut c_void
      } else {
        null_mut()
      };
      let base = unsafe { win::VirtualAlloc(aligned_hint, size, alloc_type, prot) };
      if !base.is_null() {
        if (base as usize) % alignment == 0 {
          return Ok(base);
        }
        unsafe { win::VirtualFree(base, 0, win::MEM_RELEASE) };
      }

      let padded_size = size
        .checked_add(alignment.saturating_sub(page_size))
        .ok_or(Error::InvalidArgument)?;
      let padded_size = align_up(padded_size, page_size);
      for _ in 0..3 {
        let base = unsafe { win::VirtualAlloc(null_mut(), padded_size, alloc_type, prot) };
        if base.is_null() {
          return Err(Error::MmapFailed);
        }
        let aligned_base = align_up(base as usize, alignment);
        unsafe { win::VirtualFree(base, 0, win::MEM_RELEASE) };

        // 尝试在探测出的对齐基址上精确申请
        let realloc =
          unsafe { win::VirtualAlloc(aligned_base as *mut c_void, size, alloc_type, prot) };
        if !realloc.is_null() {
          if realloc as usize == aligned_base {
            return Ok(realloc);
          }
          unsafe { win::VirtualFree(realloc, 0, win::MEM_RELEASE) };
        }
      }
      Err(Error::MmapFailed)
    }
  }

  /// 对应 V8 `OS::Free(address, size)`。
  ///
  /// # Safety
  /// `address` 必须是此前 `allocate` 返回且未释放。
  pub unsafe fn free(address: *mut c_void, size: usize) -> Result<()> {
    #[cfg(unix)]
    {
      // SAFETY: 由调用方保证。
      let rc = unsafe { libc::munmap(address, size) };
      if rc != 0 {
        Err(Error::MunmapFailed)
      } else {
        Ok(())
      }
    }
    #[cfg(windows)]
    {
      let _ = size;
      let rc = unsafe { win::VirtualFree(address, 0, win::MEM_RELEASE) };
      if rc == 0 {
        Err(Error::MunmapFailed)
      } else {
        Ok(())
      }
    }
  }

  /// 对应 V8 `OS::SetPermissions(address, size, access)`。
  ///
  /// # Safety
  /// `address`+`size` 必须落在合法映射内且页对齐。
  pub unsafe fn set_permissions(
    address: *mut c_void,
    size: usize,
    access: MemoryPermission,
  ) -> Result<()> {
    #[cfg(unix)]
    {
      let prot = protection_from_permission(access);
      // SAFETY: 由调用方保证范围。
      let rc = unsafe { libc::mprotect(address, size, prot) };
      if rc != 0 {
        #[cfg(target_os = "macos")]
        {
          // macOS kNoAccess 特例：mprotect 失败回退 `MADV_FREE_REUSABLE`。
          if access == MemoryPermission::NoAccess {
            let rc2 = unsafe { madvise(address, size, libc::MADV_FREE_REUSABLE) };
            if rc2 == 0 {
              return Ok(());
            }
          }
        }
        return Err(Error::MProtectFailed);
      }
      if access == MemoryPermission::NoAccess {
        // 与原 V8 语义一致：kNoAccess 视作 DiscardSystemPages。
        // 忽略结果，与原实现一致。
        let _ = unsafe { Self::discard_system_pages(address, size) };
      }
      #[cfg(target_os = "macos")]
      {
        if access != MemoryPermission::NoAccess {
          // 原实现：MADV_FREE_REUSE 提示内核释放 reusable 状态。
          unsafe {
            madvise(address, size, libc::MADV_FREE_REUSE);
          };
        }
      }
      Ok(())
    }
    #[cfg(windows)]
    {
      if matches!(
        access,
        MemoryPermission::NoAccess | MemoryPermission::NoAccessWillJitLater
      ) {
        // 对标 V8 platform-win32.cc:1064 kNoAccess 走 VirtualFree MEM_DECOMMIT，真实归还内核 commit charge
        let rc = unsafe { win::VirtualFree(address, size, win::MEM_DECOMMIT) };
        if rc != 0 {
          Ok(())
        } else {
          // 若本就处于 MEM_RESERVE 未提交状态，VirtualFree(MEM_DECOMMIT) 返回 0 乃正常现象，尝试 VirtualProtect
          let mut old_prot = 0u32;
          let _ = unsafe { win::VirtualProtect(address, size, win::PAGE_NOACCESS, &mut old_prot) };
          Ok(())
        }
      } else {
        // 对标 V8 platform-win32.cc:1068-1075: VirtualAlloc MEM_COMMIT 原生支持提交未提交页或更新已提交页保护属性
        let prot = protection_from_permission(access) as u32;
        let ptr = unsafe { win::VirtualAlloc(address, size, win::MEM_COMMIT, prot) };
        if !ptr.is_null() {
          Ok(())
        } else {
          Err(Error::MProtectFailed)
        }
      }
    }
  }

  /// 对应 V8 `OS::DiscardSystemPages`。
  ///
  /// # Safety
  /// `address`+`size` 必须在合法映射内且页对齐。
  pub unsafe fn discard_system_pages(address: *mut c_void, size: usize) -> Result<()> {
    #[cfg(target_os = "macos")]
    {
      // SAFETY: madvise 是 POSIX；由调用方保证范围。
      loop {
        let rc = unsafe { madvise(address, size, libc::MADV_FREE_REUSABLE) };
        if rc == 0 {
          return Ok(());
        }
        if io::Error::last_os_error().raw_os_error() == Some(libc::EAGAIN) {
          thread::yield_now();
          continue;
        }
        // 回退：MADV_DONTNEED。
        let rc2 = unsafe { madvise(address, size, libc::MADV_DONTNEED) };
        if rc2 == 0 {
          return Ok(());
        }
        return Err(Error::MadviseFailed);
      }
    }
    #[cfg(target_os = "linux")]
    {
      // SAFETY: madvise 是 POSIX；由调用方保证范围。
      let rc = unsafe { libc::madvise(address, size, libc::MADV_DONTNEED) };
      if rc == 0 {
        Ok(())
      } else {
        Err(Error::MadviseFailed)
      }
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
      let _ = (address, size);
      Ok(())
    }
  }

  /// 对应 V8 `OS::DecommitPages`：POSIX 下以匿名 PROT_NONE 覆盖。
  ///
  /// # Safety
  /// `address`+`size` 必须页对齐且属于合法预留范围。
  pub unsafe fn decommit_pages(address: *mut c_void, size: usize) -> Result<()> {
    #[cfg(unix)]
    {
      let flags = libc::MAP_FIXED | libc::MAP_ANONYMOUS | libc::MAP_PRIVATE;
      // SAFETY: MAP_FIXED 由调用方保证覆盖既有映射。
      let p = unsafe { libc::mmap(address, size, libc::PROT_NONE, flags, -1, 0) };
      if p == address {
        Ok(())
      } else if p == libc::MAP_FAILED {
        Err(Error::MmapFailed)
      } else {
        // 意外地址：回退并报错。
        unsafe {
          libc::munmap(p, size);
        }
        Err(Error::MmapFailed)
      }
    }
    #[cfg(windows)]
    {
      let rc = unsafe { win::VirtualFree(address, size, win::MEM_DECOMMIT) };
      if rc != 0 {
        Ok(())
      } else {
        Err(Error::MunmapFailed)
      }
    }
  }

  /// 对应 V8 `OS::RecommitPages`（platform-posix.cc:621-630）：darwin 走
  /// `MADV_FREE_REUSE`（EAGAIN 忙重试），原版无论最终成败一律 `return true`
  /// （失败仅发生在参数非法的调用方 bug 场景）；其他 POSIX 直接成功。
  ///
  /// # Safety
  /// `address`+`size` 必须页对齐且在合法映射内。
  pub unsafe fn recommit_pages(
    address: *mut c_void,
    size: usize,
    access: MemoryPermission,
  ) -> Result<()> {
    #[cfg(target_os = "macos")]
    {
      let _ = access;
      loop {
        // SAFETY: madvise 是 POSIX；调用方保证范围。
        let rc = unsafe { madvise(address, size, libc::MADV_FREE_REUSE) };
        if rc == 0 {
          break;
        }
        if io::Error::last_os_error().raw_os_error() == Some(libc::EAGAIN) {
          thread::yield_now();
          continue;
        }
        // 原版语义：非 EAGAIN 失败也返回 true。
        break;
      }
      Ok(())
    }
    #[cfg(windows)]
    {
      // 对标 V8 platform-win32.cc:1083：bool OS::RecommitPages(...) { return SetPermissions(...); }
      unsafe { Self::set_permissions(address, size, access) }
    }
    #[cfg(all(not(target_os = "macos"), not(windows)))]
    {
      // 原 POSIX 非 darwin 分支：直接 return true。
      let _ = (address, size, access);
      Ok(())
    }
  }

  /// 对应 V8 `OS::SealPages`；仅 darwin x64 有 `mcache_t`；本轮 no-op 返回 false。
  ///
  /// # Safety
  /// `address`/`size` 需构成一个此前由 `Allocate` 返回、仍然有效的页范围；当前实现不会解引用。
  pub unsafe fn seal_pages(_address: *mut c_void, _size: usize) -> Result<()> {
    Err(Error::Unsupported)
  }

  /// 对应 V8 `OS::IsRemapPageSupported()`；macOS / Linux x64 且非 ppc64/s390x。
  pub const fn is_remap_page_supported() -> bool {
    (cfg!(target_os = "macos") || cfg!(target_os = "linux"))
      && !cfg!(target_arch = "powerpc64")
      && !cfg!(target_arch = "s390x")
  }

  /// 对应 V8 `OS::RemapPages`；darwin `mach_vm_remap`；linux `mremap(MREMAP_DONTUNMAP)`。
  ///
  /// # Safety
  /// 源与目标地址必须页对齐，size 为页大小倍数。
  pub unsafe fn remap_pages(
    address: *mut c_void,
    size: usize,
    new_address: *mut c_void,
    access: MemoryPermission,
  ) -> Result<()> {
    if !Self::is_remap_page_supported() {
      return Err(Error::RemapUnsupported);
    }
    #[cfg(target_os = "macos")]
    {
      // mach_vm_remap；参见 platform-darwin.cc。
      unsafe extern "C" {
        static mach_task_self_: u32;
        fn mach_vm_remap(
          target: u32,
          address: *mut u64,
          size: u64,
          mask: u64,
          flags: i32,
          src_task: u32,
          src_address: u64,
          copy: i32,
          cur_prot: *mut i32,
          max_prot: *mut i32,
          inheritance: i32,
        ) -> i32;
      }
      const VM_FLAGS_FIXED: i32 = 0x0000;
      const VM_FLAGS_OVERWRITE: i32 = 0x4000;
      const VM_INHERIT_NONE: i32 = 2;
      // VM_PROT_*：与原版 GetVMProtFromMemoryPermission 一致。
      const VM_PROT_NONE: i32 = 0;
      const VM_PROT_READ: i32 = 1;
      const VM_PROT_WRITE: i32 = 2;
      const VM_PROT_EXECUTE: i32 = 4;
      let req_prot = match access {
        MemoryPermission::NoAccess | MemoryPermission::NoAccessWillJitLater => VM_PROT_NONE,
        MemoryPermission::Read => VM_PROT_READ,
        MemoryPermission::ReadWrite => VM_PROT_READ | VM_PROT_WRITE,
        MemoryPermission::ReadWriteExecute => VM_PROT_READ | VM_PROT_WRITE | VM_PROT_EXECUTE,
        MemoryPermission::ReadExecute => VM_PROT_READ | VM_PROT_EXECUTE,
      };
      let mut target_addr = new_address as u64;
      // cur_protection 为 in/out：内核取其与源映射保护的交集写回（platform-darwin.cc:197-206）。
      let mut cur: i32 = req_prot;
      let mut max: i32 = 0;
      // SAFETY: 系统调用；由调用方保证地址有效；mach_task_self_ 由 libSystem 保证。
      let kr = unsafe {
        mach_vm_remap(
          mach_task_self_,
          &mut target_addr,
          size as u64,
          0,
          VM_FLAGS_FIXED | VM_FLAGS_OVERWRITE,
          mach_task_self_,
          address as u64,
          0,
          &mut cur,
          &mut max,
          VM_INHERIT_NONE,
        )
      };
      if kr != 0 {
        return Err(Error::MmapFailed);
      }
      // 原版 CHECK_EQ(new_address, target)（platform-darwin.cc:211）：
      // FIXED|OVERWRITE 必须落在请求地址，否则视为失败。
      if target_addr != new_address as u64 {
        return Err(Error::MmapFailed);
      }
      // 原版 remap 后不再 mprotect：实际保护为请求 access 与源映射的交集。
      Ok(())
    }
    #[cfg(target_os = "linux")]
    {
      // 通过 `syscall(SYS_mremap, ...)` 传入第五参数 `new_address`；
      // libc::mremap 的签名不含 `new_address`，无法直接支持 `MREMAP_FIXED`。
      const MREMAP_MAYMOVE: usize = 1;
      const MREMAP_FIXED: usize = 2;
      const MREMAP_DONTUNMAP: usize = 4;
      // SAFETY: SYS_mremap 是 Linux 系统调用；由调用方保证页对齐。
      let p = unsafe {
        libc::syscall(
          libc::SYS_mremap,
          address as usize,
          size,
          size,
          MREMAP_MAYMOVE | MREMAP_FIXED | MREMAP_DONTUNMAP,
          new_address as usize,
        )
      };
      if p as *mut libc::c_void == libc::MAP_FAILED {
        return Err(Error::MmapFailed);
      }
      // 原版 mremap 成功后 mprotect 到请求权限（platform-linux.cc:244-245
      // `CHECK_EQ(0, mprotect(new_address, size, protection))`）；darwin 无此步
      // （mach_vm_remap 的 cur_protection 已按请求取交集）。
      let prot = protection_from_permission(access);
      // SAFETY: mremap 已成功建立 new_address 映射；由调用方保证页对齐。
      if unsafe { libc::mprotect(new_address, size, prot) } != 0 {
        return Err(Error::MProtectFailed);
      }
      // 原版成功后另有 memcmp sanity CHECK（linux cc:247-252）；crate 禁 panic，
      // 且校验需读源/目标映射（保护未知的场景有 SIGSEGV 风险），登记 readme 不迁。
      Ok(())
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
      let _ = (address, size, new_address, access);
      Err(Error::RemapUnsupported)
    }
  }

  /// 对应 V8 `OS::SetDataReadOnly`；POSIX 下等价 `mprotect(PROT_READ)`。
  ///
  /// # Safety
  /// `address`+`size` 必须属于合法数据映射且页对齐。
  pub unsafe fn set_data_read_only(address: *mut c_void, size: usize) -> Result<()> {
    unsafe { Self::set_permissions(address, size, MemoryPermission::Read) }
  }

  /// 对应 V8 `OS::SignalCodeMovingGC`；POSIX 无实现。
  pub fn signal_code_moving_gc() {}

  /// 对应 V8 `OS::AdjustSchedulingParams`；仅 darwin x86_64 有效。
  pub fn adjust_scheduling_params() {}

  /// 对应 V8 `OS::CanReserveAddressSpace()`；POSIX 分支恒为 true。
  #[inline]
  pub const fn can_reserve_address_space() -> bool {
    true
  }

  /// 对应 V8 `OS::Release(address, size)`；
  /// POSIX 分支等价 munmap；Windows 分支为 VirtualFree(MEM_DECOMMIT)（对标 platform-win32.cc:1054）。
  ///
  /// # Safety
  /// 见 `Self::free`。
  #[inline]
  pub unsafe fn release(address: *mut c_void, size: usize) -> Result<()> {
    #[cfg(unix)]
    {
      // SAFETY: 由调用方保证。
      unsafe { Self::free(address, size) }
    }
    #[cfg(windows)]
    {
      let rc = unsafe { win::VirtualFree(address, size, win::MEM_DECOMMIT) };
      if rc == 0 {
        Err(Error::MunmapFailed)
      } else {
        Ok(())
      }
    }
    #[cfg(not(any(unix, windows)))]
    {
      let _ = (address, size);
      Ok(())
    }
  }

  /// 对应 V8 `OS::SetMemoryRegionName`；Linux ≥5.17 使用 `prctl(PR_SET_VMA_ANON_NAME)`，
  /// 其他平台返回 false（对应原 POSIX 分支的静默失败语义）。
  ///
  /// # Safety
  /// `address`+`size` 必须落在合法映射内；`name` 为 NUL 结尾 ASCII。
  pub unsafe fn set_memory_region_name(address: *mut c_void, size: usize, name: &str) -> bool {
    #[cfg(target_os = "linux")]
    {
      const PR_SET_VMA: i32 = 0x5356_4d41;
      const PR_SET_VMA_ANON_NAME: i32 = 0;
      let mut buf = [0u8; 80];
      let bytes = name.as_bytes();
      if bytes.is_empty() || bytes.len() >= buf.len() {
        return false;
      }
      buf[..bytes.len()].copy_from_slice(bytes);
      // SAFETY: `prctl` 是 POSIX 变参系统调用；此处传 4 个固定参数。
      let rc = unsafe {
        libc::prctl(
          PR_SET_VMA,
          PR_SET_VMA_ANON_NAME,
          address as libc::c_ulong,
          size,
          buf.as_ptr() as libc::c_ulong,
        )
      };
      rc == 0
    }
    #[cfg(not(target_os = "linux"))]
    {
      let _ = (address, size, name);
      false
    }
  }

  /// CPU 逻辑核数；对应上层 Platform `NumberOfCPUs` 的探测能力，走
  /// `sysconf(_SC_NPROCESSORS_ONLN)`，失败回退 `std::thread::available_parallelism`。
  pub fn number_of_cpus() -> usize {
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    {
      // SAFETY: `sysconf` 无副作用；常量查询。
      let n = unsafe { libc::sysconf(libc::_SC_NPROCESSORS_ONLN) };
      if n > 0 {
        return n as usize;
      }
    }
    thread::available_parallelism()
      .map(|n| n.get())
      .unwrap_or(1)
  }

  /// 对应 V8 `OS::ExitProcess`（platform-posix.cc:939-945）：先 flush stdio 再 `_exit`。
  pub fn exit_process(code: i32) -> ! {
    #[cfg(unix)]
    unsafe {
      libc::fflush(null_mut());
      libc::_exit(code);
    }
    #[cfg(not(unix))]
    {
      std::process::exit(code);
    }
  }
}

/// 与 V8 `GetProtectionFromMemoryPermission` 对应。
fn protection_from_permission(access: MemoryPermission) -> i32 {
  #[cfg(unix)]
  match access {
    MemoryPermission::NoAccess | MemoryPermission::NoAccessWillJitLater => libc::PROT_NONE,
    MemoryPermission::Read => libc::PROT_READ,
    MemoryPermission::ReadWrite => libc::PROT_READ | libc::PROT_WRITE,
    MemoryPermission::ReadExecute => libc::PROT_READ | libc::PROT_EXEC,
    MemoryPermission::ReadWriteExecute => libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC,
  }
  #[cfg(windows)]
  match access {
    MemoryPermission::NoAccess | MemoryPermission::NoAccessWillJitLater => {
      win::PAGE_NOACCESS as i32
    }
    MemoryPermission::Read => win::PAGE_READONLY as i32,
    MemoryPermission::ReadWrite => win::PAGE_READWRITE as i32,
    MemoryPermission::ReadExecute => win::PAGE_EXECUTE_READ as i32,
    MemoryPermission::ReadWriteExecute => win::PAGE_EXECUTE_READWRITE as i32,
  }
}

/// 与 V8 POSIX `Allocate` 的 `flags` 一致。
#[cfg(unix)]
fn mmap_flags_for_access(access: MemoryPermission) -> i32 {
  let mut flags = libc::MAP_PRIVATE | libc::MAP_ANONYMOUS;
  #[cfg(target_os = "macos")]
  {
    if access == MemoryPermission::ReadWriteExecute
      || access == MemoryPermission::NoAccessWillJitLater
    {
      // MAP_JIT on darwin = 0x0800。
      flags |= 0x0800;
    }
  }
  // 原版（platform-posix.cc:142-150）：kNoAccess 与 kNoAccessWillJitLater
  // 均加 MAP_NORESERVE（AIX/FREEBSD/QNX 除外，本 crate 不覆盖）。
  if matches!(
    access,
    MemoryPermission::NoAccess | MemoryPermission::NoAccessWillJitLater
  ) {
    flags |= libc::MAP_NORESERVE;
  }
  flags
}

#[cfg(unix)]
fn mmap_fd_for_access(_access: MemoryPermission) -> i32 {
  // 原版 darwin 分支（platform-posix.cc:170-177）：fd = VM_MAKE_TAG(255)，
  // 用于 vmmap(1) 等内存分析工具识别 V8 区域；其余平台匿名映射 -1。
  #[cfg(target_os = "macos")]
  {
    (255u32 << 24) as i32
  }
  #[cfg(not(target_os = "macos"))]
  {
    -1
  }
}

// 避免 `madvise` 在 macos 上作为自由函数缺失；`libc::madvise` 已提供。
#[cfg(target_os = "macos")]
#[inline]
unsafe fn madvise(addr: *mut c_void, len: usize, advice: i32) -> i32 {
  // SAFETY: 见调用点。
  unsafe { libc::madvise(addr, len, advice) }
}
