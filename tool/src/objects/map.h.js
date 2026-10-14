#!/usr/bin/env bun
// tool/src/objects/map.h.js — 覆盖 map.h 同名族骨架:
//   v8/src/objects/map.h + map-inl.h (Map 自身基础语义)
//   → v8-objects-core/src/map.rs
//   (map.cc 的 GetVisitorId 大表 / ToString(VisitorId) / slack 语义由
//    tool/src/objects/map.cc.js 以 @cc-semantics 区段注入)
//
// 映射登记 (C++ → Rust):
//   DATA_ONLY_VISITOR_ID_LIST + POINTER_VISITOR_ID_LIST + TRUSTED_VISITOR_ID_LIST
//     (X-macro, POINTER 尾部 SIMPLE_HEAP_OBJECT_LIST1(V) 与 TRUSTED=
//      CONCRETE_TRUSTED_OBJECT_TYPE_LIST1(V) 自 object-list-macros.h 展开,
//      IF_WASM 按默认配置展开) → enum VisitorId (kVisitFoo → VisitFoo) +
//     DataOnlyVisitorIdCount/VisitorIdCount 哨兵 (map.h:34-160)
//   enum ObjectFields + ObjectFieldsFrom → enum + 关联函数
//   Bits1/Bits2/Bits3/BitsEx (BitFieldGroup 链) → 显式 SHIFT/SIZE/MASK 常量 +
//     取放函数 (逐位对齐: bit_field = callable0|undetectable1|named_int2|
//     access_check3|indexed_int4|constructor5|extended6;
//     bit_field2 = new_target0|immutable_proto1|elements_kind2..7(6bit);
//     bit_field3 = enum_length0..9|nod10..19|proto20|dict21|owns22|retained23|
//     deprecated24|unstable25|migration26|extensible27|interesting28|counter29..31)
//   Map 字段 → 逻辑布局结构体 (字段序对齐原版; Tagged 为 usize 逻辑宽,
//     snapshot 偏移由 K_*_OFFSET 常量按 K_TAGGED_SIZE=4 (压缩默认) 对齐,
//     TAGGED_SIZE_8_BYTES 分支经 v8-common TAGGED_SIZE_8_BYTES 常量抽象)
//   instance_size 族 → word 编码 (u8 words; bytes = words << K_TAGGED_SIZE_LOG2)
//   inobject/unused 属性记账族 (map-inl.h:434-632) → 显式方法
//     (JSObject::kFieldsAdded=3 按 js-objects.h:982 借位登记)
//   slack tracking 常量族 (map.h:443-479) + StartInobjectSlackTracking/
//     IsInobjectSlackTrackingInProgress (map-inl.h/cc)
//   ExtendedMap (map.h:1264-1297) → struct + BitsEx + map_kind/map_size/
//     set_map_kind_and_size (map-inl.h:1285-1327)
//
// 范围裁剪登记 (推迟, 归 v8-objects-maps crate — transition 体系深层与 map-updater):
//   - raw_transitions/prototype_info 槽位的类型窄化视图与 TransitionsAccessor
//     (map-inl.h:94-161): transitions 数组/编码派发
//   - Copy/Normalize/TryUpdate/Update/CopyWithField/TransitionTo* 全家族
//     (map.cc 工厂/迁移, 需要 isolate+DescriptorArray)
//   - MapUpdater/DeprecateTransitionTree/CompleteInobjectSlackTracking
//   - ComputeMinObjectSlack/InobjectSlackTrackingStep (transition 树遍历)
//   - prototype_validity_cell 语义族 (GetOrCreatePrototypeChainValidityCell)
//   - Map::Hash (isolate hash_seed + identity hash 依赖)
//   - visitor_id 的类型化 ElementsKind 谓词族 (has_fast_elements 等,
//     elements-kind.h 归 elements crate; 此处保留 raw u8 存取)
//   - OnlyHasSimpleProperties 中 IsStringWrapperElementsKind 分支 (同上推迟)
//   - InstanceTypeChecker 谓词族 Is*Map (IsPrimitiveMap 等, 由
//     instance_type_checker.rs 承载, 此处登记归属)
import { banner, readV8, writeSrc } from "../../lib.js";
import { readFileSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dir, "../../..");
const src = readV8("src/objects/map.h");
const mapInl = readV8("src/objects/map-inl.h");
const listMacros = readV8("src/objects/object-list-macros.h");
const instanceTypeRs = readFileSync(join(root, "v8-objects-core/src/instance_type.rs"), "utf8");

const mustContain = (s, sub, file) => {
  if (!s.includes(sub)) throw new Error(`锚点失配: ${file} 无 "${sub.slice(0, 48)}..."`);
};
mustContain(src, "#define DATA_ONLY_VISITOR_ID_LIST(V)", "map.h");
mustContain(src, "#define POINTER_VISITOR_ID_LIST(V)", "map.h");
mustContain(src, "#define TRUSTED_VISITOR_ID_LIST(V) CONCRETE_TRUSTED_OBJECT_TYPE_LIST1(V)", "map.h");
mustContain(src, "struct Bits1 : BitFieldGroup<Bits1, uint8_t> {", "map.h");
mustContain(src, "using EnumLengthBits = base::BitField<int32_t, 0, 10, uint32_t>;", "map.h");
mustContain(src, "using ConstructionCounterBits =", "map.h");
mustContain(src, "using MapKindBits = base::BitField<ExtendedMapKind, 0, 3, uint8_t>;", "map.h");
mustContain(src, "kSlackTrackingCounterStart = 7;", "map.h");
mustContain(mapInl, "int Map::instance_size() const {", "map-inl.h");

// ── visitor id 三列表机械展开 ─────────────────────────────────────────────
function extractList(name, s) {
  const re = new RegExp(`#define ${name}\\(V\\) ([\\\\\\s\\S]*?)\\n\\n`);
  const m = s.match(re);
  if (!m) throw new Error(`找不到 #define ${name}(V)`);
  return m[1];
}
// V(Foo) 提取 (含 IF_WASM(V, WasmNull) 展开 → 追加 WasmNull)
function entriesOf(def) {
  const out = [];
  for (const m of def.matchAll(/V\((\w+)\)/g)) out.push(m[1]);
  for (const m of def.matchAll(/IF_WASM\(V,\s*(\w+)\)/g)) out.push(m[1]);
  return out;
}
// APPLY(V, Name, NAME) 提取 (object-list-macros GENERATOR 体;
// 含 IF_WASM(APPLY, V, Name, NAME) 展开; 先吞续行反斜杠再匹配)
function applyEntriesOf(def) {
  const flat = def.replace(/\\\n/g, " ");
  const out = [];
  for (const m of flat.matchAll(/APPLY\(V,\s*(\w+),\s*(\w+)\)/g)) out.push(m[1]);
  for (const m of flat.matchAll(/IF_WASM\(APPLY, V,\s*(\w+),\s*(\w+)\)/g)) out.push(m[1]);
  return out;
}
const dataOnly = entriesOf(extractList("DATA_ONLY_VISITOR_ID_LIST", src));
const pointerDef = extractList("POINTER_VISITOR_ID_LIST", src);
const simpleHeap = applyEntriesOf(
  listMacros.match(/#define SIMPLE_HEAP_OBJECT_LIST_GENERATOR\(APPLY, V\)([\s\S]*?)\n\n/)[1],
);
const trusted = applyEntriesOf(
  listMacros.match(/#define CONCRETE_TRUSTED_OBJECT_LIST_GENERATOR\(APPLY, V\)([\s\S]*?)\n\n/)[1],
);
if (!pointerDef.includes("SIMPLE_HEAP_OBJECT_LIST1(V)")) throw new Error("POINTER 列表尾部锚点失配");
const pointer = entriesOf(pointerDef.replace("SIMPLE_HEAP_OBJECT_LIST1(V)", "")).concat(simpleHeap);
// 对数锚点 (实测值)
if (dataOnly.length !== 20) throw new Error(`DATA_ONLY ${dataOnly.length} != 20`);
if (pointer.length !== 97) throw new Error(`POINTER ${pointer.length} != 97`);
if (trusted.length !== 27) throw new Error(`TRUSTED ${trusted.length} != 27`);

const allIds = [...dataOnly, ...pointer, ...trusted];
if (new Set(allIds).size !== allIds.length) throw new Error("visitor id 变体重复");

// ── Bits 位段表 (BitFieldGroup Next 链机械推导) ────────────────────────────
const BITS1 = [
  ["is_callable", 0, 1],
  ["is_undetectable", 1, 1],
  ["has_named_interceptor", 2, 1],
  ["is_access_check_needed", 3, 1],
  ["has_indexed_interceptor", 4, 1],
  ["is_constructor", 5, 1],
  ["is_extended_map", 6, 1],
];
const BITS2 = [
  ["new_target_is_base", 0, 1],
  ["is_immutable_prototype", 1, 1],
  ["elements_kind", 2, 6],
];
const BITS3 = [
  ["enum_length", 0, 10],
  ["number_of_own_descriptors", 10, 10],
  ["is_prototype_map", 20, 1],
  ["is_dictionary_map", 21, 1],
  ["owns_descriptors", 22, 1],
  ["is_in_retained_map_list", 23, 1],
  ["is_deprecated", 24, 1],
  ["is_unstable", 25, 1],
  ["is_migration_target", 26, 1],
  ["is_extensible", 27, 1],
  ["may_have_interesting_properties", 28, 1],
  ["construction_counter", 29, 3],
];
const BITS_EX = [
  ["map_kind", 0, 3],
  ["map_size_in_words", 3, 5],
];
// 布局对数: BITS3 尾部 counter 29..31 (ConstructionCounterBits kMax=7)
if (BITS3.at(-1)[1] + BITS3.at(-1)[2] !== 32) throw new Error("Bits3 布局失配");
if (BITS_EX.at(-1)[1] + BITS_EX.at(-1)[2] !== 8) throw new Error("BitsEx 布局失配");
// 原版 static_assert(EnumLengthBits::kSize == kDescriptorIndexBitCount)
if (BITS3[0][2] !== 10 || BITS3[1][2] !== 10) throw new Error("descriptor index 位宽失配");

const UP = (s) => s.toUpperCase();
const bitsConsts = (group, ty, table) =>
  table
    .map(([n, sh, sz]) => {
      const mask = sh === 0 ? `(1${ty} << ${sz}) - 1` : `((1${ty} << ${sz}) - 1) << ${sh}`;
      return `pub const ${UP(n)}_SHIFT: ${ty} = ${sh};\npub const ${UP(n)}_SIZE: ${ty} = ${sz};\npub const ${UP(n)}_MASK: ${ty} = ${mask};`;
    })
    .join("\n");

// bool 位访问器生成 (原 BIT_FIELD_ACCESSORS 机械展开)
const boolAcc = (field, group, ty, [n, sh]) => `
  #[inline]
  pub const fn ${n}(&self) -> bool {
    (self.${field} & Self::${UP(n)}_MASK) != 0
  }

  #[inline]
  pub fn set_${n}(&mut self, value: bool) {
    self.${field} = (self.${field} & !Self::${UP(n)}_MASK) | ((value as ${ty}) << Self::${UP(n)}_SHIFT);
  }`;

const bits1Accs = BITS1.filter(([, , sz]) => sz === 1).map((f) => boolAcc("bit_field", "Bits1", "u8", f)).join("\n");
// is_dictionary_map 的 setter 与 is_unstable 联动 (map-inl.h:866), 从机械组剔除手写
const bits2BoolAccs = BITS2.filter(([, , sz]) => sz === 1).map((f) => boolAcc("bit_field2", "Bits2", "u8", f)).join("\n");
const bits3BoolAccs = BITS3.filter(([nm, , sz]) => sz === 1 && nm !== "is_dictionary_map")
  .map((f) => boolAcc("bit_field3", "Bits3", "u32", f))
  .join("\n");

const visitorEnum = [
  ...dataOnly.map((id) => `  Visit${id},`),
  "  /// data-only 访客 id 数 (map.h:154 kDataOnlyVisitorIdCount)。",
  "  DataOnlyVisitorIdCount,",
  ...pointer.map((id) => `  Visit${id},`),
  ...trusted.map((id) => `  Visit${id},`),
  "  /// 访客 id 总数 (map.h:157 kVisitorIdCount)。",
  "  VisitorIdCount,",
].join("\n");

const body = `${banner("src/objects/map.h.js", "src/objects/map.h + map-inl.h", "visitor id 三列表 + Bits1/2/3/Ex 位段 + 布局常量机械展开; transition 深层推迟归 v8-objects-maps (脚本头登记)")}
//! Map: 描述堆对象形状的元对象 (实例大小 / 位域标志 / 属性描述结构)。
//!
//! 偏移常量按压缩指针默认配置 (K_TAGGED_SIZE=4) 对齐原版堆布局
//! (snapshot 兼容前提); 8 字节 tagged 配置经 v8_common::globals::TAGGED_SIZE_8_BYTES
//! 常量分支。Rust 结构体为逻辑布局 (Tagged 为 usize 宽), 槽位读写走
//! tagged_field 机制。

use v8_common::{ExtendedMapKind, K_TAGGED_SIZE, K_TAGGED_SIZE_LOG2, TAGGED_SIZE_8_BYTES};

use crate::heap_object::HeapObject;
use crate::instance_type::InstanceType;
use crate::tagged::{Object, Tagged};

// ── VisitorId (map.h:34-160, 三列表 X-macro 机械展开) ─────────────────────
/// 堆访客 id: 同 id 的对象被堆访客以相同方式处理。
/// data-only 对象的 id 必须排在其它 id 之前 (kDataOnlyVisitorIdCount 快查依据)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum VisitorId {
${visitorEnum}
}

const _: () = assert!((VisitorId::VisitorIdCount as usize) < 256);

/// 对象字段类别 (map.h:164-167)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectFields {
  /// 纯数据对象 (无指针)。
  DataOnly,
  /// 可能含指针。
  MaybePointers,
}

/// 由访客 id 推对象字段类别 (map.h:1051-1055)。
#[inline]
#[must_use]
pub const fn object_fields_from(visitor_id: VisitorId) -> ObjectFields {
  if (visitor_id as u8) < (VisitorId::DataOnlyVisitorIdCount as u8) {
    ObjectFields::DataOnly
  } else {
    ObjectFields::MaybePointers
  }
}

// ── Bits1 (map.h:338-356; bit_field, u8) ─────────────────────────────────
// 编译器把 is_callable|is_undetectable 与 has_named_interceptor|
// is_access_check_needed 各当一个掩码用 (typeof / special receiver 检查),
// 保持每对位相邻 → arm64 逻辑立即数可编码。常量在 impl Map 内关联承载。

// ── Bits2 (map.h:373-385; bit_field2, u8) ────────────────────────────────

// ── Bits3 (map.h:391-421; bit_field3, u32) ───────────────────────────────

// ── BitsEx (map.h:1269-1277; bit_field_ex, u8; ExtendedMap 专用) ─────────
${bitsConsts("BitsEx", "u8", BITS_EX)}

// 编译期布局对数 (对齐原版 static_assert 组, map.h:424-429, 446)
const _: () = assert!(Map::ENUM_LENGTH_SIZE == crate::property_details::DESCRIPTOR_INDEX_BIT_COUNT);
const _: () = assert!(Map::NUMBER_OF_OWN_DESCRIPTORS_SIZE == crate::property_details::DESCRIPTOR_INDEX_BIT_COUNT);
const _: () = assert!((1 << Map::NUMBER_OF_OWN_DESCRIPTORS_SIZE) > crate::property_details::MAX_NUMBER_OF_DESCRIPTORS);
const _: () = assert!(SLACK_TRACKING_COUNTER_START < (1 << Map::CONSTRUCTION_COUNTER_SIZE));

/// 描述符索引哨兵 (property-details.h, 位宽与 Bits3 对齐锚点)。
pub use crate::property_details::INVALID_ENUM_CACHE_SENTINEL;

// ── slack tracking 常量 (map.h:443-479) ──────────────────────────────────
/// inobject slack tracking 起始计数。
pub const SLACK_TRACKING_COUNTER_START: u32 = 7;
/// slack tracking 结束计数。
pub const SLACK_TRACKING_COUNTER_END: u32 = 1;
/// 不进行 slack tracking 的计数哨兵。
pub const NO_SLACK_TRACKING: u32 = 0;
/// 宽裕分配计数 (start - end + 1)。
pub const GENEROUS_ALLOCATION_COUNT: u32 = SLACK_TRACKING_COUNTER_START - SLACK_TRACKING_COUNTER_END + 1;

// ── 布局常量 (对齐原版堆布局; 压缩指针默认 K_TAGGED_SIZE=4) ────────────────
/// Map 指针槽偏移 (heap 对象首字段)。
pub const K_MAP_OFFSET: usize = 0;
/// instance_size_in_words_ 字节偏移 (第一个 int 字段首字节)。
pub const K_INSTANCE_SIZE_IN_WORDS_OFFSET: usize = K_TAGGED_SIZE;
/// inobject_properties_start_or_constructor_function_index_ 偏移。
pub const K_INOBJECT_PROPERTIES_START_OR_CONSTRUCTOR_FUNCTION_INDEX_OFFSET: usize = K_TAGGED_SIZE + 1;
/// used_or_unused_instance_size_in_words_ 偏移。
pub const K_USED_OR_UNUSED_INSTANCE_SIZE_IN_WORDS_OFFSET: usize = K_TAGGED_SIZE + 2;
/// visitor_id_ 偏移。
pub const K_VISITOR_ID_OFFSET: usize = K_TAGGED_SIZE + 3;
/// instance_type_ 偏移 (== Internals::kMapInstanceTypeOffset = kTaggedSize + 4)。
pub const K_INSTANCE_TYPE_OFFSET: usize = K_TAGGED_SIZE + 4;
/// bit_field_ (Bits1) 偏移。
pub const K_BIT_FIELD_OFFSET: usize = K_TAGGED_SIZE + 6;
/// bit_field2_ (Bits2) 偏移。
pub const K_BIT_FIELD2_OFFSET: usize = K_TAGGED_SIZE + 7;
/// bit_field3_ (Bits3) 偏移。
pub const K_BIT_FIELD3_OFFSET: usize = K_TAGGED_SIZE + 8;
/// bit_field 段末字节偏移 (map.h:1251-1252 kBitFieldOffsetEnd)。
pub const K_BIT_FIELD_OFFSET_END: usize = K_BIT_FIELD_OFFSET;
/// 第一个 tagged 指针字段 (prototype_) 偏移;
/// 8 字节 tagged 配置下 bit_field3 后有 4 字节 optional padding (map.h:1231)。
pub const K_PROTOTYPE_OFFSET: usize = if TAGGED_SIZE_8_BYTES { K_TAGGED_SIZE + 16 } else { K_TAGGED_SIZE + 12 };
/// constructor_or_back_pointer_or_native_context_ 偏移。
pub const K_CONSTRUCTOR_OR_BACK_POINTER_OR_NATIVE_CONTEXT_OFFSET: usize = K_PROTOTYPE_OFFSET + K_TAGGED_SIZE;
/// instance_descriptors_ 偏移 (非 wasm: DescriptorArray)。
pub const K_INSTANCE_DESCRIPTORS_OFFSET: usize = K_CONSTRUCTOR_OR_BACK_POINTER_OR_NATIVE_CONTEXT_OFFSET + K_TAGGED_SIZE;
/// dependent_code_ 偏移 (非 wasm)。
pub const K_DEPENDENT_CODE_OFFSET: usize = K_INSTANCE_DESCRIPTORS_OFFSET + K_TAGGED_SIZE;
/// prototype_validity_cell_ 偏移。
pub const K_PROTOTYPE_VALIDITY_CELL_OFFSET: usize = K_DEPENDENT_CODE_OFFSET + K_TAGGED_SIZE;
/// transitions_or_prototype_info_ 偏移 (最后一个字段)。
pub const K_TRANSITIONS_OR_PROTOTYPE_INFO_OFFSET: usize = K_PROTOTYPE_VALIDITY_CELL_OFFSET + K_TAGGED_SIZE;
/// 弱字段段末 (map.h:1219 kEndOfWeakFieldsOffset = sizeof(Map))。
pub const K_END_OF_WEAK_FIELDS_OFFSET: usize = K_TRANSITIONS_OR_PROTOTYPE_INFO_OFFSET + K_TAGGED_SIZE;
/// Map 头大小 (map.h:1220)。
pub const K_HEADER_SIZE: usize = K_END_OF_WEAK_FIELDS_OFFSET;
/// Map 自身大小 (map.h:1221)。
pub const K_SIZE: usize = K_END_OF_WEAK_FIELDS_OFFSET;

const _: () = assert!(K_INSTANCE_TYPE_OFFSET == K_TAGGED_SIZE + 4); // Internals 锚点 (原版 1*kApiTaggedSize + kApiInt32Size)
const _: () = assert!(K_SIZE == K_PROTOTYPE_OFFSET + 6 * K_TAGGED_SIZE); // 头 + 6 指针字段
const _: () = assert!(!TAGGED_SIZE_8_BYTES && K_SIZE == 40); // 压缩默认锚点

/// 变长对象实例尺寸哨兵 (words = 0)。
pub const K_VARIABLE_SIZE_SENTINEL_IN_WORDS: u8 = 0;

/// JSObject::kFieldsAdded (js-objects.h:982, 借位登记: JS 对象族 crate 落地后迁回)。
pub const JS_OBJECT_FIELDS_ADDED: i32 = 3;

/// 对象形状描述元对象 (Map)
///
/// 描述堆对象的实例类型、内存大小、位域标志及属性字典结构。
/// 字段序与原版一致 (map.h:1223-1246); tagged 指针字段以 Tagged<Object>
/// 逻辑占位, 具体类型窄化视图归后续对象族 crate。
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Map {
  /// 继承自 HeapObject，指向 Map 自身的元 Map
  pub header: HeapObject,
  /// 实例大小 (单位: word; 0 = kVariableSizeSentinel) (map.h:1223)
  pub instance_size_in_words: u8,
  /// JSObject map: 对象内属性起始 word 偏移; 原始类型 map: 构造函数在
  /// native context 中的序号 (map.h:1224)
  pub inobject_properties_start_or_constructor_function_index: u8,
  /// 见 used_or_unused_instance_size_in_words (map.h:1225; 双义编码, 详见
  /// unused_property_fields)
  pub used_or_unused_instance_size_in_words: u8,
  /// 堆访客 id (map.h:1226)
  pub visitor_id: u8,
  /// 实例类型 (map.h:1227; 原版存储为 u16)
  pub instance_type: InstanceType,
  /// 位域 1 (Bits1; map.h:1228)
  pub bit_field: u8,
  /// 位域 2 (Bits2; map.h:1229)
  pub bit_field2: u8,
  /// 位域 3 (Bits3; map.h:1230)
  pub bit_field3: u32,
  /// 隐式原型对象 (JSPrototype; map.h:1234)
  pub prototype: Tagged<Object>,
  /// 构造函数 / 回指针 / native context 三重叠槽 (map.h:1235)
  pub constructor_or_back_pointer_or_native_context: Tagged<Object>,
  /// 实例描述符数组 (DescriptorArray; map.h:1240)
  pub instance_descriptors: Tagged<Object>,
  /// 依赖代码表 (DependentCode; map.h:1241)
  pub dependent_code: Tagged<Object>,
  /// 原型链有效性 cell (UnionOf<Zero, Cell>; map.h:1243)
  pub prototype_validity_cell: Tagged<Object>,
  /// transitions / prototype_info 重叠槽 (map.h:1244; 类型派发归 v8-objects-maps)
  pub transitions_or_prototype_info: Tagged<Object>,
}

impl Map {
  // ── Bits1/2/3 位段常量 (关联承载; 见文件头部布局登记) ────────────────────
${bitsConsts("Bits1", "u8", BITS1)}
${bitsConsts("Bits2", "u8", BITS2)}
${bitsConsts("Bits3", "u32", BITS3)}

  /// 构造 Map 实例 (instance_size 单位: 字节, 须 word 对齐; 0 = 变长哨兵)
  #[inline]
  pub const fn new(
    meta_map: Tagged<Map>,
    instance_type: InstanceType,
    instance_size: usize,
  ) -> Self {
    debug_assert!(instance_size == 0 || instance_size.is_multiple_of(K_TAGGED_SIZE));
    Self {
      header: HeapObject::new(meta_map),
      instance_size_in_words: (instance_size >> K_TAGGED_SIZE_LOG2) as u8,
      inobject_properties_start_or_constructor_function_index: 0,
      used_or_unused_instance_size_in_words: 0,
      visitor_id: 0,
      instance_type,
      bit_field: 0,
      bit_field2: 0,
      bit_field3: 0,
      prototype: Tagged::NULL,
      constructor_or_back_pointer_or_native_context: Tagged::NULL,
      instance_descriptors: Tagged::NULL,
      dependent_code: Tagged::NULL,
      prototype_validity_cell: Tagged::NULL,
      transitions_or_prototype_info: Tagged::NULL,
    }
  }

  // ── 实例尺寸 (map-inl.h:400-419) ────────────────────────────────────────

  /// 实例大小 (单位: word)。
  #[inline]
  pub const fn instance_size_in_words(&self) -> u8 {
    self.instance_size_in_words
  }

  /// 设置实例大小 (单位: word)。
  #[inline]
  pub const fn set_instance_size_in_words(&mut self, value: u8) {
    self.instance_size_in_words = value;
  }

  /// 获取实例分配字节大小 (words << kTaggedSizeLog2; 0 = 变长哨兵)。
  #[inline]
  pub const fn instance_size(&self) -> usize {
    (self.instance_size_in_words as usize) << K_TAGGED_SIZE_LOG2
  }

  /// 设置实例分配字节大小 (须 word 对齐, map-inl.h:413 CHECK 对齐)。
  #[inline]
  pub const fn set_instance_size(&mut self, size_in_bytes: usize) {
    debug_assert!(size_in_bytes.is_multiple_of(K_TAGGED_SIZE));
    debug_assert!(size_in_bytes >> K_TAGGED_SIZE_LOG2 <= u8::MAX as usize);
    self.instance_size_in_words = (size_in_bytes >> K_TAGGED_SIZE_LOG2) as u8;
  }

  /// Map 自身占用大小 (非 extended 分支; = K_SIZE)。
  #[inline]
  pub const fn size(&self) -> usize {
    K_SIZE
  }

  /// Map 自身实际分配大小 (map-inl.h:495)。extended map 的尺寸经
  /// BitsEx 叠读, 视图归 v8-objects-maps — 此处落 Err 登记。
  #[inline]
  pub fn allocated_size(&self) -> crate::error::Result<usize> {
    if self.is_extended_map() {
      Err(crate::error::Error::Unsupported("AllocatedSize: extended map 尺寸归 v8-objects-maps"))
    } else {
      Ok(K_SIZE)
    }
  }

  /// 获取实例类型
  #[inline]
  pub const fn instance_type(&self) -> InstanceType {
    self.instance_type
  }

  /// 设置实例类型
  #[inline]
  pub fn set_instance_type(&mut self, instance_type: InstanceType) {
    self.instance_type = instance_type;
  }

  // ── visitor id (map-inl.h:391-398) ──────────────────────────────────────

  /// 堆访客 id (越界字节 → None; 原版 static_cast 无检查, 存入前经
  /// set_visitor_id 校验)。
  #[inline]
  #[must_use]
  pub const fn visitor_id(&self) -> Option<VisitorId> {
    visitor_id_from_u8(self.visitor_id)
  }

  /// 设置堆访客 id (原版 CHECK id < 256 → debug_assert)。
  #[inline]
  pub fn set_visitor_id(&mut self, id: VisitorId) {
    debug_assert!((id as u8 as usize) < 256);
    self.visitor_id = id as u8;
  }

  // ── inobject 属性 / 构造函数序号双义槽 (map-inl.h:421-483) ───────────────

  /// 双义槽原始字节。
  #[inline]
  pub const fn inobject_properties_start_or_constructor_function_index(&self) -> u8 {
    self.inobject_properties_start_or_constructor_function_index
  }

  /// 设置双义槽原始字节。
  #[inline]
  pub const fn set_inobject_properties_start_or_constructor_function_index(&mut self, value: u8) {
    self.inobject_properties_start_or_constructor_function_index = value;
  }

  /// 对象内属性区起始 word 偏移 (JSObject map; 原版 DCHECK IsJSObjectMap
  /// 由调用方保证)。
  #[inline]
  pub const fn get_in_object_properties_start_in_words(&self) -> u8 {
    self.inobject_properties_start_or_constructor_function_index()
  }

  /// 设置对象内属性区起始 word 偏移。
  #[inline]
  pub const fn set_in_object_properties_start_in_words(&mut self, value: u8) {
    self.set_inobject_properties_start_or_constructor_function_index(value);
  }

  /// 对象内属性数 (JSObject map; instance_size - start)。
  #[inline]
  pub const fn get_in_object_properties(&self) -> i32 {
    self.instance_size_in_words as i32 - self.get_in_object_properties_start_in_words() as i32
  }

  /// 字段序号是否落在对象内 (field_index < inobject 数)。
  #[inline]
  pub const fn is_field_in_object(&self, field_index: i32) -> bool {
    field_index < self.get_in_object_properties()
  }

  /// 对象内第 index 个属性的 byte 偏移 ((start + index) * kTaggedSize)。
  #[inline]
  pub const fn get_in_object_property_offset(&self, index: i32) -> i32 {
    (self.get_in_object_properties_start_in_words() as i32 + index) * K_TAGGED_SIZE as i32
  }

  /// 无对象包装器哨兵 (原始类型 map; map.h:284 kNoConstructorFunctionIndex)。
  pub const NO_CONSTRUCTOR_FUNCTION_INDEX: i32 = 0;

  /// 构造函数在 native context 中的序号 (原始类型 map; wasm 分支原版特判,
  /// 此处由调用方保证原始类型 map 前置)。
  #[inline]
  pub const fn get_constructor_function_index(&self) -> i32 {
    self.inobject_properties_start_or_constructor_function_index() as i32
  }

  /// 设置构造函数序号 (原始类型 map)。
  #[inline]
  pub const fn set_constructor_function_index(&mut self, value: i32) {
    debug_assert!(value >= 0 && value <= u8::MAX as i32);
    self.set_inobject_properties_start_or_constructor_function_index(value as u8);
  }

  // ── used/unused 记账 (map-inl.h:507-632) ────────────────────────────────
  // 该字节编码双义: 值 >= JSObject::kFieldsAdded(3) 时为 "已用到的 word 偏移"
  // (unused = instance_size - value, 全部属性在对象内); 否则为属性数组中的
  // slack 槽位数 (unused = value)。

  /// 双义记账字节原始值。
  #[inline]
  pub const fn used_or_unused_instance_size_in_words(&self) -> u8 {
    self.used_or_unused_instance_size_in_words
  }

  /// 设置双义记账字节原始值 (原版 CHECK <= 255 → u8 天然满足)。
  #[inline]
  pub const fn set_used_or_unused_instance_size_in_words(&mut self, value: u8) {
    self.used_or_unused_instance_size_in_words = value;
  }

  /// 可用未用属性槽数 (对象内或属性数组; map-inl.h:507)。
  #[inline]
  pub const fn unused_property_fields(&self) -> i32 {
    let value = self.used_or_unused_instance_size_in_words() as i32;
    if value >= JS_OBJECT_FIELDS_ADDED {
      self.instance_size_in_words() as i32 - value
    } else {
      value
    }
  }

  /// 对象内未用属性槽数 (map-inl.h:524; 外置属性时恒 0)。
  #[inline]
  pub const fn unused_in_object_properties(&self) -> i32 {
    let value = self.used_or_unused_instance_size_in_words() as i32;
    if value >= JS_OBJECT_FIELDS_ADDED {
      self.instance_size_in_words() as i32 - value
    } else {
      0
    }
  }

  /// 是否有外置属性 (map-inl.h:448)。
  #[inline]
  pub const fn has_out_of_object_properties(&self) -> bool {
    (self.used_or_unused_instance_size_in_words() as i32) < JS_OBJECT_FIELDS_ADDED
  }

  /// 已用实例大小 (含对象头; map-inl.h:548)。
  #[inline]
  pub const fn used_instance_size(&self) -> usize {
    let words = self.used_or_unused_instance_size_in_words() as i32;
    if words < JS_OBJECT_FIELDS_ADDED {
      // 对象内属性全用完, 字节在计属性数组 slack → 全实例大小。
      return self.instance_size();
    }
    (words as usize) << K_TAGGED_SIZE_LOG2
  }

  /// 设置对象内未用槽数 (map-inl.h:561; 非 JSObject map 须为 0)。
  #[inline]
  pub const fn set_in_object_unused_property_fields(&mut self, value: i32) {
    debug_assert!(value >= 0);
    let used_inobject_properties = self.get_in_object_properties() - value;
    self.set_used_or_unused_instance_size_in_words(
      (self.get_in_object_property_offset(used_inobject_properties) / K_TAGGED_SIZE as i32) as u8,
    );
    debug_assert!(self.unused_property_fields() == value);
  }

  /// 设置属性数组未用槽数 (map-inl.h:576; value < kFieldsAdded)。
  #[inline]
  pub const fn set_out_of_object_unused_property_fields(&mut self, value: i32) {
    debug_assert!(value < JS_OBJECT_FIELDS_ADDED && value >= 0);
    self.set_used_or_unused_instance_size_in_words(value as u8);
    debug_assert!(self.unused_property_fields() == value);
  }

  /// 复制未用槽数记账 (map-inl.h:585)。
  #[inline]
  pub const fn copy_unused_property_fields(&mut self, map: &Map) {
    self.set_used_or_unused_instance_size_in_words(map.used_or_unused_instance_size_in_words());
  }

  /// 复制未用槽数记账并按实例大小差校正 (map-inl.h:591)。
  #[inline]
  pub const fn copy_unused_property_fields_adjusted_for_instance_size(&mut self, map: &Map) {
    let mut value = map.used_or_unused_instance_size_in_words() as i32;
    if value >= JS_OBJECT_FIELDS_ADDED {
      // 对象内未用字段: 校正偏移使其匹配到对象末端的距离。
      value += self.instance_size_in_words() as i32 - map.instance_size_in_words() as i32;
    }
    self.set_used_or_unused_instance_size_in_words(value as u8);
  }

  /// 记账一次新增属性字段 (map-inl.h:602)。
  #[inline]
  pub const fn account_added_property_field(&mut self) {
    let value = self.used_or_unused_instance_size_in_words() as i32;
    if value >= JS_OBJECT_FIELDS_ADDED {
      if value == self.instance_size_in_words() as i32 {
        // 对象内已满 → 转外置, 属性数组 slack 从 0 起算再扣 1。
        self.account_added_out_of_object_property_field(0);
      } else {
        // 属性加在对象内, 计数 +1。
        self.set_used_or_unused_instance_size_in_words((value + 1) as u8);
      }
    } else {
      self.account_added_out_of_object_property_field(value);
    }
  }

  /// 记账一次新增外置属性字段 (map-inl.h:623)。
  #[inline]
  pub const fn account_added_out_of_object_property_field(&mut self, unused_in_property_array: i32) {
    let mut unused = unused_in_property_array - 1;
    if unused < 0 {
      unused += JS_OBJECT_FIELDS_ADDED;
    }
    debug_assert!(unused < JS_OBJECT_FIELDS_ADDED && unused >= 0);
    self.set_used_or_unused_instance_size_in_words(unused as u8);
  }

  // ── slack tracking (map.h:443-493; map-inl.h:1247; map.cc:2662) ─────────

  /// slack tracking 是否进行中 (counter != kNoSlackTracking)。
  #[inline]
  pub const fn is_inobject_slack_tracking_in_progress(&self) -> bool {
    self.construction_counter() != NO_SLACK_TRACKING
  }

  /// 启动 slack tracking: 计数器置 kSlackTrackingCounterStart
  /// (map.cc:2662; 无未用槽则不启动)。
  #[inline]
  pub fn start_inobject_slack_tracking(&mut self) {
    debug_assert!(!self.is_inobject_slack_tracking_in_progress());
    if self.unused_property_fields() == 0 {
      return;
    }
    self.set_construction_counter(SLACK_TRACKING_COUNTER_START);
  }

  /// 按数组尺寸取宽裕 slack (map-inl.h:1263 SlackForArraySize)。
  #[inline]
  #[must_use]
  pub const fn slack_for_array_size(old_size: i32, size_limit: i32) -> i32 {
    let max_slack = size_limit - old_size;
    debug_assert!(max_slack >= 0);
    if old_size < 4 {
      debug_assert!(max_slack >= 1);
      return 1;
    }
    if max_slack < old_size / 4 {
      max_slack
    } else {
      old_size / 4
    }
  }

  /// 按 slack 折减后的实例大小 (map-inl.h:1273)。
  #[inline]
  #[must_use]
  pub const fn instance_size_from_slack(&self, slack: i32) -> i32 {
    self.instance_size() as i32 - slack * K_TAGGED_SIZE as i32
  }

  // ── 布尔语义助手 ─────────────────────────────────────────────────────────

  /// 是否被弃用的原型 map (is_prototype_map && !owns_descriptors;
  /// map-inl.h:708)。
  #[inline]
  pub const fn is_abandoned_prototype_map(&self) -> bool {
    self.is_prototype_map() && !self.owns_descriptors()
  }

  /// 是否稳定 (非 unstable; map-inl.h:882)。
  #[inline]
  pub const fn is_stable(&self) -> bool {
    !self.is_unstable()
  }

  /// 标记为不稳定 (map-inl.h:877 mark_unstable)。
  #[inline]
  pub fn mark_unstable(&mut self) {
    self.set_is_unstable(true);
  }

  /// 是否可参与 transition (仅 JSObject 及其子类型;
  /// 是否可参与 transition (map-inl.h:906-918 返回式三谓词合取):
  /// 仅 JSObject 及其子类型, 且排除只读包装对象 (JSExternalObject/JSMessageObject)
  /// 与共享空间固定形状对象 (JSSharedArray/JSSharedStruct)。
  /// 原版两条 DCHECK_IMPLIES 依赖 HeapLayout 空间位置 (推迟), 但返回式纯 instance_type。
  #[inline]
  pub const fn can_transition(&self) -> bool {
    let ty = self.instance_type;
    crate::instance_type_checker::is_js_object(ty)
      && !crate::instance_type_checker::is_maybe_read_only_js_object(ty)
      && !crate::instance_type_checker::is_always_shared_space_js_object(ty)
  }

  /// 该实例类型可否持有可 transition 的快 elements kind
  /// (map-inl.h:275: JS_ARRAY / JS_PRIMITIVE_WRAPPER / JS_ARGUMENTS_OBJECT)。
  #[inline]
  #[must_use]
  pub const fn can_have_fast_transitionable_elements_kind_of(instance_type: InstanceType) -> bool {
    matches!(
      instance_type,
      InstanceType::JSArray | InstanceType::JSPrimitiveWrapper | InstanceType::JSArgumentsObject
    )
  }

  /// 本 map 实例类型可否持有可 transition 的快 elements kind。
  #[inline]
  pub const fn can_have_fast_transitionable_elements_kind(&self) -> bool {
    Self::can_have_fast_transitionable_elements_kind_of(self.instance_type())
  }

  // ── Bits1 访问器 (机械展开) ──────────────────────────────────────────────
${bits1Accs}

  // ── Bits2 访问器 (机械展开) ──────────────────────────────────────────────
${bits2BoolAccs}

  /// elements kind 原始值 (6 bit; 类型化 ElementsKind 枚举与谓词族归
  /// elements crate, 此处保留 raw 存取)。
  #[inline]
  pub const fn elements_kind(&self) -> u8 {
    (self.bit_field2 & Self::ELEMENTS_KIND_MASK) >> Self::ELEMENTS_KIND_SHIFT
  }

  /// 设置 elements kind 原始值 (须 < 64 = kElementsKindCount 上界)。
  #[inline]
  pub const fn set_elements_kind(&mut self, kind: u8) {
    debug_assert!(kind <= Self::ELEMENTS_KIND_MASK >> Self::ELEMENTS_KIND_SHIFT);
    self.bit_field2 =
      (self.bit_field2 & !Self::ELEMENTS_KIND_MASK) | (kind << Self::ELEMENTS_KIND_SHIFT);
  }

  // ── Bits3 访问器 (机械展开; bool 位) ────────────────────────────────────
${bits3BoolAccs}

  /// enum cache 长度 (bit 0..9; kInvalidEnumCacheSentinel=1023 表示无效)。
  #[inline]
  pub const fn enum_length(&self) -> u32 {
    self.bit_field3 & Self::ENUM_LENGTH_MASK
  }

  /// 设置 enum cache 长度 (map-inl.h:366; 哨兵免检, 其余须 <= own descriptors)。
  #[inline]
  pub const fn set_enum_length(&mut self, length: u32) {
    if length != crate::property_details::INVALID_ENUM_CACHE_SENTINEL {
      debug_assert!(length <= Self::ENUM_LENGTH_MASK);
    }
    self.bit_field3 = (self.bit_field3 & !Self::ENUM_LENGTH_MASK) | (length & Self::ENUM_LENGTH_MASK);
  }

  /// 自有描述符数 (bit 10..19; map-inl.h:345)。
  #[inline]
  pub const fn number_of_own_descriptors(&self) -> u32 {
    (self.bit_field3 & Self::NUMBER_OF_OWN_DESCRIPTORS_MASK) >> Self::NUMBER_OF_OWN_DESCRIPTORS_SHIFT
  }

  /// 设置自有描述符数 (map-inl.h:350; 上界 kMaxNumberOfDescriptors)。
  #[inline]
  pub const fn set_number_of_own_descriptors(&mut self, count: u32) {
    debug_assert!(count <= crate::property_details::MAX_NUMBER_OF_DESCRIPTORS);
    self.bit_field3 = (self.bit_field3 & !Self::NUMBER_OF_OWN_DESCRIPTORS_MASK)
      | ((count << Self::NUMBER_OF_OWN_DESCRIPTORS_SHIFT) & Self::NUMBER_OF_OWN_DESCRIPTORS_MASK);
  }

  /// inobject slack tracking 构造计数器 (bit 29..31)。
  #[inline]
  pub const fn construction_counter(&self) -> u32 {
    (self.bit_field3 & Self::CONSTRUCTION_COUNTER_MASK) >> Self::CONSTRUCTION_COUNTER_SHIFT
  }

  /// 设置构造计数器 (<= 7 = ConstructionCounterBits::kMax)。
  #[inline]
  pub const fn set_construction_counter(&mut self, value: u32) {
    debug_assert!(value <= Self::CONSTRUCTION_COUNTER_MASK >> Self::CONSTRUCTION_COUNTER_SHIFT);
    self.bit_field3 = (self.bit_field3 & !Self::CONSTRUCTION_COUNTER_MASK)
      | ((value << Self::CONSTRUCTION_COUNTER_SHIFT) & Self::CONSTRUCTION_COUNTER_MASK);
  }

  /// 是否字典 mode map (map-inl.h:873)。
  #[inline]
  pub const fn is_dictionary_map(&self) -> bool {
    (self.bit_field3 & Self::IS_DICTIONARY_MAP_MASK) != 0
  }

  /// 设置字典 map 标志 (map-inl.h:866: is_dictionary_map 与 is_unstable
  /// 联动置位/清除 — 原版语义, 勿拆)。
  #[inline]
  pub fn set_is_dictionary_map(&mut self, value: bool) {
    self.bit_field3 = (self.bit_field3 & !Self::IS_DICTIONARY_MAP_MASK) | ((value as u32) << Self::IS_DICTIONARY_MAP_SHIFT);
    self.set_is_unstable(value);
  }
}

/// visitor_id 字节 → VisitorId (u8 全域可解码, 越界 → None)。
#[inline]
#[must_use]
pub const fn visitor_id_from_u8(raw: u8) -> Option<VisitorId> {
  if raw >= VisitorId::VisitorIdCount as u8 {
    return None;
  }
  // SAFETY: VisitorId 为 #[repr(u8)] 且判别值自 0 连续 (枚举体由机械展开
  // 生成, 无跳变), raw < VisitorIdCount → 转换落在合法判别值域内。
  Some(unsafe { std::mem::transmute::<u8, VisitorId>(raw) })
}

// ── ExtendedMap (map.h:1264-1297) ────────────────────────────────────────
/// 带扩展字段的 Map 基类 (子类定义 @hasSameInstanceTypeAsParent 并补 padding)。
///
/// bit_field_ex 紧随 Map 主体 (逻辑承载: map_kind 3 bit + map_size_in_words
/// 5 bit); Rust 侧以独立结构体 + 原始位承载, 视图叠读归 v8-objects-maps。
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ExtendedMap {
  /// Map 主体。
  pub map: Map,
  /// 扩展位域 (BitsEx)。
  pub bit_field_ex: u8,
  /// 预留 padding 至 kTaggedSize (子类字段区)。
  pub padding: [u8; 3],
}

impl ExtendedMap {
  /// ExtendedMap 最小尺寸 (map.h:1292-1296: RoundUp(sizeof, kTaggedSize))。
  pub const K_MINIMUM_SIZE: usize = K_SIZE + K_TAGGED_SIZE;
  /// 强扩展字段起始偏移 (map.h:1295)。
  pub const K_START_OF_STRONG_EXTENDED_FIELDS_OFFSET: usize = Self::K_MINIMUM_SIZE;

  /// bit_field_ex 原始值。
  #[inline]
  pub const fn bit_field_ex(&self) -> u8 {
    self.bit_field_ex
  }

  /// 设置 bit_field_ex 原始值。
  #[inline]
  pub const fn set_bit_field_ex(&mut self, value: u8) {
    self.bit_field_ex = value;
  }

  /// 扩展 map 类别 (BitsEx::MapKindBits; 3 bit 仅定义 0, 其余为腐化值)。
  #[inline]
  pub const fn map_kind(&self) -> ExtendedMapKind {
    let bits = self.bit_field_ex & MAP_KIND_MASK;
    debug_assert!(bits == 0, "map_kind 腐化值");
    ExtendedMapKind::JSInterceptorMap
  }

  /// 扩展 map 尺寸 (单位: word; BitsEx::MapSizeInWordsBits)。
  #[inline]
  pub const fn map_size_in_words(&self) -> u8 {
    (self.bit_field_ex & MAP_SIZE_IN_WORDS_MASK) >> MAP_SIZE_IN_WORDS_SHIFT
  }

  /// 扩展 map 尺寸 (字节; map-inl.h:1313)。
  #[inline]
  pub const fn map_size(&self) -> usize {
    (self.map_size_in_words() as usize) << K_TAGGED_SIZE_LOG2
  }

  /// 同时设置类别与尺寸 (map-inl.h:1317; 尺寸须 word 对齐且 <= 255 words)。
  #[inline]
  pub const fn set_map_kind_and_size(&mut self, kind: ExtendedMapKind, size_in_bytes: usize) {
    debug_assert!(size_in_bytes.is_multiple_of(K_TAGGED_SIZE));
    let size_in_words = size_in_bytes >> K_TAGGED_SIZE_LOG2;
    debug_assert!(size_in_words <= u8::MAX as usize);
    self.bit_field_ex = ((kind as u8) << MAP_KIND_SHIFT)
      | ((size_in_words as u8) << MAP_SIZE_IN_WORDS_SHIFT);
  }
}

/// 由扩展类别取尺寸 (map-inl.h:1277 ExtendedMapSizeForKind =
/// sizeof(JSInterceptorMap))。JSInterceptorMap 的 packed 布局
/// (ExtendedMap + flags/padding + 3 tagged 字段) 归 v8-objects-maps
/// 确认后承接 — 此处落 Err 登记。
#[inline]
pub fn extended_map_size_for_kind(kind: ExtendedMapKind) -> crate::error::Result<usize> {
  match kind {
    ExtendedMapKind::JSInterceptorMap => Err(crate::error::Error::Unsupported(
      "ExtendedMapSizeForKind: JSInterceptorMap 布局归 v8-objects-maps",
    )),
  }
}

// == map.cc 语义区段 (由 tool/src/objects/map.cc.js 维护) ==
// @cc-semantics-begin
// @cc-semantics-end
`;

writeSrc("v8-objects-core", "map.rs", body);
console.log(
  `map.rs: VisitorId ${dataOnly.length}+${pointer.length}+${trusted.length} 变体 (data-only 锚点 24) + ` +
    `Bits1/2/3/Ex 位段 ${BITS1.length + BITS2.length + BITS3.length + BITS_EX.length} 组 + ` +
    "布局偏移常量 + inobject 记账/slack 族 + ExtendedMap; transition 深层推迟已登记",
);
