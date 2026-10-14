//! 原版 v8/src/base/sys-info.cc/h — 机器/进程资源探测
//!
//! 覆盖 macOS + Linux 两条 cfg (其余平台后置); 严禁新增依赖, 使用 `libc` 直接调
//! `sysctl`/`sysconf`/`getrlimit`.

use std::thread::available_parallelism;

/// `SysInfo` 类以模块函数形式呈现 (原版为 `final class` 静态方法集合)
/// 原版 `SysInfo::NumberOfProcessors`:
/// - 采用标准库 `available_parallelism` 获取在线核心数，跨所有平台统一且无 unsafe
#[must_use]
pub fn number_of_processors() -> i32 {
  available_parallelism().map_or(1, |n| n.get() as i32)
}

/// 原版 `SysInfo::AmountOfPhysicalMemory`:
/// - Darwin: `sysctl CTL_HW, HW_MEMSIZE` 读 int64
/// - Windows: `GlobalMemoryStatusEx`
/// - Linux/POSIX: `sysconf(_SC_PHYS_PAGES) * sysconf(_SC_PAGESIZE)`
#[must_use]
pub fn amount_of_physical_memory() -> i64 {
  #[cfg(target_os = "macos")]
  {
    use core::ptr::null_mut;
    // Darwin sysctl MIB (uapi sysctl.h): CTL_HW = 6, HW_MEMSIZE = 10.
    const CTL_HW: libc::c_int = 6;
    const HW_MEMSIZE: libc::c_int = 10;
    let mut mib = [CTL_HW, HW_MEMSIZE];
    let mut memsize: i64 = 0;
    let mut len = size_of::<i64>();
    // SAFETY: mib 是长度 2 的常量数组, oldp/oldlenp 指向本机可写的 i64 / usize;
    // newp 为 NULL, newlen 为 0, 与原版 sysctl(mib, 2, &memsize, &len, nullptr, 0) 一致.
    let rc = unsafe {
      libc::sysctl(
        mib.as_mut_ptr(),
        mib.len() as libc::c_uint,
        &raw mut memsize as *mut libc::c_void,
        &raw mut len,
        null_mut(),
        0,
      )
    };
    if rc != 0 { 0 } else { memsize }
  }
  #[cfg(target_os = "windows")]
  {
    #[repr(C)]
    struct MemoryStatusEx {
      length: u32,
      memory_load: u32,
      total_phys: u64,
      avail_phys: u64,
      total_page_file: u64,
      avail_page_file: u64,
      total_virtual: u64,
      avail_virtual: u64,
      avail_extended_virtual: u64,
    }
    unsafe extern "system" {
      fn GlobalMemoryStatusEx(status: *mut MemoryStatusEx) -> i32;
    }
    let mut status = MemoryStatusEx {
      length: core::mem::size_of::<MemoryStatusEx>() as u32,
      memory_load: 0,
      total_phys: 0,
      avail_phys: 0,
      total_page_file: 0,
      avail_page_file: 0,
      total_virtual: 0,
      avail_virtual: 0,
      avail_extended_virtual: 0,
    };
    if unsafe { GlobalMemoryStatusEx(&mut status) } != 0 {
      status.total_phys as i64
    } else {
      0
    }
  }
  #[cfg(all(not(target_os = "macos"), not(target_os = "windows")))]
  {
    // SAFETY: sysconf 只读常量, 无副作用.
    let pages = unsafe { libc::sysconf(libc::_SC_PHYS_PAGES) };
    // SAFETY: 同上.
    let page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) };
    if pages == -1 || page_size == -1 {
      return 0;
    }
    pages * page_size
  }
}

/// 原版 `SysInfo::AmountOfVirtualMemory`:
/// - Win/Fuchsia 返回 0
/// - POSIX: `getrlimit(RLIMIT_DATA, ...)`, RLIM_INFINITY 视为 0
#[must_use]
pub fn amount_of_virtual_memory() -> i64 {
  #[cfg(any(target_os = "windows", target_os = "fuchsia"))]
  {
    0
  }
  #[cfg(not(any(target_os = "windows", target_os = "fuchsia")))]
  {
    let mut rlim = libc::rlimit {
      rlim_cur: 0,
      rlim_max: 0,
    };
    // SAFETY: rlim 是本机可写结构, getrlimit 按指针填充; RLIMIT_DATA 常量在两个 cfg 上均存在.
    let rc = unsafe { libc::getrlimit(libc::RLIMIT_DATA, &raw mut rlim) };
    if rc != 0 {
      return 0;
    }
    if rlim.rlim_cur == libc::RLIM_INFINITY {
      0
    } else {
      rlim.rlim_cur as i64
    }
  }
}

/// 原版 `SysInfo::AddressSpaceEnd`:
/// - Win: 查询 SYSTEM_INFO.lpMaximumApplicationAddress + 1
/// - 其余 POSIX: `uintptr_t::MAX` (rlimit 不限制地址空间, 只限制映射大小)
#[must_use]
pub fn address_space_end() -> usize {
  #[cfg(target_os = "windows")]
  {
    #[repr(C)]
    struct SystemInfo {
      w_processor_architecture: u16,
      w_reserved: u16,
      dw_page_size: u32,
      lp_minimum_application_address: *mut core::ffi::c_void,
      lp_maximum_application_address: *mut core::ffi::c_void,
      dw_active_processor_mask: usize,
      dw_number_of_processors: u32,
      dw_processor_type: u32,
      dw_allocation_granularity: u32,
      w_processor_level: u16,
      w_processor_revision: u16,
    }
    unsafe extern "system" {
      fn GetSystemInfo(info: *mut SystemInfo);
    }
    let mut info = core::mem::MaybeUninit::<SystemInfo>::uninit();
    unsafe {
      GetSystemInfo(info.as_mut_ptr());
      let info = info.assume_init();
      info.lp_maximum_application_address as usize + 1
    }
  }
  #[cfg(not(target_os = "windows"))]
  {
    usize::MAX
  }
}
