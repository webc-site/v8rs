// 由 tool/src/utils/consts.js 从 v8/src/utils/memcopy.h 生成; 修订就地改, 复跑覆盖

//! 内存拷贝, 移植自 v8/src/utils/memcopy。
//!
//! 上游为裸指针 + 字节数; Rust 版保持裸指针形态 (调用方保证区间合法),
//! 切片形态由 `copy_chars` 等高层入口提供。

use std::{
  ptr::{copy, copy_nonoverlapping},
  sync::atomic::{AtomicU8, AtomicU16, AtomicU32, Ordering},
};

/// MemMove: 可重叠区间的字节搬移 (memmove)
///
/// # Safety
/// dst/src 起的 size 字节都必须可读写/读。
#[inline]
pub unsafe fn mem_move(dst: *mut u8, src: *const u8, size: usize) {
  unsafe { copy(src, dst, size) };
}

/// MemCopy: 不可重叠区间的字节拷贝 (memcpy)
///
/// # Safety
/// 区间不得重叠; dst/src 起的 size 字节都必须可写/读。
#[inline]
pub unsafe fn mem_copy(dst: *mut u8, src: *const u8, size: usize) {
  unsafe { copy_nonoverlapping(src, dst, size) };
}

/// CopyWords: 按系统字拷贝 count 个 (上游 CopyImpl<Address*>)
///
/// # Safety
/// 区间不得重叠; 指针须 usize 对齐; count*size 可达。
#[inline]
pub unsafe fn copy_words(dst: *mut usize, src: *const usize, count: usize) {
  unsafe { copy_nonoverlapping(src, dst, count) };
}

/// CopyTagged: 按 tagged (u32) 拷贝 count 个 (上游 CopyImpl<Tagged_t*>)
///
/// # Safety
/// 区间不得重叠; 指针须 u32 对齐。
#[inline]
pub unsafe fn copy_tagged(dst: *mut u32, src: *const u32, count: u32) {
  unsafe { copy_nonoverlapping(src, dst, count as usize) };
}

/// CopyBytes: 字节拷贝 (sizeof(T)==1 特化)
///
/// # Safety
/// 区间不得重叠。
#[inline]
pub unsafe fn copy_bytes(dst: *mut u8, src: *const u8, count: usize) {
  unsafe { copy_nonoverlapping(src, dst, count) };
}

/// CopyChars: 8/16 位码元互拷, 零扩展, 区间不得重叠 (上游模板)
pub fn copy_chars<S: Char, D: Char>(dst: &mut [D], src: &[S]) {
  debug_assert_eq!(dst.len(), src.len());
  // 同宽走 memcpy 快路径 (上游 CopyChars 同类型特化; 等宽位保持)
  if size_of::<S>() == size_of::<D>() {
    let n = dst.len().min(src.len());
    // SAFETY: 等宽 POD 逐元素位保持; dst 独占借用, 两切片不重叠由契约保证
    unsafe {
      copy_nonoverlapping(src.as_ptr() as *const D, dst.as_mut_ptr(), n);
    }
    return;
  }
  for (d, s) in dst.iter_mut().zip(src) {
    *d = D::from_u32(s.to_u32());
  }
}

/// CopyChars 的原始指针形态 (对齐上游签名)
///
/// # Safety
/// 区间不得重叠; count 个元素可达。
#[inline]
pub unsafe fn copy_chars_raw<S: Char, D: Char>(dst: *mut D, src: *const S, count: usize) {
  unsafe {
    for i in 0..count {
      *dst.add(i) = D::from_u32((*src.add(i)).to_u32());
    }
  }
}

/// 码元: u8/u16 零扩展互转 (CopyChars 的类型域)
pub trait Char: Copy {
  fn to_u32(self) -> u32;
  fn from_u32(v: u32) -> Self;
}

impl Char for u8 {
  #[inline]
  fn to_u32(self) -> u32 {
    u32::from(self)
  }
  #[inline]
  fn from_u32(v: u32) -> Self {
    v as u8
  }
}

impl Char for u16 {
  #[inline]
  fn to_u32(self) -> u32 {
    u32::from(self)
  }
  #[inline]
  fn from_u32(v: u32) -> Self {
    v as u16
  }
}

/// Relaxed_Memset: 以 value 填充 count 个 T (上游 base::Relaxed_Memset 的
/// atomic_ref relaxed 原子存储语义; 上游整字分组仅是吞吐优化, 逐元素
/// relaxed 存储的可观测结果一致)
///
/// # Safety
/// destination 起 count 个 T 可写; 并发读者按原子访问对齐读取。
#[inline]
pub unsafe fn relaxed_memset_u8(destination: *mut u8, value: u8, count: usize) {
  for i in 0..count {
    // SAFETY: 调用方保证可写; AtomicU8 与 u8 布局位级兼容 (std 保证)
    unsafe { AtomicU8::from_ptr(destination.add(i)).store(value, Ordering::Relaxed) };
  }
}

/// Relaxed_Memset 的 u16 形态
///
/// # Safety
/// destination 起 count 个 u16 可写; 指针须 u16 对齐。
#[inline]
pub unsafe fn relaxed_memset_u16(destination: *mut u16, value: u16, count: usize) {
  for i in 0..count {
    // SAFETY: 调用方保证可写对齐; AtomicU16 与 u16 布局位级兼容
    unsafe { AtomicU16::from_ptr(destination.add(i)).store(value, Ordering::Relaxed) };
  }
}

/// Relaxed_Memset 的 u32 形态
///
/// # Safety
/// destination 起 count 个 u32 可写; 指针须 u32 对齐。
#[inline]
pub unsafe fn relaxed_memset_u32(destination: *mut u32, value: u32, count: usize) {
  for i in 0..count {
    // SAFETY: 调用方保证可写对齐; AtomicU32 与 u32 布局位级兼容
    unsafe { AtomicU32::from_ptr(destination.add(i)).store(value, Ordering::Relaxed) };
  }
}

/// MemCopyAndSwitchEndianness: 拷贝并逐元素反字节序 (仅大端目标)
///
/// # Safety
/// 区间不得重叠; num_elements*element_size 字节可达。
#[cfg(target_endian = "big")]
pub unsafe fn mem_copy_and_switch_endianness(
  dst: *mut u8,
  src: *const u8,
  num_elements: usize,
  element_size: usize,
) {
  unsafe {
    copy_nonoverlapping(src, dst, num_elements * element_size);
    let mut i = 0;
    while i < num_elements * element_size {
      let mut lo = i;
      let mut hi = i + element_size - 1;
      while lo < hi {
        std::ptr::swap(dst.add(lo), dst.add(hi));
        lo += 1;
        hi -= 1;
      }
      i += element_size;
    }
  }
}
