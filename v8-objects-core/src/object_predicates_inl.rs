// 由 tool/src/objects/object-predicates-inl.h.js 从 v8/src/objects/object-predicates-inl.h 生成; 修订就地改, 复跑覆盖
// 13 定义 / 9 语义; 名单表面在 object_predicates.rs

//! 对象谓词内联实现 (原版 object-predicates-inl.h)
//!
//! 名单型谓词 (Is##Type X-macro 家族) 的声明与展开表面在
//! object_predicates.rs; 本模块承接通用谓词与特化谓词内联实现。

use crate::{
  heap_number::HeapNumber,
  heap_object::HeapObject,
  instance_type_checker,
  smi::Smi,
  symbol::Symbol,
  tagged::{MaybeObject, Object, Tagged, TaggedIndex},
};

/// Smi 判定 (原版 IsSmi(Tagged<Object>))
#[inline]
pub const fn is_smi(obj: Tagged<Object>) -> bool {
  obj.is_smi()
}

/// 强堆对象判定 (原版 IsHeapObject(Tagged<Object>))
#[inline]
pub const fn is_heap_object(obj: Tagged<Object>) -> bool {
  obj.is_heap_object()
}

/// 弱引用判定 (原版 IsWeak(TaggedImpl<WEAK, ...>))
#[inline]
pub const fn is_weak(obj: Tagged<MaybeObject>) -> bool {
  obj.is_weak_live()
}

/// 对象判定 (原版 IsObject: 强堆引用或 Smi)
#[inline]
pub const fn is_object(obj: Tagged<MaybeObject>) -> bool {
  obj.is_object()
}

/// TaggedIndex 判定: Smi 且值域经 TaggedIndex::IsValid
#[inline]
pub fn is_tagged_index(obj: Tagged<Object>) -> bool {
  obj.is_smi() && TaggedIndex::is_valid(TaggedIndex::value(obj.cast()))
}

/// 零值判定: 与 Smi::zero() 全等
#[inline]
pub fn is_zero(obj: Tagged<Object>) -> bool {
  obj == Smi::zero().cast()
}

/// 无共享名哨兵判定 (SharedFunctionInfo::kNoSharedNameSentinel 即 Smi::zero())
#[inline]
pub fn is_no_shared_name_sentinel(obj: Tagged<Object>) -> bool {
  obj == Smi::zero().cast()
}

/// 可作原型跟踪的 JSObject (Object 重载先 TryCast HeapObject)
#[inline]
pub fn is_js_object_that_can_be_tracked_as_prototype(obj: Tagged<Object>) -> bool {
  if !obj.is_heap_object() {
    return false;
  }
  let ho = Tagged::<HeapObject>::new(obj.ptr());
  unsafe {
    let map = ho.map();
    instance_type_checker::is_js_object((*map.as_ptr()).instance_type())
  }
}

/// 可作原型跟踪的任意对象 (JSObject 或 WasmObject, 排除共享空间)
#[inline]
pub fn is_any_object_that_can_be_tracked_as_prototype(obj: Tagged<Object>) -> bool {
  if !obj.is_heap_object() {
    return false;
  }
  let ho = Tagged::<HeapObject>::new(obj.ptr());
  unsafe {
    let map = ho.map();
    let it = (*map.as_ptr()).instance_type();
    instance_type_checker::is_js_object(it) || instance_type_checker::is_wasm_object(it)
  }
}

/// Number 判定: Smi 或 HeapNumber
///
/// # Safety
/// 若 obj 为堆对象，调用方须确保其指向有效堆内存。
#[inline]
pub unsafe fn is_number(obj: Tagged<Object>) -> bool {
  if obj.is_smi() {
    return true;
  }
  // SAFETY: 调用方保证堆对象指向有效堆内存
  let map = unsafe { obj.cast::<HeapObject>().map() };
  // SAFETY: map 指向有效 Map 结构体
  let instance_type = unsafe { (*map.as_ptr()).instance_type() };
  instance_type_checker::is_heap_number(instance_type)
}

/// Numeric 判定: Smi 或 HeapNumber 或 BigInt
///
/// # Safety
/// 若 obj 为堆对象，调用方须确保其指向有效堆内存。
#[inline]
pub unsafe fn is_numeric(obj: Tagged<Object>) -> bool {
  if obj.is_smi() {
    return true;
  }
  // SAFETY: 调用方保证堆对象指向有效堆内存
  let map = unsafe { obj.cast::<HeapObject>().map() };
  // SAFETY: map 指向有效 Map 结构体
  let instance_type = unsafe { (*map.as_ptr()).instance_type() };
  instance_type_checker::is_heap_number(instance_type)
    || instance_type_checker::is_big_int_base(instance_type)
}

/// NaN 判定: 是 HeapNumber 且值为 NaN
///
/// # Safety
/// 若 obj 为堆对象，调用方须确保其指向有效堆内存。
#[inline]
pub unsafe fn is_nan(obj: Tagged<Object>) -> bool {
  if !obj.is_heap_object() {
    return false;
  }
  // SAFETY: 调用方保证堆对象指向有效堆内存
  let map = unsafe { obj.cast::<HeapObject>().map() };
  // SAFETY: map 指向有效 Map 结构体
  let instance_type = unsafe { (*map.as_ptr()).instance_type() };
  if instance_type_checker::is_heap_number(instance_type) {
    // SAFETY: instance_type 确认当前堆对象为 HeapNumber 且首地址对齐有效
    let heap_num = unsafe { &*(obj.cast::<HeapObject>().address() as *const HeapNumber) };
    heap_num.value().is_nan()
  } else {
    false
  }
}

/// 负零判定: 是 HeapNumber 且值为 -0.0
///
/// # Safety
/// 若 obj 为堆对象，调用方须确保其指向有效堆内存。
#[inline]
pub unsafe fn is_minus_zero(obj: Tagged<Object>) -> bool {
  if !obj.is_heap_object() {
    return false;
  }
  // SAFETY: 调用方保证堆对象指向有效堆内存
  let map = unsafe { obj.cast::<HeapObject>().map() };
  // SAFETY: map 指向有效 Map 结构体
  let instance_type = unsafe { (*map.as_ptr()).instance_type() };
  if instance_type_checker::is_heap_number(instance_type) {
    // SAFETY: instance_type 确认当前堆对象为 HeapNumber 且首地址对齐有效
    let heap_num = unsafe { &*(obj.cast::<HeapObject>().address() as *const HeapNumber) };
    let val = heap_num.value();
    val == 0.0 && val.is_sign_negative()
  } else {
    false
  }
}

/// 公开 Symbol 判定: 是 Symbol 且非任意私有
///
/// # Safety
/// 若 obj 为堆对象，调用方须确保其指向有效堆内存。
#[inline]
pub unsafe fn is_public_symbol(obj: Tagged<Object>) -> bool {
  if !obj.is_heap_object() {
    return false;
  }
  // SAFETY: 调用方保证堆对象指向有效堆内存
  let map = unsafe { obj.cast::<HeapObject>().map() };
  // SAFETY: map 指向有效 Map 结构体
  let instance_type = unsafe { (*map.as_ptr()).instance_type() };
  if instance_type_checker::is_symbol(instance_type) {
    // SAFETY: instance_type 确认当前堆对象为 Symbol
    let sym = unsafe { &*(obj.cast::<HeapObject>().address() as *const Symbol) };
    !sym.is_any_private()
  } else {
    false
  }
}

/// 私有 Symbol 判定: 是 Symbol 且为私有
///
/// # Safety
/// 若 obj 为堆对象，调用方须确保其指向有效堆内存。
#[inline]
pub unsafe fn is_private_symbol(obj: Tagged<Object>) -> bool {
  if !obj.is_heap_object() {
    return false;
  }
  // SAFETY: 调用方保证堆对象指向有效堆内存
  let map = unsafe { obj.cast::<HeapObject>().map() };
  // SAFETY: map 指向有效 Map 结构体
  let instance_type = unsafe { (*map.as_ptr()).instance_type() };
  if instance_type_checker::is_symbol(instance_type) {
    // SAFETY: instance_type 确认当前堆对象为 Symbol
    let sym = unsafe { &*(obj.cast::<HeapObject>().address() as *const Symbol) };
    sym.is_any_private()
  } else {
    false
  }
}

/// UniqueName 判定: 内化字符串或 Symbol
///
/// # Safety
/// 若 obj 为堆对象，调用方须确保其指向有效堆内存。
#[inline]
pub unsafe fn is_unique_name(obj: Tagged<Object>) -> bool {
  if !obj.is_heap_object() {
    return false;
  }
  // SAFETY: 调用方保证堆对象指向有效堆内存
  let map = unsafe { obj.cast::<HeapObject>().map() };
  // SAFETY: map 指向有效 Map 结构体
  let instance_type = unsafe { (*map.as_ptr()).instance_type() };
  instance_type_checker::is_internalized_string(instance_type)
    || instance_type_checker::is_symbol(instance_type)
}

/// IsShared 直判 InstanceType 分段表 (15 个 case, WASM 开: 共享串 6 + wasm 2 + 内化串 6 + HEAP_NUMBER 1)
///
/// - 直接共享: 共享串 6 种 (SHARED_{SEQ,EXTERNAL,UNCACHED_EXTERNAL}_{ONE,TWO}_BYTE)
/// - 条件共享: WASM_STRUCT/WASM_ARRAY (InAnySharedSpace), 内化串 6 种
///   (v8_flags.shared_string_table 开时共享), HEAP_NUMBER (InWritableSharedSpace)
pub const IS_SHARED_CASES: [&str; 15] = [
  "SHARED_SEQ_TWO_BYTE_STRING_TYPE",
  "SHARED_SEQ_ONE_BYTE_STRING_TYPE",
  "SHARED_EXTERNAL_TWO_BYTE_STRING_TYPE",
  "SHARED_EXTERNAL_ONE_BYTE_STRING_TYPE",
  "SHARED_UNCACHED_EXTERNAL_TWO_BYTE_STRING_TYPE",
  "SHARED_UNCACHED_EXTERNAL_ONE_BYTE_STRING_TYPE",
  "WASM_STRUCT_TYPE",
  "WASM_ARRAY_TYPE",
  "INTERNALIZED_TWO_BYTE_STRING_TYPE",
  "INTERNALIZED_ONE_BYTE_STRING_TYPE",
  "EXTERNAL_INTERNALIZED_TWO_BYTE_STRING_TYPE",
  "EXTERNAL_INTERNALIZED_ONE_BYTE_STRING_TYPE",
  "UNCACHED_EXTERNAL_INTERNALIZED_TWO_BYTE_STRING_TYPE",
  "UNCACHED_EXTERNAL_INTERNALIZED_ONE_BYTE_STRING_TYPE",
  "HEAP_NUMBER_TYPE",
];

/// 共享性判定 (原版 IsShared, 快路径 IsAlwaysSharedSpaceJSObject + switch 表)
#[inline]
pub fn is_shared(obj: Tagged<Object>) -> bool {
  if obj.is_smi() {
    return true;
  }
  let ho = Tagged::<HeapObject>::new(obj.ptr());
  unsafe {
    let map = ho.map();
    let it = (*map.as_ptr()).instance_type();
    if instance_type_checker::is_always_shared_space_js_object(it) {
      return true;
    }
    instance_type_checker::is_shared_string(it) || instance_type_checker::is_internalized_string(it)
  }
}
