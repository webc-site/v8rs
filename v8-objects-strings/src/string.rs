// 由 tool/src/objects/string.h.js 从 v8/src/objects/string.h + include/v8-primitive.h + include/v8-internal.h (声明性基座) 生成; 修订就地改, 复跑覆盖

// 修订就地改, 复跑覆盖; @inl / @cc-semantics 区段由同名脚本追加/替换

//! 字符串对象族 (String 及其 7 个具体子类布局与派发)。
//!
//! 布局红线 (todo 10.3): Seq/Sliced/Cons/Thin/External 布局是 snapshot 兼容前提。
//! 结构体为逻辑视图 (字段序 1:1 原版), snapshot 偏移由 K_* 常量按压缩默认
//! (K_TAGGED_SIZE=4) 锚定; Tagged 槽当前为 usize 逻辑宽, 压缩收窄在 heap 层落地。
//!
//! 字符数据 (SeqString 的 FLEXIBLE_ARRAY_MEMBER chars) 不入结构体:
//! 经对象基址 + CHARS_OFFSET 裸指针构切片 (DST 惯用法, 见 SeqOneByteString 文档)。

use std::{
  ptr::{copy_nonoverlapping, null},
  slice::from_raw_parts,
  string,
};

use gxhash::HashMap;
use v8_common::K_TAGGED_SIZE;
use v8_objects_core::{
  instance_type::{
    InstanceType, K_INTERNALIZED_TAG, K_IS_NOT_INTERNALIZED_MASK, K_NOT_INTERNALIZED_TAG,
    K_ONE_BYTE_STRING_TAG, K_SHARED_STRING_MASK, K_SHARED_STRING_TAG, K_STRING_ENCODING_MASK,
    K_STRING_REPRESENTATION_MASK, K_TWO_BYTE_STRING_TAG, K_UNCACHED_EXTERNAL_STRING_MASK,
    K_UNCACHED_EXTERNAL_STRING_TAG,
  },
  instance_type_checker::{
    is_cons_string, is_direct_string, is_external_string, is_indirect_string,
    is_internalized_string, is_one_byte_string, is_seq_string, is_shared_string, is_sliced_string,
    is_string, is_thin_string, is_two_byte_string, is_uncached_external_string,
  },
  map::Map,
  name::{EMPTY_HASH_FIELD, Name},
  objects::ComparisonResult,
  smi::Smi,
  tagged::Tagged,
  tagged_field::TaggedMember,
};
use v8_strings::{
  hash_field::{
    MAX_ARRAY_INDEX_SIZE, MAX_CACHED_ARRAY_INDEX_LENGTH, MAX_INTEGER_INDEX_SIZE,
    contains_cached_array_index, is_integer_index,
  },
  hash_seed::HashSeedData,
  string_hasher::{
    decode_array_index_from_hash_field, get_trivial_hash, hash_sequential_one_byte,
    hash_sequential_two_byte,
  },
  string_search::{
    search_string_u8, search_string_u8_in_u16, search_string_u16, search_string_u16_in_u8,
  },
  unicode::{
    utf8::{SIZE_OF_UNMATCHED_SURROGATE, encode, length},
    utf16::{NO_PREVIOUS_CHARACTER, is_surrogate_pair},
  },
};

use crate::{
  Error, Result,
  string_comparator::{FlatSegment, SegmentSource, StringComparator},
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
pub const K_EXTERNAL_RESOURCE_DATA_OFFSET: usize =
  K_EXTERNAL_RESOURCE_OFFSET + K_EXTERNAL_POINTER_SLOT_SIZE;
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
  /// 顺序单字节。
  SeqOneByte,
  /// 顺序双字节。
  SeqTwoByte,
  /// 拼接树节点 (编码位仅作声明, 双分支合一)。
  Cons,
  /// 外部单字节。
  ExternalOneByte,
  /// 外部双字节。
  ExternalTwoByte,
  /// 切片 (编码位仅作声明, 双分支合一)。
  Sliced,
  /// 薄串转发 (编码位仅作声明, 双分支合一)。
  Thin,
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

// @inl-marker
// 由 tool/src/objects/string-inl.h.js 从 v8/src/objects/string-inl.h (内联语义) 生成; 修订就地改, 复跑覆盖

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
    Self {
      ty: unsafe { (*map.as_ptr()).instance_type },
    }
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
    is_one_byte_string(self.ty)
  }

  /// IsTwoByte。
  #[inline]
  #[must_use]
  pub const fn is_two_byte(&self) -> bool {
    is_two_byte_string(self.ty)
  }

  /// IsInternalized。
  #[inline]
  #[must_use]
  pub const fn is_internalized(&self) -> bool {
    is_internalized_string(self.ty)
  }

  /// IsCons。
  #[inline]
  #[must_use]
  pub const fn is_cons(&self) -> bool {
    is_cons_string(self.ty)
  }

  /// IsThin。
  #[inline]
  #[must_use]
  pub const fn is_thin(&self) -> bool {
    is_thin_string(self.ty)
  }

  /// IsSliced。
  #[inline]
  #[must_use]
  pub const fn is_sliced(&self) -> bool {
    is_sliced_string(self.ty)
  }

  /// IsIndirect (Cons/Sliced/Thin)。
  #[inline]
  #[must_use]
  pub const fn is_indirect(&self) -> bool {
    is_indirect_string(self.ty)
  }

  /// IsDirect (Seq/External)。
  #[inline]
  #[must_use]
  pub const fn is_direct(&self) -> bool {
    is_direct_string(self.ty)
  }

  /// IsExternal。
  #[inline]
  #[must_use]
  pub const fn is_external(&self) -> bool {
    is_external_string(self.ty)
  }

  /// IsSequential。
  #[inline]
  #[must_use]
  pub const fn is_sequential(&self) -> bool {
    is_seq_string(self.ty)
  }

  /// IsUncachedExternal。
  #[inline]
  #[must_use]
  pub const fn is_uncached_external(&self) -> bool {
    is_uncached_external_string(self.ty)
  }

  /// IsShared。
  #[inline]
  #[must_use]
  pub const fn is_shared(&self) -> bool {
    is_shared_string(self.ty)
  }

  /// IsSequentialOneByte (string-inl.h:439)。
  #[inline]
  #[must_use]
  pub const fn is_sequential_one_byte(&self) -> bool {
    is_seq_string(self.ty) && is_one_byte_string(self.ty)
  }

  /// IsSequentialTwoByte。
  #[inline]
  #[must_use]
  pub const fn is_sequential_two_byte(&self) -> bool {
    is_seq_string(self.ty) && is_two_byte_string(self.ty)
  }

  /// IsExternalOneByte (string-inl.h:449)。
  #[inline]
  #[must_use]
  pub const fn is_external_one_byte(&self) -> bool {
    is_external_string(self.ty) && is_one_byte_string(self.ty)
  }

  /// IsExternalTwoByte。
  #[inline]
  #[must_use]
  pub const fn is_external_two_byte(&self) -> bool {
    is_external_string(self.ty) && is_two_byte_string(self.ty)
  }
}

// ── String 值视图 (string-inl.h:140-156) ──────────────────────────────────
impl String {
  /// 以 Map 与长度构造未计算哈希的 String 头。
  #[inline]
  pub const fn new(map: Tagged<Map>, length: u32) -> Self {
    Self {
      name: Name::new(map),
      length,
    }
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
    InstanceType::SeqTwoByteString
      | InstanceType::SeqOneByteString
      | InstanceType::SharedSeqTwoByteString
      | InstanceType::SharedSeqOneByteString
      | InstanceType::ExternalTwoByteString
      | InstanceType::ExternalOneByteString
      | InstanceType::SharedExternalTwoByteString
      | InstanceType::SharedExternalOneByteString
  )
}

/// Tagged 版 + 排除 External (string-inl.h:1946)。
#[inline]
#[must_use]
pub const fn is_in_place_internalizable_excluding_external(ty: InstanceType) -> bool {
  is_in_place_internalizable_type(ty) && !is_external_string(ty)
}

// ── StringKind::of 派发表 (string-inl.h:407 DispatchToSpecificType switch; 24 型) ──
impl StringKind {
  /// 实例类型 → 7 叶类判别 (representation 3 bit + encoding 1 bit;
  /// 24 个字符串实例型全表, 编码位对 Cons/Sliced/Thin 双分支合一)。
  #[inline]
  #[must_use]
  pub const fn of(ty: InstanceType) -> Self {
    use v8_objects_core::instance_type::{
      K_CONS_STRING_TAG, K_EXTERNAL_STRING_TAG, K_ONE_BYTE_STRING_TAG, K_SEQ_STRING_TAG,
      K_SLICED_STRING_TAG, K_STRING_ENCODING_MASK, K_STRING_REPRESENTATION_MASK, K_THIN_STRING_TAG,
    };
    debug_assert!(is_string(ty));
    let v = ty as u32;
    let rep = v & K_STRING_REPRESENTATION_MASK;
    let one_byte = (v & K_STRING_ENCODING_MASK) == K_ONE_BYTE_STRING_TAG;
    if rep == K_SEQ_STRING_TAG {
      if one_byte {
        Self::SeqOneByte
      } else {
        Self::SeqTwoByte
      }
    } else if rep == K_CONS_STRING_TAG {
      Self::Cons
    } else if rep == K_EXTERNAL_STRING_TAG {
      if one_byte {
        Self::ExternalOneByte
      } else {
        Self::ExternalTwoByte
      }
    } else if rep == K_SLICED_STRING_TAG {
      Self::Sliced
    } else if rep == K_THIN_STRING_TAG {
      Self::Thin
    } else {
      // representation 4/6/7 未定义; is_string 前置已拦
      Self::SeqOneByte
    }
  }
}

// ── Seq 尺寸公式 (string-inl.h:1388-1405) ─────────────────────────────────
/// OBJECT_POINTER_ALIGN: 压缩默认 kObjectAlignment = kTaggedSize = 4
/// (globals.h:1090-1091)。
const OBJECT_POINTER_ALIGN: usize = K_TAGGED_SIZE;

impl SeqOneByteString {
  /// 数据尺寸 = 头 + n 字符 (DataSizeFor; string-inl.h:1388)。
  #[inline]
  #[must_use]
  pub const fn data_size_for(length: u32) -> usize {
    K_HEADER_SIZE + length as usize
  }

  /// 对齐后对象尺寸 (SizeFor; string-inl.h:1398)。
  #[inline]
  #[must_use]
  pub const fn size_for(length: u32) -> usize {
    Self::data_size_for(length).div_ceil(OBJECT_POINTER_ALIGN) * OBJECT_POINTER_ALIGN
  }
}

impl SeqTwoByteString {
  /// 数据尺寸 = 头 + 2n 字节 (DataSizeFor; string-inl.h:1393)。
  #[inline]
  #[must_use]
  pub const fn data_size_for(length: u32) -> usize {
    K_HEADER_SIZE + length as usize * 2
  }

  /// 对齐后对象尺寸 (SizeFor; string-inl.h:1403)。
  #[inline]
  #[must_use]
  pub const fn size_for(length: u32) -> usize {
    Self::data_size_for(length).div_ceil(OBJECT_POINTER_ALIGN) * OBJECT_POINTER_ALIGN
  }
}

// ── 值构造器 (测试/宿主堆层布局锚; 原版经 Factory 分配, 此处只构造逻辑值) ──
impl ConsString {
  /// 以 Map/长度/左右子树构造 (字段序 1:1 string.h:1124-1126)。
  #[inline]
  #[must_use]
  pub fn new(map: Tagged<Map>, length: u32, first: Tagged<String>, second: Tagged<String>) -> Self {
    Self {
      string: String::new(map, length),
      first: TaggedMember::new(first),
      second: TaggedMember::new(second),
    }
  }
}

impl ThinString {
  /// 以 Map/长度/转发目标构造 (string.h:1175)。
  #[inline]
  #[must_use]
  pub fn new(map: Tagged<Map>, length: u32, actual: Tagged<String>) -> Self {
    Self {
      string: String::new(map, length),
      actual: TaggedMember::new(actual),
    }
  }
}

impl SlicedString {
  /// 以 Map/长度/父串/偏移构造 (string.h:1228-1229; offset 为 Smi 槽)。
  #[inline]
  #[must_use]
  pub fn new(map: Tagged<Map>, length: u32, parent: Tagged<String>, offset: i32) -> Self {
    Self {
      string: String::new(map, length),
      parent: TaggedMember::new(parent),
      offset: TaggedMember::new(Smi::from_int(offset)),
    }
  }
}

impl ExternalString {
  /// 以 Map/长度/资源指针/数据缓存指针构造 (string.h:1244/1300)。
  #[inline]
  #[must_use]
  pub fn new(map: Tagged<Map>, length: u32, resource: usize, resource_data: usize) -> Self {
    Self {
      string: String::new(map, length),
      resource,
      resource_data,
    }
  }
}

// ── FlatContent (string.h:137-217) ────────────────────────────────────────
/// 展平内容视图: 非展平 / 单字节 / 双字节三态 (string.h:142-166)。
///
/// 借用期内底层字符内存必须有效且不被 GC 移动 (原版 DisallowGarbageCollection
/// 生命周期约束的 Rust 借用承接)。
#[derive(Debug, Clone, Copy)]
pub enum FlatContent<'a> {
  /// 非展平串无内容。
  NonFlat,
  /// 单字节内容。
  OneByte(&'a [u8]),
  /// 双字节内容。
  TwoByte(&'a [u16]),
}

impl FlatContent<'_> {
  /// IsFlat (string.h:142)。
  #[inline]
  #[must_use]
  pub const fn is_flat(&self) -> bool {
    !matches!(self, Self::NonFlat)
  }

  /// IsOneByte (string.h:144)。
  #[inline]
  #[must_use]
  pub const fn is_one_byte(&self) -> bool {
    matches!(self, Self::OneByte(_))
  }

  /// IsTwoByte (string.h:145)。
  #[inline]
  #[must_use]
  pub const fn is_two_byte(&self) -> bool {
    matches!(self, Self::TwoByte(_))
  }

  /// ToOneByteVector (string.h:150; 非 OneByte 态为空切片, 原版 DCHECK 承接)。
  #[inline]
  #[must_use]
  pub const fn to_one_byte_vector(&self) -> &[u8] {
    match self {
      Self::OneByte(v) => v,
      _ => &[],
    }
  }

  /// ToUC16Vector (string.h:154)。
  #[inline]
  #[must_use]
  pub const fn to_uc16_vector(&self) -> &[u16] {
    match self {
      Self::TwoByte(v) => v,
      _ => &[],
    }
  }

  /// length (string.h:183)。
  #[inline]
  #[must_use]
  pub const fn length(&self) -> u32 {
    match self {
      Self::NonFlat => 0,
      Self::OneByte(v) => v.len() as u32,
      Self::TwoByte(v) => v.len() as u32,
    }
  }

  /// Get (string.h:161)。
  #[inline]
  #[must_use]
  pub fn get(&self, i: u32) -> u16 {
    match self {
      Self::OneByte(v) => u16::from(v[i as usize]),
      Self::TwoByte(v) => v[i as usize],
      Self::NonFlat => {
        debug_assert!(false, "NonFlat 无内容");
        0
      }
    }
  }

  /// UsesSameString (string.h:168): 原版比较 onebyte_start 裸指针 (union 首
  /// 槽), NonFlat 态指针为 null — 双 NonFlat 同源成立, 与原版逐位等价。
  #[inline]
  #[must_use]
  pub fn uses_same_string(&self, other: &Self) -> bool {
    fn head_ptr(c: &FlatContent<'_>) -> usize {
      match c {
        FlatContent::OneByte(v) => v.as_ptr() as usize,
        FlatContent::TwoByte(v) => v.as_ptr() as usize,
        FlatContent::NonFlat => 0,
      }
    }
    head_ptr(self) == head_ptr(other)
  }
}

// ── Ext trait 族 (tests/ 与宿主堆层的 Tagged 视图访问面) ──────────────────
/// Seq 字符尾的运行时偏移 (Rust 逻辑布局: Tagged 槽 usize 宽 ⇒ String 头实际
/// 尺寸; 压缩 snapshot 布局的 12 字节锚见 K_SEQ_CHARS_OFFSET, 两者不可混用)。
pub const CHARS_OFFSET: usize = size_of::<String>();

/// String 值视图访问器 (string-inl.h 140-1305 内联语义 + string.cc 慢路径)。
///
/// # Safety
/// 实现经裸指针读写堆上对象; 调用方须保证 Tagged 指向有效初始化的
/// 对应字符串对象, 且借用期内不被移动/释放 (无 GC 竞争)。
pub trait StringExt {
  /// 长度 (string-inl.h:140)。
  ///
  /// # Safety
  /// 继承 trait 级 Safety 契约 (裸指针访问, 见 trait 文档)。
  unsafe fn length(&self) -> u32;
  /// 原始哈希字段 (Name::raw_hash_field 读)。
  ///
  /// # Safety
  /// 继承 trait 级 Safety 契约 (裸指针访问, 见 trait 文档)。
  unsafe fn raw_hash_field(&self) -> u32;
  /// TryGetHash (name-inl.h:243; HashBits 解码)。
  ///
  /// # Safety
  /// 继承 trait 级 Safety 契约 (裸指针访问, 见 trait 文档)。
  unsafe fn try_get_hash(&self) -> Option<u32>;
  /// set_raw_hash_field_if_empty (string.cc:1940; 仅空字段写入)。
  ///
  /// # Safety
  /// 继承 trait 级 Safety 契约 (裸指针访问, 见 trait 文档)。
  unsafe fn set_raw_hash_field_if_empty(&self, hash: u32);
  /// 取形状 (StringShape(Tagged<String>); string-inl.h:160)。
  ///
  /// # Safety
  /// 继承 trait 级 Safety 契约 (裸指针访问, 见 trait 文档)。
  unsafe fn shape(&self) -> StringShape;
  /// 单字符读 (string-inl.h:1129 Get → GetImpl 派发)。
  ///
  /// # Safety
  /// 继承 trait 级 Safety 契约 (裸指针访问, 见 trait 文档)。
  unsafe fn get(&self, index: u32) -> u16;
  /// 等值比较 (string-inl.h:680 Equals: 指针同一/双内部化快路径 + 慢路径)。
  ///
  /// # Safety
  /// 继承 trait 级 Safety 契约 (裸指针访问, 见 trait 文档)。
  unsafe fn equals(&self, other: Tagged<String>) -> bool;
  /// IsEqualTo u8 数据 (string-inl.h:698-787; 全串/前缀/免检三型)。
  ///
  /// # Safety
  /// 继承 trait 级 Safety 契约 (裸指针访问, 见 trait 文档)。
  unsafe fn is_equal_to_u8(&self, data: &[u8], ty: EqualityType) -> bool;
  /// IsEqualTo u16 数据。
  ///
  /// # Safety
  /// 继承 trait 级 Safety 契约 (裸指针访问, 见 trait 文档)。
  unsafe fn is_equal_to_u16(&self, data: &[u16], ty: EqualityType) -> bool;
  /// IsOneByteRepresentationUnderneath (string-inl.h:529; 间接串穿透)。
  ///
  /// # Safety
  /// 继承 trait 级 Safety 契约 (裸指针访问, 见 trait 文档)。
  unsafe fn is_one_byte_representation_underneath(&self) -> bool;
  /// GetFlatContent (string-inl.h:1026-1086 + string.cc:695 SlowGetFlatContent)。
  ///
  /// # Safety
  /// 继承 trait 级 Safety 契约 (裸指针访问, 见 trait 文档)。
  unsafe fn get_flat_content(&self) -> FlatContent<'_>;
  /// ComputeAndSetRawHash (string.cc:1954; Thin 等长复用 + Cons/Sliced 解包)。
  ///
  /// # Safety
  /// 继承 trait 级 Safety 契约 (裸指针访问, 见 trait 文档)。
  unsafe fn compute_and_set_raw_hash(
    &self,
    out_one_byte_content: Option<&mut bool>,
    seed: &HashSeedData,
  ) -> Result<u32>;
  /// AsArrayIndex (string-inl.h:1815 + string.cc:2007 慢路径)。
  ///
  /// # Safety
  /// 继承 trait 级 Safety 契约 (裸指针访问, 见 trait 文档)。
  unsafe fn as_array_index(&self) -> Option<u32>;
  /// AsIntegerIndex (string-inl.h:1829 + string.cc:2022 慢路径)。
  ///
  /// # Safety
  /// 继承 trait 级 Safety 契约 (裸指针访问, 见 trait 文档)。
  unsafe fn as_integer_index(&self) -> Option<usize>;
  /// TryGetFlatContentFromDirectString (string-inl.h:1002; 间接串 → NonFlat)。
  ///
  /// # Safety
  /// 继承 trait 级 Safety 契约 (裸指针访问, 见 trait 文档)。
  unsafe fn try_flat_content_of_direct<'a>(
    string: Tagged<String>,
    offset: u32,
    length: u32,
  ) -> FlatContent<'a>;
}

/// SeqOneByteString 访问器 (string-inl.h:1307-1353)。
///
/// # Safety
/// 同 [StringExt]; 且对象必须是 SeqOneByteString 布局。
pub trait SeqOneByteExt {
  /// GetChars (string-inl.h:1340; 字符尾切片)。
  ///
  /// # Safety
  /// 继承 trait 级 Safety 契约 (裸指针访问, 见 trait 文档)。
  unsafe fn chars<'a>(&self) -> &'a [u8];
  /// Get (string-inl.h:1312)。
  ///
  /// # Safety
  /// 继承 trait 级 Safety 契约 (裸指针访问, 见 trait 文档)。
  unsafe fn get(&self, index: u32) -> u8;
  /// SeqOneByteStringSet (string-inl.h:1319; 值域 ≤ 0xFF debug 断言)。
  ///
  /// # Safety
  /// 继承 trait 级 Safety 契约 (裸指针访问, 见 trait 文档)。
  unsafe fn set(&self, index: u32, value: u16);
  /// SeqOneByteStringSetChars (string-inl.h:1327)。
  ///
  /// # Safety
  /// 继承 trait 级 Safety 契约 (裸指针访问, 见 trait 文档)。
  unsafe fn set_chars(&self, index: u32, src: &[u8]);
}

/// SeqTwoByteString 访问器 (string-inl.h:1359-1385)。
///
/// # Safety
/// 同 [SeqOneByteExt]; 布局为 SeqTwoByteString。
pub trait SeqTwoByteExt {
  /// GetChars (string-inl.h:1359)。
  ///
  /// # Safety
  /// 继承 trait 级 Safety 契约 (裸指针访问, 见 trait 文档)。
  unsafe fn chars<'a>(&self) -> &'a [u16];
  /// Get (string-inl.h:1374)。
  ///
  /// # Safety
  /// 继承 trait 级 Safety 契约 (裸指针访问, 见 trait 文档)。
  unsafe fn get(&self, index: u32) -> u16;
  /// SeqTwoByteStringSet (string-inl.h:1381)。
  ///
  /// # Safety
  /// 继承 trait 级 Safety 契约 (裸指针访问, 见 trait 文档)。
  unsafe fn set(&self, index: u32, value: u16);
}

/// ConsString 访问器 (string-inl.h:1441-1458 + string.cc:2135 Get)。
///
/// # Safety
/// 同 [StringExt]; 布局为 ConsString。
pub trait ConsExt {
  /// first (string-inl.h:1441)。
  ///
  /// # Safety
  /// 继承 trait 级 Safety 契约 (裸指针访问, 见 trait 文档)。
  unsafe fn first(&self) -> Tagged<String>;
  /// second (string-inl.h:1446)。
  ///
  /// # Safety
  /// 继承 trait 级 Safety 契约 (裸指针访问, 见 trait 文档)。
  unsafe fn second(&self) -> Tagged<String>;
  /// unchecked_second (string-inl.h:1454; Relaxed 读, GC 期免检路径)。
  ///
  /// # Safety
  /// 继承 trait 级 Safety 契约 (裸指针访问, 见 trait 文档)。
  unsafe fn unchecked_second(&self) -> Tagged<String>;
  /// IsFlat (string-inl.h:1458: second 空 = 已展平)。
  ///
  /// # Safety
  /// 继承 trait 级 Safety 契约 (裸指针访问, 见 trait 文档)。
  unsafe fn is_flat(&self) -> bool;
  /// Get (string.cc:2135: 展平左取/树内定位)。
  ///
  /// # Safety
  /// 继承 trait 级 Safety 契约 (裸指针访问, 见 trait 文档)。
  unsafe fn get(&self, index: u32) -> u16;
}

/// SlicedString 访问器 (string-inl.h:1428-1439)。
///
/// # Safety
/// 同 [StringExt]; 布局为 SlicedString。
pub trait SlicedExt {
  /// parent (string-inl.h:1428)。
  ///
  /// # Safety
  /// 继承 trait 级 Safety 契约 (裸指针访问, 见 trait 文档)。
  unsafe fn parent(&self) -> Tagged<String>;
  /// offset (string-inl.h:1435; Smi 解码)。
  ///
  /// # Safety
  /// 继承 trait 级 Safety 契约 (裸指针访问, 见 trait 文档)。
  unsafe fn offset(&self) -> i32;
}

/// ThinString 访问器 (string-inl.h:1460)。
///
/// # Safety
/// 同 [StringExt]; 布局为 ThinString。
pub trait ThinExt {
  /// actual (string-inl.h:1460)。
  ///
  /// # Safety
  /// 继承 trait 级 Safety 契约 (裸指针访问, 见 trait 文档)。
  unsafe fn actual(&self) -> Tagged<String>;
}

/// ExternalString 访问器 (string-inl.h:1489 resource_as_address + 1564/1628
/// GetChars 的底层槽)。
///
/// # Safety
/// 同 [StringExt]; 布局为对应编码的 cached ExternalString
/// (uncached 型资源协议归 v8-api/sandbox, 未承接)。
pub trait ExternalExt {
  /// resource 槽裸值 (string-inl.h:1489)。
  ///
  /// # Safety
  /// 继承 trait 级 Safety 契约 (裸指针访问, 见 trait 文档)。
  unsafe fn resource(&self) -> usize;
  /// resource_data 槽裸值 (cached 字符数据指针; string-inl.h:1534-1542)。
  ///
  /// # Safety
  /// 继承 trait 级 Safety 契约 (裸指针访问, 见 trait 文档)。
  unsafe fn resource_data(&self) -> usize;
}

// ── 简单访问器 impl (string-inl.h 直迁) ────────────────────────────────────
impl SeqOneByteExt for Tagged<SeqOneByteString> {
  unsafe fn chars<'a>(&self) -> &'a [u8] {
    // SAFETY: 布局契约; 字符尾在对象基址 + K_SEQ_CHARS_OFFSET, 长度字段定长
    unsafe {
      let s = self.as_ref();
      from_raw_parts(
        (self.address() + CHARS_OFFSET) as *const u8,
        s.string.length as usize,
      )
    }
  }

  unsafe fn get(&self, index: u32) -> u8 {
    // SAFETY: chars 契约 + 调用方索引域
    unsafe { *self.chars().as_ptr().add(index as usize) }
  }

  unsafe fn set(&self, index: u32, value: u16) {
    debug_assert!(u32::from(value) <= K_MAX_ONE_BYTE_CHAR_CODE);
    // SAFETY: chars 契约 + 索引域 (string-inl.h:1319 chars()[index] = value)
    unsafe {
      let chars = self.chars();
      chars
        .as_ptr()
        .add(index as usize)
        .cast_mut()
        .write(value as u8);
    }
  }

  unsafe fn set_chars(&self, index: u32, src: &[u8]) {
    // SAFETY: chars 契约 + [index, index+src.len()) 在段内 (string-inl.h:1327)
    unsafe {
      let chars = self.chars();
      copy_nonoverlapping(
        src.as_ptr(),
        chars.as_ptr().add(index as usize).cast_mut(),
        src.len(),
      );
    }
  }
}

impl SeqTwoByteExt for Tagged<SeqTwoByteString> {
  unsafe fn chars<'a>(&self) -> &'a [u16] {
    // SAFETY: 布局契约; u16 尾 (基址 8 对齐 ⇒ 2 对齐满足)
    unsafe {
      let s = self.as_ref();
      from_raw_parts(
        (self.address() + CHARS_OFFSET) as *const u16,
        s.string.length as usize,
      )
    }
  }

  unsafe fn get(&self, index: u32) -> u16 {
    // SAFETY: chars 契约 + 调用方索引域
    unsafe { *self.chars().as_ptr().add(index as usize) }
  }

  unsafe fn set(&self, index: u32, value: u16) {
    // SAFETY: chars 契约 + 索引域 (string-inl.h:1381)
    unsafe {
      let chars = self.chars();
      chars.as_ptr().add(index as usize).cast_mut().write(value);
    }
  }
}

impl ConsExt for Tagged<ConsString> {
  unsafe fn first(&self) -> Tagged<String> {
    // SAFETY: 布局契约 (string-inl.h:1441 first_.load())
    unsafe { self.as_ref().first.load() }
  }

  unsafe fn second(&self) -> Tagged<String> {
    // SAFETY: 布局契约 (string-inl.h:1446)
    unsafe { self.as_ref().second.load() }
  }

  unsafe fn unchecked_second(&self) -> Tagged<String> {
    // SAFETY: 布局契约 (string-inl.h:1454 second_.Relaxed_Load())
    unsafe { self.as_ref().second.relaxed_load() }
  }

  unsafe fn is_flat(&self) -> bool {
    // SAFETY: second 读契约 (string-inl.h:1458)
    unsafe { self.second().length() == 0 }
  }

  unsafe fn get(&self, index: u32) -> u16 {
    // string.cc:2135-2163
    // SAFETY: 子树访问契约
    unsafe {
      // 展平 cons: 左子树直取
      if self.second().length() == 0 {
        return self.first().get(index);
      }
      // 树内定位: 左子树长于 index 则下行, 否则扣减后转右
      let mut string = Tagged::<String>::new(self.ptr());
      let mut index = index;
      loop {
        if string.shape().is_cons() {
          let cons: Tagged<ConsString> = string.cast();
          let left = cons.first();
          if left.length() > index {
            string = left;
          } else {
            index -= left.length();
            string = cons.second();
          }
        } else {
          return string.get(index);
        }
      }
    }
  }
}

impl SlicedExt for Tagged<SlicedString> {
  unsafe fn parent(&self) -> Tagged<String> {
    // SAFETY: 布局契约 (string-inl.h:1428)
    unsafe { self.as_ref().parent.load() }
  }

  unsafe fn offset(&self) -> i32 {
    // SAFETY: 布局契约 (string-inl.h:1435 offset_.load().value())
    unsafe { Smi::value(self.as_ref().offset.load()) }
  }
}

impl ThinExt for Tagged<ThinString> {
  unsafe fn actual(&self) -> Tagged<String> {
    // SAFETY: 布局契约 (string-inl.h:1460)
    unsafe { self.as_ref().actual.load() }
  }
}

impl ExternalExt for Tagged<ExternalOneByteString> {
  unsafe fn resource(&self) -> usize {
    // SAFETY: 布局契约 (string-inl.h:1489)
    unsafe { self.as_ref().external.resource }
  }

  unsafe fn resource_data(&self) -> usize {
    // SAFETY: 布局契约; cached 字符数据裸指针 (string-inl.h:1534-1542)
    unsafe { self.as_ref().external.resource_data }
  }
}

impl ExternalExt for Tagged<ExternalTwoByteString> {
  unsafe fn resource(&self) -> usize {
    // SAFETY: 布局契约
    unsafe { self.as_ref().external.resource }
  }

  unsafe fn resource_data(&self) -> usize {
    // SAFETY: 布局契约
    unsafe { self.as_ref().external.resource_data }
  }
}

// @cc-semantics-begin
// 由 tool/src/objects/string.cc.js 从 v8/src/objects/string.cc (规则代码区段) 生成; 修订就地改, 复跑覆盖

// ── 位布局 const assert (string.cc:2418-2444; StringInstanceType 位段) ─────
// representation 3 bit | is_one_byte 1 bit | is_uncached 1 bit | is_shared 1 bit
// | is_not_internalized 1 bit (string.tq bitfield struct 1:1)
const _: () = assert!(K_STRING_REPRESENTATION_MASK == 0x7);
const _: () = assert!(K_STRING_ENCODING_MASK == 0x8);
const _: () = assert!(K_TWO_BYTE_STRING_TAG == 0);
const _: () = assert!(K_ONE_BYTE_STRING_TAG == 0x8);
const _: () = assert!(K_UNCACHED_EXTERNAL_STRING_MASK == 0x10);
const _: () = assert!(K_UNCACHED_EXTERNAL_STRING_TAG == 0x10);
const _: () = assert!(K_IS_NOT_INTERNALIZED_MASK == 0x40);
const _: () = assert!(K_NOT_INTERNALIZED_TAG == 0x40);
const _: () = assert!(K_INTERNALIZED_TAG == 0x0);
const _: () = assert!(K_SHARED_STRING_MASK == 0x20);
const _: () = assert!(K_SHARED_STRING_TAG == 0x20);

// ── 调试打印前后缀 (string.cc:589-623; if 链机械展开) ─────────────────────
/// 短打印前缀 (u 前缀 = 双字节表示)。
#[must_use]
pub fn prefix_for_debug_print(shape: StringShape) -> &'static str {
  if shape.is_two_byte() {
    if shape.is_internalized() {
      "u#"
    } else if shape.is_cons() {
      "uc\""
    } else if shape.is_thin() {
      "u>\""
    } else if shape.is_external() {
      "ue\""
    } else {
      "u\""
    }
  } else if shape.is_internalized() {
    "#"
  } else if shape.is_cons() {
    "c\""
  } else if shape.is_thin() {
    ">\""
  } else if shape.is_external() {
    "e\""
  } else {
    "\""
  }
}

/// 短打印后缀 (string.cc:619-623)。
#[must_use]
pub fn suffix_for_debug_print(shape: StringShape) -> &'static str {
  if shape.is_internalized() { "" } else { "\"" }
}

// ── ConsStringIterator (string.h:1416-1468 + string.cc:2226-2357) ─────────
/// ConsString 树迭代器: 全迭代可重启栈式遍历 (string.h:1416)。
pub struct ConsStringIterator {
  /// 栈帧 (右遍历未做的 ConsString)。
  frames: [Option<Tagged<ConsString>>; Self::STACK_SIZE],
  /// 根节点。
  root: Option<Tagged<ConsString>>,
  /// 当前深度。
  depth: i32,
  /// 深度峰值 (爆栈判定)。
  maximum_depth: i32,
  /// 已消费字符数。
  consumed: u32,
}

impl ConsStringIterator {
  /// 栈容量 (string.h:1443)。
  pub const STACK_SIZE: usize = 32;
  /// 深度掩码 (kStackSize - 1, 2 的幂)。
  pub const DEPTH_MASK: i32 = Self::STACK_SIZE as i32 - 1;

  /// OffsetForDepth (string-inl.h:1663)。
  #[inline]
  const fn offset_for_depth(depth: i32) -> usize {
    (depth & Self::DEPTH_MASK) as usize
  }

  /// PushLeft (string-inl.h:1665)。
  #[inline]
  fn push_left(&mut self, string: Tagged<ConsString>) {
    self.frames[Self::offset_for_depth(self.depth)] = Some(string);
    self.depth += 1;
  }

  /// PushRight 原地更新栈顶 (string-inl.h:1669)。
  #[inline]
  fn push_right(&mut self, string: Tagged<ConsString>) {
    self.frames[Self::offset_for_depth(self.depth - 1)] = Some(string);
  }

  /// AdjustMaximumDepth (string-inl.h:1674)。
  #[inline]
  fn adjust_maximum_depth(&mut self) {
    if self.depth > self.maximum_depth {
      self.maximum_depth = self.depth;
    }
  }

  /// Pop (string-inl.h:1678)。
  #[inline]
  fn pop(&mut self) {
    debug_assert!(self.depth > 0);
    debug_assert!(self.depth <= self.maximum_depth);
    self.depth -= 1;
  }

  /// StackBlown: maximum_depth - depth == kStackSize (string.h:1454)。
  #[inline]
  fn stack_blown(&self) -> bool {
    self.maximum_depth - self.depth == Self::STACK_SIZE as i32
  }

  /// 构造 + Reset (string.h:1419-1430; None → 迭代器终态)。
  #[must_use]
  pub fn new(cons_string: Option<Tagged<ConsString>>, offset: u32) -> Self {
    let mut it = Self {
      frames: [None; Self::STACK_SIZE],
      root: None,
      depth: 0,
      maximum_depth: 0,
      consumed: 0,
    };
    it.reset(cons_string, offset);
    it
  }

  /// Reset (string.h:1425): 清栈; 空根 → next 恒 None。
  pub fn reset(&mut self, cons_string: Option<Tagged<ConsString>>, offset: u32) {
    self.depth = 0;
    if let Some(cons) = cons_string {
      self.initialize(cons, offset);
    }
  }

  /// Next (string.h:1436): 产出下一个叶段; None 表示遍历完成。
  /// `offset_out` 为段内起始偏移 (仅 Reset 带偏移后首段非零)。
  pub fn next(&mut self, offset_out: &mut u32) -> Option<Tagged<String>> {
    *offset_out = 0;
    if self.depth == 0 {
      return None;
    }
    // SAFETY: 树访问契约 (string.h:1436; 调用方持对象存活保证)
    unsafe { self.continue_(offset_out) }
  }

  /// Initialize (string.cc:2226-2235): 记根 + 强制爆栈态触发重启搜索。
  fn initialize(&mut self, cons_string: Tagged<ConsString>, offset: u32) {
    self.root = Some(cons_string);
    self.consumed = offset;
    self.depth = 1;
    self.maximum_depth = Self::STACK_SIZE as i32 + self.depth;
    debug_assert!(self.stack_blown());
  }

  /// Continue (string.cc:2237-2252): 取下一叶; 爆栈则从根重启搜索。
  ///
  /// # Safety
  /// 继承 trait 级 Safety 契约 (裸指针访问, 见 trait 文档)。
  unsafe fn continue_(&mut self, offset_out: &mut u32) -> Option<Tagged<String>> {
    // SAFETY: 树访问契约 (string.cc:2237)
    unsafe {
      debug_assert_ne!(self.depth, 0);
      debug_assert_eq!(*offset_out, 0);
      let mut blew_stack = self.stack_blown();
      let mut string = None;
      if !blew_stack {
        string = self.next_leaf(&mut blew_stack);
      }
      if blew_stack {
        debug_assert!(string.is_none());
        string = self.search(offset_out);
      }
      // 叶为空 → 终态 (Reset({}) 语义)
      if string.is_none() {
        self.reset(None, 0);
      }
      string
    }
  }

  /// Search (string.cc:2254-2311): 从根按已消费数定位包含目标偏移的叶。
  ///
  /// # Safety
  /// 继承 trait 级 Safety 契约 (裸指针访问, 见 trait 文档)。
  unsafe fn search(&mut self, offset_out: &mut u32) -> Option<Tagged<String>> {
    // SAFETY: 树访问契约 (string.cc:2254; 调用方 continue_ 已持 unsafe 语境)
    unsafe {
      let mut cons_string = self.root?;
      self.depth = 1;
      self.maximum_depth = 1;
      self.frames[0] = Some(cons_string);
      let consumed = self.consumed;
      let mut offset = 0u32;
      loop {
        // 定位包含目标偏移的分支
        let mut string = cons_string.first();
        let mut length = string.length();
        if consumed < offset + length {
          // 目标在左支: 仍为 Cons 则继续下行
          if string.shape().is_cons() {
            cons_string = string.cast();
            self.push_left(cons_string);
            continue;
          }
          self.adjust_maximum_depth();
        } else {
          // 下行右支: 推进偏移
          offset += length;
          string = cons_string.second();
          if string.shape().is_cons() {
            cons_string = string.cast();
            self.push_right(cons_string);
            continue;
          }
          length = string.length();
          // 右叶为空: 目标偏移越界 → 终态
          if length == 0 {
            self.reset(None, 0);
            return None;
          }
          self.adjust_maximum_depth();
          self.pop();
        }
        debug_assert_ne!(length, 0);
        self.consumed = offset + length;
        *offset_out = consumed - offset;
        return Some(string);
      }
    }
  }

  /// NextLeaf (string.cc:2313-2357): 沿栈右行, 爆栈报告 true。
  ///
  /// # Safety
  /// 继承 trait 级 Safety 契约 (裸指针访问, 见 trait 文档)。
  unsafe fn next_leaf(&mut self, blew_stack: &mut bool) -> Option<Tagged<String>> {
    // SAFETY: 树访问契约 (string.cc:2313)
    unsafe {
      loop {
        if self.depth == 0 {
          *blew_stack = false;
          return None;
        }
        if self.stack_blown() {
          *blew_stack = true;
          return None;
        }
        // 右行
        let cons_string = self.frames[Self::offset_for_depth(self.depth - 1)]?;
        let mut string = cons_string.second();
        if !string.shape().is_cons() {
          self.pop();
          let length = string.length();
          // 可能是展平 Cons 的空右叶
          if length == 0 {
            continue;
          }
          self.consumed += length;
          return Some(string);
        }
        let mut cons = string.cast::<ConsString>();
        self.push_right(cons);
        // 全左下行
        loop {
          string = cons.first();
          if !string.shape().is_cons() {
            self.adjust_maximum_depth();
            let length = string.length();
            // 跳过 Cons 左侧空叶
            if length == 0 {
              break;
            }
            self.consumed += length;
            return Some(string);
          }
          cons = string.cast();
          self.push_left(cons);
        }
      }
    }
  }
}

// ── StringExt 主实现 (string-inl.h 内联 + string.cc 慢路径) ───────────────
impl StringExt for Tagged<String> {
  unsafe fn length(&self) -> u32 {
    // SAFETY: 布局契约 (string-inl.h:140)
    unsafe { self.as_ref().length }
  }

  unsafe fn raw_hash_field(&self) -> u32 {
    // SAFETY: 布局契约
    unsafe { self.as_ref().name.raw_hash_field }
  }

  unsafe fn try_get_hash(&self) -> Option<u32> {
    // SAFETY: raw_hash_field 契约 (name-inl.h:243 TryGetHash)
    unsafe {
      let field = self.raw_hash_field();
      Name::is_hash_field_computed(field).then_some((field >> 2) & 0x3FFF_FFFF)
    }
  }

  unsafe fn set_raw_hash_field_if_empty(&self, hash: u32) {
    // SAFETY: 布局契约; 仅空字段写入 (string.cc:1940 compare_exchange 的
    // 「失败不覆盖」语义承接, 原子性归宿主堆层)
    unsafe {
      let field = &mut (*self.as_mut_ptr()).name.raw_hash_field;
      if *field == EMPTY_HASH_FIELD {
        *field = hash;
      }
    }
  }

  unsafe fn shape(&self) -> StringShape {
    // SAFETY: 布局契约 (string-inl.h:160)
    unsafe { StringShape::of_map(self.as_ref().name.header.map) }
  }

  unsafe fn get(&self, index: u32) -> u16 {
    // SAFETY: GetImpl 派发 (string-inl.h:1149); 各叶访问契约成立
    unsafe {
      match StringKind::of(self.shape().instance_type()) {
        StringKind::SeqOneByte => u16::from(self.cast::<SeqOneByteString>().get(index)),
        StringKind::SeqTwoByte => self.cast::<SeqTwoByteString>().get(index),
        StringKind::ExternalOneByte => {
          let s = self.cast::<ExternalOneByteString>().as_ref();
          let chars = from_raw_parts(
            s.external.resource_data as *const u8,
            s.external.string.length as usize,
          );
          u16::from(chars[index as usize])
        }
        StringKind::ExternalTwoByte => {
          let s = self.cast::<ExternalTwoByteString>().as_ref();
          let chars = from_raw_parts(
            s.external.resource_data as *const u16,
            s.external.string.length as usize,
          );
          chars[index as usize]
        }
        StringKind::Cons => self.cast::<ConsString>().get(index),
        StringKind::Sliced => {
          let s = self.cast::<SlicedString>();
          let off = s.offset() as u32;
          s.parent().get(off + index)
        }
        StringKind::Thin => self.cast::<ThinString>().actual().get(index),
      }
    }
  }

  unsafe fn equals(&self, other: Tagged<String>) -> bool {
    // SAFETY: 慢路径契约 (string-inl.h:680)
    unsafe {
      if other.ptr() == self.ptr() {
        return true;
      }
      // 双内部化不同指针: 内部化唯一性不变量 → false
      if self.shape().is_internalized() && other.shape().is_internalized() {
        return false;
      }
      slow_equals(self, other)
    }
  }

  unsafe fn is_equal_to_u8(&self, data: &[u8], ty: EqualityType) -> bool {
    // SAFETY: 同 is_equal_to_impl
    unsafe { is_equal_to_impl(Tagged::new(self.ptr()), data, ty) }
  }

  unsafe fn is_equal_to_u16(&self, data: &[u16], ty: EqualityType) -> bool {
    // SAFETY: 同上
    unsafe { is_equal_to_impl(Tagged::new(self.ptr()), data, ty) }
  }

  unsafe fn is_one_byte_representation_underneath(&self) -> bool {
    // SAFETY: get_underlying 契约 (string-inl.h:529; 前置 IsFlat)
    unsafe {
      let mut string = Tagged::<String>::new(self.ptr());
      loop {
        match StringKind::of(string.shape().instance_type()) {
          StringKind::SeqOneByte | StringKind::ExternalOneByte => return true,
          StringKind::SeqTwoByte | StringKind::ExternalTwoByte => return false,
          // Cons/Sliced/Thin 穿透 (GetUnderlying; string-inl.h:1177)
          StringKind::Cons => string = string.cast::<ConsString>().first(),
          StringKind::Sliced => string = string.cast::<SlicedString>().parent(),
          StringKind::Thin => string = string.cast::<ThinString>().actual(),
        }
      }
    }
  }

  unsafe fn get_flat_content(&self) -> FlatContent<'_> {
    // SAFETY: TryGetFlatContentFromDirectString + SlowGetFlatContent
    // (string-inl.h:1026-1086 + string.cc:695-730)
    unsafe {
      let length = self.length();
      let direct = Self::try_flat_content_of_direct(Tagged::new(self.ptr()), 0, length);
      if direct.is_flat() {
        return direct;
      }
      let mut string = Tagged::<String>::new(self.ptr());
      let mut offset = 0u32;
      let mut shape = string.shape();
      if shape.is_cons() {
        let cons: Tagged<ConsString> = string.cast();
        if !cons.is_flat() {
          return FlatContent::NonFlat;
        }
        string = cons.first();
        shape = string.shape();
      }
      // SlicedString 链循环穿透 (原版单层 + 「sliced 不嵌套」不变量;
      // 防御穿透, 合法输入下单轮即出 — 与 VisitFlat 同款承接)
      while shape.is_sliced() {
        let slice: Tagged<SlicedString> = string.cast();
        offset += slice.offset() as u32;
        string = slice.parent();
        shape = string.shape();
      }
      if shape.is_thin() {
        string = string.cast::<ThinString>().actual();
      }
      Self::try_flat_content_of_direct(string, offset, length)
    }
  }

  unsafe fn compute_and_set_raw_hash(
    &self,
    out_one_byte_content: Option<&mut bool>,
    seed: &HashSeedData,
  ) -> Result<u32> {
    // SAFETY: 解包与哈希路径契约 (string.cc:1954-2005)
    unsafe {
      let mut start = 0u32;
      let mut string = Tagged::<String>::new(self.ptr());
      let mut shape = string.shape();
      if shape.is_sliced() {
        let sliced = string.cast::<SlicedString>();
        start = sliced.offset() as u32;
        string = sliced.parent();
        shape = string.shape();
      }
      if shape.is_cons() && string.cast::<ConsString>().is_flat() {
        string = string.cast::<ConsString>().first();
        shape = string.shape();
      }
      if shape.is_thin() {
        string = string.cast::<ThinString>().actual();
        shape = string.shape();
        if self.length() == string.length() {
          // 等长 Thin: 复用 actual 哈希 (string.cc:1984; 不跑 hasher)
          let raw_hash = string.raw_hash_field();
          debug_assert!(Name::is_hash_field_computed(raw_hash));
          self.set_raw_hash_field_if_empty(raw_hash);
          return Ok(raw_hash);
        }
      }
      let raw = if shape.is_one_byte() {
        hash_string_u8(string, start, self.length(), seed, out_one_byte_content)?
      } else {
        hash_string_u16(string, start, self.length(), seed, out_one_byte_content)?
      };
      self.set_raw_hash_field_if_empty(raw);
      Ok(raw)
    }
  }

  unsafe fn as_array_index(&self) -> Option<u32> {
    // SAFETY: 快慢路径契约 (string-inl.h:1815 + string.cc:2007)
    unsafe {
      let field = self.raw_hash_field();
      if contains_cached_array_index(field) {
        return Some(decode_array_index_from_hash_field(field));
      }
      if Name::is_hash_field_computed(field) && !is_integer_index(field) {
        return None;
      }
      slow_as_array_index(self)
    }
  }

  unsafe fn as_integer_index(&self) -> Option<usize> {
    // SAFETY: 快慢路径契约 (string-inl.h:1829 + string.cc:2022)
    unsafe {
      let field = self.raw_hash_field();
      if contains_cached_array_index(field) {
        return Some(u64::from(decode_array_index_from_hash_field(field)) as usize);
      }
      if Name::is_hash_field_computed(field) && !is_integer_index(field) {
        return None;
      }
      slow_as_integer_index(self)
    }
  }

  unsafe fn try_flat_content_of_direct<'a>(
    string: Tagged<String>,
    offset: u32,
    length: u32,
  ) -> FlatContent<'a> {
    // SAFETY: 各叶访问契约 (string-inl.h:1002-1024)
    unsafe {
      debug_assert!(offset + length <= string.length());
      match StringKind::of(string.shape().instance_type()) {
        StringKind::SeqOneByte => {
          let chars = string.cast::<SeqOneByteString>().chars();
          FlatContent::OneByte(&chars[offset as usize..][..length as usize])
        }
        StringKind::SeqTwoByte => {
          let chars = string.cast::<SeqTwoByteString>().chars();
          FlatContent::TwoByte(&chars[offset as usize..][..length as usize])
        }
        StringKind::ExternalOneByte => {
          let s = string.cast::<ExternalOneByteString>().as_ref();
          let chars = from_raw_parts(
            s.external.resource_data as *const u8,
            s.external.string.length as usize,
          );
          FlatContent::OneByte(&chars[offset as usize..][..length as usize])
        }
        StringKind::ExternalTwoByte => {
          let s = string.cast::<ExternalTwoByteString>().as_ref();
          let chars = from_raw_parts(
            s.external.resource_data as *const u16,
            s.external.string.length as usize,
          );
          FlatContent::TwoByte(&chars[offset as usize..][..length as usize])
        }
        // 间接串无直接内容 (std::nullopt → NonFlat)
        _ => FlatContent::NonFlat,
      }
    }
  }
}

// ── 慢路径私有助手 (string.cc; Tagged 固有 impl 受孤儿规则限制, 落自由函数) ──
/// SlowEquals (string.cc:1310-1353)。
///
/// # Safety
/// 同 [StringExt]。
unsafe fn slow_equals(s: &Tagged<String>, other: Tagged<String>) -> bool {
  unsafe {
    let len = s.length();
    if len != other.length() {
      return false;
    }
    if len == 0 {
      return true;
    }
    // Thin 参与则解引用重启 (shape 单次读取复用)
    let s_thin = s.shape().is_thin();
    let other_thin = other.shape().is_thin();
    if s_thin || other_thin {
      let other = if other_thin {
        other.cast::<ThinString>().actual()
      } else {
        other
      };
      if s_thin {
        return s.cast::<ThinString>().actual().equals(other);
      }
      return s.equals(other);
    }
    // 哈希快否 (双方已计算才比较)
    if let (Some(h1), Some(h2)) = (s.try_get_hash(), other.try_get_hash())
      && h1 != h2
    {
      return false;
    }
    slow_equals_non_thin_same_length(s, len, other)
  }
}

/// SlowEqualsNonThinSameLength (string.cc:1363-1385)。
///
/// # Safety
/// 同 [StringExt]; len ≠ 0 且双方等长非 Thin。
unsafe fn slow_equals_non_thin_same_length(
  s: &Tagged<String>,
  len: u32,
  other: Tagged<String>,
) -> bool {
  unsafe {
    debug_assert_ne!(len, 0);
    // 首字符先比
    if s.get(0) != other.get(0) {
      return false;
    }
    // Seq+Seq 快路径 (CompareCharsEqual)
    let s1 = s.shape();
    let s2 = other.shape();
    if s1.is_sequential_one_byte() && s2.is_sequential_one_byte() {
      return s.cast::<SeqOneByteString>().chars() == other.cast::<SeqOneByteString>().chars();
    }
    // StringComparator 分段状态机 (string-comparator.cc:50)
    let mut a = StringSegmentSource::new(Tagged::new(s.ptr()));
    let mut b = StringSegmentSource::new(other);
    StringComparator.equals(&mut a, &mut b, len)
  }
}

/// SlowAsArrayIndex (string.cc:2007-2020)。
///
/// # Safety
/// 同 [StringExt]。
unsafe fn slow_as_array_index(s: &Tagged<String>) -> Option<u32> {
  unsafe {
    let length = s.length();
    if length <= MAX_CACHED_ARRAY_INDEX_LENGTH {
      // EnsureRawHash: 未计算则补算 (seed 默认; 宿主接 roots 后改注入)
      let field = if Name::is_hash_field_computed(s.raw_hash_field()) {
        s.raw_hash_field()
      } else {
        s.compute_and_set_raw_hash(None, &HashSeedData::DEFAULT)
          .ok()?
      };
      if !is_integer_index(field) {
        return None;
      }
      return Some(decode_array_index_from_hash_field(field));
    }
    if length == 0 || length > MAX_ARRAY_INDEX_SIZE {
      return None;
    }
    let mut stream = StringCharacterStream::new(Tagged::new(s.ptr()), 0);
    string_to_index_array(&mut stream)
  }
}

/// SlowAsIntegerIndex (string.cc:2022-2036)。
///
/// # Safety
/// 同 [StringExt]。
unsafe fn slow_as_integer_index(s: &Tagged<String>) -> Option<usize> {
  unsafe {
    let length = s.length();
    if length <= MAX_CACHED_ARRAY_INDEX_LENGTH {
      let field = if Name::is_hash_field_computed(s.raw_hash_field()) {
        s.raw_hash_field()
      } else {
        s.compute_and_set_raw_hash(None, &HashSeedData::DEFAULT)
          .ok()?
      };
      if !is_integer_index(field) {
        return None;
      }
      return Some(u64::from(decode_array_index_from_hash_field(field)) as usize);
    }
    if length == 0 || length > MAX_INTEGER_INDEX_SIZE {
      return None;
    }
    let mut stream = StringCharacterStream::new(Tagged::new(s.ptr()), 0);
    string_to_index_integer(&mut stream)
  }
}

// ── StringComparator 段源适配 (string-comparator.h 的 String 绑定) ─────────
/// Tagged<String> 分段源: direct 单段, Cons 树经迭代器产段。
struct StringSegmentSource {
  /// 预产的首段 (direct string 直取)。
  first: Option<FlatSegment>,
  /// Cons 驱动迭代器 (direct string 时为终态)。
  iter: ConsStringIterator,
}

impl StringSegmentSource {
  /// 构造 (先试直接路径, 间接串交迭代器)。
  ///
  /// # Safety
  /// string 须为有效字符串对象, 且段源存活期内字符内存稳定。
  unsafe fn new(string: Tagged<String>) -> Self {
    unsafe {
      if string.shape().is_cons() {
        return Self {
          first: None,
          iter: ConsStringIterator::new(Some(string.cast()), 0),
        };
      }
      // direct/sliced/thin: VisitFlat 同款循环穿透 (string-inl.h:1198-1242;
      // offset 随 sliced 链累积 — 原版靠「sliced 不嵌套」不变量, 此处防御穿透)
      let mut s = string;
      let mut offset = 0u32;
      loop {
        let shape = s.shape();
        if shape.is_sliced() {
          let sliced = s.cast::<SlicedString>();
          offset += sliced.offset() as u32;
          s = sliced.parent();
        } else if shape.is_thin() {
          s = s.cast::<ThinString>().actual();
        } else {
          break;
        }
      }
      let flat =
        <Tagged<String> as StringExt>::try_flat_content_of_direct(s, offset, string.length());
      let first = match flat {
        f if f.is_one_byte() => Some(FlatSegment::one_byte(f.to_one_byte_vector())),
        f if f.is_two_byte() => Some(FlatSegment::two_byte(f.to_uc16_vector())),
        _ => None,
      };
      Self {
        first,
        iter: ConsStringIterator::new(None, 0),
      }
    }
  }
}

impl SegmentSource for StringSegmentSource {
  fn next_segment(&mut self) -> Option<FlatSegment> {
    if let Some(seg) = self.first.take() {
      return Some(seg);
    }
    let mut offset = 0u32;
    let seg = self.iter.next(&mut offset)?;
    // SAFETY: 段内存稳定契约 (构造方保证)
    let flat = unsafe { seg.get_flat_content() };
    if flat.is_one_byte() {
      Some(FlatSegment::one_byte(flat.to_one_byte_vector()))
    } else if flat.is_two_byte() {
      Some(FlatSegment::two_byte(flat.to_uc16_vector()))
    } else {
      None
    }
  }
}

// ── IsEqualToImpl (string-inl.h:730-820) ──────────────────────────────────
/// 三型长度前置 + 间接串穿透 + Cons 离线比对。
///
/// # Safety
/// s 须为有效字符串对象。
unsafe fn is_equal_to_impl<C: CharData>(s: Tagged<String>, data: &[C], ty: EqualityType) -> bool {
  unsafe {
    let len = data.len();
    match ty {
      EqualityType::WholeString => {
        if s.length() as usize != len {
          return false;
        }
      }
      EqualityType::Prefix => {
        if (s.length() as usize) < len {
          return false;
        }
      }
      EqualityType::NoLengthCheck => debug_assert_eq!(s.length() as usize, len),
    }
    let mut slice_offset = 0u32;
    let mut string = s;
    loop {
      match StringKind::of(string.shape().instance_type()) {
        StringKind::SeqOneByte => {
          // SAFETY: chars 契约
          let chars = string.cast::<SeqOneByteString>().chars();
          return cmp_chars_equal(&chars[slice_offset as usize..][..len], data);
        }
        StringKind::SeqTwoByte => {
          // SAFETY: chars 契约
          let chars = string.cast::<SeqTwoByteString>().chars();
          return cmp_chars_equal(&chars[slice_offset as usize..][..len], data);
        }
        StringKind::ExternalOneByte => {
          let ext = string.cast::<ExternalOneByteString>().as_ref();
          // SAFETY: resource_data 契约
          let chars = from_raw_parts(
            ext.external.resource_data as *const u8,
            ext.external.string.length as usize,
          );
          return cmp_chars_equal(&chars[slice_offset as usize..][..len], data);
        }
        StringKind::ExternalTwoByte => {
          let ext = string.cast::<ExternalTwoByteString>().as_ref();
          // SAFETY: resource_data 契约
          let chars = from_raw_parts(
            ext.external.resource_data as *const u16,
            ext.external.string.length as usize,
          );
          return cmp_chars_equal(&chars[slice_offset as usize..][..len], data);
        }
        StringKind::Sliced => {
          // SAFETY: offset/parent 契约
          slice_offset += string.cast::<SlicedString>().offset() as u32;
          string = string.cast::<SlicedString>().parent();
        }
        StringKind::Thin => {
          // SAFETY: actual 契约
          string = string.cast::<ThinString>().actual();
        }
        // Cons 路径离线 (string-inl.h:773-779)
        StringKind::Cons => return is_cons_string_equal_to_impl(string.cast(), data, ty),
      }
    }
  }
}

/// IsConsStringEqualToImpl (string-inl.h:791-820): 迭代器分段比对。
///
/// # Safety
/// string 须为有效 ConsString。
unsafe fn is_cons_string_equal_to_impl<C: CharData>(
  string: Tagged<ConsString>,
  data: &[C],
  _ty: EqualityType,
) -> bool {
  unsafe {
    let mut iter = ConsStringIterator::new(Some(string), 0);
    let mut consumed = 0usize;
    let mut offset = 0u32;
    while let Some(segment) = iter.next(&mut offset) {
      debug_assert_eq!(offset, 0);
      let remaining = &data[consumed..];
      let len = (segment.length() as usize).min(remaining.len());
      // Prefix 承接「长度已检」语义 (原版 kNoLengthCheck 的免检意图;
      // 段长可大于剩余子串, NoLengthCheck 的 debug 断言在此合法路径会误伤)
      if !is_equal_to_impl(segment, &remaining[..len], EqualityType::Prefix) {
        return false;
      }
      consumed += len;
      if consumed >= data.len() {
        break;
      }
    }
    consumed == data.len()
  }
}

/// 比较数据字符 (源/目标独立字符域, 宽化至 u16; CompareCharsEqual)。
fn cmp_chars_equal<S: CharData, P: CharData>(chars: &[S], data: &[P]) -> bool {
  chars.len() == data.len()
    && chars
      .iter()
      .zip(data.iter())
      .all(|(&a, &b)| a.widen() == b.widen())
}

/// IsEqualTo 数据字符域 (u8/u16 宽化至 u16 比较)。
trait CharData: Copy {
  /// 宽化比较键。
  fn widen(self) -> u16;
}

impl CharData for u8 {
  fn widen(self) -> u16 {
    u16::from(self)
  }
}

impl CharData for u16 {
  fn widen(self) -> u16 {
    self
  }
}

// ── HashString (string.cc:1898-1923) ──────────────────────────────────────
/// u8 域哈希: 长 > kMaxHashCalcLength 按长度; 非展平 Cons 经 WriteToFlat 缓冲。
///
/// # Safety
/// string 须为有效字符串对象。
unsafe fn hash_string_u8(
  string: Tagged<String>,
  start: u32,
  length: u32,
  seed: &HashSeedData,
  out_one_byte_content: Option<&mut bool>,
) -> Result<u32> {
  unsafe {
    if length > K_MAX_HASH_CALC_LENGTH {
      return Ok(get_trivial_hash(length));
    }
    if string.shape().is_cons() {
      debug_assert_eq!(0, start);
      debug_assert!(!string.cast::<ConsString>().is_flat());
      let mut buffer = vec![0u8; length as usize];
      write_to_flat(string, &mut buffer, 0, length)?;
      Ok(hash_sequential_one_byte(
        &buffer,
        seed,
        out_one_byte_content,
      ))
    } else {
      // SAFETY: direct chars (GetDirectStringChars<Char>)
      let chars = direct_chars_u8(string)?.add(start as usize);
      let chars = from_raw_parts(chars, length as usize);
      Ok(hash_sequential_one_byte(chars, seed, out_one_byte_content))
    }
  }
}

/// u16 域哈希。
///
/// # Safety
/// 同 [hash_string_u8]。
unsafe fn hash_string_u16(
  string: Tagged<String>,
  start: u32,
  length: u32,
  seed: &HashSeedData,
  out_one_byte_content: Option<&mut bool>,
) -> Result<u32> {
  unsafe {
    if length > K_MAX_HASH_CALC_LENGTH {
      return Ok(get_trivial_hash(length));
    }
    if string.shape().is_cons() {
      debug_assert_eq!(0, start);
      debug_assert!(!string.cast::<ConsString>().is_flat());
      let mut buffer = vec![0u16; length as usize];
      write_to_flat(string, &mut buffer, 0, length)?;
      Ok(hash_sequential_two_byte(
        &buffer,
        seed,
        out_one_byte_content,
      ))
    } else {
      // SAFETY: direct chars
      let chars = direct_chars_u16(string)?.add(start as usize);
      let chars = from_raw_parts(chars, length as usize);
      Ok(hash_sequential_two_byte(chars, seed, out_one_byte_content))
    }
  }
}

/// GetDirectStringChars<u8> (string-inl.h:824-851)。
///
/// # Safety
/// string 须为单字节 direct string。
unsafe fn direct_chars_u8(string: Tagged<String>) -> Result<*const u8> {
  unsafe {
    match StringKind::of(string.shape().instance_type()) {
      StringKind::SeqOneByte => Ok(string.cast::<SeqOneByteString>().chars().as_ptr()),
      StringKind::ExternalOneByte => Ok(
        string
          .cast::<ExternalOneByteString>()
          .as_ref()
          .external
          .resource_data as *const u8,
      ),
      _ => Err(Error::InvalidDispatch("direct_chars_u8 非单字节 direct")),
    }
  }
}

/// GetDirectStringChars<u16>。
///
/// # Safety
/// string 须为双字节 direct string。
unsafe fn direct_chars_u16(string: Tagged<String>) -> Result<*const u16> {
  unsafe {
    match StringKind::of(string.shape().instance_type()) {
      StringKind::SeqTwoByte => Ok(string.cast::<SeqTwoByteString>().chars().as_ptr()),
      StringKind::ExternalTwoByte => Ok(
        string
          .cast::<ExternalTwoByteString>()
          .as_ref()
          .external
          .resource_data as *const u16,
      ),
      _ => Err(Error::InvalidDispatch("direct_chars_u16 非双字节 direct")),
    }
  }
}

// ── StringCharacterStream (string-inl.h:1684-1813) ────────────────────────
/// 逐字符消费流: Cons 树迭代 + 段缓冲 (指针推进)。
pub struct StringCharacterStream {
  /// Cons 驱动。
  iter: ConsStringIterator,
  /// 当前段是否单字节。
  is_one_byte: bool,
  /// 段内游标 (字节指针; 原版 union buffer8_/buffer16_ 同址内存的统一承接,
  /// 到尾判定与推进均在字节域, 按编码解释读取)。
  cursor: *const u8,
  /// 段尾 (字节指针)。
  end: *const u8,
}

impl StringCharacterStream {
  /// 构造 + Reset (string-inl.h:1728)。
  ///
  /// # Safety
  /// string 须为有效字符串对象, 流存活期内字符内存稳定。
  pub unsafe fn new(string: Tagged<String>, offset: u32) -> Self {
    let mut stream = Self {
      iter: ConsStringIterator::new(None, 0),
      is_one_byte: false,
      cursor: null(),
      end: null(),
    };
    // SAFETY: 委托 reset
    unsafe { stream.reset(string, offset) };
    stream
  }

  /// GetNext (string-inl.h:1717): 游标到尾先续段。
  ///
  /// # Safety
  /// 段内存稳定契约。
  pub unsafe fn get_next(&mut self) -> u16 {
    // SAFETY: has_more 续段契约
    unsafe {
      if self.cursor == self.end {
        self.has_more();
      }
      debug_assert!(self.cursor < self.end);
      if self.is_one_byte {
        let c = u16::from(*self.cursor);
        self.cursor = self.cursor.add(1);
        c
      } else {
        let c = *(self.cursor as *const u16);
        self.cursor = self.cursor.add(2);
        c
      }
    }
  }

  /// HasMore (string-inl.h:1749): 缓冲有剩余或能续段。
  ///
  /// # Safety
  /// 段内存稳定契约。
  pub unsafe fn has_more(&mut self) -> bool {
    // SAFETY: 迭代器与 VisitFlat 契约
    unsafe {
      if self.cursor != self.end {
        return true;
      }
      let mut offset = 0u32;
      let Some(string) = self.iter.next(&mut offset) else {
        return false;
      };
      debug_assert_eq!(offset, 0);
      self.visit_flat(string, 0);
      debug_assert!(self.cursor != self.end);
      true
    }
  }

  /// Reset (string-inl.h:1734): 重建游标与迭代器。
  ///
  /// # Safety
  /// 同 [Self::new]。
  pub unsafe fn reset(&mut self, string: Tagged<String>, offset: u32) {
    // SAFETY: visit_flat + 迭代器契约
    unsafe {
      self.cursor = null();
      self.end = null();
      let cons_string = visit_flat(self, string, offset);
      self.iter.reset(cons_string, offset);
      if cons_string.is_some() {
        let mut offset = offset;
        if let Some(string) = self.iter.next(&mut offset) {
          self.visit_flat(string, offset);
        }
      }
    }
  }

  /// VisitOneByteString (string-inl.h:1760)。
  fn visit_one_byte(&mut self, chars: &[u8]) {
    self.is_one_byte = true;
    self.cursor = chars.as_ptr();
    // SAFETY: 段长域内指针运算
    self.end = unsafe { chars.as_ptr().add(chars.len()) };
  }

  /// VisitTwoByteString (string-inl.h:1767)。
  fn visit_two_byte(&mut self, chars: &[u16]) {
    self.is_one_byte = false;
    self.cursor = chars.as_ptr().cast();
    // SAFETY: 段长域内指针运算
    self.end = unsafe { chars.as_ptr().add(chars.len()).cast() };
  }

  /// VisitFlat 通用路径: 穿透 Sliced/Thin 后按段类型设缓冲, Cons 返回自身。
  ///
  /// # Safety
  /// string 须为有效字符串对象。
  unsafe fn visit_flat(&mut self, string: Tagged<String>, mut slice_offset: u32) {
    // SAFETY: 各叶访问契约 (string-inl.h:1198-1242)
    unsafe {
      let mut string = string;
      loop {
        match StringKind::of(string.shape().instance_type()) {
          StringKind::SeqOneByte => {
            let chars = string.cast::<SeqOneByteString>().chars();
            self.visit_one_byte(&chars[slice_offset as usize..]);
            return;
          }
          StringKind::SeqTwoByte => {
            let chars = string.cast::<SeqTwoByteString>().chars();
            self.visit_two_byte(&chars[slice_offset as usize..]);
            return;
          }
          StringKind::ExternalOneByte => {
            let ext = string.cast::<ExternalOneByteString>().as_ref();
            let chars = from_raw_parts(
              ext.external.resource_data as *const u8,
              ext.external.string.length as usize,
            );
            self.visit_one_byte(&chars[slice_offset as usize..]);
            return;
          }
          StringKind::ExternalTwoByte => {
            let ext = string.cast::<ExternalTwoByteString>().as_ref();
            let chars = from_raw_parts(
              ext.external.resource_data as *const u16,
              ext.external.string.length as usize,
            );
            self.visit_two_byte(&chars[slice_offset as usize..]);
            return;
          }
          StringKind::Sliced => {
            slice_offset += string.cast::<SlicedString>().offset() as u32;
            string = string.cast::<SlicedString>().parent();
          }
          StringKind::Thin => string = string.cast::<ThinString>().actual(),
          StringKind::Cons => return,
        }
      }
    }
  }

  /// CountUtf8Bytes (string-inl.h:1774): 前进 n 字符并累计 UTF-8 字节数。
  ///
  /// # Safety
  /// 同 [Self::new]。
  pub unsafe fn count_utf8_bytes(&mut self, n_chars: u32) -> usize {
    // SAFETY: get_next 契约
    unsafe {
      let mut utf8_bytes = 0usize;
      let mut remaining = n_chars;
      let mut last = NO_PREVIOUS_CHARACTER;
      while remaining != 0 && self.has_more() {
        remaining -= 1;
        let character = self.get_next();
        utf8_bytes += length(u32::from(character), last) as usize;
        last = i32::from(character);
      }
      utf8_bytes
    }
  }

  /// WriteUtf8Bytes (string-inl.h:1786): 前进 n 字符并编码写入。
  /// NUL 字符替换为空格 (ToCString 语义)。
  ///
  /// # Safety
  /// 同 [Self::new]; output 容量足够。
  pub unsafe fn write_utf8_bytes(&mut self, n_chars: u32, output: &mut [u8]) -> usize {
    // SAFETY: get_next + Utf8::Encode 契约
    unsafe {
      let mut pos = 0usize;
      let mut remaining = n_chars;
      let mut last = NO_PREVIOUS_CHARACTER;
      while self.has_more() && remaining != 0 {
        remaining -= 1;
        let mut character = self.get_next();
        if character == 0 {
          character = u16::from(b' ');
        }
        // 原版 SBXCHECK 承接: 容量不足按 debug 断言拦截
        let need = length(u32::from(character), last) as usize;
        debug_assert!(need <= output.len() - pos);
        if need > output.len() - pos {
          break;
        }
        // 代理合并: encode 契约要求 out 自 lead 的 3 字节起 (覆盖回写),
        // 返回净增 — pos 维持原位累加 (与 write_utf8_flat 同款承接)
        let base = if is_surrogate_pair(last, character as i32) {
          pos - SIZE_OF_UNMATCHED_SURROGATE as usize
        } else {
          pos
        };
        pos += encode(&mut output[base..], u32::from(character), last, false) as usize;
        last = i32::from(character);
      }
      pos
    }
  }
}

/// VisitFlat (string-inl.h:1190-1242): 顶层入口, Cons 返回待迭代根。
///
/// # Safety
/// string 须为有效字符串对象。
unsafe fn visit_flat(
  visitor: &mut StringCharacterStream,
  string: Tagged<String>,
  offset: u32,
) -> Option<Tagged<ConsString>> {
  // SAFETY: 委托流内 visit_flat (Cons 返回自身即待迭代根)
  unsafe {
    visitor.visit_flat(string, offset);
    if string.shape().is_cons() {
      Some(string.cast())
    } else {
      None
    }
  }
}

// ── WriteToFlat (string.cc:781-907) ───────────────────────────────────────
/// 汇字符域: u8/u16 (CopyChars 宽化语义的零扩展承接)。
pub trait SinkChar: Copy {
  /// 从 u16 源宽化/收窄。
  fn from_u16(c: u16) -> Self;
  /// 汇宽字节量。
  const WIDTH: usize;
}

impl SinkChar for u8 {
  fn from_u16(c: u16) -> Self {
    debug_assert!(u32::from(c) <= K_MAX_ONE_BYTE_CHAR_CODE);
    c as u8
  }
  const WIDTH: usize = 1;
}

impl SinkChar for u16 {
  fn from_u16(c: u16) -> Self {
    c
  }
  const WIDTH: usize = 2;
}

/// CopyChars 泛型 (utils/utils.h): src → sink[pos..][n]。
fn copy_chars<S: SinkChar, C: CharData>(sink: &mut [S], pos: usize, src: &[C], n: usize) {
  for (i, &c) in src.iter().take(n).enumerate() {
    sink[pos + i] = S::from_u16(c.widen());
  }
}

/// String::WriteToFlat (string.cc:790-907): 区间 [start, start+length) 展平写入。
///
/// Cons 二分递归 + 幂等复本消除 + 右单字符内联。
///
/// # Safety
/// source 须为有效字符串对象; sink 容量 ≥ length。
pub unsafe fn write_to_flat<S: SinkChar>(
  source: Tagged<String>,
  sink: &mut [S],
  start: u32,
  length: u32,
) -> Result<()> {
  // SAFETY: 全路径堆访问契约
  unsafe {
    if length == 0 {
      return Ok(());
    }
    let mut source = source;
    let mut sink_pos = 0usize;
    let mut start = start;
    let mut length = length;
    loop {
      debug_assert!(length > 0);
      debug_assert!(start + length <= source.length());

      match StringKind::of(source.shape().instance_type()) {
        StringKind::SeqOneByte => {
          // SAFETY: chars 契约
          let chars = source.cast::<SeqOneByteString>().chars();
          copy_chars(sink, sink_pos, &chars[start as usize..], length as usize);
          return Ok(());
        }
        StringKind::SeqTwoByte => {
          // SAFETY: chars 契约
          let chars = source.cast::<SeqTwoByteString>().chars();
          copy_chars(sink, sink_pos, &chars[start as usize..], length as usize);
          return Ok(());
        }
        StringKind::ExternalOneByte => {
          let ext = source.cast::<ExternalOneByteString>().as_ref();
          // SAFETY: resource_data 契约
          let chars = from_raw_parts(
            ext.external.resource_data as *const u8,
            ext.external.string.length as usize,
          );
          copy_chars(sink, sink_pos, &chars[start as usize..], length as usize);
          return Ok(());
        }
        StringKind::ExternalTwoByte => {
          let ext = source.cast::<ExternalTwoByteString>().as_ref();
          // SAFETY: resource_data 契约
          let chars = from_raw_parts(
            ext.external.resource_data as *const u16,
            ext.external.string.length as usize,
          );
          copy_chars(sink, sink_pos, &chars[start as usize..], length as usize);
          return Ok(());
        }
        StringKind::Cons => {
          let cons: Tagged<ConsString> = source.cast();
          let first = cons.first();
          let boundary = first.length();
          // 有符号域: 一侧可为负 (该侧无字符需求)
          let first_length = boundary as i64 - i64::from(start);
          let second_length = i64::from(length) - first_length;
          debug_assert_eq!(first_length + second_length, i64::from(length));
          if second_length >= first_length {
            debug_assert!(second_length > 0);
            // 右长左短: 递归左
            if first_length > 0 {
              debug_assert!(first_length < i64::from(length));
              debug_assert!(second_length < i64::from(length));
              // SAFETY: 递归契约
              write_to_flat(first, &mut sink[sink_pos..], start, first_length as u32)?;
              if start == 0 && cons.second().ptr() == first.ptr() {
                // 幂等 Cons: 右子树复本从已写左半拷贝 (string.cc:848-852)
                debug_assert!(u64::from(boundary) * 2 <= u64::from(length));
                let copied = boundary as usize;
                sink.copy_within(sink_pos..sink_pos + copied, sink_pos + copied);
                return Ok(());
              }
              sink_pos += first_length as usize;
              start = 0;
              length -= first_length as u32;
            } else {
              start -= boundary;
            }
            source = cons.second();
          } else {
            debug_assert!(first_length > 0);
            // 左长右短: 内联右
            if second_length > 0 {
              debug_assert!(first_length < i64::from(length));
              debug_assert!(second_length < i64::from(length));
              let second_start = first_length as usize;
              debug_assert_eq!(
                second_start as u64 + second_length as u64,
                u64::from(length)
              );
              let second = cons.second();
              // 尾递归不消除的右子内联 (string.cc:870-884)
              if second_length == 1 {
                // SAFETY: Get(0)
                sink[sink_pos + second_start] = S::from_u16(second.get(0));
              } else if StringKind::of(second.shape().instance_type()) == StringKind::SeqOneByte {
                // SAFETY: chars 契约
                let chars = second.cast::<SeqOneByteString>().chars();
                copy_chars(sink, sink_pos + second_start, chars, second_length as usize);
              } else {
                // SAFETY: 递归契约
                write_to_flat(
                  second,
                  &mut sink[sink_pos + second_start..],
                  0,
                  second_length as u32,
                )?;
              }
              length -= second_length as u32;
            }
            source = first;
          }
        }
        StringKind::Sliced => {
          // SAFETY: offset/parent 契约
          let sliced = source.cast::<SlicedString>();
          start += sliced.offset() as u32;
          source = sliced.parent();
        }
        StringKind::Thin => {
          // SAFETY: actual 契约
          source = source.cast::<ThinString>().actual();
        }
      }
    }
  }
}

// ── WriteToFlat2 (string.cc:1081-1123) ────────────────────────────────────
/// 重复消除器 (string.cc:988-1024): 指针同源子串复用首展平区。
struct RepeatOptimizer {
  /// s.ptr() → 首次展平时的汇尾位置 (None = 未启用)。
  first_occurrence: Option<gxhash::HashMap<usize, usize>>,
}

impl RepeatOptimizer {
  const fn new() -> Self {
    Self {
      first_occurrence: None,
    }
  }

  /// RecordFirstOccurrence (string.cc:990)。
  fn record(&mut self, ptr: usize, position: usize) {
    self
      .first_occurrence
      .get_or_insert_with(HashMap::default)
      .entry(ptr)
      .or_insert(position);
  }

  /// TryApply (string.cc:999-1015): 同位不消费; 异位回拷首区并消费。
  fn try_apply(&mut self, ptr: usize, length: u32, pos: &mut usize, dst: &mut [impl Copy]) -> bool {
    let Some(map) = self.first_occurrence.as_ref() else {
      return false;
    };
    let Some(&previous_position) = map.get(&ptr) else {
      return false;
    };
    if *pos == previous_position {
      return false;
    }
    let len = length as usize;
    debug_assert!(*pos + len <= previous_position);
    let src = previous_position - len;
    *pos -= len;
    let dest = *pos;
    dst.copy_within(src..src + len, dest);
    true
  }

  /// enabled (string.cc:1017)。
  const fn enabled(&self) -> bool {
    self.first_occurrence.is_some()
  }
}

/// wtf_push (string.cc:963): 旧 top 压栈, 新值占 top。
fn wtf_push(
  top: &mut Option<Tagged<String>>,
  stack: &mut Vec<Tagged<String>>,
  value: Tagged<String>,
) {
  if let Some(t) = top.take() {
    stack.push(t);
  }
  *top = Some(value);
}

/// wtf_try_pop (string.cc:969)。
fn wtf_try_pop(
  top: &mut Option<Tagged<String>>,
  stack: &mut Vec<Tagged<String>>,
) -> Option<Tagged<String>> {
  top.take().or_else(|| stack.pop())
}

/// WriteToFlat2Impl (string.cc:1026-1076): 单变体循环。
///
/// SEQ_ONE_BYTE 专用变体遇非 SeqOneByte 叶或 ropt 启用时压回转通用
/// (返回 true); 通用变体 (SEQ_ONE_BYTE=false) 前置 TryApply。
///
/// # Safety
/// 同 [write_to_flat2]。
unsafe fn wtf_impl<S: SinkChar, const SEQ_ONE_BYTE: bool>(
  pos: &mut usize,
  top: &mut Option<Tagged<String>>,
  stack: &mut Vec<Tagged<String>>,
  ropt: &mut RepeatOptimizer,
  dst: &mut [S],
) -> Result<bool> {
  // SAFETY: 全路径堆访问契约
  unsafe {
    while let Some(mut s) = wtf_try_pop(top, stack) {
      if !SEQ_ONE_BYTE && ropt.try_apply(s.ptr(), s.length(), pos, dst) {
        continue;
      }
      // 右行到底, 左支经 top/stack 缓冲压栈 (string.cc:1047-1056)
      let mut shape = s.shape();
      while shape.is_cons() {
        let cons: Tagged<ConsString> = s.cast();
        let first = cons.first();
        wtf_push(top, stack, first);
        s = cons.second();
        if s.ptr() == first.ptr() {
          ropt.record(s.ptr(), *pos);
        }
        shape = s.shape();
      }
      if SEQ_ONE_BYTE && (!shape.is_sequential_one_byte() || ropt.enabled()) {
        // 退出专用变体 (string.cc:1058-1064); 调用方必须续接通用变体
        wtf_push(top, stack, s);
        return Ok(true);
      }
      let len = s.length() as usize;
      *pos -= len;
      if SEQ_ONE_BYTE {
        // SAFETY: chars 契约
        let chars = s.cast::<SeqOneByteString>().chars();
        copy_chars(dst, *pos, chars, len);
      } else {
        // SAFETY: WriteNonConsToFlat2 (string.cc:911-949)
        write_non_cons_to_flat2(s, shape, dst, *pos, 0, s.length())?;
      }
    }
    Ok(false)
  }
}

/// String::WriteToFlat2 (string.cc:1082-1123): 全串反向 DFS 序展平。
///
/// 左倾 append 链栈深恒 1; 专用 SeqOneByte 循环 + 通用循环双段
/// (src_index/length 恒为 0/src->length(), 见头注释)。
///
/// # Safety
/// src 须为有效非展平 ConsString; dst 容量 ≥ src 长度。
pub unsafe fn write_to_flat2<S: SinkChar>(dst: &mut [S], src: Tagged<ConsString>) -> Result<()> {
  // SAFETY: 委托 wtf_impl, 契约同上
  unsafe {
    let src_len = src.as_ref().string.length;
    debug_assert_ne!(src_len, 0);
    debug_assert!(!src.is_flat());
    debug_assert!(dst.len() >= src_len as usize);

    let mut pos = dst.len();
    let mut stack: Vec<Tagged<String>> = Vec::with_capacity(32);
    stack.push(src.first());
    let mut top = Some(src.second());
    let mut ropt = RepeatOptimizer::new();

    wtf_impl::<S, true>(&mut pos, &mut top, &mut stack, &mut ropt, dst)?;
    wtf_impl::<S, false>(&mut pos, &mut top, &mut stack, &mut ropt, dst)?;
    debug_assert_eq!(pos, 0);
    Ok(())
  }
}

/// WriteNonConsToFlat2 (string.cc:911-949): 非串叶写段。
///
/// # Safety
/// 同 [write_to_flat2]; shape 为 s 的形状且非 Cons。
unsafe fn write_non_cons_to_flat2<S: SinkChar>(
  src: Tagged<String>,
  shape: StringShape,
  dst: &mut [S],
  dst_pos: usize,
  src_index: u32,
  length: u32,
) -> Result<()> {
  // SAFETY: 各叶访问契约
  unsafe {
    debug_assert!(!shape.is_cons());
    debug_assert!(src_index + length <= src.length());
    match StringKind::of(shape.instance_type()) {
      StringKind::SeqOneByte => {
        let chars = src.cast::<SeqOneByteString>().chars();
        copy_chars(dst, dst_pos, &chars[src_index as usize..], length as usize);
      }
      StringKind::SeqTwoByte => {
        let chars = src.cast::<SeqTwoByteString>().chars();
        copy_chars(dst, dst_pos, &chars[src_index as usize..], length as usize);
      }
      StringKind::ExternalOneByte => {
        let ext = src.cast::<ExternalOneByteString>().as_ref();
        let chars = from_raw_parts(
          ext.external.resource_data as *const u8,
          ext.external.string.length as usize,
        );
        copy_chars(dst, dst_pos, &chars[src_index as usize..], length as usize);
      }
      StringKind::ExternalTwoByte => {
        let ext = src.cast::<ExternalTwoByteString>().as_ref();
        let chars = from_raw_parts(
          ext.external.resource_data as *const u16,
          ext.external.string.length as usize,
        );
        copy_chars(dst, dst_pos, &chars[src_index as usize..], length as usize);
      }
      StringKind::Sliced => {
        let sliced = src.cast::<SlicedString>();
        let parent = sliced.parent();
        // SAFETY: 递归契约
        write_non_cons_to_flat2(
          parent,
          parent.shape(),
          dst,
          dst_pos,
          src_index + sliced.offset() as u32,
          length,
        )?;
      }
      StringKind::Thin => {
        let actual = src.cast::<ThinString>().actual();
        // SAFETY: 递归契约
        write_non_cons_to_flat2(actual, actual.shape(), dst, dst_pos, src_index, length)?;
      }
      // shape 已断言非 Cons; 到达即形状错乱
      StringKind::Cons => return Err(Error::InvalidDispatch("WriteNonConsToFlat2 遇 Cons")),
    }
    Ok(())
  }
}

/// 数组索引流式解析 (kToArrayIndex: u32 域, 上限 2^32-2)。
///
/// # Safety
/// 流内堆访问契约由调用方承接。
unsafe fn string_to_index_array(stream: &mut StringCharacterStream) -> Option<u32> {
  // SAFETY: 流契约由调用方承接
  unsafe {
    let ch = u32::from(stream.get_next());
    // 前导 0: 仅 "0" 合法
    if ch == u32::from(b'0') {
      return (!stream.has_more()).then_some(0);
    }
    let d0 = ch.wrapping_sub(u32::from(b'0'));
    if d0 > 9 {
      return None;
    }
    let mut result = d0;
    while stream.has_more() {
      let c = u32::from(stream.get_next());
      let d = c.wrapping_sub(u32::from(b'0'));
      if d > 9 {
        return None;
      }
      // TryAddArrayIndexChar: 分支无关上界 (utils-inl.h:36-44)
      if result > 429_496_729 - ((d + 3) >> 3) {
        return None;
      }
      result = result * 10 + d;
    }
    Some(result)
  }
}

/// 整数索引流式解析 (kToIntegerIndex: usize 域, 上限 2^53-1)。
///
/// # Safety
/// 流内堆访问契约由调用方承接。
unsafe fn string_to_index_integer(stream: &mut StringCharacterStream) -> Option<usize> {
  // SAFETY: 流契约由调用方承接
  unsafe {
    const MAX_SAFE_INTEGER: u64 = (1 << 53) - 1;
    let ch = u32::from(stream.get_next());
    if ch == u32::from(b'0') {
      return (!stream.has_more()).then_some(0);
    }
    let d0 = u64::from(ch.wrapping_sub(u32::from(b'0')));
    if d0 > 9 {
      return None;
    }
    let mut result = d0;
    while stream.has_more() {
      let c = u32::from(stream.get_next());
      let d = u64::from(c.wrapping_sub(u32::from(b'0')));
      if d > 9 {
        return None;
      }
      // TryAddIntegerIndexChar (utils-inl.h:49-55)
      result = result * 10 + d;
      if result > MAX_SAFE_INTEGER {
        return None;
      }
    }
    Some(result as usize)
  }
}

// ── FlatStringReader (string.h:1397-1411 + string.cc:2203-2224) ───────────
/// 展平串随机读 (与字符宽度无关; 段首裸指针承接原版 start_)。
pub struct FlatStringReader {
  /// 单字节标记。
  is_one_byte: bool,
  /// 长度。
  length: u32,
  /// 段首 (字节解释)。
  start8: *const u8,
  /// 段首 (双字节解释)。
  start16: *const u16,
}

impl FlatStringReader {
  /// 构造 + 取段 (PostGarbageCollection; 输入须已展平)。
  ///
  /// # Safety
  /// string 须为有效已展平字符串; reader 存活期内字符内存稳定。
  pub unsafe fn new(string: Tagged<String>) -> Self {
    // SAFETY: get_flat_content 契约
    unsafe {
      let content = string.get_flat_content();
      debug_assert!(content.is_flat());
      Self {
        is_one_byte: content.is_one_byte(),
        length: content.length(),
        start8: content.to_one_byte_vector().as_ptr(),
        start16: content.to_uc16_vector().as_ptr(),
      }
    }
  }

  /// length。
  #[must_use]
  pub const fn length(&self) -> u32 {
    self.length
  }

  /// Get (string-inl.h:546): uc32 宽读。
  #[must_use]
  pub fn get(&self, index: u32) -> u32 {
    if self.is_one_byte {
      // SAFETY: 段内存稳定契约; 索引在 length 域内
      u32::from(unsafe { *self.start8.add(index as usize) })
    } else {
      // SAFETY: 同上
      u32::from(unsafe { *self.start16.add(index as usize) })
    }
  }
}

// ── SubStringRange (string.h:855-870 + string-inl.h:1858-1906) ────────────
/// 子串码元迭代器 (GetFlatContent 惰性视图)。
pub struct SubStringRange {
  /// 宿主串。
  string: Tagged<String>,
  /// 起始。
  first: i32,
  /// 剩余长度。
  length: i32,
}

impl SubStringRange {
  /// 构造 (string-inl.h:1858; length None → 串尾)。
  ///
  /// # Safety
  /// string 须为有效已展平字符串; 迭代期内字符内存稳定。
  pub unsafe fn new(string: Tagged<String>, first: i32, length: Option<i32>) -> Self {
    // SAFETY: length 字段读取契约
    let full = unsafe { string.length() } as i32;
    Self {
      string,
      first,
      length: length.unwrap_or(full),
    }
  }
}

impl Iterator for SubStringRange {
  type Item = u16;

  fn next(&mut self) -> Option<u16> {
    if self.length <= 0 {
      return None;
    }
    self.length -= 1;
    let offset = self.first;
    self.first += 1;
    // SAFETY: GetFlatContent 惰性取段 (iterator 解引用的原版语义)
    unsafe {
      let content = self.string.get_flat_content();
      Some(content.get(offset as u32))
    }
  }
}

/// SubStringRange 便捷入口 (tests/宿主用)。
///
/// # Safety
/// 同 [SubStringRange::new]。
pub unsafe fn sub_string_range(
  string: Tagged<String>,
  first: i32,
  length: Option<i32>,
) -> SubStringRange {
  // SAFETY: 委托构造
  unsafe { SubStringRange::new(string, first, length) }
}

// ── ToArrayIndex / ToCString / ToStdString (string.cc:669/732-778) ────────
/// String::ToArrayIndex (string.cc:669): array index 且 ≤ INT_MAX。
///
/// # Safety
/// 同 [StringExt]。
pub unsafe fn to_array_index(key: Tagged<String>) -> Option<i32> {
  // SAFETY: as_array_index 契约
  unsafe {
    let index = key.as_array_index()?;
    (index <= i32::MAX as u32).then_some(index as i32)
  }
}

/// String::ToCString(offset, length) (string.cc:732-759): NUL 替换空格 +
/// 显式 NUL 终止 (返回含 NUL 的字节向量)。
///
/// # Safety
/// 同 [StringExt]。
pub unsafe fn to_cstring(string: Tagged<String>, offset: u32, length: u32) -> Result<Vec<u8>> {
  // SAFETY: 流契约
  unsafe {
    debug_assert!(length <= string.length());
    debug_assert!(offset <= string.length() - length);
    let mut stream = StringCharacterStream::new(string, offset);
    let utf8_bytes = stream.count_utf8_bytes(length);
    let mut result = vec![0u8; utf8_bytes + 1];
    stream.reset(string, offset);
    let pos = stream.write_utf8_bytes(length, &mut result[..utf8_bytes]);
    debug_assert!(pos < result.len());
    result[pos] = 0;
    Ok(result)
  }
}

/// String::ToStdString (string.cc:765-778)。
///
/// # Safety
/// 同 [StringExt]。
pub unsafe fn to_std_string(string: Tagged<String>) -> Result<string::String> {
  // SAFETY: 流契约
  unsafe {
    let length = string.length();
    let mut stream = StringCharacterStream::new(string, 0);
    let utf8_bytes = stream.count_utf8_bytes(length);
    let mut result = vec![0u8; utf8_bytes];
    stream.reset(string, 0);
    stream.write_utf8_bytes(length, &mut result);
    string::String::from_utf8(result)
      .map_err(|_| Error::Unsupported("ToStdString 非法 UTF-8 (孤立代理)"))
  }
}

// ── DoesNotContainEscapeCharacters (string.cc:1157-1168) ──────────────────
/// 校验路径专用: 遇需转义字符 (控制/引号/反斜杠/代理区) 即假。
///
/// # Safety
/// 同 [StringExt]。
pub unsafe fn does_not_contain_escape_characters(string: Tagged<String>) -> bool {
  // SAFETY: 流契约
  unsafe {
    let mut stream = StringCharacterStream::new(string, 0);
    while stream.has_more() {
      let c = stream.get_next();
      if c < 0x20 || c == 0x22 || c == 0x5C || (0xD800..=0xDFFF).contains(&c) {
        return false;
      }
    }
    true
  }
}

// ── 展平内容版自由函数族 (输入自带数据, 安全入口) ──────────────────────────
/// String::IndexOf 展平版 (string.cc:1566-1593 + SearchString)。
#[must_use]
pub fn index_of_flat(receiver: &FlatContent<'_>, search: &FlatContent<'_>, start: i32) -> i32 {
  if search.length() == 0 {
    return start;
  }
  if start < 0
    || u64::from(start as u32) + u64::from(search.length()) > u64::from(receiver.length())
  {
    return -1;
  }
  let start = start as usize;
  match (receiver, search) {
    (FlatContent::OneByte(s), FlatContent::OneByte(p)) => {
      search_string_u8(s, p, start).map_or(-1, |i| i as i32)
    }
    (FlatContent::OneByte(s), FlatContent::TwoByte(p)) => {
      search_string_u16_in_u8(s, p, start).map_or(-1, |i| i as i32)
    }
    (FlatContent::TwoByte(s), FlatContent::OneByte(p)) => {
      search_string_u8_in_u16(s, p, start).map_or(-1, |i| i as i32)
    }
    (FlatContent::TwoByte(s), FlatContent::TwoByte(p)) => {
      search_string_u16(s, p, start).map_or(-1, |i| i as i32)
    }
    // 非展平输入: 长度检查已拦常规越界, 此处兜底
    (FlatContent::NonFlat, _) | (_, FlatContent::NonFlat) => -1,
  }
}

/// StringMatchBackwards (string.cc:1755-1785) + LastIndexOf 窗口饱和
/// (string.cc:1789-1859)。
#[must_use]
pub fn last_index_of_flat(receiver: &FlatContent<'_>, search: &FlatContent<'_>, start: i32) -> i32 {
  let pattern_length = i64::from(search.length());
  let receiver_length = i64::from(receiver.length());
  // 窗口起点饱和 (start + pattern > receiver → 收到 receiver - pattern)
  let mut idx = i64::from(start);
  if idx + pattern_length > receiver_length {
    idx = receiver_length - pattern_length;
  }
  if search.length() == 0 {
    return idx as i32;
  }
  if idx < 0 {
    return -1;
  }
  let idx = idx as usize;
  // 主串单字节 + 模式双字节: 含非 Latin-1 即 -1 (string.cc:1761-1768)
  if receiver.is_one_byte()
    && search.is_two_byte()
    && search
      .to_uc16_vector()
      .iter()
      .any(|&c| u32::from(c) > K_MAX_ONE_BYTE_CHAR_CODE)
  {
    return -1;
  }
  match (receiver, search) {
    (FlatContent::OneByte(s), FlatContent::OneByte(p)) => match_backwards(s, p, idx),
    (FlatContent::OneByte(s), FlatContent::TwoByte(p)) => match_backwards(s, p, idx),
    (FlatContent::TwoByte(s), FlatContent::OneByte(p)) => match_backwards(s, p, idx),
    (FlatContent::TwoByte(s), FlatContent::TwoByte(p)) => match_backwards(s, p, idx),
    (FlatContent::NonFlat, _) | (_, FlatContent::NonFlat) => -1,
  }
}

/// StringMatchBackwards 泛型 (string.cc:1755-1785)。
fn match_backwards<S: CharData, P: CharData>(subject: &[S], pattern: &[P], idx: usize) -> i32 {
  let pattern_length = pattern.len();
  debug_assert!(pattern_length >= 1);
  debug_assert!(idx + pattern_length <= subject.len());
  let pattern_first = pattern[0].widen();
  let mut i = idx as i64;
  while i >= 0 {
    let iu = i as usize;
    if subject[iu].widen() == pattern_first
      && pattern[1..]
        .iter()
        .zip(&subject[iu + 1..])
        .all(|(&p, &s)| p.widen() == s.widen())
    {
      return i as i32;
    }
    i -= 1;
  }
  -1
}

/// String::Compare 展平版 (string.cc:1453-1513)。
#[must_use]
pub fn compare_flat(x: &FlatContent<'_>, y: &FlatContent<'_>) -> ComparisonResult {
  use v8_objects_core::objects::ComparisonResult as Cr;
  // 首字符快判 (双非空)
  if x.length() != 0 && y.length() != 0 {
    let d = i32::from(x.get(0)) - i32::from(y.get(0));
    if d < 0 {
      return Cr::LessThan;
    }
    if d > 0 {
      return Cr::GreaterThan;
    }
  }
  // 前缀长与倾侧
  let mut result = Cr::Equal;
  let mut prefix_length = x.length();
  if y.length() < prefix_length {
    prefix_length = y.length();
    result = Cr::GreaterThan;
  } else if y.length() > prefix_length {
    result = Cr::LessThan;
  }
  // 逐字符 (交叉宽度宽化比较; CompareChars)
  let r = compare_chars(x, y, prefix_length as usize);
  if r < 0 {
    Cr::LessThan
  } else if r > 0 {
    Cr::GreaterThan
  } else {
    result
  }
}

/// CompareChars (交叉宽度; 切片 zip, 免逐次 FlatContent::get 分派)。
fn compare_chars(x: &FlatContent<'_>, y: &FlatContent<'_>, n: usize) -> i32 {
  fn cmp_zip<A: CharData, B: CharData>(a: &[A], b: &[B], n: usize) -> i32 {
    for (&ca, &cb) in a.iter().zip(b).take(n) {
      let (ca, cb) = (ca.widen(), cb.widen());
      if ca != cb {
        return if ca < cb { -1 } else { 1 };
      }
    }
    0
  }
  match (x, y) {
    (FlatContent::OneByte(a), FlatContent::OneByte(b)) => cmp_zip(a, b, n),
    (FlatContent::OneByte(a), FlatContent::TwoByte(b)) => cmp_zip(a, b, n),
    (FlatContent::TwoByte(a), FlatContent::OneByte(b)) => cmp_zip(a, b, n),
    (FlatContent::TwoByte(a), FlatContent::TwoByte(b)) => cmp_zip(a, b, n),
    // 非展平输入 length 为 0 (compare_flat 前置已拦), n=0 空转
    (FlatContent::NonFlat, _) | (_, FlatContent::NonFlat) => 0,
  }
}

/// CalculateLineEndsImpl (string.cc:1171-1246; SIMD 路径标量化等价)。
#[must_use]
pub fn calculate_line_ends(content: &FlatContent<'_>, include_ending_line: bool) -> Vec<i32> {
  use v8_strings::char_predicates::{
    is_line_terminator_sequence_one_byte, is_line_terminator_sequence_two_byte,
  };
  // 尾前扫描 (\r 后随 \n 计 \n 位); 最末字符无后继 → 0 占位 (C::Default = 0)
  fn scan<C: Copy + Default>(src: &[C], is_term: fn(C, C) -> bool) -> Vec<i32> {
    let mut line_ends = Vec::new();
    if src.is_empty() {
      return line_ends;
    }
    for i in 0..src.len() - 1 {
      if is_term(src[i], src[i + 1]) {
        line_ends.push(i as i32);
      }
    }
    if is_term(src[src.len() - 1], C::default()) {
      line_ends.push((src.len() - 1) as i32);
    }
    line_ends
  }
  let mut line_ends = match content {
    FlatContent::OneByte(src) => scan(src, is_line_terminator_sequence_one_byte),
    FlatContent::TwoByte(src) => scan(src, is_line_terminator_sequence_two_byte),
    FlatContent::NonFlat => Vec::new(),
  };
  if include_ending_line {
    // 含结尾后一位 (rewriter 隐式 return 位; string.cc:1241-1245)
    line_ends.push(content.length() as i32);
  }
  line_ends
}

/// IsIdentifierVector (string.cc:1868-1882)。
#[must_use]
pub fn is_identifier_flat(content: &FlatContent<'_>) -> bool {
  use v8_strings::char_predicates::{is_identifier_part, is_identifier_start};
  // 单遍码点流 (免 Vec<u32> 中转): 首字符判 start, 其余判 part, 提前终止
  fn check<I: Iterator<Item = u32>>(mut chars: I) -> bool {
    match chars.next() {
      None => false,
      Some(first) => is_identifier_start(first) && chars.all(is_identifier_part),
    }
  }
  match content {
    FlatContent::OneByte(v) => check(v.iter().map(|&c| u32::from(c))),
    FlatContent::TwoByte(v) => check(v.iter().map(|&c| u32::from(c))),
    FlatContent::NonFlat => false,
  }
}

/// Utf8Length 展平版 (string-inl.h:1244-1271; simdutf 计数的 Length 循环等价)。
#[must_use]
pub fn utf8_length_flat(content: &FlatContent<'_>) -> usize {
  use v8_strings::unicode::{utf8, utf16};
  match content {
    FlatContent::OneByte(v) => v.iter().map(|&c| utf8::length_one_byte(c) as usize).sum(),
    FlatContent::TwoByte(v) => {
      // 合法 UTF-16 的 simdutf::utf8_length_from_utf16 与 Length 循环逐码点
      // 等价; 非法回退同循环 (孤立代理 3 字节; string-inl.h:1263-1270)
      let mut total = 0usize;
      let mut last = utf16::NO_PREVIOUS_CHARACTER;
      for &c in *v {
        total += utf8::length(u32::from(c), last) as usize;
        last = i32::from(c);
      }
      total
    }
    FlatContent::NonFlat => 0,
  }
}

/// WriteUtf8 展平版 (string.cc:1126-1153 + unicode-inl.h:210-281)。
///
/// 返回 (bytes_written, characters_processed); kNullTerminate 时 NUL 计入。
#[must_use]
pub fn write_utf8_flat(content: &FlatContent<'_>, buffer: &mut [u8], flags: u32) -> (usize, usize) {
  use v8_strings::unicode::{utf8, utf16};
  let write_null = flags & UTF8_FLAG_NULL_TERMINATE != 0;
  let mut replace_invalid = flags & UTF8_FLAG_REPLACE_INVALID != 0;
  // 单字节源不含无效 unicode (unicode-inl.h:215-218)
  if content.is_one_byte() {
    replace_invalid = false;
  }
  let content_capacity = buffer.len() - usize::from(write_null);
  let mut write_index = 0usize;
  let mut read_index = 0usize;
  let mut last = utf16::NO_PREVIOUS_CHARACTER;

  let get_char = |i: usize| -> u32 {
    match content {
      FlatContent::OneByte(v) => u32::from(v[i]),
      FlatContent::TwoByte(v) => u32::from(v[i]),
      FlatContent::NonFlat => 0,
    }
  };

  if let FlatContent::OneByte(chars) = content {
    // WriteLeadingAscii 快路径: ASCII 前缀直拷 (unicode-inl.h:225-231)
    let writeable = chars.len().min(content_capacity);
    let ascii_length = chars[..writeable]
      .iter()
      .take_while(|&c| c.is_ascii())
      .count();
    buffer[..ascii_length].copy_from_slice(&chars[..ascii_length]);
    read_index = ascii_length;
    write_index = ascii_length;
  }

  let total = content.length() as usize;
  while read_index < total {
    let character = get_char(read_index);
    let required_capacity = if content.is_one_byte() {
      utf8::length_one_byte(character as u8) as usize
    } else {
      utf8::length(character, last) as usize
    };
    let remaining_capacity = content_capacity - write_index;
    if remaining_capacity < required_capacity {
      // 容量不足中断: 代理对中段回退已写的 lead (unicode-inl.h:244-253)
      if utf16::is_surrogate_pair(last, character as i32) {
        debug_assert!(write_index >= utf8::SIZE_OF_UNMATCHED_SURROGATE as usize);
        write_index -= utf8::SIZE_OF_UNMATCHED_SURROGATE as usize;
        debug_assert_ne!(read_index, 0);
        read_index -= 1;
      }
      break;
    }
    if content.is_one_byte() {
      write_index += utf8::encode_one_byte(&mut buffer[write_index..], character as u8) as usize;
    } else if read_index + 1 < total
      && utf16::is_surrogate_pair(character as i32, get_char(read_index + 1) as i32)
    {
      // 代理对 lead: 预留 3 字节占位, trail 合并时回写 (unicode-inl.h:259-262)
      write_index += utf8::SIZE_OF_UNMATCHED_SURROGATE as usize;
    } else {
      // 代理合并: encode 契约要求 out 自 lead 的 3 字节预留位起 (覆盖回写),
      // 返回净增字节数 — write_index 维持原位累加 (unicode-inl.h:259-267)
      // SAFETY: 容量已检
      let base = if utf16::is_surrogate_pair(last, character as i32) {
        write_index - utf8::SIZE_OF_UNMATCHED_SURROGATE as usize
      } else {
        write_index
      };
      write_index += utf8::encode(&mut buffer[base..], character, last, replace_invalid) as usize;
    }
    last = character as i32;
    read_index += 1;
  }
  if write_null {
    debug_assert!(write_index < buffer.len());
    buffer[write_index] = 0;
    write_index += 1;
  }
  (write_index, read_index)
}

/// IsWellFormedUnicode 展平版 (string-inl.h:1273-1289)。
#[must_use]
pub fn is_well_formed_unicode_flat(content: &FlatContent<'_>) -> bool {
  use v8_strings::unicode::utf16;
  match content {
    // 单字节串定义即良构
    FlatContent::OneByte(_) => true,
    FlatContent::TwoByte(v) => !utf16::has_unpaired_surrogate(v),
    FlatContent::NonFlat => false,
  }
}

/// String::IsAscii (string.h:622)。
#[must_use]
pub fn is_ascii(chars: &[u8]) -> bool {
  chars.is_ascii()
}

/// String::IsOneByte (string.h:670)。
#[must_use]
pub fn is_one_byte_uc16(chars: &[u16]) -> bool {
  non_one_byte_start(chars) >= chars.len() as u32
}

/// String::NonOneByteStart (string.h:633-668; SWAR 字路径标量化等价)。
#[must_use]
pub fn non_one_byte_start(chars: &[u16]) -> u32 {
  chars
    .iter()
    .position(|&c| u32::from(c) > K_MAX_ONE_BYTE_CHAR_CODE)
    .map_or(chars.len() as u32, |pos| pos as u32)
}

// @cc-semantics-end
