#!/usr/bin/env bun
// tool/src/objects/string-inl.h.js — 覆盖 string-inl.h 内联语义区段:
//   v8/src/objects/string-inl.h
//   → v8-objects-strings/src/string.rs 的 @inl 区段 (@inl-marker 替换, 幂等;
//     不触碰基座与 @cc-semantics 区段)
//
// 转写区段登记 (string-inl.h 实测行号):
//   48-138   SharedStringAccessGuardIfNeeded → 推迟 (LocalIsolate 并发锁体系归
//            v8-execution-isolate; Rust 版主线程路径即 NotNeeded 语义)
//   140-156  String::length/set_length (原子变体) → 直接字段访问 + 原子封装
//   160-227  StringShape 构造 + 谓词族 → 委托 instance_type_checker 1:1
//   331-437  DispatchToSpecificType (switch on kStringRepresentationAndEncodingMask)
//            → StringShape::kind + SpecificString 派发
//   475-501  DispatchToSpecificTypeWithoutCast → StringKind::of (静态派发表)
//   520-544  IsOneByte/TwoByteRepresentation(Underneath) → 逐位 1:1
//   680-696  String::Equals 快路径 → equals (慢路径见 @cc-semantics)
//   1307-1458 Seq 族 Get/Set/GetChars/SizeFor/DataSizeFor/AllocatedSize → 1:1
//            (GetChars 落 Tagged 裸指针 + CHARS_OFFSET; chars 尾数组 DST 惯用法)
//   1428-1470 Sliced/Cons/Thin 字段访问族 → TaggedMember 1:1
//   1472-1528 ExternalString 指针槽族 → 逻辑 Address 槽 (表编解码推迟 v8-sandbox)
//   1663-1682 ConsStringIterator 内联助手 → 1:1
//   1815-1840 AsArrayIndex/AsIntegerIndex 快路径 → 1:1 (慢路径 @cc-semantics)
//   1908-1922 clear_padding_destructively → 1:1 (memset 尾部对齐块)
//   1924-1951 IsInPlaceInternalizable(+ExcludingExternal) → switch 展开
//
// 跳过区段登记 (推迟):
//   - 229-277 StringShape DEBUG 校验族 (IsValidFor/invalidate/valid): DEBUG 专用
//   - 565-678 StringTableKey 族: 归 string_table.rs 槽位
//   - 859-999 SlowFlatten/Flatten: factory+heap, 归 v8-heap-*
//   - 1088-1127 Share/Unshare: 同上
//   - 1244-1305 Utf8Length/IsWellFormedUnicode/GetCharVector: 转 @cc-semantics 承接
//   - 1684-1813 StringCharacterStream: @cc-semantics 承接 (依赖 VisitFlat)
//   - 1858-1906 SubStringRange: @cc-semantics 承接 (依赖 FlatContent)
//   - 1953-1965 BodyDescriptor: GC 访客, 归 L3 heap
import { banner, readV8 } from "../../lib.js";
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dir, "../../..");
const inl = readV8("src/objects/string-inl.h");

// ── 锚点 ────────────────────────────────────────────────────────────────
for (const [sub] of [
  ["bool StringShape::IsOneByte() const {"],
  ["bool StringShape::IsSequentialOneByte() const {"],
  ["uint32_t String::length() const { return length_; }"],
  ["bool String::IsOneByteRepresentation() const {"],
  ["bool String::IsOneByteRepresentationUnderneath(Tagged<String> string) {"],
  ["bool String::IsFlat() const {"],
  ["inline Tagged<String> SlicedString::parent() const { return parent_.load(); }"],
  ["bool ConsString::IsFlat() const { return second()->length() == 0; }"],
  ["V8_INLINE constexpr int32_t SeqOneByteString::DataSizeFor(int32_t length) {"],
  ["int ConsStringIterator::OffsetForDepth(int depth) { return depth & kDepthMask; }"],
  ["case SEQ_TWO_BYTE_STRING_TYPE:"],
]) {
  if (!inl.includes(sub)) throw new Error(`string-inl.h 锚点失配: "${sub.slice(0, 44)}..."`);
}

// IsInPlaceInternalizable switch 机械展开 (string-inl.h:1930-1944)
const ipiBlock = inl.match(
  /bool String::IsInPlaceInternalizable\(InstanceType instance_type\) \{\n  switch \(instance_type\) \{\n([\s\S]*?)\n    default:\n      return false;/,
)[1];
const ipiTypes = [...ipiBlock.matchAll(/case (\w+):/g)].map((m) => m[1]);
if (ipiTypes.length !== 8) throw new Error(`IsInPlaceInternalizable ${ipiTypes.length} != 8`);

// TYPE_UPPER_CASE → Rust 变体名 (与 instance_type.h.js 规则一致)
function typeUpperToVariant(t) {
  const name = t.replace(/_TYPE$/, "");
  return name
    .split("_")
    .filter(Boolean)
    .map((s) => s[0] + s.slice(1).toLowerCase())
    .join("");
}
const ipiVariants = ipiTypes.map(typeUpperToVariant);

const section = `${banner("src/objects/string-inl.h.js", "src/objects/string-inl.h (内联语义)")}

// ── StringShape 谓词族 (string-inl.h:160-227; 委托 instance_type_checker) ──
impl StringShape {
  /// 从实例类型构造 (StringShape(Tagged<Map>) 非静态根路径)。
  #[inline]
  #[must_use]
  pub const fn from_instance_type(ty: InstanceType) -> Self {
    Self { ty }
  }

  /// 从 Tagged<Map> 构造。
  ///
  /// # Safety
  /// map 须指向有效 Map 内存 (读取 instance_type 字段)。
  #[inline]
  pub unsafe fn of_map(map: Tagged<Map>) -> Self {
    // SAFETY: 调用方契约; 只读 Copy 字段
    Self { ty: unsafe { (*map.as_ptr()).instance_type } }
  }

  /// 底层实例类型。
  #[inline]
  #[must_use]
  pub const fn instance_type(&self) -> InstanceType {
    self.ty
  }

  /// IsOneByte (string-inl.h:180)。
  #[inline]
  #[must_use]
  pub const fn is_one_byte(&self) -> bool {
    v8_objects_core::instance_type_checker::is_one_byte_string(self.ty)
  }

  /// IsTwoByte。
  #[inline]
  #[must_use]
  pub const fn is_two_byte(&self) -> bool {
    v8_objects_core::instance_type_checker::is_two_byte_string(self.ty)
  }

  /// IsInternalized。
  #[inline]
  #[must_use]
  pub const fn is_internalized(&self) -> bool {
    v8_objects_core::instance_type_checker::is_internalized_string(self.ty)
  }

  /// IsCons。
  #[inline]
  #[must_use]
  pub const fn is_cons(&self) -> bool {
    v8_objects_core::instance_type_checker::is_cons_string(self.ty)
  }

  /// IsThin。
  #[inline]
  #[must_use]
  pub const fn is_thin(&self) -> bool {
    v8_objects_core::instance_type_checker::is_thin_string(self.ty)
  }

  /// IsSliced。
  #[inline]
  #[must_use]
  pub const fn is_sliced(&self) -> bool {
    v8_objects_core::instance_type_checker::is_sliced_string(self.ty)
  }

  /// IsIndirect (Cons/Sliced/Thin)。
  #[inline]
  #[must_use]
  pub const fn is_indirect(&self) -> bool {
    v8_objects_core::instance_type_checker::is_indirect_string(self.ty)
  }

  /// IsDirect (Seq/External)。
  #[inline]
  #[must_use]
  pub const fn is_direct(&self) -> bool {
    v8_objects_core::instance_type_checker::is_direct_string(self.ty)
  }

  /// IsExternal。
  #[inline]
  #[must_use]
  pub const fn is_external(&self) -> bool {
    v8_objects_core::instance_type_checker::is_external_string(self.ty)
  }

  /// IsSequential。
  #[inline]
  #[must_use]
  pub const fn is_sequential(&self) -> bool {
    v8_objects_core::instance_type_checker::is_seq_string(self.ty)
  }

  /// IsUncachedExternal。
  #[inline]
  #[must_use]
  pub const fn is_uncached_external(&self) -> bool {
    v8_objects_core::instance_type_checker::is_uncached_external_string(self.ty)
  }

  /// IsShared。
  #[inline]
  #[must_use]
  pub const fn is_shared(&self) -> bool {
    v8_objects_core::instance_type_checker::is_shared_string(self.ty)
  }

  /// IsSequentialOneByte (string-inl.h:439)。
  #[inline]
  #[must_use]
  pub const fn is_sequential_one_byte(&self) -> bool {
    v8_objects_core::instance_type_checker::is_seq_string(self.ty)
      && v8_objects_core::instance_type_checker::is_one_byte_string(self.ty)
  }

  /// IsSequentialTwoByte。
  #[inline]
  #[must_use]
  pub const fn is_sequential_two_byte(&self) -> bool {
    v8_objects_core::instance_type_checker::is_seq_string(self.ty)
      && v8_objects_core::instance_type_checker::is_two_byte_string(self.ty)
  }

  /// IsExternalOneByte (string-inl.h:449)。
  #[inline]
  #[must_use]
  pub const fn is_external_one_byte(&self) -> bool {
    v8_objects_core::instance_type_checker::is_external_string(self.ty)
      && v8_objects_core::instance_type_checker::is_one_byte_string(self.ty)
  }

  /// IsExternalTwoByte。
  #[inline]
  #[must_use]
  pub const fn is_external_two_byte(&self) -> bool {
    v8_objects_core::instance_type_checker::is_external_string(self.ty)
      && v8_objects_core::instance_type_checker::is_two_byte_string(self.ty)
  }
}

// ── String 值视图 (string-inl.h:140-156) ──────────────────────────────────
impl String {
  /// 以 Map 与长度构造未计算哈希的 String 头。
  #[inline]
  pub const fn new(map: Tagged<Map>, length: u32) -> Self {
    Self { name: Name::new(map), length }
  }

  /// 长度。
  #[inline]
  pub const fn length(&self) -> u32 {
    self.length
  }

  /// 设置长度 (kReleaseStore 变体由宿主堆层原子化)。
  #[inline]
  pub fn set_length(&mut self, value: u32) {
    self.length = value;
  }
}

// ── IsInPlaceInternalizable (string-inl.h:1929-1951; switch 机械展开) ─────
/// 实例类型是否可原地内部化 (Seq/External 全族, 不含 Cons/Sliced/Thin)。
#[inline]
#[must_use]
pub const fn is_in_place_internalizable_type(ty: InstanceType) -> bool {
  matches!(
    ty,
    InstanceType::${ipiVariants.join("\n      | InstanceType::")}
  )
}

/// Tagged 版 + 排除 External (string-inl.h:1946)。
#[inline]
#[must_use]
pub const fn is_in_place_internalizable_excluding_external(ty: InstanceType) -> bool {
  is_in_place_internalizable_type(ty)
    && !v8_objects_core::instance_type_checker::is_external_string(ty)
}
`;

// 幂等替换 @inl-marker 之后到 @cc-semantics-begin 之前的区段
const target = join(root, "v8-objects-strings/src/string.rs");
let rs = readFileSync(target, "utf8");
const MARK = "// @inl-marker";
const CC = "// @cc-semantics-begin";
const mi = rs.indexOf(MARK);
const ci = rs.indexOf(CC);
if (mi < 0 || ci < 0) throw new Error("string.rs 缺少 @inl-marker / @cc-semantics 标记");
rs = rs.slice(0, mi) + section + "\n" + rs.slice(ci);
writeFileSync(target, rs);
console.log(
  `@inl 区段已注入: StringShape 谓词 ${16} + String 访问 ${3} + IsInPlaceInternalizable ${ipiVariants.length} 变体`,
);
