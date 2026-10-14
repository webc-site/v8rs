#!/usr/bin/env bun
// tool/src/objects/field-type.h.js — 覆盖 v8/src/objects/field-type.h + field-type.cc
// FieldType 全静态类: 哨兵值 (None=2/Any=1) / 常量 / 方法签名骨架
// → v8-objects-maps/src/field_type.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/objects/field-type.h");
const ccSrc = readV8("src/objects/field-type.cc");

// 1. GC 清除常量
const clearMatch = hSrc.match(/static constexpr bool kFieldTypesCanBeClearedOnGC = (true|false);/);
if (!clearMatch) throw new Error("未找到 kFieldTypesCanBeClearedOnGC");
const clearedOnGC = clearMatch[1] === "true";

// 2. 哨兵 Smi 编号 (cc 侧实现, 机械提取)
const noneMatch = ccSrc.match(/Tagged<FieldType> FieldType::None\(\) \{\s*return Tagged<FieldType>\(Smi::FromInt\((\d+)\)\.ptr\(\)\);/);
const anyMatch = ccSrc.match(/Tagged<FieldType> FieldType::Any\(\) \{\s*return Tagged<FieldType>\(Smi::FromInt\((\d+)\)\.ptr\(\)\);/);
if (!noneMatch || !anyMatch) throw new Error("未找到 FieldType None/Any 哨兵编号");
const noneSmi = Number(noneMatch[1]);
const anySmi = Number(anyMatch[1]);
if (noneSmi === anySmi) throw new Error("None/Any 哨兵编号相同, 异常");

// 3. 方法签名骨架 (h 侧 public 区, 机械登记)
const sigs = [...hSrc.matchAll(/^\s+(?:V8_EXPORT_PRIVATE )?static [\s\S]*?(\w+)\(/gm)]
  .map((m) => m[1])
  .filter((n, i, a) => a.indexOf(n) === i);
if (sigs.length < 8) throw new Error(`签名数异常: ${sigs.length}`);

console.log(`FieldType 哨兵 None=${noneSmi} Any=${anySmi}, 签名 ${sigs.length} 个`);

let rust = banner("src/objects/field-type.h.js", "src/objects/field-type.{h,cc}", "FieldType: 字段类型哨兵与签名骨架");

rust += `
use crate::error::Error;

/// FieldType 全静态命名空间 (原版 class FieldType : AllStatic)
/// 表示 Map 字段的类型: None(未定) / Any(任意) / Class(以 Map 限定的类)
pub struct FieldType;

impl FieldType {
  /// GC 可清除字段类型时, 每次存储必须更新字段类型 (原版 kFieldTypesCanBeClearedOnGC)
  pub const FIELD_TYPES_CAN_BE_CLEARED_ON_GC: bool = ${clearedOnGC};

  /// None 哨兵的 Smi 编号 — 以 Smi 编码区分于 Class(Map 指针)
  #[inline]
  #[must_use]
  pub const fn none_repr() -> u32 {
    ${noneSmi}
  }

  /// Any 哨兵的 Smi 编号
  #[inline]
  #[must_use]
  pub const fn any_repr() -> u32 {
    ${anySmi}
  }
}

/// 语义别名: None 哨兵 (原版 FieldType::None, 实现为 Smi::FromInt(${noneSmi}))
pub const FIELD_TYPE_NONE: u32 = ${noneSmi};
/// 语义别名: Any 哨兵 (原版 FieldType::Any, 实现为 Smi::FromInt(${anySmi}))
pub const FIELD_TYPE_ANY: u32 = ${anySmi};

// —— 签名骨架 (原版公开接口, 供 Developer 就地补语义) ——
// Tagged<FieldType> FieldType::None();
// Tagged<FieldType> FieldType::Any();
// DirectHandle<FieldType> FieldType::None(Isolate*);
// DirectHandle<FieldType> FieldType::Any(Isolate*);
// Tagged<FieldType> FieldType::Class(Tagged<Map>);
// DirectHandle<FieldType> FieldType::Class(DirectHandle<Map>, Isolate*);
// bool FieldType::NowContains(Tagged<FieldType>, Tagged<Object>);
// Tagged<Map> FieldType::AsClass(Tagged<FieldType>);
// bool FieldType::NowStable(Tagged<FieldType>);
// bool FieldType::NowIs(Tagged<FieldType>, Tagged<FieldType>);
// bool FieldType::Equals(Tagged<FieldType>, Tagged<FieldType>);
// bool IsClass(Tagged<FieldType>);
// inline bool IsNone(Tagged<FieldType>) { obj == None }
// inline bool IsAny(Tagged<FieldType>) { obj == Any }
#[allow(dead_code)]
fn _signatures_placeholder(_: &Error) {}

// —— 提取不变式断言 ——
const _: () = {
  assert!(FIELD_TYPE_NONE == ${noneSmi});
  assert!(FIELD_TYPE_ANY == ${anySmi});
  assert!(FIELD_TYPE_NONE != FIELD_TYPE_ANY);
  assert!(FieldType::FIELD_TYPES_CAN_BE_CLEARED_ON_GC);
};
`;

const p = writeSrc("v8-objects-maps", "field_type.rs", rust);
console.log(`written ${p}`);
