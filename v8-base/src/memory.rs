//! 原版 v8/src/base/memory.h — 裸地址访存原语

use std::{
  mem::{MaybeUninit, size_of},
  ptr, slice,
};

use crate::build_config::TARGET_LITTLE_ENDIAN;

/// base::Address = uintptr_t
pub type Address = usize;

/// 对齐检查 (macros.h::IsAligned 的本模块私有等价物)
#[inline]
#[must_use]
pub const fn is_aligned(addr: usize, size: usize) -> bool {
  size.is_power_of_two() && addr & (size - 1) == 0
}

/// Memory<T>(addr) — 将裸地址视为 &T; 要求按 T 对齐
///
/// # Safety
/// 地址必须指向已初始化的 T 且满足对齐与生命周期约定。
#[inline]
pub unsafe fn memory<'a, T>(addr: Address) -> &'a T {
  debug_assert!(is_aligned(addr, align_of::<T>()));
  // SAFETY: 契约由调用方保证地址有效且已初始化
  unsafe { &*(addr as *const T) }
}

/// Memory<T>(addr) 可变版本
///
/// # Safety
/// 同 `memory`, 且需独占访问。
#[inline]
pub unsafe fn memory_mut<'a, T>(addr: Address) -> &'a mut T {
  debug_assert!(is_aligned(addr, align_of::<T>()));
  // SAFETY: 契约由调用方保证
  unsafe { &mut *(addr as *mut T) }
}

/// ReadUnalignedValue<T>(p)
///
/// # Safety
/// 地址起 sizeof(T) 字节必须可读。
#[inline]
#[must_use]
pub unsafe fn read_unaligned_value<T: Copy>(addr: Address) -> T {
  debug_assert!(size_of::<T>() > 0);
  // SAFETY: 契约保证 sizeof(T) 字节可读; read_unaligned 无对齐要求
  unsafe { (addr as *const T).read_unaligned() }
}

/// WriteUnalignedValue<T>(p, v)
///
/// # Safety
/// 地址起 sizeof(T) 字节必须可写。
#[inline]
pub unsafe fn write_unaligned_value<T: Copy>(addr: Address, value: T) {
  // SAFETY: 契约保证 sizeof(T) 字节可写
  unsafe { (addr as *mut T).write_unaligned(value) };
}

/// 支持小端读取的原始值类型 (对应模板 V 的平凡可复制约束)
pub trait PrimitiveLe: Copy {
  fn from_le_bytes_slice(bytes: &[u8]) -> Self;
  fn into_le_bytes_slice(self) -> Vec<u8>;
}

macro_rules! impl_primitive_le {
  ($($t:ty),*) => {
    $(
      impl PrimitiveLe for $t {
        fn from_le_bytes_slice(bytes: &[u8]) -> Self {
          let mut buf = [0u8; size_of::<$t>()];
          buf.copy_from_slice(bytes);
          <$t>::from_le_bytes(buf)
        }
        fn into_le_bytes_slice(self) -> Vec<u8> {
          self.to_le_bytes().to_vec()
        }
      }
    )*
  };
}

impl_primitive_le!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);

/// ReadLittleEndianValue<T>(p) — 大端平台反序读取; 当前双平台均为小端直读
///
/// # Safety
/// 地址起 sizeof(T) 字节必须可读。
#[inline]
#[must_use]
pub unsafe fn read_little_endian_value<T: PrimitiveLe>(addr: Address) -> T {
  if TARGET_LITTLE_ENDIAN {
    // SAFETY: 同上
    unsafe { read_unaligned_value::<T>(addr) }
  } else {
    // SAFETY: 契约保证 sizeof(T) 字节可读
    let bytes = unsafe { slice::from_raw_parts(addr as *const u8, size_of::<T>()) };
    let mut rev = bytes.to_vec();
    rev.reverse();
    T::from_le_bytes_slice(&rev)
  }
}

/// WriteLittleEndianValue<T>(p, v)
///
/// # Safety
/// 地址起 sizeof(T) 字节必须可写。
#[inline]
pub unsafe fn write_little_endian_value<T: PrimitiveLe>(addr: Address, value: T) {
  if TARGET_LITTLE_ENDIAN {
    // SAFETY: 契约保证可写
    unsafe { write_unaligned_value(addr, value) };
  } else {
    let mut bytes = value.into_le_bytes_slice();
    bytes.reverse();
    // SAFETY: 契约保证 sizeof(T) 字节可写
    unsafe {
      ptr::copy_nonoverlapping(bytes.as_ptr(), addr as *mut u8, bytes.len());
    }
  }
}

/// bit_cast — 类型间按位重解释 (原版 base::bit_cast 的函数化等价)
#[inline]
#[must_use]
pub fn bit_cast<T: Copy, U: Copy>(v: &T) -> U {
  debug_assert_eq!(size_of::<T>(), size_of::<U>());
  // SAFETY: MaybeUninit 避免全零位模式对部分类型 (如非零判别枚举) 构成 UB;
  // 下方按 size_of::<T>() 全量覆写后 assume_init, 与原版 memcpy 契约一致
  let mut out = MaybeUninit::<U>::uninit();
  unsafe {
    ptr::copy_nonoverlapping(
      v as *const T as *const u8,
      out.as_mut_ptr().cast::<u8>(),
      size_of::<T>(),
    );
    out.assume_init()
  }
}
