#!/usr/bin/env bun
// tool/src/sandbox/trusted-pointer-table.h.js
// 机械提取 v8/src/sandbox/trusted-pointer-table.{h,-inl.h} —
// TrustedPointerTable tagging scheme 常量 (值解析自 indirect-pointer-tag.h,
// x64-LA57 / 其他目标双分支展开) + 类面骨架。
// 产物: v8-sandbox/src/trusted_pointer_table.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const H_REL = "src/sandbox/trusted-pointer-table.h";
const INL_REL = "src/sandbox/trusted-pointer-table-inl.h";
const h = readV8(H_REL);
const inl = readV8(INL_REL);
const tagH = readV8("src/sandbox/indirect-pointer-tag.h"); // scheme 常量实值来源

// scheme 绑定锚 (表头侧)
for (const anchor of [
  "using TagType = IndirectPointerTag;",
  "static constexpr uint64_t kMarkBit = kTrustedPointerTableMarkBit;",
  "static constexpr uint64_t kTagShift = kTrustedPointerTableTagShift;",
  "static constexpr uint64_t kTagMask = kTrustedPointerTableTagMask;",
  "static constexpr uint64_t kPayloadShift = kTrustedPointerTablePayloadShift;",
  "static constexpr uint64_t kPayloadMask = kTrustedPointerTablePayloadMask;",
  "static constexpr TagType kFreeEntryTag = kIndirectPointerFreeEntryTag;",
]) {
  if (!h.includes(anchor)) throw new Error(`缺 scheme 锚: ${anchor}`);
}
const writeProtected = /IsWriteProtected = (true|false)/.exec(h)[1] === "true";
if (writeProtected !== false) throw new Error("IsWriteProtected 预期 false");
const capM = h.match(/static_assert\((\w+) == kMaxCapacity\)/);
if (!capM) throw new Error("未找到容量 static_assert");

// 实值双分支解析: #if V8_TARGET_ARCH_X64 && V8_SUPPORT_LA57 … #else … #endif
function branchValues(beginMarker) {
  const begin = tagH.indexOf(beginMarker);
  if (begin < 0) throw new Error(`未找到分支锚 ${beginMarker}`);
  const elseAt = tagH.indexOf("#else", begin);
  const endAt = tagH.indexOf("#endif", begin);
  if (elseAt < 0 || endAt < 0 || elseAt > endAt) throw new Error("分支结构异常");
  const take = (seg) => Object.fromEntries(
    [...seg.matchAll(/constexpr uint64_t (\w+) = (0x[0-9a-fA-F']+'?[0-9a-fA-F']*|\d+);/g)]
      .map((m) => [m[1], BigInt(m[2].replace(/'/g, ""))]),
  );
  return {
    la57: take(tagH.slice(begin, elseAt)),
    other: take(tagH.slice(elseAt, endAt)),
  };
}
const br = branchValues("constexpr uint64_t kTrustedPointerTableTagMask");
const payloadShift = tagH.match(/constexpr uint64_t kTrustedPointerTablePayloadShift = (\d+);/);
if (!payloadShift) throw new Error("未找到 kTrustedPointerTablePayloadShift");
for (const k of ["kTrustedPointerTableTagMask", "kTrustedPointerTableMarkBit", "kTrustedPointerTablePayloadMask", "kTrustedPointerTableTagShift"]) {
  if (!(k in br.la57) || !(k in br.other)) throw new Error(`分支缺 ${k}`);
}
// x64-LA57: tag 7 位 (shift 57); 其他: tag 8 位 (shift 49) — 语义核对
if (br.la57.kTrustedPointerTableTagShift !== 57n || br.other.kTrustedPointerTableTagShift !== 49n) {
  throw new Error("TagShift 分支值与预期不符");
}
if (!inl.includes("TrustedPointerTableEntry")) throw new Error("inl 缺入口锚");

let rust = banner("src/sandbox/trusted-pointer-table.h.js", "src/sandbox/trusted-pointer-table.{h,-inl.h}", "TrustedPointerTable 骨架");
rust += `
// scheme 实值解析自 src/sandbox/indirect-pointer-tag.h, 按目标双分支展开:
// x64 (LA57 五级页表) 与其他目标 (arm64 等)。IndirectPointerTag 枚举本体
// 在 indirect-pointer-tag.h (既有 tags 覆盖面)。

/// 原 TrustedPointerTable::TrustedPointerTableTaggingScheme
pub mod trusted_pointer_table_tagging_scheme {
  /// TagType = IndirectPointerTag (u16)
  pub type TagType = u16;

  /// x64 + LA57 分支 (原 #if V8_TARGET_ARCH_X64 && V8_SUPPORT_LA57)
  pub mod la57 {
    /// 原 kTrustedPointerTableTagMask
    pub const TAG_MASK: u64 = 0x${br.la57.kTrustedPointerTableTagMask.toString(16).toUpperCase()};
    /// 原 kTrustedPointerTableMarkBit
    pub const MARK_BIT: u64 = 0x${br.la57.kTrustedPointerTableMarkBit.toString(16).toUpperCase()};
    /// 原 kTrustedPointerTablePayloadMask
    pub const PAYLOAD_MASK: u64 = 0x${br.la57.kTrustedPointerTablePayloadMask.toString(16).toUpperCase()};
    /// 原 kTrustedPointerTableTagShift
    pub const TAG_SHIFT: u64 = ${br.la57.kTrustedPointerTableTagShift};
  }

  /// 其他目标分支 (arm64 / x64 非 LA57)
  pub mod non_la57 {
    pub const TAG_MASK: u64 = 0x${br.other.kTrustedPointerTableTagMask.toString(16).toUpperCase()};
    pub const MARK_BIT: u64 = 0x${br.other.kTrustedPointerTableMarkBit.toString(16).toUpperCase()};
    pub const PAYLOAD_MASK: u64 = 0x${br.other.kTrustedPointerTablePayloadMask.toString(16).toUpperCase()};
    pub const TAG_SHIFT: u64 = ${br.other.kTrustedPointerTableTagShift};
  }

  /// 原 kTrustedPointerTablePayloadShift (双分支共用)
  pub const PAYLOAD_SHIFT: u64 = ${payloadShift[1]};
  /// 原 kFreeEntryTag = kIndirectPointerFreeEntryTag
  pub const FREE_ENTRY_TAG_NAME: &str = "kIndirectPointerFreeEntryTag";
  /// 原 kEvacuationEntryTag = kIndirectPointerEvacuationEntryTag
  pub const EVACUATION_ENTRY_TAG_NAME: &str = "kIndirectPointerEvacuationEntryTag";
}

/// 表是否写保护 (原 IsWriteProtected)
pub const IS_WRITE_PROTECTED: bool = false;

/// 容量断言锚: ${capM[1]} == kMaxCapacity
/// (kMaxTrustedPointers = kTrustedPointerTableReservationSize(64MB) / 8)
pub const CAPACITY_ASSERT_SOURCE: &str = "${capM[1]}";

/// 原 class TrustedPointerTable (Entry + Space)
pub struct TrustedPointerTable;

// TODO(dev): 依 ExternalEntityTable 基类补 Entry/Get/Set/Space 实现。

// —— 提取不变式断言 ——
const _: () = {
  use trusted_pointer_table_tagging_scheme as s;
  assert!(s::la57::TAG_SHIFT == 57);
  assert!(s::non_la57::TAG_SHIFT == 49);
  assert!(s::la57::PAYLOAD_MASK == 0x00FF_FFFF_FFFF_FFFF);
  assert!(s::non_la57::PAYLOAD_MASK == 0x0000_FFFF_FFFF_FFFF);
  assert!(!IS_WRITE_PROTECTED);
};
`;

const p = writeSrc("v8-sandbox", "trusted_pointer_table.rs", rust);
console.log(`written ${p} (scheme 双分支 4+4+1 常量)`);
