#!/usr/bin/env bun
// tool/src/heap/slot-set.cc.js
// 机械提取 v8/src/heap/slot-set.{h,cc}: TypedSlots 位域编码 / 容量增长 /
// 区间命中谓词 (upper_bound 语义) + 方法签名骨架
// 槽类型枚举与布局常量已在 slot_set.rs (slot-set.h.js), 不重复

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/heap/slot-set.h");
const ccSrc = readV8("src/heap/slot-set.cc");

// —— 位域布局 (slot-set.h:284-285) ——
const offField = hSrc.match(/using OffsetField = base::BitField<int, (\d+), (\d+)>;/);
const typeField = hSrc.match(/using TypeField = base::BitField<SlotType, (\d+), (\d+)>;/);
if (!offField || !typeField) throw new Error("TypedSlots 位域定义缺失");
if (offField[1] !== "0" || offField[2] !== "29") throw new Error(`OffsetField 布局漂移: ${offField.slice(1)}`);
if (typeField[1] !== "29" || typeField[2] !== "3") throw new Error(`TypeField 布局漂移: ${typeField.slice(1)}`);

// —— 容量常量 (slot-set.h:293-297) ——
const initBuf = hSrc.match(/kInitialBufferSize = (\d+);/);
if (!initBuf || initBuf[1] !== "100") throw new Error("kInitialBufferSize 漂移");
const maxBuf = hSrc.match(/kMaxBufferSize = ([^;]+);/);
if (!maxBuf || maxBuf[1].replace(/\s+/g, "") !== "16*KB") throw new Error(`kMaxBufferSize 漂移: ${maxBuf && maxBuf[1]}`);
const nextCap = hSrc.match(/NextCapacity\(size_t capacity\)\s*\{\s*return std::min\(\{kMaxBufferSize, capacity \* 2\}\);/);
if (!nextCap) throw new Error("NextCapacity 增长式漂移");

// —— kMaxOffset (slot-set.h:277) ——
const maxOff = hSrc.match(/kMaxOffset = \(1 << (\d+)\);/) || hSrc.match(/kMaxOffset = 1 << (\d+);/);
if (!maxOff || maxOff[1] !== "29") throw new Error("kMaxOffset 漂移");

// —— SlotType 序 (复核 kCleared=6, 与 slot_set.rs 同源) ——
const st = hSrc.match(/enum class SlotType : uint8_t \{([\s\S]*?)\n\};/);
if (!st) throw new Error("未找到 SlotType");
const slotTypes = st[1].replace(/\/\/[^\n]*/g, "").split(",").map((s) => s.trim()).filter(Boolean).filter((s) => !s.includes("="));
if (slotTypes.length !== 7) throw new Error(`SlotType ${slotTypes.length}, 期望 7`);
const clearedIdx = slotTypes.indexOf("kCleared");
if (clearedIdx !== 6) throw new Error(`kCleared 判别值漂移: ${clearedIdx}`);

// —— .cc 方法清单 (签名骨架素材) ——
function collectDefs(src, re) {
  const out = [];
  const lines = src.split("\n");
  for (let i = 0; i < lines.length; i++) {
    const m = lines[i].match(re);
    if (!m) continue;
    // 跨行签名吞并: 括号闭合为止
    let full = lines[i].trim();
    let j = i;
    while (!/\)\s*(?:const\s*)?[;{=]?\s*$/.test(full) && j + 1 < lines.length) {
      full += " " + lines[++j].trim();
    }
    out.push({ line: i + 1, cls: m[1], name: m[2], sig: full.replace(/\s+/g, " ").replace(/\s*\{\s*$/, "") });
  }
  return out;
}
const defs = collectDefs(
  ccSrc,
  /^(?:[A-Za-z_][\w:<>,&*\[\] ]*[\s>])?(TypedSlots|TypedSlotSet)::([A-Za-z_~]\w*)\s*\(/,
);
const byCls = {};
for (const d of defs) byCls[d.cls] = (byCls[d.cls] || 0) + 1;
if (byCls.TypedSlots !== 5) throw new Error(`TypedSlots 定义数 ${byCls.TypedSlots}, 期望 5`);
if (byCls.TypedSlotSet !== 3) throw new Error(`TypedSlotSet 定义数 ${byCls.TypedSlotSet}, 期望 3`);
const defNames = defs.map((d) => d.name).join(",");
if (!/^~TypedSlots,Insert,Merge,EnsureChunk,NewChunk,ClearInvalidSlots,AssertNoInvalidSlots,IterateSlotsInRanges$/.test(defNames)) {
  throw new Error(`slot-set.cc 定义序漂移: ${defNames}`);
}

// —— ClearedTypedSlot 规则锚 (cc 侧 ClearedTypedSlot 在 h:387-390) ——
if (!hSrc.includes("TypeField::encode(SlotType::kCleared) |")) throw new Error("ClearedTypedSlot 编码式缺失");
// —— IterateSlotsInRanges 语义锚 (cc:88-94 upper_bound 回退) ——
if (!ccSrc.includes("ranges.upper_bound(offset)")) throw new Error("upper_bound 语义锚缺失");
if (!ccSrc.includes("upper_bound--")) throw new Error("upper_bound 回退锚缺失");
if (!ccSrc.includes("if (upper_bound->second > offset)")) throw new Error("区间终点判定锚缺失");

// —— 数值对账 (JS 侧预演) ——
const CLEARED_U32 = (clearedIdx << 29) >>> 0;
if (CLEARED_U32 !== 0xc0000000) throw new Error(`ClearedTypedSlot 预演漂移: 0x${CLEARED_U32.toString(16)}`);

console.log(`slot-set.cc: 位域 29+3, 容量 100→16384, 定义 ${defs.length} (${byCls.TypedSlots}+${byCls.TypedSlotSet}), cleared=0x${CLEARED_U32.toString(16)}`);

let rust = banner(
  "src/heap/slot-set.cc.js",
  "src/heap/slot-set.{h,cc}",
  "TypedSlots 位域编码/容量增长/区间命中谓词; 枚举面在 slot_set.rs",
);

const snake = (s) => s.replace(/^~/, "").replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();

rust += `
//! TypedSlots / TypedSlotSet 的机械规则面。
//!
//! - TypedSlot = 32 位打包: [31:29] SlotType (3 bit) | [28:0] 偏移 (29 bit)
//! - 容量增长: 新块容量 = min(kMaxBufferSize, 旧容量 × 2), 初块 100
//! - IterateSlotsInRanges 的 std::map upper_bound 语义 → 有序切片 partition_point 等价

/// 偏移域位宽 (OffsetField<int, 0, 29>)
pub const K_OFFSET_BITS: u32 = 29;
/// 类型域起始位 (TypeField<SlotType, 29, 3>)
pub const K_TYPE_SHIFT: u32 = 29;
/// 类型域位宽
pub const K_TYPE_BITS: u32 = 3;
/// 偏移上限 (TypedSlots::kMaxOffset, 含)
pub const K_MAX_OFFSET: u32 = (1 << 29) - 1;
/// 初块容量 (kInitialBufferSize)
pub const K_INITIAL_BUFFER_SIZE: usize = 100;
/// 块容量上限 (kMaxBufferSize = 16 * KB)
pub const K_MAX_BUFFER_SIZE: usize = 16 * 1024;
/// kCleared 判别值 (SlotType 第 7 员; 槽集枚举面见 slot_set.rs)
pub const K_SLOT_TYPE_CLEARED: u8 = ${clearedIdx};

/// TypedSlot 编码 (原版 TypeField::encode(type) | OffsetField::encode(offset))
/// v8: src/heap/slot-set.cc:25 TypedSlots::Insert
#[must_use]
pub const fn typed_slot_encode(slot_type: u8, offset: u32) -> u32 {
  ((slot_type as u32) << K_TYPE_SHIFT) | (offset & K_MAX_OFFSET)
}

/// 类型域解码 (TypeField::decode)
#[must_use]
pub const fn typed_slot_decode_type(type_and_offset: u32) -> u8 {
  (type_and_offset >> K_TYPE_SHIFT) as u8
}

/// 偏移域解码 (OffsetField::decode)
#[must_use]
pub const fn typed_slot_decode_offset(type_and_offset: u32) -> u32 {
  type_and_offset & K_MAX_OFFSET
}

/// ClearedTypedSlot — 已清除槽的编码值 (kCleared << 29 | 0)
/// v8: src/heap/slot-set.h:387 TypedSlotSet::ClearedTypedSlot
#[must_use]
pub const fn cleared_typed_slot() -> u32 {
  (K_SLOT_TYPE_CLEARED as u32) << K_TYPE_SHIFT
}

/// 新块容量 (原版 NextCapacity = min(kMaxBufferSize, capacity * 2))
/// v8: src/heap/slot-set.h:295
#[must_use]
pub const fn next_capacity(capacity: usize) -> usize {
  if K_MAX_BUFFER_SIZE < capacity * 2 { K_MAX_BUFFER_SIZE } else { capacity * 2 }
}

/// 槽偏移是否落入失效区间 — IterateSlotsInRanges 的命中判定
/// (原版 std::map::upper_bound(offset) 后回退一格, 再比区间终点;
///  此处 ranges 为按起点升序的切片, 线性扫描定位首个起点大于 offset 的区间)
/// v8: src/heap/slot-set.cc:78 TypedSlotSet::IterateSlotsInRanges
#[must_use]
pub const fn slot_in_ranges(ranges: &[(u32, u32)], offset: u32) -> bool {
  let mut idx = 0;
  while idx < ranges.len() {
    if ranges[idx].0 > offset {
      break;
    }
    idx += 1;
  }
  if idx == 0 {
    return false;
  }
  ranges[idx - 1].1 > offset
}
`;

// —— 方法签名骨架 ——
rust += `
// —— 方法签名骨架 (语义实现由 Developer 就地修订) ——
`;
for (const d of defs) {
  const owner = d.cls === "TypedSlots" ? "typed_slots" : "typed_slot_set";
  rust += `/// 原版签名: ${d.sig}
/// v8: src/heap/slot-set.cc:${d.line}
pub fn ${owner}_${snake(d.name)}() {
  todo!("就地修订")
}
`;
}

rust += `
// —— 提取不变式断言 ——
const _: () = {
  assert!(K_OFFSET_BITS == 29);
  assert!(K_TYPE_SHIFT == 29);
  assert!(K_TYPE_BITS == 3);
  assert!(K_MAX_OFFSET == (1 << 29) - 1);
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
  // upper_bound 语义: 起点严格大于 offset 的首区间, 回退一格比终点 (区间终点排他)
  assert!(!slot_in_ranges(&[(10u32, 20u32), (30, 40)], 5)); // 无前驱区间
  assert!(slot_in_ranges(&[(10u32, 20u32), (30, 40)], 10));
  assert!(slot_in_ranges(&[(10u32, 20u32), (30, 40)], 19));
  assert!(!slot_in_ranges(&[(10u32, 20u32), (30, 40)], 20)); // 终点排他
  assert!(!slot_in_ranges(&[(10u32, 20u32), (30, 40)], 21));
  assert!(slot_in_ranges(&[(10u32, 20u32), (30, 40)], 39));
  assert!(!slot_in_ranges(&[(10u32, 20u32), (30, 40)], 40));
};
`;

const p = writeSrc("v8-heap-core", "slot_set_cc.skel", rust);
console.log(`written ${p}`);
