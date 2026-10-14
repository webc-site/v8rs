#!/usr/bin/env bun
// tool/src/objects/property-descriptor-object.h.js
//   — 覆盖 v8/src/objects/property-descriptor-object.{h,-inl.h,tq}
// → v8-objects-rest/src/property_descriptor_object.rs
// 机械提取: BitField 位链 (9 位) → 显式掩码 + Flag 枚举 + 组合常量 + tq 布局字段

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/objects/property-descriptor-object.h");
const tqSrc = readV8("src/objects/property-descriptor-object.tq");

// 位链声明顺序 = 位序 (原版 using ... = base::BitField<bool, 0, 1, uint32_t>; Next 链)
const CHAIN = [
  "IsEnumerableBit", "HasEnumerableBit", "IsConfigurableBit", "HasConfigurableBit",
  "IsWritableBit", "HasWritableBit", "HasValueBit", "HasGetBit", "HasSetBit",
];
let pos = 0;
const bits = CHAIN.map((name) => {
  // 每段都声明为 1 位 (bool)
  const decl = new RegExp(`using ${name} = base::BitField<bool, 0, 1, uint32_t>|using ${name} = \\w+Bit::Next<bool, 1>`);
  if (!decl.test(hSrc)) throw new Error(`未找到位链声明 ${name}`);
  const b = { name, start: pos, len: 1 };
  pos += 1;
  return b;
});
if (bits.length !== 9) throw new Error(`位链段数 ${bits.length} != 9`);
const maskOf = (b) => (1 << b.start) >>> 0;

// Flag 枚举条目 (kNone + 9 位掩码)
const flagRe = hSrc.match(/enum Flag : uint32_t\s*\{([\s\S]*?)\n\s*\};/);
if (!flagRe) throw new Error("未找到 Flag 枚举");
const flagNames = [...flagRe[1].matchAll(/^\s*(k\w+)\s*=/gm)].map((m) => m[1]);
if (flagNames.length !== 10) throw new Error(`Flag 条数 ${flagNames.length} != 10`);
if (flagNames[0] !== "kNone") throw new Error("Flag 首条应为 kNone");

// kFlagCount 对数
const flagCount = hSrc.match(/static constexpr int kFlagCount = (\d+);/);
if (!flagCount || flagCount[1] !== "9") throw new Error("kFlagCount != 9");

// 组合常量 (原版显式表达式, 机械求值)
const kRegularAccessor = maskOf(bits[0]) | maskOf(bits[3]) | maskOf(bits[7]) | maskOf(bits[8]); // HasEnumerable|HasConfigurable|HasGet|HasSet
const kRegularData = maskOf(bits[0]) | maskOf(bits[3]) | maskOf(bits[5]) | maskOf(bits[6]); // HasEnumerable|HasConfigurable|HasWritable|HasValue
const kHasMask = maskOf(bits[0]) | maskOf(bits[3]) | maskOf(bits[5]) | maskOf(bits[6]) | maskOf(bits[7]) | maskOf(bits[8]);
// 原版 kRegularAccessorPropertyBits 注释对数: 含 HasEnumerableBit/HasConfigurableBit/HasGetBit/HasSetBit
if (!hSrc.match(/kRegularAccessorPropertyBits =\s*\n?\s*HasEnumerableBit::kMask \| HasConfigurableBit::kMask \| HasGetBit::kMask \|[\s\S]{0,40}HasSetBit::kMask/)) {
  throw new Error("kRegularAccessorPropertyBits 表达式失配");
}

// tq: bitfield struct PropertyDescriptorObjectFlags (9 snake 位) + 布局字段 (4)
const bfRe = tqSrc.match(/bitfield struct PropertyDescriptorObjectFlags extends uint31\s*\{([\s\S]*?)\n\}/);
if (!bfRe) throw new Error("未找到 tq bitfield struct");
const tqBits = [...bfRe[1].matchAll(/^\s{2}(\w+):\s*bool:\s*1 bit;/gm)].map((m) => m[1]);
if (tqBits.length !== 9) throw new Error(`tq 位字段数 ${tqBits.length} != 9`);
// tq 位序与 C++ 位链 1:1 对数 (IsEnumerable→is_enumerable ...)
const cppSnake = bits.map((b) => b.name.replace(/Bit$/, "").replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase());
for (let i = 0; i < 9; i++) {
  if (cppSnake[i] !== tqBits[i]) throw new Error(`tq/C++ 位序失配 @${i}: ${cppSnake[i]} vs ${tqBits[i]}`);
}

const classRe = tqSrc.match(/extern class PropertyDescriptorObject extends Struct\s*\{([\s\S]*?)\n\}/);
if (!classRe) throw new Error("未找到 tq extern class");
const tqFields = [...classRe[1].matchAll(/^\s{2}(\w+):\s*([^;{}]+);/gm)].map((m) => ({ name: m[1], type: m[2].trim() }));
if (tqFields.length !== 4) throw new Error(`tq 字段数 ${tqFields.length} != 4 (flags/value/get/set)`);

const pascal = (s) => s.replace(/^k/, "").toLowerCase().replace(/(^|_)([a-z0-9])/g, (_, p, c) => c.toUpperCase());

let rust = banner("src/objects/property-descriptor-object.h.js", "src/objects/property-descriptor-object.{h,-inl.h,tq}", "PropertyDescriptorObject 标志位链展开与布局");
rust += `//! PropertyDescriptorObject: 属性描述符堆对象; 9 位标志链 + 3 载荷字段。

`;
for (const b of bits) {
  rust += `/// 原版 ${b.name} (bit ${b.start})。
pub const ${snakeUpper(b.name)}_MASK: u32 = 0x${maskOf(b).toString(16)};
`;
}
rust += `
/// 标志位总数 (原版 kFlagCount)。
pub const FLAG_COUNT: u32 = 9;

/// 原版 enum Flag (各 = 对应位掩码; kNone = 0)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum Flag {
  /// kNone
  None = 0,
`;
for (let i = 1; i < flagNames.length; i++) {
  const name = flagNames[i];
  // kIsEnumerable → IsEnumerableBit (原版 Flag 条目名 = 位链名去 Bit 加 k)
  const bitName = name.slice(1) + "Bit";
  const bit = bits.find((b) => b.name === bitName);
  if (!bit) throw new Error(`Flag ${name} 无对应位段 ${bitName}`);
  rust += `  /// ${name}\n  ${pascal(name)} = 0x${maskOf(bit).toString(16)},\n`;
}
rust += `}

/// 常规访问器属性位组合 (原版 kRegularAccessorPropertyBits)。
pub const REGULAR_ACCESSOR_PROPERTY_BITS: u32 = 0x${kRegularAccessor.toString(16)};
/// 常规数据属性位组合 (原版 kRegularDataPropertyBits)。
pub const REGULAR_DATA_PROPERTY_BITS: u32 = 0x${kRegularData.toString(16)};
/// 全部 Has 位掩码 (原版 kHasMask)。
pub const HAS_MASK: u32 = 0x${kHasMask.toString(16)};

/// Torque 布局字段表 (property-descriptor-object.tq, 顺序 1:1)。
pub const LAYOUT_FIELDS: [(&str, &str); ${tqFields.length}] = [
`;
for (const f of tqFields) rust += `  ("${f.name}", "${f.type}"),\n`;
rust += `];

/// Torque 侧位序 (bitfield struct PropertyDescriptorObjectFlags, 与 C++ 位链 1:1)。
pub const TQ_FLAG_ORDER: [&str; ${tqBits.length}] = [
`;
for (const b of tqBits) rust += `  "${b}",\n`;
rust += `];

/// 位段解码 (原版 BitField::decode)。
#[inline]
#[must_use]
pub const fn decode(flags: u32, mask: u32) -> bool {
  (flags & mask) != 0
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(FLAG_COUNT == 9);
  assert!(IS_ENUMERABLE_BIT_MASK == 0x1);
  assert!(HAS_SET_BIT_MASK == 0x100);
  assert!(REGULAR_ACCESSOR_PROPERTY_BITS == 0x${kRegularAccessor.toString(16)});
  assert!(REGULAR_DATA_PROPERTY_BITS == 0x${kRegularData.toString(16)});
  assert!(HAS_MASK == 0x${kHasMask.toString(16)});
  assert!(LAYOUT_FIELDS.len() == 4);
  assert!(TQ_FLAG_ORDER.len() == 9);
  assert!(TQ_FLAG_ORDER[0] == "is_enumerable" && TQ_FLAG_ORDER[8] == "has_set");
};
`;

function snakeUpper(s) {
  return s.replace(/Bit$/, "").replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase() + "_BIT";
}

const p = writeSrc("v8-objects-rest", "property_descriptor_object.rs", rust);
console.log(`property_descriptor_object.rs: 9 位链掩码 + Flag 10 变体 + 3 组合常量 + 4 布局字段, 对数通过`);
