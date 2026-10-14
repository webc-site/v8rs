#!/usr/bin/env bun
// tool/src/objects/fixed-array-base.h.js — 覆盖 fixed-array-base 同名族骨架:
//   v8/src/objects/fixed-array-base.h + fixed-array-base-inl.h + fixed-array-base.tq
//   → v8-objects-arrays/src/fixed_array_base.rs
//
// 映射登记 (C++ → Rust):
//   kMaxFixedArrayCapacity (V8_LOWER_LIMITS_MODE 默认关 → 128M 分支) →
//     K_MAX_FIXED_ARRAY_CAPACITY
//   FixedArrayBase (HeapObject + length_ [+ optional_padding_]) →
//     struct FixedArrayBase { header, length } + snapshot 偏移常量
//   TrustedFixedArrayBase (TrustedObject + length_) → 同构 struct
//     (sizeof(TrustedObject) == sizeof(HeapObject) 对齐断言)
//   length()/ulength()/length(Acquire/Relaxed)/set_length(+Release) →
//     Tagged<..> 扩展 trait (glossary: 继承 → 包装 + trait; C++ 普通读写 →
//     Relaxed 原子读写, 同 v8-objects-core tagged_field 先例)
//   SafeHeapObjectSize (StrongAlias<uint32_t>) → u32 直返
//     (Rust 类型系统天然挡隐式收窄, 新类型层省略)
//
// 跳过区段登记 (推迟):
//   - FixedArrayBase::IsCowArray / TaggedArrayBase::IsCowArray 的
//     ReadOnlyHeap::EarlyGetReadOnlyRoots 依赖 → 以 cow map 参数注入版承载
//     (roots 表归 L3 heap/roots)
//   - GetMaxLengthForNewSpaceAllocation 的 ElementsKindToShiftSize 依赖 →
//     以 shift_size 参数注入版承载 (elements-kind.h 归 elements crate)
//   - DECL_VERIFIER / DECL_PRINTER 族 → heap 验证器层
import { spawnSync } from "node:child_process";
import { join } from "node:path";
import { banner, readV8, writeSrc } from "../../lib.js";

// 仓库 rustfmt 归一 (rustfmt.toml: imports_granularity=Crate / tab_spaces=2);
// 生成即 CI fmt 形态 → 复跑幂等且不与就地修订的 fmt 形态冲突; 解析失败即 throw。
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

const h = readV8("src/objects/fixed-array-base.h");
const inl = readV8("src/objects/fixed-array-base-inl.h");
const tq = readV8("src/objects/fixed-array-base.tq");

// ── 对数断言 (原版锚点 + 机械提取 + 产出对数; 任一不符 throw) ───────────
let assertCount = 0;
const fail = (label, got, want) => {
  throw new Error(`对数失配: ${label} got=${JSON.stringify(got)} want=${JSON.stringify(want)}`);
};
const count = (src, re) => (src.match(re) || []).length;

// 1) 文本锚点在位
for (const [sub, file] of [
  ["static constexpr uint32_t kMaxFixedArrayCapacity =", h],
  ["V8_OBJECT class FixedArrayBase : public HeapObject {", h],
  ["V8_OBJECT class TrustedFixedArrayBase : public TrustedObject {", h],
  ["inline SafeHeapObjectSize FixedArrayBase::length() const {", inl],
  ["inline SafeHeapObjectSize TrustedFixedArrayBase::length() const {", inl],
  ["class FixedArrayBase extends HeapObject", tq],
  ["class TrustedFixedArrayBase extends TrustedObject", tq],
]) {
  if (!file.includes(sub)) throw new Error(`锚点失配: 无 "${sub.slice(0, 48)}..."`);
  assertCount++;
}

// 2) kMaxFixedArrayCapacity 机械提取 (V8_LOWER_LIMITS_MODE_BOOL 默认 false 分支)
const capm = h.match(
  /kMaxFixedArrayCapacity =\s*V8_LOWER_LIMITS_MODE_BOOL \? \(([^)]+)\) : \(([^)]+)\);/,
);
if (!capm) throw new Error("kMaxFixedArrayCapacity 展开失配");
assertCount++;
const MAX_CAP = capm[2].replace(/[^0-9* ]/g, "").trim(); // "128 * 1024 * 1024"
const prod = (s) => s.split("*").reduce((a, b) => a * Number(b), 1);
if (prod(MAX_CAP) !== 128 * 1024 * 1024)
  throw new Error(`kMaxFixedArrayCapacity 默认分支值漂移: ${MAX_CAP}`);
assertCount++;

// 3) 原版字段/布局声明对数 (两基类同构)
for (const [got, want, label] of [
  [count(h, /V8_TQ_CONST uint32_t length_;/g), 2, "C++ length_ 字段"],
  [count(h, /uint32_t optional_padding_;/g), 2, "C++ optional_padding_ 字段"],
  [count(h, /static constexpr int kLengthOffset = sizeof\((HeapObject|TrustedObject)\);/g), 2, "kLengthOffset 公式"],
  [count(h, /static constexpr uint32_t kHeaderSize = kLengthOffset \+ kUInt32Size;/g), 2, "非 8 字节 kHeaderSize"],
  [count(h, /static constexpr int kMaxRegularLength =/g), 1, "kMaxRegularLength"],
  [count(h, /static constexpr uint32_t kMaxLength = kMaxFixedArrayCapacity;/g), 1, "kMaxLength 别名"],
  [count(h, /#if TAGGED_SIZE_8_BYTES/g), 4, "padding 条件分支"],
  [count(h, /DECL_VERIFIER\(FixedArrayBase\)/g), 1, "DECL_VERIFIER (跳过区段)"],
  [count(inl, /ReadOnlyHeap::EarlyGetReadOnlyRoots/g), 1, "IsCowArray roots 依赖 (参数注入)"],
  [count(inl, /ElementsKindToShiftSize\(kind\)/g), 1, "GetMaxLength ElementsKind 依赖 (参数注入)"],
  [count(tq, /const length: uint32;/g), 2, "tq length 字段"],
  [count(tq, /@if\(TAGGED_SIZE_8_BYTES\) optional_padding: uint32;/g), 2, "tq optional_padding"],
  [count(tq, /extern class (?:FixedArrayBase extends HeapObject|TrustedFixedArrayBase extends TrustedObject)/g), 2, "tq 基类声明"],
  [count(tq, /protected length_:/g), 0, "tq 字段 protected 漂移"],
  [/\bclass FixedArrayBase extends HeapObject \{[\s\S]*?\n\}/.test(tq), true, "FixedArrayBase tq 块"],
  [/\bclass TrustedFixedArrayBase extends TrustedObject \{[\s\S]*?\n\}/.test(tq), true, "TrustedFixedArrayBase tq 块"],
]) {
  if (got !== want) fail(label, got, want);
  assertCount++;
}

// 4) 原版 accessor 定义对数 (与生成侧对数在 body 产出后比对)
const ORIG_ACCESSORS = {
  length: count(inl, /inline SafeHeapObjectSize (?:FixedArrayBase|TrustedFixedArrayBase)::length\(\) const \{/g),
  ulength: count(inl, /inline SafeHeapObjectSize (?:FixedArrayBase|TrustedFixedArrayBase)::ulength\(\) const/g),
  acquire: count(inl, /base::AsAtomic32::Acquire_Load\(&length_\)/g),
  relaxed: count(inl, /base::AsAtomic32::Relaxed_Load\(&length_\)/g),
  plain_write: count(inl, /length_ = value;/g),
  release: count(inl, /base::AsAtomic32::Release_Store\(&length_, value\)/g),
};
for (const [k, v] of Object.entries(ORIG_ACCESSORS)) {
  if (v !== 2) fail(`原版 accessor ${k}`, v, 2);
  assertCount++;
}


const body = `${banner(
  "src/objects/fixed-array-base.h.js",
  "src/objects/fixed-array-base.h + fixed-array-base-inl.h + fixed-array-base.tq",
  "FixedArrayBase/TrustedFixedArrayBase 头布局 + length 语义; cow/ElementsKind 依赖参数注入 (脚本头登记)",
)}
//! FixedArray 族公共头: FixedArrayBase / TrustedFixedArrayBase。
//!
//! 布局双轨制 (同 v8-objects-strings 先例):
//! - snapshot 偏移常量 (K_*): 压缩指针默认配置 (K_TAGGED_SIZE=4) 的堆布局,
//!   snapshot 兼容锚点;
//! - 运行时访问: Rust 结构体逻辑布局 (Tagged 槽 usize 宽), length/capacity 槽
//!   经结构体字段偏移读写, Tagged 槽压缩收窄后两者合一。
//!
//! 跳过登记 (脚本头): IsCowArray roots 依赖 / GetMaxLengthForNewSpaceAllocation
//! ElementsKind 依赖 / DECL_VERIFIER 族。

use std::{
  mem::{offset_of, size_of},
  sync::atomic::Ordering,
};

use v8_common::{K_MAX_REGULAR_HEAP_OBJECT_SIZE, K_TAGGED_SIZE, TAGGED_SIZE_8_BYTES};
use v8_objects_core::{
  heap_object::HeapObject,
  map::Map,
  objects::WriteBarrierMode,
  tagged::Tagged,
};

use crate::trusted::TrustedObject;

/// 全 fixed array 族统一容量上限 (fixed-array-base.h:22)。
///
/// 统一上限保证 非定容 elements kind 迁移 (如 Smi → Double) 不因容量上限差异失败;
/// 再大一档 (2 的幂) 会把 FixedDoubleArray 最大字节尺寸推出 int32 范围。
pub const K_MAX_FIXED_ARRAY_CAPACITY: u32 = ${MAX_CAP};

// ── 布局常量 (snapshot; kLengthOffset = sizeof(HeapObject/TrustedObject)) ──
/// length_ 字段偏移 (两基类同值: sizeof(TrustedObject) == sizeof(HeapObject))。
pub const K_LENGTH_OFFSET: usize = K_TAGGED_SIZE;
/// optional_padding_ 偏移 (仅 TAGGED_SIZE_8_BYTES 配置存在)。
pub const K_PADDING_OFFSET: usize = K_LENGTH_OFFSET + 4;
/// 头大小 (8 字节 tagged 配置含 4 字节 padding)。
pub const K_HEADER_SIZE: usize = if TAGGED_SIZE_8_BYTES { K_PADDING_OFFSET + 4 } else { K_LENGTH_OFFSET + 4 };
/// 容量上限 (== kMaxFixedArrayCapacity)。
pub const K_MAX_LENGTH: u32 = K_MAX_FIXED_ARRAY_CAPACITY;
/// 常规页 (非大对象空间) 内的最大槽数。
pub const K_MAX_REGULAR_LENGTH: usize = (K_MAX_REGULAR_HEAP_OBJECT_SIZE - K_HEADER_SIZE) / K_TAGGED_SIZE;

// 编译期布局对数 (压缩默认锚点; 原版 static_assert(sizeof(FixedArray) == kHeaderSize))
const _: () = assert!(K_HEADER_SIZE == K_TAGGED_SIZE + 4);
const _: () = assert!(!TAGGED_SIZE_8_BYTES && K_HEADER_SIZE == 8);

/// FixedArray 族公共基类头: HeapObject + length_ (fixed-array-base.h:29)。
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct FixedArrayBase {
  /// HeapObject 头 (map 槽)。
  pub header: HeapObject,
  /// 元素数 (V8_TQ_CONST uint32_t length_)。
  pub length: u32,
}

impl FixedArrayBase {
  /// 构造头 (snapshot 布局下 8 字节 tagged 配置另有 optional_padding_)。
  #[inline]
  pub const fn new(map: Tagged<Map>, length: u32) -> Self {
    Self { header: HeapObject::new(map), length }
  }
}

/// Trusted 空间镜像基类头: TrustedObject + length_ (fixed-array-base.h:67)。
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TrustedFixedArrayBase {
  /// TrustedObject 头 (无额外字段, 布局与 HeapObject 同)。
  pub header: TrustedObject,
  /// 元素数。
  pub length: u32,
}

impl TrustedFixedArrayBase {
  /// 构造头。
  #[inline]
  pub const fn new(map: Tagged<Map>, length: u32) -> Self {
    Self { header: TrustedObject::new(map), length }
  }
}

// 运行时 length 槽偏移对数: 两基类字段布局一致 (sizeof(TrustedObject)==sizeof(HeapObject))
const _: () = assert!(size_of::<TrustedFixedArrayBase>() == size_of::<FixedArrayBase>());

/// 写屏障桩 (heap 层 WriteBarrier::ForValue 落地前为 no-op)。
///
/// TODO(L3/heap): 接真实 remembered-set 记录; trusted space 需
/// ForProtectedPointer 专用路径 (TRUSTED_TO_SHARED_TRUSTED 条目)。
#[inline]
pub(crate) fn write_barrier_stub(_mode: WriteBarrierMode) {}

/// u32 槽读写助手 (宿主地址 + 偏移; C++ 普通读写 → Relaxed 原子)。
pub(crate) mod u32_slot {
  use std::sync::atomic::{AtomicU32, Ordering};

  /// # Safety
  /// addr 须指向对象内有效 4 字节对齐 u32 槽。
  #[inline]
  pub unsafe fn load(addr: usize) -> u32 {
    // SAFETY: 调用方契约
    unsafe { AtomicU32::from_ptr(addr as *mut u32) }.load(Ordering::Relaxed)
  }

  /// # Safety
  /// 同 [load]。
  #[inline]
  pub unsafe fn load_order(addr: usize, order: Ordering) -> u32 {
    // SAFETY: 调用方契约
    unsafe { AtomicU32::from_ptr(addr as *mut u32) }.load(order)
  }

  /// # Safety
  /// 同 [load]; 无并发别名写。
  #[inline]
  pub unsafe fn store(addr: usize, value: u32) {
    // SAFETY: 调用方契约
    unsafe { AtomicU32::from_ptr(addr as *mut u32) }.store(value, Ordering::Relaxed);
  }

  /// # Safety
  /// 同 [load]。
  #[inline]
  pub unsafe fn store_order(addr: usize, value: u32, order: Ordering) {
    // SAFETY: 调用方契约
    unsafe { AtomicU32::from_ptr(addr as *mut u32) }.store(value, order);
  }
}

/// Tagged<FixedArrayBase> 成员语义 (fixed-array-base-inl.h:19-37)。
pub trait FixedArrayBaseExt: Copy {
  /// length() (SafeHeapObjectSize(length_))。
  ///
  /// # Safety
  /// 指针须指向有效 FixedArrayBase 内存。
  #[inline]
  unsafe fn length(&self) -> u32 {
    // SAFETY: 调用方契约; 槽位恒在对象内
    unsafe { u32_slot::load(self.slot_addr()) }
  }

  /// ulength() (无符号视图, 与 length 同)。
  ///
  /// # Safety
  /// 同 [Self::length]。
  #[inline]
  unsafe fn ulength(&self) -> u32 {
    // SAFETY: 同 length 契约
    unsafe { self.length() }
  }

  /// length(kAcquireLoad)。
  ///
  /// # Safety
  /// 同 [Self::length]。
  #[inline]
  unsafe fn length_acquire(&self) -> u32 {
    // SAFETY: 调用方契约
    unsafe { u32_slot::load_order(self.slot_addr(), Ordering::Acquire) }
  }

  /// length(kRelaxedLoad)。
  ///
  /// # Safety
  /// 同 [Self::length]。
  #[inline]
  unsafe fn length_relaxed(&self) -> u32 {
    // SAFETY: 调用方契约
    unsafe { u32_slot::load_order(self.slot_addr(), Ordering::Relaxed) }
  }

  /// set_length(value)。
  ///
  /// # Safety
  /// 同 [Self::length]; 无并发别名。
  #[inline]
  unsafe fn set_length(&self, value: u32) {
    // SAFETY: 调用方契约
    unsafe { u32_slot::store(self.slot_addr(), value) }
  }

  /// set_length(value, kReleaseStore)。
  ///
  /// # Safety
  /// 同 [Self::length]。
  #[inline]
  unsafe fn set_length_release(&self, value: u32) {
    // SAFETY: 调用方契约
    unsafe { u32_slot::store_order(self.slot_addr(), value, Ordering::Release) }
  }

  /// length 槽地址。
  ///
  /// # Safety
  /// 同 [Self::length]。
  unsafe fn slot_addr(&self) -> usize;
}

impl FixedArrayBaseExt for Tagged<FixedArrayBase> {
  #[inline]
  unsafe fn slot_addr(&self) -> usize {
    // 运行时槽位 = 结构体字段偏移 (Tagged usize 宽); snapshot 锚点 K_LENGTH_OFFSET
    self.address() + offset_of!(FixedArrayBase, length)
  }
}

/// Tagged<TrustedFixedArrayBase> 成员语义 (fixed-array-base-inl.h:39-64)。
pub trait TrustedFixedArrayBaseExt: Copy {
  /// length()。
  ///
  /// # Safety
  /// 指针须指向有效 TrustedFixedArrayBase 内存。
  #[inline]
  unsafe fn length(&self) -> u32 {
    // SAFETY: 调用方契约; 槽位恒在对象内
    unsafe { u32_slot::load(self.slot_addr()) }
  }

  /// ulength()。
  ///
  /// # Safety
  /// 同 [Self::length]。
  #[inline]
  unsafe fn ulength(&self) -> u32 {
    // SAFETY: 同 length 契约
    unsafe { self.length() }
  }

  /// length(kAcquireLoad)。
  ///
  /// # Safety
  /// 同 [Self::length]。
  #[inline]
  unsafe fn length_acquire(&self) -> u32 {
    // SAFETY: 调用方契约
    unsafe { u32_slot::load_order(self.slot_addr(), Ordering::Acquire) }
  }

  /// length(kRelaxedLoad)。
  ///
  /// # Safety
  /// 同 [Self::length]。
  #[inline]
  unsafe fn length_relaxed(&self) -> u32 {
    // SAFETY: 调用方契约
    unsafe { u32_slot::load_order(self.slot_addr(), Ordering::Relaxed) }
  }

  /// set_length(value)。
  ///
  /// # Safety
  /// 同 [Self::length]; 无并发别名。
  #[inline]
  unsafe fn set_length(&self, value: u32) {
    // SAFETY: 调用方契约
    unsafe { u32_slot::store(self.slot_addr(), value) }
  }

  /// set_length(value, kReleaseStore)。
  ///
  /// # Safety
  /// 同 [Self::length]。
  #[inline]
  unsafe fn set_length_release(&self, value: u32) {
    // SAFETY: 调用方契约
    unsafe { u32_slot::store_order(self.slot_addr(), value, Ordering::Release) }
  }

  /// length 槽地址。
  ///
  /// # Safety
  /// 同 [Self::length]。
  unsafe fn slot_addr(&self) -> usize;
}

impl TrustedFixedArrayBaseExt for Tagged<TrustedFixedArrayBase> {
  #[inline]
  unsafe fn slot_addr(&self) -> usize {
    // 运行时槽位 = 结构体字段偏移; snapshot 锚点 K_LENGTH_OFFSET
    self.address() + offset_of!(TrustedFixedArrayBase, length)
  }
}

impl FixedArrayBase {
  /// GetMaxLengthForNewSpaceAllocation (fixed-array-base-inl.h:67)。
  ///
  /// ElementsKindToShiftSize 归 elements crate (脚本头登记), 调用方传入
  /// elements kind 的 shift size (tagged 槽 = K_TAGGED_SIZE_LOG2)。
  #[inline]
  #[must_use]
  pub const fn max_length_for_new_space_allocation(shift_size: u32) -> usize {
    (K_MAX_REGULAR_HEAP_OBJECT_SIZE - K_HEADER_SIZE) >> shift_size
  }
}

// capacity+length 族 snapshot 常量再导出 (定义于 fixed_array.rs, 头层统一入口)
pub use crate::fixed_array::{
  K_ARRAY_LIST_CAPACITY_OFFSET, K_ARRAY_LIST_HEADER_SIZE, K_ARRAY_LIST_LENGTH_OFFSET,
  K_WEAK_ARRAY_LIST_CAPACITY_OFFSET, K_WEAK_ARRAY_LIST_HEADER_SIZE, K_WEAK_ARRAY_LIST_LENGTH_OFFSET,
};
`;

// 5) 产出对数: 原版 accessor 定义数 == Rust trait 方法定义数 (1:1)
const out = fmt(body);
for (const [k, re] of [
  ["length", /unsafe fn length\(&self\) -> u32 \{/g],
  ["ulength", /unsafe fn ulength\(&self\) -> u32 \{/g],
  ["acquire", /unsafe fn length_acquire\(&self\) -> u32 \{/g],
  ["relaxed", /unsafe fn length_relaxed\(&self\) -> u32 \{/g],
  ["plain_write", /unsafe fn set_length\(&self, value: u32\) \{/g],
  ["release", /unsafe fn set_length_release\(&self, value: u32\) \{/g],
]) {
  if (count(out, re) !== ORIG_ACCESSORS[k])
    fail(`产出 ${k} 与原版不对数`, count(out, re), ORIG_ACCESSORS[k]);
  assertCount++;
}
for (const [got, want, label] of [
  [count(out, /pub struct (?:Trusted)?FixedArrayBase \{/g), 2, "基类头 struct"],
  [count(out, /pub trait \w+Ext: Copy \{/g), 2, "扩展 trait"],
  [count(out, /impl (?:Trusted)?FixedArrayBaseExt for Tagged</g), 2, "ext impl"],
  [count(out, /unsafe fn slot_addr\(&self\) -> usize \{/g), 2, "slot_addr impl"],
  [count(out, /pub unsafe fn (?:load|load_order|store|store_order)\(/g), 4, "u32_slot 助手"],
  [count(out, /^pub const K_/gm), 6, "snapshot 常量"],
  [count(out, /K_ARRAY_LIST_|K_WEAK_ARRAY_LIST_/g), 6, "fixed_array.rs 常量再导出"],
  [count(out, /pub\(crate\) fn write_barrier_stub/g), 1, "写屏障桩"],
]) {
  if (got !== want) fail(label, got, want);
  assertCount++;
}
if (/(?:unwrap|panic!)\(/.test(out)) throw new Error("产出含 unwrap/panic (红线)");
assertCount++;

writeSrc("v8-objects-arrays", "fixed_array_base.rs", out);
console.log(
  `fixed_array_base.rs: 对数断言 ${assertCount} 条全过; K_MAX_FIXED_ARRAY_CAPACITY=${MAX_CAP} + ` +
    "双基类头 + length 语义族 (acquire/relaxed/release) ×2; cow/ElementsKind/verifier 推迟已登记",
);
