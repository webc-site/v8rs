//! tests/slot_set_unittest.rs — v8-heap-core/src/slot_set.rs 金标准用例 (W10HCb 收编)。
//!
//! 1:1 复刻 v8/test/unittests/heap/slot-set-unittest.cc 的 TypedSlotSet 三支纯逻辑用例
//! (TEST(TypedSlotSet, Iterate/ClearInvalidSlots/Merge), cc 起于 :62/:99/:129): 分块槽链
//! 的插入/头插扩容/合并序/区间清理, 逐断言注释指原版 file:line 算例。
//!
//! 同文件其余两支 PossiblyEmptyBucketsTest.WordsForBuckets (:18) / SlotSet.BucketsForSize
//! (:40) 与 PossiblyEmptyBuckets.ContainsAndInsert (:48) 依赖 heap/slot-set.h 的 SlotSet
//! 页位图类 (页 bitmap / MemoryChunk 运行态, HC-4), 其位图基座已由 v8-heap-base
//! basic_slot_set.rs (heap/base/basic-slot-set.h) 落地覆盖, 本 crate slot_set 未挂该类
//! → 不在此复刻 (登记推迟, 见 readme slot_set 组)。

use std::panic::{AssertUnwindSafe, catch_unwind};

use aok::{OK, Void};
use log::info;
use v8_common::Address;
use v8_heap_base::SlotCallbackResult;
use v8_heap_core::slot_set::*;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 原版 slot-set-unittest.cc:62 TEST(TypedSlotSet, Iterate)。
/// kDelta=10000001; 步进 0..kMaxOffset 插 54 槽 (i%6 循环类型); 首轮按 i%2 去留,
/// 次轮余偶数槽。
#[test]
fn typed_slot_set_iterate() -> Void {
  info!("> typed_slot_set_iterate");
  let mut set = TypedSlotSet::new(0);
  // 原版 :67 kDelta; :69 i < TypedSlotSet::kMaxOffset (1<<29) 步进。
  const K_DELTA: u32 = 10000001;
  const K_CLEARED_U8: u32 = 6;
  let mut added = 0usize;
  let mut i: u32 = 0;
  while i < K_MAX_OFFSET_EXCL {
    // 原版 :70-71 type = i % kCleared。
    let ty = slot_type_of((i % K_CLEARED_U8) as u8);
    set.insert(ty, i);
    added += 1;
    i += K_DELTA;
  }

  // 原版 :76-85 首轮: 校验 type/偏移对齐 kDelta, 偶数保留奇数移除。
  let mut iterated = 0usize;
  let kept_first = {
    let seen = &mut iterated;
    let added_ref = added;
    set.iterate(
      |ty: SlotType, addr: Address| -> SlotCallbackResult {
        let idx = addr as u32;
        // 原版 :79-81 i % kCleared == type; :81 i % kDelta == 0。
        assert_eq!(idx % K_CLEARED_U8, ty as u32, "类型域回读漂移");
        assert_eq!(idx % K_DELTA, 0, "偏移非 kDelta 倍数");
        *seen += 1;
        if idx.is_multiple_of(2) {
          SlotCallbackResult::KeepSlot
        } else {
          SlotCallbackResult::RemoveSlot
        }
      },
      IterationMode::KeepEmptyChunks,
    );
    // 返回首轮 iterated 应 == added (原版 :86 EXPECT_EQ(added, iterated))。
    assert_eq!(*seen, added_ref);
    *seen
  };
  assert_eq!(iterated, added, "cc:86 首轮遍历计数");
  assert_eq!(kept_first, added);

  // 原版 :87-96 次轮: 全保留, 仅余上轮保留的偶数槽, 计数 added/2。
  let mut iterated2 = 0usize;
  set.iterate(
    |_ty: SlotType, addr: Address| -> SlotCallbackResult {
      // 原版 :91 i % 2 == 0。
      assert!(addr.is_multiple_of(2), "次轮仅余偶数偏移槽");
      iterated2 += 1;
      SlotCallbackResult::KeepSlot
    },
    IterationMode::KeepEmptyChunks,
  );
  assert_eq!(iterated2, added / 2, "cc:96 次轮保留偶数槽数");

  OK
}

/// 原版 slot-set-unittest.cc:99 TEST(TypedSlotSet, ClearInvalidSlots)。
/// 插 10 槽 (i*100), 失效区间 [100..900 奇数位 +1], 清理后逐区间校验无残留。
#[test]
fn typed_slot_set_clear_invalid_slots() -> Void {
  info!("> typed_slot_set_clear_invalid_slots");
  let mut set = TypedSlotSet::new(0);
  const K_HOST_DELTA: u32 = 100;
  let entries: u32 = 10;
  // 原版 :103-107 插 i*kHostDelta, type = i % kCleared。
  for i in 0..entries {
    let ty = slot_type_of((i % 6) as u8);
    set.insert(ty, i * K_HOST_DELTA);
  }

  // 原版 :109-113 invalid_ranges 插 (i*delta, i*delta+1) for i 奇数 1..entries step2;
  // std::map 按起点升序 → 此处有序切片。
  let invalid_ranges: Vec<(u32, u32)> = (1..entries)
    .step_by(2)
    .map(|i| (i * K_HOST_DELTA, i * K_HOST_DELTA + 1))
    .collect();
  assert_eq!(
    invalid_ranges,
    vec![(100, 101), (300, 301), (500, 501), (700, 701), (900, 901)]
  );

  set.clear_invalid_slots(&invalid_ranges);

  // 原版 :116-126 逐区间 Iterate 校验无槽落 [start, end)。
  for &(start, end) in &invalid_ranges {
    set.iterate(
      |_ty: SlotType, addr: Address| -> SlotCallbackResult {
        assert!(
          addr < start as Address || addr >= end as Address,
          "cc:122 失效槽未清"
        );
        SlotCallbackResult::KeepSlot
      },
      IterationMode::KeepEmptyChunks,
    );
  }

  // 命中的 5 个奇数位偏移 (100..900 step200) 已清, 余 5 个 (0,200,400,600,800) 保留。
  let mut remaining = 0usize;
  set.iterate(
    |_ty: SlotType, _addr: Address| -> SlotCallbackResult {
      remaining += 1;
      SlotCallbackResult::KeepSlot
    },
    IterationMode::KeepEmptyChunks,
  );
  assert_eq!(remaining, 5, "清理后余偶数位槽数 (原版 CHECK 反推计数)");

  OK
}

/// 原版 slot-set-unittest.cc:129 TEST(TypedSlotSet, Merge)。
/// 两集各 10000 槽 (偶/奇), 合并后迭代序 = 己块全先于他块 (cc:39 tail_->next=other->head)。
#[test]
fn typed_slot_set_merge() -> Void {
  info!("> typed_slot_set_merge");
  let mut set0 = TypedSlotSet::new(0);
  let mut set1 = TypedSlotSet::new(0);
  let k_entries: u32 = 10000;
  for i in 0..k_entries {
    // 原版 :133-134 set0 偶 / set1 奇。
    set0.insert(SlotType::EmbeddedObjectFull, 2 * i);
    set1.insert(SlotType::EmbeddedObjectFull, 2 * i + 1);
  }

  // 原版 :137 set0.Merge(&set1)。
  set0.merge(&mut set1);

  let mut count = 0u32;
  set0.iterate(
    |_ty: SlotType, addr: Address| -> SlotCallbackResult {
      // 原版 :140-144 前半偶、后半奇 (头→尾序保序)。
      if count < k_entries {
        assert!(addr.is_multiple_of(2), "前半应全为 set0 偶数槽");
      } else {
        assert!(!addr.is_multiple_of(2), "后半应全为 set1 奇数槽");
      }
      count += 1;
      SlotCallbackResult::KeepSlot
    },
    IterationMode::KeepEmptyChunks,
  );
  // 原版 :149 CHECK_EQ(2*kEntries, count)。
  assert_eq!(count, 2 * k_entries, "cc:149 合并后总槽数");

  // 原版 :150-155 set1 已空, Iterate 回调不可达。
  let mut leaked = 0usize;
  set1.iterate(
    |_ty: SlotType, _addr: Address| -> SlotCallbackResult {
      leaked += 1;
      SlotCallbackResult::KeepSlot
    },
    IterationMode::KeepEmptyChunks,
  );
  assert_eq!(leaked, 0, "cc:152 合并后 set1 应空 (回调不可达)");

  OK
}

/// 边界 1: 跨 chunk 头插扩容序 + FREE_EMPTY_CHUNKS 摘块 + iterate 返回计数。
/// 原版 h:327-359 (chunk=head_ 起头→尾序, 空块 delete) + cc:50-51 (头满头插
/// NextCapacity 扩容)。250 槽: 尾块 cap100 承 0..99, 头块 cap200 承 100..249,
/// 迭代序 = 头块先 (100..249), 尾块后 (0..99)。
#[test]
fn typed_slot_set_chunk_growth_order_gold() -> Void {
  info!("> typed_slot_set_chunk_growth_order_gold");
  let mut set = TypedSlotSet::new(0);
  for i in 0..250u32 {
    set.insert(SlotType::EmbeddedObjectFull, i);
  }
  let mut order: Vec<Address> = Vec::new();
  let kept = set.iterate(
    |_ty: SlotType, addr: Address| -> SlotCallbackResult {
      order.push(addr);
      SlotCallbackResult::KeepSlot
    },
    IterationMode::KeepEmptyChunks,
  );
  // h:337-338: 全保留 → new_count == 250 (int 返回 = 保留槽数)。
  assert_eq!(kept, 250, "h:329 new_count 全保留计数");
  let expect: Vec<Address> = (100..250).chain(0..100).collect();
  assert_eq!(order, expect, "cc:51 头插扩容后 head→tail 序 (新块先)");

  // FREE_EMPTY_CHUNKS 全移除: h:346-355 空块摘除, 返回 0, 链清空。
  let kept = set.iterate(
    |_ty: SlotType, _addr: Address| -> SlotCallbackResult { SlotCallbackResult::RemoveSlot },
    IterationMode::FreeEmptyChunks,
  );
  assert_eq!(kept, 0, "h:329 全移除计数");
  // 链已空: 再插入落新初块 (cap kInitialBufferSize, cc:48)。
  set.insert(SlotType::CodeEntry, 999);
  let mut after: Vec<Address> = Vec::new();
  let kept = set.iterate(
    |_ty: SlotType, addr: Address| -> SlotCallbackResult {
      after.push(addr);
      SlotCallbackResult::KeepSlot
    },
    IterationMode::KeepEmptyChunks,
  );
  assert_eq!(kept, 1);
  assert_eq!(after, vec![999], "h:346 摘块后链重启, 仅余新槽");

  // KEEP_EMPTY_CHUNKS 不移除空块 (h:356-357 else 支): 全清后块保留, 已清槽不再回调。
  let mut set2 = TypedSlotSet::new(0);
  for i in 0..5u32 {
    set2.insert(SlotType::CodeEntry, i);
  }
  set2.iterate(
    |_ty: SlotType, _addr: Address| SlotCallbackResult::RemoveSlot,
    IterationMode::KeepEmptyChunks,
  );
  set2.insert(SlotType::CodeEntry, 5);
  let mut n = 0usize;
  let kept = set2.iterate(
    |_ty: SlotType, _addr: Address| -> SlotCallbackResult {
      n += 1;
      SlotCallbackResult::KeepSlot
    },
    IterationMode::KeepEmptyChunks,
  );
  assert_eq!(n, 1, "h:334 已清槽 (kCleared) 跳过, 仅新槽回调");
  assert_eq!(kept, 1);

  OK
}

/// 边界 2: merge 交叠面 — cc:32-34 他空早退 / cc:35-37 己空接管 / 双空。
#[test]
fn typed_slot_set_merge_edge_gold() -> Void {
  info!("> typed_slot_set_merge_edge_gold");
  let count_of = |set: &mut TypedSlotSet| -> i32 {
    set.iterate(
      |_ty: SlotType, _addr: Address| SlotCallbackResult::KeepSlot,
      IterationMode::KeepEmptyChunks,
    )
  };

  // 己非空 + 他空: cc:32-34 return, 己槽数不变。
  let mut a = TypedSlotSet::new(0);
  let mut empty = TypedSlotSet::new(0);
  a.insert(SlotType::CodeEntry, 1);
  a.merge(&mut empty);
  assert_eq!(count_of(&mut a), 1, "cc:32 他空早退");

  // 己空 + 他非空: cc:35-37 head_/tail_ 直接接管。
  let mut b = TypedSlotSet::new(0);
  b.insert(SlotType::CodeEntry, 2);
  b.insert(SlotType::CodeEntry, 3);
  let mut c = TypedSlotSet::new(0);
  c.merge(&mut b);
  assert_eq!(count_of(&mut c), 2, "cc:36 己空接管他链");
  assert_eq!(count_of(&mut b), 0, "cc:42-43 接管后他链清空");

  // 双空 merge: 无副作用。
  let mut d = TypedSlotSet::new(0);
  d.merge(&mut empty);
  assert_eq!(count_of(&mut d), 0);

  // 空集 iterate (h:330 while 不进): 返回 0。
  let mut e = TypedSlotSet::new(0);
  assert_eq!(count_of(&mut e), 0, "空集迭代计数");

  OK
}

/// 边界 3: clear_invalid_slots 空区间早退 (cc:80) + assert_no_invalid_slots
/// (cc:69-75, CHECK→debug_assert: 命中仅 debug 档 panic) + 已清槽不计命中 (cc:86)。
#[test]
fn typed_slot_set_clear_assert_edge_gold() -> Void {
  info!("> typed_slot_set_clear_assert_edge_gold");
  let mut set = TypedSlotSet::new(0);
  set.insert(SlotType::CodeEntry, 150);

  // 空 ranges: cc:80 早退, 槽保留。
  set.clear_invalid_slots(&[]);
  assert_eq!(
    set.iterate(
      |_ty: SlotType, _addr: Address| SlotCallbackResult::KeepSlot,
      IterationMode::KeepEmptyChunks,
    ),
    1,
    "cc:80 空区间早退不清槽"
  );

  // 正断言: 无命中区间 (含界外起点) → 不违例; 命中先清后断言亦净 (cc:86 跳过 cleared)。
  set.clear_invalid_slots(&[(100, 200)]);
  set.assert_no_invalid_slots(&[(100, 200)]);
  set.assert_no_invalid_slots(&[(0, 150), (151, 400)]);

  // 反断言 (仅 debug 档): 活槽落区间 → debug_assert panic (cc:72 CHECK 降级面)。
  #[cfg(debug_assertions)]
  {
    let mut bad = TypedSlotSet::new(0);
    bad.insert(SlotType::CodeEntry, 150);
    let r = catch_unwind(AssertUnwindSafe(|| {
      bad.assert_no_invalid_slots(&[(100, 200)]);
    }));
    assert!(r.is_err(), "cc:72 活槽命中区间应违例");
  }

  OK
}

/// 机械规则面补测: 位域编解码 + 区间命中边界 (承 const 断言的运行时对偶)。
#[test]
fn typed_slot_codec_and_ranges_rules() -> Void {
  info!("> typed_slot_codec_and_ranges_rules");
  // 编码往返 (slot-set.cc:25)。
  for ty in 0..=K_SLOT_TYPE_CLEARED {
    for off in [0u32, 1, 4095, K_MAX_OFFSET] {
      let enc = typed_slot_encode(ty, off);
      assert_eq!(typed_slot_decode_type(enc), ty);
      assert_eq!(typed_slot_decode_offset(enc), off);
    }
  }
  // 解码侧仅取低 29 位 (OffsetField::decode, bit-field.h:111 `(value & kMask) >> kShift`)。
  // 编码侧越界在原版是 CHECK 硬失败 (bit-field.h:100 `CHECK(is_valid(value))`),
  // **从不静默截断** → 本仓按 CHECK→debug_assert 口径拦截, 违例用例见
  // slot_set_typed_slots_unittest::typed_slot_encode_out_of_range_*。
  assert_eq!(typed_slot_decode_offset(1 << K_OFFSET_BITS), 0);
  // cleared 编码值 (h:387)。
  assert_eq!(cleared_typed_slot(), 0xc000_0000);

  // 容量增长阶梯 (h:295): 100→200→400→…→16384 饱和。
  let mut cap = K_INITIAL_BUFFER_SIZE;
  let mut steps = 0;
  while cap < K_MAX_BUFFER_SIZE {
    cap = next_capacity(cap);
    steps += 1;
  }
  assert_eq!(cap, K_MAX_BUFFER_SIZE);
  assert_eq!(next_capacity(K_MAX_BUFFER_SIZE), K_MAX_BUFFER_SIZE);
  assert!(steps > 0);

  // 区间命中: 起点升序切片, 终点排他 (cc:88-94)。
  let ranges = [(10u32, 20u32), (30, 40)];
  assert!(!slot_in_ranges(&ranges, 5));
  assert!(slot_in_ranges(&ranges, 10));
  assert!(slot_in_ranges(&ranges, 19));
  assert!(!slot_in_ranges(&ranges, 20));
  assert!(!slot_in_ranges(&ranges, 25));
  assert!(slot_in_ranges(&ranges, 30));
  assert!(slot_in_ranges(&ranges, 39));
  assert!(!slot_in_ranges(&ranges, 40));

  OK
}

/// 判别值 → SlotType (测试侧独立复刻, 验 slot_set 私有映射的公开对偶)。
fn slot_type_of(v: u8) -> SlotType {
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
