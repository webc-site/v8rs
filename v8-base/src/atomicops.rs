//! 原版 v8/src/base/atomicops.h — 原子操作与内存序封装
//! C++ 版基于 std::atomic_ref 在裸指针上操作; Rust 版直接以标准库原子类型引用为参数。

use std::{
  mem::size_of,
  sync::atomic::{AtomicI8, AtomicI16, AtomicI32, AtomicI64, Ordering, fence},
};

use crate::immediate_crash::debug_unreachable;

/// Atomic8 = char (本平台签名) → i8
pub type Atomic8 = i8;
/// Atomic16
pub type Atomic16 = i16;
/// Atomic32
pub type Atomic32 = i32;
/// Atomic64 = intptr_t (64 位平台)
pub type Atomic64 = isize;
/// AtomicWord = machine-sized pointer
pub type AtomicWord = Atomic64;

// 64 位平台下 Atomic64 与 AtomicWord 同型
const _: () = assert!(size_of::<usize>() == size_of::<AtomicWord>());

/// SeqCst_MemoryFence
pub fn seq_cst_memory_fence() {
  fence(Ordering::SeqCst);
}

/// trivial 操作允许的原子类型 (对应 AtomicTypeForTrivialOperations 概念)
pub trait AtomicInt {
  fn relaxed_load(&self) -> i64;
  fn acquire_load(&self) -> i64;
  fn seq_cst_load(&self) -> i64;
  fn relaxed_store(&self, v: i64);
  fn release_store(&self, v: i64);
  fn seq_cst_store(&self, v: i64);
  fn relaxed_exchange(&self, v: i64) -> i64;
  fn seq_cst_exchange(&self, v: i64) -> i64;
  fn relaxed_fetch_or(&self, bits: i64) -> i64;
  fn relaxed_compare_and_swap(&self, old: i64, new: i64) -> i64;
  fn release_compare_and_swap(&self, old: i64, new: i64) -> i64;
  fn acquire_release_compare_and_swap(&self, old: i64, new: i64) -> i64;
  fn seq_cst_compare_and_swap(&self, old: i64, new: i64) -> i64;
  fn relaxed_atomic_increment(&self, increment: i64) -> i64;
}

macro_rules! impl_atomic_int {
  ($t:ty, $w:ty) => {
    impl AtomicInt for $t {
      fn relaxed_load(&self) -> i64 {
        self.load(Ordering::Relaxed) as i64
      }
      fn acquire_load(&self) -> i64 {
        self.load(Ordering::Acquire) as i64
      }
      fn seq_cst_load(&self) -> i64 {
        self.load(Ordering::SeqCst) as i64
      }
      fn relaxed_store(&self, v: i64) {
        self.store(v as $w, Ordering::Relaxed);
      }
      fn release_store(&self, v: i64) {
        self.store(v as $w, Ordering::Release);
      }
      fn seq_cst_store(&self, v: i64) {
        self.store(v as $w, Ordering::SeqCst);
      }
      fn relaxed_exchange(&self, v: i64) -> i64 {
        self.swap(v as $w, Ordering::Relaxed) as i64
      }
      fn seq_cst_exchange(&self, v: i64) -> i64 {
        self.swap(v as $w, Ordering::SeqCst) as i64
      }
      fn relaxed_fetch_or(&self, bits: i64) -> i64 {
        self.fetch_or(bits as $w, Ordering::Relaxed) as i64
      }
      fn relaxed_compare_and_swap(&self, old: i64, new: i64) -> i64 {
        // 原版: 无论成否均返回操作前 *ptr 的值
        match self.compare_exchange(old as $w, new as $w, Ordering::Relaxed, Ordering::Relaxed) {
          Ok(prev) | Err(prev) => prev as i64,
        }
      }
      fn release_compare_and_swap(&self, old: i64, new: i64) -> i64 {
        match self.compare_exchange(old as $w, new as $w, Ordering::Release, Ordering::Relaxed) {
          Ok(prev) | Err(prev) => prev as i64,
        }
      }
      fn acquire_release_compare_and_swap(&self, old: i64, new: i64) -> i64 {
        match self.compare_exchange(old as $w, new as $w, Ordering::AcqRel, Ordering::Acquire) {
          Ok(prev) | Err(prev) => prev as i64,
        }
      }
      fn seq_cst_compare_and_swap(&self, old: i64, new: i64) -> i64 {
        match self.compare_exchange(old as $w, new as $w, Ordering::SeqCst, Ordering::SeqCst) {
          Ok(prev) | Err(prev) => prev as i64,
        }
      }
      fn relaxed_atomic_increment(&self, increment: i64) -> i64 {
        // 原版: increment + fetch_add(increment, relaxed)
        increment.wrapping_add(self.fetch_add(increment as $w, Ordering::Relaxed) as i64)
      }
    }
  };
}

impl_atomic_int!(AtomicI8, i8);
impl_atomic_int!(AtomicI16, i16);
impl_atomic_int!(AtomicI32, i32);
impl_atomic_int!(AtomicI64, i64);

/// 按字节 Relaxed 复制 (原版以 AtomicWord 宽做原子搬运, 保证单字读写不被撕裂)
///
/// # Safety
/// `dst`/`src` 区间不重叠, dst 起 bytes 字节可写, src 起可读。
pub use crate::memory::is_aligned;

/// # Safety
/// dst/src 起 bytes 字节须有效且 dst/src 按 AtomicWord 对齐, 区间不重叠
pub unsafe fn relaxed_memcpy(dst: *mut Atomic8, src: *const Atomic8, bytes: usize) {
  const WORD: usize = size_of::<AtomicWord>();
  let mut dst = dst;
  let mut src = src;
  let mut bytes = bytes;
  while bytes > 0 && !is_aligned(dst as usize, WORD) {
    byte_copy(&mut dst, &mut src, &mut bytes);
  }
  if is_aligned(src as usize, WORD) {
    while bytes >= WORD {
      // SAFETY: 双指针对齐到 WORD, 原子字宽读写
      unsafe {
        let d = &*(dst as *const AtomicWord as *const AtomicI64);
        let s = &*(src as *const AtomicWord as *const AtomicI64);
        d.relaxed_store(s.relaxed_load());
      }
      dst = unsafe { dst.add(WORD) };
      src = unsafe { src.add(WORD) };
      bytes -= WORD;
    }
  }
  while bytes > 0 {
    byte_copy(&mut dst, &mut src, &mut bytes);
  }
}

fn byte_copy(dst: &mut *mut Atomic8, src: &mut *const Atomic8, bytes: &mut usize) {
  // SAFETY: 调用方保证至少 1 字节可读/可写
  unsafe {
    let d = &*(*dst as *const AtomicI8);
    let s = &*(*src as *const AtomicI8);
    d.relaxed_store(s.relaxed_load());
  }
  *dst = unsafe { (*dst).add(1) };
  *src = unsafe { (*src).add(1) };
  *bytes -= 1;
}

/// 按字节 Relaxed 后向复制 (重叠安全)
///
/// # Safety
/// 区间各自可读/可写。
pub unsafe fn relaxed_memmove(dst: *mut Atomic8, src: *const Atomic8, bytes: usize) {
  // 前向安全: dst 与 src 距离 >= bytes (无重叠或 dst 在前)
  if (dst as usize).wrapping_sub(src as usize) >= bytes {
    // SAFETY: 同上契约
    unsafe { relaxed_memcpy(dst, src, bytes) };
    return;
  }
  let mut dst = unsafe { dst.add(bytes) };
  let mut src = unsafe { src.add(bytes) };
  let mut bytes = bytes;
  const WORD: usize = size_of::<AtomicWord>();
  while bytes > 0 && !is_aligned(dst as usize, WORD) {
    byte_copy_back(&mut dst, &mut src, &mut bytes);
  }
  if is_aligned(src as usize, WORD) {
    while bytes >= WORD {
      dst = unsafe { dst.sub(WORD) };
      src = unsafe { src.sub(WORD) };
      bytes -= WORD;
      // SAFETY: 已对齐
      unsafe {
        let d = &*(dst as *const AtomicWord as *const AtomicI64);
        let s = &*(src as *const AtomicWord as *const AtomicI64);
        d.relaxed_store(s.relaxed_load());
      }
    }
  }
  while bytes > 0 {
    byte_copy_back(&mut dst, &mut src, &mut bytes);
  }
}

fn byte_copy_back(dst: &mut *mut Atomic8, src: &mut *const Atomic8, bytes: &mut usize) {
  *dst = unsafe { (*dst).sub(1) };
  *src = unsafe { (*src).sub(1) };
  // SAFETY: 调用方保证可读/可写
  unsafe {
    let d = &*(*dst as *const AtomicI8);
    let s = &*(*src as *const AtomicI8);
    d.relaxed_store(s.relaxed_load());
  }
  *bytes -= 1;
}

/// MemcmpNotEqualFundamental (AtomicWord) — 小端逐字节比较
/// 原版 `uint8_t byte1 = u1 & 0xFF` 为无符号字节比较, 不得按 i8 比较符号
fn word_not_equal(v1: i64, v2: i64) -> i32 {
  debug_assert_ne!(v1, v2);
  let (mut a, mut b) = (v1 as u64, v2 as u64);
  for _ in 0..size_of::<AtomicWord>() {
    let byte1 = (a & 0xFF) as u8;
    let byte2 = (b & 0xFF) as u8;
    if byte1 != byte2 {
      return if byte1 < byte2 { -1 } else { 1 };
    }
    a >>= 8;
    b >>= 8;
  }
  // 两值不等的契约下不可达
  unsafe { debug_unreachable() }
}

/// 原子 Relaxed_Memcmp — 返回 <0 / 0 / >0
///
/// # Safety
/// 两个区间各自 len 字节可读。
pub unsafe fn relaxed_memcmp(s1: *const Atomic8, s2: *const Atomic8, len: usize) -> i32 {
  const WORD: usize = size_of::<AtomicWord>();
  let mut s1 = s1;
  let mut s2 = s2;
  let mut len = len;
  while len > 0 && !(is_aligned(s1 as usize, WORD) && is_aligned(s2 as usize, WORD)) {
    // SAFETY: 契约保证可读
    let u1 = unsafe { (&*(s1 as *const AtomicI8)).relaxed_load() };
    let u2 = unsafe { (&*(s2 as *const AtomicI8)).relaxed_load() };
    if u1 != u2 {
      return if (u1 as i8) < (u2 as i8) { -1 } else { 1 };
    }
    s1 = unsafe { s1.add(1) };
    s2 = unsafe { s2.add(1) };
    len -= 1;
  }
  if is_aligned(s1 as usize, WORD) && is_aligned(s2 as usize, WORD) {
    while len >= WORD {
      // SAFETY: 已对齐; relaxed_load 经 AtomicInt
      let u1 = unsafe { (&*(s1 as *const AtomicWord as *const AtomicI64)).relaxed_load() };
      let u2 = unsafe { (&*(s2 as *const AtomicWord as *const AtomicI64)).relaxed_load() };
      if u1 != u2 {
        return word_not_equal(u1, u2);
      }
      s1 = unsafe { s1.add(WORD) };
      s2 = unsafe { s2.add(WORD) };
      len -= WORD;
    }
  }
  while len > 0 {
    // SAFETY: 契约保证可读
    let u1 = unsafe { (&*(s1 as *const AtomicI8)).relaxed_load() };
    let u2 = unsafe { (&*(s2 as *const AtomicI8)).relaxed_load() };
    if u1 != u2 {
      return if (u1 as i8) < (u2 as i8) { -1 } else { 1 };
    }
    s1 = unsafe { s1.add(1) };
    s2 = unsafe { s2.add(1) };
    len -= 1;
  }
  0
}
