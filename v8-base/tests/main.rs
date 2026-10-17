use std::slice::from_raw_parts_mut;

use aok::{OK, Void};
use log::info;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test() -> Void {
  info!("> test {}", 123456);
  OK
}

#[test]
fn test_os_seal_pages() -> Void {
  use v8_base::page_allocator::{PageAllocator, Permission, global_page_allocator, os_seal_pages};

  // 0 地址 / 0 大小应当返回 false
  assert!(!os_seal_pages(0, 0));

  let alloc = global_page_allocator();
  let page_size = alloc.allocate_page_size();
  let page = alloc.allocate_pages(0, page_size, page_size, Permission::ReadWrite);
  assert!(page.is_some());
  let addr = page.unwrap();

  // 写入数据验证当前可写
  let slice = unsafe { from_raw_parts_mut(addr as *mut u8, page_size) };
  slice[0] = 42;

  // 封锁页面 (只读保护)
  let sealed = alloc.seal_pages(addr, page_size);
  assert!(sealed);

  // 释放页面
  alloc.free_pages(addr, page_size);

  OK
}
