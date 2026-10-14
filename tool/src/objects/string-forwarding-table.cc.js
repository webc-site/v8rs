#!/usr/bin/env bun
// tool/src/objects/string-forwarding-table.cc.js — 覆盖 string-forwarding-table 同名族:
//   v8/src/objects/string-forwarding-table.h + -inl.h + .cc
//   → v8-objects-strings/src/string_forwarding_table.rs
//
// 映射登记 (C++ → Rust):
//   Record (Tagged_t 槽) → 原子 usize 槽 (Acquire/Release/CAS 逐点对齐)
//   OffHeapObjectSlot Acquire_Load/Release_Store → AtomicUsize load/store
//   base::AsAtomicPointer (external_resource_) → AtomicUsize
//   AsAtomicTagged::Release_CompareAsSwap (set_raw_hash_if_empty) →
//     compare_exchange with success=Release
//   Block (变长 malloc) → Vec<Record> 等价存储; BlockVector (Block** 原子) →
//     Vec<AtomicPtr<Block>> + AtomicUsize size
//   std::atomic<BlockVector*> blocks_ → AtomicPtr<BlockVector>
//   block_vector_storage_ (vector<unique_ptr>) → Mutex<Vec<Box<BlockVector>>>
//     (旧向量保活供并发读者)
//   base::Mutex grow_mutex_ → std::sync::Mutex (parking_lot 未登记, 遗留;
//     锁粒度对齐: 仅 EnsureCapacity 增长段 + storage 登记段)
//   std::atomic<int> next_free_index_ → AtomicI32 (fetch_add SeqCst = 原版 ++)
//   UpdateForwardedSlot (map_word 转发判定) → v8-objects-core map_word
//     (from_map 复用同一槽位值; 转发判定按位)
//   Heap::InFromPage / HeapLayout 判定 → in_from_page 回调参数化
//     (堆页归属 L3 heap 层, 推迟绑定)
//   v8::String::ExternalStringResourceBase* → 裸地址 usize + one-byte LSB tag
//     (原版本就以 Address 存储; Dispose 调用 → teardown 的 dispose 回调)
//   CHECK_LT(index, size()) → Result<_, crate::Error::Unsupported> (禁 panic;
//     复用 crate error.rs 既有变体, 不新增本地错误类型)
//
// 跳过区段登记 (推迟):
//   Record::raw_hash 的堆对象分支 (Cast<String>->RawHash): String 对象哈希
//    读取归 OS-1 string.rs / L4 → RawHash::ForwardStringObject 变体透传
//   Record::DisposeExternalResource / DisposeUnusedExternalResource 的资源
//    对象方法调用: API 层资源对象推迟 → teardown(dispose 回调) 承接去重语义
//   Reset/UpdateAfter*Evacuation 的 safepoint 断言: heap 层推迟 (契约注释保留)
//   GetForwardStringAddress/GetRawHashStatic (isolate 静态入口): L4 isolate 绑定
import { banner, readV8, writeSrc } from "../../lib.js";

const h = readV8("src/objects/string-forwarding-table.h");
const inl = readV8("src/objects/string-forwarding-table-inl.h");
const cc = readV8("src/objects/string-forwarding-table.cc");

// ── 锚点 (实测) ─────────────────────────────────────────────────────────
const anchors = [
  [h, "static constexpr int kInitialBlockSize = 16;", "h"],
  [h, "static constexpr int kInitialBlockVectorCapacity = 4;", "h"],
  [h, "static constexpr Tagged<Smi> unused_element() { return Smi::FromInt(0); }", "h"],
  [inl, "static constexpr intptr_t kExternalResourceIsOneByteTag = 1;", "inl"],
  [inl, "kBitsPerInt -", "inl (BlockForIndex)"],
  [inl, "AsAtomicTagged::Release_CompareAndSwap(&forward_string_or_hash_,", "inl"],
  [cc, "block->record(index_in_block)->SetInternalized(string, forward_to);", "cc"],
  [cc, "if (block_index >= blocks->capacity()) {", "cc (EnsureCapacity)"],
];
for (const [s, sub, f] of anchors) {
  if (!s.includes(sub)) throw new Error(`锚点失配: ${f} 无 "${sub.slice(0, 52)}..."`);
}

// ── 机械推导: kInitialBlockSizeHighestBit = 32 - clz32(16) - 1 ──────────
const clz32 = (x) => 32 - x.toString(2).length;
const kInitialBlockSize = 16;
const kHighestBit = 32 - clz32(kInitialBlockSize) - 1; // = 4
if (kHighestBit !== 4) throw new Error(`kInitialBlockSizeHighestBit 推导异常: ${kHighestBit}`);

const body = `${banner("src/objects/string-forwarding-table.cc.js", "src/objects/string-forwarding-table.{h,-inl.h,cc}", "并发分段转发索引表 1:1; 锁粒度/原子序对齐; 资源 Dispose/safepoint 段回调化或推迟 (登记见脚本头)")}
//! 字符串转发表: 转发索引 (存于串的哈希字段) → 内部化串/外部资源 的映射。
//!
//! 分 "块" 组织: 写只追加 (无锁分配索引), 仅增长表需锁; 旧块向量保活,
//! 并发读者持有的指针在 Reset 前恒有效。原版 string-forwarding-table.h:22-25
//! 并发论证 1:1。

use std::ptr::NonNull;
use std::sync::atomic::{AtomicI32, AtomicPtr, AtomicUsize, Ordering};
use std::sync::{Mutex, MutexGuard, PoisonError};

use v8_objects_core::heap_object::HeapObject;
use v8_objects_core::map_word::MapWord;
use v8_objects_core::smi::Smi;
use v8_objects_core::tagged::{Object, Tagged};
use v8_objects_core::tagged_constants::{HEAP_OBJECT_TAG, HEAP_OBJECT_TAG_MASK};

use crate::error::{Error, Result};

// ── 常量 (string-forwarding-table.h:29-36 + inl.h:89-92) ──────────────
/// 首块容量 (kInitialBlockSize; 2 的幂)。
pub const INITIAL_BLOCK_SIZE: u32 = ${kInitialBlockSize};
/// 首块容量的最高位序号 (kInitialBlockSizeHighestBit; 32-clz32(16)-1)。
pub const INITIAL_BLOCK_SIZE_HIGHEST_BIT: u32 = ${kHighestBit};
/// 块向量初始容量 (kInitialBlockVectorCapacity)。
pub const INITIAL_BLOCK_VECTOR_CAPACITY: usize = 4;

/// 未用槽哨兵 (unused_element = Smi 0; ptr 值 = 0, 原版 static_assert 同款)。
pub const fn unused_element() -> Tagged<Smi> {
  Smi::from_int(0)
}
/// 删除槽哨兵 (deleted_element = Smi 1)。
pub const fn deleted_element() -> Tagged<Smi> {
  Smi::from_int(1)
}

/// 未用槽裸值 (= 0; Record 零初始化即未用)。
const UNUSED_PTR: usize = 0;
/// 删除槽裸值。
const DELETED_PTR: usize = {
  let smi = Smi::from_int(1);
  smi.ptr()
};

/// 外部资源地址的 one-byte 标记位 (kExternalResourceIsOneByteTag)。
const EXTERNAL_RESOURCE_IS_ONE_BYTE_TAG: usize = 1;
/// 外部资源编码掩码 (kExternalResourceEncodingMask)。
const EXTERNAL_RESOURCE_ENCODING_MASK: usize = 1;
/// 外部资源地址掩码 (kExternalResourceAddressMask)。
const EXTERNAL_RESOURCE_ADDRESS_MASK: usize = !EXTERNAL_RESOURCE_ENCODING_MASK;

/// 索引越界错误 (原版 CHECK_LT 崩溃路径 → crate::Error::Unsupported)。
#[inline]
fn index_out_of_range() -> Error {
  Error::Unsupported("StringForwardingTable: 索引越界 (CHECK_LT 等价)")
}

// ── 块索引位运算 (string-forwarding-table-inl.h:303-331) ──────────────
/// 索引 → (块号, 块内索引)。块号 = (index+首块容量) 的最高位序号修正值。
#[inline]
#[must_use]
pub fn block_for_index(index: i32) -> (u32, u32) {
  debug_assert!(index >= 0);
  let v = (index as u32) + INITIAL_BLOCK_SIZE;
  let block = 32 - v.leading_zeros() - INITIAL_BLOCK_SIZE_HIGHEST_BIT - 1;
  (block, index_in_block(index, block))
}

/// 块内索引: 清除最左定位位 (IndexInBlock)。
#[inline]
#[must_use]
pub const fn index_in_block(index: i32, block: u32) -> u32 {
  debug_assert!(index >= 0);
  ((index as u32) + INITIAL_BLOCK_SIZE)
    & !(1u32 << (block + INITIAL_BLOCK_SIZE_HIGHEST_BIT))
}

/// 块容量: 块号越大容量翻倍 (CapacityForBlock)。
#[inline]
#[must_use]
pub const fn capacity_for_block(block: u32) -> u32 {
  1u32 << (block + INITIAL_BLOCK_SIZE_HIGHEST_BIT)
}

/// 原始槽值是否堆对象 (按 heap object tag 位判定)。
#[inline]
const fn is_heap_object_bits(v: usize) -> bool {
  (v & HEAP_OBJECT_TAG_MASK) == HEAP_OBJECT_TAG
}

// ── Record (string-forwarding-table-inl.h:24-130) ─────────────────────
/// 单条转发记录: 原串 / 转发串或裸哈希 / 外部资源地址。
///
/// 字段语义 (inl.h:113-127 注释 1:1):
/// - 内部化转发: forward 槽存内部化串对象, external 为空
/// - 外部化转发: forward 槽存裸哈希 (LSB=0 与堆对象 tag 可区分),
///   external 存资源地址 (LSB=one-byte 标记)
/// - 先外部化再内部化: 裸哈希被内部化串覆写, 资源地址不覆写
pub struct Record {
  /// 原始串 (待转型串) 槽。
  original_string: AtomicUsize,
  /// 转发串对象或裸哈希槽。
  forward_string_or_hash: AtomicUsize,
  /// 外部资源地址 + one-byte LSB 标记 (0 = 无资源)。
  external_resource: AtomicUsize,
}

impl Record {
  /// 零值记录 (块构造用; 原版 memset 0)。
  #[inline]
  const fn zeroed() -> Self {
    Self {
      original_string: AtomicUsize::new(UNUSED_PTR),
      forward_string_or_hash: AtomicUsize::new(UNUSED_PTR),
      external_resource: AtomicUsize::new(UNUSED_PTR),
    }
  }

  /// 原始串对象 (OriginalStringObject 的 Acquire_Load)。
  #[inline]
  #[must_use]
  pub fn original_string_object(&self) -> Tagged<Object> {
    Tagged::new(self.original_string.load(Ordering::Acquire))
  }

  /// 转发槽原始值 (ForwardStringObjectOrHash 的 Acquire_Load)。
  #[inline]
  #[must_use]
  pub fn forward_string_or_hash_object(&self) -> Tagged<Object> {
    Tagged::new(self.forward_string_or_hash.load(Ordering::Acquire))
  }

  /// 转发串对象 (Cast<InternalizedString> 占位为 Object)。
  #[inline]
  #[must_use]
  pub fn forward_string(&self) -> Tagged<Object> {
    self.forward_string_or_hash_object()
  }

  /// 外部资源地址 (含编码位; ExternalResourceAddress 的 Acquire_Load)。
  #[inline]
  #[must_use]
  pub fn external_resource_address(&self) -> usize {
    self.external_resource.load(Ordering::Acquire)
  }

  /// 外部资源 (地址, 是否 one-byte)。
  #[inline]
  #[must_use]
  pub fn external_resource(&self) -> (usize, bool) {
    let address = self.external_resource_address();
    let is_one_byte = (address & EXTERNAL_RESOURCE_ENCODING_MASK)
      == EXTERNAL_RESOURCE_IS_ONE_BYTE_TAG;
    (address & EXTERNAL_RESOURCE_ADDRESS_MASK, is_one_byte)
  }

  /// 解码裸哈希 (raw_hash; 堆对象分支归 String 层 → RawHash 透传)。
  #[inline]
  #[must_use]
  pub fn raw_hash(&self) -> RawHash {
    let v = self.forward_string_or_hash.load(Ordering::Acquire);
    if is_heap_object_bits(v) {
      RawHash::ForwardStringObject(Tagged::new(v))
    } else {
      debug_assert!(v8_objects_core::name::Name::is_hash_field_computed(v as u32));
      RawHash::Hash(v as u32)
    }
  }

  /// 写原始串 (set_original_string 的 Release_Store)。
  #[inline]
  pub fn set_original_string(&self, object: Tagged<Object>) {
    self.original_string.store(object.ptr(), Ordering::Release);
  }

  /// 写转发串 (set_forward_string 的 Release_Store; 覆盖裸哈希)。
  #[inline]
  pub fn set_forward_string(&self, object: Tagged<Object>) {
    self.forward_string_or_hash.store(object.ptr(), Ordering::Release);
  }

  /// 仅当转发槽为空时写入裸哈希 (set_raw_hash_if_empty 的 CAS;
  /// 已有转发串则不覆盖 — 原版语义)。
  #[inline]
  pub fn set_raw_hash_if_empty(&self, raw_hash: u32) {
    debug_assert!(v8_objects_core::name::Name::is_hash_field_computed(raw_hash));
    let _ = self.forward_string_or_hash.compare_exchange(
      UNUSED_PTR,
      raw_hash as usize,
      Ordering::Release,
      Ordering::Relaxed,
    );
  }

  /// 写外部资源地址 (set_external_resource 的 Release_Store)。
  #[inline]
  pub fn set_external_resource_address(&self, address: usize) {
    self.external_resource.store(address, Ordering::Release);
  }

  /// 写外部资源 (带 one-byte 编码位)。
  #[inline]
  pub fn set_external_resource(&self, resource: usize, is_one_byte: bool) {
    let mut address = resource;
    if is_one_byte && address != UNUSED_PTR {
      address |= EXTERNAL_RESOURCE_IS_ONE_BYTE_TAG;
    }
    self.set_external_resource_address(address);
  }

  /// 内部化转发登记 (SetInternalized: 原串 + 转发串 + 清资源)。
  #[inline]
  pub fn set_internalized(&self, string: Tagged<Object>, forward_to: Tagged<Object>) {
    self.set_original_string(string);
    self.set_forward_string(forward_to);
    self.set_external_resource_address(UNUSED_PTR);
  }

  /// 外部化转发登记 (SetExternal: 原串 + 空写哈希 + 资源)。
  #[inline]
  pub fn set_external(
    &self,
    string: Tagged<Object>,
    resource: usize,
    is_one_byte: bool,
    raw_hash: u32,
  ) {
    self.set_original_string(string);
    self.set_raw_hash_if_empty(raw_hash);
    self.set_external_resource(resource, is_one_byte);
  }

  /// 尝试写外部资源 (TryUpdateExternalResource 的 CAS: 已有资源则拒写防泄漏)。
  #[inline]
  #[must_use]
  pub fn try_update_external_resource(&self, resource: usize, is_one_byte: bool) -> bool {
    let mut address = resource;
    if is_one_byte && address != UNUSED_PTR {
      address |= EXTERNAL_RESOURCE_IS_ONE_BYTE_TAG;
    }
    self.try_update_external_resource_address(address)
  }

  /// 尝试写外部资源地址 (AcquireRelease CAS, null → address)。
  #[inline]
  #[must_use]
  pub fn try_update_external_resource_address(&self, address: usize) -> bool {
    self
      .external_resource
      .compare_exchange(UNUSED_PTR, address, Ordering::AcqRel, Ordering::Acquire)
      .is_ok()
  }
}

/// raw_hash 的两分支 (堆对象分支的哈希读取推迟, 见脚本头登记)。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RawHash {
  /// 裸哈希值。
  Hash(u32),
  /// 转发槽持有堆对象 (其 RawHash 读取归 String 层)。
  ForwardStringObject(Tagged<Object>),
}

// ── Block (string-forwarding-table-inl.h:234-261 + cc:21-63) ──────────
/// 定容记录块 (原版变长分配 → Vec 等价)。
struct Block {
  capacity: usize,
  records: Vec<Record>,
}

impl Block {
  /// 新建并清零 (Block 构造 + memset 0)。
  fn new(capacity: usize) -> Box<Self> {
    Box::new(Self {
      capacity,
      records: (0..capacity).map(|_| Record::zeroed()).collect(),
    })
  }

  #[inline]
  const fn capacity(&self) -> usize {
    self.capacity
  }

  #[inline]
  fn record(&self, index: usize) -> &Record {
    debug_assert!(index < self.capacity);
    &self.records[index]
  }

  /// 年轻代疏散后修正 (cc:93; in_from_page 判定回调 = Heap::InFromPage)。
  fn update_after_young_evacuation(
    &self,
    up_to_index: usize,
    in_from_page: &dyn Fn(Tagged<HeapObject>) -> bool,
  ) {
    for record in &self.records[..up_to_index] {
      let slot = &record.original_string;
      let original = Tagged::<Object>::new(slot.load(Ordering::Acquire));
      if !original.is_heap_object() {
        continue;
      }
      let object: Tagged<HeapObject> = original.cast();
      if in_from_page(object) {
        // SAFETY: GC 疏散期调用契约由外层 update_after_young_evacuation 保证
        let was_forwarded = unsafe { update_forwarded_slot(object, slot) };
        if !was_forwarded {
          // 年轻代中死亡: 标删除
          slot.store(DELETED_PTR, Ordering::Release);
        }
      }
      // 转发串 (内部化) 恒不在年轻代 — 原版 DEBUG 断言, 语义注释保留
    }
  }

  /// 全量疏散后修正 (cc:122: 原串与转发槽都要跟随转发)。
  fn update_after_full_evacuation(&self, up_to_index: usize) {
    for record in &self.records[..up_to_index] {
      let original = record.original_string_object();
      if original.is_heap_object() {
        let object: Tagged<HeapObject> = original.cast();
        // SAFETY: GC 疏散期调用契约同上
        unsafe { update_forwarded_slot(object, &record.original_string) };
      }
      let forward = record.forward_string_or_hash_object();
      if forward.is_heap_object() {
        let object: Tagged<HeapObject> = forward.cast();
        // SAFETY: GC 疏散期调用契约同上
        unsafe { update_forwarded_slot(object, &record.forward_string_or_hash) };
      }
    }
  }
}

/// 槽位随 map word 转发地址更新 (cc:76 UpdateForwardedSlot)。
///
/// # SAFETY (调用方)
/// object 须指向有效堆对象首地址 (原版同样仅在 GC 疏散期调用);
/// 本函数只读对象首字 map 槽。
unsafe fn update_forwarded_slot(
  object: Tagged<HeapObject>,
  slot: &AtomicUsize,
) -> bool {
  // SAFETY: 调用方契约保证对象有效; 仅读首字
  let map = unsafe { (*object.as_ptr()).map };
  let map_word = MapWord::from_map(map);
  if map_word.is_forwarding_address() {
    slot.store(
      map_word.to_forwarding_address(object).ptr(),
      Ordering::Release,
    );
    true
  } else {
    false
  }
}

// ── BlockVector (string-forwarding-table-inl.h:263-298 + cc:137-160) ──
/// 块指针向量 (原子读 + 互斥写)。
struct BlockVector {
  blocks: Vec<AtomicPtr<Block>>,
  size: AtomicUsize,
}

impl BlockVector {
  fn new(capacity: usize) -> Self {
    Self {
      blocks: (0..capacity).map(|_| AtomicPtr::new(std::ptr::null_mut())).collect(),
      size: AtomicUsize::new(0),
    }
  }

  #[inline]
  const fn capacity(&self) -> usize {
    self.blocks.len()
  }

  #[inline]
  fn size(&self) -> usize {
    self.size.load(Ordering::Acquire)
  }

  /// 原子读块指针 (LoadBlock 的 Acquire_Load)。
  #[inline]
  fn load_block(&self, index: usize) -> *mut Block {
    debug_assert!(index < self.size());
    self.blocks[index].load(Ordering::Acquire)
  }

  /// 追加块 (AddBlock 的 Release_Store + size 递增)。
  fn add_block(&self, block: Box<Block>) {
    debug_assert!(self.size() < self.capacity());
    let index = self.size();
    self.blocks[index].store(Box::into_raw(block), Ordering::Release);
    self.size.store(index + 1, Ordering::Release);
  }

  /// 倍容增长 (BlockVector::Grow; 调用方持 grow 锁)。
  fn grow(data: &Self, capacity: usize) -> Self {
    let mut new_data = BlockVector::new(capacity);
    for (dst, src) in new_data
      .blocks
      .iter_mut()
      .zip(data.blocks.iter())
      .take(data.size())
    {
      dst.store(src.load(Ordering::Acquire), Ordering::Relaxed);
    }
    new_data.size.store(data.size(), Ordering::Release);
    new_data
  }

  /// 释放全部块 (Reset/析构路径; 当前向量独占块所有权)。
  fn free_blocks(&self) {
    for slot in &self.blocks[..self.size()] {
      let ptr = slot.load(Ordering::Relaxed);
      if !ptr.is_null() {
        // SAFETY: 块指针由 Box::into_raw 得来, 所有权唯一在当前向量
        unsafe { drop(Box::from_raw(ptr)) };
      }
    }
  }
}

// ── StringForwardingTable (string-forwarding-table.h:26) ──────────────
/// 并发分段字符串转发表。
pub struct StringForwardingTable {
  /// 当前块向量 (无锁 Acquire 读)。
  blocks: AtomicPtr<BlockVector>,
  /// 旧块向量保活链 (增长期并发读者可能仍持旧指针)。
  block_vector_storage: Mutex<Vec<NonNull<BlockVector>>>,
  next_free_index: AtomicI32,
  /// 增长锁 (锁粒度对齐原版 grow_mutex_)。
  grow_mutex: Mutex<()>,
}

impl Default for StringForwardingTable {
  #[inline]
  fn default() -> Self {
    Self::new()
  }
}

impl StringForwardingTable {
  /// 构造 (cc:162; isolate 参数推迟)。
  pub fn new() -> Self {
    let table = Self {
      blocks: AtomicPtr::new(std::ptr::null_mut()),
      block_vector_storage: Mutex::new(Vec::new()),
      next_free_index: AtomicI32::new(0),
      grow_mutex: Mutex::new(()),
    };
    table.initialize_block_vector();
    table
  }

  fn lock_storage(&self) -> MutexGuard<'_, Vec<NonNull<BlockVector>>> {
    self.block_vector_storage
      .lock()
      .unwrap_or_else(PoisonError::into_inner)
  }

  fn lock_grow(&self) -> MutexGuard<'_, ()> {
    self.grow_mutex.lock().unwrap_or_else(PoisonError::into_inner)
  }

  /// 初始化块向量 + 首块 (cc:174 InitializeBlockVector)。
  fn initialize_block_vector(&self) {
    let blocks = Box::new(BlockVector::new(INITIAL_BLOCK_VECTOR_CAPACITY));
    blocks.add_block(Block::new(INITIAL_BLOCK_SIZE as usize));
    // SAFETY: Box 目标地址一经分配不再移动; 指针在 storage 析构/Reset 前恒有效
    let ptr = Box::into_non_null(blocks);
    let mut storage = self.lock_storage();
    storage.push(ptr);
    self.blocks.store(ptr.as_ptr(), Ordering::Relaxed);
  }

  /// 记录数 (size; next_free_index 直读)。
  #[inline]
  #[must_use]
  pub fn size(&self) -> i32 {
    self.next_free_index.load(Ordering::SeqCst)
  }

  /// 是否空表 (empty)。
  #[inline]
  #[must_use]
  pub fn is_empty(&self) -> bool {
    self.size() == 0
  }

  /// 确保块存在, 必要时增长 (cc:183 EnsureCapacity; 双检锁)。
  fn ensure_capacity(&self, block_index: u32) -> *mut BlockVector {
    let blocks = self.blocks.load(Ordering::Acquire);
    if (block_index as usize) >= block_vector_size(blocks) {
      let _grow_guard = self.lock_grow();
      // 重读: 他线程可能已增长
      let mut blocks = self.blocks.load(Ordering::Relaxed);
      if (block_index as usize) >= block_vector_size(blocks) {
        if (block_index as usize) >= block_vector_capacity(blocks) {
          // 块向量倍容 → 保活旧向量 → Release 发布新向量
          // SAFETY: 持增长锁; 旧向量经 storage 保活不被释放
          let new_blocks =
            BlockVector::grow(unsafe { &*blocks }, block_vector_capacity(blocks) * 2);
          // SAFETY: 同 initialize_block_vector 的地址稳定性论证
          let ptr = Box::into_non_null(Box::new(new_blocks));
          let mut storage = self.lock_storage();
          storage.push(ptr);
          blocks = ptr.as_ptr();
          self.blocks.store(blocks, Ordering::Release);
        }
        let capacity = capacity_for_block(block_index) as usize;
        let new_block = Block::new(capacity);
        // SAFETY: blocks 有效且容量足够
        unsafe { (*blocks).add_block(new_block) };
      }
    }
    blocks
  }

  /// 登记内部化转发 (cc:209 AddForwardString), 返回新记录索引。
  pub fn add_forward_string(
    &self,
    string: Tagged<Object>,
    forward_to: Tagged<Object>,
  ) -> i32 {
    let index = self.next_free_index.fetch_add(1, Ordering::SeqCst);
    let (block, index_in_block) = block_for_index(index);
    let blocks = self.ensure_capacity(block);
    // SAFETY: ensure_capacity 保证块存在; 块在 Reset 前不释放
    let block = unsafe { &*(*blocks).load_block(block as usize) };
    block.record(index_in_block as usize).set_internalized(string, forward_to);
    index
  }

  /// 登记外部化转发 (cc:236 AddExternalResourceAndHash), 返回新记录索引。
  pub fn add_external_resource_and_hash(
    &self,
    string: Tagged<Object>,
    resource: usize,
    is_one_byte: bool,
    raw_hash: u32,
  ) -> i32 {
    let index = self.next_free_index.fetch_add(1, Ordering::SeqCst);
    let (block, index_in_block) = block_for_index(index);
    let blocks = self.ensure_capacity(block);
    // SAFETY: 同上
    let block = unsafe { &*(*blocks).load_block(block as usize) };
    block
      .record(index_in_block as usize)
      .set_external(string, resource, is_one_byte, raw_hash);
    index
  }

  /// 覆写转发串 (cc:225 UpdateForwardString)。
  pub fn update_forward_string(
    &self,
    index: i32,
    forward_to: Tagged<Object>,
  ) -> Result<(), Error> {
    let record = self.record_for(index)?;
    record.set_forward_string(forward_to);
    Ok(())
  }

  /// 尝试覆写外部资源 (cc:264 TryUpdateExternalResource)。
  pub fn try_update_external_resource(
    &self,
    index: i32,
    resource: usize,
    is_one_byte: bool,
  ) -> Result<bool, Error> {
    let record = self.record_for(index)?;
    Ok(record.try_update_external_resource(resource, is_one_byte))
  }

  /// 读转发串 (cc:285 GetForwardString)。
  pub fn get_forward_string(
    &self,
    index: i32,
  ) -> Result<Tagged<Object>, Error> {
    Ok(self.record_for(index)?.forward_string())
  }

  /// 读裸哈希 (cc:303 GetRawHash)。
  pub fn get_raw_hash(&self, index: i32) -> Result<RawHash, Error> {
    Ok(self.record_for(index)?.raw_hash())
  }

  /// 读外部资源 (cc:318 GetExternalResource)。
  pub fn get_external_resource(
    &self,
    index: i32,
  ) -> Result<(usize, bool), Error> {
    Ok(self.record_for(index)?.external_resource())
  }

  /// 索引 → 记录引用 (越界 Err; 原版 CHECK_LT)。
  fn record_for(&self, index: i32) -> Result<&Record, Error> {
    if index < 0 || index >= self.size() {
      return Err(index_out_of_range());
    }
    let (block, index_in_block) = block_for_index(index);
    let blocks = self.blocks.load(Ordering::Acquire);
    // SAFETY: index < size 保证该块已分配; 块在 Reset 前不释放
    let block = unsafe { &*(*blocks).load_block(block as usize) };
    Ok(block.record(index_in_block as usize))
  }

  /// 遍历全部已用记录 (inl.h:333 IterateElements; 末块按用量截断)。
  pub fn iterate_elements(&self, mut callback: impl FnMut(&Record)) {
    if self.is_empty() {
      return;
    }
    let blocks = self.blocks.load(Ordering::Relaxed);
    // SAFETY: Reset 前块向量有效
    let size = unsafe { (*blocks).size() };
    let last_block_index = size - 1;
    for block_index in 0..last_block_index {
      // SAFETY: 同上
      let block = unsafe { &*(*blocks).load_block(block_index) };
      for index in 0..block.capacity() {
        callback(block.record(index));
      }
    }
    // 末块未填满: 按实际用量截断
    let max_index = index_in_block(self.size() - 1, last_block_index as u32) + 1;
    // SAFETY: 同上
    let block = unsafe { &*(*blocks).load_block(last_block_index) };
    for index in 0..max_index as usize {
      callback(block.record(index));
    }
  }

  /// 关停: 释放全部外部资源并复位 (cc:328 TearDown; Dispose 调用 → 回调)。
  pub fn teardown(&mut self, mut dispose: impl FnMut(usize)) {
    // 去重集合: 原版 unordered_set; 本侧 Vec 去重 (资源数小, 且确定性迭代)
    let mut disposed: Vec<usize> = Vec::new();
    self.iterate_elements(|record| {
      if record.original_string_object().ptr() != DELETED_PTR {
        let resource = record.external_resource_address();
        if resource != UNUSED_PTR && !disposed.contains(&resource) {
          dispose(resource);
          disposed.push(resource);
        }
      }
    });
    self.reset();
  }

  /// 复位 (cc:342 Reset; safepoint 契约: 调用方须在 GC 停止点,
  /// 保证无并发读者持有块指针 — 原版断言推迟登记)。
  pub fn reset(&mut self) {
    let blocks = *self.blocks.get_mut();
    if !blocks.is_null() {
      // SAFETY: 独占引用, 块所有权唯一在当前向量
      unsafe { (*blocks).free_blocks() };
    }
    {
      let storage = self
        .block_vector_storage
        .get_mut()
        .unwrap_or_else(PoisonError::into_inner);
      // SAFETY: 元素均由 Box::into_non_null 得来, 所有权唯一在 storage
      for ptr in storage.drain(..) {
        unsafe { drop(Box::from_raw(ptr.as_ptr())) };
      }
    }
    *self.next_free_index.get_mut() = 0;
    self.initialize_block_vector();
  }

  /// 年轻代疏散后修正 (cc:356; in_from_page = Heap::InFromPage 回调,
  /// 堆页归属判定推迟 L3 heap 层)。
  pub fn update_after_young_evacuation(
    &self,
    in_from_page: impl Fn(Tagged<HeapObject>) -> bool,
  ) {
    if self.is_empty() {
      return;
    }
    let blocks = self.blocks.load(Ordering::Relaxed);
    // SAFETY: Reset 前有效
    let size = unsafe { (*blocks).size() };
    let last_block_index = size - 1;
    let predicate = &in_from_page;
    for block_index in 0..last_block_index {
      // SAFETY: 同上
      let block = unsafe { &*(*blocks).load_block(block_index) };
      block.update_after_young_evacuation(block.capacity(), predicate);
    }
    let max_index = index_in_block(self.size() - 1, last_block_index as u32) + 1;
    // SAFETY: 同上
    let block = unsafe { &*(*blocks).load_block(last_block_index) };
    block.update_after_young_evacuation(max_index as usize, predicate);
  }

  /// 全量疏散后修正 (cc:377)。
  pub fn update_after_full_evacuation(&self) {
    if self.is_empty() {
      return;
    }
    let blocks = self.blocks.load(Ordering::Relaxed);
    // SAFETY: Reset 前有效
    let size = unsafe { (*blocks).size() };
    let last_block_index = size - 1;
    for block_index in 0..last_block_index {
      // SAFETY: 同上
      let block = unsafe { &*(*blocks).load_block(block_index) };
      block.update_after_full_evacuation(block.capacity());
    }
    let max_index = index_in_block(self.size() - 1, last_block_index as u32) + 1;
    // SAFETY: 同上
    let block = unsafe { &*(*blocks).load_block(last_block_index) };
    block.update_after_full_evacuation(max_index as usize);
  }
}

/// 块向量已用块数 (size 断言安全读)。
#[inline]
fn block_vector_size(blocks: *mut BlockVector) -> usize {
  debug_assert!(!blocks.is_null());
  // SAFETY: 调用方保证指针来自 blocks_ 且 Reset 前有效
  unsafe { (*blocks).size() }
}

/// 块向量容量。
#[inline]
fn block_vector_capacity(blocks: *mut BlockVector) -> usize {
  debug_assert!(!blocks.is_null());
  // SAFETY: 同上
  unsafe { (*blocks).capacity() }
}

impl Drop for StringForwardingTable {
  /// 析构 (cc:167): 释放当前向量的全部块。
  fn drop(&mut self) {
    let blocks = *self.blocks.get_mut();
    if !blocks.is_null() {
      // SAFETY: 独占引用
      unsafe { (*blocks).free_blocks() };
    }
  }
}
`;

writeSrc("v8-objects-strings", "string_forwarding_table.rs", body);
console.log(
  `string_forwarding_table.rs: 块位运算 (HighestBit=${kHighestBit}) + Record 原子族 + 双检锁增长; ` +
    "Dispose/safepoint 回调化登记",
);
