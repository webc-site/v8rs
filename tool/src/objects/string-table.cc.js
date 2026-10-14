#!/usr/bin/env bun
// tool/src/objects/string-table.cc.js — 覆盖 string-table 同名族:
//   v8/src/objects/string-table.h + string-table-inl.h + string-table.cc
//   + off-heap-hash-table.h + off-heap-hash-table-inl.h (Data 实现类的基座,
//     按 string-table.cc:35-126 组合机械内联进本模块)
//   → v8-objects-strings/src/string_table.rs
//
// 映射登记 (C++ → Rust):
//   StringTableKey 虚基类 → pub trait StringTableKey
//     (raw_hash_field/set_raw_hash_field/length/hash 默认体 = HashBits::decode;
//      IsThinString/UnwrapThinString 默认 false + 契约方法;
//      PrepareForInsertion/GetHandleForInsertion → 默认空/必须实现,
//      isolate/factory 段推迟 L4)
//   OffHeapHashTableBase<OffHeapStringHashSet> (CRTP) → 具体结构体
//     OffHeapStringHashSet (Rust 无 CRTP 必要, 关联常量直接展开)
//   empty_element/deleted_element (Smi 0/1) → const fn + EMPTY_PTR/DELETED_PTR
//   FirstProbe/NextProbe 二次探测 → first_probe/next_probe
//   HasSufficientCapacityToAdd/ComputeCapacity/ComputeCapacityWithShrink/
//   ShouldResizeToAdd → const fn / Option<usize>
//   Data::New/Resize/previous_data 链 → Box 链 (unique_ptr → Box)
//   std::atomic<Data*> → AtomicPtr<Data> (Acquire/Release 逐点对齐)
//   base::Mutex write_mutex_ → std::sync::Mutex
//     (parking_lot 未登记本 crate 依赖, 遗留; 锁粒度对齐: 仅 NumberOfElements/
//      LookupKey 写段/反序列化/引导插入持锁)
//   StringTable::Data::TryStringToIndexOrLookupExisting + ResultSentinel →
//     RESULT_NOT_FOUND/RESULT_UNSUPPORTED 常量落地; 查询本体依赖
//     Flatten/转发表/HashSeed/String 分派 → 推迟 L4 (登记)
//   IterateElements/IterateElementsRange → RootVisitor::visit_compressed_root_pointers
//     (OffHeapObjectSlot 地址语义, 槽宽受 visitors 骨架限制, 登记遗留)
//
// 语义偏差登记 (审阅必读):
//   1. 槽位存 (AtomicUsize element, AtomicU32 hash) 双数组: 原版
//      RehashInto/KeyIsMatch 现场读堆对象哈希; L3 前无堆读通道 → 插入时缓存。
//      并发拆对 (element 新/hash 旧) 仅产生假阴性, 与原版 "false negatives ok"
//      契约 (string-table.cc:482-491) 一致, 不产生假阳性。
//   2. KeyIsMatch 的 string->length() 前置检查 (string-table.cc:54) 归
//      is_match 实现方 (无堆读); 哈希前置检查由表完成 (第 53 行)。
//   3. GetCurrentMemoryUsage 数值按 Rust 布局 (size_of), 公式结构 1:1。
//   4. InsertForIsolateDeserialization 的 StringTableInsertionKey (string-inl.h)
//      → 内部 PtrKey (指针相等查重), 反序列化串互异前提下等价。
//
// 跳过区段登记 (推迟, 归属):
//   LookupString/HasString 的 Flatten+InternalizedStringKey 构造 (string-table.cc:369-449):
//     依赖 String 对象与 factory → OS-1 string.rs / L4 集成
//   SetInternalizedReference (string-table.cc:320-365): 转发表/MakeThin → L4
//   TryStringToIndexOrLookupExisting (string-table.cc:671-809): String 形状分派 → L4
//   InternalizedStringKey (string-table.cc:188-316): factory → L4
//   VerifyIfOwnedBy/Print (OFStream): isolate/diagnostics → L4
import { banner, readV8, writeSrc } from "../../lib.js";

const h = readV8("src/objects/string-table.h");
const inl = readV8("src/objects/string-table-inl.h");
const cc = readV8("src/objects/string-table.cc");
const oht = readV8("src/objects/off-heap-hash-table.h");
const ohti = readV8("src/objects/off-heap-hash-table-inl.h");

// ── 锚点 (实测行号) ─────────────────────────────────────────────────────
const anchors = [
  [h, "class StringTableKey {", "string-table.h"],
  [h, "static constexpr Tagged<Smi> empty_element() { return Smi::FromInt(0); }", "string-table.h"],
  [h, "static constexpr Tagged<Smi> deleted_element() { return Smi::FromInt(1); }", "string-table.h"],
  [inl, "return Name::HashBits::decode(raw_hash_field_);", "string-table-inl.h"],
  [cc, "static constexpr int kEntrySize = 1;", "string-table.cc"],
  [cc, "static constexpr int kMaxEmptyFactor = 4;", "string-table.cc"],
  [cc, "static constexpr int kMinCapacity = 2048;", "string-table.cc"],
  [oht, "return InternalIndex(hash & (size - 1));", "off-heap-hash-table.h (FirstProbe)"],
  [oht, "quadratic probing and Smi 0 and Smi 1 as the empty and deleted sentinels", "off-heap-hash-table.h"],
  [ohti, "int raw_capacity = at_least_space_for + (at_least_space_for >> 1);", "off-heap-hash-table-inl.h"],
  [ohti, "entry = NextProbe(entry, count++, capacity_)) {", "off-heap-hash-table-inl.h (NextProbe 消费点)"],
];
for (const [s, sub, f] of anchors) {
  if (!s.includes(sub)) throw new Error(`锚点失配: ${f} 无 "${sub.slice(0, 52)}..."`);
}

// ── 机械提取常量 ────────────────────────────────────────────────────────
const grab = (s, re, f) => {
  const m = s.match(re);
  if (!m) throw new Error(`常量提取失配: ${f}`);
  return m[1];
};
const kEntrySize = grab(cc, /static constexpr int kEntrySize = (\d+);/, "kEntrySize");
const kMaxEmptyFactor = grab(cc, /static constexpr int kMaxEmptyFactor = (\d+);/, "kMaxEmptyFactor");
const kMinCapacity = grab(cc, /static constexpr int kMinCapacity = (\d+);/, "kMinCapacity");
// ResultSentinel (globals.h:1963, string-table.cc:741/752 消费)
const globals = readV8("src/common/globals.h");
const kNotFound = grab(globals, /kNotFound = (-\d+)/, "ResultSentinel::kNotFound");
const kUnsupported = grab(globals, /kUnsupported = (-\d+)/, "ResultSentinel::kUnsupported");

const body = `${banner("src/objects/string-table.cc.js", "src/objects/string-table.{h,-inl.h,cc} + off-heap-hash-table.{h,-inl.h}", "开放寻址 + 二次探测 1:1; isolate/factory 段推迟 L4 (登记见脚本头)")}
//! 字符串内部化表: 开放寻址 (二次探测) 自研哈希表。
//!
//! - 空槽 = Smi 0, 删除槽 = Smi 1 哨兵
//! - 容量恒为 2 的幂; 探测: first = hash & (size-1), next = (last+n) & (size-1)
//! - 扩容阈值: 加元素后仍保有 ~50% 空闲且删除槽至多占空闲一半
//! - 写路径持 write_mutex; 读路径无锁 (假阴性允许, 假阳性禁止 —
//!   原版 string-table.cc:482-491 并发论证 1:1)

use std::sync::atomic::{AtomicPtr, AtomicU32, AtomicUsize, Ordering};
use std::sync::{Mutex, MutexGuard, PoisonError};

use v8_objects_core::name::{HASH_MASK, HASH_SHIFT};
use v8_objects_core::smi::Smi;
use v8_objects_core::tagged::{Object, Tagged};
use v8_objects_core::visitors::{OffHeapObjectSlot, Root, RootVisitor};

// ── 常量 (string-table.cc:38-40 + globals.h:1963) ─────────────────────
/// 每表项元素数 (OffHeapStringHashSet::kEntrySize)。
pub const ENTRY_SIZE: usize = ${kEntrySize};
/// 收缩判定因子: 需求量 ≤ capacity/N 才考虑收缩 (kMaxEmptyFactor)。
pub const MAX_EMPTY_FACTOR: usize = ${kMaxEmptyFactor};
/// 新表最小容量 (kMinCapacity)。
pub const MIN_CAPACITY: usize = ${kMinCapacity};

/// 查找失败哨兵 (ResultSentinel::kNotFound)。
pub const RESULT_NOT_FOUND: i32 = ${kNotFound};
/// 不支持的整数索引哨兵 (ResultSentinel::kUnsupported)。
pub const RESULT_UNSUPPORTED: i32 = ${kUnsupported};

/// 空槽哨兵 (StringTable::empty_element = Smi 0)。
#[inline]
#[must_use]
pub const fn empty_element() -> Tagged<Smi> {
  Smi::from_int(0)
}

/// 删除槽哨兵 (StringTable::deleted_element = Smi 1)。
#[inline]
#[must_use]
pub const fn deleted_element() -> Tagged<Smi> {
  Smi::from_int(1)
}

/// 空槽裸指针值 (= Smi 0 的 ptr; 原版 static_assert 同款约束)。
const EMPTY_PTR: usize = 0;
/// 删除槽裸指针值 (= Smi 1 的 ptr, 64 位下高位 1)。
const DELETED_PTR: usize = {
  let smi = Smi::from_int(1);
  smi.ptr()
};

/// 内部化键 (string-table.h:20 StringTableKey)。
pub trait StringTableKey {
  /// 原始哈希字段 (须已计算; 原版 DCHECK_NE(0))。
  fn raw_hash_field(&self) -> u32;
  /// 更新原始哈希字段 (protected set_raw_hash_field)。
  fn set_raw_hash_field(&mut self, raw_hash_field: u32);
  /// 键字符串长度。
  fn length(&self) -> u32;
  /// 30 位哈希载荷 (string-table-inl.h:24 hash = HashBits::decode)。
  #[inline]
  fn hash(&self) -> u32 {
    (self.raw_hash_field() & HASH_MASK) >> HASH_SHIFT
  }
  /// 键是否为 ThinString (原版虚函数默认 false)。
  #[inline]
  fn is_thin_string(&self) -> bool {
    false
  }
  /// 解 ThinString 取内部化串。
  ///
  /// 契约 (对应原版 UNREACHABLE 默认体): 仅 \`is_thin_string()\` 为真时调用,
  /// 返回值否则无意义 — 调用方不得在非 thin 键上使用返回值。
  fn unwrap_thin_string(&self) -> Tagged<Object>;
  /// 插入前准备 (InternalizedStringKey::PrepareForInsertion 的
  /// isolate/factory 段; 默认空, L4 绑定后覆写)。
  fn prepare_for_insertion(&mut self) {}
  /// 取待插入元素 (GetHandleForInsertion)。
  fn get_handle_for_insertion(&self) -> Tagged<Object>;
  /// 与表内元素比对 (KeyIsMatch: 哈希前置过滤已由表完成,
  /// 长度/内容比对归本方法 — 无堆读通道, 登记见脚本头偏差 2)。
  fn is_match(&self, element: Tagged<Object>, element_hash: u32) -> bool;
}

/// 首探测位置 (off-heap-hash-table.h:148 FirstProbe)。
#[inline]
#[must_use]
pub const fn first_probe(hash: u32, size: usize) -> usize {
  (hash as usize) & (size - 1)
}

/// 下一探测位置: 二次探测 (off-heap-hash-table.h:152 NextProbe)。
#[inline]
#[must_use]
pub const fn next_probe(last: usize, number: u32, size: usize) -> usize {
  (last.wrapping_add(number as usize)) & (size - 1)
}

/// 元素是否为真键 (非空/非删除哨兵; off-heap-hash-table.h:73 IsKey)。
#[inline]
#[must_use]
pub const fn is_key(element: Tagged<Object>) -> bool {
  let p = element.ptr();
  p != EMPTY_PTR && p != DELETED_PTR
}

// ── 容量策略 (off-heap-hash-table-inl.h:74-115) ───────────────────────
/// 是否仍有足够容量再添加 (50% 空闲 + 删除槽 ≤ 空闲一半;
/// off-heap-hash-table-inl.h:74 HasSufficientCapacityToAdd)。
#[inline]
#[must_use]
pub const fn has_sufficient_capacity_to_add(
  capacity: usize,
  number_of_elements: usize,
  number_of_deleted_elements: usize,
  number_of_additional_elements: usize,
) -> bool {
  let nof = number_of_elements + number_of_additional_elements;
  nof < capacity
    && number_of_deleted_elements <= (capacity - nof) / 2
    && nof + nof / 2 <= capacity
}

/// 计算至少容纳 at_least_space_for 的容量 (加 50% 松弛 + 2 的幂上取整;
/// off-heap-hash-table-inl.h:91 ComputeCapacity)。
#[inline]
#[must_use]
pub const fn compute_capacity(at_least_space_for: usize) -> usize {
  let raw = at_least_space_for + (at_least_space_for >> 1);
  let ceil = raw.next_power_of_two();
  if ceil < MIN_CAPACITY {
    MIN_CAPACITY
  } else {
    ceil
  }
}

/// 计算收缩后的容量 (过空才收缩; off-heap-hash-table-inl.h:101
/// ComputeCapacityWithShrink)。
#[inline]
#[must_use]
pub const fn compute_capacity_with_shrink(
  current_capacity: usize,
  at_least_space_for: usize,
) -> usize {
  if at_least_space_for > current_capacity / MAX_EMPTY_FACTOR {
    return current_capacity;
  }
  let new_capacity = compute_capacity(at_least_space_for);
  if new_capacity < MIN_CAPACITY {
    current_capacity
  } else {
    new_capacity
  }
}

/// 开放寻址字符串哈希集 (string-table.cc:35 OffHeapStringHashSet;
/// 原版堆外变长 malloc 数组 → 双原子数组等价存储, 登记见脚本头偏差 1)。
pub struct OffHeapStringHashSet {
  number_of_elements: usize,
  number_of_deleted_elements: usize,
  capacity: usize,
  /// 槽位元素 (原版 OffHeapObjectSlot 的 Acquire_Load/Release_Store)。
  elements: Vec<AtomicUsize>,
  /// 插入时缓存的哈希 (原版现场读堆对象; RehashInto/前置过滤消费)。
  hashes: Vec<AtomicU32>,
}

impl OffHeapStringHashSet {
  /// 构造并清空为空槽 (off-heap-hash-table-inl.h:19 构造 + MemsetTagged)。
  #[inline]
  pub fn new(capacity: usize) -> Self {
    Self {
      number_of_elements: 0,
      number_of_deleted_elements: 0,
      capacity,
      elements: (0..capacity).map(|_| AtomicUsize::new(EMPTY_PTR)).collect(),
      hashes: (0..capacity).map(|_| AtomicU32::new(0)).collect(),
    }
  }

  /// 表容量。
  #[inline]
  #[must_use]
  pub const fn capacity(&self) -> usize {
    self.capacity
  }

  /// 元素数。
  #[inline]
  #[must_use]
  pub const fn number_of_elements(&self) -> usize {
    self.number_of_elements
  }

  /// 删除槽计数。
  #[inline]
  #[must_use]
  pub const fn number_of_deleted_elements(&self) -> usize {
    self.number_of_deleted_elements
  }

  /// 读槽元素 (原版 GetKey 的 Acquire_Load)。
  #[inline]
  pub fn get_key(&self, index: usize) -> Tagged<Object> {
    Tagged::new(self.elements[index].load(Ordering::Acquire))
  }

  /// 写槽元素 + 缓存哈希 (原版 SetKey 的 Release_Store)。
  #[inline]
  fn set_key(&self, index: usize, key: Tagged<Object>, hash: u32) {
    self.hashes[index].store(hash, Ordering::Relaxed);
    self.elements[index].store(key.ptr(), Ordering::Release);
  }

  /// 在空槽登记新元素 (off-heap-hash-table.h:88 AddAt)。
  pub fn add_at(&mut self, entry: usize, key: Tagged<Object>, hash: u32) {
    debug_assert_eq!(self.get_key(entry).ptr(), EMPTY_PTR);
    debug_assert!(self.number_of_elements + 1 < self.capacity);
    debug_assert!(has_sufficient_capacity_to_add(
      self.capacity,
      self.number_of_elements,
      self.number_of_deleted_elements,
      1
    ));
    self.set_key(entry, key, hash);
    self.number_of_elements += 1;
  }

  /// 覆盖删除槽登记新元素 (off-heap-hash-table.h:100 OverwriteDeletedAt)。
  pub fn overwrite_deleted_at(&mut self, entry: usize, key: Tagged<Object>, hash: u32) {
    debug_assert_eq!(self.get_key(entry).ptr(), DELETED_PTR);
    self.set_key(entry, key, hash);
    self.number_of_elements += 1;
    self.number_of_deleted_elements -= 1;
  }

  /// GC 移除元素后的计数回收 (off-heap-hash-table.h:114 ElementsRemoved)。
  pub fn elements_removed(&mut self, count: usize) {
    debug_assert!(count <= self.number_of_elements);
    self.number_of_elements -= count;
    self.number_of_deleted_elements += count;
  }

  /// 查找键所在槽 (off-heap-hash-table-inl.h:139 FindEntry)。
  ///
  /// 无锁读: 并发拆对仅假阴性 (原版并发契约)。
  pub fn find_entry(&self, key: &dyn StringTableKey, hash: u32) -> Option<usize> {
    let mut count: u32 = 1;
    let mut entry = first_probe(hash, self.capacity);
    loop {
      let element = self.get_key(entry);
      let p = element.ptr();
      if p == EMPTY_PTR {
        return None;
      }
      if p != DELETED_PTR {
        let element_hash = self.hashes[entry].load(Ordering::Relaxed);
        // KeyIsMatch 前置哈希过滤 (string-table.cc:53)
        if element_hash == hash && key.is_match(element, element_hash) {
          return Some(entry);
        }
      }
      entry = next_probe(entry, count, self.capacity);
      count += 1;
    }
  }

  /// 查找可插入槽 (off-heap-hash-table-inl.h:156 FindInsertionEntry;
  /// 持锁互斥路径)。
  pub fn find_insertion_entry(&self, hash: u32) -> usize {
    debug_assert!(has_sufficient_capacity_to_add(
      self.capacity,
      self.number_of_elements,
      self.number_of_deleted_elements,
      1
    ));
    let mut count: u32 = 1;
    let mut entry = first_probe(hash, self.capacity);
    loop {
      let element = self.get_key(entry);
      if !is_key(element) {
        return entry;
      }
      entry = next_probe(entry, count, self.capacity);
      count += 1;
    }
  }

  /// 查找键所在槽或可插入槽 (off-heap-hash-table-inl.h:173
  /// FindEntryOrInsertionEntry; 持锁互斥路径; 返回槽由调用方检查内容)。
  pub fn find_entry_or_insertion_entry(
    &self,
    key: &dyn StringTableKey,
    hash: u32,
  ) -> usize {
    debug_assert!(has_sufficient_capacity_to_add(
      self.capacity,
      self.number_of_elements,
      self.number_of_deleted_elements,
      1
    ));
    let mut insertion_entry: Option<usize> = None;
    let mut count: u32 = 1;
    let mut entry = first_probe(hash, self.capacity);
    loop {
      let element = self.get_key(entry);
      let p = element.ptr();
      if p == EMPTY_PTR {
        return insertion_entry.unwrap_or(entry);
      }
      if p == DELETED_PTR {
        // 删除槽是候选插入位, 但继续找可能存在的匹配槽
        if insertion_entry.is_none() {
          insertion_entry = Some(entry);
        }
      } else {
        let element_hash = self.hashes[entry].load(Ordering::Relaxed);
        if element_hash == hash && key.is_match(element, element_hash) {
          return entry;
        }
      }
      entry = next_probe(entry, count, self.capacity);
      count += 1;
    }
  }

  /// 是否需要扩容/收缩 (off-heap-hash-table-inl.h:49 ShouldResizeToAdd)。
  #[must_use]
  pub fn should_resize_to_add(&self, additional_elements: usize) -> Option<usize> {
    let shrink = compute_capacity_with_shrink(
      self.capacity,
      self.number_of_elements + additional_elements,
    );
    if shrink < self.capacity {
      Some(shrink)
    } else if !has_sufficient_capacity_to_add(
      self.capacity,
      self.number_of_elements,
      self.number_of_deleted_elements,
      additional_elements,
    ) {
      Some(compute_capacity(self.number_of_elements + additional_elements))
    } else {
      None
    }
  }

  /// 将本表全部键重哈希进新表 (off-heap-hash-table-inl.h:28 RehashInto;
  /// 持锁互斥路径 — 键哈希读缓存值)。
  pub fn rehash_into(&self, new_table: &mut Self) {
    debug_assert!(self.number_of_elements < new_table.capacity);
    for i in 0..self.capacity {
      let element = self.get_key(i);
      if !is_key(element) {
        continue;
      }
      let hash = self.hashes[i].load(Ordering::Relaxed);
      let insertion_index = new_table.find_insertion_entry(hash);
      new_table.set_key(insertion_index, element, hash);
    }
    new_table.number_of_elements = self.number_of_elements;
  }

  /// GC 遍历槽区间 (off-heap-hash-table-inl.h:118 IterateElementsRange 的
  /// 地址语义透传; 原版 VisitCompressedRootPointers)。
  ///
  /// # Safety
  /// start ≤ end ≤ capacity; 访客只可经由槽地址原子改写元素
  /// (写删除哨兵), 不得越过容量边界。
  pub unsafe fn iterate_elements_range(
    &self,
    visitor: &mut dyn RootVisitor,
    start: usize,
    end: usize,
  ) -> v8_objects_core::Result<()> {
    debug_assert!(start <= end && end <= self.capacity);
    let base = self.elements.as_ptr();
    // SAFETY: base 来自活 Vec; start/end 在容量内 (end 为 one-past-end 合法)
    let first = OffHeapObjectSlot::new(unsafe { base.add(start) } as usize);
    let last = OffHeapObjectSlot::new(unsafe { base.add(end) } as usize);
    visitor.visit_compressed_root_pointers(Root::StringTable, None, first, last)
  }

  /// GC 遍历全表 (IterateElements)。
  pub fn iterate_elements(
    &self,
    visitor: &mut dyn RootVisitor,
  ) -> v8_objects_core::Result<()> {
    // SAFETY: 全量区间在容量内
    unsafe { self.iterate_elements_range(visitor, 0, self.capacity) }
  }
}

// ── Data: 表数据 + 旧表保活链 (string-table.cc:87-157) ────────────────
struct Data {
  /// 上一次 resize 的旧表 (无锁读者保活; DropOldData 释放)。
  previous_data: Option<Box<Data>>,
  table: OffHeapStringHashSet,
}

impl Data {
  /// Data::New (string-table.cc:147)。
  fn new(capacity: usize) -> Box<Self> {
    Box::new(Self {
      previous_data: None,
      table: OffHeapStringHashSet::new(capacity),
    })
  }

  /// Data::Resize (string-table.cc:151): 旧表重哈希进新表并挂链。
  fn resize(data: Box<Self>, capacity: usize) -> Box<Self> {
    let mut new_data = Data::new(capacity);
    data.table.rehash_into(&mut new_data.table);
    new_data.previous_data = Some(data);
    new_data
  }

  /// 内存占用 (Rust 布局; 公式结构对齐 GetCurrentMemoryUsage)。
  fn current_memory_usage(&self) -> usize {
    let mut usage = size_of::<Self>()
      + self.table.capacity * (size_of::<AtomicUsize>() + size_of::<AtomicU32>());
    if let Some(prev) = &self.previous_data {
      usage += prev.current_memory_usage();
    }
    usage
  }
}

// ── StringTable (string-table.h:51) ──────────────────────────────────
/// 字符串内部化表。
///
/// 写路径持 \`write_mutex\`; 读路径无锁。resize 先复制后发布
/// (Release 存指针), 旧表经 previous_data 链保活供并发读者。
pub struct StringTable {
  data: AtomicPtr<Data>,
  /// 写互斥 (原版 mutable base::Mutex; parking_lot 未登记, std 等价)。
  write_mutex: Mutex<()>,
}

impl Default for StringTable {
  #[inline]
  fn default() -> Self {
    Self::new()
  }
}

impl StringTable {
  /// 构造 (string-table.cc:169; isolate 参数推迟)。
  pub fn new() -> Self {
    Self {
      data: AtomicPtr::new(Box::into_raw(Data::new(MIN_CAPACITY))),
      write_mutex: Mutex::new(()),
    }
  }

  fn lock_write(&self) -> MutexGuard<'_, ()> {
    // 锁中毒恢复: 表数据无不变量破坏风险 (原子槽 + 计数仅持锁更新)
    self.write_mutex.lock().unwrap_or_else(PoisonError::into_inner)
  }

  /// 当前容量 (string-table.cc:178)。
  #[must_use]
  pub fn capacity(&self) -> usize {
    // SAFETY: data_ 指针在析构前恒有效 (旧链保活, 见 DropOldData)
    unsafe { (*self.data.load(Ordering::Acquire)).table.capacity() }
  }

  /// 元素数 (string-table.cc:181; 原版持锁读)。
  #[must_use]
  pub fn number_of_elements(&self) -> usize {
    let _guard = self.lock_write();
    // SAFETY: 同上
    unsafe { (*self.data.load(Ordering::Relaxed)).table.number_of_elements() }
  }

  /// 只读探测 (string-table.cc:452 TryLookupKey; 无锁, 假阴性允许)。
  #[must_use]
  pub fn try_lookup_key(&self, key: &dyn StringTableKey) -> Option<Tagged<Object>> {
    let data = self.data.load(Ordering::Acquire);
    // SAFETY: 旧链保活使 data 在读期间有效 (string-table.cc:505-509 论证)
    let table = unsafe { &(*data).table };
    let entry = table.find_entry(key, key.hash());
    entry.map(|e| table.get_key(e))
  }

  /// 查询是否已在表中 (string-table.cc:435 HasString 的表内语义;
  /// Flatten/键构造归 String 层, 登记见脚本头)。
  #[must_use]
  pub fn has_string(&self, key: &dyn StringTableKey) -> bool {
    self.try_lookup_key(key).is_some()
  }

  /// 查找或插入 (string-table.cc:469 LookupKey)。
  ///
  /// 无锁先探; 未命中则准备插入 → 持锁二次探测 (含删除槽复用)。
  pub fn lookup_key(&self, key: &mut dyn StringTableKey) -> Tagged<Object> {
    // 先无锁探测: 新表不会在串死亡前删除对应项, 旧表至多假阴性
    if let Some(found) = self.try_lookup_key(key) {
      return found;
    }

    // 未命中: 准备插入 (锁外, 原版避免锁内分配)
    key.prepare_for_insertion();
    let _guard = self.lock_write();

    // SAFETY: 持锁期间独占; data_ 链保活使指针有效
    let data = unsafe { &mut *self.ensure_capacity(1) };

    // 分配可能已使键变 ThinString (共享堆并发内部化): 直接解引用返回
    if key.is_thin_string() {
      return key.unwrap_thin_string();
    }

    // 持锁后最后查一次 (可能与无锁探测间被并发写入)
    let entry = data.table.find_entry_or_insertion_entry(key, key.hash());
    let element = data.table.get_key(entry);
    let p = element.ptr();
    if p == EMPTY_PTR {
      // 空槽: 写入并计数
      let new_string = key.get_handle_for_insertion();
      let hash = key.hash();
      data.table.add_at(entry, new_string, hash);
      new_string
    } else if p == DELETED_PTR {
      // 删除槽: 覆盖并回退删除计数
      let new_string = key.get_handle_for_insertion();
      let hash = key.hash();
      data.table.overwrite_deleted_at(entry, new_string, hash);
      new_string
    } else {
      // 已存在: 直接返回现有元素
      element
    }
  }

  /// 扩容保障 (string-table.cc:583 EnsureCapacity); 返回裸指针规避
  /// &self → &mut 假共享 (clippy::mut_from_ref)。
  ///
  /// # Safety
  /// 调用方须持 write_mutex (原版 write_mutex_.AssertHeld);
  /// 返回指针仅在锁存活期内解引用, 且须以 &mut 使用。
  unsafe fn ensure_capacity(&self, additional_elements: usize) -> *mut Data {
    let data = self.data.load(Ordering::Relaxed);

    // SAFETY: 持锁期间独占访问
    if let Some(new_capacity) =
      unsafe { (*data).table.should_resize_to_add(additional_elements) }
    {
      // 取回所有权 → 复制进新表 → 挂链 → Release 发布
      // SAFETY: data 来自 data_ 指针且独占; from_raw 后原指针经新表链继续保活
      let old = unsafe { Box::from_raw(data) };
      let new_data = Data::resize(old, new_capacity);
      let ptr = Box::into_raw(new_data);
      self.data.store(ptr, Ordering::Release);
      ptr
    } else {
      data
    }
  }

  /// 反序列化批量插入 (string-table.cc:811; StringTableInsertionKey →
  /// 指针相等查重, 登记见脚本头偏差 4)。
  pub fn insert_for_isolate_deserialization(&self, strings: &[(Tagged<Object>, u32)]) {
    debug_assert_eq!(self.number_of_elements(), 0);

    let _guard = self.lock_write();
    // SAFETY: 持锁期间独占; data_ 链保活使指针有效
    let data = unsafe { &mut *self.ensure_capacity(strings.len()) };
    for &(element, hash) in strings {
      let key = PtrKey { element, hash };
      let entry = data
        .table
        .find_entry_or_insertion_entry(&key, key.hash());
      let existing = data.table.get_key(entry);
      if !is_key(existing) {
        data.table.add_at(entry, element, hash);
      }
    }

    // 持锁中直读计数 (不可再入 number_of_elements, std Mutex 非递归)
    debug_assert_eq!(data.table.number_of_elements(), strings.len());
  }

  /// 引导期插入空串 (string-table.cc:837; 原版 empty_string 取自 roots +
  /// EnsureHash — 参数化传入)。
  pub fn insert_empty_string_for_bootstrapping(&self, empty: Tagged<Object>, hash: u32) {
    debug_assert_eq!(self.number_of_elements(), 0);
    let _guard = self.lock_write();
    // SAFETY: 持锁期间独占; data_ 链保活使指针有效
    let data = unsafe { &mut *self.ensure_capacity(1) };
    let entry = data.table.find_insertion_entry(hash);
    data.table.add_at(entry, empty, hash);
    debug_assert_eq!(data.table.number_of_elements(), 1);
  }

  /// 内存占用 (string-table.cc:859; Rust 布局口径, 登记偏差 3)。
  #[must_use]
  pub fn get_current_memory_usage(&self) -> usize {
    let data = self.data.load(Ordering::Acquire);
    // SAFETY: 指针有效 (旧链保活)
    size_of::<Self>() + unsafe { (*data).current_memory_usage() }
  }

  /// GC 全表遍历 (string-table.cc:864; safepoint 契约由调用方保证)。
  pub fn iterate_elements(
    &self,
    visitor: &mut dyn RootVisitor,
  ) -> v8_objects_core::Result<()> {
    let data = self.data.load(Ordering::Relaxed);
    // SAFETY: GC safepoint 下无并发写
    unsafe { (*data).table.iterate_elements(visitor) }
  }

  /// GC 区间遍历 (string-table.cc:871)。
  pub fn iterate_elements_range(
    &self,
    visitor: &mut dyn RootVisitor,
    start: usize,
    end: usize,
  ) -> v8_objects_core::Result<()> {
    let data = self.data.load(Ordering::Relaxed);
    // SAFETY: GC safepoint 下无并发写; 区间合法性由 callee debug_assert
    unsafe { (*data).table.iterate_elements_range(visitor, start, end) }
  }

  /// 释放旧表链 (string-table.cc:880 DropOldData; GC safepoint 契约)。
  pub fn drop_old_data(&self) {
    let data = self.data.load(Ordering::Relaxed);
    // SAFETY: GC safepoint 下独占; 仅清 previous_data 链, 当前表不动
    unsafe { (*data).previous_data = None };
  }

  /// GC 移除元素通知 (string-table.cc:888 NotifyElementsRemoved)。
  pub fn notify_elements_removed(&self, count: usize) {
    let data = self.data.load(Ordering::Relaxed);
    // SAFETY: GC safepoint 下独占 (槽位清删除哨兵由 GC 访客完成)
    unsafe { (*data).table.elements_removed(count) };
  }
}

impl Drop for StringTable {
  /// 析构 (string-table.cc:176): 释放当前表及旧表保活链。
  fn drop(&mut self) {
    let ptr = *self.data.get_mut();
    if !ptr.is_null() {
      // SAFETY: ptr 由 Box::into_raw 得来且唯一所有权在 data_
      unsafe { drop(Box::from_raw(ptr)) };
    }
  }
}

/// 反序列化插入的指针键 (StringTableInsertionKey 的指针相等形态)。
///
/// 持 30 位哈希载荷, raw_hash_field 按位段编码 (与键约定一致)。
struct PtrKey {
  element: Tagged<Object>,
  hash: u32,
}

impl StringTableKey for PtrKey {
  #[inline]
  fn raw_hash_field(&self) -> u32 {
    self.hash << HASH_SHIFT
  }
  #[inline]
  fn set_raw_hash_field(&mut self, raw_hash_field: u32) {
    self.hash = (raw_hash_field & HASH_MASK) >> HASH_SHIFT;
  }
  #[inline]
  fn length(&self) -> u32 {
    0
  }
  #[inline]
  fn unwrap_thin_string(&self) -> Tagged<Object> {
    // 契约: 反序列化键恒非 ThinString (见 trait 文档)
    self.element
  }
  #[inline]
  fn get_handle_for_insertion(&self) -> Tagged<Object> {
    self.element
  }
  #[inline]
  fn is_match(&self, element: Tagged<Object>, element_hash: u32) -> bool {
    element.ptr() == self.element.ptr() && element_hash == self.hash()
  }
}
`;

writeSrc("v8-objects-strings", "string_table.rs", body);
console.log(
  `string_table.rs: 常量 ${[kEntrySize, kMaxEmptyFactor, kMinCapacity].join("/")} + 哨兵 ${kNotFound}/${kUnsupported}; ` +
    "探测/容量/rehash/lookup 逐行对齐; isolate 段推迟已登记",
);
