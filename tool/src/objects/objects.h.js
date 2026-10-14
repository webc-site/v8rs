#!/usr/bin/env bun
// tool/src/objects/objects.h.js — 覆盖 objects.h 家族 (同名族合并为一个模块, todo 十一):
//   v8/src/objects/objects.h (枚举/常量/BooleanBit)
//   + v8/src/objects/casting.h (Cast 体系 → Rust trait)
//   + v8/src/objects/object-conversions-inl.h (NumberValue/ToUint32/ToArrayLength/
//     ToArrayIndex/ToIntegerIndex)
//   + v8/src/objects/objects-inl.h 的类型无关谓词 (IsSpecialReceiverInstanceType 等,
//     已由 instance-type-checker.h.js 承载, 此处登记)
//   → v8-objects-core/src/objects.rs
// objects.cc 语义区段由 tool/src/objects/objects.cc.js 以区段标记注入本文件尾部;
// DEF_CAST_TRAITS 登记表由 tool/src/objects/object-macros.h.js 以区段标记注入。
//
// 映射登记 (C++ → Rust):
//   enum WriteBarrierMode/PropertyNormalizationMode/TransitionFlag/TransitionKindFlag/
//   DescriptorFlag/ComparisonResult/OnNonExistent/ElementTypes/EnforceDefineSemantics/
//   EnsureElementsMode/AccessorComponent → 独立 enum (kFoo → Foo)
//   ComparisonResult 显式值 (-1..2) 1:1
//   kVariableSizeSentinel/kStubMajorKeyBits/kStubMinorKeyBits → const
//     (kSmiValueSize 取 32, 64 位未压缩指针模型 SmiTagging<8>)
//   casting.h CastTraits<T>/Is<T>/Cast<T>/TryCast<T>/UncheckedCast<T> →
//     trait Cast { allow_from } + 泛型函数 is/try_cast/cast/unchecked_cast
//     (glossary: C++ 继承 → 包装类型 + trait)
//   Object::Conversion/Smi::ToInt 等 → 见 objects.cc.js 区段
//
// 跳过区段登记 (推迟):
//   - Object 的 Isolate/Handle 依赖方法 (ToObject/ToPrimitive/ToNumber/ToString/
//     GetProperty/SetProperty/Equals/Compare/TypeOf/Add/InstanceOf/... 全家族):
//     归 L3+ v8-objects-js-core / v8-execution-isolate / LookupIterator
//   - Print/ShortPrint/Brief/operator<< (objects.h:655-698, objects.cc:1959-2006):
//     归 L4 v8-diagnostics
//   - Relocatable/SharedObjectSafePublishGuard/SharedObjectConditionalSafePublishGuard
//     (GC/isolate 依赖): 归 v8-heap-gc
//   - FixedBodyDescriptor/FlexibleWeakBodyDescriptor 前置声明 (objects.h:700-713):
//     归 visitors 族 (OC-C)
//   - WriteBarrierMode 的实际屏障执行 (WRITE_BARRIER 宏族): 归 v8-heap-core
//   - DEF_HEAP_OBJECT_PREDICATE 族 (objects-inl.h:494-617): 依赖 map()/isolate,
//     由 Cast 体系 + instance_type_checker 承载类型无关子集
import { banner, readV8, writeSrc } from "../../lib.js";

const CRATE = "v8-objects-core";
const src = readV8("src/objects/objects.h");

// 机械提取: enum 块 (名 + 变体 + 可选显式值)
function extractEnum(name) {
  const re = new RegExp(`enum (?:class )?${name}\\s*(?::\\s*([\\w:]+))?\\s*\\{([\\s\\S]*?)\\};`);
  const m = src.match(re);
  if (!m) throw new Error(`objects.h 中找不到 enum ${name}`);
  const body = m[2].replace(/\/\/[^\n]*/g, "");
  const variants = [];
  for (let raw of body.split(",")) {
    raw = raw.trim();
    if (!raw) continue;
    const vm = raw.match(/^(\w+)(?:\s*=\s*([^]+))?$/);
    if (!vm) continue;
    variants.push({ name: vm[1], value: vm[2]?.trim() ?? null });
  }
  if (!variants.length) throw new Error(`enum ${name} 变体提取为空`);
  return variants;
}

const ENUMS = [
  "WriteBarrierMode", "PropertyNormalizationMode", "TransitionFlag", "TransitionKindFlag",
  "DescriptorFlag", "ComparisonResult", "OnNonExistent", "ElementTypes",
  "EnforceDefineSemantics", "EnsureElementsMode", "AccessorComponent",
];
// 变体名统一 PascalCase: kLessThan → LessThan; SKIP_WRITE_BARRIER → SkipWriteBarrier
function pascal(v) {
  let s = v.replace(/^k(?=[A-Z])/, "");
  return s
    .split("_")
    .filter(Boolean)
    .map((p) => {
      // 纯大写段 (SKIP) → 首大写+尾部小写; 驼峰段 (StringAndSymbol) 原样
      const tail = p === p.toUpperCase() ? p.slice(1).toLowerCase() : p.slice(1);
      return p[0].toUpperCase() + tail;
    })
    .join("");
}
const enumBlocks = ENUMS.map((n) => {
  const vs = extractEnum(n);
  const lines = vs.map((v) => {
    const rn = pascal(v.name);
    if (!/^[A-Z][A-Za-z0-9]*$/.test(rn)) throw new Error(`变体名转换失败: ${v.name} → ${rn}`);
    return v.value !== null ? `  ${rn} = ${v.value},` : `  ${rn},`;
  });
  const repr = n === "ComparisonResult" ? "#[repr(i32)]\n" : "";
  return `/// 原版 enum ${n} (objects.h)。\n#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\n${repr}pub enum ${n} {\n${lines.join("\n")}\n}`;
});
// 对数断言: ComparisonResult 显式值锚点
const cmp = extractEnum("ComparisonResult");
if (cmp[0].value !== "-1" || cmp[3].value !== "2") throw new Error("ComparisonResult 值锚点失配");
if (extractEnum("WriteBarrierMode").length !== 6) throw new Error("WriteBarrierMode 变体数失配");

// ---------------------------------------------------------------------------
// Rust 发射 (类型层; 语义函数由 objects.cc.js 区段注入)
// ---------------------------------------------------------------------------
const body = `${banner("src/objects/objects.h.js", "src/objects/objects.h + casting.h + object-conversions-inl.h", "objects.h 家族合并: 枚举/常量/BooleanBit 机械展开 + casting.h Cast 体系 trait 化; objects.cc 语义区段由 objects.cc.js 注入")}
//! Object 语义核心: 顶层对象枚举/常量 + Cast 体系 + 数值转换语义。
//!
//! C++ Object 是 AllStatic 静态类; Rust 侧语义函数挂在自由函数与
//! \`impl Tagged<Object>\` 上, Cast 体系建模为 trait + 泛型 (glossary)。

use crate::{
  error::{Error, Result},
  heap_number::HeapNumber,
  heap_object::HeapObject,
  instance_type::InstanceType,
  instance_type_checker as itc,
  map::Map,
  oddball::Oddball,
  smi::Smi,
  symbol::Symbol,
  tagged::{Object, Tagged},
};

// ── 枚举 (objects.h:51-131, 715-722, 机械展开) ─────────────────────────────
${enumBlocks.join("\n\n")}

// ── 常量 (objects.h:98-104) ────────────────────────────────────────────────
/// 变长对象实例尺寸哨兵。
pub const K_VARIABLE_SIZE_SENTINEL: i32 = 0;
/// Stub 主键位宽。
pub const K_STUB_MAJOR_KEY_BITS: i32 = 8;
/// Stub 次键位宽 (本 crate 为 64 位未压缩指针模型, SmiTagging<8> kSmiValueSize=32)。
pub const K_STUB_MINOR_KEY_BITS: i32 = 32 - K_STUB_MAJOR_KEY_BITS - 1;

// ── BooleanBit (objects.h:750-764) ─────────────────────────────────────────
/// 整数位读写助手。
pub struct BooleanBit;

impl BooleanBit {
  /// 原版 BooleanBit::get。
  #[inline]
  #[must_use]
  pub const fn get(value: i32, bit_position: u32) -> bool {
    (value & (1 << bit_position)) != 0
  }

  /// 原版 BooleanBit::set。
  #[inline]
  #[must_use]
  pub const fn set(value: i32, bit_position: u32, v: bool) -> i32 {
    if v {
      value | (1 << bit_position)
    } else {
      value & !(1 << bit_position)
    }
  }
}

// ── Cast 体系 (casting.h → trait + 泛型) ──────────────────────────────────
/// 对应 C++ \`CastTraits<T>::AllowFrom(Tagged<Object>)\`:
/// 判定任意 Tagged 值能否视为 \`T\` (含 Smi/堆对象区分与实例类型检查)。
pub trait Cast: Sized {
  /// 值是否属于本类型。
  fn allow_from(value: Tagged<Object>) -> bool;
}

/// 对应 C++ \`Is<T>(value)\`。
#[inline]
#[must_use]
pub fn is<T: Cast>(value: Tagged<Object>) -> bool {
  T::allow_from(value)
}

/// 对应 C++ \`TryCast<T>(value, &out)\`。
#[inline]
#[must_use]
pub fn try_cast<T: Cast>(value: Tagged<Object>) -> Option<Tagged<T>> {
  if T::allow_from(value) {
    Some(value.cast())
  } else {
    None
  }
}

/// 带总是校验的转换 (对应 C++ \`CheckedCast<T>\`; 禁 panic → Result)。
#[inline]
pub fn cast<T: Cast>(value: Tagged<Object>) -> Result<Tagged<T>> {
  try_cast(value).ok_or(Error::Unsupported("cast: 类型不匹配"))
}

/// 对应 C++ \`UncheckedCast<T>\`: 零检查裸转 (调用者保证类型正确)。
#[inline]
#[must_use]
pub const fn unchecked_cast<T>(value: Tagged<Object>) -> Tagged<T> {
  value.cast()
}

/// 读取堆对象 Map 的实例类型; Smi / 非堆对象返回 None。
///
/// # Safety 显式证明
/// 堆对象指针由调用方保证有效 (Tagged::is_heap_object 只验证 tag 位);
/// Map 首字段偏移 0 (HEAP_OBJECT_MAP_OFFSET), 读 Map 结构体的 instance_type
/// 字段不涉及写。对象来自 Zone 分配或真实堆, 与 C++ map()->instance_type() 等价。
#[inline]
#[must_use]
pub fn instance_type_of(obj: Tagged<Object>) -> Option<InstanceType> {
  let ho = obj.to_heap_object()?;
  // SAFETY: obj 已验证堆对象 tag; as_ptr 还原对齐地址后读首字段 Map 指针,
  // 再读 Map::instance_type (Copy 字段)。两跳均为只读。
  unsafe {
    let map: Tagged<Map> = (*ho.as_ptr()).map;
    Some((*map.as_ptr()).instance_type)
  }
}

impl Cast for Object {
  #[inline]
  fn allow_from(_value: Tagged<Object>) -> bool {
    true
  }
}

impl Cast for Smi {
  #[inline]
  fn allow_from(value: Tagged<Object>) -> bool {
    value.is_smi()
  }
}

impl Cast for HeapObject {
  #[inline]
  fn allow_from(value: Tagged<Object>) -> bool {
    value.is_heap_object()
  }
}

/// Number 伪类型标记 (Smi 或 HeapNumber; C++ UnionOf<Smi, HeapNumber>)。
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Number;

impl Cast for Number {
  #[inline]
  fn allow_from(value: Tagged<Object>) -> bool {
    value.is_smi() || instance_type_of(value).is_some_and(itc::is_heap_number)
  }
}

impl Cast for HeapNumber {
  #[inline]
  fn allow_from(value: Tagged<Object>) -> bool {
    instance_type_of(value).is_some_and(itc::is_heap_number)
  }
}

impl Cast for Map {
  #[inline]
  fn allow_from(value: Tagged<Object>) -> bool {
    instance_type_of(value).is_some_and(itc::is_map)
  }
}

impl Cast for Symbol {
  #[inline]
  fn allow_from(value: Tagged<Object>) -> bool {
    instance_type_of(value).is_some_and(itc::is_symbol)
  }
}

impl Cast for Oddball {
  #[inline]
  fn allow_from(value: Tagged<Object>) -> bool {
    instance_type_of(value).is_some_and(itc::is_oddball)
  }
}

// == objects.cc 语义区段 (由 tool/src/objects/objects.cc.js 维护) ==
// @cc-semantics-begin
// @cc-semantics-end

// == object-macros 展开登记 (由 tool/src/objects/object-macros.h.js 维护) ==
// @object-macros-begin
// @object-macros-end
`;

writeSrc(CRATE, "objects.rs", body);
console.log(`objects.rs: 枚举 ${ENUMS.length} + Cast 体系 + 常量; ComparisonResult/WriteBarrierMode 锚点断言通过`);
