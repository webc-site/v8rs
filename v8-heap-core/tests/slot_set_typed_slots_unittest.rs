//! tests/slot_set_typed_slots_unittest.rs — src/slot_set.rs 分块链与迭代模式面对偶补测
//! (W10HCR round1 复审席新增)。
//!
//! 现行 slot_set_unittest.rs 复刻原版 slot-set-unittest.cc 三支用例, 但全部走
//! `KEEP_EMPTY_CHUNKS` + `page_start = 0` + 单块或少量槽, 关键路径存在盲区。本文件按
//! 原版 src/heap/slot-set.{h,cc} 逐语义补 1:1 用例 (禁零断言冒烟, 逐断言注原版 file:line):
//!   分块增长策略  cc:50-52 `head_ = NewChunk(head_, NextCapacity(...))` 头插 → 用例 A/B/C
//!   容量饱和       h:295-297 `std::min({kMaxBufferSize, capacity*2})`        → 用例 C
//!   迭代顺序       h:327/359 `chunk = head_` → `chunk = next` (头=最新块)      → 用例 A/B/D/E
//!   FREE_EMPTY_CHUNKS 摘块 h:346-355                                        → 用例 D
//!   KEEP_EMPTY_CHUNKS 保留 cleared 占位 (本体不清 buffer)                     → 用例 E
//!   new_count 返回值 h:338/361                                              → 用例 F
//!   地址基址 h:336 `page_start_ + offset`; 区间键为偏移非地址 (h:308)          → 用例 G
//!   空集/空区间边界 h:330 循环体 + cc:80 `ranges.empty()` 早退                  → 用例 H/I
//!   区间命中回退 quirk cc:88-94 (std::map upper_bound 回退, 重叠区间仅比末格)   → 用例 J
//!   AssertNoInvalidSlots cc:69-75 CHECK                                     → 用例 K
//!   BitField::encode 越界硬失败 bit-field.h:91/100 + 区间起点升序入参 (map 键序) → 用例 L/L3
//!   Merge 跨扩容链序 cc:35-41                                               → 用例 M/N
//! dev profile 承载 debug_assert (nextest 默认, 本仓 spaces_unittest should_panic 先例);
//! should_panic 形按 rustc 约束返回 `()`，常规用例保持本仓 Void/OK 尾。

use aok::{OK, Void};
use log::info;
use v8_common::Address;
use v8_heap_base::SlotCallbackResult;
use v8_heap_core::slot_set::{
  IterationMode, K_INITIAL_BUFFER_SIZE, K_MAX_BUFFER_SIZE, K_MAX_OFFSET_EXCL, SlotType,
  TypedSlotSet, typed_slot_encode,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 头→尾序收集全部存活槽偏移 (存活 = 非 kCleared, h:334 跳过清槽)。
fn live_offsets(set: &mut TypedSlotSet, page_start: Address) -> Vec<u32> {
  let mut out: Vec<u32> = Vec::new();
  set.iterate(
    |_ty: SlotType, addr: Address| -> SlotCallbackResult {
      out.push((addr - page_start) as u32);
      SlotCallbackResult::KeepSlot
    },
    IterationMode::KeepEmptyChunks,
  );
  out
}

/// 拼接升序区间序列, 便于逐块边界对账。
fn span(start: u32, end: u32) -> Vec<u32> {
  (start..end).collect()
}

// ══════════════════════════════════════════════════════════════════════════
// A/B/C 分块增长 + 迭代序 (cc:46-54 / h:295-297 / h:327)
// ══════════════════════════════════════════════════════════════════════════

/// 用例 A (cc:50-52 头插扩容): 初块 kInitialBufferSize=100 装满后, 第 101 槽落新头块,
/// 迭代从 head_ (最新块) 起 → 新块槽先于旧块槽。原版序非插入序, 此为 1:1 关键面。
#[test]
fn typed_slots_growth_prepends_head_chunk() -> Void {
  info!("> typed_slots_growth_prepends_head_chunk");
  const K_TOTAL: u32 = 105;
  let mut set = TypedSlotSet::new(0);
  for offset in 0..K_TOTAL {
    set.insert(SlotType::CodeEntry, offset);
  }

  let live = live_offsets(&mut set, 0);
  // cc:51 `head_ = NewChunk(head_, ...)` → 头块 = 第 100..104 槽, 次块 = 第 0..99 槽。
  let mut expected = span(K_INITIAL_BUFFER_SIZE as u32, K_TOTAL);
  expected.extend(span(0, K_INITIAL_BUFFER_SIZE as u32));
  assert_eq!(
    live, expected,
    "cc:50-52 头插后迭代须为 [新块 100..104] ++ [旧块 0..99]"
  );

  OK
}

/// 用例 B (h:295-297 倍增阶梯): 3100 槽成 5 块, 容量 100/200/400/800/1600,
/// 头→尾序 = 容量降序块序 (head_ 为最新最大块)。
#[test]
fn typed_slots_capacity_doubling_ladder() -> Void {
  info!("> typed_slots_capacity_doubling_ladder");
  // 100+200+400+800+1600 = 3100 → 恰填满至第 5 块。
  const K_TOTAL: u32 = 3100;
  let mut set = TypedSlotSet::new(0);
  for offset in 0..K_TOTAL {
    set.insert(SlotType::EmbeddedObjectFull, offset);
  }

  let live = live_offsets(&mut set, 0);
  // 块边界 (head→tail): [1500..3100) [700..1500) [300..700) [100..300) [0..100)。
  let blocks: &[(u32, u32)] = &[
    (1500, K_TOTAL),
    (700, 1500),
    (300, 700),
    (100, 300),
    (0, 100),
  ];
  let mut expected: Vec<u32> = Vec::with_capacity(K_TOTAL as usize);
  for &(start, end) in blocks {
    expected.extend(span(start, end));
  }
  assert_eq!(live.len(), expected.len(), "h:296 阶梯块总槽数");
  assert_eq!(live, expected, "cc:51 NextCapacity 倍增阶梯序");

  OK
}

/// 用例 C (h:294 kMaxBufferSize 饱和): 25500 槽后第九块容量落 16384 而非 2*12800=25600,
/// 即 std::min 的 kMaxBufferSize 支生效; 第 41885 槽另起新头块 (仍 16384 上限)。
#[test]
fn typed_slots_capacity_saturates_at_max_buffer() -> Void {
  info!("> typed_slots_capacity_saturates_at_max_buffer");
  // 100+200+400+800+1600+3200+6400 = 12700; 第八块 min(16384, 12800) = 12800 → 累计 25500;
  // 第九块 min(16384, 25600) = 16384 → 累计 41884。
  const K_SATURATE_TOTAL: u32 = 41884;
  let mut set = TypedSlotSet::new(0);
  for offset in 0..K_SATURATE_TOTAL {
    set.insert(SlotType::ConstPoolCodeEntry, offset);
  }
  // 41884 槽恰填满第九块 → head_ 块跨度须为 kMaxBufferSize (未饱和则为 25600)。
  let live = live_offsets(&mut set, 0);
  assert_eq!(
    live[0],
    K_SATURATE_TOTAL - K_MAX_BUFFER_SIZE as u32,
    "h:294 饱和块首槽"
  );
  assert_eq!(live.len(), K_SATURATE_TOTAL as usize);

  // 第 41885 槽: 第九块已满 → 头插第十块 (cap 仍 min(16384, 32768) = 16384)。
  set.insert(SlotType::ConstPoolCodeEntry, K_SATURATE_TOTAL);
  let live = live_offsets(&mut set, 0);
  assert_eq!(
    live[0..2],
    [
      K_SATURATE_TOTAL,
      K_SATURATE_TOTAL - K_MAX_BUFFER_SIZE as u32
    ],
    "cc:50 饱和后仍按 kMaxBufferSize 另起新头块"
  );
  assert_eq!(
    live.len(),
    K_SATURATE_TOTAL as usize + 1,
    "h:361 槽总数不减"
  );

  OK
}

// ══════════════════════════════════════════════════════════════════════════
// D/E 迭代模式双支 (h:310 / h:346-358)
// ══════════════════════════════════════════════════════════════════════════

/// 用例 D (FREE_EMPTY_CHUNKS): 全清后块被 delete (h:355), 后续插入重建初块
/// (cc:47-48 `!head_` → 新 kInitialBufferSize 块), 不被残留 cleared 槽挤占。
#[test]
fn typed_slot_set_free_empty_chunks_deletes_chunk() -> Void {
  info!("> typed_slot_set_free_empty_chunks_deletes_chunk");
  let mut set = TypedSlotSet::new(0);
  for offset in 0..3u32 {
    set.insert(SlotType::CodeEntry, offset);
  }

  // h:346-355: empty && FREE_EMPTY_CHUNKS → 摘块。
  let kept = set.iterate(
    |_ty: SlotType, _addr: Address| -> SlotCallbackResult { SlotCallbackResult::RemoveSlot },
    IterationMode::FreeEmptyChunks,
  );
  assert_eq!(kept, 0, "h:361 全清返回 0");
  assert!(live_offsets(&mut set, 0).is_empty(), "h:355 块已摘除");

  // 重建: 初块 cap 100, 填 3..103 共 100 槽恰满 → 第 104 槽另起新头块。
  // 若旧块未删 (残留 3 cleared 占位), 此段只余 97 空位, 边界会提前一格。
  for offset in 3..103u32 {
    set.insert(SlotType::CodeEntry, offset);
  }
  set.insert(SlotType::CodeEntry, 103);
  let mut expected = vec![103u32];
  expected.extend(span(3, 103));
  assert_eq!(
    live_offsets(&mut set, 0),
    expected,
    "h:355 摘块后 cc:48 重建初块 cap=100"
  );

  OK
}

/// 用例 E (KEEP_EMPTY_CHUNKS): 全清块保留 (h:356-358), 且本 revision 的 Iterate **不**
/// clear buffer → cleared 槽仍占容量, 后续插入把初块填到 100 才头插扩容。
#[test]
fn typed_slot_set_keep_empty_chunks_retains_cleared_capacity() -> Void {
  info!("> typed_slot_set_keep_empty_chunks_retains_cleared_capacity");
  let mut set = TypedSlotSet::new(0);
  for offset in 0..3u32 {
    set.insert(SlotType::CodeEntry, offset);
  }

  let kept = set.iterate(
    |_ty: SlotType, _addr: Address| -> SlotCallbackResult { SlotCallbackResult::RemoveSlot },
    IterationMode::KeepEmptyChunks,
  );
  assert_eq!(kept, 0, "h:361 全清返回 0");
  // h:334: 次轮迭代仍跳过 cleared → 回调不可达 (块在, 槽不报)。
  assert!(
    live_offsets(&mut set, 0).is_empty(),
    "h:334 cleared 槽不再回调"
  );

  // 初块已占 3 格 cleared → 再填 97 槽 (3..100) 达 cap 100, 第 100 槽头插新块。
  for offset in 3..100u32 {
    set.insert(SlotType::CodeEntry, offset);
  }
  set.insert(SlotType::CodeEntry, 100);
  let mut expected = vec![100u32];
  expected.extend(span(3, 100));
  assert_eq!(
    live_offsets(&mut set, 0),
    expected,
    "h:356 保留块含 cleared 占位 → 扩容边界落在第 101 次插入"
  );

  OK
}

/// 用例 F (h:338/361 new_count): 去留混合轮次返回保留槽数, 且 cleared 不再计入。
#[test]
fn typed_slot_set_iterate_returns_kept_count() -> Void {
  info!("> typed_slot_set_iterate_returns_kept_count");
  let mut set = TypedSlotSet::new(0);
  for offset in 0..10u32 {
    set.insert(SlotType::EmbeddedObjectCompressed, offset);
  }

  // 首轮: 偶数留、奇数清 → new_count = 5。
  let first = set.iterate(
    |_ty: SlotType, addr: Address| -> SlotCallbackResult {
      if (addr as u32).is_multiple_of(2) {
        SlotCallbackResult::KeepSlot
      } else {
        SlotCallbackResult::RemoveSlot
      }
    },
    IterationMode::KeepEmptyChunks,
  );
  assert_eq!(first, 5, "h:338 首轮保留计数");

  // 次轮: cleared 不回调 (h:334) → 计数仍 5, 返回值同 5。
  let mut seen = 0usize;
  let second = set.iterate(
    |_ty: SlotType, _addr: Address| -> SlotCallbackResult {
      seen += 1;
      SlotCallbackResult::KeepSlot
    },
    IterationMode::KeepEmptyChunks,
  );
  assert_eq!(seen, 5, "h:334 cleared 跳过");
  assert_eq!(second, 5, "h:361 次轮计数稳定");

  OK
}

// ══════════════════════════════════════════════════════════════════════════
// G 地址基址 (h:336) + 偏移键区间 (h:308)
// ══════════════════════════════════════════════════════════════════════════

/// 用例 G (h:336 `addr = page_start_ + offset`): 非零基址下回调地址逐字节对账;
/// 且 clear_invalid_slots 的区间键是**偏移**而非地址 (FreeRangesMap, h:308/cc:87)。
#[test]
fn typed_slot_set_page_start_bases_addresses_and_ranges_use_offsets() -> Void {
  info!("> typed_slot_set_page_start_bases_addresses_and_ranges_use_offsets");
  const K_PAGE_START: Address = 0x0000_7f00_1000;
  let mut set = TypedSlotSet::new(K_PAGE_START);
  for offset in 0..10u32 {
    set.insert(SlotType::ConstPoolEmbeddedObjectFull, offset * 100);
  }

  // 回调地址须严格 = 基址 + 偏移。
  let mut addrs: Vec<Address> = Vec::new();
  set.iterate(
    |_ty: SlotType, addr: Address| -> SlotCallbackResult {
      addrs.push(addr);
      SlotCallbackResult::KeepSlot
    },
    IterationMode::KeepEmptyChunks,
  );
  for &addr in &addrs {
    assert!(
      (K_PAGE_START..K_PAGE_START + 1000).contains(&addr),
      "h:336 地址落页内"
    );
  }
  assert_eq!(addrs.len(), 10);

  // 按偏移区间清理: [200,201) ∪ [400,401) → 命中偏移 200/400 两槽。
  set.clear_invalid_slots(&[(200, 201), (400, 401)]);
  let live = live_offsets(&mut set, K_PAGE_START);
  assert_eq!(
    live,
    vec![0, 100, 300, 500, 600, 700, 800, 900],
    "cc:64 偏移键命中清理"
  );

  OK
}

// ══════════════════════════════════════════════════════════════════════════
// H/I 空集与空区间边界
// ══════════════════════════════════════════════════════════════════════════

/// 用例 H: 空集 iterate 双模式均返回 0 且不建块 (h:330 while 体不进)。
#[test]
fn typed_slot_set_empty_iterate_both_modes() -> Void {
  info!("> typed_slot_set_empty_iterate_both_modes");
  let mut set = TypedSlotSet::new(0);
  let mut calls = 0usize;
  let free = set.iterate(
    |_ty: SlotType, _addr: Address| -> SlotCallbackResult {
      calls += 1;
      SlotCallbackResult::KeepSlot
    },
    IterationMode::FreeEmptyChunks,
  );
  let keep = set.iterate(
    |_ty: SlotType, _addr: Address| -> SlotCallbackResult {
      calls += 1;
      SlotCallbackResult::KeepSlot
    },
    IterationMode::KeepEmptyChunks,
  );
  assert_eq!(free, 0, "h:330 空集 FREE 计数");
  assert_eq!(keep, 0, "h:330 空集 KEEP 计数");
  assert_eq!(calls, 0, "h:330 空集回调不可达");
  // 空集 iterate 不得建块: 随后插 1 槽再清, FREE 模式下整链归零。
  set.insert(SlotType::Cleared, 7);
  assert_eq!(
    set.iterate(
      |_ty: SlotType, _addr: Address| -> SlotCallbackResult { SlotCallbackResult::RemoveSlot },
      IterationMode::FreeEmptyChunks,
    ),
    0,
    "h:334 预置 kCleared 槽不参与计数"
  );
  assert!(live_offsets(&mut set, 0).is_empty(), "h:346 空块摘除后链空");

  OK
}

/// 用例 I (cc:80-81 `ranges.empty()` 早退): 空区间清理不得改任何槽。
#[test]
fn typed_slot_set_clear_invalid_slots_empty_ranges_noop() -> Void {
  info!("> typed_slot_set_clear_invalid_slots_empty_ranges_noop");
  let mut set = TypedSlotSet::new(0);
  for offset in 0..5u32 {
    set.insert(SlotType::EmbeddedObjectFull, offset * 8);
  }
  set.clear_invalid_slots(&[]);
  assert_eq!(
    live_offsets(&mut set, 0),
    vec![0, 8, 16, 24, 32],
    "cc:80 空区间早退不改槽"
  );

  // 全清槽集合上再清: 已 cleared 槽二次跳过 (cc:86), 计数稳定。
  set.clear_invalid_slots(&[(0, K_MAX_OFFSET_EXCL)]);
  assert_eq!(
    set.iterate(
      |_ty: SlotType, _addr: Address| -> SlotCallbackResult { SlotCallbackResult::KeepSlot },
      IterationMode::KeepEmptyChunks,
    ),
    0,
    "cc:86 cleared 二次跳过"
  );

  OK
}

// ══════════════════════════════════════════════════════════════════════════
// J 区间回退 quirk (cc:88-94)
// ══════════════════════════════════════════════════════════════════════════

/// 用例 J (cc:88-94): std::map upper_bound 回退**只比末格**, 重叠区间下
/// offset 25/26 落 [10,30) 却因末格 [20,25) 终点不 > offset 而命中失败 —— 原版 quirk,
/// 二分复刻必须逐位一致, 不得"顺手修正"。
#[test]
fn typed_slot_set_ranges_use_upper_bound_backoff_quirk() -> Void {
  info!("> typed_slot_set_ranges_use_upper_bound_backoff_quirk");
  let mut set = TypedSlotSet::new(0);
  let probes: [u32; 8] = [5, 10, 15, 19, 20, 24, 25, 26];
  for offset in probes {
    set.insert(SlotType::CodeEntry, offset);
  }

  // 起点升序的区间 (std::map 键序), 第二条嵌于第一条内。
  set.clear_invalid_slots(&[(10, 30), (20, 25)]);

  let live = live_offsets(&mut set, 0);
  assert_eq!(
    live,
    vec![5, 25, 26],
    "cc:92-94 回退一格: 25/26 命中末格 [20,25) 终点不 > offset → 不清"
  );

  OK
}

// ══════════════════════════════════════════════════════════════════════════
// K AssertNoInvalidSlots (cc:69-75)
// ══════════════════════════════════════════════════════════════════════════

/// 用例 K1: 无命中区间时断言面静默通过 (cc:69 正向)。
#[test]
fn typed_slot_set_assert_no_invalid_slots_clean() -> Void {
  info!("> typed_slot_set_assert_no_invalid_slots_clean");
  let mut set = TypedSlotSet::new(0);
  for offset in 0..6u32 {
    set.insert(SlotType::ConstPoolEmbeddedObjectCompressed, offset * 10);
  }
  set.assert_no_invalid_slots(&[(60, 70)]);
  // 空区间亦不得违例 (IterateSlotsInRanges cc:80 早退)。
  set.assert_no_invalid_slots(&[]);
  // 已清槽不计命中 (cc:86)。
  set.clear_invalid_slots(&[(0, 10)]);
  set.assert_no_invalid_slots(&[(0, 10)]);
  assert_eq!(live_offsets(&mut set, 0).len(), 5, "cc:64 首槽已清");

  OK
}

/// 用例 K2 (cc:72 `CHECK_WITH_MSG(false, ...)`): 命中即违例, 本仓 CHECK→debug_assert 形。
#[test]
#[should_panic(expected = "no slot in ranges expected")]
fn typed_slot_set_assert_no_invalid_slots_panics_on_hit() {
  let mut set = TypedSlotSet::new(0);
  set.insert(SlotType::CodeEntry, 15);
  set.assert_no_invalid_slots(&[(10, 20)]);
}

// ══════════════════════════════════════════════════════════════════════════
// L BitField::encode 越界 (bit-field.h:91/100)
// ══════════════════════════════════════════════════════════════════════════

/// 用例 L1: 偏移域越界 (>= kMaxOffset 1<<29) 在原版为 CHECK 硬失败 → debug_assert 承接。
#[test]
#[should_panic(expected = "偏移须 < kMaxOffset")]
fn typed_slot_encode_out_of_range_offset_panics() {
  let _ = typed_slot_encode(SlotType::CodeEntry as u8, K_MAX_OFFSET_EXCL);
}

/// 用例 L2: 类型域越界 (> 3 位 / > kLast=6) 在原版为 DCHECK 承接 (bit-field.h:91)。
#[test]
#[should_panic(expected = "SlotType 须落 3 位类型域")]
fn typed_slot_encode_out_of_range_type_panics() {
  let _ = typed_slot_encode(7, 0);
}

/// 用例 L3: 区间乱序 (起点非升序) — 原版 std::map 键序不可能出现, 切片承接须拦截
/// (cc:88 upper_bound 回退只对有序键成立)。
#[test]
#[should_panic(expected = "按起点升序")]
fn typed_slot_set_unsorted_ranges_panics() {
  let mut set = TypedSlotSet::new(0);
  set.insert(SlotType::CodeEntry, 5);
  set.clear_invalid_slots(&[(30, 40), (10, 20)]);
}

// ══════════════════════════════════════════════════════════════════════════
// M/N Merge (cc:31-44)
// ══════════════════════════════════════════════════════════════════════════

/// 用例 M (cc:35-37): 己空 → 整体接管他链 (含已扩容的多块链), 他置空。
#[test]
fn typed_slot_set_merge_into_empty_takes_whole_chain() -> Void {
  info!("> typed_slot_set_merge_into_empty_takes_whole_chain");
  let mut dst = TypedSlotSet::new(0);
  let mut src = TypedSlotSet::new(0);
  for offset in 0..150u32 {
    src.insert(SlotType::CodeEntry, offset);
  }
  // src: 初块 [0..99] + 头块 [100..149] (cap 200)。
  dst.merge(&mut src);
  let mut expected = span(100, 150);
  expected.extend(span(0, 100));
  assert_eq!(
    live_offsets(&mut dst, 0),
    expected,
    "cc:36 接管整链保 head→tail 序"
  );
  assert!(live_offsets(&mut src, 0).is_empty(), "cc:42-43 他链置空");

  // 他空: 己链不动 (cc:32-34 早退)。
  let before = live_offsets(&mut dst, 0);
  let mut empty_src = TypedSlotSet::new(0);
  dst.merge(&mut empty_src);
  assert_eq!(live_offsets(&mut dst, 0), before, "cc:32 空他链早退");

  OK
}

/// 用例 N (cc:39 `tail_->next = other->head_`): 双方均已跨块扩容时,
/// 序 = 己 (最新块→最旧块) 全先于 他 (最新块→最旧块)。
#[test]
fn typed_slot_set_merge_keeps_order_across_growth() -> Void {
  info!("> typed_slot_set_merge_keeps_order_across_growth");
  let mut set0 = TypedSlotSet::new(0);
  let mut set1 = TypedSlotSet::new(0);
  for offset in 0..105u32 {
    set0.insert(SlotType::EmbeddedObjectFull, offset);
  }
  for offset in 1000..1051u32 {
    set1.insert(SlotType::EmbeddedObjectFull, offset);
  }
  set0.merge(&mut set1);

  // set0: 头块 [100..104] + 初块 [0..99]; set1: 单块 [1000..1050] (未满 100)。
  let mut expected = span(100, 105);
  expected.extend(span(0, 100));
  expected.extend(span(1000, 1051));
  assert_eq!(
    live_offsets(&mut set0, 0),
    expected,
    "cc:39 尾接他链头, 块内序不变"
  );
  assert!(
    live_offsets(&mut set1, 0).is_empty(),
    "cc:43 合并后 set1 空"
  );

  OK
}
