use std::{
  cmp::Ordering,
  fmt,
  hash::{Hash, Hasher},
  marker::PhantomData,
  mem::size_of,
  ptr::{from_ref, write},
};

pub use v8_base::memory::Address;

use crate::{
  error::Result,
  heap_object::HeapObject,
  smi::Smi,
  tagged_constants::{
    HEAP_OBJECT_TAG, HEAP_OBJECT_TAG_MASK, SMI_TAG, SMI_TAG_MASK, SMI_TAG_SIZE,
    WEAK_HEAP_OBJECT_TAG,
  },
};

/// 顶层通用 Object 标记类型，用于表达任意 V8 对象引用 (`Tagged<Object>`)
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Object;

/// 强类型 Tagged 指针包装类型
///
/// 在 V8 内存模型中，所有堆对象引用与小整数均通过指针的最低位 Tag 进行区分：
/// - 最低位为 `0`：小整数 (Smi)
/// - 最低两位为 `01`：强引用的堆对象指针 (HeapObject)
/// - 最低两位为 `11`：弱引用的堆对象指针 (Weak HeapObject)
#[repr(transparent)]
pub struct Tagged<T> {
  ptr: Address,
  _phantom: PhantomData<*const T>,
}

impl<T> Tagged<T> {
  /// 空指针常量
  pub const NULL: Self = Self::new(0);

  /// 以裸指针整数值构造 Tagged 指针
  #[inline]
  pub const fn new(ptr: usize) -> Self {
    Self {
      ptr,
      _phantom: PhantomData,
    }
  }

  /// 获取存储的原始指针数值 (包含 tag 位)
  #[inline]
  pub const fn ptr(&self) -> usize {
    self.ptr
  }

  /// 判断指针是否为空 (NULL 地址)
  #[inline]
  pub const fn is_null(&self) -> bool {
    self.ptr == 0
  }

  /// 判断当前 Tagged 指针是否编码为小整数 (Smi)
  #[inline]
  pub const fn is_smi(&self) -> bool {
    (self.ptr & SMI_TAG_MASK) == SMI_TAG
  }

  /// 判断当前 Tagged 指针是否为强引用的堆对象 (HeapObject)
  #[inline]
  pub const fn is_heap_object(&self) -> bool {
    (self.ptr & HEAP_OBJECT_TAG_MASK) == HEAP_OBJECT_TAG
  }

  /// 判断当前 Tagged 指针是否为弱引用堆对象 (Weak HeapObject)
  #[inline]
  pub const fn is_weak(&self) -> bool {
    (self.ptr & HEAP_OBJECT_TAG_MASK) == WEAK_HEAP_OBJECT_TAG
  }

  /// 如果是 Smi 则安全转换为 `Tagged<Smi>`
  #[inline]
  pub fn to_smi(&self) -> Option<Tagged<Smi>> {
    if self.is_smi() {
      Some(Tagged::new(self.ptr))
    } else {
      None
    }
  }

  /// 如果是强引用堆对象则安全转换为 `Tagged<HeapObject>`
  #[inline]
  pub fn to_heap_object(&self) -> Option<Tagged<HeapObject>> {
    if self.is_heap_object() {
      Some(Tagged::new(self.ptr))
    } else {
      None
    }
  }

  /// 零开销强转目标类型标记
  #[inline]
  pub const fn cast<U>(self) -> Tagged<U> {
    Tagged::new(self.ptr)
  }

  /// 跨 cage 安全对象相等比较 (原版 TaggedImpl::SafeEquals)
  #[inline]
  pub const fn safe_equals<U>(&self, other: Tagged<U>) -> bool {
    self.ptr == other.ptr
  }

  /// 获取底层 `HeapObject` 的裸内存对齐地址。
  ///
  /// # Safety 显式证明
  /// V8 的 `HeapObject` 强引用指针最低两位为 `0b01` (tag=1)，弱引用指针最低两位为 `0b11` (tag=3)。
  /// 在已知 `is_heap_object()` 或 `is_weak()` 为真时，清空最低两位 Tag 掩码
  /// (`self.ptr & !HEAP_OBJECT_TAG_MASK`) 严格还原该对象在内存堆中的真实起始对齐地址
  ///（至少 4 字节/8 字节对齐）。
  #[inline]
  pub fn address(&self) -> usize {
    debug_assert!(
      self.is_heap_object() || self.is_weak(),
      "address() 只能在堆对象指针上调用"
    );
    self.ptr & !HEAP_OBJECT_TAG_MASK
  }

  /// 从堆对象已对齐的真实内存地址构造 `Tagged<T>`
  ///
  /// # Safety
  /// 调用者必须确保 `address` 是对齐的有效堆内存首地址（最低 2 位全为 0）。
  #[inline]
  pub const unsafe fn from_raw_address(address: usize) -> Self {
    Self::new(address | HEAP_OBJECT_TAG)
  }

  /// 从裸指针构造 `Tagged<T>`
  #[inline]
  pub fn from_ptr(ptr: *const T) -> Self {
    Self::new((ptr as usize) | HEAP_OBJECT_TAG)
  }

  /// 在 Zone 区域中分配对象并返回 `Tagged<T>` 指针
  pub fn allocate_in_zone(zone: &v8_zone::Zone, value: T) -> Result<Self> {
    let size = size_of::<T>();
    let slice = zone.allocate_bytes_ref(size)?;
    let ptr = slice.as_mut_ptr() as *mut T;
    // SAFETY: slice 长度为 size_of::<T>(), 且由 Zone 分配确保空间充足与基础对齐
    unsafe {
      write(ptr, value);
    }
    Ok(Self::from_ptr(ptr))
  }

  /// 从引用构造 `Tagged<T>`
  #[inline]
  pub fn from_ref(val: &T) -> Self {
    let raw = from_ref(val) as usize;
    Self::new(raw | HEAP_OBJECT_TAG)
  }

  /// 获取指向 `T` 的裸只读指针
  ///
  /// # Safety
  /// 调用者必须确保当前指针指向已初始化的有效 `T` 实例堆内存。
  #[inline]
  pub unsafe fn as_ptr(&self) -> *const T {
    self.address() as *const T
  }

  /// 获取指向 `T` 的裸可变指针
  ///
  /// # Safety
  /// 调用者必须确保当前指针指向已初始化的有效 `T` 实例堆内存，且无并发冲突。
  #[inline]
  pub unsafe fn as_mut_ptr(&self) -> *mut T {
    self.address() as *mut T
  }

  /// 转换为 Rust 不可变引用
  ///
  /// # Safety
  /// 调用者必须确保指针有效、对齐、符合生命周期且无未初始化的脏数据。
  #[inline]
  pub unsafe fn as_ref<'a>(&self) -> &'a T {
    // SAFETY: 调用者必须确保指针有效、对齐、符合生命周期且无未初始化的脏数据
    unsafe { &*self.as_ptr() }
  }

  /// 转换为 Rust 可变引用
  ///
  /// # Safety
  /// 调用者必须确保指针有效、独占访问且无别名冲突。
  #[inline]
  pub unsafe fn as_mut<'a>(&mut self) -> &'a mut T {
    // SAFETY: 调用者必须确保指针有效、独占访问且无别名冲突
    unsafe { &mut *self.as_mut_ptr() }
  }
}

impl Tagged<Smi> {
  /// 解包 Smi 数值
  #[inline]
  pub const fn value(&self) -> i32 {
    Smi::value(*self)
  }
}

impl<T> Copy for Tagged<T> {}

impl<T> Clone for Tagged<T> {
  #[inline]
  fn clone(&self) -> Self {
    *self
  }
}

impl<T> PartialEq for Tagged<T> {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.ptr == other.ptr
  }
}

impl<T> Eq for Tagged<T> {}

impl<T> PartialOrd for Tagged<T> {
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl<T> Ord for Tagged<T> {
  #[inline]
  fn cmp(&self, other: &Self) -> Ordering {
    self.ptr.cmp(&other.ptr)
  }
}

impl<T> Hash for Tagged<T> {
  #[inline]
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.ptr.hash(state);
  }
}

impl<T> Default for Tagged<T> {
  #[inline]
  fn default() -> Self {
    Self::NULL
  }
}

impl<T> fmt::Debug for Tagged<T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Tagged(0x{:x})", self.ptr)
  }
}

impl<T> fmt::Display for Tagged<T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "0x{:x}", self.ptr)
  }
}

// ============================================================================
// 弱引用谓词族 / 取出族 / MakeWeak 系列 / TaggedIndex
// 来源: v8/src/objects/tagged-impl.h + tagged-impl-inl.h + tagged-index.h
// (由 tool/src/objects/tagged-impl.h.js 生成差集骨架后就地修订)
// ============================================================================

/// 已清除弱引用值的低 32 位 (0b11)
pub const CLEARED_WEAK_HEAP_OBJECT_LOWER32: u32 = 3;

/// 已清除弱引用的完整哨兵值
///
/// 判定按原版只比较低 32 位; 完整值即低 32 位常量本身 (高 32 位为 0)。
pub const CLEARED_WEAK_VALUE: usize = CLEARED_WEAK_HEAP_OBJECT_LOWER32 as usize;

/// 弱引用位掩码 (bit 1, 原版 kWeakHeapObjectMask)
const WEAK_HEAP_OBJECT_MASK: usize = 1 << 1;

/// MaybeObject 标记类型: Smi / 强堆对象 / 弱堆对象 / 已清除弱引用 四态联合
///
/// 对应 C++ `MaybeObject = Union<Smi, HeapObject, Weak<HeapObject>>`,
/// 其 `Tagged<MaybeObject>` 走 WeakTaggedBase (kCanBeWeak = true),
/// 与泛型 `Tagged<T>` (StrongTaggedBase) 的谓词语义差异见各方法注释。
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct MaybeObject;

/// 堆对象引用标记类型 (原版 tagged.h `HeapObjectReference`)
///
/// 值层与 `HeapObject` 同一 (堆对象指针), 类型层表达"引用"语义:
/// `Tagged<HeapObjectReference>` 可出现在带弱标记位的槽位
/// (HeapObjectSlot), 读出后用 `HAS_STRONG_HEAP_OBJECT_TAG` 判定强引用。
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct HeapObjectReference;

/// 弱引用哨兵类型标记 (C++ `Weak<T>`, 仅类型层标记, 无运行时数据)
pub struct Weak<T> {
  _ty: PhantomData<*const T>,
}

/// 已清除弱引用哨兵类型 (C++ `ClearedWeakValue`)
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ClearedWeakValue;

impl Tagged<MaybeObject> {
  /// 构造已清除弱引用值
  #[inline]
  pub const fn cleared() -> Self {
    Tagged::new(CLEARED_WEAK_VALUE)
  }

  /// 是否为已清除弱引用 (低 32 位 == 0b11)
  #[inline]
  pub const fn is_cleared(&self) -> bool {
    self.ptr() as u32 == CLEARED_WEAK_HEAP_OBJECT_LOWER32
  }

  /// 是否为弱引用或已清除弱引用 (低 2 位 == 0b11)
  ///
  /// 与泛型 [`Tagged::is_weak`] 同为位级判定。
  #[inline]
  pub const fn is_weak_or_cleared(&self) -> bool {
    (self.ptr() & HEAP_OBJECT_TAG_MASK) == WEAK_HEAP_OBJECT_TAG
  }

  /// 是否为"活"弱引用: 弱指针且未清除 (对应 C++ MaybeObject 的 IsWeak)
  #[inline]
  pub const fn is_weak_live(&self) -> bool {
    self.is_weak_or_cleared() && !self.is_cleared()
  }

  /// 是否为强引用堆对象 (低 2 位 == 0b01)
  #[inline]
  pub const fn is_strong(&self) -> bool {
    (self.ptr() & HEAP_OBJECT_TAG_MASK) == HEAP_OBJECT_TAG
  }

  /// 是否为强引用堆对象或 Smi (低 2 位 != 0b11)
  #[inline]
  pub const fn is_strong_or_smi(&self) -> bool {
    (self.ptr() & HEAP_OBJECT_TAG_MASK) != WEAK_HEAP_OBJECT_TAG
  }

  /// 是否为强/弱堆对象 (非 Smi 且未清除)
  #[inline]
  pub const fn is_strong_or_weak(&self) -> bool {
    !self.is_smi() && !self.is_cleared()
  }

  /// 是否为强指针或 Smi (对应 MaybeObject 基类的 IsObject)
  #[inline]
  pub const fn is_object(&self) -> bool {
    !self.is_weak_or_cleared()
  }

  /// 强引用时还原堆对象指针
  #[inline]
  pub const fn get_heap_object_if_strong(&self) -> Option<Tagged<HeapObject>> {
    if self.is_strong() {
      Some(Tagged::new(self.ptr()))
    } else {
      None
    }
  }

  /// 活弱引用时还原堆对象指针 (清除弱 tag 位)
  #[inline]
  pub const fn get_heap_object_if_weak(&self) -> Option<Tagged<HeapObject>> {
    if self.is_weak_live() {
      Some(Tagged::new(self.ptr() & !WEAK_HEAP_OBJECT_MASK))
    } else {
      None
    }
  }

  /// 强/弱堆对象时还原堆对象指针, 否则 None
  #[inline]
  pub const fn get_heap_object(&self) -> Option<Tagged<HeapObject>> {
    if self.is_strong_or_weak() {
      Some(Tagged::new(self.ptr() & !WEAK_HEAP_OBJECT_MASK))
    } else {
      None
    }
  }

  /// Smi 时原样返回, 否则还原堆对象 (对应 GetHeapObjectOrSmi)
  #[inline]
  pub const fn get_heap_object_or_smi(&self) -> Tagged<Object> {
    if self.is_smi() {
      Tagged::new(self.ptr())
    } else {
      Tagged::new(self.ptr() & !WEAK_HEAP_OBJECT_MASK)
    }
  }

  /// Smi 时转换为 `Tagged<Smi>` (对应 MaybeObject 基类的 ToSmi)
  #[inline]
  pub const fn as_smi(&self) -> Option<Tagged<Smi>> {
    if self.is_smi() {
      Some(Tagged::new(self.ptr()))
    } else {
      None
    }
  }
}

/// 强引用 → 确定弱引用 (tagged.h: MakeWeak)
///
/// 禁止对 Smi 调用 (位或会破坏 Smi 负载位, C++ 以 static_assert 拦截)。
#[inline]
pub const fn make_weak<T>(value: Tagged<T>) -> Tagged<Weak<T>> {
  debug_assert!(value.is_heap_object(), "Smi 不允许弱化");
  Tagged::new(value.ptr() | WEAK_HEAP_OBJECT_TAG)
}

/// Smi 或堆对象 → 弱感知引用 (tagged.h: MakeWeakOrSmi)
#[inline]
pub const fn make_weak_or_smi(value: Tagged<Object>) -> Tagged<MaybeObject> {
  if value.is_smi() {
    Tagged::new(value.ptr())
  } else {
    Tagged::new(value.ptr() | WEAK_HEAP_OBJECT_TAG)
  }
}

/// (可能) 弱引用 → 强引用 (tagged.h: MakeStrong)
///
/// 清弱 tag 位但保留堆对象 tag 位 (`ptr & (!kWeakHeapObjectTag | kHeapObjectTag)`),
/// 对 Smi 调用会破坏 31 位 Smi 的负载位 1, C++ 以 static_assert 拦截。
#[inline]
pub const fn make_strong<T>(value: Tagged<Weak<T>>) -> Tagged<T> {
  Tagged::new(value.ptr() & (!WEAK_HEAP_OBJECT_TAG | HEAP_OBJECT_TAG))
}

/// TaggedIndex: 31 位有符号整数索引, 恒符号扩展 (tagged-index.h)
///
/// 与 Smi 的差异: 负载恒 31 位且恒符号扩展, 与指针压缩配置无关;
/// 可安全用作运行时/内置函数栈上的偏移量下标。
pub const TAGGED_INDEX_VALUE_SIZE: usize = 31;

/// TaggedIndex 最小值 (-2^30)
pub const TAGGED_INDEX_MIN: i64 = -(1 << (TAGGED_INDEX_VALUE_SIZE - 1));

/// TaggedIndex 最大值 (2^30 - 1)
pub const TAGGED_INDEX_MAX: i64 = -TAGGED_INDEX_MIN - 1;

/// TaggedIndex 标记类型 (栈上下索引用, 不走堆分配)
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TaggedIndex;

impl TaggedIndex {
  /// 数值 → TaggedIndex (值域超界走 debug 断言)
  #[inline]
  pub const fn from_intptr(value: isize) -> Tagged<Self> {
    debug_assert!(Self::is_valid(value), "TaggedIndex 值域超界");
    // 原版: (value << kSmiTagSize) | kSmiTag, 符号位随移位自然扩展
    Tagged::new(((value as usize) << SMI_TAG_SIZE) | SMI_TAG)
  }

  /// 值域检查 ([-2^30, 2^30 - 1])
  #[inline]
  pub const fn is_valid(value: isize) -> bool {
    value >= TAGGED_INDEX_MIN as isize && value <= TAGGED_INDEX_MAX as isize
  }

  /// 解包数值 (算术右移恢复符号)
  #[inline]
  pub const fn value(tagged: Tagged<Self>) -> isize {
    (tagged.ptr() as isize) >> SMI_TAG_SIZE
  }
}
