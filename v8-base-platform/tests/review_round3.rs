//! 对抗审查 round3 修复回归用例（原版无 1:1 gtest 对应，守护本轮修复项）。
#[cfg(target_os = "linux")]
use std::ptr::null_mut;
use std::{
  env::temp_dir,
  fs::{create_dir_all, metadata},
};

use aok::{OK, Void};
#[cfg(any(target_os = "linux", all(target_os = "linux", target_arch = "x86_64")))]
use log::info;
use v8_base_platform::{
  Error, MemoryMappedFile, MemoryProtectionKey, Options, Permission, Thread, allocate_at_least,
};
#[cfg(target_os = "linux")]
use v8_base_platform::{MemoryPermission, OS};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// `MemoryMappedFile::create` 短 initial 失败语义（platform-posix.cc:888-892：
/// `fwrite(initial, 1, size)` 短写/错误 → nullptr）；足量 initial 整块写入成功。
#[test]
fn memory_mapped_file_create_short_initial_fails() -> Void {
  let dir = temp_dir().join("v8bp_r3_mmf");
  create_dir_all(&dir).unwrap();
  // 短 initial → None（round3 修复前：静默补零返回 Some）。
  assert!(
    MemoryMappedFile::create(dir.join("short.dat"), 16, b"abc").is_none(),
    "initial 不足 size 应失败（原版短写 nullptr）"
  );
  // 足量 initial → Some，size/内容匹配。
  let content = b"0123456789abcdef";
  let h = MemoryMappedFile::create(dir.join("full.dat"), 16, content).expect("足量 initial 应成功");
  assert_eq!(16, h.size());
  assert_eq!(content.as_slice(), h.as_slice(), "映射内容 == initial 整块");
  // 文件落盘大小 == size（无零填充尾巴差异）。
  assert_eq!(16, metadata(dir.join("full.dat")).unwrap().len());
  OK
}

/// `Thread::join` 失败映射 `Error::JoinFailed`（round3 修复前误映射
/// `ThreadStartFailed`）；worker panic 只走 Err，不 re-panic。
#[test]
fn thread_join_panic_maps_join_failed() -> Void {
  let mut t = Thread::new(Options::new("v8:rs3-panic"), || {
    panic!("worker 预期 panic");
  });
  assert!(t.start().is_ok());
  match t.join() {
    Err(Error::JoinFailed) => {}
    other => panic!("应返回 JoinFailed, 实得 {other:?}"),
  }
  OK
}

/// `allocate_at_least` 失败返回 `{null, 0}`（原版 memory.h:144-163 usable_size=0
/// 时 count=0）；正常路径 count >= n。
#[test]
fn allocate_at_least_failure_returns_null_zero_count() -> Void {
  // 正常路径。
  let ok = allocate_at_least(100);
  assert!(!ok.ptr.is_null(), "100 字节分配应成功");
  assert!(ok.count >= 100, "usable >= 请求值");

  // 失败路径：超大请求（darwin malloc 直接返回 null）。
  let bad = allocate_at_least(isize::MAX as usize);
  assert!(bad.ptr.is_null(), "超大请求应分配失败");
  assert_eq!(0, bad.count, "失败时 count 应为 0（原版 nullptr+0）");
  OK
}

/// `MemoryProtectionKey::set_key_permissions`（原版 SetPermissionsForKey，
/// memory-protection-key.cc:118-126：wrpkru 改当前线程 PKRU 的 key 权限位）。
/// 非 linux x64 为 no-op 不 panic；linux x64 用临时 key 验证 get/set 往返。
/// 注意：绝不可对 key 0（默认 key，覆盖全部普通页）设置 DisableWrite/Access，
/// 否则全进程读写被禁。
#[test]
fn mpk_set_key_permissions_roundtrip() -> Void {
  #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
  {
    let Ok(key) = MemoryProtectionKey::allocate_key() else {
      info!("pkey 不可用，跳过往返断言");
      return OK;
    };
    MemoryProtectionKey::set_key_permissions(key, Permission::DisableWrite);
    assert_eq!(
      Permission::DisableWrite,
      MemoryProtectionKey::get_key_permission(key)
    );
    MemoryProtectionKey::set_key_permissions(key, Permission::NoRestrictions);
    assert_eq!(
      Permission::NoRestrictions,
      MemoryProtectionKey::get_key_permission(key)
    );
    assert!(MemoryProtectionKey::free_key(key).is_ok());
  }
  #[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
  {
    // 非 linux x64：no-op 冒烟，不得 panic。
    MemoryProtectionKey::set_key_permissions(0, Permission::NoRestrictions);
    assert_eq!(
      Permission::NoRestrictions,
      MemoryProtectionKey::get_key_permission(0)
    );
  }
  OK
}

/// linux RemapPages 成功后应 mprotect 到请求权限（platform-linux.cc:244-245）。
/// darwin 分支由 tests/platform_unittest.rs::os_remap_pages 覆盖。
#[cfg(target_os = "linux")]
#[test]
fn os_remap_pages_linux_applies_mprotect() -> Void {
  let page = OS::allocate_page_size();
  // SAFETY: 页对齐分配，用例内释放。
  let src =
    unsafe { OS::allocate(null_mut(), page, page, MemoryPermission::ReadWrite) }.expect("src");
  // SAFETY: 已分配 RW。
  unsafe { (src as *mut u64).write_volatile(0x1234_5678) };
  // SAFETY: 页对齐分配，用例内释放。
  let dst =
    unsafe { OS::allocate(null_mut(), page, page, MemoryPermission::ReadWrite) }.expect("dst");
  // SAFETY: 源/目标页对齐且 size 为页倍数。
  unsafe { OS::remap_pages(src, page, dst, MemoryPermission::ReadExecute) }.expect("remap");
  // remap 后目标应为源数据（mprotect 已生效为 RX，读校验）。
  // SAFETY: dst 映射存在且 remap 后可读（ReadExecute）。
  assert_eq!(0x1234_5678, unsafe { (dst as *const u64).read_volatile() });
  // SAFETY: 释放目标映射。
  unsafe { OS::free(dst, page) }.expect("free dst");
  // SAFETY: DONTUNMAP 保留源映射，同样释放。
  unsafe { OS::free(src, page) }.expect("free src");
  OK
}
