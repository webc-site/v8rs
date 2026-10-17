#!/usr/bin/env bun
// tool/src/heap/mutable-page.cc.js
// 机械提取 v8/src/heap/mutable-page.{cc,inl.h}: 记忆集释放序 /
// 页旗标计算规则面 (MarkingMode × Space 分支) + 方法签名骨架
// 枚举面 (RememberedSetType/MarkingMode/ConcurrentSweepingState) 已在 mutable_page_enums.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/heap/mutable-page.h");
const ccSrc = readV8("src/heap/mutable-page.cc");
const inlSrc = readV8("src/heap/mutable-page-inl.h");

// —— RememberedSetType 序 (mutable-page.h:32-42) ——
const rs = hSrc.match(/enum RememberedSetType \{([^}]*)\};/);
if (!rs) throw new Error("RememberedSetType 缺失");
const rsTypes = rs[1]
  .replace(/\/\/[^\n]*/g, "")
  .split(",")
  .map((s) => s.trim())
  .filter((s) => s && !s.includes("=") && s !== "NUMBER_OF_REMEMBERED_SET_TYPES");
if (rsTypes.length !== 8) throw new Error(`RememberedSetType ${rsTypes.length}, 期望 8`);
if (rsTypes.join() !== "OLD_TO_NEW,OLD_TO_NEW_BACKGROUND,OLD_TO_OLD,OLD_TO_SHARED,TRUSTED_TO_CODE,TRUSTED_TO_TRUSTED,TRUSTED_TO_SHARED_TRUSTED,SURVIVOR_TO_EXTERNAL_POINTER") {
  throw new Error(`RememberedSetType 序漂移: ${rsTypes.join()}`);
}
const rsIdx = new Map(rsTypes.map((n, i) => [n, i]));

// —— 释放序 (mutable-page.cc:183-205 ReleaseAllocatedMemoryNeededForWritableChunk) ——
const relStart = ccSrc.indexOf("void MutablePage::ReleaseAllocatedMemoryNeededForWritableChunk");
if (relStart < 0) throw new Error("ReleaseAllocatedMemoryNeededForWritableChunk 缺失");
const relEnd = ccSrc.indexOf("\n}", relStart);
const relBody = ccSrc.slice(relStart, relEnd);
const slotRelease = [...relBody.matchAll(/ReleaseSlotSet\((\w+)\)/g)].map((m) => {
  if (!rsIdx.has(m[1])) throw new Error(`释放序未知种类 ${m[1]}`);
  return rsIdx.get(m[1]);
});
const typedRelease = [...relBody.matchAll(/ReleaseTypedSlotSet\((\w+)\)/g)].map((m) => {
  if (!rsIdx.has(m[1])) throw new Error(`typed 释放序未知种类 ${m[1]}`);
  return rsIdx.get(m[1]);
});
if (slotRelease.length !== 8) throw new Error(`SlotSet 释放序 ${slotRelease.length}, 期望 8`);
if (typedRelease.length !== 3) throw new Error(`TypedSlotSet 释放序 ${typedRelease.length}, 期望 3`);
if (slotRelease.join() !== "0,1,2,4,3,5,6,7") throw new Error(`SlotSet 释放序漂移: ${slotRelease.join()}`);
if (typedRelease.join() !== "0,2,3") throw new Error(`TypedSlotSet 释放序漂移: ${typedRelease.join()}`);

// —— 旗标规则面: 逐函数 token 在位核对 ——
function fnBody(sig) {
  const i = ccSrc.indexOf(sig);
  if (i < 0) throw new Error(`${sig} 缺失`);
  const b = ccSrc.indexOf("{", i);
  let depth = 0;
  for (let j = b; j < ccSrc.length; j++) {
    if (ccSrc[j] === "{") depth++;
    if (ccSrc[j] === "}") {
      depth--;
      if (depth === 0) return ccSrc.slice(i, j + 1);
    }
  }
  throw new Error(`${sig} 函数体未闭合`);
}
const FLAG_TOKENS = [
  "POINTERS_TO_HERE_ARE_INTERESTING",
  "POINTERS_FROM_HERE_ARE_INTERESTING",
  "INCREMENTAL_MARKING",
  "STICKY_MARK_BIT_CONTAINS_ONLY_OLD",
  "STICKY_MARK_BIT_IS_MAJOR_GC_IN_PROGRESS",
  "IN_WRITABLE_SHARED_SPACE",
  "BLACK_ALLOCATED",
];
const flagFns = {};
for (const fn of [
  "OldGenerationPageFlags",
  "YoungGenerationPageFlags",
  "ComputeInitialFlags",
  "SetOldGenerationPageFlags",
  "SetYoungGenerationPageFlags",
]) {
  const body = fnBody(`MutablePage::${fn}(`);
  flagFns[fn] = FLAG_TOKENS.filter((t) => body.includes(t));
}
if (flagFns.OldGenerationPageFlags.join() !== FLAG_TOKENS.slice(0, 5).join()) {
  throw new Error(`OldGenerationPageFlags 旗标面漂移: ${flagFns.OldGenerationPageFlags.join()}`);
}
if (flagFns.YoungGenerationPageFlags.join() !== "POINTERS_TO_HERE_ARE_INTERESTING,POINTERS_FROM_HERE_ARE_INTERESTING,INCREMENTAL_MARKING,STICKY_MARK_BIT_IS_MAJOR_GC_IN_PROGRESS") {
  throw new Error(`YoungGenerationPageFlags 旗标面漂移: ${flagFns.YoungGenerationPageFlags.join()}`);
}
if (!flagFns.ComputeInitialFlags.includes("BLACK_ALLOCATED") || !flagFns.ComputeInitialFlags.includes("IN_WRITABLE_SHARED_SPACE")) {
  throw new Error("ComputeInitialFlags 旗标面漂移");
}

// —— 快字段缓存行规则锚 (cc:45-54) ——
if (!ccSrc.includes("kOffsetOfFirstFastField = offsetof(MutablePage, heap_)")) {
  throw new Error("快字段首锚缺失");
}
if (!/kOffsetOfLastFastField =\s*offsetof\(MutablePage, slot_set_\) \+\s*sizeof\(SlotSet\*\) \* RememberedSetType::OLD_TO_NEW/.test(ccSrc)) {
  throw new Error("快字段尾锚缺失");
}

// —— 方法清单 (cc + inl) ——
function collectDefs(src, re) {
  const out = [];
  const lines = src.split("\n");
  for (let i = 0; i < lines.length; i++) {
    const m = lines[i].match(re);
    if (!m) continue;
    let full = lines[i].trim();
    const tpl = /^template\s*</.test(lines[i - 1] ?? "") ? lines[i - 1].trim().replace(/\s+/g, " ") + " " : "";
    let j = i;
    while (!/\)\s*(?:const\s*)?[;{=]?\s*$/.test(full) && j + 1 < lines.length) {
      full += " " + lines[++j].trim();
    }
    out.push({ line: i + 1, name: m[1], sig: (tpl + full).replace(/\s+/g, " ").replace(/\s*\{\s*$/, "") });
  }
  return out;
}
const ccDefs = collectDefs(ccSrc, /^(?:[A-Za-z_][\w:<>,&*\[\] ]*[\s>])?MutablePage::([A-Za-z_~]\w*)\s*\(/);
const inlDefs = collectDefs(inlSrc, /^(?:[A-Za-z_][\w:<>,&*\[\] ]*[\s>])?MutablePage::([A-Za-z_~]\w*)\s*\(/);
if (ccDefs.length !== 19) throw new Error(`mutable-page.cc 定义数 ${ccDefs.length}, 期望 19`);
if (inlDefs.length !== 15) throw new Error(`mutable-page-inl.h 定义数 ${inlDefs.length}, 期望 15`);
// ContainsAnySlots 上界锚 (NUMBER_OF_REMEMBERED_SET_TYPES)
if (!ccSrc.includes("rs_type < NUMBER_OF_REMEMBERED_SET_TYPES")) throw new Error("ContainsAnySlots 上界锚缺失");

console.log(`mutable-page: RS 类型 ${rsTypes.length}, 释放序 ${slotRelease.join(",")}|${typedRelease.join(",")}, 定义 ${ccDefs.length}+${inlDefs.length}`);

let rust = banner(
  "src/heap/mutable-page.cc.js",
  "src/heap/mutable-page.{cc,inl.h} + mutable-page.h (枚举锚)",
  "记忆集释放序/页旗标规则面; 枚举面在 mutable_page_enums.rs",
);

const snake = (s) => s.replace(/^~/, "").replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();

rust += `
//! MutablePage 的机械规则面。
//!
//! - 释放序: 可写块释放时逐 RememberedSetType 释放 SlotSet (8) 与 TypedSlotSet (3)
//! - 旗标计算: OldGeneration/YoungGenerationPageFlags 按 MarkingMode × Space 分支
//! - 快字段 (heap_/slot_set_[0]) 同缓存行断言 (cc:45-54, 目标 x64 干扰行 64)

/// 记忆集种类总数 (NUMBER_OF_REMEMBERED_SET_TYPES)
pub const K_NUMBER_OF_REMEMBERED_SET_TYPES: usize = ${rsTypes.length};

/// 可写块释放时 SlotSet 的释放序 — 值 = RememberedSetType 判别值
/// v8: src/heap/mutable-page.cc:189-196 ReleaseAllocatedMemoryNeededForWritableChunk
pub const SLOT_SET_RELEASE_ORDER: [u8; ${slotRelease.length}] = [${slotRelease.join(", ")}];

/// 可写块释放时 TypedSlotSet 的释放序 — 值 = RememberedSetType 判别值
/// v8: src/heap/mutable-page.cc:197-199
pub const TYPED_SLOT_SET_RELEASE_ORDER: [u8; ${typedRelease.length}] = [${typedRelease.join(", ")}];

// —— 页旗标计算规则面 (token 在位由脚本核对; 位值属 memory-chunk 族, 不在此落值) ——
// OldGenerationPageFlags (cc:58-88), MarkingMode × AllocationSpace 分支:
//   kMajorMarking            → TO_HERE | FROM_HERE | INCREMENTAL_MARKING (+ STICKY_IS_MAJOR_GC)
//   writable shared 空间      → TO_HERE (OLD_TO_SHARED 追踪)
//   其余                     → FROM_HERE; kMinorMarking 再加 INCREMENTAL_MARKING
//   sticky 构建非 OLD_SPACE  → STICKY_MARK_BIT_CONTAINS_ONLY_OLD (V8_ENABLE_STICKY_MARK_BITS_BOOL)
// YoungGenerationPageFlags (cc:91-105):
//   基线 TO_HERE; marking != kNoMarking → + FROM_HERE + INCREMENTAL_MARKING
//   (+ kMajorMarking 再加 STICKY_IS_MAJOR_GC)
// ComputeInitialFlags (cc:107-140):
//   新生页 → Young 旗标; 老生页 → Old 旗标, black_allocation 且非大对象页时
//   摘 TO_HERE (可写共享页除外) 并置 BLACK_ALLOCATED; 可写共享页恒置 IN_WRITABLE_SHARED_SPACE
// SetOldGenerationPageFlags (cc:142-163) / SetYoungGenerationPageFlags (cc:165-176):
//   先置 set 面再清 clear 面 (SetFlagsNonExecutable + ClearFlagsNonExecutable)
`;

// —— 方法签名骨架 ——
rust += `
// —— 方法签名骨架 (语义实现由 Developer 就地修订) ——
`;
const seen = new Map();
for (const d of [...ccDefs.map((d) => ({ ...d, src: "cc" })), ...inlDefs.map((d) => ({ ...d, src: "inl.h" }))]) {
  const base = snake(d.name);
  const n = (seen.get(base) ?? 0) + 1;
  seen.set(base, n);
  const nm = n === 1 ? base : `${base}_v${n}`;
  rust += `/// 原版签名: ${d.sig}
/// v8: src/heap/mutable-page.${d.src}:${d.line}
pub fn mutable_page_${nm}() {
  todo!("就地修订")
}
`;
}

rust += `
// —— 提取不变式断言 ——
const _: () = {
  assert!(K_NUMBER_OF_REMEMBERED_SET_TYPES == 8);
  assert!(SLOT_SET_RELEASE_ORDER.len() == 8);
  assert!(SLOT_SET_RELEASE_ORDER[0] == 0);
  assert!(SLOT_SET_RELEASE_ORDER[1] == 1);
  assert!(SLOT_SET_RELEASE_ORDER[2] == 2);
  assert!(SLOT_SET_RELEASE_ORDER[3] == 4);
  assert!(SLOT_SET_RELEASE_ORDER[4] == 3);
  assert!(SLOT_SET_RELEASE_ORDER[5] == 5);
  assert!(SLOT_SET_RELEASE_ORDER[6] == 6);
  assert!(SLOT_SET_RELEASE_ORDER[7] == 7);
  assert!(TYPED_SLOT_SET_RELEASE_ORDER.len() == 3);
  assert!(TYPED_SLOT_SET_RELEASE_ORDER[0] == 0);
  assert!(TYPED_SLOT_SET_RELEASE_ORDER[1] == 2);
  assert!(TYPED_SLOT_SET_RELEASE_ORDER[2] == 3);
};
`;

const p = writeSrc("v8-heap-core", "mutable_page_cc.skel", rust);
console.log(`written ${p}`);
