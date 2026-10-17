//! 反馈来源值类型 — FeedbackSource(vector+slot) / EmbeddedFeedbackSource(bytecode+offset)
//!
//! 1:1 对应 v8/src/compiler/feedback-source.{h,cc}。
//! vector 以裸地址承载 (原版 IndirectHandle<FeedbackVector>; handles 层执行面推迟)。

use std::{
  fmt,
  hash::{Hash, Hasher},
  ptr::null,
};

use v8_base::hashing::hash_combine;

/// 无效槽位哨兵 (feedback-vector.h kInvalidSlot = -1)
/// v8: src/objects/feedback-vector.h kInvalidSlot
const K_INVALID_SLOT: i32 = -1;

/// 反馈来源 — (反馈向量, 槽位) 二元组
/// v8: src/compiler/feedback-source.h FeedbackSource
#[derive(Debug, Clone, Copy)]
pub struct FeedbackSource {
  /// 反馈向量裸地址 (原版 vector, IndirectHandle<FeedbackVector>)
  pub vector: *const (),
  /// 槽位 (原版 slot, FeedbackSlot)
  pub slot: i32,
}

impl FeedbackSource {
  /// 默认构造即无效 (原版 DCHECK(!IsValid()))
  /// v8: src/compiler/feedback-source.h FeedbackSource::FeedbackSource()
  pub const fn invalid() -> Self {
    Self {
      vector: null(),
      slot: K_INVALID_SLOT,
    }
  }

  /// 带参构造 (原版 DCHECK(!slot.IsInvalid()))
  /// v8: src/compiler/feedback-source.cc FeedbackSource::FeedbackSource(vector_, slot_)
  pub const fn new(vector: *const (), slot: i32) -> Self {
    debug_assert!(slot != K_INVALID_SLOT, "slot.IsInvalid()");
    Self { vector, slot }
  }

  /// 有效性 — 向量非空且槽位有效
  /// v8: src/compiler/feedback-source.h FeedbackSource::IsValid
  pub const fn is_valid(&self) -> bool {
    !self.vector.is_null() && self.slot != K_INVALID_SLOT
  }

  /// 槽位下标 — CHECK(IsValid()); FeedbackVector::GetIndex(slot) = slot.ToInt()
  /// v8: src/compiler/feedback-source.cc FeedbackSource::index
  pub fn index(&self) -> i32 {
    debug_assert!(self.is_valid(), "FeedbackSource::index 要求有效来源");
    self.slot
  }

  /// hash_combine(vector.address(), slot)
  /// v8: src/compiler/feedback-source.h FeedbackSource::Hash
  pub fn hash_value(&self) -> usize {
    hash_combine(self.vector as usize, self.slot as usize)
  }
}

/// v8: src/compiler/feedback-source.h FeedbackSource::Equal
impl PartialEq for FeedbackSource {
  fn eq(&self, other: &Self) -> bool {
    self.vector == other.vector && self.slot == other.slot
  }
}
impl Eq for FeedbackSource {}

impl Hash for FeedbackSource {
  /// v8: src/compiler/feedback-source.h FeedbackSource::Hash
  fn hash<H: Hasher>(&self, state: &mut H) {
    (self.vector as usize).hash(state);
    self.slot.hash(state);
  }
}

/// v8: src/compiler/feedback-source.cc operator<<(ostream, FeedbackSource)
impl fmt::Display for FeedbackSource {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if self.is_valid() {
      write!(f, "FeedbackSource({})", self.slot)
    } else {
      write!(f, "FeedbackSource(INVALID)")
    }
  }
}

/// 转换为上游图算子占位形 (v8-compiler-graph use_info::FeedbackSource)
/// v8: src/compiler/feedback-source.h FeedbackSource
impl From<FeedbackSource> for v8_compiler_graph::FeedbackSource {
  fn from(s: FeedbackSource) -> Self {
    Self {
      vector: s.vector,
      slot: s.slot,
    }
  }
}

/// 嵌入式提示来源 — (BytecodeArray, offset) 二元组
/// v8: src/compiler/feedback-source.h EmbeddedFeedbackSource
#[derive(Debug, Clone, Copy)]
pub struct EmbeddedFeedbackSource {
  /// 字节码数组裸地址 (原版 bytecode_array_, IndirectHandle<BytecodeArray>)
  pub bytecode_array: *const (),
  /// 字节码偏移 (原版 offset_)
  pub offset: i32,
}

impl EmbeddedFeedbackSource {
  /// 默认构造即无效 (原版 DCHECK(!IsValid()))
  /// v8: src/compiler/feedback-source.h EmbeddedFeedbackSource::EmbeddedFeedbackSource()
  pub const fn invalid() -> Self {
    Self {
      bytecode_array: null(),
      offset: 0,
    }
  }

  /// 带参构造 (原版 DCHECK(!bytecode_array.is_null()))
  /// v8: src/compiler/feedback-source.cc EmbeddedFeedbackSource::EmbeddedFeedbackSource
  pub const fn new(bytecode_array: *const (), offset: i32) -> Self {
    debug_assert!(!bytecode_array.is_null(), "bytecode_array 非空");
    Self {
      bytecode_array,
      offset,
    }
  }

  /// v8: src/compiler/feedback-source.h EmbeddedFeedbackSource::IsValid
  pub const fn is_valid(&self) -> bool {
    !self.bytecode_array.is_null()
  }

  /// hash_combine(bytecode_array_.address(), offset_)
  /// v8: src/compiler/feedback-source.h EmbeddedFeedbackSource::Hash
  pub fn hash_value(&self) -> usize {
    hash_combine(self.bytecode_array as usize, self.offset as usize)
  }
}

/// v8: src/compiler/feedback-source.h EmbeddedFeedbackSource::Equal
impl PartialEq for EmbeddedFeedbackSource {
  fn eq(&self, other: &Self) -> bool {
    self.bytecode_array == other.bytecode_array && self.offset == other.offset
  }
}
impl Eq for EmbeddedFeedbackSource {}

impl Hash for EmbeddedFeedbackSource {
  /// v8: src/compiler/feedback-source.h EmbeddedFeedbackSource::Hash
  fn hash<H: Hasher>(&self, state: &mut H) {
    (self.bytecode_array as usize).hash(state);
    self.offset.hash(state);
  }
}

/// v8: src/compiler/feedback-source.cc operator<<(ostream, EmbeddedFeedbackSource)
impl fmt::Display for EmbeddedFeedbackSource {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if self.is_valid() {
      write!(f, "EmbeddedFeedbackSource({})", self.offset)
    } else {
      write!(f, "EmbeddedFeedbackSource(INVALID)")
    }
  }
}
