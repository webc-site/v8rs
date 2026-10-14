#!/usr/bin/env bun
// tool/src/objects/string.tq.js — 覆盖 v8/src/objects/string.tq
//   → v8-objects-collections/src/string.rs
// 机械提取: StringRepresentationTag 枚举 + StringInstanceType 位段
//   + String 类族层级 (11 个 extern class) + 长度上限制度。

import { readV8, writeSrc, banner } from "../../lib.js";

const tq = readV8("src/objects/string.tq");

// ── StringRepresentationTag (tq 顺序值) ───────────────────────────────────
const tagM = tq.match(/extern enum StringRepresentationTag extends uint32 \{([\s\S]*?)\}/);
if (!tagM) throw new Error("未找到 StringRepresentationTag");
const tags = tagM[1].split(",").map((s) => s.trim()).filter(Boolean);
if (tags.join(",") !== "kSeqStringTag,kConsStringTag,kExternalStringTag,kSlicedStringTag,kThinStringTag") {
  throw new Error(`StringRepresentationTag 漂移: ${tags.join(",")}`);
}

// ── StringInstanceType 位段 ───────────────────────────────────────────────
const bfM = tq.match(/bitfield struct StringInstanceType extends uint16 \{([\s\S]*?)\n\}/);
if (!bfM) throw new Error("未找到 StringInstanceType");
const fields = [...bfM[1].matchAll(/(\w+): (\w+): (\d+) bit/g)].map((f) => ({ name: f[1], ty: f[2], bits: Number(f[3]) }));
if (fields.length !== 5) throw new Error(`StringInstanceType 字段应 5, 实得 ${fields.length}`);
const totalBits = fields.reduce((a, f) => a + f.bits, 0);
if (totalBits !== 7) throw new Error(`StringInstanceType 总位宽应 7, 实得 ${totalBits}`);
if (fields[0].name !== "representation" || fields[0].bits !== 3) throw new Error("representation 位段失配");

// @reserveBitsInInstanceType(7) 与位段总宽对齐
if (!tq.includes("@reserveBitsInInstanceType(7)")) throw new Error("reserveBitsInInstanceType(7) 锚失配");

// ── 类族层级 (extern class 清单 + 数量 must 断言) ─────────────────────────
const classes = [...tq.matchAll(/extern class (\w+) extends ([\w| ]+) \{/g)].map((m) => ({ name: m[1], base: m[2].trim() }));
const EXPECTED_CLASSES = [
  ["String", "Name"], ["ConsString", "String"], ["ExternalString", "String"],
  ["ExternalOneByteString", "ExternalString"], ["ExternalTwoByteString", "ExternalString"],
  ["InternalizedString", "String"], ["SeqString", "String"], ["SeqOneByteString", "SeqString"],
  ["SeqTwoByteString", "SeqString"], ["SlicedString", "String"], ["ThinString", "String"],
];
if (classes.length !== 11) throw new Error(`extern class 应 11, 实得 ${classes.length}`);
for (const [n, b] of EXPECTED_CLASSES) {
  if (!classes.some((c) => c.name === n && c.base === b)) throw new Error(`类层级失配: ${n} extends ${b}`);
}

// ── 字段布局锚 ────────────────────────────────────────────────────────────
const fieldAnchors = [
  "const length: int32;",
  "first: String;\n  second: String;",
  "parent: String;\n  offset: Smi;",
  "actual: InternalizedString;",
  "const chars[length]: char8;",
  "const chars[length]: char16;",
];
for (const a of fieldAnchors) {
  if (!tq.includes(a)) throw new Error(`字段布局锚失配: ${a.slice(0, 40)}`);
}

// ── 长度上限 (string.h:552 String::kMaxLength = v8::String::kMaxLength) ──
const stringH = readV8("src/objects/string.h");
if (!stringH.includes("static const uint32_t kMaxLength = v8::String::kMaxLength;")) {
  throw new Error("String::kMaxLength 锚失配");
}
const v8StringH = readV8("include/v8-primitive.h");
// 非 LOWER_LIMITS 配置: 32 位 (1<<28)-16 / 64 位 (1<<29)-24 (v8-primitive.h:129-130)
if (!v8StringH.includes("internal::kApiSystemPointerSize == 4 ? (1 << 28) - 16 : (1 << 29) - 24;")) {
  throw new Error("v8::String::kMaxLength 公式锚失配 (64 位应 (1<<29)-24)");
}
const K_MAX_LENGTH_64 = (1 << 29) - 24;
// 静态断言规则面: StaticAssertStringLengthFitsSmi (tq:360-164)
if (!tq.includes("macro StaticAssertStringLengthFitsSmi(): void {")) throw new Error("StaticAssertStringLengthFitsSmi 锚失配");

const body = `${banner("src/objects/string.tq.js", "src/objects/string.tq + 锚 src/objects/string.h", "String 类族: 表示标签/实例类型位段/层级规则面")}
//! String 的 Torque 侧规则面。
//!
//! instance_type 低 7 位 (@reserveBitsInInstanceType(7)):
//! - bit0-2: representation (StringRepresentationTag)
//! - bit3: is_one_byte; bit4: is_uncached; bit5: is_shared; bit6: is_not_internalized

// ── StringRepresentationTag (string.tq:54-60, 顺序值) ──────────────────
/// 字符串表示法标签 (instance_type bit0-2)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum StringRepresentationTag {
  /// 顺序字符串。
  SeqStringTag = 0,
  /// 拼接字符串 (first/second)。
  ConsStringTag = 1,
  /// 外部字符串 (resource/resource_data 指针)。
  ExternalStringTag = 2,
  /// 切片字符串 (parent/offset)。
  SlicedStringTag = 3,
  /// 薄字符串 (actual 指向内部化串)。
  ThinStringTag = 4,
}

/// 表示法标签位宽。
pub const K_STRING_REPRESENTATION_BITS: u32 = 3;

// ── StringInstanceType 位段 (string.tq:62-68) ─────────────────────────
/// is_one_byte 移位 (bit3)。
pub const IS_ONE_BYTE_SHIFT: u32 = 3;
/// is_uncached 移位 (bit4; ExternalString 无 resource_data)。
pub const IS_UNCACHED_SHIFT: u32 = 4;
/// is_shared 移位 (bit5)。
pub const IS_SHARED_SHIFT: u32 = 5;
/// is_not_internalized 移位 (bit6)。
pub const IS_NOT_INTERNALIZED_SHIFT: u32 = 6;
/// 位段总宽 (与 @reserveBitsInInstanceType(7) 对齐)。
pub const K_STRING_INSTANCE_TYPE_RESERVED_BITS: u32 = ${totalBits};
/// 掩码: 7 位全部。
pub const K_STRING_INSTANCE_TYPE_MASK: u16 = 0b0111_1111;

// ── 类族层级 (string.tq extern class 清单, ${classes.length} 个) ─────────
/// String 类族层级表 (名, 直接基类)。
pub const STRING_CLASS_HIERARCHY: [(&str, &str); ${classes.length}] = [
${classes.map((c) => `  ("${c.name}", "${c.base}"),`).join("\n")}
];

// ── 字段布局规则 (string.tq 各 class) ─────────────────────────────────
/// String 专属字段: length (int32)。
pub const STRING_FIELDS: [&str; 1] = ["length"];
/// ConsString 字段: first, second (second.length == 0 即扁平)。
pub const CONS_STRING_FIELDS: [&str; 2] = ["first", "second"];
/// ExternalString 字段: resource, resource_data (uncached 时缺后者)。
pub const EXTERNAL_STRING_FIELDS: [&str; 2] = ["resource", "resource_data"];
/// SlicedString 字段: parent, offset(Smi)。
pub const SLICED_STRING_FIELDS: [&str; 2] = ["parent", "offset"];
/// ThinString 字段: actual (InternalizedString)。
pub const THIN_STRING_FIELDS: [&str; 1] = ["actual"];
/// SeqOneByteString / SeqTwoByteString 变长负载字段。
pub const SEQ_STRING_CHARS_FIELD: &str = "chars";

// ── 长度上限制度 (string.h:552 / v8-primitive.h:129-130) ──────────────
/// 字符串长度上限 = v8::String::kMaxLength (64 位非 LOWER_LIMITS: (1<<29)-24)。
/// StaticAssertStringLengthFitsSmi (string.tq:360) 静态断言其可入 Smi。
pub const K_MAX_LENGTH: u32 = ${K_MAX_LENGTH_64};

// —— 提取不变式断言 ——
const _: () = {
  assert!(StringRepresentationTag::SeqStringTag as u32 == 0);
  assert!(StringRepresentationTag::ThinStringTag as u32 == 4);
  assert!(K_STRING_REPRESENTATION_BITS == 3);
  assert!(IS_ONE_BYTE_SHIFT == 3 && IS_UNCACHED_SHIFT == 4 && IS_SHARED_SHIFT == 5 && IS_NOT_INTERNALIZED_SHIFT == 6);
  assert!(K_STRING_INSTANCE_TYPE_RESERVED_BITS == 7);
  assert!(K_STRING_INSTANCE_TYPE_MASK == 0x7F);
  assert!(STRING_CLASS_HIERARCHY.len() == ${classes.length});
  assert!(STRING_CLASS_HIERARCHY[0] == ("String", "Name"));
  assert!(K_MAX_LENGTH == (1 << 29) - 24);
  assert!(CONS_STRING_FIELDS.len() == 2 && SLICED_STRING_FIELDS[1] == "offset");
};
`;

writeSrc("v8-objects-collections", "string.rs", body);
console.log(
  `string.rs: 表示标签 ${tags.length} + 位段 ${fields.length} 项 (总 ${totalBits} 位) + 类族 ${classes.length} + 布局表 6, 对数通过`,
);
