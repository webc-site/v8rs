#!/usr/bin/env bun
// tool/src/sandbox/cppheap-pointer-table.h.js
// 机械提取 v8/src/sandbox/cppheap-pointer-table.{h,-inl.h} —
// CppHeapPointerTable tagging scheme 常量 (值解析自 include/v8-internal.h) + 类面骨架。
// 产物: v8-sandbox/src/cppheap_pointer_table.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const H_REL = "src/sandbox/cppheap-pointer-table.h";
const INL_REL = "src/sandbox/cppheap-pointer-table-inl.h";
const h = readV8(H_REL);
const inl = readV8(INL_REL);
const internal = readV8("include/v8-internal.h"); // scheme 常量实值来源

function needInternal(name) {
  const m = internal.match(new RegExp(`constexpr\\s+u?int(?:64|32)_t\\s+${name}\\s*=\\s*([\\s\\S]*?);`));
  if (!m) throw new Error(`v8-internal.h 未找到 ${name}`);
  return m[1].replace(/\s+/g, " ").trim();
}

// scheme 绑定锚 (表头侧)
for (const anchor of [
  "using TagType = CppHeapPointerTag;",
  "static constexpr uint64_t kMarkBit = kCppHeapPointerMarkBit;",
  "static constexpr uint64_t kTagShift = kCppHeapPointerTagShift;",
  "static constexpr uint64_t kTagMask = kCppHeapPointerTagMask;",
  "static constexpr uint64_t kPayloadShift = kCppHeapPointerPayloadShift;",
]) {
  if (!h.includes(anchor)) throw new Error(`缺 scheme 锚: ${anchor}`);
}
const writeProtected = /IsWriteProtected = (true|false)/.exec(h)[1] === "true";
if (writeProtected !== false) throw new Error("IsWriteProtected 预期 false");
const capM = h.match(/static_assert\((\w+) == Base::kMaxCapacity\)/);
if (!capM) throw new Error("未找到容量 static_assert");

// 实值解析 (无条件常量)
const markBit = needInternal("kCppHeapPointerMarkBit");
const tagShift = needInternal("kCppHeapPointerTagShift");
const payloadShift = needInternal("kCppHeapPointerPayloadShift");
const tagMaskExpr = needInternal("kCppHeapPointerTagMask");
// TagMask = ((1<<(payloadShift-tagShift))-1)<<tagShift → 算出字面值
const tagMask = ((1n << (BigInt(payloadShift) - BigInt(tagShift))) - 1n) << BigInt(tagShift);
if (!tagMaskExpr.includes("kCppHeapPointerPayloadShift")) throw new Error("TagMask 表达式锚不符");

// kNullCppHeapPointerHandle
const nullM = internal.match(/constexpr CppHeapPointerHandle kNullCppHeapPointerHandle = (\d+);/);
if (!nullM) throw new Error("未找到 kNullCppHeapPointerHandle");

// inl 侧模板访问器计数 (template 块锚)
const inlTplCount = [...inl.matchAll(/^template </gm)].length;
if (inlTplCount < 2) throw new Error(`inl 模板访问器数异常: ${inlTplCount}`);

let rust = banner("src/sandbox/cppheap-pointer-table.h.js", "src/sandbox/cppheap-pointer-table.{h,-inl.h}", "CppHeapPointerTable 骨架");
rust += `
// scheme 实值解析自 include/v8-internal.h; TagType 枚举在
// src/sandbox/indirect-pointer-tag.h (既有覆盖面)。

/// 原 CppHeapPointerTable::CppHeapPointerTablePayloadTaggingScheme
pub mod cppheap_pointer_table_payload_tagging_scheme {
  /// TagType = CppHeapPointerTag (indirect-pointer-tag.h)
  pub type TagType = u16;
  /// 原 kCppHeapPointerMarkBit
  pub const MARK_BIT: u64 = ${markBit};
  /// 原 kCppHeapPointerTagShift
  pub const TAG_SHIFT: u64 = ${tagShift};
  /// 原 kCppHeapPointerTagMask = ${tagMaskExpr}
  pub const TAG_MASK: u64 = 0x${tagMask.toString(16).toUpperCase()};
  /// 原 kCppHeapPointerPayloadShift
  pub const PAYLOAD_SHIFT: u64 = ${payloadShift};
  /// 原 kPayloadMask = ~0ULL (payload 全宽)
  pub const PAYLOAD_MASK: u64 = u64::MAX;
  /// 原 kFreeEntryTag = CppHeapPointerTag::kFreeEntryTag
  pub const FREE_ENTRY_TAG_NAME: &str = "kFreeEntryTag";
  /// 原 kEvacuationEntryTag = CppHeapPointerTag::kEvacuationEntryTag
  pub const EVACUATION_ENTRY_TAG_NAME: &str = "kEvacuationEntryTag";
}

/// 原 CppHeapPointerHandle = uint32_t (v8-internal.h)
pub type CppHeapPointerHandle = u32;

/// 表是否写保护 (原 IsWriteProtected)
pub const IS_WRITE_PROTECTED: bool = false;

/// 原 kNullHandle = kNullCppHeapPointerHandle
pub const NULL_HANDLE: CppHeapPointerHandle = ${nullM[1]};

/// 容量断言锚: ${capM[1]} == ExternalEntityTable::kMaxCapacity
/// (kMaxCppHeapPointers = kCppHeapPointerTableReservationSize / 8, 随压缩/沙箱配置分支)
pub const CAPACITY_ASSERT_SOURCE: &str = "${capM[1]}";

/// 原 class CppHeapPointerTable (Entry + Space + inl 访问器, template 块 ${inlTplCount} 处)
pub struct CppHeapPointerTable;

// TODO(dev): 依 ExternalEntityTable 基类补 Entry/Space/Get/Set 实现。

// —— 提取不变式断言 ——
const _: () = {
  use cppheap_pointer_table_payload_tagging_scheme as s;
  assert!(s::MARK_BIT == 1);
  assert!(s::TAG_SHIFT == 1);
  assert!(s::PAYLOAD_SHIFT == 16);
  assert!(s::TAG_MASK == 0xFFFE);
  assert!(NULL_HANDLE == 0);
  assert!(!IS_WRITE_PROTECTED);
};
`;

const p = writeSrc("v8-sandbox", "cppheap_pointer_table.rs", rust);
console.log(`written ${p} (scheme 5+2 常量, inl template ${inlTplCount})`);
