#!/usr/bin/env bun
// tool/src/objects/turboshaft-types.h.js — Turboshaft 类型族生成
// 组→文件映射 (本脚本管辖):
//   v8/src/objects/turboshaft-types.{h,-inl.h,tq} → v8-objects-rest/src/turboshaft_types.rs
// 规则面: 9 类布局 (3 抽象 + 6 具体, 柔性尾 3 种) + FloatSpecialValues 位域 +
//   SizeFor 公式族; 断言类测试宏 (tq builtin) 语义留骨架
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/turboshaft-types.h");
const inl = readV8("src/objects/turboshaft-types-inl.h");
const tq = readV8("src/objects/turboshaft-types.tq");

let checks = 0;
const need = (cond, msg) => { checks++; if (!cond) throw new Error(msg); };

// —— C++ 类清单 (声明序) ——
const classes = [...h.matchAll(/^V8_OBJECT class (\w+) : public (\w+)(?: \{)?/gm)].map((m) => ({ name: m[1], base: m[2] }));
const CLASSES = ["TurboshaftType", "TurboshaftWord32Type", "TurboshaftWord32RangeType", "TurboshaftWord32SetType",
  "TurboshaftWord64Type", "TurboshaftWord64RangeType", "TurboshaftWord64SetType", "TurboshaftFloat64Type",
  "TurboshaftFloat64RangeType", "TurboshaftFloat64SetType"];
if (classes.length !== CLASSES.length || classes.some((c, i) => c.name !== CLASSES[i]))
  throw new Error(`类清单漂移 (应 ${CLASSES.length}):\n  ${classes.map((c) => c.name)}`);
const abstractCount = classes.filter((c) => /V8_IT_ABSTRACT/.test(h.slice(h.indexOf(`class ${c.name}`), h.indexOf(`class ${c.name}`) + 400))).length;

// —— FloatSpecialValues 位域 (h 侧 BitField 链) ——
need(/using NanBit = base::BitField<bool, 0, 1, uint32_t>;/.test(h), "NanBit 定义缺失");
need(/using MinusZeroBit = NanBit::Next<bool, 1>;/.test(h), "MinusZeroBit 链缺失");
need(/using UnusedBits = MinusZeroBit::Next<uint32_t, 30>;/.test(h), "UnusedBits 链缺失");
// tq 双源核对
const tqBit = tq.match(/bitfield struct TurboshaftFloatSpecialValues extends uint32 \{([\s\S]*?)\}/);
need(!!tqBit, "tq TurboshaftFloatSpecialValues 缺失");
const tqSegs = [...tqBit[1].matchAll(/(\w+): (\w+): (\d+) bit;/g)].map((m) => ({ name: m[1], type: m[2], width: Number(m[3]) }));
need(tqSegs.map((s) => s.name).join() === "nan,minus_zero,_unused", `tq 位域段序漂移: ${tqSegs.map((s) => s.name)}`);
need(tqSegs.reduce((a, b) => a + b.width, 0) === 32, "tq 位域宽合计应 32");

// —— SizeFor 公式族 (h 内联定义) ——
need(/SizeFor\(int set_size\) \{\s*\n\s*return OBJECT_POINTER_ALIGN\(OFFSET_OF_DATA_START\(TurboshaftWord32SetType\) \+\s*\n\s*set_size \* sizeof\(uint32_t\)\);/.test(h), "Word32Set SizeFor 公式缺失");
need(/2 \* set_size \* sizeof\(uint32_t\)/.test(h), "Word64Set 双数组系数缺失");
need(/OFFSET_OF_DATA_START\(TurboshaftFloat64SetType\) \+\s*\n\s*set_size \* sizeof\(double\)/.test(h), "Float64Set SizeFor 无对齐 (原版不 ALIGN) 公式缺失");
need(/static constexpr int SizeFor\(\) \{ return sizeof\(TurboshaftWord32RangeType\); \}/.test(h), "Range SizeFor=sizeof 缺失");

// —— 柔性尾三态 ——
need(h.includes("V8_TQ_TAIL_NAME(elements);\n  V8_TQ_TAIL_LENGTH(set_size);\n  FLEXIBLE_ARRAY_MEMBER(uint32_t, elements);"), "Word32Set 柔性尾缺失");
need(h.includes("V8_TQ_TAIL_SECTIONS(elements_high[set_size] : uint32;\n                      elements_low[set_size] : uint32;)"), "Word64Set 双段柔性尾缺失");
need(h.includes("FLEXIBLE_ARRAY_MEMBER(UnalignedDoubleMember, elements);"), "Float64Set 非对齐柔性尾缺失");
need(h.includes("uint32_t _padding_;"), "Float64Range 显式 _padding_ 缺失");

// —— 布局求值 (非压缩: 头 8; clang 探针实算核对) ——
// Word32Range: 8 + 4 + 4 = 16
// Word32Set: 头 8 + set_size 4 → data start 12 → 对齐后 = ALIGN8(12 + 4n)
// Word64Range: 8 + 4×4 = 24
// Word64Set: data start 12 → ALIGN8(12 + 8n)
// Float64Type: 8 + 4 = 12
// Float64Range: 12 + padding 4 + 8 + 8 = 32
// Float64Set: 8 + special 4 + set_size 4 → data start 16 → 16 + 8n (无对齐!)
const LAYOUT = {
  Word32Range: { header: 16, kind: "定尺寸" },
  Word32Set: { header: 12, kind: "柔性尾 ALIGN8(12+4n)" },
  Word64Range: { header: 24, kind: "定尺寸" },
  Word64Set: { header: 12, kind: "柔性尾 ALIGN8(12+8n)" },
  Float64Type: { header: 12, kind: "定尺寸 (抽象)" },
  Float64Range: { header: 32, kind: "定尺寸 (显式 padding)" },
  Float64Set: { header: 16, kind: "柔性尾 16+8n 无对齐" },
};

// —— tq 双源核对 (类与字段) ——
const tqClasses = [...tq.matchAll(/^extern class (\w+) extends (\w+) \{\n([\s\S]*?)\n\}/gm)];
need(tqClasses.length === 7, `tq 带体类应 7 (抽象壳 {} 单行不进正则), 实得 ${tqClasses.length}`);
const tqByName = Object.fromEntries(tqClasses.map((c) => [c[1], c[3]]));
need(/from: uint32;\s*\n\s*to: uint32;/.test(tqByName["TurboshaftWord32RangeType"] ?? ""), "tq Word32Range 字段漂移");
need(/from_high: uint32;/.test(tqByName["TurboshaftWord64RangeType"] ?? ""), "tq Word64Range 字段漂移");
need(/elements\[set_size\]: float64;/.test(tqByName["TurboshaftFloat64SetType"] ?? ""), "tq Float64Set 字段漂移");

// —— inl 访问器面 ——
const inlDefs = (inl.match(/Turboshaft\w+::\w+\(/g) || []).length;
need(inlDefs === 32, `turboshaft-types-inl.h 定义应 32, 实得 ${inlDefs}`);
// Word64Set 低位寻址语义锚 (elements[set_size + i])
need(/elements\(\)\[set_size\(\) \+ i\]/.test(inl), "Word64Set 低位半区寻址缺失");

const body = `${banner("src/objects/turboshaft-types.h.js", "src/objects/turboshaft-types.{h,-inl.h,tq}", "非压缩布局; 柔性尾三态")}
//! Turboshaft 类型族 (原版 turboshaft-types.{h,-inl.h,tq})
//!
//! Word32/Word64/Float64 三支, 每支 Range (定尺寸) + Set (柔性尾) 两型;
//! Word64Set 尾部拆高低双段, Float64Set 尾部非对齐 double 且 SizeFor 不补对齐.

/// 特殊浮点位域 (TurboshaftFloatSpecialValues: nan@0, minus_zero@1, unused@2..31)
pub const FLOAT_SPECIAL_VALUES: [(&str, u32, u32); 3] = [
  ("nan", 0, 1),
  ("minus_zero", 1, 1),
  ("unused", 2, 30),
];

// ===== 类层级 (h 声明序 ${classes.length} 类, 其中抽象 ${abstractCount}) =====
/// 类清单 (名, 基类)
pub const TURBOSHAFT_TYPE_CLASSES: [(&str, &str); ${classes.length}] = [
${classes.map((c) => `  ("${c.name}", "${c.base}"),`).join("\n")}
];

// ===== 布局常量 (非压缩 kTaggedSize=8, 头 = HeapObject map 槽) =====
/// Word32Range: 头 8 + from 4 + to 4
pub const K_WORD32_RANGE_SIZE: usize = ${LAYOUT.Word32Range.header};
/// Word32Set 数据起点 (头 8 + set_size 4); 尺寸 = ALIGN8(12 + 4·n)
pub const K_WORD32_SET_DATA_START: usize = ${LAYOUT.Word32Set.header};
/// Word64Range: 头 8 + 4×u32
pub const K_WORD64_RANGE_SIZE: usize = ${LAYOUT.Word64Range.header};
/// Word64Set 数据起点; 尺寸 = ALIGN8(12 + 8·n) — 低位段在 elements[set_size + i]
pub const K_WORD64_SET_DATA_START: usize = ${LAYOUT.Word64Set.header};
/// Float64Type: 头 8 + special_values 4
pub const K_FLOAT64_TYPE_SIZE: usize = ${LAYOUT.Float64Type.header};
/// Float64Range: 12 + 显式 padding 4 + min 8 + max 8
pub const K_FLOAT64_RANGE_SIZE: usize = ${LAYOUT.Float64Range.header};
/// Float64Set 数据起点 (头 8 + special_values 4 + set_size 4); 尺寸 = 16 + 8·n
pub const K_FLOAT64_SET_DATA_START: usize = ${LAYOUT.Float64Set.header};

/// Word32Set 尺寸 (原版 SizeFor: OBJECT_POINTER_ALIGN(12 + set_size*4))
pub const fn word32_set_size_for(set_size: usize) -> usize {
  (K_WORD32_SET_DATA_START + set_size * 4 + 7) & !7
}

/// Word64Set 尺寸 (原版 SizeFor: OBJECT_POINTER_ALIGN(12 + 2*set_size*4))
pub const fn word64_set_size_for(set_size: usize) -> usize {
  (K_WORD64_SET_DATA_START + 2 * set_size * 4 + 7) & !7
}

/// Float64Set 尺寸 (原版 SizeFor: 16 + set_size*8, 无对齐)
pub const fn float64_set_size_for(set_size: usize) -> usize {
  K_FLOAT64_SET_DATA_START + set_size * 8
}

// ===== 访问器 (inl 定义 ${inlDefs} 个, 全部平凡读写 + 域检查) =====
/// Word32 区间界 (原版 from/to: 平凡 u32)
pub fn word32_range_bounds(/* this */) -> (u32, u32) {
  todo!("就地修订: 直读 from_/to_")
}

/// Word32 集元素 (原版 elements(i): DCHECK 域检查后柔性尾索引)
pub fn word32_set_element(/* this, i */) -> u32 {
  todo!("就地修订: DCHECK_LT(i, set_size) 后 elements()[i]")
}

/// Word64 集元素 (高/低半区: 低半区寻址 elements[set_size + i])
pub fn word64_set_element(/* this, i */) -> (u32, u32) {
  todo!("就地修订: (elements()[i], elements()[set_size + i])")
}

/// 特殊浮点标志 (nan / minus_zero 位)
pub fn float64_special_values(/* this */) -> u32 {
  todo!("就地修订: 直读 special_values_")
}

/// Float64 区间界 (非对齐 double 成员)
pub fn float64_range_bounds(/* this */) -> (f64, f64) {
  todo!("就地修订: UnalignedDoubleMember value()")
}

/// Float64 集元素 (非对齐 double 柔性尾)
pub fn float64_set_element(/* this, i */) -> f64 {
  todo!("就地修订: elements()[i].value()")
}
`;

const p = writeSrc("v8-objects-rest", "turboshaft_types.rs", body);
console.log(`turboshaft-types.h.js: 已生成 ${p} (must ${checks}: 类 ${classes.length} (抽象 ${abstractCount}), tq 类 ${tqClasses.length}, inl 定义 ${inlDefs})`);
