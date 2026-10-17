// tool/src/objects/fixed-primitive-array.h.js
// tool/src/objects/fixed-primitive-array-inl.h.js
// 来源: v8/src/objects/fixed-primitive-array.h + fixed-primitive-array-inl.h + fixed-primitive-array.tq
//! FixedPrimitiveArray 族: FixedDoubleArray / ByteArray / TrustedByteArray / FixedIntegerArrayBase。
//!
//! 继承结构:
//! - FixedDoubleArray: 存储 f64 双精度浮点数组 (8 字节对齐元素)
//! - ByteArray: 存储紧凑裸字节数组与非对齐原生类型读写
//! - TrustedByteArray: 受信任空间内的字节数组 (继承 TrustedFixedArrayBase)
//! - FixedIntegerArrayBase: 定宽整数视图 (ByteArray / TrustedByteArray 上)
//!
//! ## 布局双轨制 (AA-1 落地约定, 复用 fixed_array_base)
//! - snapshot 轨: v8_common::K_TAGGED_SIZE (压缩配置 =4) 与
//!   [`fixed_array_base::K_HEADER_SIZE`] (压缩配置 =8), 由 fixed_array_base 锚定。
//! - runtime 轨 (本模块全部读写): Rust 结构体逻辑布局, Tagged 槽 usize 宽 —
//!   `LENGTH_OFFSET = offset_of!(FixedArrayBase, length) = 8`,
//!   `PADDING_OFFSET = 12` (optional_padding 缝), `DATA_OFFSET = size_of::<FixedArrayBase>() = 16`。
//! - 对齐: 原版 SizeFor 走 snapshot 轨 OBJECT_POINTER_ALIGN (v8_common::object_pointer_align,
//!   压缩 4 字节); 本模块 runtime 轨用 [`object_align`] (kObjectAlignment=8), 票单偏差登记。
//!
//! ## 裁剪 / 落地形态登记 (详见票单偏差登记)
//! - `Allocate/New(isolate,…)`: 依赖 factory/roots, 以 [`PrimitiveArray::install`] 在调用方
//!   提供的 8 对齐 buffer 上落地; `length==0` 归一 empty_fixed_array/empty_byte_array 裁剪
//!   (canonical 空数组属 roots 域); 原 `length>kMaxLength` 的 FatalNoSecurityImpact → Err (禁 panic)。
//! - V8_ENABLE_UNDEFINED_DOUBLE = v8_common::V8_UNDEFINED_DOUBLE = false →
//!   set_undefined/is_undefined 与 get 的 undefined 分支裁剪。
//! - SafeHeapObjectSize → u32; Float64 → u64 位型; set_the_hole/is_the_hole 的 isolate
//!   冗余重载与 MoveElements 的 isolate/WriteBarrierMode 参数并入单形态 (无 GC/并发语义)。
//! - FixedDoubleArray::get(array,index,isolate) 返回 Handle (factory NewNumber/the_hole_value)
//!   → 裁剪, 由 get_scalar_checked / Invariant(Hole) 覆盖。
//! - 基类 get/set(ElementMemberT) 按类内联为具体形态 (get_scalar/set 与 get_byte/set_byte),
//!   避免 Rust trait 方法名冲突; UnalignedDoubleMember 读写以 read/write_unaligned::<u64> 等价落地。
//! - DECL_PRINTER/DECL_VERIFIER/BodyDescriptor/V8_TQ 元数据属打印校验与 Torque 域, 不落本 crate。

use std::{
  marker::PhantomData,
  mem, ptr,
  slice::{from_raw_parts, from_raw_parts_mut},
  sync::atomic::Ordering,
};

use v8_base::memory::Address;
use v8_common::{K_HOLE_NAN_INT64, K_MAX_REGULAR_HEAP_OBJECT_SIZE};
use v8_objects_core::{
  instance_type_checker as itc,
  map::Map,
  objects::{Cast, instance_type_of},
  tagged::{Object, Tagged},
  tagged_constants::HEAP_OBJECT_TAG,
};

use crate::{
  error::{Error, Result},
  fixed_array_base::{FixedArrayBase, K_MAX_FIXED_ARRAY_CAPACITY, TrustedFixedArrayBase, u32_slot},
};

/// 规范 Quiet NaN 位模式 (IEEE 754 quiet NaN)
pub const CANONICAL_QUIET_NAN_BITS: u64 = 0x7FF8_0000_0000_0000;

/// 8 字节指针与字长对齐
#[inline]
#[must_use]
pub const fn object_align(val: usize) -> usize {
  (val + 7) & !7
}

// ============================================================================
// §0 PrimitiveArrayBase 布局契约与 install
// ============================================================================

/// 原版 `PrimitiveArrayBase<Derived, ElementT_, Super>` 的落地基契约 (runtime 轨)。
///
/// 具体读写族按类内联于 FixedDoubleArrayExt / ByteArrayExt / TrustedByteArrayExt
/// (C++ 继承语义在 Rust 的布局契约 trait, 对齐 AA-1 TaggedArray 先例)。
pub trait PrimitiveArray: Copy {
  /// length_ 运行时偏移 (snapshot 轨锚点见 fixed_array_base::K_LENGTH_OFFSET)。
  const LENGTH_OFFSET: usize;
  /// optional_padding_ 运行时缝偏移 (length 之后 4 字节; 压缩 snapshot 轨不存在)。
  const PADDING_OFFSET: usize;
  /// OFFSET_OF_DATA_START。
  const DATA_OFFSET: usize;
  /// kElementSize = sizeof(ElementMemberT)。
  const ELEMENT_SIZE: usize;
  /// kMaxLength (fixed-primitive-array.h:95, = kMaxFixedArrayCapacity)。
  const MAX_LENGTH: u32;

  /// OffsetOfElementAt(index) (h:79)。
  #[inline]
  #[must_use]
  fn offset_of_element_at(index: usize) -> usize {
    Self::DATA_OFFSET + index * Self::ELEMENT_SIZE
  }

  /// FromAddressOfFirstElement(address) (inl.h:88-95)。
  ///
  /// # Safety
  /// `address` 必须是真实对象数据区首元素地址; 原版 DCHECK_TAG_ALIGNED,
  /// 此处按 runtime 轨断言 8 字节对齐 (票单偏差)。
  #[inline]
  #[must_use]
  unsafe fn from_address_of_first_element(address: Address) -> Tagged<Self> {
    debug_assert!(address.is_multiple_of(8), "首元素地址须 8 字节对齐");
    // SAFETY: 调用方契约; 减 DATA_OFFSET 还原对齐基址后加 kHeapObjectTag
    unsafe { Tagged::from_raw_address(address - Self::DATA_OFFSET) }
  }

  /// 原版 Allocate/New 的落地形式 (inl.h:97-124 + 各类 New): 在调用方 8 对齐 `buffer`
  /// 上写 map 槽与 length, 清 padding 缝, 并将尾部填充 [written, object_align(written))
  /// 清零 (ByteArray/TrustedByteArray New 的 memset; FixedDoubleArray 间隙恒 0 同路径无害)。
  ///
  /// # Safety
  /// - `map` 为有效 Map 对象指针;
  /// - `buffer` 8 字节对齐且至少 `size_for(length)` 字节可写;
  /// - 该区域无并发/别名访问。
  unsafe fn install(map: Tagged<Map>, length: u32, buffer: Address) -> Result<Tagged<Self>> {
    if length > Self::MAX_LENGTH {
      return Err(Error::Unsupported(
        "New: length > kMaxLength (原版 FatalNoSecurityImpact → Err, 禁 panic)",
      ));
    }
    debug_assert!(buffer.is_multiple_of(8), "install 基址须 8 字节对齐");
    // SAFETY: 调用方契约保证对齐、可写与别名排除
    unsafe {
      // map 槽: runtime 轨 Tagged 槽 usize 宽, 存含堆对象 tag 的指针值
      ptr::write(buffer as *mut usize, map.ptr());
      u32_slot::store(buffer + Self::LENGTH_OFFSET, length);
      u32_slot::store(buffer + Self::PADDING_OFFSET, 0);
      let written = Self::DATA_OFFSET + (length as usize) * Self::ELEMENT_SIZE;
      let total = object_align(written);
      if total > written {
        ptr::write_bytes((buffer as *mut u8).add(written), 0, total - written);
      }
    }
    Ok(Tagged::new(buffer + HEAP_OBJECT_TAG))
  }
}

/// FixedIntegerArrayBase 的 Base 类型域: ByteArray 或 TrustedByteArray (原版模板约束)。
pub trait ByteBase: PrimitiveArray {}

// 编译期布局锚点 (runtime 轨; 原版 static_assert 布局对偶, snapshot 轨由 fixed_array_base 锚定)
const _: () = assert!(mem::size_of::<FixedArrayBase>() == 16);
const _: () = assert!(mem::size_of::<TrustedFixedArrayBase>() == 16);
const _: () = assert!(mem::offset_of!(FixedArrayBase, length) == 8);

// ============================================================================
// §1 FixedDoubleArray
// ============================================================================

/// FixedDoubleArray: 存储双精度浮点数 (f64) 的定长数组
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct FixedDoubleArray {
  /// 继承 FixedArrayBase (map: Tagged<Map>, length: u32, padding: 4 字节)
  pub base: FixedArrayBase,
}

impl FixedDoubleArray {
  /// 数据区首地址偏移 (16 字节)
  pub const DATA_OFFSET: usize = mem::size_of::<FixedArrayBase>();
  /// 元素字节尺寸 (8 字节)
  pub const ELEMENT_SIZE: usize = mem::size_of::<f64>();
  /// 最大容量 (128M)
  pub const MAX_CAPACITY: u32 = K_MAX_FIXED_ARRAY_CAPACITY;

  /// 构造 FixedDoubleArray 头部
  #[inline]
  pub const fn new(map: Tagged<Map>, length: u32) -> Self {
    Self {
      base: FixedArrayBase::new(map, length),
    }
  }

  /// 依据长度计算全对象所需字节大小 (含头部与 8 字节对齐)
  #[inline]
  #[must_use]
  pub const fn size_for(length: u32) -> usize {
    object_align(Self::DATA_OFFSET + (length as usize) * Self::ELEMENT_SIZE)
  }

  /// 依据容量计算最大常规堆对象允许长度
  #[inline]
  #[must_use]
  pub const fn max_regular_length() -> usize {
    (K_MAX_REGULAR_HEAP_OBJECT_SIZE - Self::DATA_OFFSET) / Self::ELEMENT_SIZE
  }

  /// 原版 New(isolate, length, elements_callback) 的落地形式 (inl.h:142-162):
  /// 先 install 再逐槽经 set 写入 (NaN 规整语义同 [FixedDoubleArrayExt::set])。
  ///
  /// # Safety
  /// 同 [PrimitiveArray::install]。
  #[inline]
  pub unsafe fn new_with<F>(
    map: Tagged<Map>,
    length: u32,
    buffer: Address,
    elements_callback: F,
  ) -> Result<Tagged<Self>>
  where
    F: Fn(usize) -> f64,
  {
    // SAFETY: 调用方保证 buffer 为 8 字节对齐且容量充足的可用内存
    let array: Tagged<FixedDoubleArray> = unsafe { Self::install(map, length, buffer)? };
    // SAFETY: i < length, 均落在刚安装的数据区内
    unsafe {
      for i in 0..length as usize {
        array.set(i as u32, elements_callback(i));
      }
    }
    Ok(array)
  }
}

impl PrimitiveArray for FixedDoubleArray {
  const LENGTH_OFFSET: usize = mem::offset_of!(FixedArrayBase, length);
  const PADDING_OFFSET: usize = Self::LENGTH_OFFSET + 4;
  const DATA_OFFSET: usize = FixedDoubleArray::DATA_OFFSET;
  const ELEMENT_SIZE: usize = FixedDoubleArray::ELEMENT_SIZE;
  const MAX_LENGTH: u32 = FixedDoubleArray::MAX_CAPACITY;
}

impl Cast for FixedDoubleArray {
  #[inline]
  fn allow_from(value: Tagged<Object>) -> bool {
    instance_type_of(value).is_some_and(itc::is_fixed_double_array)
  }
}

/// Tagged<FixedDoubleArray> 专有操作与索引读写
pub trait FixedDoubleArrayExt: Copy {
  /// 获取长度
  fn length(&self) -> u32;

  /// length(kRelaxedLoad)
  ///
  /// # Safety
  /// 宿主指针有效。
  unsafe fn length_relaxed(&self) -> u32;

  /// length(kAcquireLoad)
  ///
  /// # Safety
  /// 宿主指针有效。
  unsafe fn length_acquire(&self) -> u32;

  /// set_length(value)
  ///
  /// # Safety
  /// 宿主指针有效可写且无别名并发写。
  unsafe fn set_length(&self, value: u32);

  /// set_length(value, kReleaseStore)
  ///
  /// # Safety
  /// 宿主指针有效可写。
  unsafe fn set_length_release(&self, value: u32);

  /// 依据 acquire 长度计算分配字节大小 (AllocatedSize, 右剪裁并发契约)
  ///
  /// # Safety
  /// 宿主指针有效。
  unsafe fn allocated_size(&self) -> usize;

  /// capacity() — 原始数组容量即长度别名 (h:53; SafeHeapObjectSize→u32)
  #[inline]
  #[must_use]
  fn capacity(&self) -> u32 {
    self.length()
  }

  /// capacity(kAcquireLoad) (h:56)
  ///
  /// # Safety
  /// 宿主指针有效。
  #[inline]
  unsafe fn capacity_acquire(&self) -> u32 {
    // SAFETY: 调用方契约透传 length_acquire
    unsafe { self.length_acquire() }
  }

  /// capacity(kRelaxedLoad) (h:59)
  ///
  /// # Safety
  /// 宿主指针有效。
  #[inline]
  unsafe fn capacity_relaxed(&self) -> u32 {
    // SAFETY: 调用方契约透传 length_relaxed
    unsafe { self.length_relaxed() }
  }

  /// set_capacity(value) = set_length (h:62)
  ///
  /// # Safety
  /// 宿主指针有效可写。
  #[inline]
  unsafe fn set_capacity(&self, value: u32) {
    // SAFETY: 调用方契约透传 set_length
    unsafe { self.set_length(value) };
  }

  /// set_capacity(value, kReleaseStore) (h:63)
  ///
  /// # Safety
  /// 宿主指针有效可写。
  #[inline]
  unsafe fn set_capacity_release(&self, value: u32) {
    // SAFETY: 调用方契约透传 set_length_release
    unsafe { self.set_length_release(value) };
  }

  /// ulength() 无符号长度视图 (IsInBounds/DataSize 内部域)
  ///
  /// # Safety
  /// 宿主指针有效。
  #[inline]
  unsafe fn ulength(&self) -> u32 {
    // SAFETY: 调用方契约; relaxed 即原版语义
    unsafe { self.length_relaxed() }
  }

  /// begin() 数据区首地址 (仅指针运算无引用, 返回 Address 避免对齐悬垂)
  #[must_use]
  fn begin(&self) -> Address;

  /// end() 末元素之后 (inl.h:70-78)
  ///
  /// # Safety
  /// 宿主指针有效。
  unsafe fn end(&self) -> Address;

  /// DataSize() 数据区字节大小 (inl.h:80-86)
  ///
  /// # Safety
  /// 宿主指针有效。
  unsafe fn data_size(&self) -> usize;

  /// clear_optional_padding() (h:67; runtime 轨 padding 缝恒存在)
  ///
  /// # Safety
  /// 宿主指针有效可写。
  unsafe fn clear_optional_padding(&self);

  /// 安全检查下标是否在边界内 (原版 IsInBounds(int) 以 unsigned 比较, 此处 u32 入参即其值域)
  #[inline]
  fn is_in_bounds(&self, index: u32) -> bool {
    index < self.length()
  }

  /// 检查指定下标元素是否为 Hole 特殊标记
  ///
  /// # Safety
  /// index 必须小于 length() 且指针有效。
  unsafe fn is_the_hole(&self, index: u32) -> bool;

  /// 读取浮点标量值 (DCHECK 非 Hole)
  ///
  /// # Safety
  /// index 必须小于 length() 且元素不能为 Hole。
  unsafe fn get_scalar(&self, index: u32) -> f64;

  /// 读取原始 64 位整数表示
  ///
  /// # Safety
  /// index 必须小于 length()。
  unsafe fn get_representation(&self, index: u32) -> u64;

  /// 写入浮点数，若为 NaN 则规整为标准 quiet NaN
  ///
  /// # Safety
  /// index 必须小于 length() 且内存具备写权限。
  unsafe fn set(&self, index: u32, value: f64);

  /// 写入原始 64 位比特值
  ///
  /// # Safety
  /// index 必须小于 length()。
  unsafe fn set_raw(&self, index: u32, bits: u64);

  /// 将指定槽位设置为 Hole 标记 (kHoleNanInt64)
  ///
  /// # Safety
  /// index 必须小于 length()。
  unsafe fn set_the_hole(&self, index: u32);

  /// 区间填充 Hole 标记 [from, to)
  ///
  /// # Safety
  /// to 必须小于等于 length()。
  unsafe fn fill_with_holes(&self, from: u32, to: u32);

  /// 元素切片搬迁 (允许内存重叠)
  ///
  /// # Safety
  /// dst_index + len 与 src_index + len 均不能越界。
  unsafe fn move_elements(&self, dst_index: u32, src_index: u32, len: u32);

  /// 边界检查的安全读取
  fn get_scalar_checked(&self, index: usize) -> Result<f64>;

  /// 边界检查的安全写入
  fn set_checked(&self, index: usize, value: f64) -> Result<()>;

  /// 获取原始 f64 切片只读视图
  ///
  /// # Safety
  /// 指针有效且切片存活期内无并发写。
  unsafe fn as_f64_slice(&self) -> &[f64];

  /// 获取原始 f64 切片可变视图
  ///
  /// # Safety
  /// 指针有效可写且无并发/别名访问。
  unsafe fn as_f64_slice_mut(&mut self) -> &mut [f64];
}

impl FixedDoubleArrayExt for Tagged<FixedDoubleArray> {
  #[inline]
  fn length(&self) -> u32 {
    let addr = self.address() + mem::offset_of!(FixedArrayBase, length);
    // SAFETY: 对象头在已分配范围内
    unsafe { u32_slot::load(addr) }
  }

  #[inline]
  unsafe fn length_relaxed(&self) -> u32 {
    let addr = self.address() + mem::offset_of!(FixedArrayBase, length);
    // SAFETY: 调用方保证指针有效
    unsafe { u32_slot::load_order(addr, Ordering::Relaxed) }
  }

  #[inline]
  unsafe fn length_acquire(&self) -> u32 {
    let addr = self.address() + mem::offset_of!(FixedArrayBase, length);
    // SAFETY: 调用方保证指针有效
    unsafe { u32_slot::load_order(addr, Ordering::Acquire) }
  }

  #[inline]
  unsafe fn set_length(&self, value: u32) {
    let addr = self.address() + mem::offset_of!(FixedArrayBase, length);
    // SAFETY: 调用方保证有效且无别名
    unsafe { u32_slot::store(addr, value) };
  }

  #[inline]
  unsafe fn set_length_release(&self, value: u32) {
    let addr = self.address() + mem::offset_of!(FixedArrayBase, length);
    // SAFETY: 调用方保证有效
    unsafe { u32_slot::store_order(addr, value, Ordering::Release) };
  }

  #[inline]
  unsafe fn allocated_size(&self) -> usize {
    // SAFETY: 调用方保证宿主指针有效
    unsafe { FixedDoubleArray::size_for(self.length_acquire()) }
  }

  #[inline]
  fn begin(&self) -> Address {
    self.address() + FixedDoubleArray::DATA_OFFSET
  }

  #[inline]
  unsafe fn end(&self) -> Address {
    // SAFETY: 调用方保证宿主指针有效
    self.begin() + (unsafe { self.ulength() } as usize) * FixedDoubleArray::ELEMENT_SIZE
  }

  #[inline]
  unsafe fn data_size(&self) -> usize {
    // SAFETY: 调用方保证宿主指针有效
    unsafe { FixedDoubleArray::size_for(self.ulength()) - FixedDoubleArray::DATA_OFFSET }
  }

  #[inline]
  unsafe fn clear_optional_padding(&self) {
    // SAFETY: 调用方保证宿主指针可写; padding 缝恒在对象头范围内
    unsafe {
      u32_slot::store(
        self.address() + <FixedDoubleArray as PrimitiveArray>::PADDING_OFFSET,
        0,
      );
    }
  }

  #[inline]
  unsafe fn is_the_hole(&self, index: u32) -> bool {
    // SAFETY: 调用方保证 index < length 且指针有效
    unsafe {
      debug_assert!(self.is_in_bounds(index));
      self.get_representation(index) == K_HOLE_NAN_INT64
    }
  }

  #[inline]
  unsafe fn get_scalar(&self, index: u32) -> f64 {
    // SAFETY: 调用方保证 index < length, 非 Hole 且指针有效
    unsafe {
      debug_assert!(self.is_in_bounds(index));
      debug_assert!(!self.is_the_hole(index));
      let bits = self.get_representation(index);
      f64::from_bits(bits)
    }
  }

  #[inline]
  unsafe fn get_representation(&self, index: u32) -> u64 {
    // SAFETY: 调用方保证 index < length 且指针有效
    unsafe {
      debug_assert!(self.is_in_bounds(index));
      let slot = (self.address()
        + FixedDoubleArray::DATA_OFFSET
        + (index as usize) * FixedDoubleArray::ELEMENT_SIZE) as *const u64;
      ptr::read_unaligned(slot)
    }
  }

  #[inline]
  unsafe fn set(&self, index: u32, value: f64) {
    // SAFETY: 调用方保证 index < length 且内存具备写权限
    unsafe {
      debug_assert!(self.is_in_bounds(index));
      let bits = if value.is_nan() {
        CANONICAL_QUIET_NAN_BITS
      } else {
        value.to_bits()
      };
      self.set_raw(index, bits);
    }
  }

  #[inline]
  unsafe fn set_raw(&self, index: u32, bits: u64) {
    // SAFETY: 调用方保证 index < length 且内存具备写权限
    unsafe {
      debug_assert!(self.is_in_bounds(index));
      let slot = (self.address()
        + FixedDoubleArray::DATA_OFFSET
        + (index as usize) * FixedDoubleArray::ELEMENT_SIZE) as *mut u64;
      ptr::write_unaligned(slot, bits);
    }
  }

  #[inline]
  unsafe fn set_the_hole(&self, index: u32) {
    // SAFETY: 调用方保证 index < length
    unsafe {
      debug_assert!(self.is_in_bounds(index));
      self.set_raw(index, K_HOLE_NAN_INT64);
    }
  }

  #[inline]
  unsafe fn fill_with_holes(&self, from: u32, to: u32) {
    if from >= to {
      return;
    }
    // SAFETY: 调用方契约保证 [from, to) 在有效数据区内
    unsafe {
      debug_assert!(to <= self.length());
      for i in from..to {
        self.set_the_hole(i);
      }
    }
  }

  #[inline]
  unsafe fn move_elements(&self, dst_index: u32, src_index: u32, len: u32) {
    if len == 0 {
      return;
    }
    // SAFETY: 调用方契约保证指针有效且区间不越界
    unsafe {
      debug_assert!(
        dst_index
          .checked_add(len)
          .is_some_and(|end| end <= self.length())
      );
      debug_assert!(
        src_index
          .checked_add(len)
          .is_some_and(|end| end <= self.length())
      );
      let base = self.address() + FixedDoubleArray::DATA_OFFSET;
      let src = (base + (src_index as usize) * FixedDoubleArray::ELEMENT_SIZE) as *const u8;
      let dst = (base + (dst_index as usize) * FixedDoubleArray::ELEMENT_SIZE) as *mut u8;
      ptr::copy(src, dst, (len as usize) * FixedDoubleArray::ELEMENT_SIZE);
    }
  }

  fn get_scalar_checked(&self, index: usize) -> Result<f64> {
    let len = self.length() as usize;
    if index >= len {
      return Err(Error::OutOfBounds { index, length: len });
    }
    let idx = index as u32;
    // SAFETY: index < len 且未越界; 单次读出 representation 规避重复内存加载
    let bits = unsafe { self.get_representation(idx) };
    if bits == K_HOLE_NAN_INT64 {
      return Err(Error::Invariant("访问了 Hole 标记元素"));
    }
    Ok(f64::from_bits(bits))
  }

  fn set_checked(&self, index: usize, value: f64) -> Result<()> {
    let len = self.length() as usize;
    if index >= len {
      return Err(Error::OutOfBounds { index, length: len });
    }
    // SAFETY: index < len 边界校验通过
    unsafe {
      self.set(index as u32, value);
    }
    Ok(())
  }

  #[inline]
  unsafe fn as_f64_slice(&self) -> &[f64] {
    let len = self.length() as usize;
    let ptr = (self.address() + FixedDoubleArray::DATA_OFFSET) as *const f64;
    // SAFETY: 调用方契约; FixedDoubleArray 元素 8 字节对齐且在对象内存范围内
    unsafe { from_raw_parts(ptr, len) }
  }

  #[inline]
  unsafe fn as_f64_slice_mut(&mut self) -> &mut [f64] {
    let len = self.length() as usize;
    let ptr = (self.address() + FixedDoubleArray::DATA_OFFSET) as *mut f64;
    // SAFETY: 调用方契约保证无并发/别名写入
    unsafe { from_raw_parts_mut(ptr, len) }
  }
}

// ============================================================================
// §2 ByteArray
// ============================================================================

/// ByteArray: 连续存储紧凑任意字节序列的堆对象
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ByteArray {
  /// 继承 FixedArrayBase
  pub base: FixedArrayBase,
}

impl ByteArray {
  /// 数据区偏移 (16 字节)
  pub const DATA_OFFSET: usize = mem::size_of::<FixedArrayBase>();
  /// 元素字节宽度 (1 字节)
  pub const ELEMENT_SIZE: usize = 1;
  /// 最大容量限制
  pub const MAX_CAPACITY: u32 = K_MAX_FIXED_ARRAY_CAPACITY;

  /// 构造 ByteArray 头部
  #[inline]
  pub const fn new(map: Tagged<Map>, length: u32) -> Self {
    Self {
      base: FixedArrayBase::new(map, length),
    }
  }

  /// 依据对象全尺寸字节数换算有效负载长度
  #[inline]
  #[must_use]
  pub const fn length_for(size_in_bytes: usize) -> usize {
    size_in_bytes.saturating_sub(Self::DATA_OFFSET)
  }

  /// 依据元素长度计算对象总字节大小 (带 8 字节指针字长对齐)
  #[inline]
  #[must_use]
  pub const fn size_for(length: u32) -> usize {
    object_align(Self::DATA_OFFSET + length as usize)
  }

  /// 依据容量计算最大常规堆对象允许长度
  #[inline]
  #[must_use]
  pub const fn max_regular_length() -> usize {
    (K_MAX_REGULAR_HEAP_OBJECT_SIZE - Self::DATA_OFFSET) / Self::ELEMENT_SIZE
  }
}

impl PrimitiveArray for ByteArray {
  const LENGTH_OFFSET: usize = mem::offset_of!(FixedArrayBase, length);
  const PADDING_OFFSET: usize = Self::LENGTH_OFFSET + 4;
  const DATA_OFFSET: usize = ByteArray::DATA_OFFSET;
  const ELEMENT_SIZE: usize = ByteArray::ELEMENT_SIZE;
  const MAX_LENGTH: u32 = ByteArray::MAX_CAPACITY;
}

impl ByteBase for ByteArray {}

impl Cast for ByteArray {
  #[inline]
  fn allow_from(value: Tagged<Object>) -> bool {
    instance_type_of(value).is_some_and(itc::is_byte_array)
  }
}

/// Tagged<ByteArray> 字节读写与原生类型访问扩展
pub trait ByteArrayExt: Copy {
  /// 获取字节长度
  fn length(&self) -> u32;

  /// length(kRelaxedLoad)
  ///
  /// # Safety
  /// 宿主指针有效。
  unsafe fn length_relaxed(&self) -> u32;

  /// length(kAcquireLoad)
  ///
  /// # Safety
  /// 宿主指针有效。
  unsafe fn length_acquire(&self) -> u32;

  /// set_length(value)
  ///
  /// # Safety
  /// 宿主指针有效可写且无并发写。
  unsafe fn set_length(&self, value: u32);

  /// set_length(value, kReleaseStore)
  ///
  /// # Safety
  /// 宿主指针有效可写。
  unsafe fn set_length_release(&self, value: u32);

  /// 依据 acquire 长度计算分配字节大小 (AllocatedSize)
  ///
  /// # Safety
  /// 宿主指针有效。
  unsafe fn allocated_size(&self) -> usize;

  /// capacity() — 容量即长度别名 (SafeHeapObjectSize→u32)
  #[inline]
  #[must_use]
  fn capacity(&self) -> u32 {
    self.length()
  }

  /// capacity(kAcquireLoad)
  ///
  /// # Safety
  /// 宿主指针有效。
  #[inline]
  unsafe fn capacity_acquire(&self) -> u32 {
    // SAFETY: 调用方契约透传 length_acquire
    unsafe { self.length_acquire() }
  }

  /// capacity(kRelaxedLoad)
  ///
  /// # Safety
  /// 宿主指针有效。
  #[inline]
  unsafe fn capacity_relaxed(&self) -> u32 {
    // SAFETY: 调用方契约透传 length_relaxed
    unsafe { self.length_relaxed() }
  }

  /// set_capacity(value) = set_length
  ///
  /// # Safety
  /// 宿主指针有效可写。
  #[inline]
  unsafe fn set_capacity(&self, value: u32) {
    // SAFETY: 调用方契约透传 set_length
    unsafe { self.set_length(value) };
  }

  /// set_capacity(value, kReleaseStore)
  ///
  /// # Safety
  /// 宿主指针有效可写。
  #[inline]
  unsafe fn set_capacity_release(&self, value: u32) {
    // SAFETY: 调用方契约透传 set_length_release
    unsafe { self.set_length_release(value) };
  }

  /// ulength() 无符号长度视图
  ///
  /// # Safety
  /// 宿主指针有效。
  #[inline]
  unsafe fn ulength(&self) -> u32 {
    // SAFETY: 调用方契约; relaxed 即原版语义
    unsafe { self.length_relaxed() }
  }

  /// begin() 数据区首地址
  #[must_use]
  fn begin(&self) -> Address;

  /// end() 末元素之后
  ///
  /// # Safety
  /// 宿主指针有效。
  unsafe fn end(&self) -> Address;

  /// DataSize() 数据区字节大小
  ///
  /// # Safety
  /// 宿主指针有效。
  unsafe fn data_size(&self) -> usize;

  /// clear_optional_padding() (runtime 轨 padding 缝)
  ///
  /// # Safety
  /// 宿主指针有效可写。
  unsafe fn clear_optional_padding(&self);

  /// 读取单字节
  ///
  /// # Safety
  /// index 必须小于 length()。
  unsafe fn get_byte(&self, index: u32) -> u8;

  /// 写入单字节
  ///
  /// # Safety
  /// index 必须小于 length()。
  unsafe fn set_byte(&self, index: u32, value: u8);

  /// 读取非对齐 32 位无符号整数 (小端)
  ///
  /// # Safety
  /// offset + 4 必须小于等于 length()。
  unsafe fn get_u32(&self, offset: usize) -> u32;

  /// 写入非对齐 32 位无符号整数 (小端)
  ///
  /// # Safety
  /// offset + 4 必须小于等于 length()。
  unsafe fn set_u32(&self, offset: usize, value: u32);

  /// 读取非对齐 16 位无符号整数 (小端)
  ///
  /// # Safety
  /// offset + 2 必须小于等于 length()。
  unsafe fn get_u16(&self, offset: usize) -> u16;

  /// 写入非对齐 16 位无符号整数 (小端)
  ///
  /// # Safety
  /// offset + 2 必须小于等于 length()。
  unsafe fn set_u16(&self, offset: usize, value: u16);

  /// 读取非对齐 64 位无符号整数 (小端)
  ///
  /// # Safety
  /// offset + 8 必须小于等于 length()。
  unsafe fn get_u64(&self, offset: usize) -> u64;

  /// 写入非对齐 64 位无符号整数 (小端)
  ///
  /// # Safety
  /// offset + 8 必须小于等于 length()。
  unsafe fn set_u64(&self, offset: usize, value: u64);

  /// 原版 C++ get_int (以 offset 读 32 位整数)
  ///
  /// # Safety
  /// offset + 4 必须小于等于 length()。
  unsafe fn get_int(&self, offset: usize) -> u32 {
    // SAFETY: 调用方满足 offset + 4 <= length 且对象指针有效
    unsafe { self.get_u32(offset) }
  }

  /// 原版 C++ set_int (以 offset 写 32 位整数)
  ///
  /// # Safety
  /// offset + 4 必须小于等于 length()。
  unsafe fn set_int(&self, offset: usize, value: u32) {
    // SAFETY: 调用方满足 offset + 4 <= length 且对象指针有效
    unsafe { self.set_u32(offset, value) }
  }

  /// 边界检查的安全读字节
  fn get_byte_checked(&self, index: usize) -> Result<u8>;

  /// 边界检查的安全写字节
  fn set_byte_checked(&self, index: usize, value: u8) -> Result<()>;

  /// 切片复制
  ///
  /// # Safety
  /// dst_offset + len 与 src_offset + len 必须不越界。
  unsafe fn copy_bytes(&self, dst_offset: usize, src_offset: usize, len: usize);

  /// 获取底层字节切片只读视图
  ///
  /// # Safety
  /// 数组指针有效且切片存活期内无并发写入。
  unsafe fn raw_bytes(&self) -> &[u8];

  /// 获取底层字节切片可变视图
  ///
  /// # Safety
  /// 数组指针有效可写且无并发/别名写入。
  unsafe fn raw_bytes_mut(&mut self) -> &mut [u8];
}

impl ByteArrayExt for Tagged<ByteArray> {
  #[inline]
  fn length(&self) -> u32 {
    let addr = self.address() + mem::offset_of!(FixedArrayBase, length);
    // SAFETY: 对象头在已分配范围内
    unsafe { u32_slot::load(addr) }
  }

  #[inline]
  unsafe fn length_relaxed(&self) -> u32 {
    let addr = self.address() + mem::offset_of!(FixedArrayBase, length);
    // SAFETY: 调用方保证指针有效
    unsafe { u32_slot::load_order(addr, Ordering::Relaxed) }
  }

  #[inline]
  unsafe fn length_acquire(&self) -> u32 {
    let addr = self.address() + mem::offset_of!(FixedArrayBase, length);
    // SAFETY: 调用方保证指针有效
    unsafe { u32_slot::load_order(addr, Ordering::Acquire) }
  }

  #[inline]
  unsafe fn set_length(&self, value: u32) {
    let addr = self.address() + mem::offset_of!(FixedArrayBase, length);
    // SAFETY: 调用方保证有效且无别名
    unsafe { u32_slot::store(addr, value) };
  }

  #[inline]
  unsafe fn set_length_release(&self, value: u32) {
    let addr = self.address() + mem::offset_of!(FixedArrayBase, length);
    // SAFETY: 调用方保证有效
    unsafe { u32_slot::store_order(addr, value, Ordering::Release) };
  }

  #[inline]
  unsafe fn allocated_size(&self) -> usize {
    // SAFETY: 调用方保证指针有效，读取 acquire 语义 length 计算对象尺寸
    let len = unsafe { self.length_acquire() };
    ByteArray::size_for(len)
  }

  #[inline]
  fn begin(&self) -> Address {
    self.address() + ByteArray::DATA_OFFSET
  }

  #[inline]
  unsafe fn end(&self) -> Address {
    // SAFETY: 调用方保证宿主指针有效
    self.begin() + (unsafe { self.ulength() } as usize) * ByteArray::ELEMENT_SIZE
  }

  #[inline]
  unsafe fn data_size(&self) -> usize {
    // SAFETY: 调用方保证宿主指针有效
    unsafe { ByteArray::size_for(self.ulength()) - ByteArray::DATA_OFFSET }
  }

  #[inline]
  unsafe fn clear_optional_padding(&self) {
    // SAFETY: 调用方保证宿主指针可写; padding 缝恒在对象头范围内
    unsafe {
      u32_slot::store(
        self.address() + <ByteArray as PrimitiveArray>::PADDING_OFFSET,
        0,
      );
    }
  }

  #[inline]
  unsafe fn get_byte(&self, index: u32) -> u8 {
    debug_assert!(index < self.length());
    let ptr = (self.address() + ByteArray::DATA_OFFSET + index as usize) as *const u8;
    // SAFETY: 调用方保证指针有效且 index < length
    unsafe { *ptr }
  }

  #[inline]
  unsafe fn set_byte(&self, index: u32, value: u8) {
    debug_assert!(index < self.length());
    let ptr = (self.address() + ByteArray::DATA_OFFSET + index as usize) as *mut u8;
    // SAFETY: 调用方保证指针有效且 index < length，无并发别名写冲突
    unsafe { *ptr = value };
  }

  #[inline]
  unsafe fn get_u32(&self, offset: usize) -> u32 {
    debug_assert!(offset + 4 <= self.length() as usize);
    let ptr = (self.address() + ByteArray::DATA_OFFSET + offset) as *const u32;
    // SAFETY: 调用方保证指针有效且 offset + 4 <= length; 使用 read_unaligned 允许非对齐内存读取
    unsafe { ptr::read_unaligned(ptr) }
  }

  #[inline]
  unsafe fn set_u32(&self, offset: usize, value: u32) {
    debug_assert!(offset + 4 <= self.length() as usize);
    let ptr = (self.address() + ByteArray::DATA_OFFSET + offset) as *mut u32;
    // SAFETY: 调用方保证指针有效且 offset + 4 <= length，无并发别名写冲突; 使用 write_unaligned 允许非对齐内存写入
    unsafe { ptr::write_unaligned(ptr, value) };
  }

  #[inline]
  unsafe fn get_u16(&self, offset: usize) -> u16 {
    debug_assert!(offset + 2 <= self.length() as usize);
    let ptr = (self.address() + ByteArray::DATA_OFFSET + offset) as *const u16;
    // SAFETY: 调用方保证指针有效且 offset + 2 <= length; 使用 read_unaligned 允许非对齐内存读取
    unsafe { ptr::read_unaligned(ptr) }
  }

  #[inline]
  unsafe fn set_u16(&self, offset: usize, value: u16) {
    debug_assert!(offset + 2 <= self.length() as usize);
    let ptr = (self.address() + ByteArray::DATA_OFFSET + offset) as *mut u16;
    // SAFETY: 调用方保证指针有效且 offset + 2 <= length，无并发别名写冲突; 使用 write_unaligned 允许非对齐内存写入
    unsafe { ptr::write_unaligned(ptr, value) };
  }

  #[inline]
  unsafe fn get_u64(&self, offset: usize) -> u64 {
    debug_assert!(offset + 8 <= self.length() as usize);
    let ptr = (self.address() + ByteArray::DATA_OFFSET + offset) as *const u64;
    // SAFETY: 调用方保证指针有效且 offset + 8 <= length; 使用 read_unaligned 允许非对齐内存读取
    unsafe { ptr::read_unaligned(ptr) }
  }

  #[inline]
  unsafe fn set_u64(&self, offset: usize, value: u64) {
    debug_assert!(offset + 8 <= self.length() as usize);
    let ptr = (self.address() + ByteArray::DATA_OFFSET + offset) as *mut u64;
    // SAFETY: 调用方保证指针有效且 offset + 8 <= length，无并发别名写冲突; 使用 write_unaligned 允许非对齐内存写入
    unsafe { ptr::write_unaligned(ptr, value) };
  }

  fn get_byte_checked(&self, index: usize) -> Result<u8> {
    let len = self.length() as usize;
    if index >= len {
      return Err(Error::OutOfBounds { index, length: len });
    }
    // SAFETY: index 已经过 < len 范围检查
    unsafe { Ok(self.get_byte(index as u32)) }
  }

  fn set_byte_checked(&self, index: usize, value: u8) -> Result<()> {
    let len = self.length() as usize;
    if index >= len {
      return Err(Error::OutOfBounds { index, length: len });
    }
    // SAFETY: index 已经过 < len 范围检查
    unsafe { self.set_byte(index as u32, value) };
    Ok(())
  }

  #[inline]
  unsafe fn copy_bytes(&self, dst_offset: usize, src_offset: usize, len: usize) {
    if len == 0 {
      return;
    }
    let total_len = self.length() as usize;
    debug_assert!(
      dst_offset
        .checked_add(len)
        .is_some_and(|end| end <= total_len)
    );
    debug_assert!(
      src_offset
        .checked_add(len)
        .is_some_and(|end| end <= total_len)
    );
    let base = self.address() + ByteArray::DATA_OFFSET;
    let src = (base + src_offset) as *const u8;
    let dst = (base + dst_offset) as *mut u8;
    // SAFETY: 调用方保证 dst/src 加上 len 不超出数组边界; ptr::copy 允许重叠内存区
    unsafe {
      ptr::copy(src, dst, len);
    }
  }

  #[inline]
  unsafe fn raw_bytes(&self) -> &[u8] {
    let len = self.length() as usize;
    let ptr = (self.address() + ByteArray::DATA_OFFSET) as *const u8;
    // SAFETY: 调用方保证宿主指针有效且生命周期合法
    unsafe { from_raw_parts(ptr, len) }
  }

  #[inline]
  unsafe fn raw_bytes_mut(&mut self) -> &mut [u8] {
    let len = self.length() as usize;
    let ptr = (self.address() + ByteArray::DATA_OFFSET) as *mut u8;
    // SAFETY: 调用方保证宿主指针有效且无别名并发写入
    unsafe { from_raw_parts_mut(ptr, len) }
  }
}

// ============================================================================
// §3 TrustedByteArray
// ============================================================================

/// TrustedByteArray: 受信任空间内的字节数组 (继承 TrustedFixedArrayBase)
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TrustedByteArray {
  /// 继承 TrustedFixedArrayBase
  pub trusted_base: TrustedFixedArrayBase,
}

impl TrustedByteArray {
  /// 数据区偏移 (16 字节)
  pub const DATA_OFFSET: usize = mem::size_of::<TrustedFixedArrayBase>();
  /// 元素字节宽度
  pub const ELEMENT_SIZE: usize = 1;
  /// 最大容量限制
  pub const MAX_CAPACITY: u32 = K_MAX_FIXED_ARRAY_CAPACITY;

  /// 构造 TrustedByteArray 头部
  #[inline]
  pub const fn new(map: Tagged<Map>, length: u32) -> Self {
    Self {
      trusted_base: TrustedFixedArrayBase::new(map, length),
    }
  }

  /// 依据对象全尺寸换算有效负载长度
  #[inline]
  #[must_use]
  pub const fn length_for(size_in_bytes: usize) -> usize {
    size_in_bytes.saturating_sub(Self::DATA_OFFSET)
  }

  /// 依据元素长度计算对象总字节大小
  #[inline]
  #[must_use]
  pub const fn size_for(length: u32) -> usize {
    object_align(Self::DATA_OFFSET + length as usize)
  }

  /// 依据容量计算最大常规堆对象允许长度
  #[inline]
  #[must_use]
  pub const fn max_regular_length() -> usize {
    (K_MAX_REGULAR_HEAP_OBJECT_SIZE - Self::DATA_OFFSET) / Self::ELEMENT_SIZE
  }
}

impl Cast for TrustedByteArray {
  #[inline]
  fn allow_from(value: Tagged<Object>) -> bool {
    instance_type_of(value).is_some_and(itc::is_trusted_byte_array)
  }
}

/// Tagged<TrustedByteArray> 扩展
pub trait TrustedByteArrayExt: Copy {
  /// 获取长度
  fn length(&self) -> u32;

  /// length(kRelaxedLoad)
  ///
  /// # Safety
  /// 宿主指针有效。
  unsafe fn length_relaxed(&self) -> u32;

  /// length(kAcquireLoad)
  ///
  /// # Safety
  /// 宿主指针有效。
  unsafe fn length_acquire(&self) -> u32;

  /// set_length(value)
  ///
  /// # Safety
  /// 宿主指针有效可写且无并发写。
  unsafe fn set_length(&self, value: u32);

  /// set_length(value, kReleaseStore)
  ///
  /// # Safety
  /// 宿主指针有效可写。
  unsafe fn set_length_release(&self, value: u32);

  /// 依据 acquire 长度计算分配字节大小
  ///
  /// # Safety
  /// 宿主指针有效。
  unsafe fn allocated_size(&self) -> usize;

  /// 读字节
  ///
  /// # Safety
  /// index 必须小于 length()。
  unsafe fn get_byte(&self, index: u32) -> u8;

  /// 写字节
  ///
  /// # Safety
  /// index 必须小于 length()。
  unsafe fn set_byte(&self, index: u32, value: u8);

  /// 读 u16 (小端)
  ///
  /// # Safety
  /// offset + 2 必须小于等于 length()。
  unsafe fn get_u16(&self, offset: usize) -> u16;

  /// 写 u16 (小端)
  ///
  /// # Safety
  /// offset + 2 必须小于等于 length()。
  unsafe fn set_u16(&self, offset: usize, value: u16);

  /// 读 u32 (小端)
  ///
  /// # Safety
  /// offset + 4 必须小于等于 length()。
  unsafe fn get_u32(&self, offset: usize) -> u32;

  /// 写 u32 (小端)
  ///
  /// # Safety
  /// offset + 4 必须小于等于 length()。
  unsafe fn set_u32(&self, offset: usize, value: u32);

  /// 读 u64 (小端)
  ///
  /// # Safety
  /// offset + 8 必须小于等于 length()。
  unsafe fn get_u64(&self, offset: usize) -> u64;

  /// 写 u64 (小端)
  ///
  /// # Safety
  /// offset + 8 必须小于等于 length()。
  unsafe fn set_u64(&self, offset: usize, value: u64);

  /// get_int (以 offset 读 32 位整数)
  ///
  /// # Safety
  /// offset + 4 必须小于等于 length()。
  unsafe fn get_int(&self, offset: usize) -> u32 {
    // SAFETY: 调用方满足 offset + 4 <= length 且对象指针有效
    unsafe { self.get_u32(offset) }
  }

  /// set_int (以 offset 写 32 位整数)
  ///
  /// # Safety
  /// offset + 4 必须小于等于 length()。
  unsafe fn set_int(&self, offset: usize, value: u32) {
    // SAFETY: 调用方满足 offset + 4 <= length 且对象指针有效
    unsafe { self.set_u32(offset, value) }
  }

  /// 边界检查的安全读字节
  fn get_byte_checked(&self, index: usize) -> Result<u8>;

  /// 边界检查的安全写字节
  fn set_byte_checked(&self, index: usize, value: u8) -> Result<()>;

  /// 切片复制
  ///
  /// # Safety
  /// dst_offset + len 与 src_offset + len 必须不越界。
  unsafe fn copy_bytes(&self, dst_offset: usize, src_offset: usize, len: usize);

  /// 获取底层字节切片只读视图
  ///
  /// # Safety
  /// 数组指针有效且切片存活期内无并发写入。
  unsafe fn raw_bytes(&self) -> &[u8];

  /// 获取底层字节切片可变视图
  ///
  /// # Safety
  /// 数组指针有效可写且无并发/别名写入。
  unsafe fn raw_bytes_mut(&mut self) -> &mut [u8];
}

impl TrustedByteArrayExt for Tagged<TrustedByteArray> {
  #[inline]
  fn length(&self) -> u32 {
    let addr = self.address() + mem::offset_of!(TrustedFixedArrayBase, length);
    // SAFETY: 对象头在已分配范围内
    unsafe { u32_slot::load(addr) }
  }

  #[inline]
  unsafe fn length_relaxed(&self) -> u32 {
    let addr = self.address() + mem::offset_of!(TrustedFixedArrayBase, length);
    // SAFETY: 调用方保证指针有效
    unsafe { u32_slot::load_order(addr, Ordering::Relaxed) }
  }

  #[inline]
  unsafe fn length_acquire(&self) -> u32 {
    let addr = self.address() + mem::offset_of!(TrustedFixedArrayBase, length);
    // SAFETY: 调用方保证指针有效
    unsafe { u32_slot::load_order(addr, Ordering::Acquire) }
  }

  #[inline]
  unsafe fn set_length(&self, value: u32) {
    let addr = self.address() + mem::offset_of!(TrustedFixedArrayBase, length);
    // SAFETY: 调用方保证有效且无别名
    unsafe { u32_slot::store(addr, value) };
  }

  #[inline]
  unsafe fn set_length_release(&self, value: u32) {
    let addr = self.address() + mem::offset_of!(TrustedFixedArrayBase, length);
    // SAFETY: 调用方保证有效
    unsafe { u32_slot::store_order(addr, value, Ordering::Release) };
  }

  #[inline]
  unsafe fn allocated_size(&self) -> usize {
    // SAFETY: 调用方保证指针有效，读取 acquire 语义 length 计算对象尺寸
    let len = unsafe { self.length_acquire() };
    TrustedByteArray::size_for(len)
  }

  #[inline]
  unsafe fn get_byte(&self, index: u32) -> u8 {
    debug_assert!(index < self.length());
    let ptr = (self.address() + TrustedByteArray::DATA_OFFSET + index as usize) as *const u8;
    // SAFETY: 调用方保证指针有效且 index < length
    unsafe { *ptr }
  }

  #[inline]
  unsafe fn set_byte(&self, index: u32, value: u8) {
    debug_assert!(index < self.length());
    let ptr = (self.address() + TrustedByteArray::DATA_OFFSET + index as usize) as *mut u8;
    // SAFETY: 调用方保证指针有效且 index < length，无并发别名写冲突
    unsafe { *ptr = value };
  }

  #[inline]
  unsafe fn get_u16(&self, offset: usize) -> u16 {
    debug_assert!(offset + 2 <= self.length() as usize);
    let ptr = (self.address() + TrustedByteArray::DATA_OFFSET + offset) as *const u16;
    // SAFETY: 调用方保证指针有效且 offset + 2 <= length; 使用 read_unaligned 允许非对齐内存读取
    unsafe { ptr::read_unaligned(ptr) }
  }

  #[inline]
  unsafe fn set_u16(&self, offset: usize, value: u16) {
    debug_assert!(offset + 2 <= self.length() as usize);
    let ptr = (self.address() + TrustedByteArray::DATA_OFFSET + offset) as *mut u16;
    // SAFETY: 调用方保证指针有效且 offset + 2 <= length，无并发别名写冲突; 使用 write_unaligned 允许非对齐内存写入
    unsafe { ptr::write_unaligned(ptr, value) };
  }

  #[inline]
  unsafe fn get_u32(&self, offset: usize) -> u32 {
    debug_assert!(offset + 4 <= self.length() as usize);
    let ptr = (self.address() + TrustedByteArray::DATA_OFFSET + offset) as *const u32;
    // SAFETY: 调用方保证指针有效且 offset + 4 <= length; 使用 read_unaligned 允许非对齐内存读取
    unsafe { ptr::read_unaligned(ptr) }
  }

  #[inline]
  unsafe fn set_u32(&self, offset: usize, value: u32) {
    debug_assert!(offset + 4 <= self.length() as usize);
    let ptr = (self.address() + TrustedByteArray::DATA_OFFSET + offset) as *mut u32;
    // SAFETY: 调用方保证指针有效且 offset + 4 <= length，无并发别名写冲突; 使用 write_unaligned 允许非对齐内存写入
    unsafe { ptr::write_unaligned(ptr, value) };
  }

  #[inline]
  unsafe fn get_u64(&self, offset: usize) -> u64 {
    debug_assert!(offset + 8 <= self.length() as usize);
    let ptr = (self.address() + TrustedByteArray::DATA_OFFSET + offset) as *const u64;
    // SAFETY: 调用方保证指针有效且 offset + 8 <= length; 使用 read_unaligned 允许非对齐内存读取
    unsafe { ptr::read_unaligned(ptr) }
  }

  #[inline]
  unsafe fn set_u64(&self, offset: usize, value: u64) {
    debug_assert!(offset + 8 <= self.length() as usize);
    let ptr = (self.address() + TrustedByteArray::DATA_OFFSET + offset) as *mut u64;
    // SAFETY: 调用方保证指针有效且 offset + 8 <= length，无并发别名写冲突; 使用 write_unaligned 允许非对齐内存写入
    unsafe { ptr::write_unaligned(ptr, value) };
  }

  fn get_byte_checked(&self, index: usize) -> Result<u8> {
    let len = self.length() as usize;
    if index >= len {
      return Err(Error::OutOfBounds { index, length: len });
    }
    // SAFETY: index 已经过 < len 范围检查
    unsafe { Ok(self.get_byte(index as u32)) }
  }

  fn set_byte_checked(&self, index: usize, value: u8) -> Result<()> {
    let len = self.length() as usize;
    if index >= len {
      return Err(Error::OutOfBounds { index, length: len });
    }
    // SAFETY: index 已经过 < len 范围检查
    unsafe { self.set_byte(index as u32, value) };
    Ok(())
  }

  #[inline]
  unsafe fn copy_bytes(&self, dst_offset: usize, src_offset: usize, len: usize) {
    if len == 0 {
      return;
    }
    let total_len = self.length() as usize;
    debug_assert!(
      dst_offset
        .checked_add(len)
        .is_some_and(|end| end <= total_len)
    );
    debug_assert!(
      src_offset
        .checked_add(len)
        .is_some_and(|end| end <= total_len)
    );
    let base = self.address() + TrustedByteArray::DATA_OFFSET;
    let src = (base + src_offset) as *const u8;
    let dst = (base + dst_offset) as *mut u8;
    // SAFETY: 调用方保证 dst/src 加上 len 不超出数组边界; ptr::copy 允许重叠内存区
    unsafe {
      ptr::copy(src, dst, len);
    }
  }

  #[inline]
  unsafe fn raw_bytes(&self) -> &[u8] {
    let len = self.length() as usize;
    let ptr = (self.address() + TrustedByteArray::DATA_OFFSET) as *const u8;
    // SAFETY: 调用方保证宿主指针有效且生命周期合法
    unsafe { from_raw_parts(ptr, len) }
  }

  #[inline]
  unsafe fn raw_bytes_mut(&mut self) -> &mut [u8] {
    let len = self.length() as usize;
    let ptr = (self.address() + TrustedByteArray::DATA_OFFSET) as *mut u8;
    // SAFETY: 调用方保证宿主指针有效且无别名并发写入
    unsafe { from_raw_parts_mut(ptr, len) }
  }
}

// ============================================================================
// §4 FixedIntegerArrayBase (定宽整数视图)
// ============================================================================

/// FixedIntegerArrayBase: ByteArray / TrustedByteArray 上的定宽整数适配器
pub struct FixedIntegerArrayBase<T, Base> {
  _marker: PhantomData<(T, Base)>,
}

pub type FixedInt8Array = FixedIntegerArrayBase<i8, ByteArray>;
pub type FixedUInt8Array = FixedIntegerArrayBase<u8, ByteArray>;
pub type FixedInt16Array = FixedIntegerArrayBase<i16, ByteArray>;
pub type FixedUInt16Array = FixedIntegerArrayBase<u16, ByteArray>;
pub type FixedInt32Array = FixedIntegerArrayBase<i32, ByteArray>;
pub type FixedUInt32Array = FixedIntegerArrayBase<u32, ByteArray>;
pub type FixedInt64Array = FixedIntegerArrayBase<i64, ByteArray>;
pub type FixedUInt64Array = FixedIntegerArrayBase<u64, ByteArray>;
pub type TrustedFixedAddressArray = FixedIntegerArrayBase<Address, TrustedByteArray>;
