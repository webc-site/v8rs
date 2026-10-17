#!/usr/bin/env bun
// tool/src/objects/literal-objects.h.js — 覆盖 v8/src/objects/literal-objects.{h,-inl.h,cc,tq} (全新族)
// → v8-objects-js-func/src/literal_objects.rs
// 机械提取: 5 个字面量描述类的 tq 布局 + ClassBoilerplate 枚举/位段/常量 +
//   ObjectBoilerplateDescription 索引公式 + 方法清单。
// 配置登记: 64 位非压缩 (kTaggedSize=8, Struct 头 = map(8))。

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/literal-objects.h");
const inl = readV8("src/objects/literal-objects-inl.h");
const cc = readV8("src/objects/literal-objects.cc");
const tq = readV8("src/objects/literal-objects.tq");

let checks = 0;
const must = (c, m) => {
  checks++;
  if (!c) throw new Error(`锚点失配: ${m}`);
};

// ── 1. tq 布局 (5 类) ───────────────────────────────────────────────────
const tqClasses = [...tq.matchAll(/^extern class (\w+) extends (\w+) \{([^}]*)\n\}/gm)]
  .map((m) => ({ name: m[1], ext: m[2], body: m[3] }));
const TQ_EXPECT = [
  ["ObjectBoilerplateDescription", "FixedArrayBase", ["backing_store_size", "flags", "raw_entries[length]"]],
  ["ArrayBoilerplateDescription", "Struct", ["flags", "constant_elements"]],
  ["RegExpBoilerplateDescription", "Struct", ["data", "flags"]],
  ["ClassBoilerplate", "Struct", ["arguments_count", "static_properties_template", "static_elements_template",
    "static_computed_properties", "instance_properties_template", "instance_elements_template", "instance_computed_properties"]],
  ["PrototypeSharedClosureInfo", "Struct", ["boilerplate_description", "closure_feedback_cell_array", "context"]],
];
must(tqClasses.length === 5, `tq 类应 5, 实得 ${tqClasses.length}`);
for (let i = 0; i < 5; i++) {
  const c = tqClasses[i];
  const fields = [...c.body.matchAll(/^\s*(\w+)(\[\w+\])?:\s*([^;]+);/gm)]
    .map((f) => `${f[1]}${f[2] ?? ""}`);
  must(c.name === TQ_EXPECT[i][0], `tq 类[${i}] 应 ${TQ_EXPECT[i][0]}, 实得 ${c.name}`);
  must(c.ext === TQ_EXPECT[i][1], `${c.name} 应 extends ${TQ_EXPECT[i][1]}, 实得 ${c.ext}`);
  must(JSON.stringify(fields) === JSON.stringify(TQ_EXPECT[i][2]),
    `${c.name} 字段失配: ${JSON.stringify(fields)}`);
}
must(tq.includes("SmiTagged<JSRegExpFlags>"), "RegExpBoilerplateDescription flags 类型锚缺失");
must(tq.includes("TrustedPointer<RegExpData>"), "data TrustedPointer 锚缺失");

// ── 2. h 侧 ClassBoilerplate 规则面 ────────────────────────────────────
// ValueKind 枚举
const vk = h.match(/enum ValueKind \{([^}]+)\}/);
if (!vk) throw new Error("未找到 ClassBoilerplate::ValueKind");
const valueKinds = vk[1].split(",").map((s) => s.trim()).filter(Boolean);
must(JSON.stringify(valueKinds) === JSON.stringify(["kData", "kGetter", "kSetter", "kAutoAccessor"]),
  `ValueKind 应 4 项, 实得 ${JSON.stringify(valueKinds)}`);
// ComputedEntryFlags 位段 (X-macro: V(Name, Type, Bits, _))
const bf = h.match(/#define COMPUTED_ENTRY_BIT_FIELDS\(V, _\)([\s\S]*?)DEFINE_BIT_FIELDS/);
if (!bf) throw new Error("未找到 COMPUTED_ENTRY_BIT_FIELDS");
const bitFields = [...bf[1].matchAll(/V\((\w+),\s*(\w+),\s*(\d+),\s*_\)/g)].map((m) => [m[1], m[2], +m[3]]);
must(JSON.stringify(bitFields) === JSON.stringify([["ValueKindBits", "ValueKind", 2], ["KeyIndexBits", "unsigned", 29]]),
  `位段失配: ${JSON.stringify(bitFields)}`);
must(bitFields[0][2] + bitFields[1][2] === 31, "位段总宽应 31 (Smi 正区间)");
// DefineClassArgumentsIndices
for (const [name, val] of [["kConstructorArgumentIndex", 1], ["kPrototypeArgumentIndex", 2], ["kFirstDynamicArgumentIndex", 3]])
  must(h.includes(`${name} = ${val},`), `${name}=${val} 锚缺失`);
// 最小属性数
must(h.includes("static const int kMinimumClassPropertiesCount = 6;"), "kMinimumClassPropertiesCount=6 锚缺失");
must(h.includes("static const int kMinimumPrototypePropertiesCount = 1;"), "kMinimumPrototypePropertiesCount=1 锚缺失");
// ObjectBoilerplateDescription 索引公式与头
must(h.includes("static constexpr int kElementsPerEntry = 2;"), "kElementsPerEntry=2 锚缺失");
must(h.includes("static constexpr int NameIndex(int i) { return i * kElementsPerEntry; }"), "NameIndex 公式锚缺失");
must(h.includes("static constexpr int ValueIndex(int i) { return i * kElementsPerEntry + 1; }"), "ValueIndex 公式锚缺失");
must(h.includes("static constexpr uint32_t kLengthOffset = sizeof(HeapObject);"), "kLengthOffset 锚缺失");
must(h.includes("kLengthOffset + 3 * (TAGGED_SIZE_8_BYTES ? kTaggedSize : kApiInt32Size);"), "kHeaderSize 公式锚缺失");
must(h.includes("using KeyT = UnionOf<InternalizedString, Number>;"), "KeyT 锚缺失");

// ── 3. -inl.h 方法清单 ─────────────────────────────────────────────────
const inlDefs = [...new Set([...inl.matchAll(
  /^(?:[A-Za-z_][\w:<>,*&]*[ \t]+)?(ObjectBoilerplateDescription|ArrayBoilerplateDescription|RegExpBoilerplateDescription|ClassBoilerplate|PrototypeSharedClosureInfo)::(\w+)\(/gm,
)].map((m) => `${m[1]}::${m[2]}`))];
must(inlDefs.length >= 30, `-inl 定义应 ≥30, 实得 ${inlDefs.length}`);
// 关键语义锚
must(inl.includes("const uint32_t capacity = boilerplate * kElementsPerEntry;"), "capacity 公式锚缺失");
must(inl.includes("CHECK_LE(capacity, kMaxCapacity);"), "capacity 上限锚缺失");
must(inl.includes("result->set_flags(0);"), "flags 清零锚缺失");
must(inl.includes("MemsetTagged((*result)->RawFieldOfFirstElement(),\n               ReadOnlyRoots{isolate}.undefined_value(), capacity);"),
  "undefined 填充锚缺失");
must(inl.includes("return static_cast<ElementsKind>(flags().value());"), "elements_kind 解码锚缺失");
must(inl.includes("return constant_elements()->ulength().value() == 0;"), "is_empty 语义锚缺失");
must(inl.includes("DCHECK_EQ(0, cap % kElementsPerEntry);"), "capacity 对齐 DCHECK 锚缺失");
must(inl.includes("data_.load(isolate);"), "TrustedPointer load 锚缺失");

// ── 4. cc 函数清单 ─────────────────────────────────────────────────────
const ccDefs = [...new Set([...cc.matchAll(
  /^(?:[A-Za-z_][\w:<>,*&]*[ \t]+)?(ObjectBoilerplateDescription|ArrayBoilerplateDescription|RegExpBoilerplateDescription|ClassBoilerplate|PrototypeSharedClosureInfo)::(\w+)\(/gm,
)].map((m) => `${m[1]}::${m[2]}`))];
for (const need of ["ClassBoilerplate::AddToPropertiesTemplate", "ClassBoilerplate::AddToElementsTemplate",
  "ClassBoilerplate::New", "ArrayBoilerplateDescription::BriefPrintDetails", "RegExpBoilerplateDescription::BriefPrintDetails"])
  must(ccDefs.includes(need), `cc 缺 ${need}`);
must(cc.includes("AddToDictionaryTemplate(isolate, dictionary, name, key_index, value_kind,"), "properties 模板转发锚缺失");
must(cc.includes("AddToDictionaryTemplate(isolate, dictionary, key, key_index, value_kind,"), "elements 模板转发锚缺失");
must(cc.includes("ClassBoilerplate::kConstructorArgumentIndex"), "cc 构造实参索引锚缺失");
must(cc.includes("int dynamic_argument_index = ClassBoilerplate::kFirstDynamicArgumentIndex;"), "cc 动态实参起点锚缺失");

// ── 5. 64 位布局数值 (kTaggedSize=8; FixedArrayBase 头 16; Struct 头 8) ──
const K_OBD_HEADER = 8 + 3 * 8; // kLengthOffset(sizeof(HeapObject)=8) + 3 * kTaggedSize = 32 (TAGGED_SIZE_8_BYTES)
const K_ABD_HEADER = 8 + 2 * 8; // Struct: map8 + flags Smi8 + constant_elements 8 → 24
const K_RBD_SIZE = 8 + 8 + 8; // map + TrustedPointerMember(8) + Smi(8) → 24
const K_CLASS_HEADER = 8 + 7 * 8; // map + 7 字段 → 64
const K_PSCI_HEADER = 8 + 3 * 8; // map + 3 字段 → 32
must(K_OBD_HEADER === 32 && K_ABD_HEADER === 24 && K_RBD_SIZE === 24 && K_CLASS_HEADER === 64 && K_PSCI_HEADER === 32,
  "布局推导失配");

// ── 6. 生成 ────────────────────────────────────────────────────────────
const body = `${banner("src/objects/literal-objects.h.js", "src/objects/literal-objects.{h,-inl.h,cc,tq}", "字面量描述类 5 件套: 布局 + 位段 + 索引公式 + 方法清单")}
//! 字面量描述族: 对象/数组/正则/类字面量的 boilerplate 与共享闭包信息。
//! 配置: 64 位非压缩 (kTaggedSize=8, FixedArrayBase 头 = map(8)+length(4)+padding(4) = 16, Struct 头 = map(8))。

// —— ClassBoilerplate::ValueKind ——
/// 原版 enum ValueKind。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ValueKind {
  /// kData
  Data = 0,
  /// kGetter
  Getter = 1,
  /// kSetter
  Setter = 2,
  /// kAutoAccessor
  AutoAccessor = 3,
}

impl ValueKind {
  /// 变体数
  pub const COUNT: usize = 4;
}

// —— ComputedEntryFlags 位段 ——
/// 原版 ValueKindBits (shift 0, 2 bit)。
pub const COMPUTED_ENTRY_VALUE_KIND_SHIFT: u32 = 0;
pub const COMPUTED_ENTRY_VALUE_KIND_BITS: u32 = 2;
/// 原版 KeyIndexBits (shift 2, 29 bit)。
pub const COMPUTED_ENTRY_KEY_INDEX_SHIFT: u32 = 2;
pub const COMPUTED_ENTRY_KEY_INDEX_BITS: u32 = 29;

// —— DefineClassArgumentsIndices ——
/// 原版 kConstructorArgumentIndex。
pub const K_CONSTRUCTOR_ARGUMENT_INDEX: i32 = 1;
/// 原版 kPrototypeArgumentIndex。
pub const K_PROTOTYPE_ARGUMENT_INDEX: i32 = 2;
/// 原版 kFirstDynamicArgumentIndex (方法闭包与计算属性名的首个动态实参)。
pub const K_FIRST_DYNAMIC_ARGUMENT_INDEX: i32 = 3;

// —— ClassBoilerplate 常量 ——
/// 原版 kMinimumClassPropertiesCount。
pub const K_MINIMUM_CLASS_PROPERTIES_COUNT: usize = 6;
/// 原版 kMinimumPrototypePropertiesCount。
pub const K_MINIMUM_PROTOTYPE_PROPERTIES_COUNT: usize = 1;

// —— 64 位布局 ——
/// v8: src/objects/literal-objects.h ObjectBoilerplateDescription:
///   kHeaderSize = kLengthOffset(8) + 3 * kTaggedSize = 32 (TAGGED_SIZE_8_BYTES);
///   固定字段 backing_store_size(32)/flags(40), raw_entries 自 48 起 (TaggedArrayBase 尾续)
pub const K_OBJECT_BOILERPLATE_DESCRIPTION_HEADER_SIZE: usize = ${K_OBD_HEADER};
/// v8: src/objects/literal-objects.h ArrayBoilerplateDescription: flags(8) + constant_elements(16)
pub const K_ARRAY_BOILERPLATE_DESCRIPTION_SIZE: usize = ${K_ABD_HEADER};
/// v8: src/objects/literal-objects.h RegExpBoilerplateDescription:
///   data (TrustedPointerMember, 8) + flags (SmiTagged<JSRegExpFlags>, 8)
pub const K_REG_EXP_BOILERPLATE_DESCRIPTION_SIZE: usize = ${K_RBD_SIZE};
/// v8: src/objects/literal-objects.h ClassBoilerplate: 7 字段 (arguments_count Smi + 6 tagged)
pub const K_CLASS_BOILERPLATE_HEADER_SIZE: usize = ${K_CLASS_HEADER};
/// v8: src/objects/literal-objects.h PrototypeSharedClosureInfo: 3 tagged 字段
pub const K_PROTOTYPE_SHARED_CLOSURE_INFO_SIZE: usize = ${K_PSCI_HEADER};

// —— ObjectBoilerplateDescription 索引公式 ——
/// 原版 kElementsPerEntry。
pub const K_ELEMENTS_PER_ENTRY: usize = 2;

/// 原版 NameIndex(i) = i * 2。
#[inline]
#[must_use]
pub const fn name_index(i: usize) -> usize {
  i * K_ELEMENTS_PER_ENTRY
}

/// 原版 ValueIndex(i) = i * 2 + 1。
#[inline]
#[must_use]
pub const fn value_index(i: usize) -> usize {
  i * K_ELEMENTS_PER_ENTRY + 1
}

/// 原版 New: capacity = boilerplate * kElementsPerEntry (上限 kMaxCapacity 检查)。
#[inline]
#[must_use]
pub const fn obd_capacity(boilerplate: u32) -> u32 {
  boilerplate * K_ELEMENTS_PER_ENTRY as u32
}

/// 原版 boilerplate_properties_count = capacity / kElementsPerEntry。
#[inline]
#[must_use]
pub const fn boilerplate_properties_count(capacity: u32) -> usize {
  (capacity / K_ELEMENTS_PER_ENTRY as u32) as usize
}

// —— tq 布局表 ——
/// tq 5 类字段表 (1:1 顺序)。
pub const LITERAL_OBJECTS_TQ_LAYOUT: &[(&str, &str, &[&str])] = &[
${TQ_EXPECT.map(([n, e, fs]) => `  ("${n}", "${e}", &[${fs.map((f) => `"${f}"`).join(", ")}]),`).join("\n")}
];

// —— -inl 方法清单 (${inlDefs.length} 定义) ——
/// v8: src/objects/literal-objects-inl.h — Rust 无 inl, 语义并入实现。
pub const LITERAL_OBJECTS_INL_METHODS: &[&str] = &[
${inlDefs.map((d) => `  "${d}",`).join("\n")}
];

// —— cc 语义要点 ——
/// v8: src/objects/literal-objects.cc
///   AddToPropertiesTemplate/AddToElementsTemplate → AddToDictionaryTemplate 转发
///   (NameDictionary/SwissNameDictionary/NumberDictionary 三宿主, 模板显式实例化)
///   ClassBoilerplate::New: static/instance 两 ObjectDescriptor (最小容量 6/1) →
///   AddToPropertiesTemplate/AddToElementsTemplate 填充 → 构造实参排列
///   [constructor(1), prototype(2), dynamic(3..)]
pub const LITERAL_OBJECTS_CC_METHODS: &[&str] = &[
${ccDefs.map((d) => `  "${d}",`).join("\n")}
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(ValueKind::COUNT == 4);
  assert!(COMPUTED_ENTRY_VALUE_KIND_BITS + COMPUTED_ENTRY_KEY_INDEX_BITS == 31);
  assert!(K_CONSTRUCTOR_ARGUMENT_INDEX == 1);
  assert!(K_PROTOTYPE_ARGUMENT_INDEX == 2);
  assert!(K_FIRST_DYNAMIC_ARGUMENT_INDEX == 3);
  assert!(K_MINIMUM_CLASS_PROPERTIES_COUNT == 6);
  assert!(K_MINIMUM_PROTOTYPE_PROPERTIES_COUNT == 1);
  assert!(K_OBJECT_BOILERPLATE_DESCRIPTION_HEADER_SIZE == 32);
  assert!(K_ARRAY_BOILERPLATE_DESCRIPTION_SIZE == 24);
  assert!(K_REG_EXP_BOILERPLATE_DESCRIPTION_SIZE == 24);
  assert!(K_CLASS_BOILERPLATE_HEADER_SIZE == 64);
  assert!(K_PROTOTYPE_SHARED_CLOSURE_INFO_SIZE == 32);
  assert!(name_index(3) == 6);
  assert!(value_index(3) == 7);
  assert!(obd_capacity(5) == 10);
  assert!(boilerplate_properties_count(10) == 5);
  assert!(LITERAL_OBJECTS_TQ_LAYOUT.len() == 5);
};
`;

const p = writeSrc("v8-objects-js-func", "literal_objects.rs", body);
console.log(
  `literal-objects.h.js: ${p} — tq 类 5 (字段 ${TQ_EXPECT.reduce((a, [, , fs]) => a + fs.length, 0)})、` +
    `ValueKind ${valueKinds.length}、位段 2 (${bitFields.map((b) => b[2]).join("+")})、索引公式 3、` +
    `布局 5 值、-inl ${inlDefs.length}、cc ${ccDefs.length}、断言 ${checks} 项通过`,
);
