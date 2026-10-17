#!/usr/bin/env bun
// tool/src/objects/dictionary.h.js — 覆盖 dictionary 同名族:
//   v8/src/objects/dictionary.h + dictionary-inl.h
//   → v8-objects-collections/src/dictionary.rs
// 机械提取: Dictionary 各 Shape 布局常量 + BaseNameDictionary 前缀槽位
//   + NameDictionary flags 位段 + NumberDictionary 慢元素位掩码;
// 纯计算 (max_number_key 解码) 落 const fn, 堆访问留骨架 + todo!("就地修订")。

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/dictionary.h");
const inl = readV8("src/objects/dictionary-inl.h");

// ── 锚点 (must 断言) ──────────────────────────────────────────────────────
const anchors = [
  [h, "class BaseDictionaryShape : public BaseShape<Key> {", "dictionary.h"],
  [h, "static const bool kHasDetails = true;", "dictionary.h (BaseDictionaryShape)"],
  [h, "class NameDictionaryShape : public BaseNameDictionaryShape {", "dictionary.h"],
  [h, "static const int kPrefixSize = 3;\n  static const int kEntrySize = 3;", "dictionary.h (NameDictionaryShape)"],
  [h, "static const int kFlagsIndex = kObjectHashIndex + 1;", "dictionary.h (NameDictionary)"],
  [h, "static const int kEntryDetailsIndex = 2;", "dictionary.h (NameDictionary)"],
  [h, "static const int kInitialCapacity = 2;", "dictionary.h (NameDictionary)"],
  [h, "using MayHaveInterestingPropertiesBit = base::BitField<bool, 0, 1, uint32_t>;", "dictionary.h"],
  [h, "static constexpr int kFlagsDefault = 0;", "dictionary.h"],
  [h, "static const int kMaxNumberKeyIndex = kPrefixStartIndex;", "dictionary.h (NumberDictionary)"],
  [h, "static const int kRequiresSlowElementsMask = 1;", "dictionary.h"],
  [h, "static const int kRequiresSlowElementsTagSize = 1;", "dictionary.h"],
  [h, "static const uint32_t kRequiresSlowElementsLimit = (1 << 29) - 1;", "dictionary.h"],
  [h, "static const uint32_t kPreferFastElementsSizeFactor = 3;", "dictionary.h"],
  [h, "static const int kNextEnumerationIndexIndex =\n      HashTableBase::kPrefixStartIndex;", "dictionary.h (BaseNameDictionary)"],
  [inl, "return value >> kRequiresSlowElementsTagSize;", "dictionary-inl.h (max_number_key)"],
];
for (const [s, sub, f] of anchors) {
  if (!s.includes(sub)) throw new Error(`锚点失配: ${f} 无 "${sub.slice(0, 52)}..."`);
}

// ── Shape 常量逐条解析 + 数量 must 断言 ──────────────────────────────────
function classBody(src, className) {
  const re = new RegExp(`class (?:V8_EXPORT_PRIVATE )?${className}(?![\\w])`);
  const m = re.exec(src);
  if (!m) throw new Error(`未找到类 ${className}`);
  const rest = src.slice(m.index);
  const j = rest.slice(1).search(/\nclass |\ntemplate </);
  return j < 0 ? rest : rest.slice(0, j + 1);
}
const grab = (s, re, f) => {
  const m = s.match(re);
  if (!m) throw new Error(`常量提取失配: ${f}`);
  return m[1];
};
const shapes = {
  BaseNameDictionaryShape: {
    kEntryValueIndex: Number(grab(classBody(h, "BaseNameDictionaryShape"), /static const int kEntryValueIndex = (\d+);/, "BaseNameDictionaryShape.kEntryValueIndex")),
  },
  NameDictionaryShape: {
    kPrefixSize: Number(grab(classBody(h, "NameDictionaryShape"), /static const int kPrefixSize = (\d+);/, "NameDictionaryShape.kPrefixSize")),
    kEntrySize: Number(grab(classBody(h, "NameDictionaryShape"), /static const int kEntrySize = (\d+);/, "NameDictionaryShape.kEntrySize")),
  },
  SimpleNameDictionaryShape: {
    kHasDetails: false,
    kPrefixSize: Number(grab(classBody(h, "SimpleNameDictionaryShape"), /static const int kPrefixSize = (\d+);/, "SimpleNameDictionaryShape.kPrefixSize")),
    kEntrySize: Number(grab(classBody(h, "SimpleNameDictionaryShape"), /static const int kEntrySize = (\d+);/, "SimpleNameDictionaryShape.kEntrySize")),
  },
  GlobalDictionaryShape: {
    kPrefixSize: Number(grab(classBody(h, "GlobalDictionaryShape"), /static const int kPrefixSize = (\d+);/, "GlobalDictionaryShape.kPrefixSize")),
    kEntrySize: Number(grab(classBody(h, "GlobalDictionaryShape"), /static const int kEntrySize = (\d+);/, "GlobalDictionaryShape.kEntrySize")),
  },
  NumberDictionaryShape: {
    kPrefixSize: Number(grab(classBody(h, "NumberDictionaryShape"), /static const int kPrefixSize = (\d+);/, "NumberDictionaryShape.kPrefixSize")),
    kEntrySize: Number(grab(classBody(h, "NumberDictionaryShape"), /static const int kEntrySize = (\d+);/, "NumberDictionaryShape.kEntrySize")),
  },
  SimpleNumberDictionaryShape: {
    kPrefixSize: Number(grab(classBody(h, "SimpleNumberDictionaryShape"), /static const int kPrefixSize = (\d+);/, "SimpleNumberDictionaryShape.kPrefixSize")),
    kEntrySize: Number(grab(classBody(h, "SimpleNumberDictionaryShape"), /static const int kEntrySize = (\d+);/, "SimpleNumberDictionaryShape.kEntrySize")),
  },
};
const shapeCount = Object.keys(shapes).length;
if (shapeCount !== 6) throw new Error(`Dictionary Shape 应 6 个, 实得 ${shapeCount}`);
// 关键数值 must 断言 (禁吞账)
const must = (n, k, v) => {
  if (shapes[n][k] !== v) throw new Error(`${n}.${k} 应 ${v}, 实得 ${shapes[n][k]}`);
};
must("NameDictionaryShape", "kPrefixSize", 3);
must("NameDictionaryShape", "kEntrySize", 3);
must("BaseNameDictionaryShape", "kEntryValueIndex", 1);
must("SimpleNameDictionaryShape", "kPrefixSize", 0);
must("SimpleNameDictionaryShape", "kEntrySize", 2);
must("GlobalDictionaryShape", "kPrefixSize", 2);
must("GlobalDictionaryShape", "kEntrySize", 1);
must("NumberDictionaryShape", "kPrefixSize", 1);
must("NumberDictionaryShape", "kEntrySize", 3);
must("SimpleNumberDictionaryShape", "kPrefixSize", 0);
must("SimpleNumberDictionaryShape", "kEntrySize", 2);
// kMatchNeedsHoleCheck: Global 系 / NumberDictionaryBaseShape 为 true;
// Name/SimpleName/SimpleNumber 系为 false (NameDictionaryShape 亦无覆盖 → 继承 base 的 false)
for (const [n, v] of [["GlobalDictionaryShape", true], ["NumberDictionaryBaseShape", true], ["SimpleNameDictionaryShape", false]]) {
  if (!classBody(h, n).includes(`static const bool kMatchNeedsHoleCheck = ${v};`)) {
    throw new Error(`${n}.kMatchNeedsHoleCheck != ${v}`);
  }
}

// ── -inl 访问器骨架清单 ───────────────────────────────────────────────────
const inlDefs = [...inl.matchAll(/^([A-Za-z][\w:<>,& ]*?)\s*(\w+)::(\w+)\(/gm)].map((m) => `${m[2]}::${m[3]}`);
const inlUnique = [...new Set(inlDefs)];
if (inlUnique.length < 10) throw new Error(`dictionary-inl.h 定义应 ≥10, 实得 ${inlUnique.length}`);

const body = `${banner("src/objects/dictionary.h.js", "src/objects/dictionary.{h,-inl.h}", "Dictionary 族 Shape 布局与慢元素位掩码 (规则面)")}
//! Dictionary 家族: HashTable 之上的键值字典 (可带 PropertyDetails)。
//!
//! - entry 布局: [key, value(, details)] (kEntryValueIndex=1, kEntryDetailsIndex=2)
//! - NameDictionary 前缀: [0]=下一枚举索引 [1]=对象哈希 [2]=flags
//! - NumberDictionary 前缀: [0]=max_number_key (含 requires_slow_elements 位)

// ── 前缀槽位 (BaseNameDictionary / dictionary.h:189-192) ───────────────
/// 下一枚举索引槽 (NameDictionary/GlobalDictionary 前缀 [0])。
pub const K_NEXT_ENUMERATION_INDEX_INDEX: usize = 3; // = HashTableBase::kPrefixStartIndex
/// 对象哈希槽 (前缀 [1])。
pub const K_OBJECT_HASH_INDEX: usize = K_NEXT_ENUMERATION_INDEX_INDEX + 1;
/// entry 内 value 下标。
pub const K_ENTRY_VALUE_INDEX: usize = 1;
/// entry 内 PropertyDetails 下标。
pub const K_ENTRY_DETAILS_INDEX: usize = 2;

// ── NameDictionary (dictionary.h:284-316) ──────────────────────────────
/// flags 槽 (NameDictionary 前缀 [3] = kObjectHashIndex + 1)。
pub const K_FLAGS_INDEX: usize = K_OBJECT_HASH_INDEX + 1;
/// NameDictionary 初始容量 (kInitialCapacity)。
pub const K_INITIAL_CAPACITY: usize = 2;
/// flags 默认值。
pub const K_FLAGS_DEFAULT: u32 = 0;
/// flags 位段: may_have_interesting_properties (bit0, 1 位)。
pub const MAY_HAVE_INTERESTING_PROPERTIES_SHIFT: u32 = 0;
/// 位宽。
pub const MAY_HAVE_INTERESTING_PROPERTIES_SIZE: u32 = 1;
/// 掩码。
pub const MAY_HAVE_INTERESTING_PROPERTIES_MASK: u32 = 1;

// ── NumberDictionary 慢元素位掩码 (dictionary.h:475-482) ───────────────
/// max_number_key 的 requires_slow_elements 位 (bit0)。
pub const K_REQUIRES_SLOW_ELEMENTS_MASK: u32 = 1;
/// 位移右移量 (真实键值 = max_index_object >> 1)。
pub const K_REQUIRES_SLOW_ELEMENTS_TAG_SIZE: u32 = 1;
/// 触发慢元素的索引上限: (1 << 29) - 1。
pub const K_REQUIRES_SLOW_ELEMENTS_LIMIT: u32 = (1 << 29) - 1;
/// JSObject 偏好字典元素的经验因子 (省内存阈值)。
pub const K_PREFER_FAST_ELEMENTS_SIZE_FACTOR: u32 = 3;
/// max_number_key 前缀槽 (NumberDictionary 前缀 [0] = kPrefixStartIndex)。
pub const K_MAX_NUMBER_KEY_INDEX: usize = 3;

/// 是否要求慢元素 (dictionary-inl.h:125 requires_slow_elements)。
#[inline]
#[must_use]
pub const fn requires_slow_elements(max_index_object: u32) -> bool {
  0 != (max_index_object & K_REQUIRES_SLOW_ELEMENTS_MASK)
}

/// 解码真实 max_number_key (dictionary-inl.h:133 max_number_key)。
///
/// 契约: 仅在 requires_slow_elements() 为假时结果有意义 (原版注释)。
#[inline]
#[must_use]
pub const fn max_number_key(max_index_object: u32) -> u32 {
  max_index_object >> K_REQUIRES_SLOW_ELEMENTS_TAG_SIZE
}

/// 构造 requires_slow_elements 标记值 (set_requires_slow_elements: kMaxNumberKeyIndex ← 掩码)。
#[inline]
#[must_use]
pub const fn requires_slow_elements_marker() -> u32 {
  K_REQUIRES_SLOW_ELEMENTS_MASK
}

// ── Shape 规则面 (dictionary.h 展开) ───────────────────────────────────
/// Dictionary Shape 静态参数。
pub struct DictionaryShapeParams {
  /// 前缀槽数。
  pub prefix_size: usize,
  /// 每 entry 槽数。
  pub entry_size: usize,
  /// entry 是否含 PropertyDetails 槽。
  pub has_details: bool,
}
/// NameDictionaryShape (key/value/details, 前缀 3 槽)。
pub const NAME_DICTIONARY_SHAPE: DictionaryShapeParams = DictionaryShapeParams {
  prefix_size: ${shapes.NameDictionaryShape.kPrefixSize},
  entry_size: ${shapes.NameDictionaryShape.kEntrySize},
  has_details: true,
};
/// SimpleNameDictionaryShape (key/value, 无前缀无 details)。
pub const SIMPLE_NAME_DICTIONARY_SHAPE: DictionaryShapeParams = DictionaryShapeParams {
  prefix_size: ${shapes.SimpleNameDictionaryShape.kPrefixSize},
  entry_size: ${shapes.SimpleNameDictionaryShape.kEntrySize},
  has_details: false,
};
/// GlobalDictionaryShape (key=PropertyCell, 前缀 2 槽)。
pub const GLOBAL_DICTIONARY_SHAPE: DictionaryShapeParams = DictionaryShapeParams {
  prefix_size: ${shapes.GlobalDictionaryShape.kPrefixSize},
  entry_size: ${shapes.GlobalDictionaryShape.kEntrySize},
  has_details: true,
};
/// NumberDictionaryShape (key/value/details, 前缀 1 槽)。
pub const NUMBER_DICTIONARY_SHAPE: DictionaryShapeParams = DictionaryShapeParams {
  prefix_size: ${shapes.NumberDictionaryShape.kPrefixSize},
  entry_size: ${shapes.NumberDictionaryShape.kEntrySize},
  has_details: true,
};
/// SimpleNumberDictionaryShape (key/value, 无前缀无 details)。
pub const SIMPLE_NUMBER_DICTIONARY_SHAPE: DictionaryShapeParams = DictionaryShapeParams {
  prefix_size: ${shapes.SimpleNumberDictionaryShape.kPrefixSize},
  entry_size: ${shapes.SimpleNumberDictionaryShape.kEntrySize},
  has_details: false,
};

// ── 堆访问签名骨架 (就地修订; 参数为 Address/索引占位) ─────────────────
/// 读 entry 的 PropertyDetails (BaseDictionaryShape::DetailsAt)。
pub fn details_at(_dict: u64, _entry: u32) -> u32 { todo!("就地修订") }
/// 写 entry 的 PropertyDetails (DetailsAtPut)。
pub fn details_at_put(_dict: u64, _entry: u32, _value: u32) { todo!("就地修订") }
/// 删除 entry (ClearEntry: 键/值/细节置 the_hole)。
pub fn clear_entry(_dict: u64, _entry: u32) { todo!("就地修订") }
/// 写整 entry (SetEntry: key/value/details)。
pub fn set_entry(_dict: u64, _entry: u32, _key: u64, _value: u64, _details: u32) { todo!("就地修订") }
/// Add 派生流 (AtPut → FindInsertionEntry + SetEntry + 计数)。
pub fn at_put(_isolate: u64, _dict: u64, _key: u64, _value: u64, _details: u32) -> u64 { todo!("就地修订") }
/// 慢速反查 key (SlowReverseLookup)。
pub fn slow_reverse_lookup(_dict: u64, _value: u64) -> u64 { todo!("就地修订") }

// dictionary-inl.h 定义清单 (对数基准, ${inlUnique.length} 项):
${inlUnique.map((s) => `// ${s}`).join("\n")}

// —— 提取不变式断言 ——
const _: () = {
  assert!(K_NEXT_ENUMERATION_INDEX_INDEX == 3 && K_OBJECT_HASH_INDEX == 4 && K_FLAGS_INDEX == 5);
  assert!(K_ENTRY_VALUE_INDEX == 1 && K_ENTRY_DETAILS_INDEX == 2);
  assert!(K_INITIAL_CAPACITY == 2 && K_FLAGS_DEFAULT == 0);
  assert!(MAY_HAVE_INTERESTING_PROPERTIES_MASK == 1);
  assert!(K_REQUIRES_SLOW_ELEMENTS_MASK == 1 && K_REQUIRES_SLOW_ELEMENTS_TAG_SIZE == 1);
  assert!(K_REQUIRES_SLOW_ELEMENTS_LIMIT == (1 << 29) - 1);
  assert!(K_PREFER_FAST_ELEMENTS_SIZE_FACTOR == 3);
  assert!(!requires_slow_elements(0) && requires_slow_elements(1) && requires_slow_elements(6));
  assert!(max_number_key(6) == 3);
  assert!(NAME_DICTIONARY_SHAPE.prefix_size == 3 && NAME_DICTIONARY_SHAPE.entry_size == 3);
  assert!(GLOBAL_DICTIONARY_SHAPE.entry_size == 1 && GLOBAL_DICTIONARY_SHAPE.prefix_size == 2);
  assert!(!SIMPLE_NAME_DICTIONARY_SHAPE.has_details && SIMPLE_NAME_DICTIONARY_SHAPE.entry_size == 2);
  assert!(NUMBER_DICTIONARY_SHAPE.prefix_size == 1 && SIMPLE_NUMBER_DICTIONARY_SHAPE.prefix_size == 0);
};
`;

writeSrc("v8-objects-collections", "dictionary.rs", body);
console.log(
  `dictionary.rs: Shape ${shapeCount} + 前缀槽位 3 + 慢元素常量 5 + const fn 3 + 骨架 6 + inl 定义 ${inlUnique.length}, 对数通过`,
);
