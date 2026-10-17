#!/usr/bin/env bun
// tool/src/objects/tagged-field.h.js — tagged_field.rs 生成
// 组→文件映射 (本脚本管辖):
//   v8/src/objects/tagged-field.h + tagged-field-inl.h → v8-objects-core/src/tagged_field.rs
//   v8/src/objects/tagged-value.h (StrongTaggedValue/TaggedValue) → 登记为 Tagged<T> 同构 (非压缩)
//   v8/src/objects/objects.h enum WriteBarrierMode → objects.rs 权威定义 (objects.h.js 展开),
//       本模块经 use crate::objects::WriteBarrierMode 引入, 不再重复定义
// 默认展开: 非压缩配置 (Tagged_t = Address = usize)
// 跳过: JSDispatchHandleMember (依赖 V8_ENABLE_SANDBOX 的 JSDispatchHandle, L3 后续);
//       FLEXIBLE_ARRAY_MEMBER 宏 (Rust 用切片/DST 惯用法, 不做零长数组)
// 规则: 原子访问族按 tagged-field-inl.h 逐方法机械展开; 压缩指针走单一 const 抽象点
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/tagged-field.h");
const inl = readV8("src/objects/tagged-field-inl.h");

// —— 锚点断言: 方法签名清单必须齐 ——
const sigs = [...h.matchAll(/static inline (?:Address|\w+(?:<[^>]+>)?::PtrType|PtrType|Tagged_t|void) (\w+)\(/g)].map((m) => m[1]);
for (const need of ["address", "load", "store", "Relaxed_Load", "Relaxed_Store", "Acquire_Load", "Release_Store", "SeqCst_Load", "SeqCst_Store", "SeqCst_Swap", "Release_CompareAndSwap", "Relaxed_CompareAndSwap", "SeqCst_CompareAndSwap", "Relaxed_Load_Map_Word", "Relaxed_Store_Map_Word", "Release_Store_Map_Word"])
  if (!sigs.includes(need)) throw new Error(`tagged-field.h 方法 ${need} 缺失`);

const memberSigs = [...h.matchAll(/inline (?:Tagged<T> |void )(\w+)\(/g)].map((m) => m[1]);
for (const need of ["load", "store", "Relaxed_Load", "Relaxed_Store", "Acquire_Load", "Release_Store", "SeqCst_Load", "SeqCst_Store", "SeqCst_Swap", "SeqCst_CompareAndSwap", "store_no_write_barrier"])
  if (!memberSigs.includes(need)) throw new Error(`TaggedMember 方法 ${need} 缺失`);

// tagged-field.h:90-91 布局 static_assert (机械提取对齐要求)
if (!/alignof\(TaggedMember<Object>\) == alignof\(Tagged_t\)/.test(h)) throw new Error("TaggedMember 布局断言缺失");
if (!/sizeof\(TaggedMember<Object>\) == sizeof\(Tagged_t\)/.test(h)) throw new Error("TaggedMember 大小断言缺失");

// objects.h:51 WriteBarrierMode 枚举变体提取 (末变体无尾逗号)
const objectsH = readV8("src/objects/objects.h");
const wbVariants = [...objectsH.matchAll(/^\s{2}(\w+_WRITE_BARRIER\w*),?$/gm)].map((m) => m[1]);
if (wbVariants.length !== 6) throw new Error(`WriteBarrierMode 变体提取失败: ${wbVariants}`);

// 变体改名表: C++ SCREAMING_SNAKE → Rust 短名 (机械登记, 与 src 修订一致)
const WB_NAME = {
  SKIP_WRITE_BARRIER: "Skip",
  SKIP_WRITE_BARRIER_SCOPE: "SkipScope",
  SKIP_WRITE_BARRIER_FOR_GC: "SkipForGc",
  UNSAFE_SKIP_WRITE_BARRIER: "UnsafeSkip",
  UPDATE_EPHEMERON_KEY_WRITE_BARRIER: "UpdateEphemeronKey",
  UPDATE_WRITE_BARRIER: "Update",
};
const camel = (v) => WB_NAME[v];

const body = `${banner("src/objects/tagged-field.h.js", "src/objects/tagged-field.h + tagged-field-inl.h", "默认非压缩配置展开; 修订就地改")}
//! V8 堆对象内的 tagged 字段槽位访问器
//!
//! 原版三层: \`TaggedMember<T>\` (对象成员槽) / \`TaggedField<T, kFieldOffset>\` (静态字段访问器) /
//! \`UnalignedValueMember<T>\` (非对齐值成员). 非压缩配置下 \`Tagged_t = Address = usize\`.

use std::marker::PhantomData;
use std::sync::atomic::AtomicUsize;

use crate::heap_object::HeapObject;
use crate::map_word::MapWord;
use crate::tagged::Tagged;

/// tagged 槽位存储类型 (压缩指针抽象点: 启用 V8_COMPRESS_POINTERS 时改为 u32)
pub type TaggedSlot = usize;

/// 压缩抽象点: 槽位值 → 完整地址 (非压缩恒等)
#[inline]
pub const fn tagged_to_full(v: TaggedSlot) -> usize { v }

/// 压缩抽象点: 完整地址 → 槽位值 (非压缩恒等)
#[inline]
pub const fn full_to_tagged(v: usize) -> TaggedSlot { v }

/// 写屏障模式 (登记: 原版定义于 v8/src/objects/objects.h enum WriteBarrierMode,
/// objects.rs 代理落地后迁移并删除本定义)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriteBarrierMode {
${wbVariants.map((v) => `  /// ${v}\n  ${camel(v)},`).join("\n")}
}

/// 对象成员槽: 潜在压缩的 tagged 指针存储 (对应 C++ TaggedMember<T>)
///
/// 布局对齐原版 static_assert: alignof == alignof(Tagged_t), sizeof == sizeof(Tagged_t).
#[repr(C)]
pub struct TaggedMember<T> {
  slot: AtomicUsize,
  _ty: PhantomData<T>,
}

// ===== TaggedMember<T> (tagged-field-inl.h 逐方法展开) =====
impl<T> TaggedMember<T> {
  pub const fn from_raw(_slot: TaggedSlot) -> Self { todo!() }

  pub fn new(_value: Tagged<T>) -> Self { todo!() }

  pub fn load(&self) -> Tagged<T> { todo!() }

  pub fn store(&self, _value: Tagged<T>, _mode: WriteBarrierMode) { todo!() }

  pub fn store_no_write_barrier(&self, _value: Tagged<T>) { todo!() }

  pub fn relaxed_load(&self) -> Tagged<T> { todo!() }

  pub fn relaxed_store(&self, _value: Tagged<T>, _mode: WriteBarrierMode) { todo!() }

  pub fn relaxed_store_no_write_barrier(&self, _value: Tagged<T>) { todo!() }

  pub fn acquire_load(&self) -> Tagged<T> { todo!() }

  pub fn release_store(&self, _value: Tagged<T>, _mode: WriteBarrierMode) { todo!() }

  pub fn release_store_no_write_barrier(&self, _value: Tagged<T>) { todo!() }

  pub fn seqcst_load(&self) -> Tagged<T> { todo!() }

  pub fn seqcst_store(&self, _value: Tagged<T>, _mode: WriteBarrierMode) { todo!() }

  pub fn seqcst_store_no_write_barrier(&self, _value: Tagged<T>) { todo!() }

  pub fn seqcst_swap(&self, _value: Tagged<T>, _mode: WriteBarrierMode) -> Tagged<T> { todo!() }

  pub fn seqcst_compare_and_swap(&self, _expected: Tagged<T>, _value: Tagged<T>, _mode: WriteBarrierMode) -> Tagged<T> { todo!() }

  pub const fn slot_ptr(&self) -> *const TaggedSlot { todo!() }

}

impl<T> Clone for TaggedMember<T> {
  fn clone(&self) -> Self {
    todo!()
  }
}
impl<T> Default for TaggedMember<T> {
  fn default() -> Self {
    todo!()
  }
}

/// 受保护 (trusted space) tagged 成员 (对应 ProtectedTaggedMember, 压缩方案差异登记 TODO)
pub type ProtectedTaggedMember<T> = TaggedMember<T>;

/// 非对齐值成员 (对应 UnalignedValueMember<T>)
#[repr(C)]
pub struct UnalignedValueMember<T> { storage: [u8; 8], _ty: PhantomData<T> }

impl<T: Copy> UnalignedValueMember<T> {
  pub fn new(_value: T) -> Self { todo!() }

  pub fn value(&self) -> T { todo!() } // read_unaligned
  pub fn set_value(&mut self, _value: T) { todo!() } // write_unaligned
}

/// 非对齐 f64 成员 (对应 UnalignedDoubleMember)
#[repr(C)]
pub struct UnalignedDoubleMember { storage: [u8; 8] }
impl UnalignedDoubleMember {
  pub fn new(_value: f64) -> Self { todo!() }

  pub fn value(&self) -> f64 { todo!() }

  pub fn set_value(&mut self, _value: f64) { todo!() }

  pub fn value_as_bits(&self) -> u64 { todo!() }

  pub fn set_value_as_bits(&mut self, _bits: u64) { todo!() }

}

/// 静态字段访问器: 宿主对象内偏移 kFieldOffset 处的 tagged 字段 (对应 TaggedField<T, kFieldOffset>)
pub struct TaggedField<T, const OFFSET: usize = 0> { _ty: PhantomData<T> }

// ===== TaggedField<T, OFFSET> (tagged-field-inl.h 逐方法展开) =====
impl<T, const OFFSET: usize> TaggedField<T, OFFSET> {
  pub fn address(_host: Tagged<HeapObject>, _offset: usize) -> usize { todo!() }

  /// 原版方法 location 的骨架 (unsafe: 裸指针槽访问)。
  ///
  /// # Safety
  /// 调用者须保证宿主对象有效且字段偏移落在对象体内。
  pub unsafe fn location(_host: Tagged<HeapObject>, _offset: usize) -> *mut TaggedSlot { todo!() }

  /// 原版方法 load 的骨架 (unsafe: 裸指针槽访问)。
  ///
  /// # Safety
  /// 调用者须保证宿主对象有效且字段偏移落在对象体内。
  pub unsafe fn load(_host: Tagged<HeapObject>, _offset: usize) -> Tagged<T> { todo!() }

  /// 原版方法 store 的骨架 (unsafe: 裸指针槽访问)。
  ///
  /// # Safety
  /// 调用者须保证宿主对象有效且字段偏移落在对象体内。
  pub unsafe fn store(_host: Tagged<HeapObject>, _value: Tagged<T>) { todo!() }

  /// 原版方法 store_at 的骨架 (unsafe: 裸指针槽访问)。
  ///
  /// # Safety
  /// 调用者须保证宿主对象有效且字段偏移落在对象体内。
  pub unsafe fn store_at(_host: Tagged<HeapObject>, _offset: usize, _value: Tagged<T>) { todo!() }

  /// 原版方法 relaxed_load 的骨架 (unsafe: 裸指针槽访问)。
  ///
  /// # Safety
  /// 调用者须保证宿主对象有效且字段偏移落在对象体内。
  pub unsafe fn relaxed_load(_host: Tagged<HeapObject>, _offset: usize) -> Tagged<T> { todo!() }

  /// 原版方法 relaxed_store 的骨架 (unsafe: 裸指针槽访问)。
  ///
  /// # Safety
  /// 调用者须保证宿主对象有效且字段偏移落在对象体内。
  pub unsafe fn relaxed_store(_host: Tagged<HeapObject>, _value: Tagged<T>) { todo!() }

  /// 原版方法 relaxed_store_at 的骨架 (unsafe: 裸指针槽访问)。
  ///
  /// # Safety
  /// 调用者须保证宿主对象有效且字段偏移落在对象体内。
  pub unsafe fn relaxed_store_at(_host: Tagged<HeapObject>, _offset: usize, _value: Tagged<T>) { todo!() }

  /// 原版方法 acquire_load 的骨架 (unsafe: 裸指针槽访问)。
  ///
  /// # Safety
  /// 调用者须保证宿主对象有效且字段偏移落在对象体内。
  pub unsafe fn acquire_load(_host: Tagged<HeapObject>, _offset: usize) -> Tagged<T> { todo!() }

  /// 原版方法 release_store 的骨架 (unsafe: 裸指针槽访问)。
  ///
  /// # Safety
  /// 调用者须保证宿主对象有效且字段偏移落在对象体内。
  pub unsafe fn release_store(_host: Tagged<HeapObject>, _value: Tagged<T>) { todo!() }

  /// 原版方法 release_store_at 的骨架 (unsafe: 裸指针槽访问)。
  ///
  /// # Safety
  /// 调用者须保证宿主对象有效且字段偏移落在对象体内。
  pub unsafe fn release_store_at(_host: Tagged<HeapObject>, _offset: usize, _value: Tagged<T>) { todo!() }

  /// 原版方法 seqcst_load 的骨架 (unsafe: 裸指针槽访问)。
  ///
  /// # Safety
  /// 调用者须保证宿主对象有效且字段偏移落在对象体内。
  pub unsafe fn seqcst_load(_host: Tagged<HeapObject>, _offset: usize) -> Tagged<T> { todo!() }

  /// 原版方法 seqcst_store 的骨架 (unsafe: 裸指针槽访问)。
  ///
  /// # Safety
  /// 调用者须保证宿主对象有效且字段偏移落在对象体内。
  pub unsafe fn seqcst_store(_host: Tagged<HeapObject>, _value: Tagged<T>) { todo!() }

  /// 原版方法 seqcst_store_at 的骨架 (unsafe: 裸指针槽访问)。
  ///
  /// # Safety
  /// 调用者须保证宿主对象有效且字段偏移落在对象体内。
  pub unsafe fn seqcst_store_at(_host: Tagged<HeapObject>, _offset: usize, _value: Tagged<T>) { todo!() }

  /// 原版方法 seqcst_swap 的骨架 (unsafe: 裸指针槽访问)。
  ///
  /// # Safety
  /// 调用者须保证宿主对象有效且字段偏移落在对象体内。
  pub unsafe fn seqcst_swap(_host: Tagged<HeapObject>, _offset: usize, _value: Tagged<T>) -> Tagged<T> { todo!() }

  /// 原版方法 release_compare_and_swap 的骨架 (unsafe: 裸指针槽访问)。
  ///
  /// # Safety
  /// 调用者须保证宿主对象有效且字段偏移落在对象体内。
  pub unsafe fn release_compare_and_swap(_host: Tagged<HeapObject>, _old: TaggedSlot, _value: Tagged<T>) -> TaggedSlot { todo!() }

  /// 原版方法 relaxed_compare_and_swap 的骨架 (unsafe: 裸指针槽访问)。
  ///
  /// # Safety
  /// 调用者须保证宿主对象有效且字段偏移落在对象体内。
  pub unsafe fn relaxed_compare_and_swap(_host: Tagged<HeapObject>, _old: TaggedSlot, _value: Tagged<T>) -> TaggedSlot { todo!() }

  /// 原版方法 seqcst_compare_and_swap 的骨架 (unsafe: 裸指针槽访问)。
  ///
  /// # Safety
  /// 调用者须保证宿主对象有效且字段偏移落在对象体内。
  pub unsafe fn seqcst_compare_and_swap(_host: Tagged<HeapObject>, _offset: usize, _old: Tagged<T>, _value: Tagged<T>) -> Tagged<T> { todo!() }

}

// MapWord 字段专用 (原版 *_Map_Word 方法, T = MapWord)
impl<const OFFSET: usize> TaggedField<MapWord, OFFSET> {
  /// 原版方法 relaxed_load_map_word 的骨架 (unsafe: 裸指针槽访问)。
  ///
  /// # Safety
  /// 调用者须保证宿主对象有效且字段偏移落在对象体内。
  pub unsafe fn relaxed_load_map_word(_host: Tagged<HeapObject>) -> MapWord { todo!() }

  /// 原版方法 relaxed_store_map_word 的骨架 (unsafe: 裸指针槽访问)。
  ///
  /// # Safety
  /// 调用者须保证宿主对象有效且字段偏移落在对象体内。
  pub unsafe fn relaxed_store_map_word(_host: Tagged<HeapObject>, _value: MapWord) { todo!() }

  /// 原版方法 release_store_map_word 的骨架 (unsafe: 裸指针槽访问)。
  ///
  /// # Safety
  /// 调用者须保证宿主对象有效且字段偏移落在对象体内。
  pub unsafe fn release_store_map_word(_host: Tagged<HeapObject>, _value: MapWord) { todo!() }

}
`;

const p = writeSrc("v8-objects-core", "tagged_field.rs", body);
console.log(`tagged-field.h.js: 已生成 ${p} (TaggedField ${sigs.length} 法, TaggedMember ${memberSigs.length} 法, WriteBarrierMode ${wbVariants.length} 变体)`);
