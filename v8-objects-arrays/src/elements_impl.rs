// 由 tool/src/objects/elements.cc.js 从 v8/src/objects/elements.cc 生成; 修订就地改, 复跑覆盖
// elements.cc 签名骨架与语义实现; 对数 clang -E (87 顶层函数 / 35 类)
//! elements.cc: 顶层转换/复制函数与 TypedElementsAccessor 特化清单。
//!
//! 语义实现面：
//! - [`is_float16_raw_bits_zero`]: float16 符号位掩码零判定；
//! - [`is_identity_mapping`] / [`is_identity_mapping_slice`]: 连续索引单次单向零分配恒等映射判定；
//! - [`get_write_barrier_mode`]: Smi/Double 元素的写屏障跳过决策；
//! - [`sort_indices`] / [`sort_indices_slice`]: 原位递增排序 (std::sort 对齐，Smi/HeapNumber 数值升序，Undefined 沉底)；
//! - 各类元素拷贝函数 ([`copy_object_to_object_elements`], [`copy_double_to_double_elements`],
//!   [`copy_smi_to_double_elements`], [`copy_packed_smi_to_double_elements`],
//!   [`copy_object_to_double_elements`], [`copy_from_float16_to_f16`],
//!   [`copy_from_float16_to_f32`], [`copy_from_float16_to_f64`])。
//!
//! 深水项与推迟登记项参见 `readme/zh.md` 推迟台账 (ELEM-ISO, ELEM-CSA, ELEM-DICT, ELEM-SLOW, ELEM-STATIC)。

use std::{
  cmp::Ordering,
  ptr,
  slice::{from_raw_parts, from_raw_parts_mut},
};

use v8_base::float16::Float16;
use v8_objects_core::{
  heap_number::HeapNumber,
  instance_type_checker as itc,
  objects::{WriteBarrierMode, instance_type_of},
  smi::Smi,
  tagged::{Object, Tagged},
};

use crate::{
  elements_kind::ElementsKind,
  fixed_array::{FixedArray, TaggedArrayExt},
  primitive_array::{FixedDoubleArray, FixedDoubleArrayExt},
};

/// v8: src/objects/elements.cc 顶层自由函数 (elements.h 侧已建骨架的 4 个除外)。
pub const IMPL_ONLY_FREE_FUNCTIONS: [&str; 16] = [
  "ThrowArrayLengthRangeError",
  "GetWriteBarrierMode",
  "CopyObjectToObjectElements",
  "CopyDictionaryToObjectElements",
  "CopyDoubleToObjectElements",
  "CopyDoubleToDoubleElements",
  "CopySmiToDoubleElements",
  "CopyPackedSmiToDoubleElements",
  "CopyObjectToDoubleElements",
  "CopyDictionaryToDoubleElements",
  "SortIndices",
  "IncludesValueSlowPath",
  "IndexOfValueSlowPath",
  "IsFloat16RawBitsZero",
  "CopyFromFloat16BackingStore",
  "IsIdentityMapping",
];

/// v8: src/objects/elements.cc ElementsAccessor 静态方法 (3)。
pub const STATIC_METHODS: [&str; 3] = [
  "ElementsAccessor::InitializeOncePerProcess",
  "ElementsAccessor::TearDown",
  "ElementsAccessor::Concat",
];

/// v8: src/objects/elements.cc TypedElementsAccessor 特化清单 (kind, method)。
pub const TYPED_SPECIALIZATIONS: [(&str, &str); 64] = [
  ("INT8_ELEMENTS", "ToHandle"),
  ("UINT8_ELEMENTS", "ToHandle"),
  ("INT16_ELEMENTS", "ToHandle"),
  ("UINT16_ELEMENTS", "ToHandle"),
  ("INT32_ELEMENTS", "ToHandle"),
  ("UINT32_ELEMENTS", "ToHandle"),
  ("FLOAT16_ELEMENTS", "FromScalar"),
  ("FLOAT32_ELEMENTS", "FromScalar"),
  ("FLOAT16_ELEMENTS", "FromScalar"),
  ("FLOAT16_ELEMENTS", "FromScalar"),
  ("FLOAT16_ELEMENTS", "ToHandle"),
  ("FLOAT32_ELEMENTS", "ToHandle"),
  ("FLOAT64_ELEMENTS", "FromScalar"),
  ("FLOAT64_ELEMENTS", "ToHandle"),
  ("UINT8_CLAMPED_ELEMENTS", "FromScalar"),
  ("UINT8_CLAMPED_ELEMENTS", "FromScalar"),
  ("UINT8_CLAMPED_ELEMENTS", "FromScalar"),
  ("UINT8_CLAMPED_ELEMENTS", "ToHandle"),
  ("BIGINT64_ELEMENTS", "FromScalar"),
  ("BIGINT64_ELEMENTS", "FromScalar"),
  ("BIGINT64_ELEMENTS", "FromScalar"),
  ("BIGINT64_ELEMENTS", "FromScalar"),
  ("BIGINT64_ELEMENTS", "FromScalar"),
  ("BIGINT64_ELEMENTS", "FromObject"),
  ("BIGINT64_ELEMENTS", "ToHandle"),
  ("BIGUINT64_ELEMENTS", "FromScalar"),
  ("BIGUINT64_ELEMENTS", "FromScalar"),
  ("BIGUINT64_ELEMENTS", "FromScalar"),
  ("BIGUINT64_ELEMENTS", "FromScalar"),
  ("BIGUINT64_ELEMENTS", "FromScalar"),
  ("BIGUINT64_ELEMENTS", "FromObject"),
  ("BIGUINT64_ELEMENTS", "ToHandle"),
  ("RAB_GSAB_INT8_ELEMENTS", "ToHandle"),
  ("RAB_GSAB_UINT8_ELEMENTS", "ToHandle"),
  ("RAB_GSAB_INT16_ELEMENTS", "ToHandle"),
  ("RAB_GSAB_UINT16_ELEMENTS", "ToHandle"),
  ("RAB_GSAB_INT32_ELEMENTS", "ToHandle"),
  ("RAB_GSAB_UINT32_ELEMENTS", "ToHandle"),
  ("RAB_GSAB_FLOAT16_ELEMENTS", "FromScalar"),
  ("RAB_GSAB_FLOAT16_ELEMENTS", "FromScalar"),
  ("RAB_GSAB_FLOAT16_ELEMENTS", "FromScalar"),
  ("RAB_GSAB_FLOAT16_ELEMENTS", "ToHandle"),
  ("RAB_GSAB_FLOAT32_ELEMENTS", "FromScalar"),
  ("RAB_GSAB_FLOAT32_ELEMENTS", "ToHandle"),
  ("RAB_GSAB_FLOAT64_ELEMENTS", "FromScalar"),
  ("RAB_GSAB_FLOAT64_ELEMENTS", "ToHandle"),
  ("RAB_GSAB_UINT8_CLAMPED_ELEMENTS", "FromScalar"),
  ("RAB_GSAB_UINT8_CLAMPED_ELEMENTS", "FromScalar"),
  ("RAB_GSAB_UINT8_CLAMPED_ELEMENTS", "FromScalar"),
  ("RAB_GSAB_UINT8_CLAMPED_ELEMENTS", "ToHandle"),
  ("RAB_GSAB_BIGINT64_ELEMENTS", "FromScalar"),
  ("RAB_GSAB_BIGINT64_ELEMENTS", "FromScalar"),
  ("RAB_GSAB_BIGINT64_ELEMENTS", "FromScalar"),
  ("RAB_GSAB_BIGINT64_ELEMENTS", "FromScalar"),
  ("RAB_GSAB_BIGINT64_ELEMENTS", "FromScalar"),
  ("RAB_GSAB_BIGINT64_ELEMENTS", "FromObject"),
  ("RAB_GSAB_BIGINT64_ELEMENTS", "ToHandle"),
  ("RAB_GSAB_BIGUINT64_ELEMENTS", "FromScalar"),
  ("RAB_GSAB_BIGUINT64_ELEMENTS", "FromScalar"),
  ("RAB_GSAB_BIGUINT64_ELEMENTS", "FromScalar"),
  ("RAB_GSAB_BIGUINT64_ELEMENTS", "FromScalar"),
  ("RAB_GSAB_BIGUINT64_ELEMENTS", "FromScalar"),
  ("RAB_GSAB_BIGUINT64_ELEMENTS", "FromObject"),
  ("RAB_GSAB_BIGUINT64_ELEMENTS", "ToHandle"),
];

/// v8: src/objects/elements.cc 访问器类清单 (类名, 类内方法声明计数; 口径:
/// 预处理后类体内 2 缩进含参声明行, 多行签名计首行)。
pub const ACCESSOR_CLASSES: [(&str, usize); 32] = [
  ("ElementsKindTraits", 0),
  ("TypedArrayCTypeHelper", 43),
  ("InternalElementsAccessor", 2),
  ("ElementsAccessorBase", 240),
  ("DictionaryElementsAccessor", 127),
  ("FastElementsAccessor", 148),
  ("FastSmiOrObjectElementsAccessor", 35),
  ("FastPackedSmiElementsAccessor", 0),
  ("FastHoleySmiElementsAccessor", 0),
  ("FastPackedObjectElementsAccessor", 0),
  ("FastNonextensibleObjectElementsAccessor", 16),
  ("FastPackedNonextensibleObjectElementsAccessor", 0),
  ("FastHoleyNonextensibleObjectElementsAccessor", 0),
  ("FastSealedObjectElementsAccessor", 19),
  ("FastPackedSealedObjectElementsAccessor", 0),
  ("SharedArrayElementsAccessor", 6),
  ("FastHoleySealedObjectElementsAccessor", 0),
  ("FastFrozenObjectElementsAccessor", 12),
  ("FastPackedFrozenObjectElementsAccessor", 0),
  ("FastHoleyFrozenObjectElementsAccessor", 0),
  ("FastHoleyObjectElementsAccessor", 0),
  ("CopyBetweenBackingStoresImpl", 9),
  ("FastDoubleElementsAccessor", 32),
  ("FastPackedDoubleElementsAccessor", 0),
  ("FastHoleyDoubleElementsAccessor", 0),
  ("TypedElementsAccessor", 212),
  ("SloppyArgumentsElementsAccessor", 86),
  ("SlowSloppyArgumentsElementsAccessor", 25),
  ("FastSloppyArgumentsElementsAccessor", 31),
  ("StringWrapperElementsAccessor", 45),
  ("FastStringWrapperElementsAccessor", 2),
  ("SlowStringWrapperElementsAccessor", 24),
];

// ── 语义已落地核心辅助函数 ──────────────────────────────────────────────────

/// v8: src/objects/elements.cc IsFloat16RawBitsZero
///
/// IEEE 754 规定 +0.0 与 -0.0 在比较时相等，但位模式不同 (0x0000 与 0x8000)。
/// 本函数检测去除符号位后是否为 0。
#[inline]
#[must_use]
pub const fn is_float16_raw_bits_zero(x: u16) -> bool {
  (x & !0x8000) == 0
}

/// 索引转换契约 trait (用于 is_identity_mapping)。
pub trait AsIndex {
  /// 转换为 usize 索引。
  fn as_index(&self) -> usize;
}

impl AsIndex for usize {
  #[inline]
  fn as_index(&self) -> usize {
    *self
  }
}

impl AsIndex for u8 {
  #[inline]
  fn as_index(&self) -> usize {
    *self as usize
  }
}

impl AsIndex for u16 {
  #[inline]
  fn as_index(&self) -> usize {
    *self as usize
  }
}

impl AsIndex for u32 {
  #[inline]
  fn as_index(&self) -> usize {
    *self as usize
  }
}

impl AsIndex for ElementsKind {
  #[inline]
  fn as_index(&self) -> usize {
    *self as usize
  }
}

/// v8: src/objects/elements.cc IsIdentityMapping
///
/// 检查映射表从指定索引开始是否为恒等映射 (mapping[i] == i)。
/// 纯迭代器单次遍历，提前终止，零堆分配。
#[inline]
#[must_use]
pub fn is_identity_mapping<T: AsIndex>(mapping: &[T], index: usize) -> bool {
  mapping
    .iter()
    .enumerate()
    .skip(index)
    .all(|(i, val)| val.as_index() == i)
}

/// v8: src/objects/elements.cc IsIdentityMapping (切片安全便利版)
#[inline]
#[must_use]
pub fn is_identity_mapping_slice(mapping: &[usize], index: usize) -> bool {
  is_identity_mapping(mapping, index)
}

/// v8: src/objects/elements.cc GetWriteBarrierMode
///
/// Smi 与 Double 类型元素无需写屏障 (直接使用 SkipWriteBarrier)。
/// 纯 Object 元素写操作需要更新写屏障 (UpdateWriteBarrier)。
#[inline]
#[must_use]
pub const fn get_write_barrier_mode(kind: ElementsKind) -> WriteBarrierMode {
  if kind.is_smi_elements_kind() || kind.is_double_elements_kind() {
    WriteBarrierMode::SkipWriteBarrier
  } else {
    WriteBarrierMode::UpdateWriteBarrier
  }
}

/// 提取元素索引的数值表示 (Smi 或 HeapNumber)。
/// 若为 Undefined / Null / 非数值则返回 None。
#[inline]
#[must_use]
pub fn extract_index_number(obj: Tagged<Object>) -> Option<f64> {
  if obj.is_smi() {
    Some(Smi::value(obj.cast()) as f64)
  } else if obj.is_heap_object() && instance_type_of(obj).is_some_and(itc::is_heap_number) {
    // SAFETY: 已校验为合法 HeapNumber 实例类型，内存对齐地址经 address() 获取
    let hn = unsafe { &*(obj.address() as *const HeapNumber) };
    Some(hn.value)
  } else {
    None
  }
}

/// 比较两个索引元素，对齐 V8 std::sort 的 lambda 行为：
/// 1. 若两者均为数值，按数值大小比较；
/// 2. 数值项优先于 Undefined/非数值项；
/// 3. Undefined 项沉底。
#[inline]
#[must_use]
pub fn compare_indices(a: Tagged<Object>, b: Tagged<Object>) -> Ordering {
  match (extract_index_number(a), extract_index_number(b)) {
    (Some(na), Some(nb)) => na.partial_cmp(&nb).unwrap_or(Ordering::Equal),
    (Some(_), None) => Ordering::Less,
    (None, Some(_)) => Ordering::Greater,
    (None, None) => Ordering::Equal,
  }
}

/// v8: src/objects/elements.cc SortIndices
///
/// 对 indices 数组前 `sort_size` 项执行原位递增排序 (std::sort 原版对齐)。
/// 排序规则与 V8 严格一致：
/// 1. Smi / HeapNumber 等数值类型按其数值大小升序排列；
/// 2. Undefined / 空位项沉底到末尾；
/// 3. 零堆分配，原位切片排序。
///
/// # Safety
/// `indices` 必须为有效合法的 FixedArray 实例，数据区必须已正确初始化。
pub unsafe fn sort_indices(indices: Tagged<FixedArray>, sort_size: u32) {
  if sort_size <= 1 {
    return;
  }
  let len = unsafe { indices.length() };
  let n = (sort_size.min(len)) as usize;
  if n <= 1 {
    return;
  }
  // SAFETY: 索引槽位连续且处于有效长度范围内，转换为可变切片原位排序
  unsafe {
    let slot_ptr = indices.element_slot(0) as *mut Tagged<Object>;
    let slice = from_raw_parts_mut(slot_ptr, n);
    sort_indices_slice(slice);
  }
}

/// 对元素切片执行原位排序 (纯切片安全面，零堆分配)。
#[inline]
pub fn sort_indices_slice(slice: &mut [Tagged<Object>]) {
  if slice.len() <= 1 {
    return;
  }
  slice.sort_unstable_by(|&a, &b| compare_indices(a, b));
}

/// 对象元素拷贝配置参数。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CopyObjectElementsArgs {
  pub from_kind: ElementsKind,
  pub from_start: u32,
  pub to_kind: ElementsKind,
  pub to_start: u32,
  pub raw_copy_size: u32,
  pub hole_value: Tagged<Object>,
}

/// 复制至末尾并用 hole 填充的标记常量 (kMaxUInt32)。
pub const K_COPY_TO_END_AND_INITIALIZE_TO_HOLE: u32 = u32::MAX;

/// v8: src/objects/elements.cc CopyObjectToObjectElements
///
/// 将对象元素从 `from_base` 复制到 `to_base`：
/// - 支持 kCopyToEndAndInitializeToHole 模式 (自动用 `args.hole_value` 补齐尾部)；
/// - 根据 from_kind 与 to_kind 自动派发写屏障模式。
///
/// # Safety
/// from_base 与 to_base 必须有效且均为 FixedArray; 范围不越界。
pub unsafe fn copy_object_to_object_elements(
  from_base: Tagged<FixedArray>,
  to_base: Tagged<FixedArray>,
  args: CopyObjectElementsArgs,
) {
  let from_len = unsafe { from_base.length() };
  let to_len = unsafe { to_base.length() };
  debug_assert!(args.from_start <= from_len);
  debug_assert!(args.to_start <= to_len);

  let mut copy_size = args.raw_copy_size;
  if args.raw_copy_size == K_COPY_TO_END_AND_INITIALIZE_TO_HOLE {
    copy_size =
      (from_len.saturating_sub(args.from_start)).min(to_len.saturating_sub(args.to_start));
    let start = args.to_start + copy_size;
    if start < to_len {
      unsafe {
        to_base.fill(
          start,
          to_len,
          args.hole_value,
          WriteBarrierMode::SkipWriteBarrier,
        );
      }
    }
  }

  if copy_size == 0 {
    return;
  }
  debug_assert!(copy_size + args.to_start <= to_len);
  debug_assert!(copy_size + args.from_start <= from_len);

  let mode = if args.from_kind.is_object_elements_kind() && args.to_kind.is_object_elements_kind() {
    WriteBarrierMode::UpdateWriteBarrier
  } else {
    WriteBarrierMode::SkipWriteBarrier
  };

  unsafe {
    TaggedArrayExt::copy_elements(
      &to_base,
      args.to_start,
      &from_base,
      args.from_start,
      copy_size,
      mode,
    );
  }
}

/// v8: src/objects/elements.cc CopyDoubleToDoubleElements
///
/// 将双精度浮点数组元素从 `from_base` 复制到 `to_base`：
/// - 支持 kCopyToEndAndInitializeToHole 模式 (自动将剩余项设为 Hole)；
/// - 支持同数组内存重叠自搬迁 (memmove 保源序)。
///
/// # Safety
/// from_base 与 to_base 必须有效且为 FixedDoubleArray。
pub unsafe fn copy_double_to_double_elements(
  from_base: Tagged<FixedDoubleArray>,
  from_start: u32,
  to_base: Tagged<FixedDoubleArray>,
  to_start: u32,
  raw_copy_size: u32,
) {
  let from_len = from_base.length();
  let to_len = to_base.length();
  debug_assert!(from_start <= from_len);
  debug_assert!(to_start <= to_len);

  let mut copy_size = raw_copy_size;
  if raw_copy_size == K_COPY_TO_END_AND_INITIALIZE_TO_HOLE {
    copy_size = (from_len.saturating_sub(from_start)).min(to_len.saturating_sub(to_start));
    let start = to_start + copy_size;
    if start < to_len {
      unsafe {
        to_base.fill_with_holes(start, to_len);
      }
    }
  }

  if copy_size == 0 {
    return;
  }
  debug_assert!(copy_size + to_start <= to_len);
  debug_assert!(copy_size + from_start <= from_len);

  unsafe {
    let src_ptr =
      (from_base.begin() + (from_start as usize) * FixedDoubleArray::ELEMENT_SIZE) as *const f64;
    let dst_ptr =
      (to_base.begin() + (to_start as usize) * FixedDoubleArray::ELEMENT_SIZE) as *mut f64;
    ptr::copy(src_ptr, dst_ptr, copy_size as usize);
  }
}

/// v8: src/objects/elements.cc CopySmiToDoubleElements
///
/// 将 Smi 元素从 FixedArray 转换为 f64 并写入 FixedDoubleArray。
///
/// # Safety
/// from_base 必须为有效 FixedArray; to_base 必须为有效 FixedDoubleArray。
pub unsafe fn copy_smi_to_double_elements(
  from_base: Tagged<FixedArray>,
  from_start: u32,
  to_base: Tagged<FixedDoubleArray>,
  to_start: u32,
  raw_copy_size: u32,
  the_hole: Tagged<Object>,
) {
  let from_len = unsafe { from_base.length() };
  let to_len = to_base.length();
  debug_assert!(from_start <= from_len);
  debug_assert!(to_start <= to_len);

  let mut copy_size = raw_copy_size;
  if raw_copy_size == K_COPY_TO_END_AND_INITIALIZE_TO_HOLE {
    copy_size = (from_len.saturating_sub(from_start)).min(to_len.saturating_sub(to_start));
    let start = to_start + copy_size;
    if start < to_len {
      unsafe {
        to_base.fill_with_holes(start, to_len);
      }
    }
  }

  if copy_size == 0 {
    return;
  }
  debug_assert!(copy_size + to_start <= to_len);
  debug_assert!(copy_size + from_start <= from_len);

  unsafe {
    let from_slice = from_raw_parts(
      from_base.element_slot(from_start) as *const Tagged<Object>,
      copy_size as usize,
    );
    for (i, &hole_or_smi) in from_slice.iter().enumerate() {
      let idx = to_start + i as u32;
      if hole_or_smi == the_hole || !hole_or_smi.is_smi() {
        to_base.set_the_hole(idx);
      } else {
        to_base.set(idx, Smi::value(hole_or_smi.cast()) as f64);
      }
    }
  }
}

/// v8: src/objects/elements.cc CopyPackedSmiToDoubleElements
///
/// 将连续紧凑且无 Hole 的 Smi 元素快速转换为 f64 并写入 FixedDoubleArray。
///
/// # Safety
/// from_base 必须为有效 FixedArray; to_base 必须为有效 FixedDoubleArray。
pub unsafe fn copy_packed_smi_to_double_elements(
  from_base: Tagged<FixedArray>,
  from_start: u32,
  to_base: Tagged<FixedDoubleArray>,
  to_start: u32,
  packed_size: u32,
  raw_copy_size: u32,
) {
  let from_len = unsafe { from_base.length() };
  let to_len = to_base.length();
  debug_assert!(from_start <= from_len);
  debug_assert!(to_start <= to_len);
  debug_assert!(from_start <= packed_size);

  let mut copy_size = raw_copy_size;
  if raw_copy_size == K_COPY_TO_END_AND_INITIALIZE_TO_HOLE {
    copy_size = (packed_size.saturating_sub(from_start)).min(to_len.saturating_sub(to_start));
    let start = to_start + copy_size;
    if start < to_len {
      unsafe {
        to_base.fill_with_holes(start, to_len);
      }
    }
  }

  if copy_size == 0 {
    return;
  }
  debug_assert!(copy_size + to_start <= to_len);
  debug_assert!(copy_size + from_start <= from_len);

  let num_packed = packed_size.saturating_sub(from_start).min(copy_size);
  if num_packed > 0 {
    unsafe {
      let from_slice = from_raw_parts(
        from_base.element_slot(from_start) as *const Tagged<Object>,
        num_packed as usize,
      );
      for (i, &smi) in from_slice.iter().enumerate() {
        debug_assert!(smi.is_smi());
        to_base.set(to_start + i as u32, Smi::value(smi.cast()) as f64);
      }
    }
  }
}

/// v8: src/objects/elements.cc CopyObjectToDoubleElements
///
/// 将 FixedArray 中的 Number (Smi 或 HeapNumber) 转换为 f64 写入 FixedDoubleArray，
/// 若遇到 Hole 或非数值项则写入 Hole。
///
/// # Safety
/// from_base 必须为有效 FixedArray; to_base 必须为有效 FixedDoubleArray。
pub unsafe fn copy_object_to_double_elements(
  from_base: Tagged<FixedArray>,
  from_start: u32,
  to_base: Tagged<FixedDoubleArray>,
  to_start: u32,
  raw_copy_size: u32,
  the_hole: Tagged<Object>,
) {
  let from_len = unsafe { from_base.length() };
  let to_len = to_base.length();
  debug_assert!(from_start <= from_len);
  debug_assert!(to_start <= to_len);

  let mut copy_size = raw_copy_size;
  if raw_copy_size == K_COPY_TO_END_AND_INITIALIZE_TO_HOLE {
    copy_size = (from_len.saturating_sub(from_start)).min(to_len.saturating_sub(to_start));
    let start = to_start + copy_size;
    if start < to_len {
      unsafe {
        to_base.fill_with_holes(start, to_len);
      }
    }
  }

  if copy_size == 0 {
    return;
  }
  debug_assert!(copy_size + to_start <= to_len);
  debug_assert!(copy_size + from_start <= from_len);

  unsafe {
    let from_slice = from_raw_parts(
      from_base.element_slot(from_start) as *const Tagged<Object>,
      copy_size as usize,
    );
    for (i, &hole_or_object) in from_slice.iter().enumerate() {
      let idx = to_start + i as u32;
      if hole_or_object == the_hole {
        to_base.set_the_hole(idx);
      } else if let Some(val) = extract_index_number(hole_or_object) {
        to_base.set(idx, val);
      } else {
        to_base.set_the_hole(idx);
      }
    }
  }
}

/// v8: src/objects/elements.cc CopyFromFloat16BackingStore
///
/// 拷贝 float16 底层存储至目标缓冲区 (保持原始比特位)。
///
/// # Safety
/// src 与 dst 必须有效可访问且长度至少为 length。
pub unsafe fn copy_from_float16_to_f16(src: *const u16, dst: *mut u16, length: usize) {
  unsafe {
    ptr::copy(src, dst, length);
  }
}

/// 切片安全封装：拷贝 float16 切片并转为 f32。
#[inline]
pub fn copy_from_float16_slice_to_f32(src: &[u16], dst: &mut [f32]) {
  for (s, d) in src.iter().zip(dst.iter_mut()) {
    *d = Float16::from_bits(*s).to_float32();
  }
}

/// 切片安全封装：拷贝 float16 切片并转为 f64。
#[inline]
pub fn copy_from_float16_slice_to_f64(src: &[u16], dst: &mut [f64]) {
  for (s, d) in src.iter().zip(dst.iter_mut()) {
    *d = Float16::from_bits(*s).to_float32() as f64;
  }
}

/// v8: src/objects/elements.cc CopyFromFloat16BackingStore
///
/// 拷贝 float16 底层存储至目标 f32 缓冲区 (经 fp16_ieee_to_fp32 转换)。
///
/// # Safety
/// src 与 dst 必须有效可访问且长度至少为 length。
pub unsafe fn copy_from_float16_to_f32(src: *const u16, dst: *mut f32, length: usize) {
  if length == 0 {
    return;
  }
  // SAFETY: 调用方契约保证指针有效且长度至少为 length
  unsafe {
    let src_slice = from_raw_parts(src, length);
    let dst_slice = from_raw_parts_mut(dst, length);
    copy_from_float16_slice_to_f32(src_slice, dst_slice);
  }
}

/// v8: src/objects/elements.cc CopyFromFloat16BackingStore
///
/// 拷贝 float16 底层存储至目标 f64 缓冲区 (经 fp16_ieee_to_fp32 转换为 f64)。
///
/// # Safety
/// src 与 dst 必须有效可访问且长度至少为 length。
pub unsafe fn copy_from_float16_to_f64(src: *const u16, dst: *mut f64, length: usize) {
  if length == 0 {
    return;
  }
  // SAFETY: 调用方契约保证指针有效且长度至少为 length
  unsafe {
    let src_slice = from_raw_parts(src, length);
    let dst_slice = from_raw_parts_mut(dst, length);
    copy_from_float16_slice_to_f64(src_slice, dst_slice);
  }
}

// ── 深水推迟函数 (台账登记) ────────────────────────────────────────────────

/// v8: src/objects/elements.cc ThrowArrayLengthRangeError
pub fn throw_array_length_range_error() {
  todo!("readme 台账 ELEM-ISO: 需 Isolate + MessageTemplate + RangeError 异常体系");
}

/// v8: src/objects/elements.cc CopyDictionaryToObjectElements
pub fn copy_dictionary_to_object_elements() {
  todo!("readme 台账 ELEM-DICT: 需 NumberDictionary::FindEntry 字典哈希表查找与 Isolate");
}

/// v8: src/objects/elements.cc CopyDoubleToObjectElements
pub fn copy_double_to_object_elements() {
  todo!("readme 台账 ELEM-ISO: 需 Isolate + HandleScope 分配 HeapNumber 对象与写屏障");
}

/// v8: src/objects/elements.cc CopyDictionaryToDoubleElements
pub fn copy_dictionary_to_double_elements() {
  todo!("readme 台账 ELEM-DICT: 需 NumberDictionary::FindEntry 字典哈希表查找与 Isolate");
}

/// v8: src/objects/elements.cc IncludesValueSlowPath
pub fn includes_value_slow_path() {
  todo!("readme 台账 ELEM-SLOW: 需 DirectHandle<JSObject> 原型链遍历与 Isolate 句柄体系");
}

/// v8: src/objects/elements.cc IndexOfValueSlowPath
pub fn index_of_value_slow_path() {
  todo!("readme 台账 ELEM-SLOW: 需 DirectHandle<JSObject> 原型链遍历与 Isolate 句柄体系");
}

/// v8: src/objects/elements.cc ElementsAccessor::InitializeOncePerProcess
pub fn elements_accessor_initialize_once_per_process() {
  todo!("readme 台账 ELEM-STATIC: 需完整 32 个 ElementsAccessor 子类层次表全局初始化");
}

/// v8: src/objects/elements.cc ElementsAccessor::TearDown
pub fn elements_accessor_tear_down() {
  todo!("readme 台账 ELEM-STATIC: 需完整 ElementsAccessor 实例生命周期管理与全局静态清理");
}

/// v8: src/objects/elements.cc ElementsAccessor::Concat
pub fn elements_accessor_concat() {
  todo!("readme 台账 ELEM-ISO: 需 Isolate 与 JSArray 堆分配及原型链查找");
}

// ── 向后兼容别名 ────────────────────────────────────────────────────────────
pub use copy_dictionary_to_double_elements as copydictionarytodoubleelements;
pub use copy_dictionary_to_object_elements as copydictionarytoobjectelements;
pub use copy_double_to_double_elements as copydoubletodoubleelements;
pub use copy_double_to_object_elements as copydoubletoobjectelements;
pub use copy_from_float16_to_f32 as copyfromfloat16backingstore;
pub use copy_object_to_double_elements as copyobjecttodoubleelements;
pub use copy_object_to_object_elements as copyobjecttoobjectelements;
pub use copy_packed_smi_to_double_elements as copypackedsmitodoubleelements;
pub use copy_smi_to_double_elements as copysmitodoubleelements;
pub use elements_accessor_concat as elementsaccessor_concat;
pub use elements_accessor_initialize_once_per_process as elementsaccessor_initializeonceperprocess;
pub use elements_accessor_tear_down as elementsaccessor_teardown;
pub use get_write_barrier_mode as getwritebarriermode;
pub use includes_value_slow_path as includesvalueslowpath;
pub use index_of_value_slow_path as indexofvalueslowpath;
pub use is_float16_raw_bits_zero as isfloat16rawbitszero;
pub use is_identity_mapping_slice as isidentitymapping;
pub use sort_indices as sortindices;
pub use throw_array_length_range_error as throwarraylengthrangeerror;

// —— 提取不变式断言 (clang -E 对数; 分项计数生成侧锁定后烧入) ——
const _: () = {
  assert!(IMPL_ONLY_FREE_FUNCTIONS.len() == 16);
  assert!(STATIC_METHODS.len() == 3);
  assert!(TYPED_SPECIALIZATIONS.len() == 64);
  assert!(ACCESSOR_CLASSES.len() == 32);
  assert!(
    TYPED_SPECIALIZATIONS.len() + STATIC_METHODS.len() + IMPL_ONLY_FREE_FUNCTIONS.len() + 4 == 87
  );
  assert!(TYPED_SPECIALIZATIONS.len() == 24 + 36 + 4);
};
