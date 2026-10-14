#!/usr/bin/env bun
// tool/src/objects/fixed-array.h.js — 覆盖 fixed-array 同名族骨架:
//   v8/src/objects/fixed-array.h + fixed-array-inl.h + fixed-array.tq
//   → v8-objects-arrays/src/fixed_array.rs
//   (fixed-array.cc 纯数组语义由 tool/src/objects/fixed-array.cc.js
//    以 @cc-semantics 区段注入)
//
// 映射登记 (C++ → Rust):
//   TaggedArrayBase<Derived, ElementT, Super> (CRTP) →
//     trait TaggedArray (Element 关联类型 + 槽位/数据区偏移关联常量) +
//     TaggedArrayExt blanket impl (impl<T: TaggedArray> .. for Tagged<T>)
//     (glossary: C++ 模板继承 → trait + 泛型)
//   9 个 TaggedArrayBase 具体族 (fixed-array.h 机械提取) →
//     header struct + impl TaggedArray (字段偏移 offset_of)
//   PrimitiveArrayBase<Derived, ElementT, Super> → 移交 (见下)
//   FLEXIBLE_ARRAY_MEMBER (objects 尾) → 基址 + 数据区偏移裸访问
//     (DST 惯用法, 同 v8-objects-strings Seq 串先例)
//   ElementT: Object / MaybeObject (core) + UnionOf<TrustedObject, Smi> /
//     UnionOf<MaybeWeak<TrustedObject>, Smi> (本模块标记类型)
//   kSupportsSmiElements → set_smi 族 (四元素并集均容 Smi, SKIP 屏障)
//   kDefaultMode → UPDATE_WRITE_BARRIER 恒定 (无纯 Smi 元素族)
//   SafeHeapObjectSize → u32; V8_IT_OWN_TYPE / kMapRootIndex → 推迟 roots 表
//   FixedArrayExact (final : FixedArray) → type alias + is_fixed_array_exact
//     精确谓词 (别名与 FixedArray 共享 Cast 区间谓词, 精确谓词独立成函数)
//
// 移交登记 (并发代理互斥文件清单):
//   PrimitiveArrayBase 族 (ByteArray/TrustedByteArray/FixedDoubleArray/
//     FixedIntegerArray 8 别名 + TrustedFixedAddressArray) 原版位于
//     fixed-primitive-array.h → 归 v8-objects-arrays/src/primitive_array.rs
//     (tool/src/objects/fixed-primitive-array.h.js), 本模块不实现
//
// 跳过区段登记 (推迟):
//   - New/Allocate/SetAndGrow/Grow/Resize/RightTrimOrEmpty/EnsureSpace/
//     AddToEnd/Append/ToFixedArray/CompactWeakArrayList (isolate+factory+
//     heap 依赖) → L3 heap (RightTrim 以不变式校验 + Err 登记)
//   - MoveElements/CopyElements (heap MoveRange/CopyRange 写屏障簿记) →
//     L3 heap
//   - set 内 DCHECK(!IsCowArray()) (roots 依赖) → 调用方契约登记
//   - clear_optional_padding (TAGGED_SIZE_8_BYTES-only padding, snapshot 层;
//     运行时结构体天然对齐)
//   - BodyDescriptor / DECL_VERIFIER / DECL_PRINTER 族 → heap 访客层
import { spawnSync } from "node:child_process";
import { join } from "node:path";
import { banner, readV8, writeSrc } from "../../lib.js";

// 仓库 rustfmt 归一 (rustfmt.toml): 生成即 CI fmt 形态, 复跑幂等; 解析失败即 throw。
const repoRoot = join(import.meta.dir, "..", "..", "..");
function fmt(body) {
  const r = spawnSync(
    "rustfmt",
    ["--edition", "2024", "--config-path", join(repoRoot, "rustfmt.toml")],
    { input: body, encoding: "utf8" },
  );
  if (r.status !== 0) throw new Error(`rustfmt 归一失败: ${r.stderr}`);
  return r.stdout;
}

const h = readV8("src/objects/fixed-array.h");
const inl = readV8("src/objects/fixed-array-inl.h");
const tq = readV8("src/objects/fixed-array.tq");

// ── 锚点 ────────────────────────────────────────────────────────────────
for (const [sub, file] of [
  ["class TaggedArrayBase : public Super {", h],
  ["V8_OBJECT class FixedArray\n    : public TaggedArrayBase<FixedArray, Object, FixedArrayBase> {", h],
  ["class WeakArrayList\n    : public TaggedArrayBase<WeakArrayList, MaybeObject, HeapObject> {", h],
  ["class ArrayList\n    : public TaggedArrayBase<ArrayList, Object, HeapObject> {", h],
  ["class FixedArrayExact final : public FixedArray {", h],
  ["TaggedArrayBase<D, ElementT_, P>::get(uint32_t index) const {", inl],
  ["constexpr uint32_t TaggedArrayBase<D, ElementT_, P>::NewCapacityForIndex(", inl],
  ["static constexpr uint32_t CapacityForLength(uint32_t length) {", h],
  ["extern class FixedArray extends FixedArrayBase {", tq],
]) {
  if (!file.includes(sub)) throw new Error(`锚点失配: 无 "${sub.slice(0, 52)}..."`);
}

// ── 9 个 TaggedArrayBase 具体族机械提取 ───────────────────────────────────
// V8_OBJECT class <Name> : public TaggedArrayBase<Name, <Element>, <Super>>
const decl =
  /V8_OBJECT\s+class\s+(\w+)\s*(?:final)?\s*:\s*public\s+TaggedArrayBase<\s*(\w+),\s*(.+),\s*(FixedArrayBase|TrustedFixedArrayBase|HeapObject)\s*>\s*\{/g;
const fams = [];
for (const m of h.matchAll(decl)) {
  const [, name, self, element, superCls] = m;
  if (name !== self) throw new Error(`CRTP 自引用失配: ${name} != ${self}`);
  fams.push({ name, element: element.trim(), super: superCls.trim() });
}
if (fams.length !== 9) throw new Error(`具体族 ${fams.length} != 9`);
// 元素类型 → Rust 标记
const ELEMENT_MAP = {
  Object: "Object",
  MaybeObject: "MaybeObject",
  "UnionOf<TrustedObject, Smi>": "TrustedObjectOrSmi",
  "UnionOf<MaybeWeak<TrustedObject>, Smi>": "MaybeWeakTrustedObjectOrSmi",
};
for (const f of fams) {
  f.elementRs = ELEMENT_MAP[f.element];
  if (!f.elementRs) throw new Error(`${f.name} 元素类型未映射: ${f.element}`);
}
// Super → 头部形态 (字段名/基类型)
const SUPER_FIELD = {
  FixedArrayBase: ["base", "FixedArrayBase"],
  TrustedFixedArrayBase: ["trusted_base", "TrustedFixedArrayBase"],
  HeapObject: [null, "HeapObject"],
};
const capLenFams = fams.filter((f) => f.super === "HeapObject").map((f) => f.name);
if (capLenFams.join() !== "WeakArrayList,ArrayList")
  throw new Error(`capacity+length 族漂移: ${capLenFams}`);

// ── 族表 → Rust 生成 ─────────────────────────────────────────────────────
const snake = (s) =>
  s.replace(/(?<=[a-z0-9])(?=[A-Z])/g, "_").replace(/(?<=[A-Z])(?=[A-Z][a-z])/g, "_").toLowerCase();

// ── 修订就地并入 (原 in-place 修订, 归一化元素步长 = ELEMENT_SIZE) ──────────
// 三族的结构体侧 SizeFor/OffsetOfElementAt/CapacityForLength/SizeForCapacity
// (fixed-array.h:210-225/600-663/733-740); 与 TaggedArrayExt::size_for 同式。
const SIZE_FOR_HELPER = `
  /// 计算全对象所需字节大小 (= TaggedArrayExt::size_for, 元素步长同轨)
  #[inline]
  #[must_use]
  pub const fn size_for(capacity: u32) -> usize {
    Self::OBJECTS_OFFSET + (capacity as usize) * Self::ELEMENT_SIZE
  }
`;
const OFFSET_OF_HELPER = `
  /// 元素偏移量 (= OffsetOfElementAt)
  #[inline]
  #[must_use]
  pub const fn offset_of_element_at(index: u32) -> usize {
    Self::size_for(index)
  }
`;
const CAP_FOR_LEN_HELPER = `
  /// 增长容量计算 (CapacityForLength: length + max(length/2, 2))
  #[inline]
  #[must_use]
  pub const fn capacity_for_length(length: u32) -> u32 {
    let half = length / 2;
    length + if half > 2 { half } else { 2 }
  }
`;
const EXTRA_HELPERS = {
  FixedArray: SIZE_FOR_HELPER + OFFSET_OF_HELPER,
  WeakArrayList:
    SIZE_FOR_HELPER +
    `
  /// 依据容量计算对象大小 (= SizeForCapacity)
  #[inline]
  #[must_use]
  pub const fn size_for_capacity(capacity: u32) -> usize {
    Self::size_for(capacity)
  }
` +
    CAP_FOR_LEN_HELPER,
  ArrayList: SIZE_FOR_HELPER + CAP_FOR_LEN_HELPER,
};

const familyStructImpl = (n, body) => `impl ${n} {
  /// 构造头。
  #[inline]
  pub const fn ${body}
${EXTRA_HELPERS[n] ?? ""}}`;

function familyStruct(f) {
  const n = f.name;
  if (f.super === "HeapObject") {
    return `/// ${n} (fixed-array.h: TaggedArrayBase<${n}, ${f.element}, HeapObject>)。
///
/// 自声明 capacity_ + length_ 双 u32 头 (无基类 length 继承);
/// 元素尾 (objects) 经基址 + 数据区偏移裸访问。
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ${n} {
  /// HeapObject 头 (map 槽)。
  pub header: HeapObject,
  /// 分配容量 (V8_TQ_CONST uint32_t capacity_)。
  pub capacity: u32,
  /// 已用元素数 (uint32_t length_)。
  pub length: u32,
}

${familyStructImpl(n, `new(map: Tagged<Map>, capacity: u32, length: u32) -> Self {
    Self { header: HeapObject::new(map), capacity, length }
  }`)}`;
  }
  const [field, baseTy] = SUPER_FIELD[f.super];
  return `/// ${n} (fixed-array.h: TaggedArrayBase<${n}, ${f.element}, ${f.super}>)。
///
/// 头继承 ${f.super} (map + length); 元素尾 (objects) 经基址 + 数据区偏移
/// 裸访问; snapshot 数据区锚点 = fixed_array_base::K_HEADER_SIZE。
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ${n} {
  /// ${f.super} 头。
  pub ${field}: ${baseTy},
}

${familyStructImpl(n, `new(map: Tagged<Map>, length: u32) -> Self {
    Self { ${field}: ${baseTy}::new(map, length) }
  }`)}`;
}

function familyImplTaggedArray(f) {
  const n = f.name;
  if (f.super === "HeapObject") {
    return `impl TaggedArray for ${n} {
  type Element = ${f.elementRs};

  const LENGTH_OFFSET: usize = mem::offset_of!(${n}, length);
  const CAPACITY_OFFSET: usize = mem::offset_of!(${n}, capacity);
  const OBJECTS_OFFSET: usize = mem::size_of::<${n}>();
}`;
  }
  const [field] = SUPER_FIELD[f.super];
  return `impl TaggedArray for ${n} {
  type Element = ${f.elementRs};

  const LENGTH_OFFSET: usize = mem::offset_of!(${n}, ${field}.length);
  const CAPACITY_OFFSET: usize = Self::LENGTH_OFFSET; // capacity_field() == length_
  const OBJECTS_OFFSET: usize = mem::size_of::<${n}>();
}`;
}

const familyBlocks = fams.map((f) => `${familyStruct(f)}\n\n${familyImplTaggedArray(f)}`).join("\n\n");
const familyCasts = fams
  .map((f) => {
    const fn = snake(f.name);
    return `impl Cast for ${f.name} {
  #[inline]
  fn allow_from(value: Tagged<Object>) -> bool {
    instance_type_of(value).is_some_and(itc::is_${fn})
  }
}`;
  })
  .join("\n\n");

const body = `${banner(
  "src/objects/fixed-array.h.js",
  "src/objects/fixed-array.h + fixed-array-inl.h + fixed-array.tq",
  "TaggedArrayBase → trait + blanket impl; 9 tagged 族; primitive 族移交 primitive_array.rs; isolate/factory 依赖推迟 (脚本头登记)",
)}
//! FixedArray 族: 定长 tagged 元素数组 (FixedArray / WeakFixedArray /
//! trusted/protected 变体 / ArrayList / WeakArrayList)。
//!
//! 建模 (glossary: CRTP → trait + 泛型; 继承 → 包装 + trait):
//! - [TaggedArray]: 布局差异 (length/capacity 槽位, 数据区起点) → 关联常量;
//! - [TaggedArrayExt]: 公共读写语义 (get/set 4 序 + swap/cas + 容量/尺寸)。
//!
//! 移交登记 (脚本头): PrimitiveArrayBase 族 (ByteArray/TrustedByteArray/
//! FixedDoubleArray/FixedIntegerArray) 归 primitive_array.rs。
//!
//! 布局双轨制 (同 fixed_array_base 模块头): snapshot 偏移常量按压缩默认
//! (K_TAGGED_SIZE=4); 运行时槽位按结构体逻辑布局 (Tagged 槽 usize 宽),
//! offset_of / size_of 承载, Tagged 槽压缩收窄后两者合一。
//!
//! 跳过登记 (脚本头): New/Allocate 族, MoveElements/CopyElements (写屏障簿记),
//! set 内 DCHECK(!IsCowArray()), clear_optional_padding, BodyDescriptor/verifier,
//! roots 表 (kMapRootIndex)。

use std::{
  mem, ptr,
  sync::atomic::{AtomicUsize, Ordering},
};

use v8_common::{K_MAX_REGULAR_HEAP_OBJECT_SIZE, K_TAGGED_SIZE};
use v8_objects_core::{
  heap_object::HeapObject,
  instance_type_checker as itc,
  map::Map,
  objects::{instance_type_of, Cast, WriteBarrierMode},
  smi::Smi,
  tagged::{MaybeObject, Object, Tagged},
  tagged_field::TaggedSlot,
};

use crate::{
  error::{Error, Result},
  fixed_array_base::{u32_slot, write_barrier_stub, FixedArrayBase, TrustedFixedArrayBase, K_MAX_FIXED_ARRAY_CAPACITY},
};

// ── 受保护元素标记类型 (fixed-array.h UnionOf 展开) ────────────────────────
/// UnionOf<TrustedObject, Smi>: ProtectedFixedArray 元素 (受保护指针或 Smi)。
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TrustedObjectOrSmi;

/// UnionOf<MaybeWeak<TrustedObject>, Smi>: ProtectedWeakFixedArray 元素。
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct MaybeWeakTrustedObjectOrSmi;

// ── TaggedArrayBase → trait (fixed-array.h:57-207) ────────────────────────
/// tagged 元素数组布局契约 (TaggedArrayBase&lt;Derived, ElementT, Super&gt;)。
///
/// 各具体族声明槽位/数据区偏移; 公共语义由 TaggedArrayExt blanket 承载。
pub trait TaggedArray: Sized {
  /// 元素标记类型 (ElementT)。
  type Element;

  /// length_ 槽运行时偏移 (Super 承载, 或 capacity+length 族自声明)。
  const LENGTH_OFFSET: usize;

  /// capacity_field() 槽运行时偏移 (HasCapacity → capacity_, 否则同 length)。
  const CAPACITY_OFFSET: usize;

  /// 数据区 (objects 尾) 运行时起始偏移 (= OFFSET_OF_DATA_START(Derived))。
  const OBJECTS_OFFSET: usize;

  /// 单元素字节数 (kElementSize = kTaggedSize; 运行时槽 usize 宽)。
  const ELEMENT_SIZE: usize = mem::size_of::<TaggedSlot>();
}

/// Tagged&lt;T: TaggedArray&gt; 的公共读写语义 (fixed-array-inl.h:37-226)。
///
/// 元素槽读写均为原子 (C++ 普通读写 → Relaxed, 同 core TaggedMember 先例);
/// 写屏障为桩 (L3 heap 接入, 见 fixed_array_base::write_barrier_stub)。
pub trait TaggedArrayExt: Copy {
  /// 元素标记类型透传。
  type Element;

  /// capacity() (fixed-array-inl.h:104)。
  ///
  /// # Safety
  /// 指针须指向有效数组对象内存。
  #[inline]
  unsafe fn capacity(&self) -> u32 {
    // SAFETY: 调用方契约; 槽位恒在对象内
    unsafe { u32_slot::load(self.capacity_addr()) }
  }

  /// capacity(kRelaxedLoad)。
  ///
  /// # Safety
  /// 同 capacity。
  #[inline]
  unsafe fn capacity_relaxed(&self) -> u32 {
    // SAFETY: 调用方契约
    unsafe { u32_slot::load_order(self.capacity_addr(), Ordering::Relaxed) }
  }

  /// capacity(kAcquireLoad)。
  ///
  /// # Safety
  /// 同 capacity。
  #[inline]
  unsafe fn capacity_acquire(&self) -> u32 {
    // SAFETY: 调用方契约
    unsafe { u32_slot::load_order(self.capacity_addr(), Ordering::Acquire) }
  }

  /// set_capacity(value)。
  ///
  /// # Safety
  /// 同 capacity; 无并发别名写。
  #[inline]
  unsafe fn set_capacity(&self, value: u32) {
    // SAFETY: 调用方契约
    unsafe { u32_slot::store(self.capacity_addr(), value) }
  }

  /// set_capacity(value, kReleaseStore)。
  ///
  /// # Safety
  /// 同 capacity。
  #[inline]
  unsafe fn set_capacity_release(&self, value: u32) {
    // SAFETY: 调用方契约
    unsafe { u32_slot::store_order(self.capacity_addr(), value, Ordering::Release) }
  }

  /// length() (继承自 (Trusted)FixedArrayBase / 自声明)。
  ///
  /// # Safety
  /// 同 capacity。
  #[inline]
  unsafe fn length(&self) -> u32 {
    // SAFETY: 调用方契约
    unsafe { u32_slot::load(self.length_addr()) }
  }

  /// ulength()。
  ///
  /// # Safety
  /// 同 capacity。
  #[inline]
  unsafe fn ulength(&self) -> u32 {
    // SAFETY: 同 length 契约
    unsafe { self.length() }
  }

  /// length(kRelaxedLoad)。
  ///
  /// # Safety
  /// 同 capacity。
  #[inline]
  unsafe fn length_relaxed(&self) -> u32 {
    // SAFETY: 调用方契约
    unsafe { u32_slot::load_order(self.length_addr(), Ordering::Relaxed) }
  }

  /// length(kAcquireLoad)。
  ///
  /// # Safety
  /// 同 capacity。
  #[inline]
  unsafe fn length_acquire(&self) -> u32 {
    // SAFETY: 调用方契约
    unsafe { u32_slot::load_order(self.length_addr(), Ordering::Acquire) }
  }

  /// set_length(value)。
  ///
  /// # Safety
  /// 同 capacity; 无并发别名写。
  #[inline]
  unsafe fn set_length(&self, value: u32) {
    // SAFETY: 调用方契约
    unsafe { u32_slot::store(self.length_addr(), value) }
  }

  /// set_length(value, kReleaseStore)。
  ///
  /// # Safety
  /// 同 capacity。
  #[inline]
  unsafe fn set_length_release(&self, value: u32) {
    // SAFETY: 调用方契约
    unsafe { u32_slot::store_order(self.length_addr(), value, Ordering::Release) }
  }

  /// get(index) (无 tag 重载 = Relaxed 读; 原版 TODO jgruber 同源)。
  ///
  /// # Safety
  /// 指针有效且 index &lt; capacity() (debug 断言拦截)。
  #[inline]
  unsafe fn get(&self, index: u32) -> Tagged<Self::Element> {
    // SAFETY: 调用方契约; 越界 debug 拦截
    unsafe { self.get_relaxed(index) }
  }

  /// get(index, kRelaxedLoad)。
  ///
  /// # Safety
  /// 同 get。
  #[inline]
  unsafe fn get_relaxed(&self, index: u32) -> Tagged<Self::Element> {
    debug_assert!(unsafe { self.is_in_bounds(index) });
    // SAFETY: 调用方契约; 槽位原子读
    let v = unsafe { AtomicUsize::from_ptr(self.element_slot(index)) }.load(Ordering::Relaxed);
    Tagged::new(v)
  }

  /// get(index, kAcquireLoad)。
  ///
  /// # Safety
  /// 同 get。
  #[inline]
  unsafe fn get_acquire(&self, index: u32) -> Tagged<Self::Element> {
    debug_assert!(unsafe { self.is_in_bounds(index) });
    // SAFETY: 调用方契约
    let v = unsafe { AtomicUsize::from_ptr(self.element_slot(index)) }.load(Ordering::Acquire);
    Tagged::new(v)
  }

  /// get(index, kSeqCstAccess)。
  ///
  /// # Safety
  /// 同 get。
  #[inline]
  unsafe fn get_seqcst(&self, index: u32) -> Tagged<Self::Element> {
    debug_assert!(unsafe { self.is_in_bounds(index) });
    // SAFETY: 调用方契约
    let v = unsafe { AtomicUsize::from_ptr(self.element_slot(index)) }.load(Ordering::SeqCst);
    Tagged::new(v)
  }

  /// set(index, value, mode) (无 tag 重载 = Relaxed 写;
  /// 原版 DCHECK(!IsCowArray()) 依赖 roots → 调用方契约, 脚本头登记)。
  ///
  /// # Safety
  /// 指针可写有效且 index &lt; capacity(); 非 cow 数组。
  #[inline]
  unsafe fn set(&self, index: u32, value: Tagged<Self::Element>, mode: WriteBarrierMode) {
    // SAFETY: 调用方契约
    unsafe { self.set_relaxed(index, value, mode) };
  }

  /// set(index, value, kRelaxedStore, mode)。
  ///
  /// # Safety
  /// 同 set。
  #[inline]
  unsafe fn set_relaxed(&self, index: u32, value: Tagged<Self::Element>, mode: WriteBarrierMode) {
    debug_assert!(unsafe { self.is_in_bounds(index) });
    // SAFETY: 调用方契约; 槽位原子写
    unsafe { AtomicUsize::from_ptr(self.element_slot(index)) }.store(value.ptr(), Ordering::Relaxed);
    write_barrier_stub(mode);
  }

  /// set(index, value, kReleaseStore, mode)。
  ///
  /// # Safety
  /// 同 set。
  #[inline]
  unsafe fn set_release(&self, index: u32, value: Tagged<Self::Element>, mode: WriteBarrierMode) {
    debug_assert!(unsafe { self.is_in_bounds(index) });
    // SAFETY: 调用方契约
    unsafe { AtomicUsize::from_ptr(self.element_slot(index)) }.store(value.ptr(), Ordering::Release);
    write_barrier_stub(mode);
  }

  /// set(index, value, kSeqCstAccess, mode)。
  ///
  /// # Safety
  /// 同 set。
  #[inline]
  unsafe fn set_seqcst(&self, index: u32, value: Tagged<Self::Element>, mode: WriteBarrierMode) {
    debug_assert!(unsafe { self.is_in_bounds(index) });
    // SAFETY: 调用方契约
    unsafe { AtomicUsize::from_ptr(self.element_slot(index)) }.store(value.ptr(), Ordering::SeqCst);
    write_barrier_stub(mode);
  }

  /// set(index, Smi) (SKIP_WRITE_BARRIER; kSupportsSmiElements 展开)。
  ///
  /// # Safety
  /// 同 set。
  #[inline]
  unsafe fn set_smi(&self, index: u32, value: Tagged<Smi>) {
    // SAFETY: 调用方契约; Smi 免屏障
    unsafe { self.set_relaxed(index, value.cast(), WriteBarrierMode::SkipWriteBarrier) };
  }

  /// set(index, Smi, kRelaxedStore)。
  ///
  /// # Safety
  /// 同 set。
  #[inline]
  unsafe fn set_smi_relaxed(&self, index: u32, value: Tagged<Smi>) {
    // SAFETY: 调用方契约
    unsafe { self.set_relaxed(index, value.cast(), WriteBarrierMode::SkipWriteBarrier) };
  }

  /// set(index, Smi, kReleaseStore)。
  ///
  /// # Safety
  /// 同 set。
  #[inline]
  unsafe fn set_smi_release(&self, index: u32, value: Tagged<Smi>) {
    // SAFETY: 调用方契约
    unsafe { self.set_release(index, value.cast(), WriteBarrierMode::SkipWriteBarrier) };
  }

  /// set(index, Smi, kSeqCstAccess)。
  ///
  /// # Safety
  /// 同 set。
  #[inline]
  unsafe fn set_smi_seqcst(&self, index: u32, value: Tagged<Smi>) {
    // SAFETY: 调用方契约
    unsafe { self.set_seqcst(index, value.cast(), WriteBarrierMode::SkipWriteBarrier) };
  }

  /// swap(index, value, kSeqCstAccess, mode) → 旧值。
  ///
  /// # Safety
  /// 同 set。
  #[inline]
  unsafe fn swap(&self, index: u32, value: Tagged<Self::Element>, mode: WriteBarrierMode) -> Tagged<Self::Element> {
    debug_assert!(unsafe { self.is_in_bounds(index) });
    // SAFETY: 调用方契约
    let old = unsafe { AtomicUsize::from_ptr(self.element_slot(index)) }.swap(value.ptr(), Ordering::SeqCst);
    write_barrier_stub(mode);
    Tagged::new(old)
  }

  /// compare_and_swap(index, expected, value, kSeqCstAccess, mode) → 槽位旧值。
  ///
  /// # Safety
  /// 同 set。
  #[inline]
  unsafe fn compare_and_swap(
    &self,
    index: u32,
    expected: Tagged<Self::Element>,
    value: Tagged<Self::Element>,
    mode: WriteBarrierMode,
  ) -> Tagged<Self::Element> {
    debug_assert!(unsafe { self.is_in_bounds(index) });
    // SAFETY: 调用方契约
    let r = unsafe { AtomicUsize::from_ptr(self.element_slot(index)) }.compare_exchange(
      expected.ptr(),
      value.ptr(),
      Ordering::SeqCst,
      Ordering::SeqCst,
    );
    let old = match r {
      Ok(v) | Err(v) => v,
    };
    if old == expected.ptr() {
      write_barrier_stub(mode);
    }
    Tagged::new(old)
  }

  /// IsInBounds(index) (index &lt; capacity)。
  ///
  /// # Safety
  /// 同 capacity。
  #[inline]
  unsafe fn is_in_bounds(&self, index: u32) -> bool {
    // SAFETY: 调用方契约 (capacity 槽在读)
    unsafe { index < self.capacity() }
  }

  /// SizeFor(capacity) (= OBJECTS_OFFSET + capacity * kElementSize)。
  #[must_use]
  fn size_for(capacity: u32) -> usize;

  /// OffsetOfElementAt(index) (= SizeFor(index))。
  #[must_use]
  fn offset_of_element_at(index: u32) -> usize;

  /// MaxRegularCapacity() (常规页内最大容量)。
  #[must_use]
  fn max_regular_capacity() -> usize;

  /// kMaxCapacity (= kMaxFixedArrayCapacity)。
  const MAX_CAPACITY: u32;

  /// AllocatedSize() (acquire 容量 → SizeFor; 右裁后并发访客对齐)。
  ///
  /// # Safety
  /// 同 capacity。
  #[inline]
  unsafe fn allocated_size(&self) -> usize {
    // SAFETY: 调用方契约
    Self::size_for(unsafe { self.capacity_acquire() })
  }

  /// NewCapacityForIndex (fixed-array-inl.h:351-361, 1.5x+16 增长至覆盖 index)。
  #[inline]
  #[must_use]
  fn new_capacity_for_index(index: u32, old_capacity: u32) -> u32 {
    debug_assert!(index >= old_capacity);
    let mut capacity = old_capacity;
    // 原 do-while: 至少增长一轮
    loop {
      capacity = capacity + (capacity >> 1) + 16;
      if capacity > index {
        return capacity;
      }
    }
  }

  /// IsCowArray (map 比较; roots 由调用方注入, 脚本头登记)。
  ///
  /// # Safety
  /// 指针须指向有效对象 (读 map 槽)。
  #[inline]
  unsafe fn is_cow_array(&self, fixed_cow_array_map: Tagged<Map>) -> bool {
    // SAFETY: 调用方契约; 只读 map 首槽
    unsafe { self.as_heap_object().map() == fixed_cow_array_map }
  }

  /// RightTrim 不变式校验 + heap 层登记 (fixed-array-inl.h:198-206)。
  ///
  /// 不变式: 0 &lt; new_capacity &lt;= capacity; 实际裁剪 (filler 发布 +
  /// release 长度) 归 Heap::RightTrimArray (L3 heap)。
  fn right_trim(&self, new_capacity: u32) -> Result<()> {
    // SAFETY: 调用方契约 (对象有效)
    let old = unsafe { self.capacity() };
    if new_capacity == 0 {
      return Err(Error::Invariant("RightTrim: new_capacity > 0 (可能规范化)"));
    }
    if new_capacity > old {
      return Err(Error::Invariant("RightTrim: new_capacity <= capacity"));
    }
    if new_capacity == old {
      return Ok(());
    }
    Err(Error::Unsupported("RightTrim: Heap::RightTrimArray 归 L3 heap"))
  }

  /// RawFieldOfFirstElement → 数据区首槽。
  ///
  /// # Safety
  /// 指针须指向有效数组对象。
  #[inline]
  unsafe fn raw_field_of_first_element(&self) -> *mut TaggedSlot {
    // SAFETY: 调用方契约
    unsafe { self.raw_field_of_element_at(0) }
  }

  /// RawFieldOfElementAt(index) → 元素槽。
  ///
  /// # Safety
  /// 同 raw_field_of_first_element; index &lt; capacity。
  unsafe fn raw_field_of_element_at(&self, index: u32) -> *mut TaggedSlot;

  /// MemsetTagged 语义: [from, to) 填充 value (写屏障一次结算)。
  ///
  /// # Safety
  /// 同 set; to &lt;= capacity。
  #[inline]
  unsafe fn fill(&self, from: u32, to: u32, value: Tagged<Self::Element>, mode: WriteBarrierMode) {
    // SAFETY: 调用方契约
    unsafe {
      for i in from..to {
        debug_assert!(self.is_in_bounds(i));
        AtomicUsize::from_ptr(self.element_slot(i)).store(value.ptr(), Ordering::Relaxed);
      }
    }
    write_barrier_stub(mode);
  }

  /// 切片复制 (允许内存重叠)
  ///
  /// # Safety
  /// dst 与 src 必须有效，范围不越界。
  #[inline]
  unsafe fn copy_elements(
    dst: &Self,
    dst_index: u32,
    src: &Self,
    src_index: u32,
    len: u32,
    mode: WriteBarrierMode,
  ) {
    if len == 0 {
      return;
    }
    unsafe {
      let dst_ptr = dst.element_slot(dst_index);
      let src_ptr = src.element_slot(src_index) as *const usize;
      ptr::copy(src_ptr, dst_ptr, len as usize);
    }
    write_barrier_stub(mode);
  }

  /// 带边界检查的安全读取
  #[inline]
  fn get_checked(&self, index: usize) -> Result<Tagged<Self::Element>> {
    let cap = unsafe { self.capacity() } as usize;
    if index >= cap {
      return Err(Error::OutOfBounds { index, length: cap });
    }
    unsafe { Ok(self.get(index as u32)) }
  }

  /// 带边界检查的安全写入
  #[inline]
  fn set_checked(&self, index: usize, value: Tagged<Self::Element>) -> Result<()> {
    let cap = unsafe { self.capacity() } as usize;
    if index >= cap {
      return Err(Error::OutOfBounds { index, length: cap });
    }
    unsafe { self.set(index as u32, value, WriteBarrierMode::UpdateWriteBarrier) };
    Ok(())
  }

  // ── 内部: 槽位寻址 (实现侧提供) ────────────────────────────────────────

  /// length 槽地址。
  ///
  /// # Safety
  /// 指针须指向有效对象。
  unsafe fn length_addr(&self) -> usize;

  /// capacity 槽地址。
  ///
  /// # Safety
  /// 同 length_addr。
  unsafe fn capacity_addr(&self) -> usize;

  /// 元素槽指针。
  ///
  /// # Safety
  /// 同 length_addr; index &lt; capacity。
  unsafe fn element_slot(&self, index: u32) -> *mut TaggedSlot;

  /// 转 HeapObject tagged 视图。
  fn as_heap_object(&self) -> Tagged<HeapObject>;
}

impl<T: TaggedArray> TaggedArrayExt for Tagged<T> {
  type Element = T::Element;

  #[inline]
  fn size_for(capacity: u32) -> usize {
    T::OBJECTS_OFFSET + capacity as usize * T::ELEMENT_SIZE
  }

  #[inline]
  fn offset_of_element_at(index: u32) -> usize {
    Self::size_for(index)
  }

  #[inline]
  fn max_regular_capacity() -> usize {
    (K_MAX_REGULAR_HEAP_OBJECT_SIZE - T::OBJECTS_OFFSET) / T::ELEMENT_SIZE
  }

  const MAX_CAPACITY: u32 = K_MAX_FIXED_ARRAY_CAPACITY;

  #[inline]
  unsafe fn length_addr(&self) -> usize {
    self.address() + T::LENGTH_OFFSET
  }

  #[inline]
  unsafe fn capacity_addr(&self) -> usize {
    self.address() + T::CAPACITY_OFFSET
  }

  #[inline]
  unsafe fn element_slot(&self, index: u32) -> *mut TaggedSlot {
    (self.address() + T::OBJECTS_OFFSET + index as usize * T::ELEMENT_SIZE) as *mut TaggedSlot
  }

  #[inline]
  unsafe fn raw_field_of_element_at(&self, index: u32) -> *mut TaggedSlot {
    // SAFETY: 调用方契约; 元素槽即裸槽位
    unsafe { self.element_slot(index) }
  }

  #[inline]
  fn as_heap_object(&self) -> Tagged<HeapObject> {
    self.cast()
  }
}

// ══ 具体族 (9 个 TaggedArrayBase 族, 机械展开) ═════════════════════════════

${familyBlocks}

// ── capacity+length 族 snapshot 布局 (fixed-array.h:646-648, 725-727) ──────
/// WeakArrayList::kCapacityOffset (= sizeof(HeapObject))。
pub const K_WEAK_ARRAY_LIST_CAPACITY_OFFSET: usize = K_TAGGED_SIZE;
/// WeakArrayList::kLengthOffset。
pub const K_WEAK_ARRAY_LIST_LENGTH_OFFSET: usize = K_WEAK_ARRAY_LIST_CAPACITY_OFFSET + 4;
/// WeakArrayList::kHeaderSize。
pub const K_WEAK_ARRAY_LIST_HEADER_SIZE: usize = K_WEAK_ARRAY_LIST_LENGTH_OFFSET + 4;
/// ArrayList::kCapacityOffset。
pub const K_ARRAY_LIST_CAPACITY_OFFSET: usize = K_TAGGED_SIZE;
/// ArrayList::kLengthOffset。
pub const K_ARRAY_LIST_LENGTH_OFFSET: usize = K_ARRAY_LIST_CAPACITY_OFFSET + 4;
/// ArrayList::kHeaderSize。
pub const K_ARRAY_LIST_HEADER_SIZE: usize = K_ARRAY_LIST_LENGTH_OFFSET + 4;

const _: () = assert!(K_WEAK_ARRAY_LIST_HEADER_SIZE == K_ARRAY_LIST_HEADER_SIZE);

/// FixedArrayExact (fixed-array.h:411): FixedArray 精确类型别名。
///
/// 仅 IsFixedArrayExact 谓词 (= FIXED_ARRAY_TYPE 精确匹配, 非区间) 需要;
/// Rust 侧与 FixedArray 共享类型与 Cast 区间谓词, 精确谓词独立成函数
/// (类型层合并登记)。
pub type FixedArrayExact = FixedArray;

/// IsFixedArrayExact 谓词 (== FIXED_ARRAY_TYPE 精确匹配)。
#[inline]
#[must_use]
pub fn is_fixed_array_exact(value: Tagged<Object>) -> bool {
  instance_type_of(value).is_some_and(itc::is_fixed_array_exact)
}

// ── Cast (casting.h DEF_CAST_TRAITS 对应; 覆盖 CAST_TRAITS_TYPES 核对项:
//    FixedArrayBase / FixedArrayExact / WeakHomomorphicFixedArray) ──────────
impl Cast for FixedArrayBase {
  #[inline]
  fn allow_from(value: Tagged<Object>) -> bool {
    instance_type_of(value).is_some_and(itc::is_fixed_array_base)
  }
}

impl Cast for TrustedFixedArrayBase {
  #[inline]
  fn allow_from(value: Tagged<Object>) -> bool {
    instance_type_of(value).is_some_and(itc::is_trusted_fixed_array_base)
  }
}

${familyCasts}

// ══ 族专属语义 (fixed-array-inl.h) ═══════════

/// Tagged&lt;FixedArray&gt; 专属语义 (fixed-array-inl.h:379-421)。
///
/// hole 槽值由调用方注入 (ReadOnlyRoots 归 L3 heap, 脚本头登记)。
pub trait FixedArrayExt: Copy + TaggedArrayExt<Element = Object> {
  /// is_the_hole(index) (与 hole 槽值位比较)。
  ///
  /// # Safety
  /// 同 TaggedArrayExt::get。
  #[inline]
  unsafe fn is_the_hole(&self, hole: Tagged<Object>, index: u32) -> bool {
    // SAFETY: 调用方契约
    unsafe { self.get(index) == hole }
  }

  /// set_the_hole(index) (SKIP 屏障写 hole 槽值)。
  ///
  /// # Safety
  /// 同 TaggedArrayExt::set。
  #[inline]
  unsafe fn set_the_hole(&self, hole: Tagged<Object>, index: u32) {
    // SAFETY: 调用方契约; 只读根免屏障
    unsafe { self.set_relaxed(index, hole, WriteBarrierMode::SkipWriteBarrier) };
  }

  /// FillWithHoles(from, to)。
  ///
  /// # Safety
  /// 同 TaggedArrayExt::fill。
  #[inline]
  unsafe fn fill_with_holes(&self, hole: Tagged<Object>, from: u32, to: u32) {
    // SAFETY: 调用方契约
    unsafe { self.fill(from, to, hole, WriteBarrierMode::SkipWriteBarrier) };
  }
}

impl FixedArrayExt for Tagged<FixedArray> {}

/// Tagged&lt;WeakArrayList&gt; 专属语义 (fixed-array.h:558-679 + inl:499-529)。
///
/// Get/Set 与 TaggedArrayExt 的 get/set 同签名同语义 (Relaxed), 不重复
/// 承载 (调用歧义规避); Set 的数组须足够大, 需扩容走 heap 层 AddToEnd (登记)。
pub trait WeakArrayListExt: Copy + TaggedArrayExt<Element = MaybeObject> {
  /// Set(index, Smi) (SKIP 屏障; inl:508)。
  ///
  /// # Safety
  /// 同 TaggedArrayExt::set。
  #[inline]
  unsafe fn set_smi(&self, index: u32, value: Tagged<Smi>) {
    // SAFETY: 调用方契约
    unsafe { TaggedArrayExt::set_relaxed(self, index, value.cast(), WriteBarrierMode::SkipWriteBarrier) };
  }

  /// SizeForCapacity(capacity) (= SizeFor)。
  #[inline]
  #[must_use]
  fn size_for_capacity(capacity: u32) -> usize {
    <Tagged<WeakArrayList> as TaggedArrayExt>::size_for(capacity)
  }

  /// CapacityForLength(length) (length + max(length/2, 2); inl 同源 h:600)。
  #[inline]
  #[must_use]
  fn capacity_for_length(length: u32) -> u32 {
    length + (if length / 2 > 2 { length / 2 } else { 2 })
  }

  /// data_start() (MaybeObjectSlot → 槽位指针)。
  ///
  /// # Safety
  /// 指针须指向有效数组对象。
  #[inline]
  unsafe fn data_start(&self) -> *mut TaggedSlot {
    // SAFETY: 调用方契约
    unsafe { self.raw_field_of_first_element() }
  }

  /// 弱堆对象迭代器 (Iterator::Next 语义: 跳过已清除弱引用)。
  ///
  /// # Safety
  /// 迭代期间对象须有效存活 (tagged 裸引用, GC 禁止窗口由调用方保证)。
  unsafe fn iter_weak_heap_objects(self) -> WeakArrayListIter;
}

impl WeakArrayListExt for Tagged<WeakArrayList> {
  #[inline]
  unsafe fn iter_weak_heap_objects(self) -> WeakArrayListIter {
    WeakArrayListIter { array: self, index: 0 }
  }
}

/// WeakArrayList::Iterator (fixed-array.h:667-679)。
///
/// 逐个产出未清除的弱引用堆对象; 交错 Smi 元素原版 DCHECK 拦截 →
/// 此处跳过弱谓词不满足项 (登记: 断言语义弱化为跳过)。
pub struct WeakArrayListIter {
  array: Tagged<WeakArrayList>,
  index: u32,
}

impl Iterator for WeakArrayListIter {
  type Item = Tagged<HeapObject>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.array.is_null() {
      return None;
    }
    // SAFETY: 构造方契约 (iter_weak_heap_objects): 迭代期间对象有效
    let len = unsafe { TaggedArrayExt::length(&self.array) };
    while self.index < len {
      // SAFETY: 同上
      let item = unsafe { TaggedArrayExt::get(&self.array, self.index) };
      self.index += 1;
      debug_assert!(item.is_weak_or_cleared() || item.is_smi());
      if let Some(obj) = item.get_heap_object_if_weak() {
        return Some(obj);
      }
    }
    self.array = Tagged::NULL;
    None
  }
}

// == fixed-array.cc 语义区段 (由 tool/src/objects/fixed-array.cc.js 维护) ==
// @cc-semantics-begin
// @cc-semantics-end
`;

writeSrc("v8-objects-arrays", "fixed_array.rs", body);
console.log(
  `fixed_array.rs: TaggedArrayBase trait + TaggedArrayExt + ` +
    `具体族 ${fams.length} (tagged); primitive 族移交 primitive_array.rs; ` +
    "isolate/factory/屏障簿记推迟已登记",
);
