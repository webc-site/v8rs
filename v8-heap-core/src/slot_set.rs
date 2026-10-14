//! 类型化槽集 (TypedSlots / TypedSlotSet)。
//! 对应原版 v8/src/heap/slot-set.h (398 行) + slot-set.cc (103 行)。
//!
//! 收编说明 (W10HCb, .skel 第一批挂载):
//! - 枚举/布局常量面 (slot-set.h:234-266 SlotType; :131 kBucketsRegularPage) 承原
//!   slot_set.rs 生成面; kCellsPerBucket/kBitsPerCell 主在 basic-slot-set.h:277/:281
//!   (slot-set.h:132 仅消费), 本模块直承 v8-heap-base 导出, 不复刻避免配置漂移;
//!   kPageSizeBits 同理承 v8-common (主 build_config.h:68/74/80)。
//! - 机械规则面 (位域编码/容量增长/区间命中谓词) 承 slot_set_cc.skel 生成面。
//! - TypedSlots / TypedSlotSet 数据结构体 1:1 复刻 slot-set.{h,cc}: 分块 (Chunk) 链
//!   的插入/合并/扩容/区间清理。原版以裸指针单链 + std::vector 承接, Rust 以
//!   `VecDeque<Chunk>` (头→尾序) 等价建模, 全程安全码, 无 unsafe。
//! - 推迟面 (readme 台账 slot_set 组): PossiblyEmptyBuckets (slot-set.h:35-122) 与
//!   SlotSet 位图页类 (slot-set.h:127-232, 依赖页 bitmap / MemoryChunk, HC-4) 及
//!   TypedSlotSet 并发原子迭代 (LoadHead/StoreNext relaxed atomic, h:377-386,
//!   GC 线程并发读) → 本模块取单线程语义, 并发原子面待页实体票接入。

use std::collections::VecDeque;

use v8_common::{Address, K_PAGE_SIZE_BITS, K_TAGGED_SIZE};
use v8_heap_base::{K_BITS_PER_CELL, K_CELLS_PER_BUCKET, SlotCallbackResult};

// ══════════════════════════════════════════════════════════════════════════
// 枚举面 (slot-set.h:234-266 / :310)
// ══════════════════════════════════════════════════════════════════════════

/// 槽类型 (slot-set.h:234 `enum class SlotType : uint8_t`)。
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SlotType {
  /// h:238 kEmbeddedObjectFull, 指令流内整指针对象起始。
  EmbeddedObjectFull = 0,
  /// h:243 kEmbeddedObjectCompressed, 指令流内压缩指针对象起始。
  EmbeddedObjectCompressed = 1,
  /// h:248 kCodeEntry, 指令流内 Code 对象指令起点。
  CodeEntry = 2,
  /// h:253 kConstPoolEmbeddedObjectFull, 常量池内整指针对象起始。
  ConstPoolEmbeddedObjectFull = 3,
  /// h:256 kConstPoolEmbeddedObjectCompressed, 常量池内压缩指针对象起始。
  ConstPoolEmbeddedObjectCompressed = 4,
  /// h:260 kConstPoolCodeEntry, 常量池内整指针 Code 指令起点。
  ConstPoolCodeEntry = 5,
  /// h:263 kCleared, 槽已清但未从槽集摘除 (kLast)。
  Cleared = 6,
}

/// 槽块迭代模式 (slot-set.h:310 `enum IterationMode`)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IterationMode {
  /// h:310 FREE_EMPTY_CHUNKS: 迭代后释放空块。
  FreeEmptyChunks = 0,
  /// h:310 KEEP_EMPTY_CHUNKS: 保留空块。
  KeepEmptyChunks = 1,
}

// ══════════════════════════════════════════════════════════════════════════
// 布局常量面 (slot-set.h)
// ══════════════════════════════════════════════════════════════════════════

/// 每桶单元数 / 每单元位数: 承 v8-heap-base basic_slot_set 同源常量
/// (主 basic-slot-set.h:277/:281, slot-set.h:132 仅消费), 不在此复刻避免漂移。
/// 常规页桶数 (h:131-132 kBucketsRegularPage = (1<<kPageSizeBits)/kTaggedSize/kCellsPerBucket/kBitsPerCell)。
pub const BUCKETS_REGULAR_PAGE: usize =
  (1 << K_PAGE_SIZE_BITS) / K_TAGGED_SIZE / K_CELLS_PER_BUCKET / K_BITS_PER_CELL;

/// TypedSlots::kMaxOffset (h:277 `1 << 29`) — 偏移可达上界 (不含)。
pub const K_MAX_OFFSET_EXCL: u32 = 1 << 29;
/// 偏移域位宽 (OffsetField<int, 0, 29>, h:284)。
pub const K_OFFSET_BITS: u32 = 29;
/// 类型域起始位 (TypeField<SlotType, 29, 3>, h:285)。
pub const K_TYPE_SHIFT: u32 = 29;
/// 类型域位宽。
pub const K_TYPE_BITS: u32 = 3;
/// 偏移域掩码 (29 位全 1, 含界)。
pub const K_MAX_OFFSET: u32 = (1 << 29) - 1;
/// 初块容量 (h:293 kInitialBufferSize)。
pub const K_INITIAL_BUFFER_SIZE: usize = 100;
/// 块容量上限 (h:294 kMaxBufferSize = 16 * KB)。
pub const K_MAX_BUFFER_SIZE: usize = 16 * 1024;
/// kCleared 判别值 (SlotType 第 7 员)。
pub const K_SLOT_TYPE_CLEARED: u8 = SlotType::Cleared as u8;

// ══════════════════════════════════════════════════════════════════════════
// 机械规则面 (位域编码/容量增长/区间命中)
// ══════════════════════════════════════════════════════════════════════════

/// TypedSlot 编码 (slot-set.cc:25 `TypeField::encode(type) | OffsetField::encode(offset)`)。
/// 原版 BitField::encode: 枚举类型域 DCHECK(is_valid) (bit-field.h:91), 整型偏移域
/// CHECK(is_valid) (bit-field.h:100) — 越界即硬失败, 从不静默截断; 此处按本仓
/// CHECK→debug_assert 口径断言前置, release 保留掩蔽避免污染类型域。
/// v8: src/heap/slot-set.cc:25 TypedSlots::Insert
#[must_use]
pub const fn typed_slot_encode(slot_type: u8, offset: u32) -> u32 {
  debug_assert!(
    slot_type <= K_SLOT_TYPE_CLEARED,
    "bit-field.h:91 DCHECK: SlotType 须落 3 位类型域 (<= kLast)"
  );
  debug_assert!(
    offset < K_MAX_OFFSET_EXCL,
    "bit-field.h:100 CHECK: 偏移须 < kMaxOffset (1<<29)"
  );
  ((slot_type as u32) << K_TYPE_SHIFT) | (offset & K_MAX_OFFSET)
}

/// 类型域解码 (TypeField::decode)。
/// v8: src/heap/slot-set.cc:85 TypedSlotSet::IterateSlotsInRanges
#[must_use]
pub const fn typed_slot_decode_type(type_and_offset: u32) -> u8 {
  (type_and_offset >> K_TYPE_SHIFT) as u8
}

/// 偏移域解码 (OffsetField::decode)。
/// v8: src/heap/slot-set.cc:87 TypedSlotSet::IterateSlotsInRanges
#[must_use]
pub const fn typed_slot_decode_offset(type_and_offset: u32) -> u32 {
  type_and_offset & K_MAX_OFFSET
}

/// ClearedTypedSlot 编码值 (h:387-389 `TypeField::encode(kCleared) | OffsetField::encode(0)`)。
/// v8: src/heap/slot-set.h:387 TypedSlotSet::ClearedTypedSlot
#[must_use]
pub const fn cleared_typed_slot() -> u32 {
  typed_slot_encode(K_SLOT_TYPE_CLEARED, 0)
}

/// 新块容量 (h:295-297 `NextCapacity = std::min({kMaxBufferSize, capacity * 2})`)。
/// v8: src/heap/slot-set.h:295 TypedSlots::NextCapacity
#[must_use]
pub const fn next_capacity(capacity: usize) -> usize {
  if K_MAX_BUFFER_SIZE < capacity * 2 {
    K_MAX_BUFFER_SIZE
  } else {
    capacity * 2
  }
}

/// 槽偏移是否落入失效区间 — IterateSlotsInRanges 命中判定 (cc:88-94)。
/// 原版 std::map::upper_bound(offset) 回退一格比终点 (区间终点排他); ranges 须按
/// 起点升序 (承接 std::map 键序), 手动二分定位首个起点大于 offset 的区间 (const 兼容)。
/// v8: src/heap/slot-set.cc:78 TypedSlotSet::IterateSlotsInRanges
#[must_use]
pub const fn slot_in_ranges(ranges: &[(u32, u32)], offset: u32) -> bool {
  // partition_point 语义: 首个 start > offset 的下标 (等价 map.upper_bound)。
  let mut lo = 0usize;
  let mut hi = ranges.len();
  while lo < hi {
    let mid = lo + (hi - lo) / 2;
    if ranges[mid].0 <= offset {
      lo = mid + 1;
    } else {
      hi = mid;
    }
  }
  if lo == 0 {
    return false;
  }
  // 回退一格的区间终点严格大于 offset 即命中。
  ranges[lo - 1].1 > offset
}

/// 区间入参不变式: 起点升序 (可重叠)。原版容器为 `std::map<uint32_t,uint32_t>`
/// (FreeRangesMap, h:308), 键序天然有序, cc:88 `upper_bound` 回退一格只对有序键成立;
/// 本仓以切片承接, 乱序输入会静默错判命中 → debug 档拦截, release 行为与原版一致地
/// 不额外兜底 (原版亦无重排)。
fn debug_assert_sorted(ranges: &[(u32, u32)]) {
  debug_assert!(
    ranges.windows(2).all(|w| w[0].0 <= w[1].0),
    "cc:88 upper_bound 要求 invalid_ranges 按起点升序"
  );
}

// ══════════════════════════════════════════════════════════════════════════
// TypedSlots 分块链 (h:275-302 + cc:13-62)
// ══════════════════════════════════════════════════════════════════════════

/// 槽块 (h:289-292 `struct Chunk { Chunk* next; std::vector<TypedSlot> buffer; }`)。
/// 以 `buffer` + `cap` (reserve 后的容量, 承接 vector.capacity()) 等价建模。
struct Chunk {
  /// 编码槽缓冲 (type_and_offset u32)。
  buffer: Vec<u32>,
  /// reserve 容量 (h:60 DCHECK_EQ capacity)。
  cap: usize,
}

/// 类型化槽链 (h:275-302 `class TypedSlots`) — 分块头→尾单链, 头插扩容尾保留。
/// 原版 head_/tail_ 裸指针链, 此处以 `VecDeque` 头→尾序承接 (front=head, back=tail)。
pub struct TypedSlots {
  /// 头→尾序的槽块。
  chunks: VecDeque<Chunk>,
}

impl TypedSlots {
  /// 原版 h:278 `TypedSlots() = default` (空链)。
  /// v8: src/heap/slot-set.h:278 TypedSlots::TypedSlots
  #[must_use]
  pub fn new() -> TypedSlots {
    TypedSlots {
      chunks: VecDeque::new(),
    }
  }

  /// 原版 cc:24-29 `Insert(type, offset)` — 编码入当前头块。
  /// v8: src/heap/slot-set.cc:24 TypedSlots::Insert
  pub fn insert(&mut self, slot_type: SlotType, offset: u32) {
    let slot = typed_slot_encode(slot_type as u8, offset);
    self.ensure_chunk();
    // ensure_chunk 后头块恒存在且有空间; front_mut 取头块 (None 仅本不变式被破坏时,
    // debug_assert 兜底, release 走安全无 panic 路径)。
    debug_assert!(!self.chunks.is_empty(), "cc:26 EnsureChunk 后头块必存在");
    if let Some(chunk) = self.chunks.front_mut() {
      // cc:27 DCHECK_LT(chunk->buffer.size(), chunk->buffer.capacity())。
      debug_assert!(chunk.buffer.len() < chunk.cap, "cc:27 头块满");
      chunk.buffer.push(slot);
    }
  }

  /// 原版 cc:46-54 `EnsureChunk()` — 空则建初块; 头满则头插扩容新块。
  /// 头插 (VecDeque::push_front) 使新块成 head, 旧块顺延, 尾保留 (承接 head 前移,
  /// tail 恒初块)。
  /// v8: src/heap/slot-set.cc:46 TypedSlots::EnsureChunk
  fn ensure_chunk(&mut self) {
    if self.chunks.is_empty() {
      self
        .chunks
        .push_front(Self::new_chunk(K_INITIAL_BUFFER_SIZE));
      return;
    }
    // 头满: 取旧头容量算 NextCapacity, 头插扩容新块 (next=旧头, VecDeque 前置即成 head)。
    let grow_to = match self.chunks.front() {
      Some(c) if c.buffer.len() == c.cap => Some(next_capacity(c.cap)),
      _ => None,
    };
    if let Some(cap) = grow_to {
      self.chunks.push_front(Self::new_chunk(cap));
    }
  }

  /// 原版 cc:56-62 `NewChunk(next, capacity)` — 新块 reserve capacity。
  /// v8: src/heap/slot-set.cc:56 TypedSlots::NewChunk
  fn new_chunk(capacity: usize) -> Chunk {
    // 承接 chunk->buffer.reserve(capacity): Vec 预置容量, cap 记录逻辑容量。
    Chunk {
      buffer: Vec::with_capacity(capacity),
      cap: capacity,
    }
  }

  /// 原版 cc:31-44 `Merge(other)` — 他链尾接己尾, 他清空。
  /// 迭代序 (head→tail) 保证: 己块全先于他块 (承接 tail_->next = other->head)。
  /// v8: src/heap/slot-set.cc:31 TypedSlots::Merge
  pub fn merge(&mut self, other: &mut TypedSlots) {
    if other.chunks.is_empty() {
      return;
    }
    // 己空: 直接接管他链; 己非空: 他链顺序追加至己尾 (back)。
    self.chunks.append(&mut other.chunks);
  }

  /// 头→尾序遍历全部编码槽 (迭代基座 h:327-359 的 head→next 链走法, cc:13-22 系析构
  /// 链同序; 供子类复用)。
  fn iter_slots(&self) -> impl Iterator<Item = &u32> {
    self.chunks.iter().flat_map(|c| c.buffer.iter())
  }
}

impl Default for TypedSlots {
  /// 承接 `TypedSlots() = default`。
  fn default() -> Self {
    Self::new()
  }
}

// ══════════════════════════════════════════════════════════════════════════
// TypedSlotSet (h:304-390 + cc:64-100)
// ══════════════════════════════════════════════════════════════════════════

/// 按页类型化槽的多重集 (h:304-390 `class TypedSlotSet : public TypedSlots`)。
/// 支持迭代清理失效槽; page_start_ 承接 h:312。
pub struct TypedSlotSet {
  /// 槽存储基 (h:392 page_start_; addr = page_start + offset)。
  page_start: Address,
  /// 基类 TypedSlots (h:306 继承 → Rust 组合)。
  slots: TypedSlots,
}

impl TypedSlotSet {
  /// 原版 h:312 `explicit TypedSlotSet(Address page_start)`。
  /// v8: src/heap/slot-set.h:312 TypedSlotSet::TypedSlotSet
  #[must_use]
  pub fn new(page_start: Address) -> TypedSlotSet {
    TypedSlotSet {
      page_start,
      slots: TypedSlots::new(),
    }
  }

  /// 承接基类 Insert (h:280)。
  /// v8: src/heap/slot-set.cc:24 TypedSlots::Insert
  pub fn insert(&mut self, slot_type: SlotType, offset: u32) {
    self.slots.insert(slot_type, offset);
  }

  /// 承接基类 Merge (h:281)。
  /// v8: src/heap/slot-set.cc:31 TypedSlots::Merge
  pub fn merge(&mut self, other: &mut TypedSlotSet) {
    self.slots.merge(&mut other.slots);
  }

  /// 原版 h:324-362 `Iterate(callback, mode) -> int` — 逐槽回调, REMOVE 置 cleared;
  /// FREE_EMPTY_CHUNKS 且块空则摘块。返回保留槽数。
  /// v8: src/heap/slot-set.h:325 TypedSlotSet::Iterate
  pub fn iterate(
    &mut self,
    mut callback: impl FnMut(SlotType, Address) -> SlotCallbackResult,
    mode: IterationMode,
  ) -> i32 {
    let mut new_count = 0i32;
    // 头→尾序 (h:327 chunk=head_; :359 chunk=next)。逐块重建, 承接 h:346 摘块逻辑。
    let mut kept: VecDeque<Chunk> = VecDeque::with_capacity(self.slots.chunks.len());
    for mut chunk in self.slots.chunks.drain(..) {
      let mut empty = true;
      for slot in chunk.buffer.iter_mut() {
        let ty = typed_slot_decode_type(*slot);
        if ty != K_SLOT_TYPE_CLEARED {
          let offset = typed_slot_decode_offset(*slot);
          let addr = self.page_start + offset as Address;
          if matches!(
            callback(slot_type_from_u8(ty), addr),
            SlotCallbackResult::KeepSlot
          ) {
            new_count += 1;
            empty = false;
          } else {
            *slot = cleared_typed_slot();
          }
        }
      }
      // h:346-358: 空块在 FREE_EMPTY_CHUNKS 下摘除 (delete), 否则保留。
      if mode == IterationMode::FreeEmptyChunks && empty {
        continue;
      }
      kept.push_back(chunk);
    }
    self.slots.chunks = kept;
    new_count
  }

  /// 原版 cc:64-67 `ClearInvalidSlots(invalid_ranges)` — 命中区间槽置 cleared。
  /// v8: src/heap/slot-set.cc:64 TypedSlotSet::ClearInvalidSlots
  pub fn clear_invalid_slots(&mut self, invalid_ranges: &[(u32, u32)]) {
    self.iterate_slots_in_ranges(invalid_ranges, |slot| {
      *slot = cleared_typed_slot();
    });
  }

  /// 原版 cc:69-75 `AssertNoInvalidSlots(invalid_ranges)` — 命中即违例 (CHECK→debug_assert)。
  /// v8: src/heap/slot-set.cc:69 TypedSlotSet::AssertNoInvalidSlots
  pub fn assert_no_invalid_slots(&self, invalid_ranges: &[(u32, u32)]) {
    debug_assert_sorted(invalid_ranges);
    let mut hit = false;
    for &slot in self.slots.iter_slots() {
      if typed_slot_decode_type(slot) == K_SLOT_TYPE_CLEARED {
        continue;
      }
      if slot_in_ranges(invalid_ranges, typed_slot_decode_offset(slot)) {
        hit = true;
      }
    }
    debug_assert!(!hit, "cc:72 CHECK: no slot in ranges expected");
  }

  /// 原版 cc:77-100 `IterateSlotsInRanges(callback, ranges)` — 对命中区间非 cleared 槽回调。
  /// v8: src/heap/slot-set.cc:77 TypedSlotSet::IterateSlotsInRanges
  fn iterate_slots_in_ranges(&mut self, ranges: &[(u32, u32)], mut callback: impl FnMut(&mut u32)) {
    if ranges.is_empty() {
      return;
    }
    debug_assert_sorted(ranges);
    // cc:80 ranges.empty 早退后, 逐块逐槽: cleared 跳过 (cc:86), upper_bound 回退命中判定 (cc:88-96)。
    for chunk in self.slots.chunks.iter_mut() {
      for slot in chunk.buffer.iter_mut() {
        if typed_slot_decode_type(*slot) == K_SLOT_TYPE_CLEARED {
          continue;
        }
        if slot_in_ranges(ranges, typed_slot_decode_offset(*slot)) {
          callback(slot);
        }
      }
    }
  }
}

/// u8 判别值 → SlotType (承接 TypeField::decode 回枚举; 域 3 位, 值 < 8)。
#[must_use]
const fn slot_type_from_u8(v: u8) -> SlotType {
  match v {
    0 => SlotType::EmbeddedObjectFull,
    1 => SlotType::EmbeddedObjectCompressed,
    2 => SlotType::CodeEntry,
    3 => SlotType::ConstPoolEmbeddedObjectFull,
    4 => SlotType::ConstPoolEmbeddedObjectCompressed,
    5 => SlotType::ConstPoolCodeEntry,
    _ => SlotType::Cleared,
  }
}

// ══════════════════════════════════════════════════════════════════════════
// 编译期不变式 (机械规则面对数)
// ══════════════════════════════════════════════════════════════════════════

const _: () = {
  // 复用 heap-base 常量前对 basic-slot-set.h:277/:281 (slot-set.h:132 消费) 的本头假设交叉核对。
  assert!(K_CELLS_PER_BUCKET == 32);
  assert!(K_BITS_PER_CELL == 32);
  assert!(K_OFFSET_BITS == 29);
  assert!(K_TYPE_SHIFT == 29);
  assert!(K_TYPE_BITS == 3);
  assert!(K_MAX_OFFSET == (1 << 29) - 1);
  assert!(K_MAX_OFFSET_EXCL == (1 << 29));
  assert!(K_INITIAL_BUFFER_SIZE == 100);
  assert!(K_MAX_BUFFER_SIZE == 16_384);
  assert!(K_SLOT_TYPE_CLEARED == 6);
  assert!(cleared_typed_slot() == 0xc000_0000);
  assert!(typed_slot_encode(6, 0) == 0xc000_0000);
  assert!(typed_slot_encode(2, 0x1234_5678) == 0x5234_5678);
  assert!(typed_slot_decode_type(0xc000_0000) == 6);
  assert!(typed_slot_decode_offset(0xc000_0000) == 0);
  assert!(typed_slot_decode_offset(0x9234_5678) == 0x1234_5678);
  assert!(next_capacity(100) == 200);
  assert!(next_capacity(9_000) == 16_384);
  assert!(next_capacity(16_384) == 16_384);
  // upper_bound 回退语义 (终点排他)。
  assert!(!slot_in_ranges(&[(10u32, 20u32), (30, 40)], 5));
  assert!(slot_in_ranges(&[(10u32, 20u32), (30, 40)], 10));
  assert!(slot_in_ranges(&[(10u32, 20u32), (30, 40)], 19));
  assert!(!slot_in_ranges(&[(10u32, 20u32), (30, 40)], 20));
  assert!(!slot_in_ranges(&[(10u32, 20u32), (30, 40)], 21));
  assert!(slot_in_ranges(&[(10u32, 20u32), (30, 40)], 39));
  assert!(!slot_in_ranges(&[(10u32, 20u32), (30, 40)], 40));
};
