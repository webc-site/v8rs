//! v8-base/src/sys_info.rs 集成测试 — 1:1 对应 sys-info-unittest.cc
//!
//! 原版 3 个用例:
//! - `SysInfoTest.NumberOfProcessors`: EXPECT_LT(0, ...)
//! - `SysInfoTest.AmountOfPhysicalMemory`: EXPECT_LT(0, ...)
//! - `SysInfoTest.AmountOfVirtualMemory`: EXPECT_LE(0, ...)
//!
//! 严禁依赖机器特定值: CPU 数 / 内存大小只断言 >0 或 >=0, 具体数字不核对.

use aok::{OK, Void};
use v8_base::sys_info::{
  address_space_end, amount_of_physical_memory, amount_of_virtual_memory, number_of_processors,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn sys_info_test_number_of_processors() -> Void {
  assert!(number_of_processors() > 0);
  OK
}

#[test]
fn sys_info_test_amount_of_physical_memory() -> Void {
  assert!(amount_of_physical_memory() > 0);
  OK
}

#[test]
fn sys_info_test_amount_of_virtual_memory() -> Void {
  assert!(amount_of_virtual_memory() >= 0);
  OK
}

/// 原版 `SysInfo::AddressSpaceEnd()`: POSIX 为 `uintptr_t::MAX`, Windows 为应用地址空间上限
#[test]
fn sys_info_test_address_space_end() -> Void {
  #[cfg(not(target_os = "windows"))]
  assert_eq!(address_space_end(), usize::MAX);
  #[cfg(target_os = "windows")]
  assert!(address_space_end() > 0 && address_space_end() <= usize::MAX);
  OK
}
