#!/usr/bin/env bun
// tool/src/objects/data-handler.h.js — 覆盖 v8/src/objects/data-handler.{h,-inl.h,tq}
// DataHandler 抽象基: 弹性数据域公式 (OffsetOf/SizeFor) 与字段面
// → v8-objects-rest/src/data_handler.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/objects/data-handler.h");
const inlSrc = readV8("src/objects/data-handler-inl.h");
const tqSrc = readV8("src/objects/data-handler.tq");

// 1. 结构锚点
const anchors = [
  [hSrc, "V8_OBJECT class DataHandler : public Struct"],
  [hSrc, "static constexpr int OffsetOf(int index);"],
  [hSrc, "static constexpr int SizeFor(int count);"],
  [inlSrc, "constexpr int DataHandler::OffsetOf(int index)"],
  [inlSrc, "constexpr int DataHandler::SizeFor(int count)"],
  [tqSrc, "extern class DataHandler extends Struct"],
];
for (const [src, a] of anchors) {
  if (!src.includes(a)) throw new Error(`data-handler 缺锚点: ${a}`);
}

// 2. 公式提取 (inl): OFFSET_OF_DATA_START + i * sizeof(FlexibleDataType)
const offFormula = inlSrc.match(/return OFFSET_OF_DATA_START\(DataHandler\) \+\n?\s*index \* sizeof\(DataHandler::FlexibleDataType\);/);
const sizeFormula = inlSrc.match(/return OFFSET_OF_DATA_START\(DataHandler\) \+\n?\s*count \* sizeof\(DataHandler::FlexibleDataType\);/);
if (!offFormula || !sizeFormula) throw new Error("OffsetOf/SizeFor 公式与预期不符");

// 3. 弹性数据域数量 (h 侧 data1..data5)
const dataFields = [...hSrc.matchAll(/inline Tagged<MaybeObject> data(\d)\(\) const;/g)].map((m) => Number(m[1]));
if (dataFields.length !== 5 || dataFields[0] !== 1 || dataFields[4] !== 5) throw new Error(`弹性域数不符: ${dataFields}`);

// 4. tq 布局字段序
const tqBody = tqSrc.match(/extern class DataHandler extends Struct \{([\s\S]*?)\n\}/);
if (!tqBody) throw new Error("tq 布局体缺失");
const tqFields = [...tqBody[1].matchAll(/^\s*(\w+):/gm)].map((m) => m[1]);
if (tqFields.length !== 2) throw new Error(`tq 字段数不符: ${tqFields.length}, 期望 2 (data1..5 为 C++ 弹性域, 不入 tq)`);

console.log(`DataHandler 弹性域 5, tq 字段 ${tqFields.length}`);

let rust = banner("src/objects/data-handler.h.js", "src/objects/data-handler.{h,-inl.h,tq}", "DataHandler 抽象基: 弹性域公式与字段面");

rust += `
/// 数据处理器抽象基 (原版 V8_OBJECT class DataHandler : Struct, V8_IT_ABSTRACT)
/// LoadHandler/StoreHandler 的不可 Smi 编码形态基类
pub struct DataHandler;

impl DataHandler {
  /// 弹性域字节宽 (原版 sizeof(DataHandler::FlexibleDataType) = MaybeObject 槽宽)
  /// 数值由布局求解器定版, 公式按参数直译
  #[inline]
  #[must_use]
  pub const fn offset_of(data_start: usize, flexible_size: usize, index: usize) -> usize {
    data_start + index * flexible_size
  }

  /// 原版 SizeFor
  #[inline]
  #[must_use]
  pub const fn size_for(data_start: usize, flexible_size: usize, count: usize) -> usize {
    data_start + count * flexible_size
  }
}

/// tq 布局字段序 (原版 extern class DataHandler)
pub const TQ_FIELDS: [&str; ${tqFields.length}] = [
${tqFields.map((f) => `  "${f}",`).join("\n")}
];

/// 可选数据域容量 (原版 data1..data5)
pub const MAX_DATA_FIELD_COUNT: usize = 5;

// —— 签名骨架 ——
// smi_handler / set_smi_handler(Smi|Code) / validity_cell / set_validity_cell
// data_field_count / data1..data5 + set_data1..5 (MaybeObject, 写屏障)
pub const SIGNATURES: [&str; 5] = ["smi_handler", "validity_cell", "data_field_count", "data_slots", "verify"];

// —— 提取不变式断言 ——
const _: () = {
  assert!(MAX_DATA_FIELD_COUNT == 5);
  assert!(DataHandler::size_for(8, 4, 5) == 28);
  assert!(DataHandler::offset_of(8, 4, 3) == 20);
  assert!(TQ_FIELDS.len() == 2);
  assert!(TQ_FIELDS[0] == "${tqFields[0]}");
};
`;

const p = writeSrc("v8-objects-rest", "data_handler.rs", rust);
console.log(`written ${p}`);
