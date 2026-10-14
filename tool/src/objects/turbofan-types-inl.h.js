#!/usr/bin/env bun
// tool/src/objects/turbofan-types-inl.h.js — TurboFan 类型内联访问器生成
// 组→文件映射 (本脚本管辖):
//   v8/src/objects/turbofan-types-inl.h + turbofan-types.tq → v8-objects-rest/src/turbofan_types_inl.rs
//   (turbofan-types.h 的位链声明面归 tool/src/objects/turbofan-types.h.js → turbofan_types.rs;
//    本脚本落 -inl 访问器 + tq 的 32+32 位集字段与 5 个布局类)
import { readV8, writeSrc, banner } from "../../lib.js";

const inl = readV8("src/objects/turbofan-types-inl.h");
const tq = readV8("src/objects/turbofan-types.tq");
const h = readV8("src/objects/turbofan-types.h");

// —— -inl 访问器清单 (类分组) ——
const defs = [...inl.matchAll(/^[\w:<>, ]+?\s(\w+)::(\w+)\(/gm)].map((m) => `${m[1]}::${m[2]}`);
const EXPECT = ["TurbofanBitsetType::bitset_low", "TurbofanBitsetType::set_bitset_low", "TurbofanBitsetType::bitset_high",
  "TurbofanBitsetType::set_bitset_high", "TurbofanUnionType::type1", "TurbofanUnionType::set_type1",
  "TurbofanUnionType::type2", "TurbofanUnionType::set_type2", "TurbofanRangeType::min", "TurbofanRangeType::set_min",
  "TurbofanRangeType::max", "TurbofanRangeType::set_max", "TurbofanHeapConstantType::constant",
  "TurbofanHeapConstantType::set_constant", "TurbofanOtherNumberConstantType::constant",
  "TurbofanOtherNumberConstantType::set_constant"];
if (defs.join() !== EXPECT.join())
  throw new Error(`turbofan-types-inl.h 访问器序漂移 (应 ${EXPECT.length}):\n  ${defs}`);

// —— tq 位集字段 (32 + 4 = 36 位) ——
function bitfield(structName) {
  const m = tq.match(new RegExp(`bitfield struct ${structName} extends uint32 \\{([\\s\\S]*?)\\}`));
  if (!m) throw new Error(`tq 缺 bitfield struct ${structName}`);
  let off = 0;
  return [...m[1].matchAll(/(\w+): (\w+): (\d+) bit;/g)].map((f) => {
    const r = { name: f[1], type: f[2], width: Number(f[3]), shift: off };
    off += r.width;
    return r;
  });
}
const lowBits = bitfield("TurbofanTypeLowBits");
const highBits = bitfield("TurbofanTypeHighBits");
if (lowBits.length !== 32) throw new Error(`TurbofanTypeLowBits 应 32 位段, 实得 ${lowBits.length}`);
if (highBits.length !== 4) throw new Error(`TurbofanTypeHighBits 应 4 位段, 实得 ${highBits.length}`);
if (lowBits.reduce((a, b) => a + b.width, 0) !== 32 || highBits.reduce((a, b) => a + b.width, 0) !== 4)
  throw new Error("位宽合计应 32/4");
// 段锚抽查
if (lowBits[0].name !== "_unused_padding_field_1" || lowBits[0].shift !== 0) throw new Error("低 32 位首段应为 _unused_padding_field_1@0");
if (lowBits.find((b) => b.name === "minus_zero")?.shift !== 11) throw new Error("minus_zero 应 shift 11");
if (lowBits.find((b) => b.name === "naN")?.shift !== 12) throw new Error("naN 应 shift 12");
if (lowBits.find((b) => b.name === "other_object")?.shift !== 16) throw new Error("other_object 应 shift 16");
if (lowBits.find((b) => b.name === "wasm_object")?.shift !== 30) throw new Error("wasm_object 应 shift 30");
if (highBits.find((b) => b.name === "hole")?.shift !== 1) throw new Error("hole 应 shift 1");

// —— tq 布局类 (TurbofanType 抽象基 + 5 个具体类) ——
const tqClasses = [...tq.matchAll(/^extern class (\w+) extends (\w+) \{([^{}]*)\}/gm)].map((m) => ({
  name: m[1], base: m[2], fields: [...m[3].matchAll(/^\s*(\w+): ([\w<>]+);/gm)].map((f) => `${f[1]}:${f[2]}`),
})).filter((c) => c.fields.length > 0 || c.name !== "TurbofanType");
if (tqClasses.length !== 5) throw new Error(`tq 具体布局类应 5 (抽象壳 TurbofanType 除外), 实得 ${tqClasses.length}`);
if (!/extern class TurbofanType extends HeapObject \{\}/.test(tq)) throw new Error("tq 抽象基类 TurbofanType 缺失");
if (tqClasses[0].name !== "TurbofanBitsetType" || tqClasses[0].fields.join() !== "bitset_low:TurbofanTypeLowBits,bitset_high:TurbofanTypeHighBits")
  throw new Error(`TurbofanBitsetType 字段漂移: ${tqClasses[0].fields}`);
if (tqClasses[1].name !== "TurbofanUnionType" || tqClasses[1].fields.join() !== "type1:TurbofanType,type2:TurbofanType")
  throw new Error(`TurbofanUnionType 字段漂移: ${tqClasses[1].fields}`);
if (tqClasses[4].name !== "TurbofanOtherNumberConstantType" || tqClasses[4].fields.join() !== "constant:float64")
  throw new Error(`TurbofanOtherNumberConstantType 字段漂移: ${tqClasses[4].fields}`);

// —— 常量 (tq constexpr) ——
if (!/const kMaxIntPtr: constexpr IntegerLiteral\s*\n\s*generates 'IntegerLiteral\(ca_\.Is64\(\) \? 0x7FFFFFFFFFFFFFFF : 0x7FFFFFFF\)';/.test(tq))
  throw new Error("kMaxIntPtr 生成式缺失");
if (!/0x8000000000000000 : 0x80000000/.test(tq)) throw new Error("kMinIntPtr 生成式缺失");

const body = `${banner("src/objects/turbofan-types-inl.h.js", "src/objects/turbofan-types-inl.h + turbofan-types.tq", "位集 32+4 段; 声明链在 turbofan_types.rs")}
//! TurboFan 类型内联面 (原版 turbofan-types-inl.h + turbofan-types.tq)
//!
//! 位集表示: 64 位拆两个 uint32 (Torque 64 位位集支持受限):
//! 低 32 位承载标量种类, 高 4 位承载 machine/hole 等新种类.

// ===== 位集段表 (tq bitfield struct 机械展开) =====
/// TurbofanTypeLowBits 段表 (32 段铺满 uint32)
pub const TURBOFAN_TYPE_LOW_BITS: [(&str, u32, u32); ${lowBits.length}] = [
  // (段名, shift, width)
${lowBits.map((b) => `  ("${b.name}", ${b.shift}, ${b.width}),`).join("\n")}
];

/// TurbofanTypeHighBits 段表 (4 段)
pub const TURBOFAN_TYPE_HIGH_BITS: [(&str, u32, u32); ${highBits.length}] = [
${highBits.map((b) => `  ("${b.name}", ${b.shift}, ${b.width}),`).join("\n")}
];

/// intptr 边界常量 (tq constexpr, 64 位目标)
pub const K_MAX_INT_PTR: u64 = 0x7FFF_FFFF_FFFF_FFFF;
/// intptr 下界 (64 位目标)
pub const K_MIN_INT_PTR: u64 = 0x8000_0000_0000_0000;

// ===== 布局类 (tq, TurbofanType 抽象基 + ${tqClasses.length} 具体类, 头 8 = HeapObject) =====
/// 布局类字段表 (声明序; 头 = HeapObject 8 字节, 非压缩)
pub const TURBOFAN_TYPE_CLASSES: [(&str, &str, &str); ${tqClasses.length}] = [
${tqClasses.map((c) => `  ("${c.name}", "${c.base}", "${c.fields.join(", ")}"),`).join("\n")}
];

/// 各类头大小/尺寸 (非压缩: kTaggedSize=8, double 经 UnalignedDoubleMember 8 字节)
pub const K_HEADER_SIZE: usize = 8;
/// TurbofanBitsetType 尺寸 = 头 + 4 + 4 = 16
pub const K_BITSET_TYPE_SIZE: usize = 16;
/// TurbofanUnionType 尺寸 = 头 + 8 + 8 = 24
pub const K_UNION_TYPE_SIZE: usize = 24;
/// TurbofanRangeType 尺寸 = 头 + 8 + 8 = 24 (min/max 非对齐双精度)
pub const K_RANGE_TYPE_SIZE: usize = 24;
/// TurbofanHeapConstantType 尺寸 = 头 + 8 = 16
pub const K_HEAP_CONSTANT_TYPE_SIZE: usize = 16;
/// TurbofanOtherNumberConstantType 尺寸 = 头 + 8 = 16
pub const K_OTHER_NUMBER_CONSTANT_TYPE_SIZE: usize = 16;

// ===== 访问器 (${defs.length} 个, 全部平凡读写) =====
/// 位集低 32 位读/写 (原版 bitset_low/set_bitset_low 直访 u32 字段)
pub fn bitset_low(/* this */) -> u32 {
  todo!("就地修订: 平凡 u32 字段读")
}

/// 位集高 32 位读/写 (同上)
pub fn bitset_high(/* this */) -> u32 {
  todo!("就地修订: 平凡 u32 字段读")
}

/// 联合支路 1/2 读 (原版 type1/type2: tagged 成员槽 load, 含写屏障形参的 set 版)
pub fn union_type_accessors() {
  todo!("就地修订: TaggedMember<TurbofanType>::load/store (set 带WriteBarrierMode)")
}

/// 区间 min/max (原版 UnalignedDoubleMember value/set_value)
pub fn range_min_max(/* this */) -> (f64, f64) {
  todo!("就地修订: 非对齐 f64 读写")
}

/// 堆常量 (原版 constant: Cast<HeapObject>(constant_.load()))
pub fn heap_constant(/* this */) /* -> Tagged<HeapObject> */ {
  todo!("就地修订: tagged 槽读后 Cast")
}

/// 其他数值常量 (非对齐 f64)
pub fn other_number_constant(/* this */) -> f64 {
  todo!("就地修订: 非对齐 f64 读写")
}
`;

const p = writeSrc("v8-objects-rest", "turbofan_types_inl.rs", body);
console.log(`turbofan-types-inl.h.js: 已生成 ${p} (访问器 ${defs.length}, 低位段 ${lowBits.length}, 高位段 ${highBits.length}, 布局类 ${tqClasses.length})`);
