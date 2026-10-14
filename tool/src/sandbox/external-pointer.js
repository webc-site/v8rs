#!/usr/bin/env bun
// tool/src/sandbox/external-pointer.js — 外部指针 tag 表 / Handle 语义 / payload 编解码生成
// 组→文件映射 (本脚本管辖):
//   v8/src/sandbox/external-pointer.h + external-pointer-inl.h
//   + include/v8-internal.h (ExternalPointerTag 枚举 + TagRange + 范围谓词, 本版无独立 external-pointer-tag.h, tag 定义在 v8-internal.h)
//   + v8/src/sandbox/tagged-payload.h (TaggedPayload 编解码位运算, 并入; 泛型 scheme 层待表族落地再抽)
//   + v8/src/sandbox/external-pointer-table{.h,-inl.h} (PayloadTaggingScheme + HandleToIndex/IndexToHandle 位运算段)
//   → v8-sandbox/src/external_pointer.rs
// 规则:
//   - 枚举隐式递增 / 显式赋值 / X-macro (FOREIGN_TAG_LIST) 展开, 逐 tag 计算值
//   - 显式锚点 (0x7d/0x7e/0x7f) 与派生关系断言
//   - payload 位运算 1:1 直出 (Tag/CheckTag/Untag/markbit/freelist)
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/sandbox/external-pointer.h");
const inl = readV8("src/sandbox/external-pointer-inl.h");
const internal = readV8("include/v8-internal.h");
const payload = readV8("src/sandbox/tagged-payload.h");
const eptH = readV8("src/sandbox/external-pointer-table.h");
const eptInl = readV8("src/sandbox/external-pointer-table-inl.h");

// ---- 宏列表解析: 取 #define NAME(V) 起到下一个行首 #define 之间, 收集 V(tag) ----
function parseTagList(name) {
  const all = internal.split("\n");
  const start = all.findIndex((l) => l.startsWith(`#define ${name}(V)`));
  if (start < 0) throw new Error(`external-pointer: 宏列表 ${name} 未找到`);
  const items = [];
  // #define 行自身可能带 V(tag) (单行宏)
  const defLine = all[start].slice(`#define ${name}(V)`.length);
  for (const m of defLine.matchAll(/V\((\w+)\)/g)) items.push(m[1]);
  for (let i = start + 1; i < all.length; i++) {
    const line = all[i];
    if (line.startsWith("#")) break; // 下一个宏定义
    if (line.startsWith("//")) continue;
    // 嵌套宏引用 (如 MANAGED_TAG_LIST 内的 SHARED_MANAGED_TAG_LIST(V)): 递归展开
    const nested = line.replace(/\\\s*$/, "").trim().match(/^([A-Z_]+_TAG_LIST)\(V\)$/);
    if (nested) {
      if (nested[1] === name) throw new Error(`external-pointer: 宏 ${name} 自引用`);
      items.push(...parseTagList(nested[1]));
      continue;
    }
    for (const m of line.matchAll(/V\((\w+)\)/g)) items.push(m[1]);
  }
  if (items.length === 0) throw new Error(`external-pointer: 宏列表 ${name} 为空`);
  return items;
}
const SHARED_MANAGED = parseTagList("SHARED_MANAGED_TAG_LIST");
const MANAGED = parseTagList("MANAGED_TAG_LIST");
const FOREIGN = parseTagList("FOREIGN_TAG_LIST");
// FOREIGN = 10 foreign 前缀 + MANAGED (共享 1 + ICU 9)
if (FOREIGN.length !== 10 + MANAGED.length || MANAGED.length !== 1 + 9)
  throw new Error(`external-pointer: 列表长度异常 FOREIGN=${FOREIGN.length} MANAGED=${MANAGED.length} (期望 20/10)`);
if (MANAGED[0] !== SHARED_MANAGED[0] || MANAGED.length !== SHARED_MANAGED.length + 9)
  throw new Error("external-pointer: MANAGED 应以 SHARED_MANAGED (1 项) 开头 + 9 ICU");

// ---- 枚举体提取与 X-macro 展开 ----
const enumMatch = internal.match(/enum ExternalPointerTag : uint16_t \{([\s\S]*?)\n\};/);
if (!enumMatch) throw new Error("external-pointer: ExternalPointerTag 枚举未找到");
let body = enumMatch[1]
  .replace(
    /#define AS_ENUM\(name\) k##name,\s*\n\s*FOREIGN_TAG_LIST\(AS_ENUM\)\s*\n\s*#undef AS_ENUM/,
    FOREIGN.map((t) => `k${t},`).join("\n"),
  )
  .replace(/\/\*[\s\S]*?\*\//g, "")
  .replace(/\/\/[^\n]*/g, "");
if (body.includes("FOREIGN_TAG_LIST") || body.includes("#")) throw new Error("external-pointer: 枚举内 X-macro 展开不完整");

// ---- 逐项求值 ----
const counts = {
  V8_EMBEDDER_DATA_TAG_COUNT: Number(internal.match(/#define V8_EMBEDDER_DATA_TAG_COUNT (\d+)/)[1]),
  V8_EXTERNAL_POINTER_TAG_COUNT: Number(internal.match(/#define V8_EXTERNAL_POINTER_TAG_COUNT (\d+)/)[1]),
};
const vals = new Map();
let cur = -1;
const entries = [];
for (const raw of body.split(",")) {
  const item = raw.trim();
  if (!item) continue;
  const m = item.match(/^k(\w+)(?:\s*=\s*(.+))?$/);
  if (!m) throw new Error(`external-pointer: 无法解析枚举项: ${item}`);
  const [, nameWoK, rhs] = m;
  const name = `k${nameWoK}`;
  let v;
  if (rhs === undefined) {
    v = ++cur;
  } else {
    let e = rhs.trim();
    for (const [cn, cv] of Object.entries(counts)) e = e.split(cn).join(`(${cv})`);
    for (const [pn, pv] of vals) e = e.split(pn).join(`(${pv})`);
    // 0x 十六进制字面量 → 十进制
    e = e.replace(/0x[0-9a-fA-F]+/g, (x) => String(Number(x)));
    if (!/^[\d\s()+\-*/]+$/.test(e)) throw new Error(`external-pointer: 无法求值 ${name} = ${rhs}`);
    v = Function(`"use strict"; return (${e});`)();
  }
  cur = v;
  vals.set(name, v);
  entries.push({ name, value: v });
}

// ---- 对数核对 ----
const checks = [];
function check(desc, ok) {
  if (!ok) throw new Error(`external-pointer: 对数核对失败: ${desc}`);
  checks.push(desc);
}
check("kExternalPointerNullTag == 0", vals.get("kExternalPointerNullTag") === 0);
check("kWaiterQueueNodeTag == kFirstSharedExternalPointerTag", vals.get("kWaiterQueueNodeTag") === vals.get("kFirstSharedExternalPointerTag"));
check("kLastSharedExternalPointerTag == kExternalStringResourceDataTag", vals.get("kLastSharedExternalPointerTag") === vals.get("kExternalStringResourceDataTag"));
check("kLastEmbedderDataTag - kFirstEmbedderDataTag == 15-1", vals.get("kLastEmbedderDataTag") - vals.get("kFirstEmbedderDataTag") === 14);
check("kLastExternalTypeTag - kFirstExternalTypeTag == 40-1", vals.get("kLastExternalTypeTag") - vals.get("kFirstExternalTypeTag") === 39);
check("FOREIGN 首 tag 紧跟 kWasmStackMemoryTag", vals.get(`k${FOREIGN[0]}`) === vals.get("kWasmStackMemoryTag") + 1);
check("FOREIGN 尾 tag 紧邻 kArrayBufferExtensionTag", vals.get(`k${FOREIGN.at(-1)}`) === vals.get("kArrayBufferExtensionTag") - 1);
check("kLastManagedResourceTag == kArrayBufferExtensionTag", vals.get("kLastManagedResourceTag") === vals.get("kArrayBufferExtensionTag"));
check("kExternalPointerZappedEntryTag == 0x7d", vals.get("kExternalPointerZappedEntryTag") === 0x7d);
check("kExternalPointerEvacuationEntryTag == 0x7e", vals.get("kExternalPointerEvacuationEntryTag") === 0x7e);
check("kExternalPointerFreeEntryTag == 0x7f", vals.get("kExternalPointerFreeEntryTag") === 0x7f);
check("kLastExternalPointerTag == 0x7f", vals.get("kLastExternalPointerTag") === 0x7f);
check("tag 全部 <= 0x7f (7 位)", [...vals.values()].every((v) => v <= 0x7f));
check("InterceptorInfo 范围 16 个 tag", vals.get("kLastInterceptorInfoExternalPointerTag") - vals.get("kFirstInterceptorInfoExternalPointerTag") === 15);

// tagged-payload.h 位运算核对
if (!payload.includes("(pointer << kPayloadShift) |")) throw new Error("external-pointer: Tag 公式异常");
if (!payload.includes("(content & kPayloadMask) >> kPayloadShift")) throw new Error("external-pointer: Untag 公式异常");
if (!payload.includes("(content & kTagMask) >> kTagShift")) throw new Error("external-pointer: CheckTag 公式异常");
if (!payload.includes("encoded_word_ |= kMarkBit;")) throw new Error("external-pointer: markbit 置位异常");
// ExternalPointerTablePayloadTaggingScheme (external-pointer-table.h:110-125)
const scheme = eptH.match(/struct ExternalPointerTablePayloadTaggingScheme \{([\s\S]*?)\};/);
if (!scheme) throw new Error("external-pointer: PayloadTaggingScheme 未找到");
for (const [k, v] of [
  ["kPayloadShift", "0"],
  ["kTagShift", "kExternalPointerTagShift"],
  ["kMarkBit", "kExternalPointerMarkBit"],
  ["kTagMask", "kExternalPointerTagMask"],
  ["kPayloadMask", "kExternalPointerPayloadMask"],
  ["kFreeEntryTag", "kExternalPointerFreeEntryTag"],
  ["kEvacuationEntryTag", "kExternalPointerEvacuationEntryTag"],
]) {
  if (!new RegExp(`${k}\\s*=\\s*${v}`).test(scheme[1])) throw new Error(`external-pointer: scheme ${k} != ${v}`);
}
// Handle 移位语义 (external-pointer-table-inl.h:349-381)
if (!eptInl.includes("uint32_t index = handle >> kExternalPointerIndexShift;"))
  throw new Error("external-pointer: HandleToIndex 公式异常");
if (!eptInl.includes("ExternalPointerHandle handle = index << kExternalPointerIndexShift;"))
  throw new Error("external-pointer: IndexToHandle 公式异常");
if (!/handle == index << kExternalPointerIndexShift/.test(eptInl))
  throw new Error("external-pointer: IsValidHandle 公式异常");
for (const sym of ["ExternalPointerMember", "InitExternalPointerField", "ReadExternalPointerField", "WriteExternalPointerField"]) {
  if (!h.includes(sym)) throw new Error(`external-pointer: ${sym} 缺失`);
}

// ---- 变体/别名划分 ----
// kFirst*/kLast* 为范围标记 → 优先 const 别名; 但若该值无其他非标记名, 则标记本身作变体
const nonMarkerByValue = new Map();
for (const { name, value } of entries) {
  if (/^k(First|Last)/.test(name)) continue;
  if (nonMarkerByValue.has(value)) throw new Error(`external-pointer: 非标记值重复 ${name} vs ${nonMarkerByValue.get(value)}`);
  nonMarkerByValue.set(value, name);
}
const variants = [];
const aliases = [];
const byValue = new Map();
for (const { name, value } of entries) {
  const isMarker = /^k(First|Last)/.test(name);
  if (isMarker && nonMarkerByValue.has(value)) {
    aliases.push({ name, value });
    continue;
  }
  if (!byValue.has(value)) byValue.set(value, name);
  if (!variants.some((v) => v.name === name)) variants.push({ name, value });
}
// 每个枚举值必有唯一变体
for (const v of new Set(entries.map((e) => e.value))) {
  if (!byValue.has(v)) throw new Error(`external-pointer: 值 ${v} 无变体承载`);
}

const variantLine = (v) => `  ${v.name.slice(1)} = ${v.value},`;
// kFirstSharedExternalPointerTag → K_FIRST_SHARED_EXTERNAL_POINTER_TAG
const scream = (name) =>
  "K_" +
  name
    .slice(1)
    .replace(/([a-z0-9])([A-Z])/g, "$1_$2")
    .replace(/([A-Z]+)([A-Z][a-z])/g, "$1_$2")
    .toUpperCase();
const aliasLine = (a) => `pub const ${scream(a.name)}: ExternalPointerTag = ExternalPointerTag::${byValue.get(a.value).slice(1)};`;
const v = (name) => `ExternalPointerTag::${byValue.get(vals.get(name)).slice(1)}`;

const rangeConsts = [
  `pub const K_ANY_EXTERNAL_POINTER_TAG_RANGE: ExternalPointerTagRange = ExternalPointerTagRange::new(${v("kFirstExternalPointerTag")}, ${v("kLastExternalPointerTag")});`,
  `pub const K_ANY_FOREIGN_EXTERNAL_POINTER_TAG_RANGE: ExternalPointerTagRange = ExternalPointerTagRange::new(${v(`k${FOREIGN[0]}`)}, ${v(`k${FOREIGN.at(-1)}`)});`,
  `pub const K_ANY_INTERCEPTOR_INFO_EXTERNAL_POINTER_TAG_RANGE: ExternalPointerTagRange = ExternalPointerTagRange::new(${v("kFirstInterceptorInfoExternalPointerTag")}, ${v("kLastInterceptorInfoExternalPointerTag")});`,
  `pub const K_ANY_MANAGED_EXTERNAL_POINTER_TAG_RANGE: ExternalPointerTagRange = ExternalPointerTagRange::new(${v(`k${MANAGED[0]}`)}, ${v(`k${MANAGED.at(-1)}`)});`,
  `pub const K_ANY_MAYBE_READ_ONLY_EXTERNAL_POINTER_TAG_RANGE: ExternalPointerTagRange = ExternalPointerTagRange::new(${v("kFirstMaybeReadOnlyExternalPointerTag")}, ${v("kLastMaybeReadOnlyExternalPointerTag")});`,
  `pub const K_ANY_MANAGED_RESOURCE_EXTERNAL_POINTER_TAG_RANGE: ExternalPointerTagRange = ExternalPointerTagRange::new(${v(`k${MANAGED[0]}`)}, ${v("kLastManagedResourceTag")});`,
  `pub const K_ANY_SHARED_MANAGED_EXTERNAL_POINTER_TAG_RANGE: ExternalPointerTagRange = ExternalPointerTagRange::new(${v(`k${SHARED_MANAGED[0]}`)}, ${v(`k${SHARED_MANAGED.at(-1)}`)});`,
];

const body2 = `${banner("src/sandbox/external-pointer.js", "src/sandbox/external-pointer{,-inl}.h + include/v8-internal.h(tag 段) + src/sandbox/tagged-payload.h(并入) + external-pointer-table{.h,-inl}.h(scheme/handle 位运算段)", "外部指针: tag 枚举/范围谓词 + Handle 移位语义 + payload 编解码位运算 1:1")}
//! 外部指针编码层。
//! 本版 V8 无独立 external-pointer-tag.h: tag 定义在 include/v8-internal.h, 由本模块承载。
//! tagged-payload.h 的 TaggedPayload 位运算以 ExternalPointerTable scheme 实例化并入
//! (泛型 scheme 层待 code-pointer/trusted-pointer 表族第二梯队落地再抽)。
//! Isolate 绑定 (InitExternalPointerField 等走 ExternalPointerTable 的接口) 登记第二梯队。
//! 生成脚本内置对数核对 ${checks.length} 条全部通过。

use crate::internal::{
  Address, K_EXTERNAL_POINTER_INDEX_SHIFT, K_EXTERNAL_POINTER_MARK_BIT, K_EXTERNAL_POINTER_PAYLOAD_MASK,
  K_EXTERNAL_POINTER_TAG_MASK, K_EXTERNAL_POINTER_TAG_SHIFT, K_NULL_ADDRESS,
};

// ---- ExternalPointerTag (v8-internal.h:692-771, 全 tag 逐值计算) ----
/// 7 位 tag (0..=0x7f); mark 位占 1 位, 两者合占 64 位字的高 15 位
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::FromRepr)]
pub enum ExternalPointerTag {
${variants.map(variantLine).join("\n")}
}

// ---- 范围标记别名 (原版枚举 kFirst*/kLast* 成员, 值与对应变体相同) ----
${aliases.map(aliasLine).join("\n")}

// ---- ExternalPointerTagRange (v8-internal.h:535-595 TagRange) ----
/// 闭区间 [first, last]; u32 提升防 u16 减法下溢 (原版注释语义)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExternalPointerTagRange {
  pub first: ExternalPointerTag,
  pub last: ExternalPointerTag,
}

impl ExternalPointerTagRange {
  /// 原版 TagRange(first, last)
  pub const fn new(first: ExternalPointerTag, last: ExternalPointerTag) -> Self { Self { first, last } }
  /// 原版 TagRange(tag): 单 tag 隐式转范围
  pub const fn of(tag: ExternalPointerTag) -> Self { Self { first: tag, last: tag } }
  /// 仅含 null tag 视为空
  pub const fn is_empty(&self) -> bool {
    self.first == ExternalPointerTag::ExternalPointerNullTag && self.last == ExternalPointerTag::ExternalPointerNullTag
  }
  pub const fn size(&self) -> u32 {
    if self.is_empty() { 0 } else { (self.last as u32) - (self.first as u32) + 1 }
  }
  pub const fn contains(&self, tag: ExternalPointerTag) -> bool {
    (tag as u32).wrapping_sub(self.first as u32) <= (self.last as u32).wrapping_sub(self.first as u32)
  }
  pub const fn contains_range(&self, r: Self) -> bool {
    (r.first as u32) >= (self.first as u32) && (r.last as u32) <= (self.last as u32)
  }
}

// ---- 预定义范围 (v8-internal.h:792-841, X-macro GET_FIRST/GET_LAST 展开) ----
${rangeConsts.join("\n")}

/// 原版 IsSharedExternalPointerType: shared 段 (WaiterQueueNode..ExternalStringResourceData)
/// 或 shared managed 段 → 走共享 Isolate 的外部指针表
pub const fn is_shared_external_pointer_type(tag_range: ExternalPointerTagRange) -> bool {
  const SHARED: ExternalPointerTagRange = ExternalPointerTagRange::new(
    ExternalPointerTag::WaiterQueueNodeTag,
    ExternalPointerTag::ExternalStringResourceDataTag,
  );
  SHARED.contains_range(tag_range) || K_ANY_SHARED_MANAGED_EXTERNAL_POINTER_TAG_RANGE.contains_range(tag_range)
}

/// 原版 IsMaybeReadOnlyExternalPointerType
pub const fn is_maybe_read_only_external_pointer_type(tag_range: ExternalPointerTagRange) -> bool {
  K_ANY_MAYBE_READ_ONLY_EXTERNAL_POINTER_TAG_RANGE.contains_range(tag_range)
}

/// 原版 IsManagedExternalPointerType: 生命周期绑定表项 (ManagedResource 族)
pub const fn is_managed_external_pointer_type(tag_range: ExternalPointerTagRange) -> bool {
  K_ANY_MANAGED_RESOURCE_EXTERNAL_POINTER_TAG_RANGE.contains_range(tag_range)
}

/// 原版 ExternalPointerCanBeEmpty: 范围含 ArrayBufferExtension / 与 embedder 段相交 /
/// 属 InterceptorInfo 段时, 允许 null handle (需同时放行 null tag)
pub const fn external_pointer_can_be_empty(tag_range: ExternalPointerTagRange) -> bool {
  tag_range.contains(ExternalPointerTag::ArrayBufferExtensionTag)
    || ((tag_range.first as u32) <= (ExternalPointerTag::LastEmbedderDataTag as u32)
      && (ExternalPointerTag::FirstEmbedderDataTag as u32) <= (tag_range.last as u32))
    || K_ANY_INTERCEPTOR_INFO_EXTERNAL_POINTER_TAG_RANGE.contains_range(tag_range)
}

// ---- ExternalPointerHandle (v8-internal.h:395-414) ----
/// 不透明 32 位句柄 = 表索引左移 6 位 (移位保证索引必在表预留内)
pub type ExternalPointerHandle = u32;
pub const K_NULL_EXTERNAL_POINTER_HANDLE: ExternalPointerHandle = 0;

/// 原版 ExternalPointerTable::HandleToIndex (external-pointer-table-inl.h:355)
#[inline]
pub const fn handle_to_index(handle: ExternalPointerHandle) -> u32 {
  handle >> K_EXTERNAL_POINTER_INDEX_SHIFT
}

/// 原版 ExternalPointerTable::IndexToHandle (external-pointer-table-inl.h:375)
#[inline]
pub const fn index_to_handle(index: u32) -> ExternalPointerHandle {
  index << K_EXTERNAL_POINTER_INDEX_SHIFT
}

/// 原版 IsValidHandle (external-pointer-table-inl.h:349): 移位还原后须等于原值
#[inline]
pub const fn is_valid_handle(handle: ExternalPointerHandle) -> bool {
  handle == handle >> K_EXTERNAL_POINTER_INDEX_SHIFT << K_EXTERNAL_POINTER_INDEX_SHIFT
}

// ---- ExternalPointerPayload (tagged-payload.h + ExternalPointerTablePayloadTaggingScheme) ----
/// 表项 payload 编码: pointer | (tag << 49); mark 位 1<<48;
/// free 表项: payload 低 48 位 = freelist 下一索引
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExternalPointerPayload {
  pub encoded_word: u64,
}

impl ExternalPointerPayload {
  pub const FREE_ENTRY_TAG: ExternalPointerTag = ExternalPointerTag::ExternalPointerFreeEntryTag;
  pub const EVACUATION_ENTRY_TAG: ExternalPointerTag = ExternalPointerTag::ExternalPointerEvacuationEntryTag;

  /// 原版 TaggedPayload::Tag (tagged-payload.h:38-41); kPayloadShift == 0
  #[inline]
  pub fn tag(pointer: Address, tag: ExternalPointerTag) -> u64 {
    (pointer as u64) | ((tag as u64) << K_EXTERNAL_POINTER_TAG_SHIFT)
  }

  /// 原版 TaggedPayload 构造
  #[inline]
  pub fn new(pointer: Address, tag: ExternalPointerTag) -> Self {
    Self { encoded_word: Self::tag(pointer, tag) }
  }

  /// 原版 CheckTag (tagged-payload.h:43-48)
  #[inline]
  pub fn check_tag(content: u64, tag_range: ExternalPointerTagRange) -> bool {
    let tag = ((content & K_EXTERNAL_POINTER_TAG_MASK) >> K_EXTERNAL_POINTER_TAG_SHIFT) as u16;
    match ExternalPointerTag::from_repr(tag) {
      Some(t) => tag_range.contains(t),
      // 超出 7 位 tag 域的编码视为不匹配
      None => false,
    }
  }

  /// 原版 UntagAllowNullHandle: tag 不匹配返回 0
  #[inline]
  pub fn untag_allow_null(&self, tag_range: ExternalPointerTagRange) -> Address {
    if Self::check_tag(self.encoded_word, tag_range) {
      (self.encoded_word & K_EXTERNAL_POINTER_PAYLOAD_MASK) as Address
    } else {
      K_NULL_ADDRESS
    }
  }

  /// 原版 UntagDisallowNullHandle (SBXCHECK → None, 不 panic)
  #[inline]
  pub fn untag_disallow_null(&self, tag_range: ExternalPointerTagRange) -> Option<Address> {
    if Self::check_tag(self.encoded_word, tag_range) {
      Some((self.encoded_word & K_EXTERNAL_POINTER_PAYLOAD_MASK) as Address)
    } else {
      None
    }
  }

  /// 原版 Untag: 可空范围走 allow-null 路径
  #[inline]
  pub fn untag(&self, tag_range: ExternalPointerTagRange) -> Address {
    if external_pointer_can_be_empty(tag_range) {
      self.untag_allow_null(tag_range)
    } else {
      self.untag_disallow_null(tag_range).unwrap_or(K_NULL_ADDRESS)
    }
  }

  pub fn is_tagged_with_tag_in(&self, tag_range: ExternalPointerTagRange) -> bool {
    Self::check_tag(self.encoded_word, tag_range)
  }
  pub fn is_tagged_with(&self, tag: ExternalPointerTag) -> bool {
    Self::check_tag(self.encoded_word, ExternalPointerTagRange::of(tag))
  }

  /// 原版 SetMarkBit / ClearMarkBit / HasMarkBitSet
  #[inline]
  pub fn set_mark_bit(&mut self) { self.encoded_word |= K_EXTERNAL_POINTER_MARK_BIT; }
  #[inline]
  pub fn clear_mark_bit(&mut self) { self.encoded_word &= !K_EXTERNAL_POINTER_MARK_BIT; }
  #[inline]
  pub const fn has_mark_bit_set(&self) -> bool { self.encoded_word & K_EXTERNAL_POINTER_MARK_BIT != 0 }

  /// 原版 ExtractFreelistLink (tagged-payload.h:92-99): free 表项低 48 位为下一索引
  #[inline]
  pub fn extract_freelist_link(&self) -> Option<u32> {
    if self.is_tagged_with(Self::FREE_ENTRY_TAG) {
      Some((self.encoded_word & K_EXTERNAL_POINTER_PAYLOAD_MASK) as u32)
    } else {
      None
    }
  }

  /// 原版 SetTag (tagged-payload.h:101-104)
  #[inline]
  pub fn set_tag(&mut self, tag: ExternalPointerTag) {
    self.encoded_word = (self.encoded_word & !K_EXTERNAL_POINTER_TAG_MASK)
      | ((tag as u64) << K_EXTERNAL_POINTER_TAG_SHIFT);
  }

  /// 原版 ExtractTag (tagged-payload.h:106-108)
  #[inline]
  pub fn extract_tag(&self) -> Option<ExternalPointerTag> {
    ExternalPointerTag::from_repr(
      ((self.encoded_word & K_EXTERNAL_POINTER_TAG_MASK) >> K_EXTERNAL_POINTER_TAG_SHIFT) as u16,
    )
  }

  pub fn contains_freelist_link(&self) -> bool { self.is_tagged_with(Self::FREE_ENTRY_TAG) }
  pub fn contains_evacuation_entry(&self) -> bool { self.is_tagged_with(Self::EVACUATION_ENTRY_TAG) }
  pub fn contains_pointer(&self) -> bool {
    !self.contains_freelist_link() && !self.contains_evacuation_entry()
  }
}

// ---- ExternalPointerMember 存储语义 (external-pointer.h:31-90) ----
/// 对象内外部指针槽位 (沙箱开 = ExternalPointerHandle 存储)。
/// Init/load/store/exchange 走 IsolateForSandbox + ExternalPointerTable (第二梯队登记)。
#[derive(Debug, Clone, Copy)]
pub struct ExternalPointerMember {
  pub storage: ExternalPointerHandle,
}

impl ExternalPointerMember {
  pub const fn null() -> Self { Self { storage: K_NULL_EXTERNAL_POINTER_HANDLE } }

  /// 原版 load_encoded (external-pointer-inl.h:103-106)
  #[inline]
  pub const fn load_encoded(&self) -> ExternalPointerHandle { self.storage }

  /// 原版 store_encoded (external-pointer-inl.h:108-112)
  #[inline]
  pub fn store_encoded(&mut self, value: ExternalPointerHandle) { self.storage = value; }

  /// 槽位地址
  pub fn storage_address(&mut self) -> *mut ExternalPointerHandle {
    &mut self.storage
  }
}
`;

const p = writeSrc("v8-sandbox", "external_pointer.rs", body2);
console.log(
  `external-pointer.js: ${p} (枚举项 ${entries.length}: 变体 ${variants.length} + 标记别名 ${aliases.length}; 对数核对 ${checks.length} 条通过)`,
);
