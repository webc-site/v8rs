#!/usr/bin/env bun
// tool/src/objects/string.h.js — 覆盖 string.h 声明性骨架:
//   v8/src/objects/string.h (+ include/v8-primitive.h kMaxLength / v8-internal.h 布局锚点)
//   → v8-objects-strings/src/string.rs 基座区段 (@base 区段, 全文件重写)
//   后续由 string-inl.h.js 追加 @inl 区段, string.cc.js 追加 @cc-semantics 区段
//
// 映射登记 (C++ → Rust):
//   StringShape (map/InstanceType 双形态) → 非静态根路径: struct { ty: InstanceType }
//     谓词族全部委托 v8-objects-core::instance_type_checker (inl 脚本展开)
//   StringShape::DispatchToSpecificType / String::DispatchToSpecificType(WithoutCast)
//     → enum SpecificString (7 叶类 Tagged 变体; STRING_CLASS_TYPES X-macro 展开)
//     + StringShape::kind() → enum StringKind (tag+encoding 位判定, 1:1 switch)
//   String/SeqOneByteString/SeqTwoByteString/ConsString/SlicedString/ThinString/
//     UncachedExternalString/ExternalString/ExternalOneByteString/ExternalTwoByteString
//     → #[repr(C)] 逻辑视图结构体 (字段序对齐原版; tagged 槽用 TaggedMember<usize 逻辑宽>,
//     snapshot 偏移由 K_*_OFFSET 常量按 K_TAGGED_SIZE=4 压缩默认对齐;
//     FLEXIBLE_ARRAY_MEMBER chars 尾数组 → DST 惯用法: 不入结构体, 经
//     Tagged 地址 + CHARS_OFFSET 裸指针取切片, 登记于 struct 文档)
//   enum Encoding { ONE_BYTE_ENCODING, TWO_BYTE_ENCODING } → repr(u8) enum
//   enum TrimMode / enum class EqualityType / Utf8EncodingFlag 位 → enum + 常量
//   布局常量族 (sizeof 断言系) → 显式 K_* 偏移常量 + const assert
//     (锚点: Internals::kStringResourceOffset = kTaggedSize + 2*kInt32Size = 12)
//   CharTraits<uint8_t/uint16_t> 模板特化 → 泛型/enum 承载, 登记不落类型
//   ObjectTraits<String>::kHeaderSize 等 static_assert → const assert
//
// 跳过区段登记 (推迟, 归属):
//   - MakeThin/MakeExternal*/MarkForExternalizationDuringGC/SlowShare/Share/Unshare/
//     SlowFlatten/Flatten (isolate+factory+heap 布局变更通知): 归 v8-heap-*
//   - String::ToNumber (factory NewNumber + conversions): 归 runtime 层
//   - Match/GetSubstitution (IncrementalStringBuilder): 归 v8-runtime
//   - DECL_PRINTER/DECL_VERIFIER/ToAsciiArray/PrintOn: 归 v8-diagnostics
//   - StringTableKey/SequentialStringKey/SeqSubStringKey (string-inl.h:565-678):
//     归 string_table.rs 槽位 (本 crate 另一族)
//   - FlatStringReader 的 Relocatable 基类/句柄保持: 句柄体系归 v8-handles,
//     此处保留内容读取语义 (Tagged 直取)
//   - BodyDescriptor (SeqOneByteString/SeqTwoByteString): GC 访客, 归 L3 heap
//   - maglev/wasm 友元: 归对应 L5/L6 crate
import { banner, readV8, writeSrc } from "../../lib.js";

const src = readV8("src/objects/string.h");
const inl = readV8("src/objects/string-inl.h");
const primitiveH = readV8("include/v8-primitive.h");
const internalH = readV8("include/v8-internal.h");

// ── 锚点校验 ─────────────────────────────────────────────────────────────
const mustContain = (s, sub, file) => {
  if (!s.includes(sub)) throw new Error(`锚点失配: ${file} 无 "${sub.slice(0, 48)}..."`);
};
mustContain(src, "class StringShape {", "string.h");
mustContain(src, "V8_OBJECT class String : public Name {", "string.h");
mustContain(src, "V8_OBJECT class SeqOneByteString : public SeqString {", "string.h");
mustContain(src, "V8_OBJECT class ConsString : public String {", "string.h");
mustContain(src, "V8_OBJECT class ThinString : public String {", "string.h");
mustContain(src, "V8_OBJECT class SlicedString : public String {", "string.h");
mustContain(src, "V8_OBJECT class ExternalString : public UncachedExternalString {", "string.h");
mustContain(src, "static const uint32_t kMinLength = 13;", "string.h");
mustContain(src, "static const uint32_t kMaxHashCalcLength = 16383;", "string.h");
mustContain(src, "static const uint32_t kMaxShortPrintLength = 1024;", "string.h");
mustContain(src, "static uint32_t constexpr kInlineLineEndsSize = 32;", "string.h");
mustContain(src, "enum Encoding { ONE_BYTE_ENCODING, TWO_BYTE_ENCODING };", "string.h");
mustContain(src, "enum TrimMode { kTrim, kTrimStart, kTrimEnd };", "string.h");
mustContain(src, "enum class EqualityType { kWholeString, kPrefix, kNoLengthCheck };", "string.h");
mustContain(src, "kNullTerminate = 1u << 0,", "string.h");
mustContain(src, "static const int kStackSize = 32;", "string.h");
mustContain(primitiveH, "static constexpr int kMaxLength =", "v8-primitive.h");
mustContain(internalH, "static const int kStringResourceOffset =", "v8-internal.h");

// STRING_CLASS_TYPES X-macro 机械展开 (string-inl.h:504-511)
const classTypesDef = inl.match(/#define STRING_CLASS_TYPES\(V\)([\s\S]*?)\n\n/)[1];
const classTypes = [...classTypesDef.matchAll(/V\((\w+)\)/g)].map((m) => m[1]);
if (classTypes.length !== 7) throw new Error(`STRING_CLASS_TYPES ${classTypes.length} != 7`);
// 叶类名 → (StringKind 变体, 中文说明)
const kindOf = {
  SeqOneByteString: ["SeqOneByte", "顺序单字节"],
  SeqTwoByteString: ["SeqTwoByte", "顺序双字节"],
  ConsString: ["Cons", "拼接树节点 (编码位仅作声明, 双分支合一)"],
  ExternalOneByteString: ["ExternalOneByte", "外部单字节"],
  ExternalTwoByteString: ["ExternalTwoByte", "外部双字节"],
  SlicedString: ["Sliced", "切片 (编码位仅作声明, 双分支合一)"],
  ThinString: ["Thin", "薄串转发 (编码位仅作声明, 双分支合一)"],
};

const body = `${banner("src/objects/string.h.js", "src/objects/string.h + include/v8-primitive.h + include/v8-internal.h (声明性基座)")}
// 修订就地改, 复跑覆盖; @inl / @cc-semantics 区段由同名脚本追加/替换

//! 字符串对象族 (String 及其 7 个具体子类布局与派发)。
//!
//! 布局红线 (todo 10.3): Seq/Sliced/Cons/Thin/External 布局是 snapshot 兼容前提。
//! 结构体为逻辑视图 (字段序 1:1 原版), snapshot 偏移由 K_* 常量按压缩默认
//! (K_TAGGED_SIZE=4) 锚定; Tagged 槽当前为 usize 逻辑宽, 压缩收窄在 heap 层落地。
//!
//! 字符数据 (SeqString 的 FLEXIBLE_ARRAY_MEMBER chars) 不入结构体:
//! 经对象基址 + CHARS_OFFSET 裸指针构切片 (DST 惯用法, 见 SeqOneByteString 文档)。

use v8_common::K_TAGGED_SIZE;
use v8_objects_core::{
  heap_object::HeapObject, instance_type::InstanceType, map::Map, name::Name, smi::Smi,
  tagged::Tagged, tagged_field::TaggedMember,
};

// ── 常量 (string.h:540-560 + v8-primitive.h:129) ───────────────────────────
/// 单字节串最大码点 (unibrow::Latin1::kMaxChar = 0xFF)。
pub const K_MAX_ONE_BYTE_CHAR_CODE: u32 = 0xFF;
/// UTF-16 码元上限。
pub const K_MAX_UTF16_CODE_UNIT: u16 = 0xFFFF;
/// Unicode 码点上限。
pub const K_MAX_CODE_POINT: u32 = 0x10_FFFF;
/// 字符串最大长度 (64 位平台: (1 << 29) - 24; v8-primitive.h:129)。
pub const K_MAX_LENGTH: u32 = (1 << 29) - 24;
/// 哈希计算上限长度, 超出仅按长度取哈希 (string.h:557)。
pub const K_MAX_HASH_CALC_LENGTH: u32 = 16383;
/// 短打印截断长度 (string.h:560)。
pub const K_MAX_SHORT_PRINT_LENGTH: u32 = 1024;
/// ConsString 最小长度 (string.h:1102)。
pub const CONS_STRING_MIN_LENGTH: u32 = 13;
/// SlicedString 最小长度 (string.h:1212)。
pub const SLICED_STRING_MIN_LENGTH: u32 = 13;
/// 行结束位置向量内联容量 (string.h:686)。
pub const K_INLINE_LINE_ENDS_SIZE: usize = 32;

const _: () = assert!(K_MAX_LENGTH <= i32::MAX as u32);
const _: () = assert!(K_MAX_LENGTH * 2 <= i32::MAX as u32);
const _: () = assert!(K_MAX_LENGTH * 2 + 12 <= i32::MAX as u32);

// ── snapshot 布局常量 (压缩默认 K_TAGGED_SIZE=4; map(0) + hash(4) + length(8)) ──
/// raw_hash_field 偏移 (Name 尾字段)。
pub const K_HASH_FIELD_OFFSET: usize = K_TAGGED_SIZE;
/// length 偏移。
pub const K_LENGTH_OFFSET: usize = K_TAGGED_SIZE + 4;
/// String 头大小 (= Internals::kStringResourceOffset 锚点 = 12)。
pub const K_HEADER_SIZE: usize = K_TAGGED_SIZE + 8;
/// Seq 字符数据偏移。
pub const K_SEQ_CHARS_OFFSET: usize = K_HEADER_SIZE;
/// ConsString::first 偏移。
pub const K_CONS_FIRST_OFFSET: usize = K_HEADER_SIZE;
/// ConsString::second 偏移。
pub const K_CONS_SECOND_OFFSET: usize = K_HEADER_SIZE + K_TAGGED_SIZE;
/// ConsString 尺寸。
pub const K_CONS_SIZE: usize = K_HEADER_SIZE + 2 * K_TAGGED_SIZE;
/// ThinString::actual 偏移。
pub const K_THIN_ACTUAL_OFFSET: usize = K_HEADER_SIZE;
/// ThinString 尺寸。
pub const K_THIN_SIZE: usize = K_HEADER_SIZE + K_TAGGED_SIZE;
/// SlicedString::parent 偏移。
pub const K_SLICED_PARENT_OFFSET: usize = K_HEADER_SIZE;
/// SlicedString::offset 偏移。
pub const K_SLICED_OFFSET_OFFSET: usize = K_HEADER_SIZE + K_TAGGED_SIZE;
/// SlicedString 尺寸。
pub const K_SLICED_SIZE: usize = K_HEADER_SIZE + 2 * K_TAGGED_SIZE;
/// 外部指针槽位大小 (sandbox 句柄宽; ExternalPointerTable 编解码归 v8-sandbox)。
pub const K_EXTERNAL_POINTER_SLOT_SIZE: usize = 4;
/// ExternalString::resource 偏移 (== Internals::kStringResourceOffset)。
pub const K_EXTERNAL_RESOURCE_OFFSET: usize = K_HEADER_SIZE;
/// UncachedExternalString 尺寸 (无 resource_data 缓存槽)。
pub const K_UNCACHED_EXTERNAL_SIZE: usize = K_HEADER_SIZE + K_EXTERNAL_POINTER_SLOT_SIZE;
/// ExternalString::resource_data 偏移。
pub const K_EXTERNAL_RESOURCE_DATA_OFFSET: usize = K_EXTERNAL_RESOURCE_OFFSET + K_EXTERNAL_POINTER_SLOT_SIZE;
/// ExternalString 尺寸。
pub const K_EXTERNAL_SIZE: usize = K_HEADER_SIZE + 2 * K_EXTERNAL_POINTER_SLOT_SIZE;

const _: () = assert!(K_HEADER_SIZE == 12); // kApiTaggedSize + 2*kApiInt32Size (v8-internal.h:1014)

// ── 枚举族 ───────────────────────────────────────────────────────────────
/// String::Encoding (string.h:126)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Encoding {
  /// 单字节编码。
  OneByte = 0,
  /// 双字节编码。
  TwoByte = 1,
}

/// String::TrimMode (string.h:512)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TrimMode {
  /// 两端裁剪。
  Trim,
  /// 仅头裁剪。
  TrimStart,
  /// 仅尾裁剪。
  TrimEnd,
}

/// String::EqualityType (string.h:422)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EqualityType {
  /// 全串比较 (长度必须相等)。
  WholeString,
  /// 前缀比较。
  Prefix,
  /// 已检长度, 免检。
  NoLengthCheck,
}

/// String::Utf8EncodingFlag 位 (string.h:607-611)。
pub const UTF8_FLAG_NULL_TERMINATE: u32 = 1 << 0;
/// 无效码位替换 (U+FFFD) 而非截断。
pub const UTF8_FLAG_REPLACE_INVALID: u32 = 1 << 1;

// ── StringKind (STRING_CLASS_TYPES 展开; DispatchToSpecificType 的 enum 化) ──
/// 字符串表示 × 编码 7 叶类判别 (representation 3 bit + encoding 1 bit 派发)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StringKind {
${classTypes.map((t) => `  /// ${kindOf[t][1]}。`).join("\n")}
}

// ── 结构体 (逻辑视图; 字段序 1:1 原版) ────────────────────────────────────
/// 字符串抽象基类 (String): Name 头 + length (string.h:122)。
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct String {
  /// Name 头 (map + raw_hash_field)。
  pub name: Name,
  /// 字符串长度。
  pub length: u32,
}

/// 顺序单字节串 (SeqOneByteString); chars 尾数组经 CHARS_OFFSET 裸指针访问。
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SeqOneByteString {
  /// String 头。
  pub string: String,
}

/// 顺序双字节串 (SeqTwoByteString); chars 尾数组经 CHARS_OFFSET 裸指针访问。
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SeqTwoByteString {
  /// String 头。
  pub string: String,
}

/// 拼接串 (ConsString): 二叉树节点, 叶序连接即串值 (string.h:1071)。
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConsString {
  /// String 头。
  pub string: String,
  /// 左子树 (first_)。
  pub first: TaggedMember<String>,
  /// 右子树 (second_)。
  pub second: TaggedMember<String>,
}

/// 薄串 (ThinString): 内部化转发引用 (string.h:1143)。
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ThinString {
  /// String 头。
  pub string: String,
  /// 指向内部化串 (actual_)。
  pub actual: TaggedMember<String>,
}

/// 切片串 (SlicedString): parent + offset 视图 (string.h:1195)。
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SlicedString {
  /// String 头。
  pub string: String,
  /// 父串 (parent_; 只能为 Seq/External)。
  pub parent: TaggedMember<String>,
  /// 切片起点 (offset_; Smi)。
  pub offset: TaggedMember<Smi>,
}

/// 无缓存外部串 (UncachedExternalString): 仅 resource 槽 (string.h:1240)。
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct UncachedExternalString {
  /// String 头。
  pub string: String,
  /// 外部资源指针槽 (resource_; ExternalPointerTable 归 v8-sandbox)。
  pub resource: usize,
}

/// 外部串 (ExternalString): resource + resource_data 双槽 (string.h:1259)。
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ExternalString {
  /// String 头。
  pub string: String,
  /// 外部资源指针槽。
  pub resource: usize,
  /// 字符数据缓存指针槽 (resource_data_; uncached 型无此槽)。
  pub resource_data: usize,
}

/// 单字节外部串 (ExternalOneByteString)。
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ExternalOneByteString {
  /// ExternalString 全槽 (尺寸断言: == ExternalString)。
  pub external: ExternalString,
}

/// 双字节外部串 (ExternalTwoByteString)。
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ExternalTwoByteString {
  /// ExternalString 全槽。
  pub external: ExternalString,
}

/// DispatchToSpecificType 的 Tagged 化: 7 叶类穷举派发。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecificString {
  /// 顺序单字节。
  SeqOneByte(Tagged<SeqOneByteString>),
  /// 顺序双字节。
  SeqTwoByte(Tagged<SeqTwoByteString>),
  /// 外部单字节。
  ExternalOneByte(Tagged<ExternalOneByteString>),
  /// 外部双字节。
  ExternalTwoByte(Tagged<ExternalTwoByteString>),
  /// 薄串。
  Thin(Tagged<ThinString>),
  /// 拼接串。
  Cons(Tagged<ConsString>),
  /// 切片串。
  Sliced(Tagged<SlicedString>),
}

/// 字符串形状 (StringShape): 一次取 Map 判定, 避免二跳重复加载 (string.h:63)。
///
/// 注意: 展平或 GC 捷径化可能改变字符串形状, 缓存形状变量需自证无此并发。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringShape {
  /// 实例类型 (非静态根路径)。
  ty: InstanceType,
}

// @inl-marker (string-inl.h.js 追加起点)

// @cc-semantics-begin (string.cc.js 替换区段)
// @cc-semantics-end
`;

writeSrc("v8-objects-strings", "string.rs", body);
console.log(
  `string.rs 基座已生成: 布局常量 ${[
    "K_HEADER_SIZE",
    "K_CONS_SIZE",
    "K_THIN_SIZE",
    "K_SLICED_SIZE",
    "K_EXTERNAL_SIZE",
  ].join("/")} + 枚举 4 + 结构体 10 + StringKind 7 变体`,
);
