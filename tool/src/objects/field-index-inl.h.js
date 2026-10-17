#!/usr/bin/env bun
// tool/src/objects/field-index-inl.h.js — 覆盖 v8/src/objects/field-index-inl.h
// → v8-objects-maps/src/field_index_inl.rs  (席位表 R3O4 目标 crate; 2026-10-12 归位)
// 机械提取: 5 个工厂入口 + GetLoadByFieldIndex 编码规则 + 3 个 static_assert 锚。

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/objects/field-index-inl.h");

let checks = 0;
const must = (c, m) => { checks++; if (!c) throw new Error(`[field-index-inl.h.js] ${m}`); };

// ── 1. 方法定义面 (5 个 FieldIndex 工厂 + 1 个编码出口) ────────────────────
const factories = [...src.matchAll(/^FieldIndex FieldIndex::(\w+)\(/gm)].map((m) => m[1]);
must(factories.join(",") === "ForInObjectOffset,ForSmiLoadHandler,ForPropertyIndex,ForDescriptor,ForDetails",
  `工厂序漂移: ${factories.join(",")}`);
const encoders = [...src.matchAll(/^int FieldIndex::(\w+)\(\) const/gm)].map((m) => m[1]);
must(encoders.join(",") === "GetLoadByFieldIndex", `编码出口序漂移: ${encoders.join(",")}`);

// ── 2. 对齐断言 (3 个 Encoding × kWord32/kTagged/kDouble) ──────────────────
must(src.includes("DCHECK_IMPLIES(encoding == kWord32, IsAligned(offset, kInt32Size))"), "缺 kWord32 对齐断言");
must(src.includes("DCHECK_IMPLIES(encoding == kTagged, IsAligned(offset, kTaggedSize))"), "缺 kTagged 对齐断言");
must(src.includes("DCHECK_IMPLIES(encoding == kDouble, IsAligned(offset, kDoubleSize))"), "缺 kDouble 对齐断言");

// ── 3. SmiLoadHandler 解码位 (is_inobject / storage 偏移 / is_double) ───────
for (const anchor of [
  "LoadHandler::IsInobjectBits::decode(handler)",
  "LoadHandler::StorageOffsetInWordsBits::decode(handler) * kTaggedSize",
  "LoadHandler::IsDoubleBits::decode(handler) ? kDouble : kTagged",
]) must(src.includes(anchor), `缺 LoadHandler 锚: ${anchor}`);

// ── 4. PropertyArray 数据起点的 static_assert 链 (2 条, 位宽护栏) ──────────
const saCount = [...src.matchAll(/static_assert\(OFFSET_OF_DATA_START\(PropertyArray\)/g)].length;
must(saCount === 2, `PropertyArray static_assert 数 ${saCount} != 2`);
must(src.includes("(1 << FieldIndex::kPropertyArrayDataStartBitCount)"), "缺 kPropertyArrayDataStartBitCount 上界断言");
must(src.includes("(1 << (FieldIndex::kPropertyArrayDataStartBitCount - 1))"), "缺 kPropertyArrayDataStartBitCount 下界断言");

// ── 5. GetLoadByFieldIndex 编码规则锚 ───────────────────────────────────────
must(src.includes("result -= JSObject::kHeaderSize / kTaggedSize;"), "缺 inobject 基准扣减");
must(src.includes("result -= OFFSET_OF_DATA_START(PropertyArray) / kTaggedSize;"), "缺 out-of-object 基准扣减");
must(src.includes("result = -result - 1;"), "缺 out-of-object 取负减一 (消零歧义)");
must(src.includes("result = static_cast<uint32_t>(result) << 1;"), "缺左移一位置 double 位");
must(src.includes("is_double() ? (result | 1) : result"), "缺最低 mutable-double 标志位");

// ── 6. ForDetails/ForDescriptor 锚 ──────────────────────────────────────────
must(src.includes("details.is_in_object()"), "缺 is_in_object 判定");
must(src.includes("details.field_offset() * kTaggedSize"), "缺 field_offset 字节化");
must(src.includes("map->instance_descriptors(kAcquireLoad)->GetDetails(descriptor_index)"), "缺 descriptor 读 (AcquireLoad)");

let rust = banner("src/objects/field-index-inl.h.js", "src/objects/field-index-inl.h",
  "FieldIndex 工厂入口 + LoadFieldByIndex 编码规则");
rust += `//! FieldIndex inline 面: 由 Map/PropertyDetails/SmiLoadHandler 构造字段索引。
//! (与 field_index.rs 的位段定义互补; 语义留就地实现)

// ── 工厂入口 (${factories.length} 个) ──
`;
const factoryNotes = {
  ForInObjectOffset: "offset 直取, encoding 显式给 (kWord32/kTagged/kDouble 对齐断言 ×3)",
  ForSmiLoadHandler: "LoadHandler 位解码: IsInobjectBits + StorageOffsetInWordsBits*kTaggedSize + IsDoubleBits",
  ForPropertyIndex: "inobject 首选, 越界转 PropertyArray (2 条 static_assert 护数据起点位宽)",
  ForDescriptor: "读 instance_descriptors(kAcquireLoad) 的 details 后走 ForDetails",
  ForDetails: "details.is_in_object() + field_offset()*kTaggedSize + FieldEncoding(representation)",
};
for (const f of factories) {
  const snake = f.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();
  rust += `/// 原版 FieldIndex::${f} — ${factoryNotes[f]}。\npub fn ${snake}() -> FieldIndex {\n  todo!("就地修订: field-index-inl.h")\n}\n\n`;
}
rust += `// ── LoadFieldByIndex 编码 (CSA 指令索引用) ──
/// 原版 FieldIndex::GetLoadByFieldIndex — inobject 自 JSObject::kHeaderSize 起零基;
/// out-of-object 自 PropertyArray 数据起点起零基且取 -raw-1 消零歧义;
/// 整体 <<1, 最低位 = mutable double box 标志。
pub fn get_load_by_field_index(offset_in_words: i32, is_inobject: bool, is_double: bool) -> i32 {
  let mut result = offset_in_words;
  if is_inobject {
    result -= JS_OBJECT_HEADER_SIZE_IN_WORDS;
  } else {
    result -= PROPERTY_ARRAY_DATA_START_IN_WORDS;
    result = -result - 1;
  }
  let result = ((result as u32) << 1) as i32;
  if is_double { result | 1 } else { result }
}

// ── 提取不变式断言 ──
const _: () = {
  // -raw-1 消零歧义: out-of-object 第 0 槽编码为 -1 (<<1 前为奇负)
  assert!(get_load_by_field_index(PROPERTY_ARRAY_DATA_START_IN_WORDS, false, false) == -2);
  assert!(get_load_by_field_index(PROPERTY_ARRAY_DATA_START_IN_WORDS + 1, false, false) == -4);
  // inobject: 相对 JSObject 头的字槽数, 正数
  assert!(get_load_by_field_index(JS_OBJECT_HEADER_SIZE_IN_WORDS, true, false) == 0);
  // double 标志占最低位
  assert!(get_load_by_field_index(JS_OBJECT_HEADER_SIZE_IN_WORDS, true, true) == 1);
};
`;

const p = writeSrc("v8-objects-maps", "field_index_inl.rs", rust);
console.log(`field-index-inl.h.js: ${p} — 工厂 ${factories.length} + 编码 1, 对齐断言 3, static_assert ${saCount}, 断言 ${checks} 项全过`);
