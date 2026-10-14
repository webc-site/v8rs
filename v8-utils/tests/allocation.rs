// 上游 allocation-unittest.cc 4 例全针对已推迟的平台层 (VirtualMemory 等, 见 README);
// 此处补独立子集 (str_n_dup 截断 / aligned_alloc_with_retry 往返 / 页大小) 的关键路径。
use std::ptr::{read, write};

use aok::{OK, Void};
use v8_utils::allocation::{
  aligned_alloc_with_retry, aligned_free, allocate_page_size, commit_page_size, str_n_dup,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

// str_n_dup: 超长按 n 截断, 多字节字符不撕裂
#[test]
fn str_n_dup_truncation() -> Void {
  assert_eq!(str_n_dup("abcdef", 3), "abc");
  assert_eq!(str_n_dup("abcdef", 10), "abcdef");
  assert_eq!(str_n_dup("", 3), "");
  // "你" 为 3 字节 UTF-8: n=4 恰含 "你a", n=3 截到整字符 "你", n=2 落在字符中间回退
  assert_eq!(str_n_dup("你a", 4), "你a");
  assert_eq!(str_n_dup("你a", 3), "你");
  assert_eq!(str_n_dup("你a", 2), "");
  OK
}

// aligned_alloc_with_retry: 正常分配成功路径 + aligned_free 释放往返
#[test]
fn aligned_alloc_roundtrip() -> Void {
  // SAFETY: 分配 64 字节/64 对齐块, 写读校验后以同布局释放
  unsafe {
    let p = aligned_alloc_with_retry(64, 64).expect("分配失败");
    assert!((p.as_ptr() as usize).is_multiple_of(64));
    // 逐字节写读前 8 字节
    for i in 0..8 {
      write(p.as_ptr().add(i), (0xA0 + i) as u8);
    }
    for i in 0..8 {
      assert_eq!(read(p.as_ptr().add(i)), (0xA0 + i) as u8);
    }
    aligned_free(p, 64, 64);
  }
  // 零尺寸: std::alloc 零尺寸 Layout 是 UB, 契约收敛为 None (README 登记)
  // SAFETY: size==0 提前返回, 不触碰分配器
  assert!(unsafe { aligned_alloc_with_retry(0, 64) }.is_none());
  // 非法对齐 (非 2 的幂): Layout 构造失败 → None
  // SAFETY: 提前返回 None, 不触碰分配器
  assert!(unsafe { aligned_alloc_with_retry(8, 12) }.is_none());
  OK
}

// 页大小: 恒为正且为 2 的幂 (平台页粒度契约)
#[test]
fn page_size_is_pow2() -> Void {
  let ps = allocate_page_size();
  assert!(ps > 0);
  assert!(ps.is_power_of_two());
  assert_eq!(commit_page_size(), ps);
  OK
}
