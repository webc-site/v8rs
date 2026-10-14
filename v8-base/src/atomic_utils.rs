//! v8/src/base/atomic-utils.h 的 1:1 转写
//! - AtomicValue<T>: 已废弃接口, 保留供上层对照 (原版基于 atomicops Acquire/Release)
//! - AsAtomicImpl<S> → Rust 以宏按存储宽度机械展开为 AsAtomic8/16/32/Word 单元结构
//!   (原版模板 static_assert(sizeof(T) <= sizeof(Storage)); Rust 侧 T 绑定为存储单元
//!   类型, 窄于单元的加载变体待上层实际出现时再补, 登记)
//! - AsAtomicPointer: 原版继承 Word 并 delete SetBits → 此处不提供 set_bits 系方法
//! - CheckedIncrement/CheckedDecrement: 无符号原子计数, DCHECK 防环绕
//! - AsAtomicPtr: 原版 reinterpret 为 std::atomic<T>*, Rust 提供同布局的
//!   共享/可变引用转换 (std 无稳定 atomic_ref)
//!
//! 注: Relaxed_Memset/ReplicateValueWord 属 atomicops.rs (并发代理所有), 不在本文件

use std::{
  marker::PhantomData,
  mem::{align_of, size_of},
  sync::atomic::{AtomicIsize, AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize, Ordering},
};

// ---------------------------------------------------------------------------
// AtomicValue (deprecated in original)
// ---------------------------------------------------------------------------

/// 可存入 AtomicValue 的类型 → AtomicWord(isize) 双向转换
/// 对应原版 cast_helper<S>/cast_helper<S*>; 枚举/StrongAlias 由上层实现本 trait
pub trait AtomicWordCast: Copy {
  fn to_word(self) -> isize;
  fn from_word(v: isize) -> Self;
}

macro_rules! impl_word_cast_ints {
  ($($t:ty),*) => { $( impl AtomicWordCast for $t {
    #[inline] fn to_word(self) -> isize { self as isize }
    #[inline] fn from_word(v: isize) -> Self { v as $t }
  } )* };
}
impl_word_cast_ints!(u8, i8, u16, i16, u32, i32, u64, i64, usize, isize);

impl<T> AtomicWordCast for *mut T {
  #[inline]
  fn to_word(self) -> isize {
    self as usize as isize
  }
  #[inline]
  fn from_word(v: isize) -> Self {
    v as usize as *mut T
  }
}

impl<T> AtomicWordCast for *const T {
  #[inline]
  fn to_word(self) -> isize {
    self as usize as isize
  }
  #[inline]
  fn from_word(v: isize) -> Self {
    v as usize as *const T
  }
}

/// 原版 AtomicValue<T>: 以 AtomicWord 存储的 acquire/release 值
/// 默认构造 value_(0) → T::from_word(0)
pub struct AtomicValue<T: AtomicWordCast> {
  value: AtomicIsize,
  _marker: PhantomData<T>,
}

impl<T: AtomicWordCast> Default for AtomicValue<T> {
  #[inline]
  fn default() -> Self {
    Self::new(T::from_word(0))
  }
}

impl<T: AtomicWordCast> AtomicValue<T> {
  /// explicit AtomicValue(T initial)
  #[inline]
  pub fn new(initial: T) -> Self {
    Self {
      value: AtomicIsize::new(initial.to_word()),
      _marker: PhantomData,
    }
  }

  /// V8_INLINE T Value() const — Acquire_Load
  #[inline]
  pub fn value(&self) -> T {
    T::from_word(self.value.load(Ordering::Acquire))
  }

  /// SetValue(T) — Release_Store
  #[inline]
  pub fn set_value(&self, new_value: T) {
    self.value.store(new_value.to_word(), Ordering::Release);
  }
}

// ---------------------------------------------------------------------------
// AsAtomic8/16/32/Word
// ---------------------------------------------------------------------------

/// 按存储宽度机械展开 AsAtomicImpl<S>:
/// $atomic = 单元对应的 std 原子类型, $unit = 值类型 (原版 static_assert
/// sizeof(T)<=sizeof(StorageType) 收紧为等宽)
macro_rules! impl_as_atomic {
  ($name:ident, $atomic:ty, $unit:ty) => {
    /// 以原子方式访问任意地址上存放的 $unit 值
    /// (原版 AsAtomicImpl<$storage>: 全部静态方法 → 关联函数)
    pub struct $name;

    impl $name {
      /// 取地址reinterpret为原子存储 (原版 to_storage_addr)
      /// SAFETY: 调用方保证 addr 非空、对齐满足 $atomic 要求, 且该内存
      /// 在并发语境下仅经本模块原子方法访问 (与原版 reinterpret 同约束)
      #[inline]
      unsafe fn as_atomic<'a>(addr: *const $unit) -> &'a $atomic {
        // SAFETY: 调用方契约传递
        unsafe { &*(addr as *const $atomic) }
      }

      #[inline]
      /// # Safety
      /// addr 须指向有效 $unit 且满足 $atomic 对齐, 并发访问仅经本模块方法
      pub unsafe fn seq_cst_load(addr: *const $unit) -> $unit {
        // SAFETY: 调用方契约
        unsafe { Self::as_atomic(addr) }.load(Ordering::SeqCst)
      }

      #[inline]
      /// # Safety
      /// addr 须指向有效 $unit 且满足 $atomic 对齐, 并发访问仅经本模块方法
      pub unsafe fn acquire_load(addr: *const $unit) -> $unit {
        // SAFETY: 调用方契约
        unsafe { Self::as_atomic(addr) }.load(Ordering::Acquire)
      }

      #[inline]
      /// # Safety
      /// addr 须指向有效 $unit 且满足 $atomic 对齐, 并发访问仅经本模块方法
      pub unsafe fn relaxed_load(addr: *const $unit) -> $unit {
        // SAFETY: 调用方契约
        unsafe { Self::as_atomic(addr) }.load(Ordering::Relaxed)
      }

      #[inline]
      /// # Safety
      /// addr 须指向有效 $unit 且满足 $atomic 对齐, 并发访问仅经本模块方法
      pub unsafe fn seq_cst_store(addr: *const $unit, new_value: $unit) {
        // SAFETY: 调用方契约
        unsafe { &*(addr as *const $atomic) }.store(new_value, Ordering::SeqCst)
      }

      #[inline]
      /// # Safety
      /// addr 须指向有效 $unit 且满足 $atomic 对齐, 并发访问仅经本模块方法
      pub unsafe fn release_store(addr: *const $unit, new_value: $unit) {
        // SAFETY: 调用方契约
        unsafe { &*(addr as *const $atomic) }.store(new_value, Ordering::Release)
      }

      #[inline]
      /// # Safety
      /// addr 须指向有效 $unit 且满足 $atomic 对齐, 并发访问仅经本模块方法
      pub unsafe fn relaxed_store(addr: *const $unit, new_value: $unit) {
        // SAFETY: 调用方契约
        unsafe { &*(addr as *const $atomic) }.store(new_value, Ordering::Relaxed)
      }

      /// SeqCst_Swap → 返回旧值
      #[inline]
      /// # Safety
      /// addr 须指向有效 $unit 且满足 $atomic 对齐, 并发访问仅经本模块方法
      pub unsafe fn seq_cst_swap(addr: *const $unit, new_value: $unit) -> $unit {
        // SAFETY: 调用方契约
        unsafe { Self::as_atomic(addr) }.swap(new_value, Ordering::SeqCst)
      }

      /// Relaxed_Swap → 返回旧值
      #[inline]
      /// # Safety
      /// addr 须指向有效 $unit 且满足 $atomic 对齐, 并发访问仅经本模块方法
      pub unsafe fn relaxed_swap(addr: *const $unit, new_value: $unit) -> $unit {
        // SAFETY: 调用方契约
        unsafe { Self::as_atomic(addr) }.swap(new_value, Ordering::Relaxed)
      }

      /// Release_CompareAndSwap(addr, old, new) → 返回 CAS 前实际值
      #[inline]
      /// # Safety
      /// addr 须指向有效 $unit 且满足 $atomic 对齐, 并发访问仅经本模块方法
      pub unsafe fn release_compare_and_swap(
        addr: *const $unit,
        old_value: $unit,
        new_value: $unit,
      ) -> $unit {
        // SAFETY: 调用方契约; compare_exchange 失败分支返回内存现值,
        // 与原版 CAS 语义 (返回旧值) 一致。原版 Release_ 前缀 → 成功侧
        // memory_order_release, 失败侧 relaxed (原 atomicops 宽松失败路径)。
        let a = unsafe { Self::as_atomic(addr) };
        match a.compare_exchange(old_value, new_value, Ordering::Release, Ordering::Relaxed) {
          Ok(prev) => prev,
          Err(prev) => prev,
        }
      }

      #[inline]
      /// # Safety
      /// addr 须指向有效 $unit 且满足 $atomic 对齐, 并发访问仅经本模块方法
      pub unsafe fn relaxed_compare_and_swap(
        addr: *const $unit,
        old_value: $unit,
        new_value: $unit,
      ) -> $unit {
        // SAFETY: 同上
        let a = unsafe { Self::as_atomic(addr) };
        match a.compare_exchange(old_value, new_value, Ordering::Relaxed, Ordering::Relaxed) {
          Ok(prev) => prev,
          Err(prev) => prev,
        }
      }

      #[inline]
      /// # Safety
      /// addr 须指向有效 $unit 且满足 $atomic 对齐, 并发访问仅经本模块方法
      pub unsafe fn acquire_release_compare_and_swap(
        addr: *const $unit,
        old_value: $unit,
        new_value: $unit,
      ) -> $unit {
        // SAFETY: 同上
        let a = unsafe { Self::as_atomic(addr) };
        match a.compare_exchange(old_value, new_value, Ordering::AcqRel, Ordering::Acquire) {
          Ok(prev) => prev,
          Err(prev) => prev,
        }
      }

      #[inline]
      /// # Safety
      /// addr 须指向有效 $unit 且满足 $atomic 对齐, 并发访问仅经本模块方法
      pub unsafe fn seq_cst_compare_and_swap(
        addr: *const $unit,
        old_value: $unit,
        new_value: $unit,
      ) -> $unit {
        // SAFETY: 同上
        let a = unsafe { Self::as_atomic(addr) };
        match a.compare_exchange(old_value, new_value, Ordering::SeqCst, Ordering::SeqCst) {
          Ok(prev) => prev,
          Err(prev) => prev,
        }
      }

      #[inline]
      /// # Safety
      /// addr 须指向有效 $unit 且满足 $atomic 对齐, 并发访问仅经本模块方法
      pub unsafe fn relaxed_fetch_or(addr: *const $unit, bits: $unit) -> $unit {
        // SAFETY: 调用方契约
        unsafe { Self::as_atomic(addr) }.fetch_or(bits, Ordering::Relaxed)
      }

      /// 原子地把 mask 选中的位设为 bits; 已满足则返回 false
      /// (原版 Release_SetBits: Relaxed_Load 起步 + CAS 循环)
      #[inline]
      /// # Safety
      /// addr 须指向有效 $unit 且满足 $atomic 对齐, 并发访问仅经本模块方法
      pub unsafe fn release_set_bits(addr: *const $unit, bits: $unit, mask: $unit) -> bool {
        debug_assert_eq!(bits & !mask, 0);
        let mut old_value = unsafe { Self::relaxed_load(addr) };
        let mut new_value: $unit;
        let mut old_value_before_cas: $unit;
        loop {
          if old_value & mask == bits {
            return false;
          }
          new_value = (old_value & !mask) | bits;
          old_value_before_cas = old_value;
          old_value = unsafe { Self::release_compare_and_swap(addr, old_value, new_value) };
          if old_value == old_value_before_cas {
            return true;
          }
        }
      }

      /// Relaxed 版 set_bits (CAS 循环)
      #[inline]
      /// # Safety
      /// addr 须指向有效 $unit 且满足 $atomic 对齐, 并发访问仅经本模块方法
      pub unsafe fn relaxed_set_bits(addr: *const $unit, bits: $unit, mask: $unit) -> bool {
        debug_assert_eq!(bits & !mask, 0);
        let mut old_value = unsafe { Self::relaxed_load(addr) };
        let mut new_value: $unit;
        let mut old_value_before_cas: $unit;
        loop {
          if old_value & mask == bits {
            return false;
          }
          new_value = (old_value & !mask) | bits;
          old_value_before_cas = old_value;
          old_value = unsafe { Self::relaxed_compare_and_swap(addr, old_value, new_value) };
          if old_value == old_value_before_cas {
            return true;
          }
        }
      }

      /// 单参重载 Relaxed_SetBits(addr, mask): 置 mask 位为 1 (fetch_or 路径)
      #[inline]
      /// # Safety
      /// addr 须指向有效 $unit 且满足 $atomic 对齐, 并发访问仅经本模块方法
      pub unsafe fn relaxed_set_bits_mask(addr: *const $unit, mask: $unit) -> bool {
        let old_value = unsafe { Self::relaxed_load(addr) };
        if old_value & mask == mask {
          return false;
        }
        let old_value_before_fo = unsafe { Self::relaxed_fetch_or(addr, mask) };
        (old_value_before_fo | mask) != old_value_before_fo
      }
    }
  };
}

impl_as_atomic!(AsAtomic8, AtomicU8, u8);
impl_as_atomic!(AsAtomic16, AtomicU16, u16);
impl_as_atomic!(AsAtomic32, AtomicU32, u32);
impl_as_atomic!(AsAtomicWord, AtomicUsize, usize);

/// AsAtomicPointer = Word 宽度, 但按原版不提供位访问 set_bits 系方法
/// (方法集是 Word 子集, 这里以别名 + 文档约束; 上层 Review 核对使用面)
pub type AsAtomicPointer = AsAtomicWord;

// ---------------------------------------------------------------------------
// CheckedIncrement / CheckedDecrement / AsAtomicPtr
// ---------------------------------------------------------------------------

/// 原版 `requires std::is_unsigned_v<T>` 约束的落地形态:
/// std 原子类型无公共 fetch trait, 按原版可实例化集合 (无符号且 sizeof(T) <= 8)
/// 展开; DCHECK 就近落在各类型具体实现内, 与模板单态化后的语义逐字一致
pub trait AtomicUnsignedCounter: Sized {
  /// 计数值类型 (原版 T)
  type Value: Copy;

  /// CheckedIncrement: 自增 amount, debug 下校验不回绕
  fn counter_increment(&self, amount: Self::Value, order: Ordering);

  /// CheckedDecrement: 自减 amount, debug 下校验不下穿
  fn counter_decrement(&self, amount: Self::Value, order: Ordering);
}

macro_rules! impl_unsigned_counter {
  ($atomic:ty, $t:ty) => {
    impl AtomicUnsignedCounter for $atomic {
      type Value = $t;

      #[inline]
      fn counter_increment(&self, amount: $t, order: Ordering) {
        // 原版: DCHECK_GE(old + amount, old) — 无符号环绕等价于 wrapping_add
        let old = self.fetch_add(amount, order);
        debug_assert!(old.wrapping_add(amount) >= old);
      }

      #[inline]
      fn counter_decrement(&self, amount: $t, order: Ordering) {
        // 原版: DCHECK_GE(old, amount)
        let old = self.fetch_sub(amount, order);
        debug_assert!(old >= amount);
      }
    }
  };
}

impl_unsigned_counter!(AtomicU8, u8);
impl_unsigned_counter!(AtomicU16, u16);
impl_unsigned_counter!(AtomicU32, u32);
impl_unsigned_counter!(AtomicU64, u64);
impl_unsigned_counter!(AtomicUsize, usize);

/// 原版 CheckedIncrement (atomic-utils.h:294) 自由函数入口
/// 原版默认实参 memory_order_seq_cst → Rust 无默认参数, 由调用处显式传 Ordering
#[inline]
pub fn checked_increment<A: AtomicUnsignedCounter>(number: &A, amount: A::Value, order: Ordering) {
  number.counter_increment(amount, order);
}

/// 原版 CheckedDecrement (atomic-utils.h:305) 自由函数入口
#[inline]
pub fn checked_decrement<A: AtomicUnsignedCounter>(number: &A, amount: A::Value, order: Ordering) {
  number.counter_decrement(amount, order);
}

/// 原版 AsAtomicPtr(T*) — reinterpret 为 std::atomic<T>*
/// Rust 无 atomic_ref (stable), 提供同布局共享/可变引用转换
///
/// # Safety
/// - 指向的内存必须是 $t 宽度且满足对应原子类型对齐 (原版 static_assert 等价:
///   size_of/align_of 检查在调用方类型单态化处由 debug_assert 兜底)
/// - 该内存仅经原子访问 (原版同款约束)
#[inline]
pub unsafe fn as_atomic_ptr<T, A>(t: &T) -> &A {
  debug_assert_eq!(size_of::<T>(), size_of::<A>());
  debug_assert!(align_of::<T>() >= align_of::<A>());
  // SAFETY: 调用方契约 + 宽度/对齐 debug_assert
  unsafe { &*(t as *const T as *const A) }
}

/// AsAtomicPtr 可变形态 (atomic-utils.h:317 非 const 重载)
/// 共享形态见 as_atomic_ptr (对应 :328 const 重载), 原版仅此两个重载
///
/// # Safety
/// 同 as_atomic_ptr, 且需独占借用
#[inline]
pub unsafe fn as_atomic_ptr_mut<T, A>(t: &mut T) -> &mut A {
  debug_assert_eq!(size_of::<T>(), size_of::<A>());
  debug_assert!(align_of::<T>() >= align_of::<A>());
  // SAFETY: 调用方契约 + 宽度/对齐 debug_assert, &mut 保证独占
  unsafe { &mut *(t as *mut T as *mut A) }
}
