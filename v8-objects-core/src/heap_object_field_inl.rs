// 由 tool/src/objects/heap-object-field-inl.h.js 从 v8/src/objects/heap-object-field-inl.h 生成; 修订就地改, 复跑覆盖
// HeapObject 字段访问器骨架 25 项; 沙箱表解析语义归 v8-sandbox

//! HeapObject 的字段级访问器: 沙箱指针 / BoundedSize / 外部指针 /
//! CppHeap 指针 / 间接 (受信) 指针 + Raw*Field 槽位构造。
//!
//! 目标配置 V8_ENABLE_SANDBOX 开: 外指/CppHeap/间接指针字段在沙箱下
//! 存句柄 (ExternalPointerHandle 等), 解析走 ExternalPointerTable —
//! 表依赖归 v8-sandbox, 此处只落签名骨架。

use std::sync::atomic::{
  AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicU8, AtomicU16, AtomicU32,
  AtomicU64, AtomicUsize, Ordering,
};

use crate::heap_object::HeapObject;

/// 适用 Relaxed/Acquire/Release 原子内存读写的整数字段类型
pub trait AtomicField: Copy {
  /// 松弛加载
  ///
  /// # Safety
  /// 调用方须保证 `addr` 指向对齐且合法的可读内存。
  unsafe fn relaxed_load(addr: usize) -> Self;
  /// 松弛存储
  ///
  /// # Safety
  /// 调用方须保证 `addr` 指向对齐且合法的可写内存。
  unsafe fn relaxed_store(addr: usize, val: Self);
  /// 获取式加载
  ///
  /// # Safety
  /// 调用方须保证 `addr` 指向对齐且合法的可读内存。
  unsafe fn acquire_load(addr: usize) -> Self;
  /// 释放式存储
  ///
  /// # Safety
  /// 调用方须保证 `addr` 指向对齐且合法的可写内存。
  unsafe fn release_store(addr: usize, val: Self);
  /// 比较并交换
  ///
  /// # Safety
  /// 调用方须保证 `addr` 指向对齐且合法的可读写内存。
  unsafe fn compare_exchange(addr: usize, current: Self, new: Self) -> Result<Self, Self>;
}

macro_rules! impl_atomic_field {
  ($ty:ident, $atomic_ty:ident) => {
    impl AtomicField for $ty {
      #[inline]
      unsafe fn relaxed_load(addr: usize) -> Self {
        debug_assert_eq!(
          addr % std::mem::align_of::<$ty>(),
          0,
          "Unaligned atomic load"
        );
        unsafe { (*(addr as *const $atomic_ty)).load(Ordering::Relaxed) }
      }
      #[inline]
      unsafe fn relaxed_store(addr: usize, val: Self) {
        debug_assert_eq!(
          addr % std::mem::align_of::<$ty>(),
          0,
          "Unaligned atomic store"
        );
        unsafe { (*(addr as *const $atomic_ty)).store(val, Ordering::Relaxed) }
      }
      #[inline]
      unsafe fn acquire_load(addr: usize) -> Self {
        debug_assert_eq!(
          addr % std::mem::align_of::<$ty>(),
          0,
          "Unaligned atomic load"
        );
        unsafe { (*(addr as *const $atomic_ty)).load(Ordering::Acquire) }
      }
      #[inline]
      unsafe fn release_store(addr: usize, val: Self) {
        debug_assert_eq!(
          addr % std::mem::align_of::<$ty>(),
          0,
          "Unaligned atomic store"
        );
        unsafe { (*(addr as *const $atomic_ty)).store(val, Ordering::Release) }
      }
      #[inline]
      unsafe fn compare_exchange(addr: usize, current: Self, new: Self) -> Result<Self, Self> {
        debug_assert_eq!(
          addr % std::mem::align_of::<$ty>(),
          0,
          "Unaligned atomic cas"
        );
        unsafe {
          (*(addr as *const $atomic_ty)).compare_exchange(
            current,
            new,
            Ordering::SeqCst,
            Ordering::SeqCst,
          )
        }
      }
    }
  };
}

impl_atomic_field!(u8, AtomicU8);
impl_atomic_field!(u16, AtomicU16);
impl_atomic_field!(u32, AtomicU32);
impl_atomic_field!(u64, AtomicU64);
impl_atomic_field!(usize, AtomicUsize);
impl_atomic_field!(i8, AtomicI8);
impl_atomic_field!(i16, AtomicI16);
impl_atomic_field!(i32, AtomicI32);
impl_atomic_field!(i64, AtomicI64);
impl_atomic_field!(isize, AtomicIsize);

impl HeapObject {
  /// 字段裸地址 (原版 field_address: address() + offset, address()=基址即 this)。
  ///
  /// v8: src/objects/heap-object.h HeapObject::field_address
  #[inline]
  pub(crate) fn field_address(&self, offset: usize) -> usize {
    (self as *const HeapObject as usize).wrapping_add(offset)
  }

  /// 字段裸地址 (支持有符号偏移量)
  #[inline]
  pub(crate) fn field_address_signed(&self, offset: i32) -> usize {
    (self as *const HeapObject as usize).wrapping_add_signed(offset as isize)
  }

  /// 读沙箱指针字段 (offset 寻址 + cage base 解码)。
  ///
  /// v8: src/objects/heap-object-field-inl.h HeapObject::ReadSandboxedPointerField
  #[inline]
  pub fn read_sandboxed_pointer_field(&self, offset: usize, cage_base: usize) -> usize {
    let addr = self.field_address(offset);
    unsafe { v8_sandbox::read_sandboxed_pointer_field(addr as *const u64, cage_base) }
  }

  /// 写沙箱指针字段 (cage base 编码后落槽)。
  ///
  /// v8: src/objects/heap-object-field-inl.h HeapObject::WriteSandboxedPointerField
  #[inline]
  pub fn write_sandboxed_pointer_field(&self, offset: usize, cage_base: usize, value: usize) {
    let addr = self.field_address(offset);
    let _ =
      unsafe { v8_sandbox::write_sandboxed_pointer_field(addr as *mut u64, cage_base, value) };
  }

  /// 读 BoundedSize 字段 (40 位有界尺寸, 沙箱内安全)。
  ///
  /// v8: src/objects/heap-object-field-inl.h HeapObject::ReadBoundedSizeField
  #[inline]
  pub fn read_bounded_size_field(&self, offset: usize) -> usize {
    let addr = self.field_address(offset);
    unsafe { v8_sandbox::read_bounded_size_field(addr as *const usize) }
  }

  /// 写 BoundedSize 字段。
  ///
  /// v8: src/objects/heap-object-field-inl.h HeapObject::WriteBoundedSizeField
  #[inline]
  pub fn write_bounded_size_field(&self, offset: usize, value: usize) {
    let addr = self.field_address(offset);
    unsafe { v8_sandbox::write_bounded_size_field(addr as *mut usize, value) };
  }

  /// 读非原子可能未对齐的原始字段
  ///
  /// v8: src/objects/heap-object.h HeapObject::ReadField
  #[inline]
  pub fn read_field<T: Copy>(&self, offset: usize) -> T {
    let addr = self.field_address(offset);
    unsafe { (addr as *const T).read_unaligned() }
  }

  /// 写非原子可能未对齐的原始字段
  ///
  /// v8: src/objects/heap-object.h HeapObject::WriteField
  #[inline]
  pub fn write_field<T: Copy>(&self, offset: usize, value: T) {
    let addr = self.field_address(offset);
    unsafe { (addr as *mut T).write_unaligned(value) }
  }

  /// 松弛原子读字段 (Relaxed)
  ///
  /// v8: src/objects/objects-inl.h HeapObject::Relaxed_ReadField
  #[inline]
  pub fn relaxed_read_field<T: AtomicField>(&self, offset: usize) -> T {
    let addr = self.field_address(offset);
    unsafe { T::relaxed_load(addr) }
  }

  /// 松弛原子写字段 (Relaxed)
  ///
  /// v8: src/objects/objects-inl.h HeapObject::Relaxed_WriteField
  #[inline]
  pub fn relaxed_write_field<T: AtomicField>(&self, offset: usize, value: T) {
    let addr = self.field_address(offset);
    unsafe { T::relaxed_store(addr, value) }
  }

  /// 获取式原子读字段 (Acquire)
  ///
  /// v8: src/objects/objects-inl.h HeapObject::Acquire_ReadField
  #[inline]
  pub fn acquire_read_field<T: AtomicField>(&self, offset: usize) -> T {
    let addr = self.field_address(offset);
    unsafe { T::acquire_load(addr) }
  }

  /// 释放式原子写字段 (Release)
  ///
  /// v8: src/objects/objects-inl.h HeapObject::Release_WriteField
  #[inline]
  pub fn release_write_field<T: AtomicField>(&self, offset: usize, value: T) {
    let addr = self.field_address(offset);
    unsafe { T::release_store(addr, value) }
  }

  /// 比较并交换字段 (CAS)
  ///
  /// v8: src/objects/objects-inl.h HeapObject::SeqCst_CompareAndSwapField
  #[inline]
  pub fn seq_cst_compare_and_swap_field<T: AtomicField>(
    &self,
    offset: usize,
    current: T,
    new: T,
  ) -> Result<T, T> {
    let addr = self.field_address(offset);
    unsafe { T::compare_exchange(addr, current, new) }
  }

  /// 取强指针槽 (RawField) 的槽地址。
  ///
  /// v8: src/objects/heap-object-field-inl.h HeapObject::RawField
  pub fn raw_field(&self, byte_offset: i32) -> usize {
    self.field_address_signed(byte_offset)
  }

  /// 取可能弱指针槽 (RawMaybeWeakField) 的槽地址。
  ///
  /// v8: src/objects/heap-object-field-inl.h HeapObject::RawMaybeWeakField
  pub fn raw_maybe_weak_field(&self, byte_offset: i32) -> usize {
    self.field_address_signed(byte_offset)
  }

  /// 取指令流槽 (RawInstructionStreamField) 的槽地址。
  ///
  /// v8: src/objects/heap-object-field-inl.h HeapObject::RawInstructionStreamField
  pub fn raw_instruction_stream_field(&self, byte_offset: i32) -> usize {
    self.field_address_signed(byte_offset)
  }

  /// 取外指槽 (RawExternalPointerField) 的槽地址 (含 tag range)。
  ///
  /// v8: src/objects/heap-object-field-inl.h HeapObject::RawExternalPointerField
  pub fn raw_external_pointer_field(&self, byte_offset: i32, _tag_range: u32) -> usize {
    // tag_range 为槽类型 (ExternalPointerSlot) 元信息, 返回裸槽地址不涉及。
    self.field_address_signed(byte_offset)
  }

  /// 取 CppHeap 指针槽 (RawCppHeapPointerField) 的槽地址。
  ///
  /// v8: src/objects/heap-object-field-inl.h HeapObject::RawCppHeapPointerField
  pub fn raw_cpp_heap_pointer_field(&self, byte_offset: i32) -> usize {
    self.field_address_signed(byte_offset)
  }

  /// 取间接 (受信) 指针槽 (RawIndirectPointerField) 的槽地址。
  ///
  /// v8: src/objects/heap-object-field-inl.h HeapObject::RawIndirectPointerField
  pub fn raw_indirect_pointer_field(&self, byte_offset: i32, _tag_range: u32) -> usize {
    self.field_address_signed(byte_offset)
  }
}

/// 外指/CppHeap/间接指针字段的初始化-读写-懒初始化族 (模板形参 tag 化,
/// 运行期 tag 变体 2 套; 沙箱句柄表语义归 v8-sandbox)。
pub const FIELD_ACCESSOR_FAMILY: [&str; 13] = [
  "InitExternalPointerField<tag> | InitExternalPointerField(tag)",
  "ReadExternalPointerField<tag_range> | ReadExternalPointerField(tag_range)",
  "ReadCppHeapPointerField<lower,upper> | ReadCppHeapPointerField(tag_range)",
  "WriteExternalPointerField<tag> | WriteExternalPointerField(tag)",
  "SetupLazilyInitializedExternalPointerField",
  "IsLazilyInitializedExternalPointerFieldInitialized",
  "WriteLazilyInitializedExternalPointerField<tag> | WriteLazilyInitializedExternalPointerField(tag)",
  "SetupLazilyInitializedCppHeapPointerField",
  "WriteLazilyInitializedCppHeapPointerField",
  "WriteSandboxedPointerField (Isolate* 变体)",
  "ReadSandboxedPointerField",
  "ReadBoundedSizeField",
  "WriteBoundedSizeField",
];
