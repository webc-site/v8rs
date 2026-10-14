#!/usr/bin/env bun
// tool/src/objects/tagged-impl.h.js — tagged-impl 差集骨架生成
// 组→文件映射 (本脚本管辖):
//   v8/src/objects/tagged-impl.h     + tagged-impl-inl.h → v8-objects-core/src/tagged.rs (差集)
//   v8/src/objects/tagged.h 末尾自由函数 MakeWeak/MakeStrong → 同上
//   v8/src/objects/tagged-index.h    → 同上 (TaggedIndex)
// 产物: v8-objects-core/src/tagged.rs.skel (差集骨架, 就地合并进 tagged.rs 后可删;
//       tagged.rs 已有落地实现, 直出主文件会覆盖修订, 故走 .skel)
// 规则: 谓词宏 HAS_*_TAG 机械展开为位运算; 常量从原版提取并断言锚点
import { readV8, writeSrc, banner } from "../../lib.js";

const globals = readV8("src/common/globals.h");
const taggedImpl = readV8("src/objects/tagged-impl.h");
const taggedIndex = readV8("src/objects/tagged-index.h");

// —— 常量提取 (锚点断言, 防上游漂移) ——
const num = (src, re, name) => {
  const m = src.match(re);
  if (!m) throw new Error(`无法匹配常量 ${name}`);
  return Number(m[1]);
};

// globals.h: kClearedWeakHeapObjectLower32 / kWeakHeapObjectMask
const kClearedWeakHeapObjectLower32 = num(
  globals,
  /const uint32_t kClearedWeakHeapObjectLower32 = (\d+);/,
  "kClearedWeakHeapObjectLower32",
);
const weakMaskMatch = globals.match(/const Address kWeakHeapObjectMask = (\d+) << (\d+);/);
if (!weakMaskMatch) throw new Error("无法匹配常量 kWeakHeapObjectMask");
const weakMaskShift = Number(weakMaskMatch[2]);
if (Number(weakMaskMatch[1]) !== 1 || weakMaskShift !== 1)
  throw new Error("kWeakHeapObjectMask 预期为 1<<1");

// tagged-index.h: TaggedIndex 31 位值域
const kTaggedValueSize = num(
  taggedIndex,
  /static constexpr int kTaggedValueSize = (\d+);/,
  "kTaggedValueSize",
);
const kTaggedIndexMin = -(2 ** (kTaggedValueSize - 1)); // intptr 全 1 左移 31 位 → 符号位
const kTaggedIndexMax = -kTaggedIndexMin - 1;
if (kTaggedValueSize !== 31) throw new Error("kTaggedValueSize 预期为 31");
if (kTaggedIndexMin !== -(2 ** 30) || kTaggedIndexMax !== 2 ** 30 - 1)
  throw new Error("TaggedIndex 值域推导错误");

// —— tagged-impl.h 谓词方法清单提取 (签名区行级核对) ——
const predicates = [
  ...taggedImpl.matchAll(/constexpr inline bool (Is\w+)\(\) const/g),
].map((m) => m[1]);
for (const need of ["IsObject", "IsCleared", "IsStrongOrWeak", "IsStrong", "IsStrongOrSmi", "IsWeak", "IsWeakOrCleared"])
  if (!predicates.includes(need)) throw new Error(`tagged-impl.h 谓词 ${need} 缺失`);

const getters = [
  ...taggedImpl.matchAll(/inline (?:bool|Tagged<\w+>) (GetHeapObject\w*)/g),
].map((m) => m[1]);
for (const need of ["GetHeapObjectIfStrong", "GetHeapObjectAssumeStrong", "GetHeapObjectIfWeak", "GetHeapObjectAssumeWeak", "GetHeapObject", "GetHeapObjectOrSmi"])
  if (!getters.includes(need)) throw new Error(`tagged-impl.h 取出方法 ${need} 缺失`);

// —— 宏机械展开 (禁"理解"宏, 按定义逐字替换) ——
// HAS_SMI_TAG(v)            => (v & kSmiTagMask) == kSmiTag
// HAS_STRONG_HEAP_OBJECT_TAG(v) => (v & kHeapObjectTagMask) == kHeapObjectTag
// HAS_WEAK_HEAP_OBJECT_TAG(v)   => (v & kHeapObjectTagMask) == kWeakHeapObjectTag
const hasSmi = "(self.ptr() & SMI_TAG_MASK) == SMI_TAG";
const hasStrong = "(self.ptr() & HEAP_OBJECT_TAG_MASK) == HEAP_OBJECT_TAG";
const hasWeak = "(self.ptr() & HEAP_OBJECT_TAG_MASK) == WEAK_HEAP_OBJECT_TAG";

const body = `${banner("src/objects/tagged-impl.h.js", "src/objects/tagged-impl.h + tagged-impl-inl.h + tagged-index.h", "tagged.rs 差集骨架, 合并后就地修订; 复跑覆盖本 skel")}
//! 差集骨架: 弱引用谓词族 / 取出族 / MakeWeak 系列 / TaggedIndex

// ===== 常量 (globals.h 提取) =====
/// 已清除弱引用值的低 32 位 (${kClearedWeakHeapObjectLower32} = 0b11)
pub const CLEARED_WEAK_HEAP_OBJECT_LOWER32: u32 = ${kClearedWeakHeapObjectLower32};
/// 弱引用位掩码 (bit 1)
pub const WEAK_HEAP_OBJECT_MASK: usize = ${weakMaskMatch[1]} << ${weakMaskShift};

// ===== 谓词 (宏展开: HAS_*) =====
// is_smi:          ${hasSmi}
// is_strong_tag:   ${hasStrong}
// is_weak_tag:     ${hasWeak}

impl Tagged<MaybeObject> {
  pub const fn is_smi(&self) -> bool { ${hasSmi} }
  pub const fn is_cleared(&self) -> bool {
    (self.ptr() as u32) == CLEARED_WEAK_HEAP_OBJECT_LOWER32
  }
  pub const fn is_weak_or_cleared(&self) -> bool { ${hasWeak} }
  pub const fn is_weak(&self) -> bool { self.is_weak_or_cleared() && !self.is_cleared() }
  pub const fn is_strong(&self) -> bool { ${hasStrong} }
  pub const fn is_strong_or_weak(&self) -> bool { !self.is_smi() && !self.is_cleared() }
  pub const fn is_strong_or_smi(&self) -> bool { (self.ptr() & HEAP_OBJECT_TAG_MASK) != WEAK_HEAP_OBJECT_TAG }
  pub const fn is_object(&self) -> bool { !self.is_weak_or_cleared() }
  // 取出族 (tagged-impl-inl.h)
  pub fn get_heap_object_if_strong(&self) -> Option<Tagged<HeapObject>>;
  pub fn get_heap_object_if_weak(&self) -> Option<Tagged<HeapObject>>;
  pub fn get_heap_object(&self) -> Option<Tagged<HeapObject>>;
  pub fn get_heap_object_or_smi(&self) -> Tagged<Object>;
  pub fn as_smi(&self) -> Option<Tagged<Smi>>;
}

// ===== 自由函数 (tagged.h:912-932) =====
pub fn make_weak<T>(value: Tagged<T>) -> Tagged<Weak<T>>;
pub fn make_weak_or_smi<T>(value: Tagged<T>) -> Tagged<MaybeObject>;
pub fn make_strong<T>(value: Tagged<T>) -> Tagged<T>;

// ===== TaggedIndex (tagged-index.h) =====
pub const TAGGED_INDEX_VALUE_SIZE: usize = ${kTaggedValueSize};
pub const TAGGED_INDEX_MIN: i64 = ${kTaggedIndexMin};
pub const TAGGED_INDEX_MAX: i64 = ${kTaggedIndexMax};
pub struct TaggedIndex;
impl TaggedIndex {
  pub const fn from_intptr(value: isize) -> Tagged<Self>;
  pub const fn is_valid(value: isize) -> bool;
  pub const fn value(tagged: Tagged<Self>) -> isize;
}
`;

const p = writeSrc("v8-objects-core", "tagged.rs.skel", body);
console.log(`tagged-impl.h.js: 差集骨架 ${p} (谓词 ${predicates.length} 项, 取出 ${getters.length} 项)`);
