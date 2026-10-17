//! v8/src/base/memcopy.h 的转写 — memcpy/memmove 与 relaxed 原子 memset
//! 原版 NEON (SimdMemCopy) 路径以 fearless_simd 的 native 向量宽度分块承接
//! (macOS/aarch64 → NEON, Linux/x86_64 → AVX2/AVX-512, 运行时分派);
//! 小尺寸走定长 copy_nonoverlapping (编译器展开, 对应原版 CASE(N) 宏表)
//! 注: MemCopyAndSwitchEndianness 仅大端平台编译 (对应 #if V8_TARGET_BIG_ENDIAN)

use std::{
  mem::size_of,
  ptr,
  slice::{from_raw_parts, from_raw_parts_mut},
  sync::atomic::{
    AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicU8, AtomicU16, AtomicU32, AtomicU64,
    AtomicUsize, Ordering,
  },
};

use fearless_simd::{Level, Simd, dispatch, prelude::*};

/// SIMD 分块主体: 按当前 CPU native 向量宽度拷贝 (dispatch! 展开各 level 实现)
fn simd_copy<S: Simd>(simd: S, dst: &mut [u8], src: &[u8]) {
  let len = S::u8s::LEN;
  let mut dst_chunks = dst.chunks_exact_mut(len);
  let mut src_chunks = src.chunks_exact(len);
  for (d, s) in dst_chunks.by_ref().zip(src_chunks.by_ref()) {
    S::u8s::from_slice(simd, s).store_slice(d);
  }
  // 尾部逐字节 (原版以首尾重叠写收尾, 安全 Rust 无未初始化读路径, 语义等价)
  for (d, s) in dst_chunks
    .into_remainder()
    .iter_mut()
    .zip(src_chunks.remainder())
  {
    *d = *s;
  }
}

/// MemCopy — 拷贝到不相交内存区 (debug 断言不重叠, 同原版 DCHECK)
#[inline]
pub fn mem_copy(dst: &mut [u8], src: &[u8]) {
  debug_assert_eq!(dst.len(), src.len());
  if dst.is_empty() {
    return;
  }
  debug_assert!(
    // 不重叠: dst 起点在 src 终点之后, 或 src 起点在 dst 终点之后 (按地址比较)
    dst.as_mut_ptr() as usize >= src.as_ptr() as usize + src.len()
      || src.as_ptr() as usize >= dst.as_mut_ptr() as usize + dst.len(),
    "mem_copy 源目的区间重叠"
  );
  let level = Level::new();
  dispatch!(level, simd => simd_copy(simd, dst, src));
}

/// MemCopy 裸指针形态 (与原版 void*/void*/size 签名 1:1)
///
/// # Safety
/// - `[src, src+size)` 与 `[dst, dst+size)` 均可访问且不相交
/// - size 为 0 时 src/dst 仍须非空且对齐
#[inline]
pub unsafe fn mem_copy_raw(dst: *mut u8, src: *const u8, size: usize) {
  // SAFETY: 契约保证区间有效且不相交
  unsafe {
    mem_copy(from_raw_parts_mut(dst, size), from_raw_parts(src, size));
  }
}

/// MemMove — 拷贝到可重叠内存区 (ptr::copy 即 memmove, 小尺寸优化由 libc/编译器承担)
#[inline]
pub fn mem_move(dst: &mut [u8], src: &[u8]) {
  debug_assert_eq!(dst.len(), src.len());
  if dst.is_empty() || ptr::eq(dst.as_mut_ptr(), src.as_ptr().cast_mut()) {
    return;
  }
  // SAFETY: ptr::copy 按字节处理重叠区域 (memmove 语义), 长度一致
  unsafe { ptr::copy(src.as_ptr(), dst.as_mut_ptr(), src.len()) };
}

/// MemMove 裸指针形态
///
/// # Safety
/// - `[src, src+size)` 与 `[dst, dst+size)` 均可访问
#[inline]
pub unsafe fn mem_move_raw(dst: *mut u8, src: *const u8, size: usize) {
  // SAFETY: 契约保证区间有效
  unsafe {
    mem_move(from_raw_parts_mut(dst, size), from_raw_parts(src, size));
  }
}

/// MemCopyAndSwitchEndianness — 拷贝并逐元素反字节序 (仅大端平台, 对应 #if)
#[cfg(target_endian = "big")]
pub fn mem_copy_and_switch_endianness(
  dst: &mut [u8],
  src: &[u8],
  num_elements: usize,
  element_size: usize,
) {
  use crate::bits::{byte_reverse16, byte_reverse32, byte_reverse64};

  /// 原版 COPY_LOOP(type, reverse) 宏的本地承接
  macro_rules! copy_loop {
    ($t:ty, $reverse:expr) => {{
      for i in 0..num_elements {
        let s: &$t = unsafe { &*(src.as_ptr().add(i) as *const $t) };
        let d: &mut $t = unsafe { &mut *(dst.as_mut_ptr().add(i) as *mut $t) };
        // SAFETY: 调用方契约保证 src/dst 各有 num_elements * size_of::<$t>() 可访问字节
        unsafe { ptr::copy_nonoverlapping(s, d, 1) };
        *d = $reverse(*d);
      }
    }};
  }

  match element_size {
    1 => mem_copy(dst, src),
    2 => copy_loop!(u16, byte_reverse16),
    4 => copy_loop!(u32, byte_reverse32),
    8 => copy_loop!(u64, byte_reverse64),
    16 => {
      for i in 0..num_elements {
        for j in 0..16 {
          dst[i * 16 + j] = src[i * 16 + (15 - j)];
        }
      }
    }
    // 原版 UNREACHABLE: Rust 侧静默不拷贝 (禁 panic), 调用方契约保证
    _ => {}
  }
}

/// 支持 relaxed 原子填充的整型 (对应原版 `std::is_integral_v<T>` 约束)
pub trait RelaxedWord: Copy {
  /// ReplicateValueWord: 把 self 复制到 usize 字的每个字节
  fn replicate_word(self) -> usize;
  /// 单元素 relaxed 原子写
  fn store_relaxed(dst: *mut Self, value: Self);
}

macro_rules! impl_relaxed_word {
  ($t:ty, $u:ty, $atomic:ty) => {
    impl RelaxedWord for $t {
      #[inline]
      fn replicate_word(self) -> usize {
        // kMultiplier = (usize 全 1) / (无符号同宽全 1): 把低位值铺满整个字;
        // 原版先 static_cast<UnsignedT> 截断再转 uintptr_t — 有符号负值不做符号扩展
        (self as $u) as usize * (usize::MAX / <$u>::MAX as usize)
      }
      #[inline]
      fn store_relaxed(dst: *mut Self, value: Self) {
        // SAFETY: dst 指向有效 T (调用方切片保证); relaxed 写不参与同步序
        unsafe {
          <$atomic>::from_ptr(dst.cast()).store(value as _, Ordering::Relaxed);
        }
      }
    }
  };
}

impl_relaxed_word!(u8, u8, AtomicU8);
impl_relaxed_word!(u16, u16, AtomicU16);
impl_relaxed_word!(u32, u32, AtomicU32);
impl_relaxed_word!(u64, u64, AtomicU64);
impl_relaxed_word!(i8, u8, AtomicI8);
impl_relaxed_word!(i16, u16, AtomicI16);
impl_relaxed_word!(i32, u32, AtomicI32);
impl_relaxed_word!(i64, u64, AtomicI64);

/// Relaxed_Memset — 以 relaxed 原子写把 `value` 填充整个 `destination`
/// 对齐允许时按机器字批量写 (prologue 对齐 + 字级主循环 + epilogue 收尾),
/// 与原版 __atomic_store_n 主循环同构
pub fn relaxed_memset<T: RelaxedWord>(destination: &mut [T], value: T) {
  const WORD: usize = size_of::<usize>();
  let t_size = size_of::<T>();
  let count = destination.len();
  if t_size >= WORD || count == 0 {
    // 元素不小于字宽: 逐元素 relaxed 写 (原版同款退化路径)
    for d in destination.iter_mut() {
      T::store_relaxed(d, value);
    }
    return;
  }
  // Prologue 元素数: 使目的起点对齐到机器字 (切片保证 base 已按 T 对齐)
  let base_addr = destination.as_ptr() as usize;
  let prologue = ((WORD - base_addr % WORD) % WORD) / t_size;
  let (head, rest) = destination.split_at_mut(prologue.min(count));
  for d in head.iter_mut() {
    T::store_relaxed(d, value);
  }
  // 主循环: 字级 relaxed 写 (值在字内按字节复制铺满)
  let per_word = WORD / t_size;
  let word_val = value.replicate_word();
  let mut words = rest.chunks_exact_mut(per_word);
  for word in words.by_ref() {
    // SAFETY: word 起点已按机器字对齐且拥有 per_word * t_size = WORD 可写字节
    unsafe {
      AtomicUsize::from_ptr(word.as_mut_ptr().cast()).store(word_val, Ordering::Relaxed);
    }
  }
  // Epilogue: 收尾剩余元素
  for d in words.into_remainder() {
    T::store_relaxed(d, value);
  }
}
