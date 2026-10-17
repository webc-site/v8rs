// 由 tool/src/objects/number-string-cache.h.js 从 v8/src/objects/number-string-cache.{h,-inl.h,cc} 生成; 修订就地改, 复跑覆盖
// Smi/Double 双缓存槽语义 1:1; 堆分配/flags/isolate 段参数化或推迟 (登记见脚本头)

//! 数字 → 字符串缓存: Smi 缓存与裸 double 位型缓存。
//!
//! 原版为堆上对象 (FixedArray / 变长 HeapObject); 本模块落地槽位语义
//! (entry 定位/命中/覆写/full-size 切换/清空/计数), 堆分配与 isolate
//! 绑定推迟 (登记见脚本头)。

use v8_common::{
  K_MAX_REGULAR_HEAP_OBJECT_SIZE, K_TAGGED_SIZE, K_UINT32_SIZE, object_pointer_align,
};
use v8_objects_core::{
  smi::Smi,
  tagged::{Object, Tagged},
};

// ── SmiStringCache 常量 (number-string-cache.h:27-37) ─────────────────
/// 条目键下标 (kEntryKeyIndex)。
pub const ENTRY_KEY_INDEX: usize = 0;
/// 条目值下标 (kEntryValueIndex)。
pub const ENTRY_VALUE_INDEX: usize = 1;
/// 每条目槽数 (kEntrySize)。
pub const ENTRY_SIZE: usize = 2;
/// 初始容量 (kInitialSize)。
pub const INITIAL_SIZE: u32 = 128;

// ── DoubleStringCache 常量 (number-string-cache.h:91-92) ──────────────
/// 初始容量 (DoubleStringCache::kInitialSize)。
pub const DOUBLE_INITIAL_SIZE: u32 = 128;
/// 最大容量 (kMaxCapacity; 本仓默认 kTaggedSize=4 → 0x2000)。
pub const DOUBLE_MAX_CAPACITY: u32 = 0x2000;
/// 单条目字节数 (sizeof(Entry) = 8 非对齐 double + 4 tagged)。
pub const SIZE_OF_ENTRY: usize = 12;

// 原版 static_assert: SizeFor(kMaxCapacity) < kMaxRegularHeapObjectSize
const _: () =
  assert!(double_size_for(DOUBLE_MAX_CAPACITY as usize) < K_MAX_REGULAR_HEAP_OBJECT_SIZE);

/// 值槽空哨兵 (kEmptySentinel = Smi::zero; 键值通用)。
#[inline]
#[must_use]
pub const fn empty_sentinel() -> Tagged<Smi> {
  Smi::zero()
}

// ── SmiStringCache (number-string-cache.h:20-74) ──────────────────────
/// 非零 Smi → String 缓存 (键值交错槽语义 1:1)。
pub struct SmiStringCache {
  capacity: u32,
  /// (键, 值) 槽; 空槽两者均 = kEmptySentinel。
  slots: Vec<(Tagged<Smi>, Tagged<Object>)>,
}

impl SmiStringCache {
  /// 构造并清空 (New + Clear; 容量须为 2 的幂 — 掩码定位前提)。
  #[must_use]
  pub fn new(capacity: u32) -> Self {
    debug_assert!(capacity.is_power_of_two());
    Self {
      capacity,
      slots: vec![(Smi::zero(), Tagged::new(0)); capacity as usize],
    }
  }

  /// 容量 (条目数)。
  #[inline]
  #[must_use]
  pub const fn capacity(&self) -> u32 {
    self.capacity
  }

  /// 清空全部条目 (Clear)。
  pub fn clear(&mut self) {
    self.slots.fill((Smi::zero(), Tagged::new(0)));
  }

  /// Smi → 条目下标 (inl.h:28 GetEntryFor: hash = value, 掩码定位)。
  #[inline]
  #[must_use]
  pub const fn get_entry_for(&self, number: Tagged<Smi>) -> usize {
    let hash = number.value() as u32;
    (hash & (self.capacity - 1)) as usize
  }

  /// 命中查询 (inl.h:41 Get): 键相等 → 值; 未命中 → None (原版 undefined,
  /// 登记映射)。
  #[must_use]
  pub fn get(&self, entry: usize, number: Tagged<Smi>) -> Option<Tagged<Object>> {
    let (key, value) = self.slots[entry];
    if key.ptr() == number.ptr() {
      Some(value)
    } else {
      None
    }
  }

  /// 直写条目 (inl.h:67 Set 的写段; 覆写旧条目)。
  ///
  /// number 不得为 Smi::zero (原版 DCHECK: 零值走预分配表)。
  pub fn set(&mut self, entry: usize, number: Tagged<Smi>, string: Tagged<Object>) {
    debug_assert_ne!(number.ptr(), empty_sentinel().ptr());
    self.slots[entry] = (number, string);
  }

  /// 带扩容切换的写入 (inl.h:78-92 Set 的 full-size 切换段):
  /// 初始容量 + 条目已占 + Some(full_size) → 重建全量表 (不复制条目,
  /// 原版 TODO 同款语义), 重算条目后写入; None = MemorySaver 模式 (直写)。
  ///
  /// 返回实际使用的条目下标。
  pub fn set_with_growth(
    &mut self,
    entry: usize,
    number: Tagged<Smi>,
    string: Tagged<Object>,
    full_size: Option<u32>,
  ) -> usize {
    debug_assert_ne!(number.ptr(), empty_sentinel().ptr());
    let occupied = self.slots[entry].0.ptr() != empty_sentinel().ptr();
    if self.capacity() == INITIAL_SIZE
      && occupied
      && let Some(full) = full_size
    {
      debug_assert!(INITIAL_SIZE < full);
      // 重建全量表 (原版不复制旧条目)
      *self = Self::new(full);
      let entry = self.get_entry_for(number);
      self.slots[entry] = (number, string);
      return entry;
    }
    self.slots[entry] = (number, string);
    entry
  }

  /// 已占条目计数 (cc:12 GetUsedEntriesCount)。
  #[must_use]
  pub fn get_used_entries_count(&self) -> u32 {
    self
      .slots
      .iter()
      .filter(|(key, _)| key.ptr() != empty_sentinel().ptr())
      .count() as u32
  }
}

// ── DoubleStringCache (number-string-cache.h:77-156) ──────────────────
/// 单条目: 裸 double 位型键 + 值槽。
#[derive(Clone, Copy, Debug)]
pub struct Entry {
  /// 键 (double 的位表示; 原版 UnalignedDoubleMember)。
  pub key_bits: u64,
  /// 值 (String 占位 Object; 空 = kEmptySentinel)。
  pub value: Tagged<Object>,
}

/// 裸 double 位型 → String 缓存。
pub struct DoubleStringCache {
  capacity: u32,
  entries: Vec<Entry>,
}

/// 条目数组字节偏移 (OffsetOfElementAt: header + capacity 字段 + index*Entry)。
#[inline]
#[must_use]
pub const fn double_offset_of_element_at(index: usize) -> usize {
  K_TAGGED_SIZE + K_UINT32_SIZE + index * SIZE_OF_ENTRY
}

/// 对象总大小 (SizeFor: OBJECT_POINTER_ALIGN 收尾)。
#[inline]
#[must_use]
pub const fn double_size_for(length: usize) -> usize {
  object_pointer_align(double_offset_of_element_at(length))
}

impl DoubleStringCache {
  /// 构造并清空 (New + Clear; 容量须为 2 的幂)。
  #[must_use]
  pub fn new(capacity: u32) -> Self {
    debug_assert!(capacity.is_power_of_two());
    debug_assert!(capacity >= DOUBLE_INITIAL_SIZE);
    Self {
      capacity,
      entries: vec![
        Entry {
          key_bits: 0,
          value: Tagged::new(0)
        };
        capacity as usize
      ],
    }
  }

  /// 容量。
  #[inline]
  #[must_use]
  pub const fn capacity(&self) -> u32 {
    self.capacity
  }

  /// 清空 (inl.h:213: 值槽全部置 kEmptySentinel; 键清零)。
  pub fn clear(&mut self) {
    self.entries.fill(Entry {
      key_bits: 0,
      value: Tagged::new(0),
    });
  }

  /// 裸位型 → 条目下标 (inl.h:116 GetEntryFor: hash = lo ^ hi)。
  #[inline]
  #[must_use]
  pub const fn entry_for_bits(number_bits: u64, capacity: u32) -> usize {
    let hash = (number_bits as u32) ^ ((number_bits >> 32) as u32);
    (hash & (capacity - 1)) as usize
  }

  /// 条目下标 (实例方法透传)。
  #[inline]
  #[must_use]
  pub fn get_entry_for(&self, number_bits: u64) -> usize {
    Self::entry_for_bits(number_bits, self.capacity)
  }

  /// 命中查询 (inl.h:135 Get: 位型键相等 → 值; 否则 None)。
  #[must_use]
  pub fn get(&self, entry: usize, number_bits: u64) -> Option<Tagged<Object>> {
    let e = &self.entries[entry];
    if e.key_bits == number_bits {
      Some(e.value)
    } else {
      None
    }
  }

  /// 直写条目 (inl.h:161 Set 的写段)。
  pub fn set(&mut self, entry: usize, number_bits: u64, string: Tagged<Object>) {
    self.entries[entry] = Entry {
      key_bits: number_bits,
      value: string,
    };
  }

  /// 带扩容切换的写入 (inl.h:169-184 full-size 切换; 语义同 Smi 版)。
  ///
  /// 返回实际使用的条目下标。
  pub fn set_with_growth(
    &mut self,
    entry: usize,
    number_bits: u64,
    string: Tagged<Object>,
    full_size: Option<u32>,
  ) -> usize {
    let occupied = self.entries[entry].value.ptr() != empty_sentinel().ptr();
    if self.capacity == DOUBLE_INITIAL_SIZE
      && occupied
      && let Some(full) = full_size
    {
      debug_assert!(DOUBLE_INITIAL_SIZE < full);
      debug_assert!(full <= DOUBLE_MAX_CAPACITY);
      *self = Self::new(full);
      let entry = self.get_entry_for(number_bits);
      self.entries[entry] = Entry {
        key_bits: number_bits,
        value: string,
      };
      return entry;
    }
    self.entries[entry] = Entry {
      key_bits: number_bits,
      value: string,
    };
    entry
  }

  /// 已占条目计数 (cc:39 GetUsedEntriesCount: 按值槽非空计)。
  #[must_use]
  pub fn get_used_entries_count(&self) -> u32 {
    self
      .entries
      .iter()
      .filter(|e| e.value.ptr() != empty_sentinel().ptr())
      .count() as u32
  }

  /// 分配大小 (AllocatedSize → SizeFor(capacity))。
  #[inline]
  #[must_use]
  pub const fn allocated_size(&self) -> usize {
    double_size_for(self.capacity as usize)
  }
}
