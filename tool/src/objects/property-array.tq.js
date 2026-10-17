#!/usr/bin/env bun
// tool/src/objects/property-array.tq.js — 覆盖 v8/src/objects/property-array.tq
// (差集: h/-inl.h 归既有 property-array.h.js 席, 产物 v8-objects-maps)
// → v8-objects-arrays/src/property_array_tq.rs.skel
// 机械提取: PropertyArray tq 布局 (length_and_hash) + EmptyPropertyArray 别名。

import { readV8, writeSrc, banner } from "../../lib.js";

const tq = readV8("src/objects/property-array.tq");

let checks = 0;
const must = (c, m) => {
  checks++;
  if (!c) throw new Error(`锚点失配: ${m}`);
};

// ── 1. tq 布局 ─────────────────────────────────────────────────────────
const cls = tq.match(/extern class (\w+) extends (\w+) \{\s*\n\s*(\w+):\s*([^;]+);\s*\n\}/);
if (!cls) throw new Error("未找到 PropertyArray tq 类");
must(cls[1] === "PropertyArray" && cls[2] === "HeapObject",
  `tq 类失配: ${cls[1]} extends ${cls[2]}`);
must(cls[3] === "length_and_hash" && cls[4] === "Smi",
  `字段失配: ${cls[3]}: ${cls[4]} (期望 length_and_hash: Smi)`);
must(tq.includes("@cppObjectLayoutDefinition"), "@cppObjectLayoutDefinition 注解锚缺失");

// ── 2. 别名 ────────────────────────────────────────────────────────────
must(tq.includes("type EmptyPropertyArray extends PropertyArray;"), "EmptyPropertyArray 别名锚缺失");

// ── 3. h 侧交叉锚 (只读, 规则面归既有席; 此处仅确认 length/hash 位段存在) ──
const h = readV8("src/objects/property-array.h");
must(h.includes("static const int kLengthFieldSize = 10;"), "kLengthFieldSize=10 锚缺失");
must(h.includes("using LengthField = base::BitField<int, 0, kLengthFieldSize>;"), "LengthField 位段锚缺失");
must(h.includes("using HashField = base::BitField<int, kLengthFieldSize,"), "HashField 位段锚缺失");
must(h.includes("kSmiValueSize - kLengthFieldSize - 1>"), "HashField 位宽公式锚缺失");
must(h.includes("static const int kMaxLength = LengthField::kMax;"), "kMaxLength 公式锚缺失");
// 配置登记: 64 位非压缩 → kSmiValueSize=32 → HashField bits = 32-10-1 = 21; kMaxLength = 2^10-1
const K_MAX_LENGTH = (1 << 10) - 1;
must(K_MAX_LENGTH === 1023, "kMaxLength 推导失配");

// ── 4. 生成 ────────────────────────────────────────────────────────────
const body = `${banner("src/objects/property-array.tq.js", "src/objects/property-array.tq", "差集 (h/-inl.h 归 property-array.h.js 席): tq 布局 + 位段交叉锚")}
//! PropertyArray: 属性数组 (单 Smi 头 length_and_hash 打包长度与哈希)。
//! 语义面 (Grow/Shrink/CopyUpTo/Hash 计算) 归 v8-objects-maps 的 property_array.rs 既有席。

/// tq 布局: HeapObject 头 (map) + length_and_hash (Smi, offset = kTaggedSize=8)。
pub const PROPERTY_ARRAY_TQ_FIELDS: &[(&str, &str)] = &[
  ("length_and_hash", "Smi"),
];

/// length_and_hash Smi 位段布局 (h 侧 LengthField/HashField 交叉锚;
/// kSmiValueSize 配置登记: 64 位非压缩 = 32 → HashField bits = 21)。
pub const LENGTH_FIELD_SHIFT: u32 = 0;
pub const LENGTH_FIELD_BITS: u32 = 10;
pub const HASH_FIELD_SHIFT: u32 = 10;
pub const HASH_FIELD_BITS: u32 = 21;

/// 原版 kMaxLength = LengthField::kMax。
pub const K_MAX_LENGTH: u32 = ${K_MAX_LENGTH};

/// tq 别名: 空属性数组 (kEmptyPropertyArray 单例形态)。
pub const EMPTY_PROPERTY_ARRAY_ALIAS: &str = "EmptyPropertyArray";

// —— 提取不变式断言 ——
const _: () = {
  assert!(LENGTH_FIELD_SHIFT == 0);
  assert!(LENGTH_FIELD_BITS == 10);
  assert!(HASH_FIELD_SHIFT == 10);
  assert!(HASH_FIELD_BITS == 21);
  assert!(K_MAX_LENGTH == 1023);
  assert!(PROPERTY_ARRAY_TQ_FIELDS.len() == 1);
};
`;

const p = writeSrc("v8-objects-arrays", "property_array_tq.rs.skel", body);
console.log(`property-array.tq.js: ${p} — tq 字段 1、位段 2 (0/10 + 10/21)、kMaxLength ${K_MAX_LENGTH}、断言 ${checks} 项通过`);
