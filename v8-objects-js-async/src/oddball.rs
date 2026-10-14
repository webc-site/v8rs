// 由 tool/src/objects/oddball-inl.h.js 从 v8/src/objects/oddball-inl.h 生成; 修订就地改; 复跑覆盖
// 席位表差集: kind 常量与单例构造/谓词面 (.h 布局归既有 oddball.h.js)
//! Oddball -inl 差集: 布尔/null/undefined 单例的 kind 编码与初始化。
//!
//! kind 码: False=0, True=1 (位 0 即真值), Null=3, Undefined=4;
//! 布尔判定 = 位 0 之外全零 (kind & kNotBooleanMask == 0)。

use v8_objects_core::{
  heap_number::HeapNumber,
  heap_object::HeapObject,
  instance_type::InstanceType,
  instance_type_checker::is_heap_number,
  map::Map,
  oddball::Oddball,
  smi::Smi,
  tagged::{Object, Tagged},
};

use crate::error::{Error, Result};

// ── kind 常量 (原版 static constexpr uint8_t) ──
pub const K_FALSE: u8 = 0;
pub const K_TRUE: u8 = 1;
pub const K_NOT_BOOLEAN_MASK: u8 = 0xFE;
pub const K_NULL: u8 = 3;
pub const K_UNDEFINED: u8 = 4;

/// 布尔判定 (原版 IsBoolean 谓词: kind & kNotBooleanMask == 0, 即 kind ∈ {0,1})。
#[inline]
#[must_use]
pub const fn is_boolean(kind: u8) -> bool {
  kind & K_NOT_BOOLEAN_MASK == 0
}

/// 真值读取 (原版 Boolean::ToBool = Is<True>: kind 位 0)。
#[inline]
#[must_use]
pub const fn to_bool(kind: u8) -> bool {
  kind & 0x1 != 0
}

/// 读取堆对象 map 的实例类型 (沙箱外 Buf 形态: 指针来自 [`Tagged::from_ref`])。
///
/// v8: src/objects/heap-object-inl.h HeapObject::map + map.h instance_type
///
/// # Safety
/// `value` 必须为非 Smi 强指针且指向存活的 HeapObject, 其 map 指向存活 Map
/// (对应原版 IsHeapNumber 谓词的内存前置; 本 crate Buf 形态由调用方构造保证)。
unsafe fn instance_type_of(value: Tagged<Object>) -> InstanceType {
  let heap_object: &HeapObject = unsafe { value.cast::<HeapObject>().as_ref() };
  let map: &Map = unsafe { heap_object.map().as_ref() };
  map.instance_type
}

/// 原版 FinishInitialization (inl: HeapNumber 走位拷贝, Smi 走 NumberValue)。
///
/// v8: src/objects/oddball-inl.h Oddball::FinishInitialization
///
/// to_number 为 HeapNumber 时按位模式写 to_number_raw (保 hole/NaN 位精确,
/// 对应原版 set_to_number_raw_as_bits, v8:8875 注释); Smi 时走
/// Object::NumberValue。三槽写入对应原版 SKIP_WRITE_BARRIER 直存。
pub fn finish_initialization(
  oddball: &mut Oddball,
  to_string: Tagged<Object>,
  to_number: Tagged<Object>,
  type_of: Tagged<Object>,
) -> Result<()> {
  if to_number.is_smi() {
    let smi = to_number
      .to_smi()
      .ok_or(Error::Invariant("to_number 应为 Smi"))?;
    oddball.to_number_raw = Smi::value(smi) as f64;
  } else {
    // SAFETY: 非 Smi 强指针, 指向存活堆对象 (Buf 构造前置, 见 unsafe fn 文档)。
    let ty = unsafe { instance_type_of(to_number) };
    if is_heap_number(ty) {
      // SAFETY: 实例类型已判定 HeapNumber, 指针指向存活 HeapNumber (原版 Cast<HeapNumber> 前置)。
      let heap_number: &HeapNumber = unsafe { to_number.cast::<HeapNumber>().as_ref() };
      oddball.to_number_raw = f64::from_bits(heap_number.value_as_bits());
    } else {
      return Err(Error::Invariant("FinishInitialization 需要 Number 输入"));
    }
  }
  oddball.to_number = to_number;
  oddball.to_string = to_string;
  oddball.type_of = type_of;
  Ok(())
}

/// 原版 Oddball::ToNumber (inl: to_number 缓存直读)。
///
/// v8: src/objects/oddball-inl.h Oddball::ToNumber (handle(input->to_number(), isolate)
/// 的 isolate 句柄化归上层, 此处交缓存字段本体)
#[inline]
#[must_use]
pub fn to_number(oddball: &Oddball) -> Tagged<Object> {
  oddball.to_number()
}

// ── 提取不变式断言 ──
const _: () = {
  assert!(K_FALSE == 0);
  assert!(K_TRUE == 1);
  assert!(K_NOT_BOOLEAN_MASK == 0xFE);
  assert!(K_NULL == 3);
  assert!(K_UNDEFINED == 4);
  assert!(is_boolean(K_FALSE) && is_boolean(K_TRUE));
  assert!(!is_boolean(K_NULL) && !is_boolean(K_UNDEFINED));
  assert!(!to_bool(K_FALSE) && to_bool(K_TRUE));
};
